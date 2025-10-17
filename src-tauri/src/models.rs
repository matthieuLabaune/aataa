use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub original_name: String,
    pub new_name: String,
    pub file_path: String,
    pub document_type: String,
    pub category: String,           // NEW: Catégorie principale (niveau 1)
    pub subcategory: Option<String>, // NEW: Sous-catégorie (niveau 2)
    pub confidence: Option<f32>,     // Score de confiance de classification (0.0-1.0)
    pub tags: Vec<String>,           // Niveau 3: tags libres
    pub ocr_text: String,
    pub created_at: String,
    pub file_size: u64,
    pub notes: Option<String>,
    pub deleted_at: Option<String>, // NULL = actif, Date ISO = supprimé
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

// ===== NOUVEAU SYSTÈME DE CATÉGORISATION =====

/// Catégorie principale (Niveau 1) - 8 types prédéfinis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MainCategory {
    Administratif,
    Financier,
    Sante,
    Professionnel,
    Immobilier,
    Academique,
    Personnel,
    Autre,
}

impl MainCategory {
    pub fn to_string(&self) -> String {
        match self {
            MainCategory::Administratif => "Administratif".to_string(),
            MainCategory::Financier => "Financier".to_string(),
            MainCategory::Sante => "Santé".to_string(),
            MainCategory::Professionnel => "Professionnel".to_string(),
            MainCategory::Immobilier => "Immobilier".to_string(),
            MainCategory::Academique => "Académique".to_string(),
            MainCategory::Personnel => "Personnel".to_string(),
            MainCategory::Autre => "Autre".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn prefix(&self) -> &str {
        match self {
            MainCategory::Administratif => "ADM",
            MainCategory::Financier => "FIN",
            MainCategory::Sante => "SAN",
            MainCategory::Professionnel => "PRO",
            MainCategory::Immobilier => "IMM",
            MainCategory::Academique => "ACA",
            MainCategory::Personnel => "PER",
            MainCategory::Autre => "DOC",
        }
    }

    #[allow(dead_code)]
    pub fn icon(&self) -> &str {
        match self {
            MainCategory::Administratif => "description",
            MainCategory::Financier => "account_balance",
            MainCategory::Sante => "medical_services",
            MainCategory::Professionnel => "work",
            MainCategory::Immobilier => "home",
            MainCategory::Academique => "school",
            MainCategory::Personnel => "person",
            MainCategory::Autre => "folder",
        }
    }

    pub fn all() -> Vec<MainCategory> {
        vec![
            MainCategory::Administratif,
            MainCategory::Financier,
            MainCategory::Sante,
            MainCategory::Professionnel,
            MainCategory::Immobilier,
            MainCategory::Academique,
            MainCategory::Personnel,
            MainCategory::Autre,
        ]
    }
}

impl Default for MainCategory {
    fn default() -> Self {
        MainCategory::Autre
    }
}

/// Sous-catégorie (Niveau 2) - Personnalisable par l'utilisateur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subcategory {
    pub id: i64,
    pub category: MainCategory,
    pub name: String,
    pub is_predefined: bool,  // Fournie par défaut ou créée par user
}

/// Tag (Niveau 3) - Libre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub count: i32, // Nombre de documents avec ce tag
}

/// Résultat de classification enrichi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub category: MainCategory,
    pub subcategory: Option<String>,
    pub suggested_tags: Vec<String>,
    pub confidence: f32,
}

/// Mot-clé de classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationKeyword {
    pub id: i64,
    pub category: String,
    pub subcategory: Option<String>,
    pub keyword: String,
    pub weight: f64,
}
