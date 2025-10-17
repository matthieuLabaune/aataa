# Plan d'amélioration OCR et Classification

## 🎨 Thème visuel - COMPLÉTÉ ✅

### Changements appliqués
- **Couleurs primaires** : Noir (#000000) et Blanc (#FFFFFF)
- **Couleurs secondaires** : Nuances de gris (#424242, #616161, #BDBDBD, #E0E0E0)
- **Surface** : Blanc (#FFFFFF) avec backgrounds gris clair (#FAFAFA, #F5F5F5)
- **Erreurs** : Rouge Material (#D32F2F) pour les actions dangereuses
- **Thème** : Minimaliste, épuré, professionnel

### Palette de gris
```
#000000 - Noir pur (texte principal, boutons primaires)
#212121 - Presque noir (inverse surface)
#424242 - Gris foncé (secondary)
#616161 - Gris moyen (tertiary, on-surface-variant)
#BDBDBD - Gris clair (outline)
#E0E0E0 - Gris très clair (containers)
#F5F5F5 - Gris ultra-clair (surface-variant)
#FAFAFA - Presque blanc (background)
#FFFFFF - Blanc pur (surface, on-primary)
```

---

## 🔍 Problème actuel : Classification erronée

### Symptômes
- Certaines images sont classées en "Facture" alors qu'elles ne le sont pas
- Le classificateur actuel utilise des mots-clés simples
- Pas de distinction entre types de documents similaires

### Causes probables
1. **Mots-clés trop larges** : "montant", "total", "date" apparaissent dans beaucoup de documents
2. **Manque de contexte** : Ne prend pas en compte la structure du document
3. **Ordre de priorité** : Le premier type qui match gagne, même si peu pertinent
4. **Pas de score de confiance** : Impossible de savoir si la classification est fiable

---

## 🎯 Solutions proposées

### Solution 1 : Améliorer les patterns de classification (Court terme)

**Fichier** : `src-tauri/src/classifier.rs`

#### A. Patterns plus précis pour les factures
Au lieu de :
```rust
"facture" => vec!["facture", "invoice", "total", "montant", "tva"]
```

Utiliser des patterns combinés :
```rust
"facture" => {
    // Mots obligatoires (au moins 2 sur 3)
    required: vec!["facture", "invoice", "total"],
    // Mots de support
    support: vec!["montant", "tva", "ht", "ttc", "n°", "numéro"],
    // Mots bloquants (si présents, ce n'est PAS une facture)
    blockers: vec!["contrat", "bulletin", "attestation", "certificat"]
}
```

#### B. Scoring au lieu de match simple
```rust
pub fn classify_with_confidence(&self, text: &str) -> (String, f32) {
    let mut scores: HashMap<String, f32> = HashMap::new();
    
    for (doc_type, patterns) in &self.patterns {
        let score = calculate_score(text, patterns);
        scores.insert(doc_type.clone(), score);
    }
    
    // Retourner le type avec le meilleur score
    let best = scores.iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    
    (best.0.clone(), *best.1)
}
```

#### C. Seuil de confiance minimum
```rust
const MIN_CONFIDENCE: f32 = 0.6; // 60% de confiance minimum

if confidence < MIN_CONFIDENCE {
    return "Document".to_string(); // Type générique si incertain
}
```

### Solution 2 : Analyse structurelle (Moyen terme)

**Objectif** : Analyser la structure du document, pas seulement le texte

#### A. Détection de structure
```rust
struct DocumentStructure {
    has_header: bool,
    has_table: bool,
    has_amounts: bool,
    has_signature_zone: bool,
    line_count: usize,
    number_density: f32, // % de lignes contenant des nombres
}

fn analyze_structure(text: &str) -> DocumentStructure {
    // Analyser les lignes
    // Détecter les tableaux (alignements, colonnes)
    // Détecter les zones de montants
    // etc.
}
```

#### B. Classification hybride
```rust
pub fn classify_hybrid(&self, text: &str) -> String {
    let structure = analyze_structure(text);
    let keyword_score = classify_by_keywords(text);
    
    // Combiner les deux approches
    if structure.has_table && structure.has_amounts && keyword_score["facture"] > 0.5 {
        return "Facture".to_string();
    }
    
    // Autres règles...
}
```

### Solution 3 : Machine Learning (Long terme)

**Objectif** : Entraîner un modèle spécifique sur vos documents

#### Technologies possibles
1. **Tesseract + Modèle custom** : Entraîner Tesseract sur vos types de documents
2. **Document Classification Model** : Utiliser un modèle BERT/DistilBERT fine-tuné
3. **Layout Analysis** : Modèles comme LayoutLM qui comprennent la mise en page

#### Implémentation
```rust
// Utiliser onnxruntime-rs pour charger un modèle ML
use onnxruntime::{GraphOptimizationLevel, environment::Environment};

pub struct MLClassifier {
    session: onnxruntime::Session,
}

impl MLClassifier {
    pub fn classify(&self, text: &str) -> (String, f32) {
        // Tokenizer le texte
        // Passer au modèle
        // Retourner la prédiction
    }
}
```

---

## 📝 Plan d'action recommandé

### Phase 1 : Amélioration immédiate (1-2h)
- [x] Changer le thème en noir et blanc
- [ ] Améliorer les patterns dans `classifier.rs`
- [ ] Ajouter un système de scoring
- [ ] Implémenter un seuil de confiance
- [ ] Ajouter des mots bloquants pour éviter les faux positifs

### Phase 2 : Analyse structurelle (2-4h)
- [ ] Créer la structure `DocumentStructure`
- [ ] Implémenter `analyze_structure()`
- [ ] Détecter les tableaux dans le texte OCR
- [ ] Compter les lignes avec des montants
- [ ] Combiner analyse structurelle + mots-clés

### Phase 3 : Interface utilisateur (1-2h)
- [ ] Afficher le score de confiance dans l'UI
- [ ] Permettre à l'utilisateur de corriger la classification
- [ ] Sauvegarder les corrections pour améliorer le système
- [ ] Ajouter un indicateur visuel de confiance (faible/moyen/élevé)

### Phase 4 : Machine Learning (Optionnel, 1-2 semaines)
- [ ] Collecter un dataset d'entraînement
- [ ] Fine-tuner un modèle de classification
- [ ] Intégrer le modèle dans Tauri
- [ ] Tester et valider la précision

---

## 🛠️ Code à modifier immédiatement

### 1. Fichier `src-tauri/src/classifier.rs`

```rust
use std::collections::HashMap;

pub struct DocumentClassifier {
    patterns: HashMap<String, ClassificationPattern>,
}

pub struct ClassificationPattern {
    required: Vec<String>,    // Au moins 2 doivent être présents
    support: Vec<String>,      // Augmentent le score
    blockers: Vec<String>,     // Si présents, score = 0
    weight: f32,               // Importance relative
}

impl DocumentClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // FACTURE - Pattern strict
        patterns.insert("Facture".to_string(), ClassificationPattern {
            required: vec![
                "facture".to_string(),
                "invoice".to_string(),
                "total".to_string(),
            ],
            support: vec![
                "montant".to_string(),
                "tva".to_string(),
                "ht".to_string(),
                "ttc".to_string(),
                "n°".to_string(),
                "numéro".to_string(),
                "payer".to_string(),
            ],
            blockers: vec![
                "contrat".to_string(),
                "bulletin".to_string(),
                "attestation".to_string(),
            ],
            weight: 1.0,
        });

        // CONTRAT
        patterns.insert("Contrat".to_string(), ClassificationPattern {
            required: vec![
                "contrat".to_string(),
                "contract".to_string(),
                "signataire".to_string(),
            ],
            support: vec![
                "clause".to_string(),
                "article".to_string(),
                "durée".to_string(),
                "résiliation".to_string(),
            ],
            blockers: vec!["facture".to_string()],
            weight: 1.0,
        });

        // Autres types...

        Self { patterns }
    }

    pub fn classify_with_confidence(&self, text: &str) -> (String, f32) {
        let text_lower = text.to_lowercase();
        let mut best_type = "Document".to_string();
        let mut best_score = 0.0f32;

        for (doc_type, pattern) in &self.patterns {
            let score = self.calculate_score(&text_lower, pattern);
            
            if score > best_score {
                best_score = score;
                best_type = doc_type.clone();
            }
        }

        // Seuil minimum de confiance
        if best_score < 0.6 {
            return ("Document".to_string(), best_score);
        }

        (best_type, best_score)
    }

    fn calculate_score(&self, text: &str, pattern: &ClassificationPattern) -> f32 {
        // Vérifier les bloquants
        for blocker in &pattern.blockers {
            if text.contains(blocker) {
                return 0.0;
            }
        }

        // Compter les mots requis
        let required_count = pattern.required.iter()
            .filter(|word| text.contains(word.as_str()))
            .count();

        // Au moins 2 mots requis sur 3, ou 1 si < 3 mots requis
        let required_threshold = if pattern.required.len() >= 3 { 2 } else { 1 };
        if required_count < required_threshold {
            return 0.0;
        }

        // Score de base selon les mots requis
        let required_score = required_count as f32 / pattern.required.len() as f32;

        // Bonus pour les mots de support
        let support_count = pattern.support.iter()
            .filter(|word| text.contains(word.as_str()))
            .count();
        let support_score = (support_count as f32 / pattern.support.len() as f32) * 0.3;

        // Score final
        let final_score = (required_score * 0.7 + support_score) * pattern.weight;

        final_score.min(1.0)
    }
}
```

### 2. Modifier `src-tauri/src/commands.rs`

```rust
// Ligne ~65-70, modifier la classification
let (doc_type, confidence) = state.classifier.classify_with_confidence(&ocr_text);

// Si confiance faible, log un warning
if confidence < 0.7 {
    eprintln!("Warning: Low confidence classification ({:.2}%) for {}", 
              confidence * 100.0, &original_name);
}
```

### 3. Ajouter le score dans le modèle (optionnel pour plus tard)

```rust
// src-tauri/src/models.rs
pub struct Document {
    // ... champs existants
    pub classification_confidence: Option<f32>, // Score 0.0-1.0
}
```

---

## 🧪 Tests recommandés

### Scénarios de test
1. **Facture valide** : Doit être classée "Facture" avec >80% confiance
2. **Contrat** : Ne doit PAS être classé "Facture" même s'il contient "total"
3. **Bulletin de salaire** : Doit avoir son propre type, pas "Facture"
4. **Document générique** : Si incertain, doit être classé "Document"

### Commandes de test
```bash
# Tester avec un fichier spécifique
cargo test --package app --lib classifier::tests

# Tester la classification
cargo run --bin classify_test -- test_images/facture.pdf
```

---

## 📊 Métriques de succès

### Objectifs
- **Précision** : >90% de classifications correctes
- **Faux positifs** : <5% (documents mal classés en Facture)
- **Confiance** : Score moyen >75%
- **Fallback** : Documents incertains → "Document" générique

### Monitoring
- Logger tous les scores de confiance
- Permettre feedback utilisateur
- Analyser les patterns de mauvaise classification

---

## 🚀 Prochaines étapes

1. **Appliquer Solution 1** (amélioration patterns)
2. **Tester avec vos documents problématiques**
3. **Ajuster les seuils et patterns**
4. **Si nécessaire, passer à Solution 2** (analyse structurelle)

Voulez-vous que je commence par implémenter la Solution 1 dans `classifier.rs` ?
