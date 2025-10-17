use crate::models::{DocumentType, MainCategory, ClassificationResult};
use crate::database::Database;
use chrono::Local;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct Classifier {
    patterns: Vec<ClassificationPattern>,
    db: Arc<Mutex<Database>>,
}

struct ClassificationPattern {
    doc_type: DocumentType,
    required_keywords: Vec<String>,
    support_keywords: Vec<String>,
    blocker_keywords: Vec<String>,
}

impl Classifier {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        let patterns = vec![
            // FACTURE - Pattern strict pour éviter les faux positifs
            ClassificationPattern {
                doc_type: DocumentType {
                    name: "Facture".to_string(),
                    pattern: String::new(),
                    prefix: "FACT".to_string(),
                },
                required_keywords: vec!["facture".to_string(), "invoice".to_string()],
                support_keywords: vec![
                    "montant".to_string(),
                    "total".to_string(),
                    "tva".to_string(),
                    "€".to_string(),
                    "ht".to_string(),
                    "ttc".to_string(),
                    "payer".to_string(),
                    "échéance".to_string(),
                ],
                blocker_keywords: vec![
                    "contrat".to_string(),
                    "bulletin".to_string(),
                    "relevé".to_string(),
                    "attestation".to_string(),
                ],
            },
            // CONTRAT
            ClassificationPattern {
                doc_type: DocumentType {
                    name: "Contrat".to_string(),
                    pattern: String::new(),
                    prefix: "CONT".to_string(),
                },
                required_keywords: vec!["contrat".to_string(), "contract".to_string()],
                support_keywords: vec![
                    "signataire".to_string(),
                    "signature".to_string(),
                    "clause".to_string(),
                    "article".to_string(),
                    "durée".to_string(),
                    "résiliation".to_string(),
                    "accord".to_string(),
                ],
                blocker_keywords: vec!["facture".to_string()],
            },
            // RELEVÉ BANCAIRE
            ClassificationPattern {
                doc_type: DocumentType {
                    name: "Relevé bancaire".to_string(),
                    pattern: String::new(),
                    prefix: "BANK".to_string(),
                },
                required_keywords: vec!["relevé".to_string(), "statement".to_string()],
                support_keywords: vec![
                    "compte".to_string(),
                    "iban".to_string(),
                    "solde".to_string(),
                    "crédit".to_string(),
                    "débit".to_string(),
                    "opération".to_string(),
                ],
                blocker_keywords: vec![],
            },
            // BULLETIN DE PAIE
            ClassificationPattern {
                doc_type: DocumentType {
                    name: "Bulletin de paie".to_string(),
                    pattern: String::new(),
                    prefix: "PAIE".to_string(),
                },
                required_keywords: vec![
                    "bulletin".to_string(),
                    "paie".to_string(),
                    "salaire".to_string(),
                ],
                support_keywords: vec![
                    "payslip".to_string(),
                    "net à payer".to_string(),
                    "cotisation".to_string(),
                    "urssaf".to_string(),
                    "employeur".to_string(),
                ],
                blocker_keywords: vec!["facture".to_string()],
            },
            // DOCUMENT OFFICIEL
            ClassificationPattern {
                doc_type: DocumentType {
                    name: "Document officiel".to_string(),
                    pattern: String::new(),
                    prefix: "OFFI".to_string(),
                },
                required_keywords: vec![
                    "attestation".to_string(),
                    "certificat".to_string(),
                    "carte".to_string(),
                    "passeport".to_string(),
                ],
                support_keywords: vec![
                    "identité".to_string(),
                    "officiel".to_string(),
                    "permis".to_string(),
                    "république".to_string(),
                ],
                blocker_keywords: vec![],
            },
            // REÇU
            ClassificationPattern {
                doc_type: DocumentType {
                    name: "Reçu".to_string(),
                    pattern: String::new(),
                    prefix: "RECU".to_string(),
                },
                required_keywords: vec![
                    "reçu".to_string(),
                    "receipt".to_string(),
                    "ticket".to_string(),
                ],
                support_keywords: vec![
                    "caisse".to_string(),
                    "merci".to_string(),
                    "thank you".to_string(),
                ],
                blocker_keywords: vec!["facture".to_string()],
            },
        ];

        Classifier { patterns, db }
    }

    pub fn classify(&self, text: &str) -> DocumentType {
        let text_lower = text.to_lowercase();
        let mut best_score = 0.0f32;
        let mut best_type = DocumentType::default();

        for pattern in &self.patterns {
            let score = self.calculate_score(&text_lower, pattern);

            if score > best_score {
                best_score = score;
                best_type = pattern.doc_type.clone();
            }
        }

        // Seuil minimum de confiance : 60%
        if best_score < 0.6 {
            eprintln!(
                "⚠️  Classification incertaine (score: {:.2}%) - Type par défaut utilisé",
                best_score * 100.0
            );
            return DocumentType::default();
        }

        eprintln!(
            "✓ Classification: {} (score: {:.2}%)",
            best_type.name,
            best_score * 100.0
        );
        best_type
    }

    /// Classification enrichie avec catégorie, sous-catégorie et tags suggérés
    pub fn classify_detailed(&self, text: &str) -> ClassificationResult {
        let text_lower = text.to_lowercase();
        
        // 1. Déterminer la catégorie principale et obtenir le score
        let (category, score) = self.detect_main_category_with_score(&text_lower);
        
        // 2. Suggérer une sous-catégorie basée sur le contenu
        let subcategory = self.suggest_subcategory(&category, &text_lower);
        
        // 3. Extraire les tags automatiquement (avec métadonnées avancées)
        let suggested_tags = self.extract_tags(&text);
        
        // 4. Calculer la confiance basée sur le score des mots-clés
        // Score normalisé : on considère qu'un score de 3.0 = 75% de confiance
        // et on plafonne à 100%
        let confidence = (score / 4.0).min(1.0);
        
        if confidence < 0.3 {
            eprintln!("⚠️  Catégorie incertaine (score: {:.1}, confiance: {:.2}%) - Type 'Autre' utilisé", score, confidence * 100.0);
            return ClassificationResult {
                category: MainCategory::Autre,
                subcategory: None,
                suggested_tags,
                confidence,
            };
        }

        eprintln!("✓ Catégorie finale: {} (score: {:.1}, confiance: {:.2}%)", category.to_string(), score, confidence * 100.0);
        if let Some(ref sub) = subcategory {
            eprintln!("  → Sous-catégorie suggérée: {}", sub);
        }
        
        ClassificationResult {
            category,
            subcategory,
            suggested_tags,
            confidence,
        }
    }

    fn detect_main_category_with_score(&self, text: &str) -> (MainCategory, f32) {
        let text_lower = text.to_lowercase();
        
        // Charger les mots-clés depuis la base de données
        let keywords = match self.db.lock() {
            Ok(db) => match db.get_classification_keywords(None) {
                Ok(kws) => kws,
                Err(_) => {
                    eprintln!("⚠️  Erreur lors du chargement des mots-clés, utilisation des scores par défaut");
                    return (self.detect_main_category_fallback(&text_lower), 0.0);
                }
            },
            Err(_) => {
                eprintln!("⚠️  Impossible de verrouiller la base de données");
                return (self.detect_main_category_fallback(&text_lower), 0.0);
            }
        };

        // Calculer les scores pour chaque catégorie
        let mut category_scores: HashMap<String, f64> = HashMap::new();
        
        for kw in keywords {
            if text_lower.contains(&kw.keyword.to_lowercase()) {
                let category = kw.category.clone();
                let keyword = kw.keyword.clone();
                let weight = kw.weight;
                *category_scores.entry(category.clone()).or_insert(0.0) += weight;
                eprintln!("  ✓ Mot-clé trouvé: '{}' → {} (+{:.1})", keyword, category, weight);
            }
        }

        // Trouver la catégorie avec le score le plus élevé
        let mut scores = vec![
            (MainCategory::Financier, category_scores.get("Financier").copied().unwrap_or(0.0) as f32),
            (MainCategory::Administratif, category_scores.get("Administratif").copied().unwrap_or(0.0) as f32),
            (MainCategory::Sante, category_scores.get("Santé").copied().unwrap_or(0.0) as f32),
            (MainCategory::Professionnel, category_scores.get("Professionnel").copied().unwrap_or(0.0) as f32),
            (MainCategory::Immobilier, category_scores.get("Immobilier").copied().unwrap_or(0.0) as f32),
            (MainCategory::Academique, category_scores.get("Académique").copied().unwrap_or(0.0) as f32),
            (MainCategory::Personnel, category_scores.get("Personnel").copied().unwrap_or(0.0) as f32),
        ];

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        eprintln!("  📊 Scores: Financier={:.1}, Administratif={:.1}, Santé={:.1}, Autre={:.1}", 
            scores.iter().find(|(c, _)| matches!(c, MainCategory::Financier)).map(|(_, s)| s).unwrap_or(&0.0),
            scores.iter().find(|(c, _)| matches!(c, MainCategory::Administratif)).map(|(_, s)| s).unwrap_or(&0.0),
            scores.iter().find(|(c, _)| matches!(c, MainCategory::Sante)).map(|(_, s)| s).unwrap_or(&0.0),
            0.0
        );
        
        let best_score = scores[0].1;
        let best_category = scores[0].0.clone();
        
        // Seuil minimum ajusté : au moins 1 mot-clé de poids normal (1.0)
        if best_score >= 1.0 {
            eprintln!("  ✅ Catégorie sélectionnée: {} (score: {:.1})", best_category.to_string(), best_score);
            (best_category, best_score)
        } else {
            eprintln!("  ⚠️  Score trop faible ({:.1} < 1.0) → Autre", best_score);
            (MainCategory::Autre, best_score)
        }
    }

    #[allow(dead_code)]
    fn detect_main_category(&self, text: &str) -> MainCategory {
        let (category, _score) = self.detect_main_category_with_score(text);
        category
    }

    // Méthode de fallback si la DB n'est pas accessible
    fn detect_main_category_fallback(&self, text: &str) -> MainCategory {
        let mut scores = vec![
            (MainCategory::Financier, self.score_financier(text)),
            (MainCategory::Administratif, self.score_administratif(text)),
            (MainCategory::Sante, self.score_sante(text)),
            (MainCategory::Professionnel, self.score_professionnel(text)),
            (MainCategory::Immobilier, self.score_immobilier(text)),
            (MainCategory::Academique, self.score_academique(text)),
            (MainCategory::Personnel, self.score_personnel(text)),
        ];

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        if scores[0].1 > 0.3 {
            scores[0].0.clone()
        } else {
            MainCategory::Autre
        }
    }

    fn score_financier(&self, text: &str) -> f32 {
        let keywords = ["facture", "invoice", "relevé", "bancaire", "iban", "virement", "crédit", "débit"];
        self.keyword_score(text, &keywords)
    }

    fn score_administratif(&self, text: &str) -> f32 {
        let keywords = ["carte", "identité", "passeport", "attestation", "certificat", "imposition", "fiscal"];
        self.keyword_score(text, &keywords)
    }

    fn score_sante(&self, text: &str) -> f32 {
        let keywords = ["ordonnance", "médical", "médecin", "cpam", "sécurité sociale", "mutuelle", "pharmacie"];
        self.keyword_score(text, &keywords)
    }

    fn score_professionnel(&self, text: &str) -> f32 {
        let keywords = ["contrat de travail", "fiche de paie", "salaire", "employeur", "urssaf", "bulletin"];
        self.keyword_score(text, &keywords)
    }

    fn score_immobilier(&self, text: &str) -> f32 {
        let keywords = ["bail", "location", "loyer", "propriété", "acte", "notaire", "diagnostic"];
        self.keyword_score(text, &keywords)
    }

    fn score_academique(&self, text: &str) -> f32 {
        let keywords = ["article", "publication", "thèse", "diplôme", "université", "recherche", "doi"];
        self.keyword_score(text, &keywords)
    }

    fn score_personnel(&self, text: &str) -> f32 {
        let keywords = ["ticket", "reçu", "caisse", "courrier", "lettre"];
        self.keyword_score(text, &keywords)
    }

    fn keyword_score(&self, text: &str, keywords: &[&str]) -> f32 {
        let matches = keywords.iter().filter(|k| text.contains(*k)).count();
        matches as f32 / keywords.len() as f32
    }

    fn suggest_subcategory(&self, category: &MainCategory, text: &str) -> Option<String> {
        match category {
            MainCategory::Financier => {
                if text.contains("edf") || text.contains("électricité") {
                    Some("Facture énergie".to_string())
                } else if text.contains("sfr") || text.contains("orange") || text.contains("free") {
                    Some("Facture télécom".to_string())
                } else if text.contains("relevé") || text.contains("iban") {
                    Some("Relevé bancaire".to_string())
                } else if text.contains("facture") {
                    Some("Facture fournisseur".to_string())
                } else {
                    None
                }
            },
            MainCategory::Sante => {
                if text.contains("ordonnance") {
                    Some("Ordonnance".to_string())
                } else if text.contains("cpam") || text.contains("remboursement") {
                    Some("Remboursement sécu".to_string())
                } else if text.contains("analyse") || text.contains("résultat") {
                    Some("Résultat analyse".to_string())
                } else {
                    None
                }
            },
            MainCategory::Professionnel => {
                if text.contains("fiche de paie") || text.contains("salaire") {
                    Some("Fiche de paie".to_string())
                } else if text.contains("contrat") {
                    Some("Contrat de travail".to_string())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn calculate_score(&self, text: &str, pattern: &ClassificationPattern) -> f32 {
        // 1. Vérifier les bloquants (si présent, score = 0)
        for blocker in &pattern.blocker_keywords {
            if text.contains(blocker) {
                return 0.0;
            }
        }

        // 2. Compter les mots requis (au moins 1 doit être présent)
        let required_count = pattern
            .required_keywords
            .iter()
            .filter(|word| text.contains(word.as_str()))
            .count();

        if required_count == 0 {
            return 0.0;
        }

        // 3. Score de base selon les mots requis (70% du score)
        let required_score = required_count as f32 / pattern.required_keywords.len() as f32;

        // 4. Bonus pour les mots de support (30% du score)
        let support_count = pattern
            .support_keywords
            .iter()
            .filter(|word| text.contains(word.as_str()))
            .count();

        let support_score = if !pattern.support_keywords.is_empty() {
            support_count as f32 / pattern.support_keywords.len() as f32
        } else {
            0.0
        };

        // 5. Score final (70% requis + 30% support)
        let final_score = (required_score * 0.7) + (support_score * 0.3);

        final_score.min(1.0)
    }

    pub fn extract_tags(&self, text: &str) -> Vec<String> {
        let mut tags = Vec::new();

        // Extraire le numéro de facture
        if let Some(invoice_num) = self.extract_invoice_number(text) {
            tags.push(format!("N°{}", invoice_num));
        }

        // Extraire le montant total
        if let Some(amount) = self.extract_amount(text) {
            tags.push(format!("{}€", amount));
        }

        // Extraire le nom du client
        if let Some(client) = self.extract_client_name(text) {
            tags.push(client);
        }

        // Extraire l'entreprise/entité
        if let Some(entity) = self.extract_entity(text) {
            tags.push(entity);
        }

        // Extraire l'année
        if let Some(year) = self.extract_year(text) {
            tags.push(year);
        }

        // Extract dates complètes
        let date_re = Regex::new(r"\d{1,2}[/-]\d{1,2}[/-]\d{2,4}").unwrap();
        if date_re.is_match(text) {
            tags.push("contains_date".to_string());
        }

        // Extract company names (simple pattern)
        let company_re = Regex::new(r"(?i)(s\.a\.s|sarl|sas|sa|eurl|sci)\b").unwrap();
        if company_re.is_match(text) {
            tags.push("company_document".to_string());
        }

        // Extract email
        let email_re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        if email_re.is_match(text) {
            tags.push("contains_email".to_string());
        }

        // Extract phone
        let phone_re = Regex::new(r"(?i)(\+33|0)[1-9](\s?\d{2}){4}").unwrap();
        if phone_re.is_match(text) {
            tags.push("contains_phone".to_string());
        }

        tags
    }

    fn extract_year(&self, text: &str) -> Option<String> {
        let re = Regex::new(r"(20[0-2]\d)").ok()?;
        re.captures(text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn extract_amount(&self, text: &str) -> Option<String> {
        // Chercher les montants avec € ou EUR
        let patterns = vec![
            r"total\s*[:\s]*(\d+[\s,.]?\d+[.,]\d{2})\s*€",  // Total: 1234.56 €
            r"(\d+[\s,.]?\d+[.,]\d{2})\s*€",                // 1234.56 €
            r"(\d+[\s,.]?\d+[.,]\d{2})\s*eur",              // 1234.56 EUR
        ];
        
        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(&text.to_lowercase()) {
                    if let Some(m) = caps.get(1) {
                        return Some(m.as_str().replace(",", ".").replace(" ", ""));
                    }
                }
            }
        }
        None
    }

    fn extract_invoice_number(&self, text: &str) -> Option<String> {
        let patterns = vec![
            r"facture\s*n[°º]\s*[:\s]*(\d{4}-\d{3})",           // Facture N° 2024-004
            r"facture\s*n[°º]\s*[:\s]*([\d-]+)",                // Facture N° 2024-004
            r"invoice\s*#?\s*[:\s]*([\d-]+)",                   // Invoice #2024-004
            r"n[°º]\s*facture\s*[:\s]*([\d-]+)",                // N° facture 2024-004
        ];
        
        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(&text.to_lowercase()) {
                    if let Some(m) = caps.get(1) {
                        return Some(m.as_str().to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_client_name(&self, text: &str) -> Option<String> {
        // Chercher des patterns de noms (nom prénom ou prénom nom)
        // Après des marqueurs comme "client:", "à:", etc.
        let patterns = vec![
            r"(?i)(?:client|à|pour)\s*[:\s]*([A-ZÉÈÊÀÂÔÛÇ][a-zéèêàâôûç]+(?:\s+[A-ZÉÈÊÀÂÔÛÇ][a-zéèêàâôûç]+)+)",
            r"([A-ZÉÈÊÀÂÔÛÇ][a-zéèêàâôûç]+\s+[A-ZÉÈÊÀÂÔÛÇ][a-zéèêàâôûç]+)\s*\(\s*ei\s*\)", // Nom (EI)
        ];
        
        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(text) {
                    if let Some(m) = caps.get(1) {
                        let name = m.as_str().trim().to_string();
                        // Vérifier que ce n'est pas un mot commun
                        if !name.to_lowercase().contains("facture") 
                            && !name.to_lowercase().contains("client")
                            && name.len() > 5 {
                            return Some(name);
                        }
                    }
                }
            }
        }
        None
    }

    fn extract_entity(&self, text: &str) -> Option<String> {
        // Chercher des noms connus d'entreprises
        let entities = vec![
            ("edf", "EDF"),
            ("engie", "Engie"),
            ("orange", "Orange"),
            ("sfr", "SFR"),
            ("free", "Free"),
            ("bouygues", "Bouygues"),
            ("cpam", "CPAM"),
            ("la poste", "La Poste"),
            ("sncf", "SNCF"),
            ("ratp", "RATP"),
            ("gray matter technology", "Gray Matter Technology"),
        ];
        
        let text_lower = text.to_lowercase();
        for (search, display) in entities {
            if text_lower.contains(search) {
                return Some(display.to_string());
            }
        }
        
        None
    }

    pub fn generate_filename(&self, doc_type: &DocumentType, original_name: &str) -> String {
        let now = Local::now();
        let date_str = now.format("%Y%m%d_%H%M%S");

        // Get file extension
        let extension = std::path::Path::new(original_name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("txt");

        format!("{}_{}.{}", doc_type.prefix, date_str, extension)
    }
}
