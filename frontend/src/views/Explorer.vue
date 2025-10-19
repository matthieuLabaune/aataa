<template>
  <div class="explorer">
    <!-- Top App Bar -->
    <header class="md-top-app-bar">
      <div class="app-bar-content">
        <div class="app-bar-title">
          <span class="material-icons app-icon">folder_open</span>
          <h1 class="headline-medium">Explorateur</h1>
          <span class="body-small app-subtitle">Recherche et filtres avancés</span>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <div class="explorer-content">
      <!-- Search & Filters Bar -->
      <div class="search-filters-bar animate-fade-in">
        <!-- Search -->
        <div class="md-search-field">
          <svg class="search-icon" width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
            <circle cx="8" cy="8" r="6" stroke-width="2"/>
            <path d="M13 13l4 4" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Rechercher par nom, contenu, tags..."
            class="search-input"
          />
        </div>

        <!-- Filter Button -->
        <button @click="showFilters = !showFilters" class="md-outlined-button">
          <span class="material-icons">filter_list</span>
          Filtres
          <span v-if="activeFiltersCount > 0" class="filter-badge">{{ activeFiltersCount }}</span>
        </button>

        <!-- Sort Dropdown -->
        <select v-model="sortBy" class="md-select-compact">
          <option value="date-desc">Plus récent</option>
          <option value="date-asc">Plus ancien</option>
          <option value="name-asc">Nom A-Z</option>
          <option value="name-desc">Nom Z-A</option>
          <option value="size-desc">Taille décroissante</option>
          <option value="size-asc">Taille croissante</option>
        </select>
      </div>

      <!-- Advanced Filters Panel -->
      <div v-if="showFilters" class="filters-panel animate-slide-in-down">
        <div class="filters-grid">
          <!-- Category Filter -->
          <div class="filter-group">
            <label class="label-medium">Catégorie</label>
            <select v-model="filterCategory" class="md-select">
              <option value="">Toutes les catégories</option>
              <option value="Administratif">Administratif</option>
              <option value="Financier">Financier</option>
              <option value="Santé">Santé</option>
              <option value="Professionnel">Professionnel</option>
              <option value="Immobilier">Immobilier</option>
              <option value="Académique">Académique</option>
              <option value="Personnel">Personnel</option>
              <option value="Autre">Autre</option>
            </select>
          </div>

          <!-- Subcategory Filter -->
          <div class="filter-group">
            <label class="label-medium">Sous-catégorie</label>
            <select v-model="filterSubcategory" class="md-select" :disabled="!filterCategory">
              <option value="">Toutes</option>
              <option v-for="sub in availableSubcategories" :key="sub" :value="sub">
                {{ sub }}
              </option>
            </select>
          </div>

          <!-- Document Type Filter -->
          <div class="filter-group">
            <label class="label-medium">Type de document</label>
            <select v-model="filterType" class="md-select">
              <option value="">Tous les types</option>
              <option v-for="type in documentTypes" :key="type" :value="type">
                {{ type }}
              </option>
            </select>
          </div>

          <!-- Date Range Filter -->
          <div class="filter-group">
            <label class="label-medium">Période</label>
            <select v-model="filterDateRange" class="md-select">
              <option value="">Toutes les dates</option>
              <option value="today">Aujourd'hui</option>
              <option value="week">Cette semaine</option>
              <option value="month">Ce mois</option>
              <option value="year">Cette année</option>
            </select>
          </div>

          <!-- Size Range Filter -->
          <div class="filter-group">
            <label class="label-medium">Taille</label>
            <select v-model="filterSizeRange" class="md-select">
              <option value="">Toutes tailles</option>
              <option value="small">< 1 Mo</option>
              <option value="medium">1-10 Mo</option>
              <option value="large">> 10 Mo</option>
            </select>
          </div>

          <!-- Tags Filter -->
          <div class="filter-group filter-group-full">
            <label class="label-medium">Tags</label>
            <div class="tags-filter">
              <button
                v-for="tag in topTags"
                :key="tag"
                @click="toggleTag(tag)"
                class="md-chip"
                :class="{ 'md-chip-selected': selectedTags.includes(tag) }"
              >
                {{ tag }}
              </button>
            </div>
          </div>
        </div>

        <div class="filters-actions">
          <button @click="resetFilters" class="md-text-button">
            Réinitialiser
          </button>
          <button @click="showFilters = false" class="md-filled-button">
            Appliquer
          </button>
        </div>
      </div>

      <!-- Results Info -->
      <div class="results-info">
        <span class="body-medium">
          {{ filteredDocuments.length }} document{{ filteredDocuments.length > 1 ? 's' : '' }} trouvé{{ filteredDocuments.length > 1 ? 's' : '' }}
        </span>

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
        <h3 class="title-medium">Aucun document trouvé</h3>
        <p class="body-medium">Essayez de modifier vos critères de recherche</p>
      </div>

      <!-- Documents Grid -->
      <div v-else class="documents-grid">
        <div
          v-for="(doc, index) in paginatedDocuments"
          :key="doc.id"
          class="md-card document-card animate-scale-in"
          :style="{ animationDelay: `${index * 30}ms` }"
          @click="selectedDocument = doc"
        >
          <!-- Category Badge -->
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

    <!-- Document Detail Modal (reuse from Home) -->
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
          <button @click="openDocument(selectedDocument)" class="md-filled-button md-ripple">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
              <path d="M14 9v4a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h4M10 2h4v4M7 9l7-7" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            Ouvrir le fichier
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Document } from '../types/document'

// State
const documents = ref<Document[]>([])
const loading = ref(true)
const searchQuery = ref('')
const showFilters = ref(false)
const selectedDocument = ref<Document | null>(null)

// Filter states
const filterCategory = ref('')
const filterSubcategory = ref('')
const filterType = ref('')
const filterDateRange = ref('')
const filterSizeRange = ref('')
const selectedTags = ref<string[]>([])
const sortBy = ref('date-desc')

// Pagination
const itemsPerPage = ref(25)
const currentPage = ref(1)

// Load documents
async function loadDocuments() {
  try {
    loading.value = true
    const docs = await invoke<Document[]>('get_documents')
    documents.value = docs
  } catch (error) {
    console.error('Failed to load documents:', error)
  } finally {
    loading.value = false
  }
}

// Filtered documents
const filteredDocuments = computed(() => {
  let result = documents.value

  // Search query
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(doc =>
      doc.new_name.toLowerCase().includes(query) ||
      doc.original_name.toLowerCase().includes(query) ||
      doc.ocr_text.toLowerCase().includes(query) ||
      doc.tags.some(tag => tag.toLowerCase().includes(query))
    )
  }

  // Category filter
  if (filterCategory.value) {
    result = result.filter(doc => doc.category === filterCategory.value)
  }

  // Subcategory filter
  if (filterSubcategory.value) {
    result = result.filter(doc => doc.subcategory === filterSubcategory.value)
  }

  // Type filter
  if (filterType.value) {
    result = result.filter(doc => doc.document_type === filterType.value)
  }

  // Date range filter
  if (filterDateRange.value) {
    const now = new Date()
    const filterDate = (dateStr: string) => {
      const date = new Date(dateStr)
      switch (filterDateRange.value) {
        case 'today':
          return date.toDateString() === now.toDateString()
        case 'week':
          const weekAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000)
          return date >= weekAgo
        case 'month':
          const monthAgo = new Date(now.getFullYear(), now.getMonth() - 1, now.getDate())
          return date >= monthAgo
        case 'year':
          const yearAgo = new Date(now.getFullYear() - 1, now.getMonth(), now.getDate())
          return date >= yearAgo
        default:
          return true
      }
    }
    result = result.filter(doc => filterDate(doc.created_at))
  }

  // Size range filter
  if (filterSizeRange.value) {
    const MB = 1024 * 1024
    result = result.filter(doc => {
      switch (filterSizeRange.value) {
        case 'small':
          return doc.file_size < MB
        case 'medium':
          return doc.file_size >= MB && doc.file_size <= 10 * MB
        case 'large':
          return doc.file_size > 10 * MB
        default:
          return true
      }
    })
  }

  // Tags filter
  if (selectedTags.value.length > 0) {
    result = result.filter(doc =>
      selectedTags.value.some(tag => doc.tags.includes(tag))
    )
  }

  // Sorting
  result = [...result].sort((a, b) => {
    switch (sortBy.value) {
      case 'date-desc':
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
      case 'date-asc':
        return new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
      case 'name-asc':
        return a.new_name.localeCompare(b.new_name)
      case 'name-desc':
        return b.new_name.localeCompare(a.new_name)
      case 'size-desc':
        return b.file_size - a.file_size
      case 'size-asc':
        return a.file_size - b.file_size
      default:
        return 0
    }
  })

  return result
})

// Paginated documents
const paginatedDocuments = computed(() => {
  if (itemsPerPage.value >= filteredDocuments.value.length) {
    return filteredDocuments.value
  }
  const start = (currentPage.value - 1) * itemsPerPage.value
  const end = start + itemsPerPage.value
  return filteredDocuments.value.slice(start, end)
})

// Total pages
const totalPages = computed(() => {
  if (itemsPerPage.value >= filteredDocuments.value.length) return 1
  return Math.ceil(filteredDocuments.value.length / itemsPerPage.value)
})

// Active filters count
const activeFiltersCount = computed(() => {
  let count = 0
  if (filterCategory.value) count++
  if (filterSubcategory.value) count++
  if (filterType.value) count++
  if (filterDateRange.value) count++
  if (filterSizeRange.value) count++
  if (selectedTags.value.length > 0) count++
  return count
})

// Document types
const documentTypes = computed(() => {
  return Array.from(new Set(documents.value.map(d => d.document_type)))
})

// Available subcategories based on selected category
const availableSubcategories = computed(() => {
  if (!filterCategory.value) return []
  const docs = documents.value.filter(d => d.category === filterCategory.value)
  return Array.from(new Set(docs.map(d => d.subcategory).filter(Boolean))) as string[]
})

// Top tags
const topTags = computed(() => {
  const tagCounts = new Map<string, number>()
  documents.value.forEach(doc => {
    doc.tags.forEach(tag => {
      tagCounts.set(tag, (tagCounts.get(tag) || 0) + 1)
    })
  })
  return Array.from(tagCounts.entries())
    .sort((a, b) => b[1] - a[1])
    .slice(0, 10)
    .map(([tag]) => tag)
})

// Toggle tag selection
function toggleTag(tag: string) {
  const index = selectedTags.value.indexOf(tag)
  if (index > -1) {
    selectedTags.value.splice(index, 1)
  } else {
    selectedTags.value.push(tag)
  }
}

// Reset filters
function resetFilters() {
  filterCategory.value = ''
  filterSubcategory.value = ''
  filterType.value = ''
  filterDateRange.value = ''
  filterSizeRange.value = ''
  selectedTags.value = []
  searchQuery.value = ''
}

// Delete document
async function deleteDocument(id: string) {
  if (!confirm('Êtes-vous sûr de vouloir supprimer ce document ?')) return

  try {
    await invoke('delete_document', { id })
    await loadDocuments()
  } catch (error) {
    console.error('Failed to delete document:', error)
    alert(`Erreur lors de la suppression: ${error}`)
  }
}

// Open document
async function openDocument(doc: Document) {
  try {
    await invoke('open_file', { file_path: doc.file_path })
  } catch (error) {
    console.error('Failed to open document:', error)
    alert(`Erreur lors de l'ouverture: ${error}`)
  }
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
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

// Get category icon
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

// Get category color
function getCategoryColor(category: string): string {
  const colors: Record<string, string> = {
    'Administratif': '#757575',
    'Financier': '#000000',
    'Santé': '#D32F2F',
    'Professionnel': '#424242',
    'Immobilier': '#616161',
    'Académique': '#212121',
    'Personnel': '#9E9E9E',
    'Autre': '#BDBDBD'
  }
  return colors[category] || '#BDBDBD'
}

// Reset page on filters change
watch([searchQuery, filterCategory, filterSubcategory, filterType, filterDateRange, filterSizeRange, selectedTags, sortBy, itemsPerPage], () => {
  currentPage.value = 1
})

onMounted(() => {
  loadDocuments()
})
</script>

<style scoped>
.explorer {
  display: flex;
  flex-direction: column;
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

.app-subtitle {
  color: var(--md-sys-color-on-surface-variant);
  margin-left: var(--md-sys-spacing-sm);
}

/* Main Content */
.explorer-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: var(--md-sys-spacing-xl) var(--md-sys-spacing-lg);
  width: 100%;
}

/* Search & Filters Bar */
.search-filters-bar {
  display: flex;
  gap: var(--md-sys-spacing-md);
  align-items: center;
  margin-bottom: var(--md-sys-spacing-lg);
  flex-wrap: wrap;
}

.md-search-field {
  position: relative;
  flex: 1;
  min-width: 300px;
}

.search-icon {
  position: absolute;
  left: var(--md-sys-spacing-md);
  top: 50%;
  transform: translateY(-50%);
  color: var(--md-sys-color-on-surface-variant);
}

.search-input {
  width: 100%;
  padding: var(--md-sys-spacing-md) var(--md-sys-spacing-md) var(--md-sys-spacing-md) 48px;
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  background-color: var(--md-sys-color-surface);
  color: var(--md-sys-color-on-surface);
  font-family: var(--md-sys-typescale-body-large-font);
  font-size: var(--md-sys-typescale-body-large-size);
}

.search-input:focus {
  outline: 2px solid var(--md-sys-color-primary);
  outline-offset: 2px;
}

.filter-badge {
  background-color: var(--md-sys-color-error);
  color: var(--md-sys-color-on-error);
  border-radius: var(--md-sys-shape-corner-full);
  padding: 2px 6px;
  font-size: 12px;
  margin-left: var(--md-sys-spacing-xs);
}

.md-select-compact {
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

.md-select-compact:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 8%, var(--md-sys-color-surface-container-highest));
  box-shadow: var(--md-sys-elevation-level2);
}

.md-select-compact:focus {
  outline: none;
  background-color: var(--md-sys-color-surface-container-highest);
  box-shadow: var(--md-sys-elevation-level2);
}

/* Filters Panel */
.filters-panel {
  background-color: var(--md-sys-color-surface-variant);
  border-radius: var(--md-sys-shape-corner-large);
  padding: var(--md-sys-spacing-lg);
  margin-bottom: var(--md-sys-spacing-lg);
}

.filters-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--md-sys-spacing-md);
  margin-bottom: var(--md-sys-spacing-md);
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-xs);
}

.filter-group label {
  color: var(--md-sys-color-on-surface-variant);
  font-weight: 500;
}

.filter-group .md-select {
  padding: var(--md-sys-spacing-md);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-medium);
  background-color: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  font-family: var(--md-sys-typescale-body-large-font-family);
  font-size: var(--md-sys-typescale-body-large-font-size);
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
}

.filter-group .md-select:hover:not(:disabled) {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 8%, var(--md-sys-color-surface-container));
  border-color: var(--md-sys-color-on-surface);
}

.filter-group .md-select:focus {
  outline: 2px solid var(--md-sys-color-primary);
  outline-offset: 2px;
  border-color: var(--md-sys-color-primary);
}

.filter-group .md-select:disabled {
  opacity: 0.38;
  cursor: not-allowed;
  background-color: var(--md-sys-color-surface-variant);
}

.filter-group-full {
  grid-column: 1 / -1;
}

.tags-filter {
  display: flex;
  gap: var(--md-sys-spacing-sm);
  flex-wrap: wrap;
}

.filters-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--md-sys-spacing-md);
}

/* Results Info */
.results-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--md-sys-spacing-lg);
  flex-wrap: wrap;
  gap: var(--md-sys-spacing-md);
}

.pagination-controls {
  display: flex;
  gap: var(--md-sys-spacing-sm);
}

.items-per-page-select {
  padding: var(--md-sys-spacing-sm) var(--md-sys-spacing-md);
  border: none;
  border-radius: var(--md-sys-shape-corner-full);
  background-color: var(--md-sys-color-surface-container-highest);
  color: var(--md-sys-color-on-surface);
  font-family: var(--md-sys-typescale-body-small-font-family);
  font-size: var(--md-sys-typescale-body-small-font-size);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
  box-shadow: var(--md-sys-elevation-level1);
}

.items-per-page-select:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 8%, var(--md-sys-color-surface-container-highest));
  box-shadow: var(--md-sys-elevation-level2);
}

.items-per-page-select:focus {
  outline: none;
  background-color: var(--md-sys-color-surface-container-highest);
  box-shadow: var(--md-sys-elevation-level2);
}

/* Documents Grid */
.documents-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--md-sys-spacing-md);
  gap: var(--md-sys-spacing-lg); /* More vertical space between cards */
  margin-bottom: var(--md-sys-spacing-lg);
  margin-top: var(--md-sys-spacing-md);
}

.document-card {
  padding: var(--md-sys-spacing-lg);
  padding-top: calc(var(--md-sys-spacing-lg) + 8px); /* Extra space for category badge */
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-md);
  position: relative;
  overflow: visible; /* Allow badge to overflow */
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
  border-radius: 12px;
  color: white;
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
  margin-top: var(--md-sys-spacing-sm);
}

.doc-type-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.subcategory-text {
  color: var(--md-sys-color-on-surface-variant);
  font-style: italic;
}

.doc-name {
  color: var(--md-sys-color-on-surface);
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
}

.meta-item {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-xs);
}

.doc-tags {
  display: flex;
  gap: var(--md-sys-spacing-xs);
  flex-wrap: wrap;
}

.doc-action-btn {
  margin-top: auto;
  width: 100%;
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

/* Loading & Empty States */
.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--md-sys-spacing-2xl);
  text-align: center;
  gap: var(--md-sys-spacing-md);
}

.loading-skeleton {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: var(--md-sys-spacing-md);
  width: 100%;
}

/* Modal (reuse from Home) */
.md-modal-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: var(--md-sys-spacing-lg);
}

.md-modal {
  background-color: var(--md-sys-color-surface);
  border-radius: var(--md-sys-shape-corner-large);
  max-width: 600px;
  width: 100%;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.md-modal-header {
  padding: var(--md-sys-spacing-lg);
  border-bottom: 1px solid var(--md-sys-color-outline-variant);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.md-modal-content {
  padding: var(--md-sys-spacing-lg);
  overflow-y: auto;
  flex: 1;
}

.modal-info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--md-sys-spacing-md);
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

.modal-tags,
.modal-ocr {
  margin-top: var(--md-sys-spacing-lg);
}

.tags-list {
  display: flex;
  gap: var(--md-sys-spacing-xs);
  flex-wrap: wrap;
  margin-top: var(--md-sys-spacing-sm);
}

.ocr-text {
  margin-top: var(--md-sys-spacing-sm);
  white-space: pre-wrap;
  color: var(--md-sys-color-on-surface-variant);
}

.md-modal-footer {
  padding: var(--md-sys-spacing-lg);
  border-top: 1px solid var(--md-sys-color-outline-variant);
  display: flex;
  justify-content: flex-end;
  gap: var(--md-sys-spacing-md);
}

/* Animations */
@keyframes fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes slide-in-down {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes scale-in {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.animate-fade-in {
  animation: fade-in 0.3s ease-out;
}

.animate-slide-in-down {
  animation: slide-in-down 0.3s ease-out;
}

.animate-scale-in {
  animation: scale-in 0.2s ease-out;
}
</style>
