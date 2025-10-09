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
