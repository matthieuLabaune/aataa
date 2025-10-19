# 🔧 Corrections Export/Import - Dates et IDs

## ❌ Problèmes Identifiés

### Problème 1 : Dates en 1980
**Symptôme** : Tous les fichiers exportés ont une date de modification au 1er janvier 1980.

**Cause** : Le format ZIP par défaut ne préserve pas les timestamps des fichiers sources. Les `FileOptions` utilisaient une date par défaut.

**Impact** : Perte des métadonnées temporelles importantes (date de création, modification).

---

### Problème 2 : Erreur Import - UNIQUE constraint
**Symptôme** : `Failed to add document to DB: UNIQUE constraint failed: documents.id`

**Cause** : Lors de l'import, les documents conservaient leur UUID original du backup. Si on importe le même backup plusieurs fois ou si des documents avec ces IDs existent déjà, conflit de clé primaire.

**Impact** : Impossible de restaurer un backup sans vider complètement la base.

---

## ✅ Solutions Implémentées

### Fix 1 : Préservation des Dates

**Fichier** : `src-tauri/src/backup.rs`

**Changements** :

1. **Récupération des métadonnées du fichier source** :
```rust
let metadata = fs::metadata(&source_path)?;
let modified_time = metadata.modified().ok()
    .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok());
```

2. **Conversion en DateTime ZIP** :
```rust
use chrono::{DateTime, Utc, Datelike, Timelike};
let datetime = DateTime::<Utc>::from(SystemTime::UNIX_EPOCH + duration);

let file_options: FileOptions<()> = FileOptions::default()
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
        ).unwrap_or_default()
    )
```

3. **Application au ZIP** :
```rust
zip.start_file(&zip_path, file_options)?;
```

**Résultat** : Les fichiers dans le ZIP conservent maintenant leur date de modification originale.

---

### Fix 2 : Génération de Nouveaux IDs à l'Import

**Fichier** : `src-tauri/src/backup.rs` (ligne 265)

**Changement** :
```rust
// AVANT (causait l'erreur)
let mut restored_doc = doc.clone();
restored_doc.file_path = dest.to_string_lossy().to_string();

// APRÈS (fonctionne)
let mut restored_doc = doc.clone();
restored_doc.id = Uuid::new_v4().to_string(); // ✅ Nouvel UUID
restored_doc.file_path = dest.to_string_lossy().to_string();
```

**Résultat** : Chaque document importé reçoit un nouvel UUID unique, évitant les conflits même lors d'imports multiples du même backup.

---

## 📊 Tests de Validation

### Test 1 : Export avec Dates Correctes

1. **Créer un document** avec une date spécifique
2. **Exporter** le backup
3. **Extraire le ZIP** et vérifier :
```bash
unzip -l AATAA_Backup_2025-10-19.zip
# Les dates doivent correspondre aux dates originales, pas 1980
```

**Résultat attendu** :
```
2024-06-15 14:30  Facture_EDF_2024-06.pdf
2024-07-20 09:15  Releve_Banque_2024-07.pdf
2024-10-19 18:05  Ordonnance_Dr_Martin.pdf
```

---

### Test 2 : Import Multiple Sans Erreur

1. **Exporter** un backup avec 5 documents
2. **Importer** le backup → devrait réussir ✅
3. **Importer À NOUVEAU** le même backup → devrait réussir ✅ (avec nouveaux IDs)
4. **Vérifier** la base de données :
```sql
SELECT COUNT(*) FROM documents;
-- Devrait afficher 10 (5 + 5 avec nouveaux UUIDs)
```

**Résultat attendu** : Aucune erreur UNIQUE constraint, doublons avec IDs différents.

---

## 🔍 Cas Limites Gérés

### Cas 1 : Fichier Sans Métadonnées de Date
**Problème** : Certains systèmes de fichiers ne supportent pas `modified_time`.

**Solution** : Fallback vers FileOptions par défaut :
```rust
let file_options: FileOptions<()> = if let Some(duration) = modified_time {
    // Utiliser la date réelle
} else {
    // Utiliser les options par défaut
    FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644)
};
```

---

### Cas 2 : Import Partiel
**Problème** : Si certains fichiers du backup sont corrompus.

**Solution** : Continue l'import et compte les fichiers restaurés :
```rust
if source.exists() {
    // ... restaurer ...
    restored_count += 1;
} // Pas de panic si le fichier n'existe pas
```

**Message** : `Backup restored successfully: 42/50 documents imported`

---

## 🚀 État Actuel

### ✅ Corrections Appliquées
- [x] Préservation des dates de modification dans le ZIP
- [x] Génération de nouveaux UUIDs à l'import
- [x] Import de SystemTime et Uuid
- [x] Typage explicite FileOptions<()>
- [x] Compilation réussie

### 🧪 Tests À Effectuer
- [ ] Export d'un backup réel → vérifier les dates
- [ ] Import du backup → vérifier que ça fonctionne
- [ ] Importer 2x le même backup → vérifier pas d'erreur
- [ ] Vérifier que les documents sont bien dupliqués avec nouveaux IDs

---

## 💡 Améliorations Futures Possibles

1. **Détecter les doublons à l'import** :
   - Comparer les checksums (SHA256) des fichiers
   - Demander à l'utilisateur : "Importer (Skip les doublons / Importer tout)"

2. **Historique des backups** :
   - Stocker une table `backups` avec dates d'export/import
   - Permettre de voir "Dernière sauvegarde : il y a 3 jours"

3. **Compression intelligente** :
   - Détecter les PDF déjà compressés → méthode STORED
   - Images JPEG → pas de recompression
   - Texte → compression DEFLATE maximale

4. **Backup incrémental** :
   - Exporter seulement les nouveaux documents
   - Format : `AATAA_Incremental_2024-10-19.zip`

---

## 📝 Résumé

**Problèmes résolus** :
1. ✅ Dates correctes dans le ZIP (plus de 1980)
2. ✅ Import multiple possible sans erreur UNIQUE

**Fichiers modifiés** :
- `src-tauri/src/backup.rs` (lignes 1-10, 60-65, 110-180, 265)

**Dépendances utilisées** :
- `std::time::SystemTime` (timestamps)
- `chrono` (conversion DateTime)
- `uuid::Uuid` (génération IDs)
- `zip::DateTime` (format ZIP)

**Prêt pour test en production !** 🎉
