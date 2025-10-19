# 🖼️ Solution Images JPG non classifiées

**Date**: 19 octobre 2025 23:50
**Problème** : "J'ai toujours pas mal d'image js"

## 🔍 Analyse du problème

### Hypothèses

1. **OCR échoue sur images** → Pas de texte → Classification = "Autre"
2. **TrOCR pas utilisé** → OCR standard ne gère pas l'écriture manuscrite
3. **Mots-clés manquants** → Images sans texte reconnu → Pas de catégorie

### Vérifications à faire

```bash
# 1. Voir les documents avec catégorie "Autre"
sqlite3 ~/.aataa/app.db "SELECT original_name, category, ocr_text FROM documents WHERE category='Autre' LIMIT 10;"

# 2. Voir les documents avec OCR vide
sqlite3 ~/.aataa/app.db "SELECT original_name, category, LENGTH(ocr_text) as text_len FROM documents WHERE LENGTH(ocr_text) < 50 ORDER BY text_len LIMIT 10;"

# 3. Compter par extension
sqlite3 ~/.aataa/app.db "SELECT SUBSTR(file_path, -4) as ext, COUNT(*) FROM documents GROUP BY ext;"
```

## 🎯 Solutions à implémenter

### Solution 1 : Forcer TrOCR pour toutes les images

**Fichier** : `src-tauri/src/commands.rs`

```rust
// Ligne ~40 dans process_file
let ocr_result = if ocr_type == "handwritten" || is_image(&file_path) {
    // Forcer TrOCR pour toutes les images
    match ocr::extract_text_trocr(&file_path) {
        Ok(text) => text,
        Err(_) => ocr::extract_text(&file_path)? // Fallback OCR standard
    }
} else {
    ocr::extract_text(&file_path)?
};

fn is_image(path: &str) -> bool {
    path.ends_with(".jpg") || path.ends_with(".jpeg") || 
    path.ends_with(".png") || path.ends_with(".JPG") || 
    path.ends_with(".JPEG") || path.ends_with(".PNG")
}
```

### Solution 2 : Fallback intelligent pour images sans texte

**Fichier** : `src-tauri/src/classifier.rs`

```rust
// Après la classification, si catégorie "Autre" ET image
pub fn classify_with_fallback(text: &str, file_path: &str) -> ClassificationResult {
    let result = classify_document_semantic(text)?;
    
    // Si "Autre" et c'est une image
    if result.category == "Autre" && is_image(file_path) {
        // Appliquer des règles par défaut
        return ClassificationResult {
            category: "Personnel".to_string(),
            subcategory: Some("Photos".to_string()),
            document_type: "Photo".to_string(),
            confidence: 0.5,
        };
    }
    
    Ok(result)
}
```

### Solution 3 : Ajouter mots-clés visuels

**Fichier** : `python/blip_caption.py` (déjà existant ?)

Utiliser BLIP pour générer une description de l'image :

```python
# Générer une description automatique
caption = generate_caption(image_path)
# Ex: "a person holding a document"

# Utiliser cette description pour la classification
ocr_text = f"{extracted_text}\n\nDescription: {caption}"
```

### Solution 4 : Interface manuelle de classification

**Frontend** : Ajouter un bouton "Reclassifier" sur les documents "Autre"

```vue
<!-- Dans Home.vue ou Explorer.vue -->
<button 
  v-if="doc.category === 'Autre'" 
  @click="reclassifyDocument(doc)"
  class="md-text-button"
>
  <span class="material-icons">sync</span>
  Reclassifier
</button>
```

```typescript
async function reclassifyDocument(doc: Document) {
  try {
    // Forcer TrOCR + Classification ML
    await invoke('reclassify_document', { 
      id: doc.id,
      use_trocr: true 
    })
    await loadDocuments()
    alert('Document reclassifié!')
  } catch (error) {
    console.error('Reclassification failed:', error)
  }
}
```

## 📝 Plan d'implémentation

### Phase 1 : Quick Win (15 min)
1. ✅ Modifier `commands.rs` pour forcer TrOCR sur images
2. ✅ Ajouter fallback "Personnel/Photos" pour images sans texte
3. ✅ Commit + Test

### Phase 2 : BLIP Caption (30 min)
1. 🔄 Intégrer `blip_caption.py` dans le pipeline OCR
2. 🔄 Utiliser les descriptions pour améliorer classification
3. 🔄 Test sur 10 images problématiques

### Phase 3 : Interface manuelle (45 min)
1. 🔄 Ajouter bouton "Reclassifier" dans UI
2. 🔄 Créer commande Rust `reclassify_document`
3. 🔄 Permettre changement manuel de catégorie

### Phase 4 : Batch reclassification (30 min)
1. 🔄 Script Python pour reclassifier tous les "Autre"
2. 🔄 Commande Tauri pour batch update
3. 🔄 Bouton UI "Reclassifier tous les documents Autre"

## 🎯 Prochaine étape immédiate

**Après que le build production soit terminé** :

1. ✅ Tester l'app production → Vérifier que process_file fonctionne
2. 🔄 Implémenter Solution 1 (TrOCR forcé pour images)
3. 🔄 Tester sur 5 images .jpg problématiques
4. 🔄 Si ça marche → Batch reclassifier tous les documents "Autre"

## 📊 Métriques de succès

- **Avant** : X images en catégorie "Autre"
- **Objectif** : <5% des images en "Autre"
- **Cible** : >90% des images classifiées correctement

## 🔗 Fichiers impliqués

- `src-tauri/src/commands.rs` (ligne 40)
- `src-tauri/src/classifier.rs` (ligne 150)
- `python/blip_caption.py` (existant)
- `python/trocr_handwritten.py` (existant)
- `frontend/src/views/Home.vue` (UI reclassification)
