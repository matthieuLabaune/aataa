<template>
  <AppLayout>
    <div class="documents-page">
      <!-- Page Header -->
      <div class="page-header">
        <h1 class="headline-large on-surface">Tous les documents</h1>
        <ViewToggle v-model="viewMode" />
      </div>

      <!-- Filter Chips -->
      <div v-if="hasActiveFilters" class="filter-chips">
        <span v-if="activeFilters.type" class="chip chip-small chip-primary">
          {{ activeFilters.type }}
          <button @click="clearTypeFilter" class="chip-clear">×</button>
        </span>
        <span v-if="activeFilters.year" class="chip chip-small chip-secondary">
          {{ activeFilters.year }}
          <button @click="clearYearFilter" class="chip-clear">×</button>
        </span>
        <span v-for="tag in activeFilters.tags" :key="tag" class="chip chip-small chip-tertiary">
          {{ tag }}
          <button @click="removeTag(tag)" class="chip-clear">×</button>
        </span>
        <button @click="clearAllFilters" class="chip chip-small chip-outline">
          Tout effacer
        </button>
      </div>

      <!-- Document Count -->
      <div class="document-count label-medium on-surface-variant">
        {{ filteredDocuments.length }} document{{ filteredDocuments.length > 1 ? 's' : '' }}
      </div>

      <!-- Document Display -->
      <DocumentGrid
        v-if="viewMode === 'grid'"
        :documents="filteredDocuments"
        @select="handleDocumentSelect"
      />
      <DocumentList
        v-else
        :documents="filteredDocuments"
        @select="handleDocumentSelect"
        @open="handleOpenDocument"
        @edit="handleEditDocument"
      />
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { Document } from '~/types/document'

const { activeFilters, viewMode, selectedDocument } = useDocumentFilters()

// Load documents from database
const documents = ref<Document[]>([])

async function loadDocuments() {
  try {
    const docs = await invoke<Document[]>('get_all_documents')
    documents.value = docs
  } catch (error) {
    console.error('Failed to load documents:', error)
  }
}

onMounted(() => {
  loadDocuments()
})

// Filtered documents based on active filters
const filteredDocuments = computed(() => {
  let result = documents.value

  // Filter by type
  if (activeFilters.value.type) {
    result = result.filter(doc => doc.type === activeFilters.value.type)
  }

  // Filter by year
  if (activeFilters.value.year) {
    result = result.filter(doc => {
      const docYear = new Date(doc.date).getFullYear().toString()
      return docYear === activeFilters.value.year
    })
  }

  // Filter by tags
  if (activeFilters.value.tags && activeFilters.value.tags.length > 0) {
    result = result.filter(doc => {
      return activeFilters.value.tags!.some(tag => doc.tags?.includes(tag))
    })
  }

  // Filter by search query
  if (activeFilters.value.search) {
    const query = activeFilters.value.search.toLowerCase()
    result = result.filter(doc => {
      return (
        doc.new_name?.toLowerCase().includes(query) ||
        doc.original_name?.toLowerCase().includes(query) ||
        doc.type?.toLowerCase().includes(query) ||
        doc.ocr_text?.toLowerCase().includes(query) ||
        doc.tags?.some(tag => tag.toLowerCase().includes(query))
      )
    })
  }

  return result
})

// Active filters check
const hasActiveFilters = computed(() => {
  return !!(
    activeFilters.value.type ||
    activeFilters.value.year ||
    (activeFilters.value.tags && activeFilters.value.tags.length > 0) ||
    activeFilters.value.search
  )
})

// Filter management
function clearTypeFilter() {
  activeFilters.value.type = null
}

function clearYearFilter() {
  activeFilters.value.year = null
}

function removeTag(tag: string) {
  activeFilters.value.tags = activeFilters.value.tags?.filter(t => t !== tag) || []
}

function clearAllFilters() {
  activeFilters.value.type = null
  activeFilters.value.year = null
  activeFilters.value.tags = []
  activeFilters.value.search = ''
}

// Document actions
function handleDocumentSelect(doc: Document) {
  selectedDocument.value = doc
}

function handleOpenDocument(doc: Document) {
  // TODO: Implement document opening
  console.log('Open document:', doc)
}

function handleEditDocument(doc: Document) {
  // TODO: Implement document editing
  console.log('Edit document:', doc)
}
</script>

<style scoped>
.documents-page {
  display: flex;
  flex-direction: column;
  gap: 24px;
  height: 100%;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.page-header h1 {
  margin: 0;
}

.filter-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}

.chip-clear {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  margin-left: 4px;
  padding: 0;
  border: none;
  background: none;
  cursor: pointer;
  font-size: 18px;
  line-height: 1;
  opacity: 0.7;
  transition: opacity 0.2s;
}

.chip-clear:hover {
  opacity: 1;
}

.document-count {
  padding-bottom: 8px;
  border-bottom: 1px solid var(--md-sys-color-outline-variant);
}

/* Mobile: Smaller spacing */
@media (max-width: 767px) {
  .documents-page {
    gap: 16px;
  }

  .filter-chips {
    gap: 6px;
  }

  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }

  .page-header h1 {
    font-size: 24px;
  }
}
</style>
