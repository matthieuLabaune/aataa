<template>
  <div class="home">
    <!-- Top App Bar -->
    <header class="md-top-app-bar">
      <div class="app-bar-content">
        <div class="app-bar-title">
          <span class="material-icons app-icon">lock</span>
          <h1 class="headline-medium">PaperVault</h1>
          <span class="body-small app-subtitle">Coffre-fort documentaire</span>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <div class="home-content">
      <!-- Drag & Drop Zone with OCR Selection -->
      <div class="import-section animate-slide-in-up">
        <div 
          class="drag-drop-zone"
          :class="{ 'drag-over': isDragging }"
          @dragenter.prevent="handleDragEnter"
          @dragover.prevent="handleDragOver"
          @dragleave.prevent="handleDragLeave"
          @drop.prevent="handleDrop"
          @click="selectFile"
        >
          <svg class="drag-drop-icon" width="48" height="48" viewBox="0 0 48 48" fill="none" stroke="currentColor">
            <path d="M40 30v6a4 4 0 0 1-4 4H12a4 4 0 0 1-4-4v-6M32 14l-8-8m0 0l-8 8m8-8v28" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <h3 class="title-medium">Glissez vos documents ici</h3>
          <p class="body-medium">ou cliquez pour sélectionner un fichier</p>
          <p class="body-small drag-drop-hint">PDF, JPG, PNG - Max 50 Mo</p>
        </div>

        <!-- Action Buttons with OCR Selection -->
        <div class="action-buttons">
          <button @click="selectFile" class="md-filled-button md-ripple">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
              <path d="M17 13v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2M14 7l-4-4m0 0L6 7m4-4v12" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Importer un document
          </button>
          <button @click="scanFolder" class="md-outlined-button md-ripple">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
              <path d="M9 1H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-9M9 1l8 8M9 1v8h8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Scanner un dossier
          </button>
          
          <!-- OCR Type Selection -->
          <div class="ocr-type-selector">
            <label for="ocr-type" class="body-medium">OCR :</label>
            <select id="ocr-type" v-model="selectedOcrType" class="ocr-select">
              <option value="standard">Standard</option>
              <option value="handwritten">Manuscrit</option>
              <option value="printed">Imprimé</option>
              <option value="caption">Légende</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Dashboard Section -->
      <div class="dashboard-section animate-slide-in-up" :style="{ animationDelay: documents.length > 0 ? '100ms' : '50ms' }">
        <h2 class="title-large section-title">Synthèse</h2>
        <div class="stats-grid">
          <div class="md-card stat-card">
            <div class="stat-icon">📚</div>
            <div class="stat-value title-large">{{ stats.total }}</div>
            <div class="stat-label body-medium">Documents</div>
          </div>
          <div class="md-card stat-card">
            <div class="stat-icon">📂</div>
            <div class="stat-value title-large">{{ stats.types }}</div>
            <div class="stat-label body-medium">Types</div>
          </div>
          <div class="md-card stat-card">
            <div class="stat-icon">🏷️</div>
            <div class="stat-value title-large">{{ stats.tags }}</div>
            <div class="stat-label body-medium">Tags</div>
          </div>
          <div class="md-card stat-card">
            <div class="stat-icon">💾</div>
            <div class="stat-value title-large">{{ formatSize(stats.size) }}</div>
            <div class="stat-label body-medium">Espace</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Document Detail Modal -->
    <div v-if="selectedDocument" class="md-modal-overlay" @click="selectedDocument = null">
      <div class="md-modal animate-scale-in" @click.stop>
        <div class="md-modal-header">
          <h2 class="title-large">{{ selectedDocument.new_name }}</h2>
          <button @click="selectedDocument = null" class="md-icon-button">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
              <path d="M15 5L5 15M5 5l10 10" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </button>
        </div>

        <div class="md-modal-content">
          <div class="modal-info-grid">
            <div class="info-item">
              <span class="label-large info-label">Catégorie</span>
              <span class="body-large">
                <span class="material-icons" style="font-size: 18px; vertical-align: middle;">
                  {{ getCategoryIcon(selectedDocument.category) }}
                </span>
                {{ selectedDocument.category }}
                <span v-if="selectedDocument.confidence" class="confidence-badge" :class="getConfidenceClass(selectedDocument.confidence)">
                  {{ Math.round(selectedDocument.confidence * 100) }}%
                </span>
              </span>
            </div>
            <div v-if="selectedDocument.subcategory" class="info-item">
              <span class="label-large info-label">Sous-catégorie</span>
              <span class="body-large">{{ selectedDocument.subcategory }}</span>
            </div>
            <div class="info-item">
              <span class="label-large info-label">Type</span>
              <span class="body-large">{{ selectedDocument.document_type }}</span>
            </div>
            <div class="info-item">
              <span class="label-large info-label">Taille</span>
              <span class="body-large">{{ formatSize(selectedDocument.file_size) }}</span>
            </div>
            <div class="info-item">
              <span class="label-large info-label">Date</span>
              <span class="body-large">{{ formatDate(selectedDocument.created_at) }}</span>
            </div>
            <div class="info-item">
              <span class="label-large info-label">Nom original</span>
              <span class="body-medium">{{ selectedDocument.original_name }}</span>
            </div>
          </div>

          <div v-if="selectedDocument.tags.length > 0" class="modal-tags">
            <span class="label-large">Tags</span>
            <div class="tags-list">
              <span v-for="tag in selectedDocument.tags" :key="tag" class="md-chip">
                {{ tag }}
              </span>
            </div>
          </div>

          <div v-if="selectedDocument.ocr_text" class="modal-ocr">
            <span class="label-large">Texte OCR</span>
            <p class="body-medium ocr-text">{{ selectedDocument.ocr_text.slice(0, 300) }}...</p>
          </div>
        </div>

        <div class="md-modal-footer">
          <button @click="selectedDocument = null" class="md-text-button">
            Fermer
          </button>
          <button @click="editDocument(selectedDocument)" class="md-outlined-button md-ripple">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
              <path d="M11 2l3 3-9 9H2v-3l9-9z" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Éditer
          </button>
          <button @click="openDocument(selectedDocument)" class="md-filled-button md-ripple">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
              <path d="M14 9v4a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h4M10 2h4v4M7 9l7-7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Ouvrir le fichier
          </button>
        </div>
      </div>
    </div>

    <!-- Edit Metadata Modal -->
    <div v-if="editingDocument" class="md-modal-overlay" @click="cancelEdit">
      <div class="md-modal edit-modal animate-scale-in" @click.stop>
        <div class="md-modal-header">
          <h2 class="title-large">Éditer le document</h2>
          <button @click="cancelEdit" class="md-icon-button">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
              <path d="M15 5L5 15M5 5l10 10" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </button>
        </div>

        <div class="md-modal-content">
          <!-- Name Field -->
          <div class="form-field">
            <label class="label-large">Nom du document</label>
            <div class="md-text-field">
              <input
                v-model="editForm.name"
                type="text"
                placeholder="Nom du document"
              />
            </div>
          </div>

          <!-- Type Field -->
          <div class="form-field">
            <label class="label-large">Type</label>
            <div class="md-text-field">
              <input
                v-model="editForm.type"
                type="text"
                placeholder="Facture, Contrat, etc."
              />
            </div>
          </div>

          <!-- Tags Field -->
          <div class="form-field">
            <label class="label-large">Tags</label>
            <div class="tags-editor">
              <div class="current-tags">
                <span
                  v-for="(tag, index) in editForm.tags"
                  :key="index"
                  class="md-chip tag-editable"
                >
                  {{ tag }}
                  <button @click="removeTag(index)" class="tag-remove">
                    <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor">
                      <path d="M9 3L3 9M3 3l6 6" stroke-width="1.5" stroke-linecap="round"/>
                    </svg>
                  </button>
                </span>
              </div>
              <div class="add-tag-input">
                <input
                  v-model="newTag"
                  type="text"
                  placeholder="Ajouter un tag..."
                  @keyup.enter="addTag"
                  class="tag-input"
                />
                <button @click="addTag" class="md-icon-button" :disabled="!newTag.trim()">
                  <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
                    <path d="M10 5v10M5 10h10" stroke-width="2" stroke-linecap="round"/>
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <!-- Notes Field -->
          <div class="form-field">
            <label class="label-large">Notes</label>
            <textarea
              v-model="editForm.notes"
              placeholder="Ajoutez des notes sur ce document..."
              class="notes-textarea"
              rows="4"
            ></textarea>
          </div>
        </div>

        <div class="md-modal-footer">
          <button @click="cancelEdit" class="md-text-button">
            Annuler
          </button>
          <button @click="saveMetadata" class="md-filled-button md-ripple">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
              <path d="M13 2L6 9l-3-3" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Enregistrer
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useProcessingState } from '../composables/useProcessingState'
import type { Document, DocumentStats } from '../types/document'

const documents = ref<Document[]>([])
const loading = ref(true)
const selectedDocument = ref<Document | null>(null)
const editingDocument = ref<Document | null>(null)
const selectedOcrType = ref('standard') // Default OCR type

const isDragging = ref(false)

// Edit form state
const editForm = ref({
  name: '',
  type: '',
  tags: [] as string[],
  notes: ''
})
const newTag = ref('')

// Load documents
async function loadDocuments() {
  try {
    loading.value = true
    const docs = await invoke<Document[]>('get_documents')
    documents.value = docs
  } catch (error) {
    console.error('Failed to load documents:', error)
    alert(`Erreur lors du chargement des documents: ${error}`)
  } finally {
    loading.value = false
  }
}

// Stats
const stats = computed<DocumentStats>(() => {
  const types = new Set(documents.value.map(d => d.document_type))
  const tags = new Set(documents.value.flatMap(d => d.tags))
  const size = documents.value.reduce((sum, d) => sum + d.file_size, 0)

  return {
    total: documents.value.length,
    types: types.size,
    tags: tags.size,
    size
  }
})

// File selection
async function selectFile() {
  try {
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
        ocrType: selectedOcrType.value
      })
      await loadDocuments()
    }
  } catch (error) {
    console.error('Failed to import document:', error)
  }
}

// Drag & Drop handlers
function handleDragEnter(_e: DragEvent) {
  isDragging.value = true
}

function handleDragOver(_e: DragEvent) {
  isDragging.value = true
}

function handleDragLeave(_e: DragEvent) {
  isDragging.value = false
}

async function handleDrop(e: DragEvent) {
  isDragging.value = false
  
  const files = e.dataTransfer?.files
  if (!files || files.length === 0) return
  
  const file = files[0]
  if (!file) return
  
  const validExtensions = ['pdf', 'png', 'jpg', 'jpeg']
  const fileExt = file.name.split('.').pop()?.toLowerCase()
  
  if (!fileExt || !validExtensions.includes(fileExt)) {
    alert('Format non supporté. Utilisez PDF, PNG ou JPG.')
    return
  }
  
  try {
    // Note: Le drag & drop web ne donne pas le chemin système
    // On utilise seulement la sélection de fichier via dialog
    alert('Veuillez utiliser le bouton "Importer" pour l\'instant. Le drag & drop sera amélioré prochainement.')
  } catch (error) {
    console.error('Failed to process dropped file:', error)
    alert(`Erreur lors du traitement du fichier: ${error}`)
  }
}

// Scan folder
async function scanFolder() {
  try {
    const selected = await open({
      directory: true
    })

    if (selected) {
      // 1. Scanner le dossier pour obtenir la liste des fichiers
      const files = await invoke<string[]>('scan_folder', { folderPath: selected })
      
      if (files.length === 0) {
        alert('Aucun fichier supporté trouvé dans ce dossier')
        return
      }

      // 2. Confirmer le traitement
      if (!confirm(`${files.length} fichier(s) trouvé(s). Voulez-vous les traiter tous ?`)) {
        return
      }

      // 3. Importer avec l'indicateur de progression
      const { startProcessing, updateProgress, completeProcessing } = useProcessingState()
      const folderId = `folder_${Date.now()}`
      startProcessing(folderId, `Dossier (${files.length} fichiers)`, 'folder')

      let processed = 0
      let errors = 0

      for (const filePath of files) {
        try {
          const fileName = filePath.split('/').pop() || filePath
          console.log(`Traitement: ${fileName}`)
          
          await invoke('process_file', { filePath })
          processed++
          
          // Mettre à jour la progression
          const progress = (processed / files.length) * 100
          updateProgress(folderId, progress)
          
        } catch (error) {
          console.error(`Erreur traitement ${filePath}:`, error)
          errors++
        }
      }

      // 4. Finaliser
      completeProcessing(folderId)
      
      // 5. Recharger les documents
      await loadDocuments()

      // 6. Afficher le résumé
      if (errors > 0) {
        alert(`Traitement terminé: ${processed} succès, ${errors} erreur(s)`)
      } else {
        alert(`${processed} fichier(s) traité(s) avec succès !`)
      }
    }
  } catch (error) {
    console.error('Failed to scan folder:', error)
    alert(`Erreur lors du scan du dossier: ${error}`)
  }
}

// Open document
async function openDocument(doc: Document) {
  try {
    await invoke('open_file', { filePath: doc.file_path })
  } catch (error) {
    console.error('Failed to open document:', error)
    alert(`Erreur lors de l'ouverture du document: ${error}`)
  }
}

// Edit document
function editDocument(doc: Document) {
  editingDocument.value = doc
  editForm.value = {
    name: doc.new_name,
    type: doc.document_type,
    tags: [...doc.tags],
    notes: doc.notes || ''
  }
  selectedDocument.value = null
}

// Add tag
function addTag() {
  const tag = newTag.value.trim()
  if (tag && !editForm.value.tags.includes(tag)) {
    editForm.value.tags.push(tag)
    newTag.value = ''
  }
}

// Remove tag
function removeTag(index: number) {
  editForm.value.tags.splice(index, 1)
}

// Save metadata
async function saveMetadata() {
  if (!editingDocument.value) return

  try {
    // Update metadata
    await invoke('update_metadata', {
      id: editingDocument.value.id,
      documentType: editForm.value.type,
      tags: editForm.value.tags,
      newName: editForm.value.name
    })

    // Update notes
    await invoke('update_notes', {
      id: editingDocument.value.id,
      notes: editForm.value.notes || null
    })

    // Reload documents
    await loadDocuments()
    editingDocument.value = null
  } catch (error) {
    console.error('Failed to save metadata:', error)
    alert(`Erreur lors de la sauvegarde: ${error}`)
  }
}

// Cancel edit
function cancelEdit() {
  editingDocument.value = null
  newTag.value = ''
}

// Format date
function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}

// Format file size
function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' o'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' Ko'
  return (bytes / (1024 * 1024)).toFixed(1) + ' Mo'
}

// Get category icon (Material Icons name)
function getCategoryIcon(category: string): string {
  const icons: Record<string, string> = {
    'Administratif': 'description',
    'Financier': 'account_balance',
    'Santé': 'medical_services',
    'Professionnel': 'work',
    'Immobilier': 'home',
    'Académique': 'school',
    'Personnel': 'person',
    'Autre': 'folder'
  }
  return icons[category] || 'folder'
}

// Get confidence CSS class based on score
function getConfidenceClass(confidence: number): string {
  if (confidence >= 0.7) return 'confidence-high'
  if (confidence >= 0.4) return 'confidence-medium'
  return 'confidence-low'
}

onMounted(() => {
  loadDocuments()

  // Load default OCR type from Settings
  const savedOcrType = localStorage.getItem('defaultOcrType')
  if (savedOcrType) {
    selectedOcrType.value = savedOcrType
  }
})
</script>

<style scoped>
.home {
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
  max-width: 1400px;
  margin: 0 auto;
}

.app-bar-title {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-sm);
}

.app-icon {
  font-size: 32px;
  color: var(--md-sys-color-primary);
}

.app-bar-title h1 {
  color: var(--md-sys-color-primary);
  margin: 0;
}

.app-subtitle {
  color: var(--md-sys-color-on-surface-variant);
  margin-left: var(--md-sys-spacing-sm);
}

/* Main Content */
.home-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: var(--md-sys-spacing-xl) var(--md-sys-spacing-lg);
}

/* Search Container */
.search-container {
  margin-bottom: var(--md-sys-spacing-lg);
}

.md-search-field {
  position: relative;
}

.search-input {
  width: 100%;
  height: var(--md-comp-search-bar-container-height);
  padding: 0 var(--md-sys-spacing-2xl) 0 var(--md-sys-spacing-2xl);
  background-color: var(--md-sys-color-surface-container-high);
  border: none;
  border-radius: var(--md-comp-search-bar-container-shape);
  font-size: var(--md-sys-typescale-body-large-font-size);
  color: var(--md-sys-color-on-surface);
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.search-input:focus {
  outline: none;
  box-shadow: var(--md-sys-elevation-level2);
  background-color: var(--md-sys-color-surface-container-highest);
}

.search-icon {
  position: absolute;
  left: var(--md-sys-spacing-md);
  top: 50%;
  transform: translateY(-50%);
  color: var(--md-sys-color-on-surface-variant);
  pointer-events: none;
}

/* Stats Grid */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--md-sys-spacing-md);
  margin-bottom: var(--md-sys-spacing-xl);
}

.stat-card {
  padding: var(--md-sys-spacing-lg);
  text-align: center;
  transition: all var(--md-sys-motion-duration-medium2) var(--md-sys-motion-easing-emphasized);
}

.stat-icon {
  font-size: 32px;
  margin-bottom: var(--md-sys-spacing-sm);
}

.stat-value {
  color: var(--md-sys-color-primary);
  margin-bottom: var(--md-sys-spacing-xs);
}

.stat-label {
  color: var(--md-sys-color-on-surface-variant);
}

/* OCR Type Inline */
.ocr-type-inline {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-sm);
  padding: var(--md-sys-spacing-md);
  background-color: var(--md-sys-color-surface);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  margin-top: var(--md-sys-spacing-md);
}

.ocr-type-inline label {
  color: var(--md-sys-color-on-surface-variant);
  font-weight: 500;
}

.md-select-inline {
  padding: var(--md-sys-spacing-sm) var(--md-sys-spacing-md);
  border: none;
  border-radius: var(--md-sys-shape-corner-full);
  background-color: var(--md-sys-color-surface-container-highest);
  color: var(--md-sys-color-on-surface);
  font-family: var(--md-sys-typescale-body-medium-font-family);
  font-size: var(--md-sys-typescale-body-medium-font-size);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
  box-shadow: var(--md-sys-elevation-level1);
}

.md-select-inline:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 8%, var(--md-sys-color-surface-container-highest));
  box-shadow: var(--md-sys-elevation-level2);
}

.md-select-inline:focus {
  outline: none;
  background-color: var(--md-sys-color-surface-container-highest);
  box-shadow: var(--md-sys-elevation-level2);
}

/* Dashboard Section */
.dashboard-section {
  margin-bottom: var(--md-sys-spacing-2xl);
}

.section-title {
  margin-bottom: var(--md-sys-spacing-lg);
  color: var(--md-sys-color-on-surface);
}

/* Import Section with Drag & Drop */
.import-section {
  margin-bottom: var(--md-sys-spacing-2xl);
}

.drag-drop-zone {
  border: 3px dashed var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-large);
  padding: var(--md-sys-spacing-2xl);
  text-align: center;
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-medium2) var(--md-sys-motion-easing-emphasized);
  background-color: var(--md-sys-color-surface-variant);
  margin-bottom: var(--md-sys-spacing-lg);
}

.drag-drop-zone:hover {
  border-color: var(--md-sys-color-primary);
  background-color: color-mix(in srgb, var(--md-sys-color-primary) 8%, var(--md-sys-color-surface-variant));
}

.drag-drop-zone.drag-over {
  border-color: var(--md-sys-color-primary);
  background-color: color-mix(in srgb, var(--md-sys-color-primary) 15%, var(--md-sys-color-surface-variant));
  transform: scale(1.02);
}

.drag-drop-icon {
  margin: 0 auto var(--md-sys-spacing-md);
  color: var(--md-sys-color-on-surface-variant);
}

.drag-drop-zone h3 {
  color: var(--md-sys-color-on-surface);
  margin-bottom: var(--md-sys-spacing-xs);
}

.drag-drop-zone p {
  color: var(--md-sys-color-on-surface-variant);
  margin: var(--md-sys-spacing-xs) 0;
}

.drag-drop-hint {
  font-style: italic;
  opacity: 0.7;
}

.action-buttons {
  display: flex;
  gap: var(--md-sys-spacing-md);
  flex-wrap: wrap;
  align-items: center;
}

.ocr-type-selector {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-sm);
  margin-left: auto;
}

.ocr-type-selector label {
  color: var(--md-sys-color-on-surface);
  font-weight: 500;
  white-space: nowrap;
}

.ocr-select {
  padding: var(--md-sys-spacing-md) var(--md-sys-spacing-lg);
  border: none;
  border-radius: var(--md-sys-shape-corner-full);
  background-color: var(--md-sys-color-surface-container-highest);
  color: var(--md-sys-color-on-surface);
  font-family: inherit;
  font-size: var(--md-sys-typescale-body-large-font-size);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
  box-shadow: var(--md-sys-elevation-level1);
  min-width: 160px;
}

.ocr-select:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 8%, var(--md-sys-color-surface-container-highest));
  box-shadow: var(--md-sys-elevation-level2);
}

.ocr-select:focus {
  outline: none;
  box-shadow: var(--md-sys-elevation-level2);
}

/* Action Section (fallback for old class) */
.action-section {
  display: flex;
  gap: var(--md-sys-spacing-md);
  margin-bottom: var(--md-sys-spacing-2xl);
  flex-wrap: wrap;
}

/* Documents Section */
.documents-section {
  margin-top: var(--md-sys-spacing-2xl);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--md-sys-spacing-md);
  margin-bottom: var(--md-sys-spacing-lg);
  flex-wrap: wrap;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-md);
}

.pagination-controls {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-sm);
}

.items-per-page-select {
  padding: var(--md-sys-spacing-sm) var(--md-sys-spacing-md);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  background-color: var(--md-sys-color-surface);
  color: var(--md-sys-color-on-surface);
  font-family: var(--md-sys-typescale-body-medium-font);
  font-size: var(--md-sys-typescale-body-medium-size);
  cursor: pointer;
}

.items-per-page-select:hover {
  background-color: var(--md-sys-color-surface-variant);
}

.count-badge {
  background-color: var(--md-sys-color-primary-container);
  color: var(--md-sys-color-on-primary-container);
  padding: var(--md-sys-spacing-xs) var(--md-sys-spacing-md);
  border-radius: var(--md-sys-shape-corner-full);
}

/* Loading State */
.loading-state {
  margin-top: var(--md-sys-spacing-xl);
}

.loading-skeleton {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--md-sys-spacing-md);
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: var(--md-sys-spacing-3xl) var(--md-sys-spacing-lg);
  color: var(--md-sys-color-on-surface-variant);
}

.empty-state svg {
  margin-bottom: var(--md-sys-spacing-lg);
}

.empty-state h3 {
  margin-bottom: var(--md-sys-spacing-sm);
  color: var(--md-sys-color-on-surface);
}

/* Documents Grid */
.documents-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--md-sys-spacing-md);
  margin-bottom: var(--md-sys-spacing-lg);
}

/* Pagination Navigation */
.pagination-nav {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--md-sys-spacing-lg);
  padding: var(--md-sys-spacing-lg) 0;
  border-top: 1px solid var(--md-sys-color-outline-variant);
}

.pagination-info {
  color: var(--md-sys-color-on-surface-variant);
  min-width: 120px;
  text-align: center;
}

.pagination-nav button:disabled {
  opacity: 0.38;
  cursor: not-allowed;
}

.document-card {
  padding: var(--md-sys-spacing-lg);
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-md);
  position: relative;
  overflow: visible;
}

/* Category Badge */
.category-badge {
  position: absolute;
  top: -8px;
  left: 12px;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 12px;
  background-color: #000000;
  color: white;
  border-radius: 12px;
  font-weight: 500;
  font-size: 11px;
  z-index: 10;
  box-shadow: var(--md-sys-elevation-level2);
}

.category-icon {
  font-size: 14px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding-top: 8px;
}

.doc-type-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.doc-type-chip {
  font-size: var(--md-sys-typescale-label-small-font-size);
  width: fit-content;
}

.subcategory-text {
  color: var(--md-sys-color-on-surface-variant);
  font-style: italic;
}

.doc-name {
  color: var(--md-sys-color-on-surface);
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
}

.doc-meta {
  display: flex;
  gap: var(--md-sys-spacing-md);
  color: var(--md-sys-color-on-surface-variant);
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-xs);
}

.doc-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--md-sys-spacing-xs);
}

.tag-chip {
  font-size: var(--md-sys-typescale-label-small-font-size);
}

.doc-action-btn {
  width: 100%;
  margin-top: auto;
}

/* Modal Specifics */
.md-modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.md-modal-header h2 {
  margin: 0;
  flex: 1;
}

.modal-info-grid {
  display: grid;
  gap: var(--md-sys-spacing-lg);
  margin-bottom: var(--md-sys-spacing-lg);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-xs);
}

.info-label {
  color: var(--md-sys-color-on-surface-variant);
}

/* Confidence Badge */
.confidence-badge {
  display: inline-block;
  margin-left: var(--md-sys-spacing-xs);
  padding: 2px 8px;
  border-radius: var(--md-sys-shape-corner-full);
  font-size: 12px;
  font-weight: 600;
}

.confidence-high {
  background-color: rgba(76, 175, 80, 0.15);
  color: #4CAF50;
}

.confidence-medium {
  background-color: rgba(255, 152, 0, 0.15);
  color: #FF9800;
}

.confidence-low {
  background-color: rgba(244, 67, 54, 0.15);
  color: #F44336;
}

.modal-tags {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-sm);
  margin-bottom: var(--md-sys-spacing-lg);
}

.tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--md-sys-spacing-xs);
}

.modal-ocr {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-sm);
}

.ocr-text {
  color: var(--md-sys-color-on-surface-variant);
  background-color: var(--md-sys-color-surface-container);
  padding: var(--md-sys-spacing-md);
  border-radius: var(--md-sys-shape-corner-small);
  max-height: 200px;
  overflow-y: auto;
}

/* Responsive */
@media (max-width: 768px) {
  .home-content {
    padding: var(--md-sys-spacing-lg) var(--md-sys-spacing-md);
  }

  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .action-section {
    flex-direction: column;
  }

  .action-section button {
    width: 100%;
  }

  .documents-grid {
    grid-template-columns: 1fr;
  }

  .md-modal {
    max-width: 95vw;
  }
}

/* Edit Modal Specific Styles */
.edit-modal {
  max-width: 600px;
  width: 90vw;
}

.form-field {
  margin-bottom: var(--md-sys-spacing-lg);
}

.form-field label {
  display: block;
  margin-bottom: var(--md-sys-spacing-sm);
  color: var(--md-sys-color-on-surface);
}

.md-text-field input {
  width: 100%;
  height: 56px;
  padding: 0 var(--md-sys-spacing-md);
  background-color: var(--md-sys-color-surface-container-highest);
  border: none;
  border-bottom: 1px solid var(--md-sys-color-on-surface-variant);
  border-radius: var(--md-sys-shape-corner-extra-small) var(--md-sys-shape-corner-extra-small) 0 0;
  font-size: var(--md-sys-typescale-body-large-font-size);
  color: var(--md-sys-color-on-surface);
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.md-text-field input:focus {
  outline: none;
  border-bottom-color: var(--md-sys-color-primary);
  border-bottom-width: 2px;
}

.tags-editor {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-md);
}

.current-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--md-sys-spacing-xs);
  min-height: 40px;
  padding: var(--md-sys-spacing-sm);
  background-color: var(--md-sys-color-surface-container);
  border-radius: var(--md-sys-shape-corner-small);
}

.tag-editable {
  display: inline-flex;
  align-items: center;
  gap: var(--md-sys-spacing-xs);
  padding-right: var(--md-sys-spacing-xs);
}

.tag-remove {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  padding: 0;
  background: transparent;
  border: none;
  border-radius: var(--md-sys-shape-corner-full);
  cursor: pointer;
  color: currentColor;
  transition: background-color var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.tag-remove:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 12%, transparent);
}

.add-tag-input {
  display: flex;
  gap: var(--md-sys-spacing-sm);
  align-items: center;
}

.tag-input {
  flex: 1;
  height: 40px;
  padding: 0 var(--md-sys-spacing-md);
  background-color: var(--md-sys-color-surface-container-highest);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  font-size: var(--md-sys-typescale-body-medium-font-size);
  color: var(--md-sys-color-on-surface);
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.tag-input:focus {
  outline: none;
  border-color: var(--md-sys-color-primary);
  border-width: 2px;
}

.notes-textarea {
  width: 100%;
  padding: var(--md-sys-spacing-md);
  background-color: var(--md-sys-color-surface-container-highest);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  font-size: var(--md-sys-typescale-body-medium-font-size);
  font-family: inherit;
  color: var(--md-sys-color-on-surface);
  resize: vertical;
  min-height: 100px;
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.notes-textarea:focus {
  outline: none;
  border-color: var(--md-sys-color-primary);
  border-width: 2px;
}
</style>
