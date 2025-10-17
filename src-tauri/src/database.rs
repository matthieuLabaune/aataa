use crate::models::Document;
use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                original_name TEXT NOT NULL,
                new_name TEXT NOT NULL,
                file_path TEXT NOT NULL,
                document_type TEXT NOT NULL,
                tags TEXT NOT NULL,
                ocr_text TEXT,
                created_at TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                notes TEXT,
                deleted_at TEXT
            )",
            [],
        )?;

        // Add notes column if it doesn't exist (migration)
        let _ = conn.execute("ALTER TABLE documents ADD COLUMN notes TEXT", []);

        // Add deleted_at column if it doesn't exist (migration)
        let _ = conn.execute("ALTER TABLE documents ADD COLUMN deleted_at TEXT", []);

        // Add category column if it doesn't exist (NEW - migration)
        let _ = conn.execute("ALTER TABLE documents ADD COLUMN category TEXT DEFAULT 'Autre'", []);

        // Add subcategory column if it doesn't exist (NEW - migration)
        let _ = conn.execute("ALTER TABLE documents ADD COLUMN subcategory TEXT", []);

        // Add confidence column if it doesn't exist (NEW - migration)
        let _ = conn.execute("ALTER TABLE documents ADD COLUMN confidence REAL", []);

        // Créer table des sous-catégories (NEW)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS subcategories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT NOT NULL,
                name TEXT NOT NULL,
                is_predefined INTEGER DEFAULT 0,
                UNIQUE(category, name)
            )",
            [],
        )?;

        // Insérer les sous-catégories prédéfinies si la table est vide
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM subcategories", [], |row| row.get(0))?;
        if count == 0 {
            Self::insert_default_subcategories(&conn)?;
        }

        // Créer table des mots-clés de classification (NEW)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS classification_keywords (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT NOT NULL,
                subcategory TEXT,
                keyword TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                UNIQUE(category, subcategory, keyword)
            )",
            [],
        )?;

        // Insérer les mots-clés par défaut si la table est vide
        let kw_count: i64 = conn.query_row("SELECT COUNT(*) FROM classification_keywords", [], |row| row.get(0))?;
        if kw_count == 0 {
            Self::insert_default_keywords(&conn)?;
        }

        Ok(Database { conn })
    }

    fn insert_default_subcategories(conn: &Connection) -> Result<()> {
        let defaults = vec![
            // Administratif
            ("Administratif", "Carte d'identité", true),
            ("Administratif", "Passeport", true),
            ("Administratif", "Avis d'imposition", true),
            ("Administratif", "Déclaration fiscale", true),
            // Financier
            ("Financier", "Facture fournisseur", true),
            ("Financier", "Facture énergie", true),
            ("Financier", "Facture télécom", true),
            ("Financier", "Relevé bancaire", true),
            ("Financier", "Contrat", true),
            // Santé
            ("Santé", "Ordonnance", true),
            ("Santé", "Résultat analyse", true),
            ("Santé", "Remboursement sécu", true),
            ("Santé", "Remboursement mutuelle", true),
            // Professionnel
            ("Professionnel", "Contrat de travail", true),
            ("Professionnel", "Fiche de paie", true),
            ("Professionnel", "Note de frais", true),
            // Immobilier
            ("Immobilier", "Acte de propriété", true),
            ("Immobilier", "Bail location", true),
            ("Immobilier", "Diagnostic", true),
            // Académique
            ("Académique", "Article publié", true),
            ("Académique", "Article soumis", true),
            ("Académique", "Thèse", true),
            ("Académique", "Diplôme", true),
            // Personnel
            ("Personnel", "Ticket de caisse", true),
            ("Personnel", "Reçu", true),
            ("Personnel", "Courrier", true),
        ];

        for (category, name, is_predefined) in defaults {
            conn.execute(
                "INSERT OR IGNORE INTO subcategories (category, name, is_predefined) VALUES (?1, ?2, ?3)",
                rusqlite::params![category, name, is_predefined as i32],
            )?;
        }

        Ok(())
    }

    fn insert_default_keywords(conn: &Connection) -> Result<()> {
        let keywords: Vec<(&str, Option<&str>, &str, f64)> = vec![
            // Administratif (poids: 1.0 = normal, 1.5 = important, 0.5 = faible)
            ("Administratif", None, "impôt", 1.5),
            ("Administratif", None, "taxe", 1.5),
            ("Administratif", None, "fiscal", 1.5),
            ("Administratif", None, "identité", 1.5),
            ("Administratif", None, "passeport", 1.5),
            ("Administratif", None, "carte", 1.0),
            ("Administratif", None, "déclaration", 1.0),
            ("Administratif", None, "administration", 1.0),
            ("Administratif", None, "officiel", 1.0),
            
            // Financier
            ("Financier", None, "facture", 2.0),
            ("Financier", None, "invoice", 2.0),
            ("Financier", None, "montant", 1.5),
            ("Financier", None, "total", 1.5),
            ("Financier", None, "tva", 1.5),
            ("Financier", None, "ht", 1.5),
            ("Financier", None, "ttc", 1.5),
            ("Financier", None, "paiement", 1.5),
            ("Financier", None, "banque", 1.5),
            ("Financier", None, "relevé", 1.5),
            ("Financier", None, "compte", 1.0),
            ("Financier", None, "crédit", 1.0),
            ("Financier", None, "débit", 1.0),
            ("Financier", None, "solde", 1.0),
            ("Financier", None, "contrat", 1.0),
            ("Financier", None, "€", 1.0),
            ("Financier", None, "eur", 1.0),
            
            // Santé
            ("Santé", None, "médecin", 1.5),
            ("Santé", None, "docteur", 1.5),
            ("Santé", None, "ordonnance", 2.0),
            ("Santé", None, "prescription", 2.0),
            ("Santé", None, "médicament", 1.5),
            ("Santé", None, "pharmacie", 1.5),
            ("Santé", None, "analyse", 1.5),
            ("Santé", None, "laboratoire", 1.5),
            ("Santé", None, "sécurité sociale", 2.0),
            ("Santé", None, "mutuelle", 1.5),
            ("Santé", None, "remboursement", 1.5),
            ("Santé", None, "cpam", 1.5),
            ("Santé", None, "patient", 1.0),
            ("Santé", None, "consultation", 1.0),
            
            // Professionnel
            ("Professionnel", None, "salaire", 1.5),
            ("Professionnel", None, "paie", 1.5),
            ("Professionnel", None, "employeur", 1.5),
            ("Professionnel", None, "salarié", 1.5),
            ("Professionnel", None, "travail", 1.0),
            ("Professionnel", None, "contrat", 1.0),
            ("Professionnel", None, "entreprise", 1.0),
            ("Professionnel", None, "société", 1.0),
            ("Professionnel", None, "note de frais", 2.0),
            ("Professionnel", None, "frais", 1.0),
            
            // Immobilier
            ("Immobilier", None, "propriété", 1.5),
            ("Immobilier", None, "immobilier", 2.0),
            ("Immobilier", None, "location", 1.5),
            ("Immobilier", None, "loyer", 1.5),
            ("Immobilier", None, "bail", 1.5),
            ("Immobilier", None, "appartement", 1.0),
            ("Immobilier", None, "maison", 1.0),
            ("Immobilier", None, "logement", 1.0),
            ("Immobilier", None, "diagnostic", 1.5),
            ("Immobilier", None, "copropriété", 1.0),
            
            // Académique
            ("Académique", None, "université", 1.5),
            ("Académique", None, "diplôme", 1.5),
            ("Académique", None, "thèse", 1.5),
            ("Académique", None, "article", 1.0),
            ("Académique", None, "publication", 1.5),
            ("Académique", None, "recherche", 1.0),
            ("Académique", None, "étudiant", 1.0),
            ("Académique", None, "école", 1.0),
            ("Académique", None, "formation", 1.0),
            
            // Personnel
            ("Personnel", None, "ticket", 1.0),
            ("Personnel", None, "reçu", 1.0),
            ("Personnel", None, "courrier", 1.0),
            ("Personnel", None, "personnel", 1.0),
        ];

        for (category, subcategory, keyword, weight) in keywords {
            conn.execute(
                "INSERT OR IGNORE INTO classification_keywords (category, subcategory, keyword, weight) 
                 VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![category, subcategory, keyword, weight],
            )?;
        }

        Ok(())
    }

    pub fn insert_document(&self, doc: &Document) -> Result<()> {
        self.conn.execute(
            "INSERT INTO documents (id, original_name, new_name, file_path, document_type, category, subcategory, confidence, tags, ocr_text, created_at, file_size, notes, deleted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![
                doc.id,
                doc.original_name,
                doc.new_name,
                doc.file_path,
                doc.document_type,
                doc.category,
                doc.subcategory,
                doc.confidence,
                serde_json::to_string(&doc.tags).unwrap_or_default(),
                doc.ocr_text,
                doc.created_at,
                doc.file_size as i64,
                doc.notes,
                doc.deleted_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_all_documents(&self) -> Result<Vec<Document>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, original_name, new_name, file_path, document_type, category, subcategory, confidence, tags, ocr_text, created_at, file_size, notes, deleted_at
             FROM documents WHERE deleted_at IS NULL ORDER BY created_at DESC"
        )?;

        let documents = stmt
            .query_map([], |row| {
                Ok(Document {
                    id: row.get(0)?,
                    original_name: row.get(1)?,
                    new_name: row.get(2)?,
                    file_path: row.get(3)?,
                    document_type: row.get(4)?,
                    category: row.get::<_, Option<String>>(5)?.unwrap_or_else(|| "Autre".to_string()),
                    subcategory: row.get(6).ok(),
                    confidence: row.get(7).ok(),
                    tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                    ocr_text: row.get(9)?,
                    created_at: row.get(10)?,
                    file_size: row.get::<_, i64>(11)? as u64,
                    notes: row.get(12).ok(),
                    deleted_at: row.get(13).ok(),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(documents)
    }

    pub fn search_documents(&self, query: &str) -> Result<Vec<Document>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, original_name, new_name, file_path, document_type, category, subcategory, confidence, tags, ocr_text, created_at, file_size, notes, deleted_at
             FROM documents
             WHERE (ocr_text LIKE ?1 OR new_name LIKE ?1 OR tags LIKE ?1 OR notes LIKE ?1)
             AND deleted_at IS NULL
             ORDER BY created_at DESC"
        )?;

        let search_query = format!("%{}%", query);
        let documents = stmt
            .query_map([&search_query], |row| {
                Ok(Document {
                    id: row.get(0)?,
                    original_name: row.get(1)?,
                    new_name: row.get(2)?,
                    file_path: row.get(3)?,
                    document_type: row.get(4)?,
                    category: row.get::<_, Option<String>>(5)?.unwrap_or_else(|| "Autre".to_string()),
                    subcategory: row.get(6).ok(),
                    confidence: row.get(7).ok(),
                    tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                    ocr_text: row.get(9)?,
                    created_at: row.get(10)?,
                    file_size: row.get::<_, i64>(11)? as u64,
                    notes: row.get(12).ok(),
                    deleted_at: row.get(13).ok(),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(documents)
    }

    // Soft delete: marque le document comme supprimé
    pub fn delete_document(&self, id: &str) -> Result<()> {
        let deleted_at = chrono::Local::now().to_rfc3339();
        self.conn.execute(
            "UPDATE documents SET deleted_at = ?1 WHERE id = ?2",
            rusqlite::params![deleted_at, id],
        )?;
        Ok(())
    }

    // Récupère les documents supprimés (corbeille)
    pub fn get_deleted_documents(&self) -> Result<Vec<Document>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, original_name, new_name, file_path, document_type, category, subcategory, confidence, tags, ocr_text, created_at, file_size, notes, deleted_at
             FROM documents WHERE deleted_at IS NOT NULL ORDER BY deleted_at DESC"
        )?;

        let documents = stmt
            .query_map([], |row| {
                Ok(Document {
                    id: row.get(0)?,
                    original_name: row.get(1)?,
                    new_name: row.get(2)?,
                    file_path: row.get(3)?,
                    document_type: row.get(4)?,
                    category: row.get::<_, Option<String>>(5)?.unwrap_or_else(|| "Autre".to_string()),
                    subcategory: row.get(6).ok(),
                    confidence: row.get(7).ok(),
                    tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                    ocr_text: row.get(9)?,
                    created_at: row.get(10)?,
                    file_size: row.get::<_, i64>(11)? as u64,
                    notes: row.get(12).ok(),
                    deleted_at: row.get(13).ok(),
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(documents)
    }

    // Restaure un document supprimé
    pub fn restore_document(&self, id: &str) -> Result<()> {
        self.conn
            .execute("UPDATE documents SET deleted_at = NULL WHERE id = ?1", [id])?;
        Ok(())
    }

    // Supprime définitivement un document (hard delete)
    pub fn permanently_delete_document(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM documents WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_document_notes(&self, id: &str, notes: Option<String>) -> Result<()> {
        self.conn.execute(
            "UPDATE documents SET notes = ?1 WHERE id = ?2",
            rusqlite::params![notes, id],
        )?;
        Ok(())
    }

    pub fn update_document_metadata(
        &self,
        id: &str,
        document_type: &str,
        tags: &Vec<String>,
        new_name: &str,
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE documents SET document_type = ?1, tags = ?2, new_name = ?3 WHERE id = ?4",
            rusqlite::params![
                document_type,
                serde_json::to_string(tags).unwrap_or_default(),
                new_name,
                id
            ],
        )?;
        Ok(())
    }

    // ===== NOUVELLES MÉTHODES POUR SOUS-CATÉGORIES =====

    pub fn get_subcategories(&self, category: Option<&str>) -> Result<Vec<crate::models::Subcategory>> {
        let query = if let Some(cat) = category {
            format!("SELECT id, category, name, is_predefined FROM subcategories WHERE category = '{}' ORDER BY name", cat)
        } else {
            "SELECT id, category, name, is_predefined FROM subcategories ORDER BY category, name".to_string()
        };

        let mut stmt = self.conn.prepare(&query)?;
        
        let subcategories = stmt
            .query_map([], |row| {
                let category_str: String = row.get(1)?;
                let category = match category_str.as_str() {
                    "Administratif" => crate::models::MainCategory::Administratif,
                    "Financier" => crate::models::MainCategory::Financier,
                    "Santé" => crate::models::MainCategory::Sante,
                    "Professionnel" => crate::models::MainCategory::Professionnel,
                    "Immobilier" => crate::models::MainCategory::Immobilier,
                    "Académique" => crate::models::MainCategory::Academique,
                    "Personnel" => crate::models::MainCategory::Personnel,
                    _ => crate::models::MainCategory::Autre,
                };

                Ok(crate::models::Subcategory {
                    id: row.get(0)?,
                    category,
                    name: row.get(2)?,
                    is_predefined: row.get::<_, i32>(3)? != 0,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(subcategories)
    }

    pub fn add_subcategory(&self, category: &str, name: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO subcategories (category, name, is_predefined) VALUES (?1, ?2, 0)",
            rusqlite::params![category, name],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn delete_subcategory(&self, id: i64) -> Result<()> {
        // Ne peut supprimer que les sous-catégories créées par l'utilisateur
        self.conn.execute(
            "DELETE FROM subcategories WHERE id = ?1 AND is_predefined = 0",
            [id],
        )?;
        Ok(())
    }

    pub fn get_all_tags(&self) -> Result<Vec<crate::models::Tag>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT json_each.value as tag, COUNT(*) as count
             FROM documents, json_each(documents.tags)
             WHERE deleted_at IS NULL
             GROUP BY tag
             ORDER BY count DESC, tag"
        )?;

        let tags = stmt
            .query_map([], |row| {
                Ok(crate::models::Tag {
                    name: row.get(0)?,
                    count: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(tags)
    }

    // ========== Classification Keywords Methods ==========

    pub fn get_classification_keywords(&self, category: Option<&str>) -> Result<Vec<crate::models::ClassificationKeyword>> {
        let query = if let Some(cat) = category {
            format!("SELECT id, category, subcategory, keyword, weight FROM classification_keywords WHERE category = '{}' ORDER BY weight DESC, keyword", cat)
        } else {
            "SELECT id, category, subcategory, keyword, weight FROM classification_keywords ORDER BY category, weight DESC, keyword".to_string()
        };

        let mut stmt = self.conn.prepare(&query)?;

        let keywords = stmt
            .query_map([], |row| {
                Ok(crate::models::ClassificationKeyword {
                    id: row.get(0)?,
                    category: row.get(1)?,
                    subcategory: row.get(2)?,
                    keyword: row.get(3)?,
                    weight: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(keywords)
    }

    pub fn add_classification_keyword(&self, category: &str, subcategory: Option<&str>, keyword: &str, weight: f64) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO classification_keywords (category, subcategory, keyword, weight) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![category, subcategory, keyword, weight],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_classification_keyword(&self, id: i64, keyword: &str, weight: f64) -> Result<()> {
        self.conn.execute(
            "UPDATE classification_keywords SET keyword = ?1, weight = ?2 WHERE id = ?3",
            rusqlite::params![keyword, weight, id],
        )?;
        Ok(())
    }

    pub fn delete_classification_keyword(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM classification_keywords WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }
}
