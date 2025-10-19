#[cfg(test)]
mod tests {
    use crate::database::Database;
    use crate::models::Document;
    use tempfile::tempdir;

    fn create_test_document(id: &str) -> Document {
        Document {
            id: id.to_string(),
            original_name: format!("original_{}.pdf", id),
            new_name: format!("renamed_{}.pdf", id),
            file_path: format!("/path/to/{}.pdf", id),
            document_type: "Facture".to_string(),
            category: "Financier".to_string(),
            subcategory: Some("Facture fournisseur".to_string()),
            confidence: Some(0.95),
            tags: vec!["urgent".to_string(), "important".to_string()],
            ocr_text: "Test OCR text".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            file_size: 1024,
            notes: Some("Test notes".to_string()),
            deleted_at: None,
        }
    }

    #[test]
    fn test_database_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let _db = Database::new(db_path).unwrap();

        // If we get here, database was created successfully
        assert!(true);
    }

    #[test]
    fn test_insert_and_get_document() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();

        let doc = create_test_document("test1");
        db.insert_document(&doc).unwrap();

        let docs = db.get_all_documents().unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].id, "test1");
        assert_eq!(docs[0].document_type, "Facture");
    }

    #[test]
    fn test_insert_multiple_documents() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();

        let doc1 = create_test_document("test1");
        let doc2 = create_test_document("test2");
        let doc3 = create_test_document("test3");

        db.insert_document(&doc1).unwrap();
        db.insert_document(&doc2).unwrap();
        db.insert_document(&doc3).unwrap();

        let docs = db.get_all_documents().unwrap();
        assert_eq!(docs.len(), 3);
    }

    #[test]
    fn test_search_documents() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();

        let mut doc1 = create_test_document("search1");
        doc1.ocr_text = "This contains the word facture".to_string();

        let mut doc2 = create_test_document("search2");
        doc2.ocr_text = "This contains the word contrat".to_string();

        db.insert_document(&doc1).unwrap();
        db.insert_document(&doc2).unwrap();

        let results = db.search_documents("facture").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "search1");
    }

    #[test]
    fn test_delete_document() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();

        let doc = create_test_document("delete_test");
        db.insert_document(&doc).unwrap();

        let docs_before = db.get_all_documents().unwrap();
        assert_eq!(docs_before.len(), 1);

        db.delete_document("delete_test").unwrap();

        let docs_after = db.get_all_documents().unwrap();
        assert_eq!(docs_after.len(), 0);
    }

    #[test]
    fn test_update_notes() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();

        let doc = create_test_document("notes_test");
        db.insert_document(&doc).unwrap();

        db.update_document_notes("notes_test", Some("Updated notes".to_string()))
            .unwrap();

        let docs = db.get_all_documents().unwrap();
        assert_eq!(docs[0].notes, Some("Updated notes".to_string()));
    }

    #[test]
    fn test_update_metadata() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();

        let doc = create_test_document("meta_test");
        db.insert_document(&doc).unwrap();

        let new_tags = vec!["tag1".to_string(), "tag2".to_string()];
        db.update_document_metadata("meta_test", "Contrat", &new_tags, "new_filename.pdf")
            .unwrap();

        let docs = db.get_all_documents().unwrap();
        assert_eq!(docs[0].document_type, "Contrat");
        assert_eq!(docs[0].new_name, "new_filename.pdf");
        assert_eq!(docs[0].tags, new_tags);
    }

    #[test]
    fn test_search_in_notes() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(db_path).unwrap();

        let mut doc = create_test_document("notes_search");
        doc.notes = Some("Important meeting on Monday".to_string());
        db.insert_document(&doc).unwrap();

        let results = db.search_documents("Monday").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "notes_search");
    }

    #[test]
    fn test_migration_adds_notes_column() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        // Create database (which runs migration)
        let db = Database::new(db_path.clone()).unwrap();

        // Insert document with notes
        let doc = create_test_document("migration_test");
        db.insert_document(&doc).unwrap();

        // Verify notes are saved
        let docs = db.get_all_documents().unwrap();
        assert!(docs[0].notes.is_some());
    }
}
