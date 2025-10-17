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
      <!-- Search Bar -->
      <div class="search-container animate-fade-in">
        <div class="md-search-field">
          <svg class="search-icon" width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
            <circle cx="8" cy="8" r="6" stroke-width="2"/>
            <path d="M13 13l4 4" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Rechercher dans les documents..."
            class="search-input"
          />
        </div>
      </div>

      <!-- Filters -->
      <div class="filters-container animate-slide-in-up" style="animation-delay: 50ms">
        <div class="filter-chips">
          <button
            @click="filterType = ''"
            class="md-chip"
            :class="{ 'md-chip-selected': filterType === '' }"
          >
            📂 Tous
          </button>
          <button
            v-for="type in documentTypes"
            :key="type"
            @click="filterType = type"
            class="md-chip"
            :class="{ 'md-chip-selected': filterType === type }"
          >
            {{ type }}
          </button>
        </div>
        <button @click="resetFilters" class="md-icon-button" title="Réinitialiser les filtres">
          <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
            <path d="M1 4v6h6M19 16v-6h-6M4 15.5A8 8 0 1 1 4 4.5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </div>

      <!-- Stats Cards -->
      <div class="stats-grid animate-slide-in-up" style="animation-delay: 100ms">
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

      <!-- OCR Type Selection -->
      <div class="ocr-selection animate-slide-in-up" style="animation-delay: 125ms">
        <label for="ocr-type" class="body-medium ocr-label">Type d'OCR</label>
        <select id="ocr-type" v-model="selectedOcrType" class="md-select">
          <option value="standard">🔤 Standard (Tesseract)</option>
          <option value="handwritten">✍️ Écriture manuscrite (TrOCR)</option>
          <option value="printed">📄 Imprimé (TrOCR)</option>
          <option value="caption">🖼️ Légende d'image (BLIP)</option>
        </select>
      </div>

      <!-- Import Actions & Drag & Drop Zone -->
      <div class="import-section animate-slide-in-up" style="animation-delay: 150ms">
        <!-- Drag & Drop Zone -->
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

        <!-- Action Buttons -->
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
        </div>
      </div>

      <!-- Documents Section -->
      <div class="documents-section">
        <div class="section-header">
          <h2 class="title-large">Documents</h2>
          <div class="header-actions">
            <span class="body-large count-badge">{{ filteredDocuments.length }}</span>
            
            <!-- Pagination Controls -->
            <div class="pagination-controls">
              <select v-model="itemsPerPage" class="items-per-page-select">
                <option :value="10">10 par page</option>
                <option :value="25">25 par page</option>
                <option :value="50">50 par page</option>
                <option :value="100">100 par page</option>
                <option :value="filteredDocuments.length">Tous ({{ filteredDocuments.length }})</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Loading State -->
        <div v-if="loading" class="loading-state">
          <div class="loading-skeleton">
            <div class="md-skeleton md-skeleton-card" v-for="i in 6" :key="i"></div>
          </div>
        </div>

        <!-- Empty State -->
        <div v-else-if="filteredDocuments.length === 0" class="empty-state">
          <svg width="120" height="120" viewBox="0 0 120 120" fill="none">
            <circle cx="60" cy="60" r="50" :fill="`color-mix(in srgb, var(--md-sys-color-primary) 12%, transparent)`"/>
            <path d="M40 50h40M40 60h40M40 70h25" stroke="var(--md-sys-color-on-surface-variant)" stroke-width="3" stroke-linecap="round"/>
          </svg>
          <h3 class="title-medium">Aucun document</h3>
          <p class="body-medium">Commencez par importer vos premiers documents</p>
        </div>

        <!-- Documents Grid -->
        <div v-else class="documents-grid">
          <div
            v-for="(doc, index) in paginatedDocuments"
            :key="doc.id"
            class="md-card document-card animate-scale-in"
            :style="{ animationDelay: `${200 + index * 50}ms` }"
            @click="selectedDocument = doc"
          >
            <!-- Category Badge (top-left corner) -->
            <div class="category-badge" :style="{ backgroundColor: getCategoryColor(doc.category) }">
              <span class="material-icons category-icon">{{ getCategoryIcon(doc.category) }}</span>
              <span class="body-small">{{ doc.category }}</span>
            </div>

            <div class="card-header">
              <div class="doc-type-info">
                <span class="doc-type-chip md-chip">{{ doc.document_type }}</span>
                <span v-if="doc.subcategory" class="subcategory-text body-small">{{ doc.subcategory }}</span>
              </div>
              <button @click.stop="deleteDocument(doc.id)" class="md-icon-button">
                <svg width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor">
                  <polyline points="3 4 5 4 17 4" stroke-width="2" stroke-linecap="round"/>
                  <path d="M15 4v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4m2 0V2a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2" stroke-width="2" stroke-linecap="round"/>
                </svg>
              </button>
            </div>

            <h3 class="title-medium doc-name">{{ doc.new_name }}</h3>

            <div class="doc-meta body-small">
              <span class="meta-item">
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor">
                  <rect x="2" y="3" width="10" height="9" rx="1" stroke-width="1.5"/>
                  <path d="M9 1v4M5 1v4M2 7h10" stroke-width="1.5" stroke-linecap="round"/>
                </svg>
                {{ formatDate(doc.created_at) }}
              </span>
              <span class="meta-item">
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor">
                  <path d="M5 1H3a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V9" stroke-width="1.5" stroke-linecap="round"/>
                  <path d="M12 1h-7v7" stroke-width="1.5" stroke-linecap="round"/>
                </svg>
                {{ formatSize(doc.file_size) }}
              </span>
            </div>

            <div v-if="doc.tags.length > 0" class="doc-tags">
              <span v-for="tag in doc.tags.slice(0, 3)" :key="tag" class="tag-chip md-chip">
                {{ tag }}
              </span>
              <span v-if="doc.tags.length > 3" class="tag-chip md-chip">
                +{{ doc.tags.length - 3 }}
              </span>
            </div>

            <button @click.stop="openDocument(doc)" class="md-filled-button doc-action-btn md-ripple">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
                <path d="M14 9v4a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h4M10 2h4v4M7 9l7-7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              Ouvrir
            </button>
          </div>
        </div>

        <!-- Pagination Navigation -->
        <div v-if="totalPages > 1" class="pagination-nav">
          <button 
            @click="currentPage = Math.max(1, currentPage - 1)"
            :disabled="currentPage === 1"
            class="md-icon-button"
          >
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
              <path d="M12 4l-6 6 6 6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
          
          <span class="body-medium pagination-info">
            Page {{ currentPage }} / {{ totalPages }}
          </span>
          
          <button 
            @click="currentPage = Math.min(totalPages, currentPage + 1)"
            :disabled="currentPage === totalPages"
            class="md-icon-button"
          >
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
              <path d="M8 4l6 6-6 6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
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
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import type { Document, DocumentStats } from '../types/document'

const documents = ref<Document[]>([])
const loading = ref(true)
const searchQuery = ref('')
const filterType = ref('')
const selectedDocument = ref<Document | null>(null)
const editingDocument = ref<Document | null>(null)
const selectedOcrType = ref('standard') // Default OCR type

// Pagination state
const itemsPerPage = ref(25)
const currentPage = ref(1)
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

// Filtered documents
const filteredDocuments = computed(() => {
  let result = documents.value

  if (filterType.value) {
    result = result.filter(doc => doc.document_type === filterType.value)
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(doc =>
      doc.new_name.toLowerCase().includes(query) ||
      doc.original_name.toLowerCase().includes(query) ||
      doc.ocr_text.toLowerCase().includes(query) ||
      doc.tags.some(tag => tag.toLowerCase().includes(query))
    )
  }

  return result
})

// Paginated documents
const paginatedDocuments = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage.value
  const end = start + itemsPerPage.value
  
  // Si "Tous" est sélectionné (itemsPerPage === total), retourner tout
  if (itemsPerPage.value >= filteredDocuments.value.length) {
    return filteredDocuments.value
  }
  
  return filteredDocuments.value.slice(start, end)
})

// Total pages
const totalPages = computed(() => {
  if (itemsPerPage.value >= filteredDocuments.value.length) return 1
  return Math.ceil(filteredDocuments.value.length / itemsPerPage.value)
})

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

// Get unique types
const documentTypes = computed(() => {
  return Array.from(new Set(documents.value.map(d => d.document_type)))
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
      await invoke('scan_folder', { folderPath: selected })
      await loadDocuments()
    }
  } catch (error) {
    console.error('Failed to scan folder:', error)
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

// Delete document
async function deleteDocument(id: string) {
  if (confirm('Déplacer ce document vers la corbeille ?')) {
    try {
      await invoke('delete_document', { id })
      await loadDocuments()
      selectedDocument.value = null
    } catch (error) {
      console.error('Failed to delete document:', error)
    }
  }
}

// Reset filters
function resetFilters() {
  searchQuery.value = ''
  filterType.value = ''
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

// Get category color
function getCategoryColor(category: string): string {
  const colors: Record<string, string> = {
    'Administratif': '#757575',  // Gray
    'Financier': '#000000',      // Black
    'Santé': '#D32F2F',          // Red
    'Professionnel': '#424242',  // Dark gray
    'Immobilier': '#616161',     // Medium gray
    'Académique': '#212121',     // Almost black
    'Personnel': '#9E9E9E',      // Light gray
    'Autre': '#BDBDBD'           // Very light gray
  }
  return colors[category] || '#BDBDBD'
}

// Reset page when filters or items per page change
watch([searchQuery, filterType, itemsPerPage], () => {
  currentPage.value = 1
})

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

/* Filters */
.filters-container {
  display: flex;
  gap: var(--md-sys-spacing-md);
  align-items: center;
  margin-bottom: var(--md-sys-spacing-xl);
  flex-wrap: wrap;
}

.filter-chips {
  display: flex;
  gap: var(--md-sys-spacing-sm);
  flex-wrap: wrap;
  flex: 1;
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
  background-color: var(--md-sys-color-surface-variant);
  border-radius: var(--md-sys-shape-corner-small);
  justify-content: center;
}

.md-select-compact {
  padding: var(--md-sys-spacing-xs) var(--md-sys-spacing-md);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  background-color: var(--md-sys-color-surface);
  color: var(--md-sys-color-on-surface);
  font-family: var(--md-sys-typescale-body-small-font);
  font-size: var(--md-sys-typescale-body-small-size);
  cursor: pointer;
}

/* Dashboard Section */
.dashboard-section {
  margin-bottom: var(--md-sys-spacing-2xl);
}

.dashboard-title {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-sm);
  margin-bottom: var(--md-sys-spacing-lg);
  color: var(--md-sys-color-on-surface);
}

.dashboard-title .material-icons {
  font-size: 28px;
}

/* OCR Selection (old) */
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
