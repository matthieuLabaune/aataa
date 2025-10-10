<template>
  <div class="document-list">
    <div
      v-for="doc in documents"
      :key="doc.id"
      class="list-item surface-container-low rounded-medium elevation-1 transition-standard"
      :class="{ 'list-item-selected': selectedDocument?.id === doc.id }"
      @click="selectDocument(doc)"
    >
      <div class="list-item-icon">📄</div>
      <div class="list-item-content">
        <h3 class="title-medium">{{ doc.new_name }}</h3>
        <div class="list-item-meta body-small">
          <span>{{ doc.document_type }}</span>
          <span>•</span>
          <span>{{ formatDate(doc.created_at) }}</span>
          <span v-if="doc.file_size">•</span>
          <span v-if="doc.file_size">{{ formatFileSize(doc.file_size) }}</span>
        </div>
        <div class="list-item-tags" v-if="doc.tags && doc.tags.length > 0">
          <span v-for="tag in doc.tags" :key="tag" class="chip-small">{{ tag }}</span>
        </div>
      </div>
      <div class="list-item-actions">
        <button class="icon-button" @click.stop="openDocument(doc)">👁️</button>
        <button class="icon-button" @click.stop="editDocument(doc)">✏️</button>
      </div>
    </div>

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
  open: [document: Document]
  edit: [document: Document]
}>()

const { selectedDocument } = useDocumentFilters()

function selectDocument(doc: Document) {
  selectedDocument.value = doc
  emit('select', doc)
}

function openDocument(doc: Document) {
  emit('open', doc)
}

function editDocument(doc: Document) {
  emit('edit', doc)
}

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}

function formatFileSize(bytes: number) {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}
</script>

<style scoped>
.document-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
}

.list-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  cursor: pointer;
  border: 1px solid transparent;
}

.list-item:hover {
  box-shadow: var(--md-sys-elevation-level2);
  transform: translateY(-1px);
}

.list-item-selected {
  border-color: var(--md-sys-color-primary);
  background-color: var(--md-sys-color-primary-container);
}

.list-item-icon {
  font-size: 32px;
  width: 48px;
  height: 48px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--md-sys-color-surface-variant);
  border-radius: var(--md-sys-shape-corner-medium);
}

.list-item-content {
  flex: 1;
  min-width: 0;
}

.list-item-content h3 {
  margin: 0 0 4px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.list-item-meta {
  display: flex;
  gap: 8px;
  color: var(--md-sys-color-on-surface-variant);
  margin-bottom: 8px;
  flex-wrap: wrap;
}

.list-item-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.list-item-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

/* Mobile: Vertical layout, full-width actions */
@media (max-width: 767px) {
  .document-list {
    gap: 8px;
  }

  .list-item {
    flex-direction: column;
    align-items: flex-start;
    padding: 12px;
    gap: 12px;
  }

  .list-item-icon {
    width: 40px;
    height: 40px;
    font-size: 24px;
  }

  .list-item-meta {
    font-size: 12px;
  }

  .list-item-actions {
    width: 100%;
    justify-content: flex-end;
  }
}

/* Tablet: Compact spacing */
@media (min-width: 768px) and (max-width: 1199px) {
  .document-list {
    gap: 12px;
  }

  .list-item {
    padding: 14px;
  }
}

/* Desktop and up: Full spacing */
@media (min-width: 1200px) {
  .document-list {
    gap: 16px;
  }

  .list-item {
    padding: 16px;
  }
}
</style>
