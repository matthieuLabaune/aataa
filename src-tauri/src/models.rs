use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub original_name: String,
    pub new_name: String,
    pub file_path: String,
    pub document_type: String,
    pub tags: Vec<String>,
    pub ocr_text: String,
    pub created_at: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentType {
    pub name: String,
    pub pattern: String,
    pub prefix: String,
}

impl Default for DocumentType {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            pattern: "".to_string(),
            prefix: "DOC".to_string(),
        }
    }
}
