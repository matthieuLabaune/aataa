# 🔧 Solutions aux 3 problèmes identifiés

## 1️⃣ Problème : Infos extraites pas assez visibles

### 📊 Ce qui se passe

L'OCR extrait bien le texte mais :
- **Tag montant** : `24600.00€` au lieu du bon format
- **Type** : "Unknown" (pas reconnu comme Facture)
- **Tags manquants** : Gray Matter Technology, N° devis, etc.

### ✅ Solution : Améliorer l'extraction de tags

Le problème vient du classifieur qui cherche "Total HT" mais le PDF dit juste "Total".

**Fichier à modifier : `src-tauri/src/classifier.rs`**

```rust
fn extract_amount(&self, text: &str) -> Option<String> {
    // Patterns plus flexibles pour capturer les montants
    let patterns = vec![
        r"total[:\s]+(\d+[\s,]?\d+[.,]\d{2})\s*€",    // Total: 24 600.00 €
        r"total[:\s]+(\d+[\s,]?\d+[.,]\d{2})",         // Total 24600.00
        r"(\d+[\s,]?\d+[.,]\d{2})\s*€\s*$",           // 24 600.00 € (fin de ligne)
        r"montant[:\s]+(\d+[\s,]?\d+[.,]\d{2})",      // Montant: 24600.00
    ];

    for pattern in patterns {
        if let Ok(re) = Regex::new(&format!("(?i){}", pattern)) {
            if let Some(caps) = re.captures(text) {
                if let Some(m) = caps.get(1) {
                    let amount = m.as_str()
                        .replace(",", ".")
                        .replace(" ", "");  // Enlever les espaces

                    // Formater avec espaces pour milliers
                    if let Ok(num) = amount.parse::<f64>() {
                        return Some(format!("{:.2}", num)
                            .chars()
                            .collect::<Vec<_>>()
                            .rchunks(3)
                            .rev()
                            .map(|chunk| chunk.iter().collect::<String>())
                            .collect::<Vec<_>>()
                            .join(" "));
                    }
                    return Some(amount);
                }
            }
        }
    }
    None
}
```

### 🎨 UI : Rendre les tags plus visibles

Les tags sont dans la modale mais pas assez mis en avant.

**Proposition :** Afficher les infos clés directement sur la carte du document

## 2️⃣ Problème : Impossible d'ouvrir les fichiers

### ❌ Erreur actuelle

```
invalid args 'filePath' for command open_file
```

### 🔍 Cause

La commande Rust attend `file_path` mais le frontend envoie `filePath` (camelCase).

**Fichier : `src-tauri/src/commands.rs` ligne 188**

```rust
#[command]
pub async fn open_file(file_path: String) -> Result<(), String> {
    // Le paramètre est "file_path" (snake_case)
```

**Mais dans `app.vue` ligne 846 :**

```ts
await invoke('open_file', { filePath })  // ❌ camelCase
```

### ✅ Solution : Corriger l'appel

**Option 1 (RECOMMANDÉ) : Corriger le frontend**

```ts
async function openDocument(filePath: string) {
  try {
    await invoke('open_file', { file_path: filePath })  // ✅ snake_case
  } catch (error) {
    console.error('Error opening file:', error)
    showMessage('❌ Erreur ouverture: ' + error, 5000)
  }
}
```

**Option 2 : Corriger Rust (déconseillé car affecte toute l'API)**

```rust
#[command]
pub async fn open_file(filePath: String) -> Result<(), String> {
    // Change partout où c'est utilisé
```

## 3️⃣ Problème : Réinitialiser l'app = Vider la DB ?

### 📦 Ce qui est stocké

1. **Base de données** : `~/Library/Application Support/aataa/documents.db`
2. **Fichiers archivés** : Dans le dossier d'archivage choisi
3. **Configuration** : Path d'archivage (stocké dans Tauri Store)

### ✅ Niveaux de réinitialisation

#### Niveau 1 : Vider la liste (garder les fichiers)

Supprime juste les entrées DB, garde les fichiers sur disque.

```rust
#[command]
pub async fn clear_database(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.conn.execute("DELETE FROM documents", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

#### Niveau 2 : Reset complet (tout supprimer)

Supprime DB + fichiers archivés.

```rust
#[command]
pub async fn reset_application(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;

    // 1. Supprimer tous les fichiers archivés
    if archive_path.exists() {
        std::fs::remove_dir_all(&*archive_path)
            .map_err(|e| format!("Erreur suppression fichiers: {}", e))?;
        std::fs::create_dir_all(&*archive_path)
            .map_err(|e| format!("Erreur recréation dossier: {}", e))?;
    }

    // 2. Vider la base de données
    db.conn.execute("DELETE FROM documents", [])
        .map_err(|e| e.to_string())?;

    // 3. Réinitialiser les sous-catégories personnalisées
    db.conn.execute("DELETE FROM subcategories WHERE is_predefined = 0", [])
        .map_err(|e| e.to_string())?;

    // 4. Réinitialiser les mots-clés personnalisés
    db.conn.execute("DELETE FROM classification_keywords WHERE id > 100", [])
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

#### Niveau 3 : Reset absolu (tout détruire)

Supprime même la base de données et les tables.

```rust
#[command]
pub async fn nuclear_reset(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // 1. Supprimer fichiers archivés
    let archive_path = state.archive_path.lock().map_err(|e| e.to_string())?;
    if archive_path.exists() {
        std::fs::remove_dir_all(&*archive_path)
            .map_err(|e| e.to_string())?;
    }

    // 2. Supprimer le fichier DB
    let db_path = app.path_resolver()
        .app_data_dir()
        .ok_or("Cannot find app data dir")?
        .join("documents.db");

    if db_path.exists() {
        std::fs::remove_file(&db_path)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
    // ⚠️ L'app doit redémarrer après !
}
```

### 🎨 UI pour la réinitialisation

```vue
<!-- Dans app.vue -->
<div class="danger-zone">
  <h3>⚠️ Zone dangereuse</h3>

  <button @click="confirmAction('clear')" class="btn-danger">
    🗑️ Vider la liste (garder fichiers)
  </button>

  <button @click="confirmAction('reset')" class="btn-danger">
    🔄 Réinitialisation complète
  </button>

  <button @click="confirmAction('nuclear')" class="btn-danger-extreme">
    💣 Reset absolu (redémarrage requis)
  </button>
</div>

<script setup>
async function confirmAction(type: string) {
  const messages = {
    clear: 'Vider la liste des documents ? Les fichiers resteront sur le disque.',
    reset: 'Réinitialiser TOUT ? Documents + fichiers supprimés définitivement !',
    nuclear: 'ATTENTION ! Suppression totale de la base de données. Redémarrage requis !'
  }

  if (!confirm(messages[type])) return

  try {
    if (type === 'clear') {
      await invoke('clear_database')
    } else if (type === 'reset') {
      await invoke('reset_application')
    } else if (type === 'nuclear') {
      await invoke('nuclear_reset')
      alert('Base de données supprimée. Redémarrez l\'application.')
      window.close()
    }

    await loadDocuments()
    showMessage('✅ Réinitialisation effectuée', 3000)
  } catch (error) {
    showMessage('❌ Erreur: ' + error, 5000)
  }
}
</script>
```

---

## 🚀 Plan d'action immédiat

### 1. Corriger l'ouverture de fichiers (5 min)

```bash
# Modifier app/app.vue ligne 846
file_path: filePath  # au lieu de filePath
```

### 2. Améliorer affichage des tags (optionnel)

Ajouter plus de contraste/taille aux infos importantes.

### 3. Ajouter commande de reset (10 min)

Choisir le niveau adapté :
- **Niveau 1** : Pour tests/développement
- **Niveau 2** : Reset utilisateur normal
- **Niveau 3** : Problème grave uniquement

---

## ❓ Quelle solution voulez-vous appliquer en premier ?

1. ✅ **Ouverture fichiers** (fix rapide)
2. 🎨 **Améliorer affichage tags**
3. 🔄 **Ajouter reset DB**
4. 🧪 **Tester classification sémantique** (solution long terme)
