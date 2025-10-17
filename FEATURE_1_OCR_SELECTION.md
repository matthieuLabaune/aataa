# Feature #1 : Sélection du type OCR à l'import

## 📋 Description
Permet à l'utilisateur de choisir le type d'OCR à utiliser lors de l'importation d'un document.

**Option implémentée** : Option A - Select discret avec valeur par défaut depuis les paramètres

## ✨ Fonctionnalités

### Interface utilisateur (Home.vue)
- **Select OCR** : Dropdown Material Design 3 situé entre les stats cards et les boutons d'action
- **4 types d'OCR disponibles** :
  - 🔤 **Standard** (Tesseract) - Pour les documents imprimés standards
  - ✍️ **Écriture manuscrite** (TrOCR) - Pour les documents manuscrits
  - 📄 **Imprimé** (TrOCR) - Pour les documents imprimés de haute qualité
  - 🖼️ **Légende d'image** (BLIP) - Pour générer des descriptions d'images

### Paramètres (Settings.vue)
- **Type OCR par défaut** : Select dans la section "OCR & Reconnaissance"
- **Sauvegarde automatique** : La valeur sélectionnée est enregistrée dans localStorage
- **Synchronisation** : Le type par défaut est automatiquement utilisé dans Home.vue

## 🔧 Implémentation technique

### Frontend

#### Home.vue
```vue
<!-- Select OCR -->
<div class="ocr-selection">
  <label for="ocr-type" class="body-medium ocr-label">Type d'OCR</label>
  <select id="ocr-type" v-model="selectedOcrType" class="md-select">
    <option value="standard">🔤 Standard (Tesseract)</option>
    <option value="handwritten">✍️ Écriture manuscrite (TrOCR)</option>
    <option value="printed">📄 Imprimé (TrOCR)</option>
    <option value="caption">🖼️ Légende d'image (BLIP)</option>
  </select>
</div>
```

**Variables d'état** :
```typescript
const selectedOcrType = ref('standard') // Type OCR sélectionné
```

**Fonction d'import modifiée** :
```typescript
async function selectFile() {
  const selected = await open({
    multiple: false,
    filters: [{
      name: 'Documents',
      extensions: ['pdf', 'png', 'jpg', 'jpeg']
    }]
  })
  
  if (selected) {
    await invoke('process_file', { 
      filePath: selected,
      ocrType: selectedOcrType.value  // ✅ Passage du type OCR
    })
    await loadDocuments()
  }
}
```

**Chargement de la valeur par défaut** :
```typescript
onMounted(() => {
  loadDocuments()
  
  // Charge le type OCR par défaut depuis Settings
  const savedOcrType = localStorage.getItem('defaultOcrType')
  if (savedOcrType) {
    selectedOcrType.value = savedOcrType
  }
})
```

#### Settings.vue
**Sauvegarde automatique dans localStorage** :
```typescript
watch(defaultOcrType, (newValue) => {
  localStorage.setItem('defaultOcrType', newValue)
})
```

**Chargement depuis localStorage** :
```typescript
async function loadSettings() {
  const path = await invoke<string>('get_archive_path')
  archivePath.value = path
  
  // Charge le type OCR depuis localStorage
  const savedOcrType = localStorage.getItem('defaultOcrType')
  if (savedOcrType) {
    defaultOcrType.value = savedOcrType
  }
}
```

### Backend (Rust)

#### Signature de la commande `process_file`
```rust
#[tauri::command]
pub async fn process_file(
    file_path: String,
    ocr_type: String, // ✅ Nouveau paramètre
    state: tauri::State<'_, AppState>,
) -> Result<Document, String>
```

**Paramètre `ocrType`** :
- **Type** : `String`
- **Valeurs possibles** : `"standard"`, `"handwritten"`, `"printed"`, `"caption"`
- **Statut** : Accepté mais pas encore implémenté dans l'exécution

**TODO Backend** :
```rust
// TODO: Implémenter les différents moteurs OCR selon ocr_type
// - "standard": Tesseract (implémentation actuelle)
// - "handwritten": TrOCR microsoft/trocr-base-handwritten
// - "printed": TrOCR microsoft/trocr-base-printed
// - "caption": BLIP Salesforce/blip-image-captioning-base
```

Actuellement, seul Tesseract est utilisé quel que soit le type sélectionné. L'infrastructure est en place pour ajouter TrOCR et BLIP ultérieurement.

## 🎨 Design Material Design 3

### Select OCR (.md-select)
```css
.md-select {
  padding: var(--md-sys-spacing-md) var(--md-sys-spacing-lg);
  background-color: var(--md-sys-color-surface-variant);
  color: var(--md-sys-color-on-surface);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  font-family: var(--md-sys-typescale-body-large-font-family);
  font-size: var(--md-sys-typescale-body-large-font-size);
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
}

.md-select:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 8%, var(--md-sys-color-surface-variant));
}

.md-select:focus {
  outline: 2px solid var(--md-sys-color-primary);
  outline-offset: 2px;
}
```

### Conteneur OCR
```css
.ocr-selection {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-xs);
  margin-bottom: var(--md-sys-spacing-lg);
  max-width: 400px;
}

.ocr-label {
  color: var(--md-sys-color-on-surface-variant);
  font-weight: 500;
}
```

## 🔄 Flux de données

```
1. L'utilisateur ouvre Settings
   └─> defaultOcrType chargé depuis localStorage
   └─> Valeur affichée dans le select

2. L'utilisateur change le type OCR dans Settings
   └─> watch() détecte le changement
   └─> Sauvegarde dans localStorage.setItem('defaultOcrType', newValue)

3. L'utilisateur retourne sur Home
   └─> onMounted() charge localStorage.getItem('defaultOcrType')
   └─> selectedOcrType.value est pré-rempli avec la valeur des Settings

4. L'utilisateur importe un document
   └─> selectFile() appelle invoke('process_file', { filePath, ocrType })
   └─> Backend reçoit le type OCR sélectionné
   └─> (Pour l'instant, utilise toujours Tesseract)
```

## 📊 État d'implémentation

### ✅ Complété
- [x] Ajout du paramètre `ocrType` à la commande `process_file` (backend)
- [x] Création du select OCR dans Home.vue
- [x] Design Material Design 3 pour le select
- [x] Animation slide-in-up avec délai
- [x] Sauvegarde automatique du type par défaut dans localStorage
- [x] Chargement automatique du type par défaut depuis Settings
- [x] Passage du type OCR sélectionné à la commande backend
- [x] Documentation complète

### ⏳ À faire (implémentation future)
- [ ] Intégrer TrOCR pour l'écriture manuscrite
- [ ] Intégrer TrOCR pour l'imprimé haute qualité
- [ ] Intégrer BLIP pour les légendes d'images
- [ ] Ajouter la sélection OCR pour `scanFolder` (traitement batch)
- [ ] Tests unitaires pour les différents types OCR

## 🎯 Utilisation

### Pour l'utilisateur final

1. **Configuration initiale (optionnel)** :
   - Aller dans ⚙️ **Paramètres**
   - Section **OCR & Reconnaissance**
   - Choisir le type OCR par défaut

2. **Import d'un document** :
   - Aller sur 🏠 **Home**
   - Le select OCR affiche le type par défaut depuis les paramètres
   - Modifier le type OCR si besoin (pour ce document uniquement)
   - Cliquer sur **"Importer un document"**
   - Sélectionner un fichier
   - Le document est traité avec le type OCR choisi

### Pour les développeurs

**Ajouter un nouveau type OCR** :

1. Ajouter l'option dans le select (Home.vue) :
```vue
<option value="nouveau_type">🆕 Nouveau Type (Engine)</option>
```

2. Ajouter l'option dans Settings.vue :
```vue
<option value="nouveau_type">Nouveau Type (Engine)</option>
```

3. Implémenter la logique backend dans `src-tauri/src/commands.rs` :
```rust
let ocr_text = match ocr_type.as_str() {
    "standard" => ocr.extract_text_from_image(&path)?,
    "handwritten" => trocr_handwritten(&path)?,
    "printed" => trocr_printed(&path)?,
    "caption" => blip_caption(&path)?,
    "nouveau_type" => nouveau_engine(&path)?,
    _ => ocr.extract_text_from_image(&path)? // Fallback
};
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Settings.vue                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Section OCR & Reconnaissance                         │  │
│  │  ┌─────────────────────────────────┐                  │  │
│  │  │  defaultOcrType (ref)           │                  │  │
│  │  │  ↓ watch()                      │                  │  │
│  │  │  localStorage.setItem()         │                  │  │
│  │  └─────────────────────────────────┘                  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
                   localStorage (persistent)
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                          Home.vue                            │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  onMounted()                                          │  │
│  │  ↓ localStorage.getItem('defaultOcrType')            │  │
│  │  ↓ selectedOcrType.value = savedOcrType              │  │
│  │  ┌─────────────────────────────────┐                 │  │
│  │  │  Select OCR (v-model)           │                 │  │
│  │  │  - Standard                     │                 │  │
│  │  │  - Handwritten                  │                 │  │
│  │  │  - Printed                      │                 │  │
│  │  │  - Caption                      │                 │  │
│  │  └─────────────────────────────────┘                 │  │
│  │                  ↓                                    │  │
│  │  ┌─────────────────────────────────┐                 │  │
│  │  │  selectFile()                   │                 │  │
│  │  │  invoke('process_file', {       │                 │  │
│  │  │    filePath,                    │                 │  │
│  │  │    ocrType: selectedOcrType     │                 │  │
│  │  │  })                             │                 │  │
│  │  └─────────────────────────────────┘                 │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ↓
                      Tauri IPC Bridge
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   Backend (commands.rs)                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  process_file(file_path, ocr_type)                    │  │
│  │  ┌─────────────────────────────────┐                  │  │
│  │  │  match ocr_type {               │                  │  │
│  │  │    "standard" => Tesseract      │  ✅ Implémenté  │  │
│  │  │    "handwritten" => TrOCR       │  ⏳ TODO        │  │
│  │  │    "printed" => TrOCR           │  ⏳ TODO        │  │
│  │  │    "caption" => BLIP            │  ⏳ TODO        │  │
│  │  │  }                              │                  │  │
│  │  └─────────────────────────────────┘                  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## 📝 Notes

- **Persistance** : Le type OCR par défaut est sauvegardé dans `localStorage` (clé : `defaultOcrType`)
- **Valeur par défaut** : Si aucune valeur n'est trouvée dans localStorage, le type "standard" est utilisé
- **Backend** : L'infrastructure est prête, mais seul Tesseract est actuellement implémenté
- **Extensibilité** : Ajouter de nouveaux types OCR ne nécessite que 3 modifications (Home.vue, Settings.vue, commands.rs)

## 🚀 Prochaines étapes recommandées

1. **Implémenter TrOCR** pour l'écriture manuscrite et l'imprimé
2. **Implémenter BLIP** pour les légendes d'images
3. **Ajouter des indicateurs visuels** pour montrer quel OCR est utilisé pendant le traitement
4. **Créer des tests unitaires** pour valider chaque type OCR
5. **Optimiser les performances** en cachant les modèles ML chargés
