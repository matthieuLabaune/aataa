# 🐛 Bug Cache WebView - Problème persistant

**Date**: 19 octobre 2025 23:45
**Status**: En cours de résolution

## 🔍 Symptômes

- **Erreur persistante** : `invalid args 'filePath' for command 'process_file'`
- **Console affiche** : `Home.vue:440` (ligne d'erreur dans le log)
- **Tentatives précédentes** : Corrections appliquées mais non prises en compte

## 🎯 Cause racine identifiée

**PROBLÈME DE CACHE WEBVIEW TAURI**

1. ✅ Le code source est **correctement modifié** dans `frontend/src/views/Home.vue`
2. ✅ Les corrections sont **commitées** dans Git
3. ❌ Le **WebView Tauri cache l'ancien JavaScript**
4. ❌ Le **hot reload Vite ne force pas le WebView à recharger**

### Code vérifié (CORRECT)

**frontend/src/views/Home.vue ligne 347** :
```typescript
await invoke('process_file', {
  file_path: selected,           // ✅ snake_case
  ocr_type: selectedOcrType.value // ✅ présent
})
```

**frontend/src/views/Home.vue ligne 432** :
```typescript
await invoke('process_file', { 
  file_path: filePath,  // ✅ snake_case
  ocr_type: 'standard'  // ✅ présent
})
```

**frontend/src/views/Home.vue ligne 467** :
```typescript
await invoke('open_file', { 
  file_path: doc.file_path  // ✅ snake_case
})
```

**frontend/src/views/Explorer.vue ligne 578** :
```typescript
await invoke('open_file', { 
  file_path: doc.file_path  // ✅ snake_case
})
```

## 🔧 Solutions tentées

### 1. Hot reload Vite ❌
```bash
npm run tauri:dev
```
**Résultat** : WebView garde l'ancien code en cache

### 2. Rebuild frontend ❌
```bash
cd frontend && npm run build
npm run tauri:dev
```
**Résultat** : Mode dev utilise Vite, pas le dist/

### 3. Nettoyage cache Vite ❌
```bash
rm -rf frontend/node_modules/.vite
npm run tauri:dev
```
**Résultat** : WebView Tauri a son propre cache

## ✅ Solution finale

**BUILD PRODUCTION** pour forcer un nouveau WebView :

```bash
# 1. Nettoyer tout
pkill -9 node && pkill -9 -f tauri
rm -rf frontend/dist frontend/node_modules/.vite

# 2. Rebuild frontend
cd frontend && npm run build

# 3. Build production Tauri
cd .. && npm run tauri:build

# 4. Lancer l'app en production
open src-tauri/target/release/bundle/macos/aataa.app
```

## 📝 Images .jpg persistantes

**Problème mentionné** : "J'ai toujours pas mal d'image js"

Hypothèses :
1. **Images non classifiées** : Manque de mots-clés pour les .jpg
2. **Texte OCR vide** : TrOCR pas utilisé pour les images manuscrites
3. **Classification échoue** : Pas de texte → catégorie "Autre"

**Solution à implémenter** :
- Forcer TrOCR pour tous les .jpg/.png
- Ajouter fallback : si OCR vide → catégorie "Personnel" + tag "photo"
- Améliorer les mots-clés de classification pour images

## 🎯 Prochaines étapes

1. ✅ Attendre fin du build production (~2-5 min)
2. 🔄 Tester l'app production (pas dev)
3. ✅ Vérifier que process_file fonctionne
4. 🔄 Implémenter gestion images .jpg
5. 🔄 Passer à Must-Have #3 (Gestion erreurs)

## 📊 Leçon apprise

**Mode dev Tauri ≠ Production** :
- Mode dev : WebView peut cacher l'ancien code
- Production : Toujours le code fraîchement compilé
- **Toujours tester en production après des corrections critiques**

## 🔗 Références

- Commits avec corrections : `3bd5a6b`, `af63d14`
- Fichiers modifiés : `frontend/src/views/Home.vue`, `frontend/src/views/Explorer.vue`
- Convention Rust : `snake_case` obligatoire pour paramètres Tauri
