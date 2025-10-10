<template>
  <AppLayout>
    <div class="test-content">
      <h1 class="headline-large on-surface">Test Material Design 3 Layout</h1>
      <p class="body-large on-surface-variant" style="margin-bottom: 32px;">
        Testez toutes les fonctionnalités du nouveau layout 🎉
      </p>

      <!-- Test Controls -->
      <div class="test-controls card surface-container-low rounded-large" style="padding: 24px; margin-bottom: 32px;">
        <h2 class="title-large on-surface" style="margin-bottom: 16px;">Contrôles de test</h2>
        
        <div style="display: flex; gap: 16px; flex-wrap: wrap; margin-bottom: 16px;">
          <button class="button button-filled" @click="addTestDocuments">
            <span class="label-large">Ajouter 20 documents de test</span>
          </button>
          
          <button class="button button-outlined" @click="clearTestDocuments">
            <span class="label-large">Effacer les documents</span>
          </button>
          
          <button class="button button-text" @click="toggleViewMode">
            <span class="label-large">Toggle {{ viewMode === 'grid' ? 'List' : 'Grid' }}</span>
          </button>
        </div>

        <div class="label-medium on-surface-variant">
          {{ testDocuments.length }} document{{ testDocuments.length > 1 ? 's' : '' }} de test
        </div>
      </div>

      <!-- Filter Chips Test -->
      <div v-if="hasFilters" class="filter-chips" style="margin-bottom: 24px;">
        <span class="chip chip-small chip-primary">
          Facture
          <button @click="clearFilters" class="chip-clear">×</button>
        </span>
        <span class="chip chip-small chip-secondary">
          2024
          <button @click="clearFilters" class="chip-clear">×</button>
        </span>
        <span class="chip chip-small chip-tertiary">
          Important
          <button @click="clearFilters" class="chip-clear">×</button>
        </span>
      </div>

      <!-- View Toggle -->
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;">
        <div class="label-large on-surface-variant">Mode: {{ viewMode }}</div>
        <ViewToggle v-model="viewMode" />
      </div>

      <!-- Document Grid/List -->
      <DocumentGrid
        v-if="viewMode === 'grid'"
        :documents="testDocuments"
        @select="handleSelect"
      />
      <DocumentList
        v-else
        :documents="testDocuments"
        @select="handleSelect"
        @open="handleOpen"
        @edit="handleEdit"
      />
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import type { Document } from '~/types/document'

const { selectedDocument, viewMode } = useDocumentFilters()
const testDocuments = ref<Document[]>([])
const hasFilters = ref(true)

function addTestDocuments() {
  const types = ['Facture', 'Contrat', 'Relevé', 'Bulletin', 'Officiel']
  const tags = ['important', 'archivé', 'urgent', 'personnel', 'professionnel']
  
  const newDocs: Document[] = []
  for (let i = 1; i <= 20; i++) {
    newDocs.push({
      id: `test-${i}`,
      new_name: `Document_Test_${i}.pdf`,
      original_name: `original_${i}.pdf`,
      type: types[Math.floor(Math.random() * types.length)],
      date: new Date(2024, Math.floor(Math.random() * 12), Math.floor(Math.random() * 28) + 1).toISOString(),
      created_at: new Date().toISOString(),
      file_size: Math.floor(Math.random() * 5000000) + 100000,
      tags: [tags[Math.floor(Math.random() * tags.length)], tags[Math.floor(Math.random() * tags.length)]],
      notes: `Notes de test pour le document ${i}`,
      ocr_text: `Ceci est le texte OCR simulé pour le document de test numéro ${i}. Il contient des informations importantes.`,
      file_path: `/test/path/document_${i}.pdf`
    })
  }
  testDocuments.value = newDocs
}

function clearTestDocuments() {
  testDocuments.value = []
  selectedDocument.value = null
}

function toggleViewMode() {
  viewMode.value = viewMode.value === 'grid' ? 'list' : 'grid'
}

function clearFilters() {
  hasFilters.value = false
}

function handleSelect(doc: Document) {
  selectedDocument.value = doc
}

function handleOpen(doc: Document) {
  console.log('Open:', doc)
}

function handleEdit(doc: Document) {
  console.log('Edit:', doc)
}

// Add some test documents on mount
onMounted(() => {
  addTestDocuments()
})
</script>

<style scoped>
.test-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.test-content h1 {
  margin: 0;
}

.test-controls {
  display: flex;
  flex-direction: column;
  gap: 16px;
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

/* Mobile adjustments */
@media (max-width: 767px) {
  .test-content {
    gap: 16px;
  }
}
</style>
