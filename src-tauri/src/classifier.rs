use regex::Regex;
use chrono::Local;
use crate::models::DocumentType;

pub struct Classifier {
    patterns: Vec<DocumentType>,
}

impl Classifier {
    pub fn new() -> Self {
        let patterns = vec![
            DocumentType {
                name: "Facture".to_string(),
                pattern: r"(?i)(facture|invoice|bill|montant|total|€|\$)".to_string(),
                prefix: "FACT".to_string(),
            },
            DocumentType {
                name: "Contrat".to_string(),
                pattern: r"(?i)(contrat|contract|agreement|accord|signé|signature)".to_string(),
                prefix: "CONT".to_string(),
            },
            DocumentType {
                name: "Relevé bancaire".to_string(),
                pattern: r"(?i)(relevé|bank\s+statement|compte|iban|solde|crédit|débit)".to_string(),
                prefix: "BANK".to_string(),
            },
            DocumentType {
                name: "Bulletin de paie".to_string(),
                pattern: r"(?i)(bulletin\s+de\s+paie|salaire|payslip|net\s+à\s+payer|cotisation)".to_string(),
                prefix: "PAIE".to_string(),
            },
            DocumentType {
                name: "Document officiel".to_string(),
                pattern: r"(?i)(carte\s+identité|passeport|attestation|certificat|permis)".to_string(),
                prefix: "OFFI".to_string(),
            },
            DocumentType {
                name: "Reçu".to_string(),
                pattern: r"(?i)(reçu|receipt|ticket|caisse)".to_string(),
                prefix: "RECU".to_string(),
            },
        ];

        Classifier { patterns }
    }

    pub fn classify(&self, text: &str) -> DocumentType {
        for pattern in &self.patterns {
            if let Ok(re) = Regex::new(&pattern.pattern) {
                if re.is_match(text) {
                    return pattern.clone();
                }
            }
        }

        DocumentType::default()
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
