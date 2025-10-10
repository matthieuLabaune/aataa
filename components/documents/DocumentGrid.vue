<template>
  <div class="document-grid">
    <DocumentCard
      v-for="doc in documents"
      :key="doc.id"
      :document="doc"
      @click="selectDocument(doc)"
    />
    
    <EmptyState
      v-if="documents.length === 0"
      message="Aucun document trouvé"
      icon="📄"
    />
  </div>
</template>

<script setup lang="ts">
import type { Document } from '~/types/document'

const props = defineProps<{
  documents: Document[]
}>()

const emit = defineEmits<{
  select: [document: Document]
}>()

const { selectedDocument } = useDocumentFilters()

function selectDocument(doc: Document) {
  selectedDocument.value = doc
  emit('select', doc)
}
</script>

<style scoped>
.document-grid {
  display: grid;
  gap: 16px;
  width: 100%;
  height: 100%;
}

/* Mobile: 1 colonne, spacing réduit */
@media (max-width: 767px) {
  .document-grid {
    grid-template-columns: 1fr;
    gap: 12px;
  }
}

/* Tablet: 2 colonnes */
@media (min-width: 768px) and (max-width: 1199px) {
  .document-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
  }
}

/* Desktop: 3 colonnes (ou 2 si supporting pane visible) */
@media (min-width: 1200px) and (max-width: 1599px) {
  .document-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
  }
}

@media (min-width: 1600px) and (max-width: 1919px) {
  .document-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: 20px;
  }
}

/* Wide: 4 colonnes */
@media (min-width: 1920px) {
  .document-grid {
    grid-template-columns: repeat(4, 1fr);
    gap: 24px;
  }
}
</style>
