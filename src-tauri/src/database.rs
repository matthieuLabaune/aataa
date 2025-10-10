use rusqlite::{Connection, Result};
use std::path::PathBuf;
use crate::models::Document;

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
                notes TEXT
            )",
            [],
        )?;

        // Add notes column if it doesn't exist (migration)
        let _ = conn.execute("ALTER TABLE documents ADD COLUMN notes TEXT", []);

        Ok(Database { conn })
    }

    pub fn insert_document(&self, doc: &Document) -> Result<()> {
        self.conn.execute(
            "INSERT INTO documents (id, original_name, new_name, file_path, document_type, tags, ocr_text, created_at, file_size, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![
                doc.id,
                doc.original_name,
                doc.new_name,
                doc.file_path,
                doc.document_type,
                serde_json::to_string(&doc.tags).unwrap_or_default(),
                doc.ocr_text,
                doc.created_at,
                doc.file_size as i64,
                doc.notes,
            ],
        )?;
        Ok(())
    }

    pub fn get_all_documents(&self) -> Result<Vec<Document>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, original_name, new_name, file_path, document_type, tags, ocr_text, created_at, file_size, notes
             FROM documents ORDER BY created_at DESC"
        )?;

        let documents = stmt.query_map([], |row| {
            Ok(Document {
                id: row.get(0)?,
                original_name: row.get(1)?,
                new_name: row.get(2)?,
                file_path: row.get(3)?,
                document_type: row.get(4)?,
                tags: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                ocr_text: row.get(6)?,
                created_at: row.get(7)?,
                file_size: row.get::<_, i64>(8)? as u64,
                notes: row.get(9).ok(),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

        Ok(documents)
    }

    pub fn search_documents(&self, query: &str) -> Result<Vec<Document>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, original_name, new_name, file_path, document_type, tags, ocr_text, created_at, file_size, notes
             FROM documents
             WHERE ocr_text LIKE ?1 OR new_name LIKE ?1 OR tags LIKE ?1 OR notes LIKE ?1
             ORDER BY created_at DESC"
        )?;

        let search_query = format!("%{}%", query);
        let documents = stmt.query_map([&search_query], |row| {
            Ok(Document {
                id: row.get(0)?,
                original_name: row.get(1)?,
                new_name: row.get(2)?,
                file_path: row.get(3)?,
                document_type: row.get(4)?,
                tags: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                ocr_text: row.get(6)?,
                created_at: row.get(7)?,
                file_size: row.get::<_, i64>(8)? as u64,
                notes: row.get(9).ok(),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

        Ok(documents)
    }

    pub fn delete_document(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM documents WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_document_notes(&self, id: &str, notes: Option<String>) -> Result<()> {
        self.conn.execute(
            "UPDATE documents SET notes = ?1 WHERE id = ?2",
            rusqlite::params![notes, id],
        )?;
        Ok(())
    }

    pub fn update_document_metadata(&self, id: &str, document_type: &str, tags: &Vec<String>, new_name: &str) -> Result<()> {
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
}
