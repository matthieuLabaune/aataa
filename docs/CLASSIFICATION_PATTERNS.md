# Patterns de Classification AATAA

Ce document décrit tous les patterns de classification et tags utilisés par AATAA.

## Types de Documents

### 1. Facture (FACT)
**Pattern regex** : `(?i)(facture|invoice|bill|montant|total|€|\$)`

**Mots-clés détectés** :
- facture
- invoice
- bill
- montant
- total
- € / $

**Exemples de texte** :
```
FACTURE N° 2024-001
Montant HT: 100.00 €
TVA: 20.00 €
Total TTC: 120.00 €
```

---

### 2. Contrat (CONT)
**Pattern regex** : `(?i)(contrat|contract|agreement|accord|signé|signature)`

**Mots-clés détectés** :
- contrat
- contract
- agreement
- accord
- signé
- signature

**Exemples de texte** :
```
CONTRAT DE PRESTATI ON
Entre les soussignés...
Signature :
```

---

### 3. Relevé bancaire (BANK)
**Pattern regex** : `(?i)(relevé|bank\s+statement|compte|iban|solde|crédit|débit)`

**Mots-clés détectés** :
- relevé
- bank statement
- compte
- iban
- solde
- crédit
- débit

**Exemples de texte** :
```
RELEVÉ DE COMPTE
IBAN: FR76 1234 5678 9012 3456 7890 123
Solde: 1,234.56 €
```

---

### 4. Bulletin de paie (PAIE)
**Pattern regex** : `(?i)(bulletin\s+de\s+paie|salaire|payslip|net\s+à\s+payer|cotisation)`

**Mots-clés détectés** :
- bulletin de paie
- salaire
- payslip
- net à payer
- cotisation

**Exemples de texte** :
```
BULLETIN DE PAIE
Salaire brut: 2,500.00 €
Cotisations: 500.00 €
Net à payer: 2,000.00 €
```

---

### 5. Document officiel (OFFI)
**Pattern regex** : `(?i)(carte\s+identité|passeport|attestation|certificat|permis)`

**Mots-clés détectés** :
- carte identité
- passeport
- attestation
- certificat
- permis

**Exemples de texte** :
```
CARTE NATIONALE D'IDENTITÉ
RÉPUBLIQUE FRANÇAISE
PASSEPORT
ATTESTATION SUR L'HONNEUR
```

---

### 6. Reçu (RECU)
**Pattern regex** : `(?i)(reçu|receipt|ticket|caisse)`

**Mots-clés détectés** :
- reçu
- receipt
- ticket
- caisse

**Exemples de texte** :
```
TICKET DE CAISSE
REÇU
Total: 45.50 €
```

---

### 7. Unknown (DOC)
**Par défaut** si aucun pattern ne correspond.

---

## Tags Automatiques

### contains_date
**Pattern regex** : `\d{1,2}[/-]\d{1,2}[/-]\d{2,4}`

**Formats détectés** :
- DD/MM/YYYY
- DD-MM-YYYY
- D/M/YY
- DD/MM/YY

**Exemples** :
- 09/10/2024
- 9-10-24
- 01/01/2025

---

### contains_amount
**Pattern regex** : `(?i)\d+[.,]\d{2}\s*(€|eur|euro|\$|usd)`

**Formats détectés** :
- 123.45 €
- 1,234.56 EUR
- 99.99 $
- 1 234,56 euro

**Exemples** :
- Montant: 120.00 €
- Total: 1,250.50 EUR
- Prix: 99.99$

---

### company_document
**Pattern regex** : `(?i)(s\.a\.s|sarl|sas|sa|eurl|sci)\b`

**Formes juridiques détectées** :
- S.A.S
- SARL
- SAS
- SA
- EURL
- SCI

**Exemples** :
- ACME SARL
- Tech Solutions SAS
- Consulting S.A.S

---

### contains_email
**Pattern regex** : `\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b`

**Formats détectés** :
- Standard : contact@example.com
- Sous-domaine : info@sub.example.com
- Numérique : user123@domain.org

**Exemples** :
- contact@entreprise.fr
- john.doe@company.com
- support@service.gouv.fr

---

### contains_phone
**Pattern regex** : `(?i)(\+33|0)[1-9](\s?\d{2}){4}`

**Formats détectés** :
- Format français : 0X XX XX XX XX
- International : +33 X XX XX XX XX
- Sans espaces : 0XXXXXXXXX

**Exemples** :
- 01 23 45 67 89
- +33 6 12 34 56 78
- 0612345678

---

## Personnalisation

### Ajouter un nouveau type

Éditez `src-tauri/src/classifier.rs` :

```rust
DocumentType {
    name: "Nouveau Type".to_string(),
    pattern: r"(?i)(mot-clé1|mot-clé2|mot-clé3)".to_string(),
    prefix: "TYPE".to_string(),
}
```

### Ajouter un nouveau tag

Éditez la méthode `extract_tags()` dans `classifier.rs` :

```rust
let new_tag_re = Regex::new(r"votre_pattern").unwrap();
if new_tag_re.is_match(text) {
    tags.push("nouveau_tag".to_string());
}
```

---

## Bonnes Pratiques

### Pour les patterns
1. **Case insensitive** : Toujours utiliser `(?i)` au début
2. **Alternation** : Utiliser `|` pour plusieurs mots-clés
3. **Word boundaries** : Utiliser `\b` pour éviter les faux positifs
4. **Spaces** : Utiliser `\s+` pour les espaces variables

### Pour les tags
1. **Noms descriptifs** : `contains_*` pour les contenus, `is_*` pour les états
2. **Snake_case** : Utiliser des underscores
3. **Anglais** : Pour la cohérence du code
4. **Spécifiques** : Éviter les tags trop génériques

---

## Exemples de Tests

### Test Facture
```
Input:
FACTURE N° 2024-001
Date: 15/10/2024
Montant HT: 100.00 €
TVA: 20.00 €
Total TTC: 120.00 €

Output:
Type: Facture
Prefix: FACT
Tags: [contains_date, contains_amount]
```

### Test Relevé Bancaire
```
Input:
RELEVÉ DE COMPTE
IBAN: FR76 1234 5678 9012 3456 7890 123
Solde: 1,234.56 €
Contact: support@banque.fr
Tél: 01 23 45 67 89

Output:
Type: Relevé bancaire
Prefix: BANK
Tags: [contains_amount, contains_email, contains_phone]
```

### Test Document d'entreprise
```
Input:
ACME SARL
123 Rue de Paris
contact@acme.fr
+33 1 23 45 67 89

Output:
Type: Unknown (pas de pattern spécifique)
Prefix: DOC
Tags: [company_document, contains_email, contains_phone]
```

---

## Statistiques de Classification

### Taux de succès par type (exemples théoriques)
- Factures : ~95% (très caractéristiques)
- Contrats : ~85% (variété de formats)
- Relevés bancaires : ~90% (IBAN souvent présent)
- Bulletins de paie : ~80% (termes spécifiques)
- Documents officiels : ~70% (grande variété)
- Reçus : ~75% (courts et simples)

### Amélioration continue
Pour améliorer la classification :
1. Collecter des exemples de faux négatifs
2. Analyser les patterns manqués
3. Ajouter des mots-clés aux regex
4. Tester avec de nouveaux documents

---

## Références

- **Regex101** : https://regex101.com/ (tester vos patterns)
- **Rust Regex Crate** : https://docs.rs/regex/
- **Tesseract OCR** : https://github.com/tesseract-ocr/tesseract

---

**Note** : Les patterns regex sont en français et anglais pour une meilleure couverture internationale.
