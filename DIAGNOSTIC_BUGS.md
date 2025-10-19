# 🔍 Diagnostic des bugs restants

**Date**: 19 octobre 2025 23:30
**Status**: En cours d'investigation

## 📋 Corrections appliquées

### ✅ Fichiers corrigés (frontend/src/views/Home.vue)

**Ligne 347-350** - Import fichier unique:
```typescript
await invoke('process_file', {
  file_path: selected,      // ✅ snake_case
  ocr_type: selectedOcrType.value  // ✅ snake_case
})
```

**Ligne 432** - Import dossier (boucle):
```typescript
await invoke('process_file', { 
  file_path: filePath,      // ✅ snake_case
  ocr_type: 'standard'      // ✅ ajouté
})
```

**Ligne 467** - Ouverture fichier (Home.vue):
```typescript
await invoke('open_file', { 
  file_path: doc.file_path  // ✅ snake_case
})
```

**Ligne 578** - Ouverture fichier (Explorer.vue):
```typescript
await invoke('open_file', { 
  file_path: doc.file_path  // ✅ snake_case
})
```

## 🎯 Actions à tester

Pour identifier précisément le problème restant, tester:

### Test 1: Import fichier unique
1. Cliquer sur "Importer un document"
2. Sélectionner un PDF
3. **Attendu**: Document traité et affiché
4. **Si erreur**: Noter le message exact

### Test 2: Scanner dossier
1. Cliquer sur "Scanner un dossier"
2. Sélectionner un dossier avec 40 fichiers
3. **Attendu**: 40 documents traités
4. **Si erreur**: Noter combien échouent et le message

### Test 3: Ouvrir document existant
1. Cliquer sur icône "Ouvrir" d'un document
2. **Attendu**: Fichier s'ouvre dans l'app système
3. **Si erreur**: Noter le message exact

## 🔍 Points de vérification

### Architecture actuelle
- ✅ `tauri.conf.json` pointe sur `frontend/` (pas `app/`)
- ✅ Vite tourne sur localhost:5173
- ✅ Tauri app démarrée et connectée
- ✅ Hot reload actif

### Signatures Rust (src-tauri/src/commands.rs)
```rust
// Ligne 39
#[tauri::command]
fn process_file(file_path: String, ocr_type: String) -> Result<Document, String>

// Ligne 235
#[tauri::command]
fn open_file(file_path: String) -> Result<(), String>
```

## ❓ Questions pour debug

1. **Quelle action exactement ne fonctionne pas?**
   - [ ] Import fichier unique
   - [ ] Scanner dossier
   - [ ] Ouvrir fichier existant
   - [ ] Toutes les trois

2. **Quel est le message d'erreur exact?**
   - Console navigateur (F12)
   - Toast/notification dans l'app
   - Rien ne se passe

3. **Y a-t-il des logs dans le terminal tauri?**
   - Regarder la sortie du terminal où tourne `npm run tauri:dev`

## 🔧 Solutions possibles

### Si le hot reload ne marche pas:
```bash
# Tuer complètement et relancer
pkill -9 node
pkill -9 -f tauri
cd /Users/matt/Documents/sites/aataa
npm run tauri:dev
```

### Si le problème persiste:
- Vérifier la console F12 pour erreurs JavaScript
- Vérifier les logs Rust dans le terminal
- Peut-être un cache navigateur/Vite à vider

### Si besoin de rebuild complet:
```bash
cd /Users/matt/Documents/sites/aataa
cd frontend && npm run build
cd ../src-tauri && cargo clean
cd .. && npm run tauri:dev
```

## 📝 Prochaine étape

**Attendre retour utilisateur avec:**
- Action exacte qui échoue
- Message d'erreur complet
- Logs console/terminal si disponibles
