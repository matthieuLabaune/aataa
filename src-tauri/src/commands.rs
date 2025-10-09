use tauri::command;
use std::sync::Mutex;
use std::path::PathBuf;
use crate::models::Document;
use crate::database::Database;
use crate::ocr::OcrEngine;
use crate::classifier::Classifier;
use uuid::Uuid;
use chrono::Local;

pub struct AppState {
    pub db: Mutex<Database>,
    pub ocr: Mutex<OcrEngine>,
    pub classifier: Classifier,
    pub archive_path: Mutex<PathBuf>,
}

#[command]
pub async fn process_file(
    file_path: String,
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

    // Perform OCR
    let mut ocr = state.ocr.lock().map_err(|e| e.to_string())?;
    let ocr_text = if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
        ocr.extract_text_from_pdf(&path).map_err(|e| e.to_string())?
    } else {
        ocr.extract_text_from_image(&path).map_err(|e| e.to_string())?
    };

    // Classify document
    let doc_type = state.classifier.classify(&ocr_text);
    let tags = state.classifier.extract_tags(&ocr_text);
    let new_name = state.classifier.generate_filename(&doc_type, &original_name);

    // Copy file to archive with folder organization
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;
    
    // Create subfolder for document type
    let type_folder = archive_path.join(&doc_type.name);
    std::fs::create_dir_all(&type_folder).map_err(|e| e.to_string())?;
    
    let new_path = type_folder.join(&new_name);

    std::fs::copy(&path, &new_path).map_err(|e| e.to_string())?;

    // Create document record
    let document = Document {
        id: Uuid::new_v4().to_string(),
        original_name,
        new_name,
        file_path: new_path.to_str().unwrap_or("").to_string(),
        document_type: doc_type.name,
        tags,
        ocr_text,
        created_at: Local::now().to_rfc3339(),
        file_size,
    };

    // Save to database
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.insert_document(&document).map_err(|e| e.to_string())?;

    Ok(document)
}

#[command]
pub async fn get_documents(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<Document>, String> {
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
pub async fn delete_document(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_document(&id).map_err(|e| e.to_string())
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
pub async fn get_archive_path(
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;
    Ok(archive_path.to_str().unwrap_or("").to_string())
}

#[command]
pub async fn write_temp_file(path: String, content: Vec<u8>) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(&content).map_err(|e| e.to_string())?;

    Ok(())
}
