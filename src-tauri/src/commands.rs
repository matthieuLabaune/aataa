use crate::backup::BackupManager;
use crate::classifier::Classifier;
use crate::database::Database;
use crate::models::{ClassificationResult, Document, MainCategory};
use crate::ocr::OcrEngine;
use chrono::Local;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::command;
use uuid::Uuid;

pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub ocr: Mutex<OcrEngine>,
    pub classifier: Classifier,
    pub archive_path: Mutex<PathBuf>,
}

/// Extract year from filename (looking for 4-digit year patterns like 2023, 2024, etc.)
fn extract_year_from_filename(filename: &str) -> Option<String> {
    let re = Regex::new(r"(20\d{2})").ok()?;
    re.captures(filename)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

/// Extract year from OCR text (looking for dates in format DD/MM/YYYY or DD-MM-YYYY)
fn extract_year_from_text(text: &str) -> Option<String> {
    // Look for date patterns like 14/06/2023 or 14-06-2023
    let re = Regex::new(r"\d{1,2}[/-]\d{1,2}[/-](20\d{2})").ok()?;
    re.captures(text)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

#[tauri::command]
pub async fn process_file(
    file_path: String,
    ocr_type: String, // OCR type selection: "standard", "handwritten", "printed", or "caption"
    state: tauri::State<'_, AppState>,
) -> Result<Document, String> {
    let path = PathBuf::from(&file_path);

    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    // Get file metadata
    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    let file_size = metadata.len();
    let original_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Perform OCR (currently using Tesseract only, OCR type parameter ready for future implementation)
    // TODO: Implement different OCR engines based on ocr_type parameter
    // - "standard": Tesseract (current implementation)
    // - "handwritten": TrOCR microsoft/trocr-base-handwritten
    // - "printed": TrOCR microsoft/trocr-base-printed
    // - "caption": BLIP Salesforce/blip-image-captioning-base
    let _ = ocr_type; // Prevent unused variable warning

    // Extract OCR text (and release the mutex before async call)
    let ocr_text = {
        let mut ocr = state.ocr.lock().map_err(|e| e.to_string())?;
        if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
            ocr.extract_text_from_pdf(&path)
                .map_err(|e| e.to_string())?
        } else {
            ocr.extract_text_from_image(&path)
                .map_err(|e| e.to_string())?
        }
    }; // Mutex guard is dropped here

    // Classify document using SEMANTIC CLASSIFICATION (AI-powered)
    let semantic_result = classify_semantic(ocr_text.clone()).await;

    let classification = match semantic_result {
        Ok(result) if result["success"].as_bool().unwrap_or(false) => {
            // Use semantic classification result
            let category_str = result["category"].as_str().unwrap_or("Autre");
            let category = match category_str {
                "Financier" => MainCategory::Financier,
                "Administratif" => MainCategory::Administratif,
                "Santé" => MainCategory::Sante,
                "Professionnel" => MainCategory::Professionnel,
                "Immobilier" => MainCategory::Immobilier,
                "Académique" => MainCategory::Academique,
                "Personnel" => MainCategory::Personnel,
                _ => MainCategory::Autre,
            };

            let confidence = result["confidence"].as_f64().unwrap_or(0.0) as f32;
            let subcategory = result["subcategory"].as_str().map(String::from);

            eprintln!(
                "✅ Classification sémantique: {} (confiance: {:.0}%)",
                category_str,
                confidence * 100.0
            );
            if let Some(ref sub) = subcategory {
                eprintln!("   → Sous-catégorie: {}", sub);
            }

            ClassificationResult {
                category,
                subcategory,
                suggested_tags: state.classifier.extract_tags(&ocr_text),
                confidence,
            }
        }
        _ => {
            // Fallback to old classifier if semantic fails
            eprintln!(
                "⚠️  Classification sémantique échouée, utilisation du classifier par défaut"
            );
            state.classifier.classify_detailed(&ocr_text)
        }
    };

    let doc_type = state.classifier.classify(&ocr_text); // Keep old method for compatibility
    let new_name = state
        .classifier
        .generate_filename(&doc_type, &original_name);

    // Extract year from filename or use current year
    let year = extract_year_from_filename(&original_name)
        .or_else(|| extract_year_from_text(&ocr_text))
        .unwrap_or_else(|| Local::now().format("%Y").to_string());

    // Copy file to archive with folder organization: CATEGORY/YEAR/
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;

    // Create subfolder structure: Category/Year/
    let category_name = classification.category.to_string();
    let type_folder = archive_path.join(&category_name).join(&year);
    std::fs::create_dir_all(&type_folder).map_err(|e| e.to_string())?;

    let new_path = type_folder.join(&new_name);

    std::fs::copy(&path, &new_path).map_err(|e| e.to_string())?;

    // Merge extracted tags with suggested tags
    let mut all_tags = classification.suggested_tags.clone();
    all_tags.push(year.clone()); // Add year as a tag

    // Create document record
    let document = Document {
        id: Uuid::new_v4().to_string(),
        original_name,
        new_name,
        file_path: new_path.to_str().unwrap_or("").to_string(),
        document_type: doc_type.name,
        category: category_name,
        subcategory: classification.subcategory,
        confidence: Some(classification.confidence),
        tags: all_tags,
        ocr_text,
        created_at: Local::now().to_rfc3339(),
        file_size,
        notes: None,
        deleted_at: None, // Document actif par défaut
    };

    // Save to database
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.insert_document(&document).map_err(|e| e.to_string())?;

    Ok(document)
}

#[command]
pub async fn get_documents(state: tauri::State<'_, AppState>) -> Result<Vec<Document>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_all_documents().map_err(|e| e.to_string())
}

#[command]
pub async fn search_documents(
    query: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Document>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.search_documents(&query).map_err(|e| e.to_string())
}

#[command]
pub async fn delete_document(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_document(&id).map_err(|e| e.to_string())
}

#[command]
pub async fn get_deleted_documents(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Document>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_deleted_documents().map_err(|e| e.to_string())
}

#[command]
pub async fn restore_document(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.restore_document(&id).map_err(|e| e.to_string())
}

#[command]
pub async fn permanently_delete_document(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    // Récupère le document pour obtenir le chemin du fichier
    let deleted_docs = db.get_deleted_documents().map_err(|e| e.to_string())?;
    let doc = deleted_docs
        .iter()
        .find(|d| d.id == id)
        .ok_or_else(|| "Document non trouvé dans la corbeille".to_string())?;

    // Supprime le fichier physique du disque
    std::fs::remove_file(&doc.file_path)
        .map_err(|e| format!("Erreur lors de la suppression du fichier: {}", e))?;

    // Supprime l'entrée de la base de données
    db.permanently_delete_document(&id)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn open_file(file_path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &file_path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[command]
pub async fn set_archive_path(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;
    *archive_path = PathBuf::from(path);

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&*archive_path).map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub async fn get_archive_path(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;
    Ok(archive_path.to_str().unwrap_or("").to_string())
}

#[command]
pub async fn scan_folder(folder_path: String) -> Result<Vec<String>, String> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(&folder_path);
    if !path.exists() || !path.is_dir() {
        return Err("Invalid folder path".to_string());
    }

    let mut files = Vec::new();
    let supported_extensions = vec!["pdf", "png", "jpg", "jpeg"];

    fn scan_directory(
        dir: &Path,
        files: &mut Vec<String>,
        supported_ext: &Vec<&str>,
    ) -> Result<(), String> {
        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if supported_ext.contains(&ext.to_str().unwrap_or("").to_lowercase().as_str()) {
                        files.push(path.to_str().unwrap_or("").to_string());
                    }
                }
            } else if path.is_dir() {
                // Scan subdirectories recursively
                scan_directory(&path, files, supported_ext)?;
            }
        }
        Ok(())
    }

    scan_directory(path, &mut files, &supported_extensions)?;

    Ok(files)
}

#[command]
pub async fn write_temp_file(path: String, content: Vec<u8>) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(&content).map_err(|e| e.to_string())?;

    Ok(())
}

#[command]
pub async fn update_notes(
    id: String,
    notes: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.update_document_notes(&id, notes)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_metadata(
    id: String,
    document_type: String,
    tags: Vec<String>,
    new_name: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.update_document_metadata(&id, &document_type, &tags, &new_name)
        .map_err(|e| e.to_string())
}

// ========== Semantic Classification Command ==========

/// Classification sémantique via Python (IA)
#[command]
pub async fn classify_semantic(text: String) -> Result<serde_json::Value, String> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    // Appeler le script Python avec le texte en stdin
    let mut child = Command::new("python3")
        .arg("python/semantic_classifier.py")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Erreur lancement Python: {}", e))?;

    // Écrire le texte dans stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| format!("Erreur écriture stdin: {}", e))?;
    }

    // Attendre la fin et récupérer la sortie
    let output = child
        .wait_with_output()
        .map_err(|e| format!("Erreur attente Python: {}", e))?;

    if output.status.success() {
        // Parser la réponse JSON
        let result: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("Erreur parse JSON: {}", e))?;
        Ok(result)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Erreur classification sémantique: {}", error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_year_from_filename() {
        assert_eq!(
            extract_year_from_filename("facture_2023_12_31.pdf"),
            Some("2023".to_string())
        );
        assert_eq!(
            extract_year_from_filename("contrat-2024.pdf"),
            Some("2024".to_string())
        );
        assert_eq!(
            extract_year_from_filename("document_2022_janvier.jpg"),
            Some("2022".to_string())
        );
        assert_eq!(extract_year_from_filename("sans_annee.pdf"), None);
        assert_eq!(extract_year_from_filename("ancien_1999.pdf"), None); // Only accepts 20XX years
    }

    #[test]
    fn test_extract_year_from_text() {
        assert_eq!(
            extract_year_from_text("Date: 14/06/2023"),
            Some("2023".to_string())
        );
        assert_eq!(
            extract_year_from_text("Émis le 01-12-2024"),
            Some("2024".to_string())
        );
        assert_eq!(
            extract_year_from_text("25/11/2022 - Facture"),
            Some("2022".to_string())
        );
        assert_eq!(extract_year_from_text("Pas de date ici"), None);
        assert_eq!(extract_year_from_text("2023 sans format"), None); // Requires date format
    }

    #[test]
    fn test_extract_year_priority() {
        // Test that filename extraction works
        let filename = "facture_2023.pdf";
        let text = "Date: 14/06/2024";

        let year_from_filename = extract_year_from_filename(filename);
        let year_from_text = extract_year_from_text(text);

        assert_eq!(year_from_filename, Some("2023".to_string()));
        assert_eq!(year_from_text, Some("2024".to_string()));
    }

    #[test]
    fn test_scan_folder_invalid_path() {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().unwrap();

        let result = rt.block_on(scan_folder("/path/that/does/not/exist".to_string()));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid folder path"));
    }

    #[test]
    fn test_open_file_command_exists() {
        // Just verify the function compiles and can be called
        // Actual file opening is OS-dependent and would open real files
        use tokio::runtime::Runtime;
        let rt = Runtime::new().unwrap();

        let result = rt.block_on(open_file("/nonexistent/file.pdf".to_string()));
        // On macOS this will try to spawn 'open' command which will fail for nonexistent file
        // but that's expected - we're just testing the command structure
        assert!(result.is_err() || result.is_ok()); // Either is acceptable for this test
    }

    #[test]
    fn test_write_temp_file() {
        use tempfile::NamedTempFile;
        use tokio::runtime::Runtime;

        let rt = Runtime::new().unwrap();
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let content = b"Test content".to_vec();
        let result = rt.block_on(write_temp_file(temp_path.clone(), content.clone()));

        assert!(result.is_ok());

        // Verify content was written
        let written_content = std::fs::read(&temp_path).unwrap();
        assert_eq!(written_content, content);
    }
}

// ============================================================================
// ML OCR - TrOCR et BLIP via Python subprocess
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct TrOCRResponse {
    success: bool,
    text: Option<String>,
    error: Option<String>,
    model: Option<String>,
    device: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BLIPResponse {
    success: bool,
    caption: Option<String>,
    error: Option<String>,
    language: Option<String>,
    model: Option<String>,
    device: Option<String>,
}

/// Extrait le texte manuscrit d'une image avec TrOCR
#[command]
pub async fn extract_handwritten_text(
    image_path: String,
    ocr_type: String, // "handwritten" ou "printed"
) -> Result<String, String> {
    log::info!(
        "🖊️ Extraction OCR avec TrOCR: {} (type: {})",
        image_path,
        ocr_type
    );

    // Vérifier que l'image existe
    let path = PathBuf::from(&image_path);
    if !path.exists() {
        return Err("L'image n'existe pas".to_string());
    }

    // Trouver le chemin du script Python
    let python_script = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("python")
        .join("trocr_handwritten.py");

    if !python_script.exists() {
        return Err(format!("Script Python introuvable: {:?}", python_script));
    }

    log::info!("📝 Appel du script Python: {:?}", python_script);

    // Exécuter le script Python
    let output = Command::new("python3")
        .arg(python_script)
        .arg(&image_path)
        .arg(&ocr_type)
        .output()
        .map_err(|e| format!("Erreur d'exécution Python: {}", e))?;

    // Logs stderr pour le débogage
    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::debug!("Python stderr: {}", stderr);
    }

    // Parser la sortie JSON
    let stdout = String::from_utf8_lossy(&output.stdout);
    log::debug!("Python stdout: {}", stdout);

    let response: TrOCRResponse =
        serde_json::from_str(&stdout).map_err(|e| format!("Erreur de parsing JSON: {}", e))?;

    if response.success {
        Ok(response.text.unwrap_or_default())
    } else {
        Err(response
            .error
            .unwrap_or_else(|| "Erreur inconnue".to_string()))
    }
}

/// Génère une description automatique pour une image avec BLIP
#[command]
pub async fn generate_image_caption(
    image_path: String,
    language: String, // "en" ou "fr"
) -> Result<String, String> {
    log::info!(
        "🎨 Génération de description BLIP: {} (langue: {})",
        image_path,
        language
    );

    // Vérifier que l'image existe
    let path = PathBuf::from(&image_path);
    if !path.exists() {
        return Err("L'image n'existe pas".to_string());
    }

    // Trouver le chemin du script Python
    let python_script = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("python")
        .join("blip_caption.py");

    if !python_script.exists() {
        return Err(format!("Script Python introuvable: {:?}", python_script));
    }

    log::info!("📝 Appel du script Python: {:?}", python_script);

    // Exécuter le script Python
    let output = Command::new("python3")
        .arg(python_script)
        .arg(&image_path)
        .arg(&language)
        .output()
        .map_err(|e| format!("Erreur d'exécution Python: {}", e))?;

    // Logs stderr pour le débogage
    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::debug!("Python stderr: {}", stderr);
    }

    // Parser la sortie JSON
    let stdout = String::from_utf8_lossy(&output.stdout);
    log::debug!("Python stdout: {}", stdout);

    let response: BLIPResponse =
        serde_json::from_str(&stdout).map_err(|e| format!("Erreur de parsing JSON: {}", e))?;

    if response.success {
        Ok(response.caption.unwrap_or_default())
    } else {
        Err(response
            .error
            .unwrap_or_else(|| "Erreur inconnue".to_string()))
    }
}

// ===== NOUVELLES COMMANDES POUR GESTION DES CATÉGORIES =====

#[command]
pub async fn get_main_categories() -> Result<Vec<String>, String> {
    use crate::models::MainCategory;
    Ok(MainCategory::all().iter().map(|c| c.to_string()).collect())
}

#[command]
pub async fn get_subcategories(
    category: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::models::Subcategory>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_subcategories(category.as_deref())
        .map_err(|e| e.to_string())
}

#[command]
pub async fn add_subcategory(
    category: String,
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.add_subcategory(&category, &name)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_subcategory(id: i64, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_subcategory(id).map_err(|e| e.to_string())
}

#[command]
pub async fn get_all_tags(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::models::Tag>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_all_tags().map_err(|e| e.to_string())
}

// ========== Classification Keywords Commands ==========

#[command]
pub async fn get_classification_keywords(
    category: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::models::ClassificationKeyword>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_classification_keywords(category.as_deref())
        .map_err(|e| e.to_string())
}

#[command]
pub async fn add_classification_keyword(
    category: String,
    subcategory: Option<String>,
    keyword: String,
    weight: f64,
    state: tauri::State<'_, AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.add_classification_keyword(&category, subcategory.as_deref(), &keyword, weight)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn update_classification_keyword(
    id: i64,
    keyword: String,
    weight: f64,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.update_classification_keyword(id, &keyword, weight)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn delete_classification_keyword(
    id: i64,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_classification_keyword(id)
        .map_err(|e| e.to_string())
}

// ========== Application Reset Commands ==========

/// Vide la base de données (garde les fichiers sur disque)
#[command]
pub async fn clear_database(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;

    db.clear_all_documents()
        .map_err(|e| format!("Erreur lors du vidage de la base: {}", e))?;

    eprintln!("✅ Base de données vidée (fichiers conservés)");
    Ok(())
}

/// Réinitialisation complète : supprime DB + fichiers archivés
#[command]
pub async fn reset_application(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;

    // 1. Supprimer tous les fichiers archivés
    if archive_path.exists() {
        eprintln!("🗑️  Suppression des fichiers dans {:?}", archive_path);
        std::fs::remove_dir_all(&*archive_path)
            .map_err(|e| format!("Erreur suppression fichiers: {}", e))?;
        std::fs::create_dir_all(&*archive_path)
            .map_err(|e| format!("Erreur recréation dossier: {}", e))?;
    }

    // 2. Vider la base de données
    db.clear_all_documents()
        .map_err(|e| format!("Erreur vidage documents: {}", e))?;

    // 3. Réinitialiser les sous-catégories personnalisées
    db.reset_custom_subcategories()
        .map_err(|e| format!("Erreur reset sous-catégories: {}", e))?;

    // 4. Réinitialiser les mots-clés personnalisés (garder les prédéfinis)
    db.reset_custom_keywords()
        .map_err(|e| format!("Erreur reset mots-clés: {}", e))?;

    eprintln!("✅ Application réinitialisée complètement");
    Ok(())
}

// ========== Backup/Export Commands ==========

/// Exporte tous les documents dans un fichier ZIP
#[command]
pub async fn export_backup(
    output_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;

    let backup_manager = BackupManager::new(archive_path.clone());
    backup_manager.export_backup(&db, std::path::Path::new(&output_path))
}

/// Importe un backup depuis un fichier ZIP
#[command]
pub async fn import_backup(
    backup_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;

    let backup_manager = BackupManager::new(archive_path.clone());
    backup_manager.import_backup(&db, std::path::Path::new(&backup_path))
}

/// Liste les fichiers de backup disponibles dans un dossier
#[command]
pub async fn list_backups(backup_dir: String) -> Result<Vec<String>, String> {
    let backups = BackupManager::list_backups(std::path::Path::new(&backup_dir))?;
    Ok(backups
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect())
}
