# 🎯 Must-Have #3 : Gestion d'Erreurs Robuste

## 📋 Objectif

Transformer toutes les erreurs techniques en messages utilisateur clairs et utiles.

**Avant** :
```
Error: rusqlite error: database locked
```

**Après** :
```
Toast Material Design 3 :
⚠️ La base de données est temporairement occupée
Réessayez dans quelques secondes...
```

---

## 🎨 Composant Toast Material Design 3

### 1. Créer le composant Toast

**Fichier** : `frontend/src/components/Toast.vue`

```vue
<template>
  <Transition name="toast">
    <div 
      v-if="visible" 
      class="toast md-elevation-6"
      :class="[typeClass, positionClass]"
      role="alert"
    >
      <div class="toast-icon">
        <component :is="iconComponent" />
      </div>
      <div class="toast-content">
        <span class="toast-message label-large">{{ message }}</span>
        <span v-if="action" class="toast-action md-text-button" @click="handleAction">
          {{ action }}
        </span>
      </div>
      <button v-if="closable" class="toast-close" @click="close">
        <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor">
          <path d="M10 8.586L6.464 5.05 5.05 6.464 8.586 10l-3.536 3.536 1.414 1.414L10 11.414l3.536 3.536 1.414-1.414L11.414 10l3.536-3.536-1.414-1.414L10 8.586z"/>
        </svg>
      </button>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface Props {
  message: string
  type?: 'info' | 'success' | 'warning' | 'error'
  duration?: number
  position?: 'top' | 'bottom'
  action?: string
  closable?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  type: 'info',
  duration: 4000,
  position: 'bottom',
  closable: true
})

const emit = defineEmits<{
  close: []
  action: []
}>()

const visible = ref(false)

const typeClass = computed(() => `toast-${props.type}`)
const positionClass = computed(() => `toast-${props.position}`)

const iconComponent = computed(() => {
  switch (props.type) {
    case 'success':
      return 'svg' // ✓ icon
    case 'error':
      return 'svg' // ✗ icon
    case 'warning':
      return 'svg' // ⚠ icon
    default:
      return 'svg' // ℹ icon
  }
})

let timeout: NodeJS.Timeout | null = null

onMounted(() => {
  visible.value = true
  if (props.duration > 0) {
    timeout = setTimeout(() => {
      close()
    }, props.duration)
  }
})

function close() {
  visible.value = false
  if (timeout) clearTimeout(timeout)
  setTimeout(() => emit('close'), 300) // Attendre la fin de l'animation
}

function handleAction() {
  emit('action')
  close()
}
</script>

<style scoped>
.toast {
  position: fixed;
  left: 50%;
  transform: translateX(-50%);
  min-width: 344px;
  max-width: 672px;
  background-color: var(--md-sys-color-inverse-surface);
  color: var(--md-sys-color-inverse-on-surface);
  padding: var(--md-sys-spacing-md) var(--md-sys-spacing-lg);
  border-radius: var(--md-sys-shape-corner-small);
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-md);
  z-index: var(--md-sys-z-index-snackbar, 9999);
  box-shadow: var(--md-sys-elevation-6);
}

.toast-bottom {
  bottom: var(--md-sys-spacing-xl);
}

.toast-top {
  top: var(--md-sys-spacing-xl);
}

.toast-content {
  flex: 1;
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-lg);
}

.toast-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
}

.toast-message {
  flex: 1;
}

.toast-action {
  color: var(--md-sys-color-inverse-primary);
  text-transform: uppercase;
  cursor: pointer;
  padding: var(--md-sys-spacing-xs) var(--md-sys-spacing-sm);
  border-radius: var(--md-sys-shape-corner-small);
  transition: background-color var(--md-sys-motion-duration-short4);
}

.toast-action:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-inverse-primary) 12%, transparent);
}

.toast-close {
  background: none;
  border: none;
  color: var(--md-sys-color-inverse-on-surface);
  cursor: pointer;
  padding: var(--md-sys-spacing-xs);
  border-radius: var(--md-sys-shape-corner-small);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0.7;
  transition: opacity var(--md-sys-motion-duration-short4);
}

.toast-close:hover {
  opacity: 1;
}

/* Type variants */
.toast-success {
  background-color: var(--md-sys-color-tertiary-container);
  color: var(--md-sys-color-on-tertiary-container);
}

.toast-error {
  background-color: var(--md-sys-color-error-container);
  color: var(--md-sys-color-on-error-container);
}

.toast-warning {
  background-color: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
}

/* Animations */
.toast-enter-active,
.toast-leave-active {
  transition: all var(--md-sys-motion-duration-medium2) var(--md-sys-motion-easing-standard);
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(-50%) translateY(20px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-20px);
}
</style>
```

---

## 🎛️ Composable useToast

**Fichier** : `frontend/src/composables/useToast.ts`

```typescript
import { ref } from 'vue'

interface ToastOptions {
  message: string
  type?: 'info' | 'success' | 'warning' | 'error'
  duration?: number
  position?: 'top' | 'bottom'
  action?: string
  onAction?: () => void
}

const toasts = ref<Array<ToastOptions & { id: number }>>([])
let nextId = 0

export function useToast() {
  function show(options: ToastOptions) {
    const id = nextId++
    toasts.value.push({ ...options, id })
    
    setTimeout(() => {
      remove(id)
    }, options.duration || 4000)
  }

  function remove(id: number) {
    const index = toasts.value.findIndex(t => t.id === id)
    if (index > -1) {
      toasts.value.splice(index, 1)
    }
  }

  function success(message: string, duration = 3000) {
    show({ message, type: 'success', duration })
  }

  function error(message: string, duration = 5000) {
    show({ message, type: 'error', duration })
  }

  function warning(message: string, duration = 4000) {
    show({ message, type: 'warning', duration })
  }

  function info(message: string, duration = 4000) {
    show({ message, type: 'info', duration })
  }

  return {
    toasts,
    show,
    remove,
    success,
    error,
    warning,
    info
  }
}
```

---

## 🔧 Wrapper des Opérations Critiques

### 1. OCR avec Fallback

```rust
// src-tauri/src/commands.rs

pub async fn process_file_with_error_handling(
    file_path: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    // Essayer OCR principal (Tesseract)
    match perform_ocr(&file_path).await {
        Ok(text) if !text.trim().is_empty() => Ok(text),
        _ => {
            eprintln!("⚠️ Tesseract OCR échoué, tentative TrOCR...");
            
            // Fallback vers TrOCR pour manuscrit
            match perform_trocr_ocr(&file_path).await {
                Ok(text) if !text.trim().is_empty() => Ok(text),
                _ => Err("Impossible d'extraire du texte de ce document. Essayez avec une image de meilleure qualité.".to_string())
            }
        }
    }
}
```

### 2. Classification avec Fallback

```rust
pub async fn classify_with_fallback(
    text: String,
    state: tauri::State<'_, AppState>
) -> ClassificationResult {
    // Essayer classification sémantique
    match classify_semantic(text.clone()).await {
        Ok(result) if result["success"].as_bool().unwrap_or(false) => {
            // Conversion du résultat JSON en ClassificationResult
            // ...
        }
        Err(e) => {
            eprintln!("⚠️ Classification IA échouée: {}, fallback mots-clés", e);
            // Fallback vers mots-clés
            state.classifier.classify_detailed(&text)
        }
    }
}
```

### 3. Ouverture Fichier avec Message Clair

```rust
#[command]
pub async fn open_file(file_path: String) -> Result<(), String> {
    // Vérifier que le fichier existe
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!(
            "Le fichier '{}' est introuvable. Il a peut-être été déplacé ou supprimé.",
            file_path
        ));
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!(
                "Impossible d'ouvrir le fichier : {}. Vérifiez qu'une application par défaut est configurée.",
                e
            ))?;
    }

    // ... autres OS ...

    Ok(())
}
```

### 4. Database avec Retry

```rust
pub fn execute_with_retry<F, T>(mut operation: F, max_retries: u32) -> Result<T, String>
where
    F: FnMut() -> Result<T, rusqlite::Error>
{
    let mut attempts = 0;
    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(rusqlite::Error::SqliteFailure(err, _)) 
                if err.code == rusqlite::ErrorCode::DatabaseBusy && attempts < max_retries => 
            {
                attempts += 1;
                eprintln!("⚠️ Base de données occupée, tentative {}/{}", attempts, max_retries);
                std::thread::sleep(std::time::Duration::from_millis(100 * attempts as u64));
            }
            Err(e) => return Err(format!("Erreur base de données : {}", e))
        }
    }
}
```

---

## 🎨 Intégration Frontend

### Remplacer les alert() par des toasts

**Avant** :
```typescript
try {
  await invoke('process_file', { filePath })
} catch (error) {
  alert('Erreur: ' + error)  // ❌ Moche et bloquant
}
```

**Après** :
```typescript
const toast = useToast()

try {
  await invoke('process_file', { filePath })
  toast.success('✅ Document importé avec succès')
} catch (error) {
  toast.error(`❌ ${error}`)  // ✅ Toast élégant et non-bloquant
}
```

---

## 📊 Messages d'Erreur Utilisateur-Friendly

### Mapping Erreurs Techniques → Messages Clairs

```typescript
// frontend/src/utils/errorMessages.ts

export function formatError(error: unknown): string {
  const errorStr = String(error)
  
  // Database locked
  if (errorStr.includes('database') && errorStr.includes('locked')) {
    return 'La base de données est temporairement occupée. Réessayez dans quelques secondes.'
  }
  
  // File not found
  if (errorStr.includes('not found') || errorStr.includes('introuvable')) {
    return 'Fichier introuvable. Il a peut-être été déplacé ou supprimé.'
  }
  
  // OCR failed
  if (errorStr.includes('OCR') || errorStr.includes('extraire')) {
    return 'Impossible de lire ce document. Essayez avec une image de meilleure qualité.'
  }
  
  // Permission denied
  if (errorStr.includes('permission') || errorStr.includes('denied')) {
    return 'Permission refusée. Vérifiez les droits d\'accès au fichier.'
  }
  
  // Network/Download
  if (errorStr.includes('network') || errorStr.includes('téléchargement')) {
    return 'Problème de connexion. Vérifiez votre connexion Internet.'
  }
  
  // Fallback
  return `Une erreur est survenue : ${errorStr.substring(0, 100)}`
}
```

---

## ✅ Checklist d'Implémentation

### Backend Rust

- [ ] Ajouter `execute_with_retry()` dans `database.rs`
- [ ] Wrapper `process_file()` avec try-catch et messages clairs
- [ ] Ajouter fallback OCR (Tesseract → TrOCR)
- [ ] Ajouter fallback classification (IA → mots-clés)
- [ ] Améliorer messages d'erreur `open_file()`
- [ ] Logs structurés avec niveaux (ERROR, WARN, INFO)

### Frontend Vue

- [ ] Créer `Toast.vue` component
- [ ] Créer `useToast.ts` composable
- [ ] Créer `errorMessages.ts` utility
- [ ] Remplacer tous les `alert()` par `toast.error()`
- [ ] Remplacer tous les `console.error()` par `toast.warning()`
- [ ] Ajouter toasts de succès pour actions critiques
- [ ] Global error handler dans `App.vue`

### Tests

- [ ] Tester avec document OCR invalide
- [ ] Tester avec fichier supprimé
- [ ] Tester avec base de données verrouillée (simuler)
- [ ] Tester sans connexion Internet (modèle IA)
- [ ] Vérifier tous les messages sont clairs et en français

---

## 📈 Estimation

**Temps total** : 3 jours

- Jour 1 : Toast component + useToast + mapping erreurs (frontend)
- Jour 2 : Wrappers Rust + retry logic + logs (backend)
- Jour 3 : Tests + polish + documentation

---

## 🎯 Résultat Attendu

**Impact utilisateur** :
- ✅ Aucune erreur technique visible
- ✅ Messages clairs et actionnables
- ✅ Retry automatique pour erreurs temporaires
- ✅ Fallback graceful (OCR, classification)
- ✅ UI non-bloquante (toasts vs alerts)

**Impact technique** :
- ✅ Logs structurés pour debugging
- ✅ Moins de tickets support (~90% réduction)
- ✅ Meilleure expérience utilisateur
- ✅ App plus robuste et professionnelle
