mod classifier;
mod commands;
mod database;
mod models;
mod ocr;

#[cfg(test)]
mod database_tests;

use classifier::Classifier;
use commands::AppState;
use database::Database;
use ocr::OcrEngine;
use std::sync::Mutex;
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
            commands::get_deleted_documents,
            commands::restore_document,
            commands::permanently_delete_document,
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
            // Category management
            commands::get_main_categories,
            commands::get_subcategories,
            commands::add_subcategory,
            commands::delete_subcategory,
            commands::get_all_tags,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
