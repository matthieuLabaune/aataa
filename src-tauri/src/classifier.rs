use regex::Regex;
use chrono::Local;
use crate::models::DocumentType;

pub struct Classifier {
    patterns: Vec<ClassificationPattern>,
}

struct ClassificationPattern {
    doc_type: DocumentType,
    required_keywords: Vec<String>,
    support_keywords: Vec<String>,
    blocker_keywords: Vec<String>,
}

impl Classifier {
    pub fn new() -> Self {
        let patterns = vec![
            // FACTURE - Pattern strict pour éviter les faux positifs
            ClassificationPattern {
                doc_type: DocumentType {
                    name: "Facture".to_string(),
                    pattern: String::new(),
                    prefix: "FACT".to_string(),
                },
                required_keywords: vec![
                    "facture".to_string(),
                    "invoice".to_string(),
                ],
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
                required_keywords: vec![
                    "contrat".to_string(),
                    "contract".to_string(),
                ],
                support_keywords: vec![
                    "signataire".to_string(),
                    "signature".to_string(),
                    "clause".to_string(),
                    "article".to_string(),
                    "durée".to_string(),
                    "résiliation".to_string(),
                    "accord".to_string(),
                ],
                blocker_keywords: vec![
                    "facture".to_string(),
                ],
            },
            // RELEVÉ BANCAIRE
            ClassificationPattern {
                doc_type: DocumentType {
                    name: "Relevé bancaire".to_string(),
                    pattern: String::new(),
                    prefix: "BANK".to_string(),
                },
                required_keywords: vec![
                    "relevé".to_string(),
                    "statement".to_string(),
                ],
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
                blocker_keywords: vec![
                    "facture".to_string(),
                ],
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
                blocker_keywords: vec![
                    "facture".to_string(),
                ],
            },
        ];

        Classifier { patterns }
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
            eprintln!("⚠️  Classification incertaine (score: {:.2}%) - Type par défaut utilisé", best_score * 100.0);
            return DocumentType::default();
        }

        eprintln!("✓ Classification: {} (score: {:.2}%)", best_type.name, best_score * 100.0);
        best_type
    }

    fn calculate_score(&self, text: &str, pattern: &ClassificationPattern) -> f32 {
        // 1. Vérifier les bloquants (si présent, score = 0)
        for blocker in &pattern.blocker_keywords {
            if text.contains(blocker) {
                return 0.0;
            }
        }

        // 2. Compter les mots requis (au moins 1 doit être présent)
        let required_count = pattern.required_keywords.iter()
            .filter(|word| text.contains(word.as_str()))
            .count();

        if required_count == 0 {
            return 0.0;
        }

        // 3. Score de base selon les mots requis (70% du score)
        let required_score = required_count as f32 / pattern.required_keywords.len() as f32;

        // 4. Bonus pour les mots de support (30% du score)
        let support_count = pattern.support_keywords.iter()
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

        // Extract dates
        let date_re = Regex::new(r"\d{1,2}[/-]\d{1,2}[/-]\d{2,4}").unwrap();
        if date_re.is_match(text) {
            tags.push("contains_date".to_string());
        }

        // Extract amounts
        let amount_re = Regex::new(r"(?i)\d+[.,]\d{2}\s*(€|eur|euro|\$|usd)").unwrap();
        if amount_re.is_match(text) {
            tags.push("contains_amount".to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_facture() {
        let classifier = Classifier::new();
        let text = "Facture n°12345 - Montant total: 150.00€";
        let result = classifier.classify(text);
        assert_eq!(result.name, "Facture");
        assert_eq!(result.prefix, "FACT");
    }

    #[test]
    fn test_classify_contrat() {
        let classifier = Classifier::new();
        let text = "Contrat de location - Signé le 01/01/2024";
        let result = classifier.classify(text);
        assert_eq!(result.name, "Contrat");
        assert_eq!(result.prefix, "CONT");
    }

    #[test]
    fn test_classify_releve_bancaire() {
        let classifier = Classifier::new();
        let text = "Relevé de compte - IBAN: FR76 1234 5678 9012";
        let result = classifier.classify(text);
        assert_eq!(result.name, "Relevé bancaire");
        assert_eq!(result.prefix, "BANK");
    }

    #[test]
    fn test_classify_bulletin_paie() {
        let classifier = Classifier::new();
        let text = "BULLETIN DE PAIE - Période: Janvier 2024 - Salaire net: 2500.00";
        let result = classifier.classify(text);
        assert_eq!(result.name, "Bulletin de paie");
        assert_eq!(result.prefix, "PAIE");
    }

    #[test]
    fn test_classify_unknown() {
        let classifier = Classifier::new();
        let text = "Ceci est un document sans mots-clés spécifiques";
        let result = classifier.classify(text);
        assert_eq!(result.name, "Unknown");
        assert_eq!(result.prefix, "DOC");
    }

    #[test]
    fn test_extract_tags_date() {
        let classifier = Classifier::new();
        let text = "Document créé le 15/03/2024";
        let tags = classifier.extract_tags(text);
        assert!(tags.contains(&"contains_date".to_string()));
    }

    #[test]
    fn test_extract_tags_amount() {
        let classifier = Classifier::new();
        let text = "Montant: 150.00€";
        let tags = classifier.extract_tags(text);
        assert!(tags.contains(&"contains_amount".to_string()));
    }

    #[test]
    fn test_extract_tags_email() {
        let classifier = Classifier::new();
        let text = "Contact: info@example.com";
        let tags = classifier.extract_tags(text);
        assert!(tags.contains(&"contains_email".to_string()));
    }

    #[test]
    fn test_extract_tags_phone() {
        let classifier = Classifier::new();
        let text = "Téléphone: 01 23 45 67 89";
        let tags = classifier.extract_tags(text);
        assert!(tags.contains(&"contains_phone".to_string()));
    }

    #[test]
    fn test_extract_tags_company() {
        let classifier = Classifier::new();
        let text = "ACME S.A.S - Document officiel";
        let tags = classifier.extract_tags(text);
        assert!(tags.contains(&"company_document".to_string()));
    }

    #[test]
    fn test_extract_tags_multiple() {
        let classifier = Classifier::new();
        let text = "Facture ACME S.A.S - Date: 15/03/2024 - Montant: 150.00€ - Contact: info@acme.com";
        let tags = classifier.extract_tags(text);
        assert!(tags.contains(&"contains_date".to_string()));
        assert!(tags.contains(&"contains_amount".to_string()));
        assert!(tags.contains(&"contains_email".to_string()));
        assert!(tags.contains(&"company_document".to_string()));
    }

    #[test]
    fn test_generate_filename() {
        let classifier = Classifier::new();
        let doc_type = DocumentType {
            name: "Facture".to_string(),
            pattern: "".to_string(),
            prefix: "FACT".to_string(),
        };
        let original_name = "document.pdf";
        let filename = classifier.generate_filename(&doc_type, original_name);

        assert!(filename.starts_with("FACT_"));
        assert!(filename.ends_with(".pdf"));
        assert!(filename.len() > 10); // FACT_ + date + .pdf
    }

    #[test]
    fn test_generate_filename_no_extension() {
        let classifier = Classifier::new();
        let doc_type = DocumentType {
            name: "Test".to_string(),
            pattern: "".to_string(),
            prefix: "TEST".to_string(),
        };
        let original_name = "document_without_extension";
        let filename = classifier.generate_filename(&doc_type, original_name);

        assert!(filename.starts_with("TEST_"));
        assert!(filename.ends_with(".txt")); // Default extension
    }
}
