use crate::database::Database;
use crate::models::Document;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use uuid::Uuid;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub version: String,
    pub created_at: String,
    pub total_documents: usize,
    pub categories: Vec<String>,
    pub app_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupManifest {
    pub metadata: BackupMetadata,
    pub documents: Vec<Document>,
}

pub struct BackupManager {
    archive_path: PathBuf,
}

impl BackupManager {
    pub fn new(archive_path: PathBuf) -> Self {
        Self { archive_path }
    }

    /// Crée un export complet de tous les documents dans un fichier ZIP
    pub fn export_backup(&self, db: &Database, output_path: &Path) -> Result<String, String> {
        // Récupérer tous les documents non supprimés
        let documents = db
            .get_all_documents()
            .map_err(|e| format!("Failed to get documents: {}", e))?;

        let active_docs: Vec<Document> = documents
            .into_iter()
            .filter(|d| d.deleted_at.is_none())
            .collect();

        if active_docs.is_empty() {
            return Err("No documents to export".to_string());
        }

        // Créer le fichier ZIP
        let file = File::create(output_path)
            .map_err(|e| format!("Failed to create backup file: {}", e))?;
        let mut zip = ZipWriter::new(file);

        let options: FileOptions<()> = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o644)
            .last_modified_time(
                zip::DateTime::from_date_and_time(2024, 10, 19, 18, 0, 0)
                    .unwrap_or_else(|_| zip::DateTime::default()),
            );

        // 1. Créer le manifest avec métadonnées
        let metadata = BackupMetadata {
            version: "1.0".to_string(),
            created_at: chrono::Local::now().to_rfc3339(),
            total_documents: active_docs.len(),
            categories: active_docs
                .iter()
                .map(|d| d.category.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let manifest = BackupManifest {
            metadata,
            documents: active_docs.clone(),
        };

        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

        zip.start_file("manifest.json", options)
            .map_err(|e| format!("Failed to create manifest: {}", e))?;
        zip.write_all(manifest_json.as_bytes())
            .map_err(|e| format!("Failed to write manifest: {}", e))?;

        // 2. Exporter la base de données SQLite
        let db_path = self.archive_path.join("documents.db");
        if db_path.exists() {
            let db_content =
                fs::read(&db_path).map_err(|e| format!("Failed to read database: {}", e))?;
            zip.start_file("database/documents.db", options)
                .map_err(|e| format!("Failed to create DB entry: {}", e))?;
            zip.write_all(&db_content)
                .map_err(|e| format!("Failed to write DB: {}", e))?;
        }

        // 3. Copier tous les fichiers organisés par catégorie
        let mut added_files = std::collections::HashSet::new();

        for doc in &active_docs {
            let source_path = PathBuf::from(&doc.file_path);
            if !source_path.exists() {
                eprintln!("Warning: File not found: {}", doc.file_path);
                continue;
            }

            // Récupérer les métadonnées du fichier source pour préserver la date
            let metadata = fs::metadata(&source_path)
                .map_err(|e| format!("Failed to get file metadata: {}", e))?;
            let modified_time = metadata.modified().ok().and_then(|t| {
                use std::time::SystemTime;
                t.duration_since(SystemTime::UNIX_EPOCH).ok()
            });

            // Organiser par catégorie dans le ZIP
            let category = doc.category.replace("/", "_"); // Éviter les problèmes de path

            // Gérer les noms dupliqués en ajoutant un suffixe si nécessaire
            let mut zip_path = format!("documents/{}/{}", category, doc.new_name);
            let mut counter = 1;

            while added_files.contains(&zip_path) {
                // Extraire l'extension et le nom de base
                let path = std::path::Path::new(&doc.new_name);
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

                let new_name = if ext.is_empty() {
                    format!("{}_{}", stem, counter)
                } else {
                    format!("{}_{}.{}", stem, counter, ext)
                };

                zip_path = format!("documents/{}/{}", category, new_name);
                counter += 1;
            }

            added_files.insert(zip_path.clone());

            // Créer les options avec la date du fichier original
            let file_options: FileOptions<()> = if let Some(duration) = modified_time {
                use chrono::{DateTime, Datelike, Timelike, Utc};
                let datetime = DateTime::<Utc>::from(SystemTime::UNIX_EPOCH + duration);

                FileOptions::default()
                    .compression_method(CompressionMethod::Deflated)
                    .unix_permissions(0o644)
                    .last_modified_time(
                        zip::DateTime::from_date_and_time(
                            datetime.year() as u16,
                            datetime.month() as u8,
                            datetime.day() as u8,
                            datetime.hour() as u8,
                            datetime.minute() as u8,
                            datetime.second() as u8,
                        )
                        .unwrap_or_default(),
                    )
            } else {
                FileOptions::default()
                    .compression_method(CompressionMethod::Deflated)
                    .unix_permissions(0o644)
            };

            let file_content = fs::read(&source_path)
                .map_err(|e| format!("Failed to read file {}: {}", doc.new_name, e))?;

            zip.start_file(&zip_path, file_options)
                .map_err(|e| format!("Failed to add file to ZIP: {}", e))?;
            zip.write_all(&file_content)
                .map_err(|e| format!("Failed to write file to ZIP: {}", e))?;
        }

        // 4. Finaliser le ZIP
        zip.finish()
            .map_err(|e| format!("Failed to finalize backup: {}", e))?;

        Ok(format!(
            "Backup created successfully: {} documents exported",
            active_docs.len()
        ))
    }

    /// Importe un backup depuis un fichier ZIP
    pub fn import_backup(&self, db: &Database, backup_path: &Path) -> Result<String, String> {
        if !backup_path.exists() {
            return Err("Backup file does not exist".to_string());
        }

        let file = File::open(backup_path).map_err(|e| format!("Failed to open backup: {}", e))?;
        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP: {}", e))?;

        // 1. Lire le manifest
        let manifest_content = {
            let mut manifest_file = archive
                .by_name("manifest.json")
                .map_err(|e| format!("Manifest not found: {}", e))?;
            let mut content = String::new();
            std::io::Read::read_to_string(&mut manifest_file, &mut content)
                .map_err(|e| format!("Failed to read manifest: {}", e))?;
            content
        };

        let manifest: BackupManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse manifest: {}", e))?;

        // 2. Créer le dossier temporaire pour l'extraction
        let temp_dir = self.archive_path.join("temp_restore");
        fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;

        // 3. Extraire tous les fichiers
        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to read file {}: {}", i, e))?;
            let outpath = temp_dir.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath).map_err(|e| format!("Failed to create dir: {}", e))?;
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("Failed to create parent dir: {}", e))?;
                }
                let mut outfile =
                    File::create(&outpath).map_err(|e| format!("Failed to create file: {}", e))?;
                io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to extract file: {}", e))?;
            }
        }

        // 4. Restaurer les fichiers dans l'archive principale
        let mut restored_count = 0;
        let mut skipped_count = 0;

        // Récupérer tous les documents existants pour la détection de doublons
        let existing_docs = db
            .get_all_documents()
            .map_err(|e| format!("Failed to get existing documents: {}", e))?;

        for doc in &manifest.documents {
            let category = doc.category.replace("/", "_");
            let source = temp_dir.join(format!("documents/{}/{}", category, doc.new_name));

            if source.exists() {
                // Calculer le checksum du fichier à importer
                let source_checksum = calculate_file_checksum(&source)?;

                // Vérifier si un document identique existe déjà
                let mut is_duplicate = false;
                for existing_doc in &existing_docs {
                    if existing_doc.deleted_at.is_none() {
                        let existing_path = PathBuf::from(&existing_doc.file_path);
                        if existing_path.exists() {
                            let existing_checksum = calculate_file_checksum(&existing_path)?;

                            // Si même checksum ET même nom → c'est un doublon
                            if source_checksum == existing_checksum
                                && doc.new_name == existing_doc.new_name
                            {
                                is_duplicate = true;
                                break;
                            }
                        }
                    }
                }

                if is_duplicate {
                    // Skip les doublons
                    skipped_count += 1;
                    eprintln!("⏭️  Skipped duplicate: {}", doc.new_name);
                    continue;
                }

                // Copier le fichier
                let dest = self.archive_path.join(&doc.new_name);

                // Si le fichier existe déjà avec un nom différent, ajouter un suffixe
                let final_dest = if dest.exists() {
                    let mut counter = 1;
                    let path = std::path::Path::new(&doc.new_name);
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
                    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

                    loop {
                        let new_name = if ext.is_empty() {
                            format!("{}_{}", stem, counter)
                        } else {
                            format!("{}_{}.{}", stem, counter, ext)
                        };
                        let new_dest = self.archive_path.join(&new_name);
                        if !new_dest.exists() {
                            break new_dest;
                        }
                        counter += 1;
                    }
                } else {
                    dest
                };

                fs::copy(&source, &final_dest)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;

                // Ajouter à la base de données avec un NOUVEL ID
                let mut restored_doc = doc.clone();
                restored_doc.id = Uuid::new_v4().to_string();
                restored_doc.file_path = final_dest.to_string_lossy().to_string();
                restored_doc.new_name = final_dest
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&doc.new_name)
                    .to_string();

                db.insert_document(&restored_doc)
                    .map_err(|e| format!("Failed to add document to DB: {}", e))?;
                restored_count += 1;
            }
        }

        // 5. Nettoyer le dossier temporaire
        let _ = fs::remove_dir_all(&temp_dir);

        let message = if skipped_count > 0 {
            format!(
                "Backup restored successfully: {} documents imported, {} duplicates skipped",
                restored_count, skipped_count
            )
        } else {
            format!(
                "Backup restored successfully: {} documents imported",
                restored_count
            )
        };

        Ok(message)
    }

    /// Liste les backups disponibles dans un dossier
    pub fn list_backups(backup_dir: &Path) -> Result<Vec<PathBuf>, String> {
        if !backup_dir.exists() {
            return Ok(vec![]);
        }

        let entries = fs::read_dir(backup_dir)
            .map_err(|e| format!("Failed to read backup directory: {}", e))?;

        let mut backups = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("zip") {
                backups.push(path);
            }
        }

        // Trier par date de modification (plus récent en premier)
        backups.sort_by(|a, b| {
            let a_modified = fs::metadata(a).and_then(|m| m.modified()).ok();
            let b_modified = fs::metadata(b).and_then(|m| m.modified()).ok();
            b_modified.cmp(&a_modified)
        });

        Ok(backups)
    }
}

/// Calcule le checksum SHA256 d'un fichier pour détecter les doublons
fn calculate_file_checksum(path: &Path) -> Result<String, String> {
    use sha2::{Digest, Sha256};

    let mut file =
        File::open(path).map_err(|e| format!("Failed to open file for checksum: {}", e))?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("Failed to read file for checksum: {}", e))?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
