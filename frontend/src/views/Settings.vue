<template>
  <div class="settings">
    <!-- Top App Bar -->
    <header class="md-top-app-bar">
      <div class="app-bar-content">
        <button @click="goBack" class="md-icon-button">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M19 12H5M12 19l-7-7 7-7" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
        <div class="app-bar-title">
          <h1 class="headline-medium">Paramètres</h1>
          <span class="body-small app-subtitle">Configuration de l'application</span>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <div class="settings-content">
      <!-- Archive Path Section -->
      <section class="settings-section md-card animate-fade-in">
        <div class="section-header">
          <div class="section-icon">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <path d="M26 6H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2z" stroke-width="2"/>
              <path d="M4 12h24M12 6v6M20 6v6" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </div>
          <div>
            <h2 class="title-large">Chemin d'archive</h2>
            <p class="body-medium section-description">Emplacement où les documents sont stockés</p>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <label class="label-large">Dossier d'archivage</label>
            <p class="body-small current-path">{{ archivePath || 'Non défini' }}</p>
          </div>
          <button @click="selectArchivePath" class="md-filled-button md-ripple">
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor">
              <path d="M16 10v5a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1v-5M13 6l-3-3m0 0L7 6m3-3v10" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Changer le dossier
          </button>
        </div>

        <div v-if="archivePathChanged" class="success-message">
          <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
            <circle cx="10" cy="10" r="9" stroke-width="2"/>
            <path d="M6 10l3 3 5-6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span class="body-medium">Chemin d'archive mis à jour avec succès !</span>
        </div>
      </section>

      <!-- OCR Settings Section -->
      <section class="settings-section md-card animate-slide-in-up" style="animation-delay: 50ms">
        <div class="section-header">
          <div class="section-icon">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <rect x="6" y="6" width="20" height="20" rx="2" stroke-width="2"/>
              <path d="M10 14h12M10 18h8" stroke-width="2" stroke-linecap="round"/>
              <circle cx="22" cy="22" r="4" stroke-width="2"/>
              <path d="M25 25l2 2" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </div>
          <div>
            <h2 class="title-large">OCR & Reconnaissance</h2>
            <p class="body-medium section-description">Paramètres de reconnaissance de texte</p>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <label class="label-large">Type OCR par défaut</label>
            <p class="body-small">Type de reconnaissance utilisé lors de l'import</p>
          </div>
          <select v-model="defaultOcrType" class="setting-select">
            <option value="standard">Standard (Tesseract)</option>
            <option value="handwritten">Manuscrit (TrOCR)</option>
            <option value="printed">Imprimé (TrOCR)</option>
            <option value="caption">Légende (BLIP)</option>
          </select>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <label class="label-large">Langues OCR</label>
            <p class="body-small">Langues utilisées pour la reconnaissance (Tesseract)</p>
          </div>
          <div class="language-chips">
            <button class="md-chip md-chip-selected">Français</button>
            <button class="md-chip">Anglais</button>
            <button class="md-chip">Allemand</button>
            <button class="md-chip">Espagnol</button>
          </div>
        </div>
      </section>

      <!-- Categories & Tags Section -->
      <section class="settings-section md-card animate-slide-in-up" style="animation-delay: 100ms">
        <div class="section-header">
          <div class="section-icon">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <path d="M4 8h24M4 16h24M4 24h24" stroke-width="2" stroke-linecap="round"/>
              <circle cx="8" cy="8" r="2" fill="currentColor"/>
              <circle cx="8" cy="16" r="2" fill="currentColor"/>
              <circle cx="8" cy="24" r="2" fill="currentColor"/>
            </svg>
          </div>
          <div>
            <h2 class="title-large">Catégories et sous-catégories</h2>
            <p class="body-medium section-description">Personnalisez vos catégories de documents</p>
          </div>
        </div>

        <SubcategoryManager />
      </section>

      <!-- Classification Keywords Section -->
      <section class="settings-section md-card animate-slide-in-up" style="animation-delay: 125ms">
        <div class="section-header">
          <div class="section-icon">
            <span class="material-icons" style="font-size: 32px; color: currentColor;">label</span>
          </div>
          <div>
            <h2 class="title-large">Mots-clés de classification</h2>
            <p class="body-medium section-description">Personnalisez les mots-clés utilisés pour la classification automatique</p>
          </div>
        </div>

        <KeywordsManager />
      </section>

      <!-- App Information Section -->
      <section class="settings-section md-card animate-slide-in-up" style="animation-delay: 150ms">
        <div class="section-header">
          <div class="section-icon">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <circle cx="16" cy="16" r="12" stroke-width="2"/>
              <path d="M16 12v8M16 8v.01" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </div>
          <div>
            <h2 class="title-large">À propos</h2>
            <p class="body-medium section-description">Informations sur l'application</p>
          </div>
        </div>

        <div class="info-grid">
          <div class="info-item">
            <span class="label-large">Application</span>
            <span class="body-large">AATAA</span>
          </div>
          <div class="info-item">
            <span class="label-large">Version</span>
            <span class="body-large">1.0.0</span>
          </div>
          <div class="info-item">
            <span class="label-large">Framework</span>
            <span class="body-large">Tauri 2.8 + Vue 3</span>
          </div>
          <div class="info-item">
            <span class="label-large">Design</span>
            <span class="body-large">Material Design 3</span>
          </div>
        </div>
      </section>

      <!-- Danger Zone -->
      <section class="settings-section md-card danger-section animate-slide-in-up" style="animation-delay: 200ms">
        <div class="section-header">
          <div class="section-icon danger-icon">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <path d="M16 4L4 28h24L16 4z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M16 12v8M16 24v.01" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </div>
          <div>
            <h2 class="title-large">Zone dangereuse</h2>
            <p class="body-medium section-description">Actions irréversibles</p>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-info">
            <label class="label-large">Réinitialiser l'application</label>
            <p class="body-small">Supprime tous les documents et paramètres</p>
          </div>
          <button @click="confirmReset = true" class="md-outlined-button danger-button md-ripple">
            <svg width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor">
              <path d="M1 4v4h4M17 14v-4h-4" stroke-width="2" stroke-linecap="round"/>
              <path d="M3.5 9A7 7 0 1 1 3 12" stroke-width="2" stroke-linecap="round"/>
            </svg>
            Réinitialiser
          </button>
        </div>
      </section>
    </div>

    <!-- Confirmation Modal: Reset -->
    <div v-if="confirmReset" class="md-modal-overlay" @click="confirmReset = false">
      <div class="md-modal confirm-modal animate-scale-in" @click.stop>
        <div class="modal-icon error-icon">
          <svg width="48" height="48" viewBox="0 0 48 48" fill="none" stroke="currentColor">
            <circle cx="24" cy="24" r="22" stroke-width="3"/>
            <path d="M24 12v12M24 30v.01" stroke-width="3" stroke-linecap="round"/>
          </svg>
        </div>

        <div class="md-modal-content">
          <h2 class="title-large">Réinitialiser l'application ?</h2>
          <p class="body-large modal-text">
            Cette action supprimera <strong>tous les documents et paramètres</strong>.
            Cette action est <strong>irréversible</strong>.
          </p>
        </div>

        <div class="md-modal-footer">
          <button @click="confirmReset = false" class="md-text-button">
            Annuler
          </button>
          <button @click="resetApp" class="md-filled-button md-ripple error-button">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
              <path d="M1 4v4h4M15 12v-4h-4" stroke-width="2" stroke-linecap="round"/>
              <path d="M3 9A6 6 0 1 1 2.5 11" stroke-width="2" stroke-linecap="round"/>
            </svg>
            Réinitialiser
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import SubcategoryManager from '../components/SubcategoryManager.vue'
import KeywordsManager from '../components/KeywordsManager.vue'

const router = useRouter()
const archivePath = ref('')
const archivePathChanged = ref(false)
const defaultOcrType = ref('standard')
const confirmReset = ref(false)

// Load settings
async function loadSettings() {
  try {
    const path = await invoke<string>('get_archive_path')
    archivePath.value = path

    // Load OCR type from localStorage
    const savedOcrType = localStorage.getItem('defaultOcrType')
    if (savedOcrType) {
      defaultOcrType.value = savedOcrType
    }
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
}

// Select archive path
async function selectArchivePath() {
  try {
    const selected = await open({
      directory: true,
      title: 'Sélectionner le dossier d\'archive'
    })

    if (selected) {
      await invoke('set_archive_path', { path: selected })
      archivePath.value = selected
      archivePathChanged.value = true

      // Hide success message after 3 seconds
      setTimeout(() => {
        archivePathChanged.value = false
      }, 3000)
    }
  } catch (error) {
    console.error('Failed to set archive path:', error)
    alert(`Erreur lors de la configuration: ${error}`)
  }
}

// Reset app (TODO: Implement backend command)
function resetApp() {
  alert('Fonctionnalité de réinitialisation à implémenter côté backend')
  confirmReset.value = false
}

// Go back
function goBack() {
  router.push('/')
}

onMounted(() => {
  loadSettings()
})

// Watch for OCR type changes and save to localStorage
watch(defaultOcrType, (newValue) => {
  localStorage.setItem('defaultOcrType', newValue)
})
</script>

<style scoped>
.settings {
  min-height: 100vh;
  background-color: var(--md-sys-color-background);
}

/* Top App Bar */
.md-top-app-bar {
  position: sticky;
  top: 0;
  z-index: var(--md-sys-z-index-app-bar);
  background-color: var(--md-sys-color-surface);
  border-bottom: 1px solid var(--md-sys-color-outline-variant);
  padding: var(--md-sys-spacing-md) var(--md-sys-spacing-lg);
}

.app-bar-content {
  max-width: 1000px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-md);
}

.app-bar-title {
  flex: 1;
}

.app-bar-title h1 {
  color: var(--md-sys-color-on-surface);
  margin: 0;
}

.app-subtitle {
  color: var(--md-sys-color-on-surface-variant);
}

/* Main Content */
.settings-content {
  max-width: 1000px;
  margin: 0 auto;
  padding: var(--md-sys-spacing-xl) var(--md-sys-spacing-lg);
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-lg);
}

/* Settings Section */
.settings-section {
  padding: var(--md-sys-spacing-xl);
}

.section-header {
  display: flex;
  gap: var(--md-sys-spacing-md);
  margin-bottom: var(--md-sys-spacing-xl);
  align-items: flex-start;
}

.section-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--md-sys-color-primary-container);
  color: var(--md-sys-color-on-primary-container);
  border-radius: var(--md-sys-shape-corner-medium);
  flex-shrink: 0;
}

.section-header h2 {
  margin: 0 0 var(--md-sys-spacing-xs) 0;
  color: var(--md-sys-color-on-surface);
}

.section-description {
  color: var(--md-sys-color-on-surface-variant);
  margin: 0;
}

/* Setting Item */
.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--md-sys-spacing-lg);
  padding: var(--md-sys-spacing-lg) 0;
  border-top: 1px solid var(--md-sys-color-outline-variant);
}

.setting-item:first-of-type {
  border-top: none;
}

.setting-info {
  flex: 1;
}

.setting-info label {
  display: block;
  margin-bottom: var(--md-sys-spacing-xs);
  color: var(--md-sys-color-on-surface);
}

.setting-info p {
  margin: 0;
  color: var(--md-sys-color-on-surface-variant);
}

.current-path {
  font-family: 'Monaco', 'Courier New', monospace;
  background-color: var(--md-sys-color-surface-container);
  padding: var(--md-sys-spacing-xs) var(--md-sys-spacing-sm);
  border-radius: var(--md-sys-shape-corner-extra-small);
  margin-top: var(--md-sys-spacing-xs);
  display: inline-block;
}

/* Select */
.setting-select {
  padding: var(--md-sys-spacing-sm) var(--md-sys-spacing-md);
  background-color: var(--md-sys-color-surface-container-high);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  font-size: var(--md-sys-typescale-body-large-font-size);
  color: var(--md-sys-color-on-surface);
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.setting-select:focus {
  outline: none;
  border-color: var(--md-sys-color-primary);
  border-width: 2px;
}

/* Language Chips */
.language-chips {
  display: flex;
  gap: var(--md-sys-spacing-sm);
  flex-wrap: wrap;
}

/* Success Message */
.success-message {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-sm);
  padding: var(--md-sys-spacing-md);
  background-color: color-mix(in srgb, var(--md-sys-color-primary) 12%, transparent);
  color: var(--md-sys-color-primary);
  border-radius: var(--md-sys-shape-corner-small);
  margin-top: var(--md-sys-spacing-md);
  animation: slideInUp var(--md-sys-motion-duration-medium3) var(--md-sys-motion-easing-emphasized);
}

/* Info Grid */
.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--md-sys-spacing-lg);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-xs);
}

.info-item span:first-child {
  color: var(--md-sys-color-on-surface-variant);
}

.info-item span:last-child {
  color: var(--md-sys-color-on-surface);
}

/* Danger Zone */
.danger-section {
  border: 1px solid var(--md-sys-color-error);
}

.danger-icon {
  background-color: var(--md-sys-color-error-container);
  color: var(--md-sys-color-on-error-container);
}

.danger-button {
  color: var(--md-sys-color-error);
  border-color: var(--md-sys-color-error);
}

.danger-button:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-error) 8%, transparent);
}

/* Confirmation Modal */
.confirm-modal {
  max-width: 500px;
  text-align: center;
}

.modal-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto var(--md-sys-spacing-lg);
}

.error-icon {
  color: var(--md-sys-color-error);
}

.modal-text {
  color: var(--md-sys-color-on-surface-variant);
  text-align: center;
}

.error-button {
  background-color: var(--md-sys-color-error);
  color: var(--md-sys-color-on-error);
}

.error-button:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-error) 92%, white);
}

/* Responsive */
@media (max-width: 768px) {
  .settings-content {
    padding: var(--md-sys-spacing-lg) var(--md-sys-spacing-md);
  }

  .setting-item {
    flex-direction: column;
    align-items: flex-start;
  }

  .setting-item button,
  .setting-select {
    width: 100%;
  }

  .info-grid {
    grid-template-columns: 1fr;
  }
}
</style>
