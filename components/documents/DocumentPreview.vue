<template>
  <div v-if="document" class="document-preview">
    <div class="preview-header">
      <h2 class="title-large">{{ document.new_name }}</h2>
      <button class="icon-button" @click="$emit('close')" title="Fermer">
        ✕
      </button>
    </div>

    <div class="preview-thumbnail surface-variant rounded-medium">
      <div class="preview-icon">📄</div>
      <span class="label-small">{{ getFileExtension(document.new_name) }}</span>
    </div>

    <div class="preview-metadata">
      <div class="metadata-row">
        <span class="label-medium">Type:</span>
        <span class="chip chip-small chip-primary">{{ document.document_type }}</span>
      </div>
      
      <div class="metadata-row">
        <span class="label-medium">Date:</span>
        <span class="body-medium">{{ formatDate(document.created_at) }}</span>
      </div>
      
      <div class="metadata-row">
        <span class="label-medium">Taille:</span>
        <span class="body-medium">{{ formatFileSize(document.file_size) }}</span>
      </div>

      <div class="metadata-row" v-if="document.tags && document.tags.length > 0">
        <span class="label-medium">Tags:</span>
        <div class="metadata-tags">
          <span v-for="tag in document.tags" :key="tag" class="chip chip-small">
            {{ tag }}
          </span>
        </div>
      </div>
    </div>

    <div class="preview-notes">
      <label class="label-large">Notes</label>
      <textarea 
        v-model="localNotes" 
        class="notes-input body-medium"
        placeholder="Ajouter des notes..."
        rows="4"
      ></textarea>
      <button 
        v-if="localNotes !== document.notes" 
        class="btn btn-primary"
        @click="saveNotes"
      >
        💾 Enregistrer
      </button>
    </div>

    <div class="preview-actions">
      <button class="btn btn-primary" @click="$emit('open', document)">
        📂 Ouvrir
      </button>
      <button class="btn btn-secondary" @click="$emit('edit', document)">
        ✏️ Éditer
      </button>
      <button class="btn btn-outlined btn-error" @click="$emit('delete', document)">
        🗑️ Supprimer
      </button>
    </div>

    <details v-if="document.ocr_text" class="preview-ocr">
      <summary class="label-large">📜 Texte extrait (OCR)</summary>
      <div class="ocr-content body-small surface-container rounded-small">
        {{ document.ocr_text }}
      </div>
    </details>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface Document {
  id: string
  original_name: string
  new_name: string
  file_path: string
  document_type: string
  tags: string[]
  ocr_text: string
  created_at: string
  file_size: number
  notes?: string | null
}

const props = defineProps<{
  document: Document | null
}>()

const emit = defineEmits<{
  close: []
  open: [document: Document]
  edit: [document: Document]
  delete: [document: Document]
  updateNotes: [id: string, notes: string]
}>()

const localNotes = ref(props.document?.notes || '')

watch(() => props.document, (newDoc) => {
  localNotes.value = newDoc?.notes || ''
})

function formatDate(dateString: string) {
  return new Date(dateString).toLocaleDateString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

function formatFileSize(bytes: number) {
  if (!bytes) return 'N/A'
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

function getFileExtension(filename: string) {
  const ext = filename.split('.').pop()?.toUpperCase()
  return ext || 'FILE'
}

function saveNotes() {
  if (props.document) {
    emit('updateNotes', props.document.id, localNotes.value)
  }
}
</script>

<style scoped>
.document-preview {
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding: 24px;
  height: 100%;
  overflow-y: auto;
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
}

.preview-header h2 {
  margin: 0;
  flex: 1;
  word-break: break-word;
}

.preview-thumbnail {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px;
  min-height: 200px;
  gap: 12px;
}

.preview-icon {
  font-size: 64px;
}

.preview-metadata {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.metadata-row {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.metadata-row > .label-medium {
  min-width: 80px;
  color: var(--md-sys-color-on-surface-variant);
}

.metadata-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  flex: 1;
}

.preview-notes {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.notes-input {
  padding: 12px;
  border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-small);
  background-color: var(--md-sys-color-surface);
  color: var(--md-sys-color-on-surface);
  font-family: inherit;
  resize: vertical;
  transition: border-color var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
}

.notes-input:focus {
  outline: none;
  border-color: var(--md-sys-color-primary);
}

.preview-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preview-actions .btn {
  width: 100%;
  justify-content: center;
}

.preview-ocr {
  margin-top: 8px;
}

.preview-ocr summary {
  cursor: pointer;
  user-select: none;
  padding: 8px 0;
  color: var(--md-sys-color-primary);
}

.ocr-content {
  margin-top: 8px;
  padding: 12px;
  max-height: 300px;
  overflow-y: auto;
  font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
  line-height: 1.6;
  white-space: pre-wrap;
  word-break: break-word;
}
</style>
