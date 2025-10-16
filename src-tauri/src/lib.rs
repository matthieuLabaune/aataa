mod models;
mod database;
mod ocr;
mod classifier;
mod commands;

#[cfg(test)]
mod database_tests;

use std::sync::Mutex;
use commands::AppState;
use database::Database;
use ocr::OcrEngine;
use classifier::Classifier;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Initialize database
      let app_dir = app.path().app_data_dir()?;
      std::fs::create_dir_all(&app_dir)?;
      let db_path = app_dir.join("aataa.db");
      let db = Database::new(db_path).expect("Failed to initialize database");

      // Initialize OCR engine
      let ocr = OcrEngine::new().expect("Failed to initialize OCR engine");

      // Initialize classifier
      let classifier = Classifier::new();

      // Set default archive path
      let archive_path = app_dir.join("archive");
      std::fs::create_dir_all(&archive_path)?;

      // Create app state
      let state = AppState {
        db: Mutex::new(db),
        ocr: Mutex::new(ocr),
        classifier,
        archive_path: Mutex::new(archive_path),
      };

      app.manage(state);

      Ok(())
    })
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![
      commands::process_file,
      commands::get_documents,
      commands::search_documents,
      commands::delete_document,
      commands::open_file,
      commands::set_archive_path,
      commands::get_archive_path,
      commands::scan_folder,
      commands::write_temp_file,
      commands::update_notes,
      commands::update_metadata,
      // ML OCR via Python
      commands::extract_handwritten_text,
      commands::generate_image_caption,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
