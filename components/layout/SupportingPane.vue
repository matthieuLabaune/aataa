<template>
  <aside class="supporting-pane surface-container elevation-1">
    <div class="pane-content">
      <!-- Preview Header -->
      <div class="preview-header">
        <h2 class="title-large">Aperçu</h2>
        <button class="icon-button" @click="closePane" aria-label="Fermer">
          <span>✕</span>
        </button>
      </div>

      <!-- Document Preview -->
      <div v-if="document" class="document-preview">
        <!-- Preview Image/Icon -->
        <div class="preview-image surface-variant rounded-medium">
          <div class="preview-placeholder">
            <span class="preview-icon">📄</span>
          </div>
        </div>

        <!-- Metadata -->
        <div class="preview-metadata">
          <h3 class="title-medium document-name">{{ document.new_name }}</h3>

          <div class="metadata-grid">
            <div class="metadata-row">
              <span class="label-medium metadata-label">Type:</span>
              <span class="body-medium">{{ document.document_type }}</span>
            </div>

            <div class="metadata-row">
              <span class="label-medium metadata-label">Date:</span>
              <span class="body-medium">{{ formatDate(document.created_at) }}</span>
            </div>

            <div class="metadata-row">
              <span class="label-medium metadata-label">Taille:</span>
              <span class="body-medium">{{ formatFileSize(document.file_size) }}</span>
            </div>
          </div>

          <!-- Tags -->
          <div v-if="document.tags && document.tags.length > 0" class="metadata-tags">
            <span
              v-for="tag in document.tags"
              :key="tag"
              class="chip chip-small"
            >
              {{ tag }}
            </span>
          </div>
        </div>

        <div class="divider"></div>

        <!-- Notes Section -->
        <div class="preview-notes">
          <label class="label-large notes-label">Notes</label>
          <textarea
            v-model="localNotes"
            class="notes-input body-medium"
            placeholder="Ajouter des notes..."
            rows="4"
          ></textarea>
        </div>

        <div class="divider"></div>

        <!-- Actions -->
        <div class="preview-actions">
          <button class="btn btn-primary">
            <span>👁️</span>
            Ouvrir
          </button>
          <button class="btn btn-outlined">
            <span>✏️</span>
            Éditer
          </button>
          <button class="btn btn-error">
            <span>🗑️</span>
            Supprimer
          </button>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="empty-preview">
        <span class="empty-icon">📄</span>
        <p class="body-large">Sélectionnez un document pour voir les détails</p>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
const document = useState('selectedDocument', () => null)
const localNotes = ref(document.value?.notes || '')

// Watch for document changes
watch(document, (newDoc) => {
  localNotes.value = newDoc?.notes || ''
})

function closePane() {
  document.value = null
}

function formatDate(dateString: string) {
  if (!dateString) return '-'
  const date = new Date(dateString)
  return date.toLocaleDateString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}

function formatFileSize(bytes: number) {
  if (!bytes) return '-'
  const kb = bytes / 1024
  if (kb < 1024) return `${kb.toFixed(1)} KB`
  const mb = kb / 1024
  return `${mb.toFixed(1)} MB`
}
</script>

<style scoped>
.supporting-pane {
  width: 360px;
  height: calc(100vh - 64px); /* Minus top bar height */
  overflow-y: auto;
  background-color: var(--md-sys-color-surface-container);
  border-left: 1px solid var(--md-sys-color-outline-variant);
}

.pane-content {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.preview-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.preview-header h2 {
  margin: 0;
}

.document-preview {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.preview-image {
  width: 100%;
  aspect-ratio: 1 / 1.4;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--md-sys-color-surface-variant);
  overflow: hidden;
}

.preview-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
}

.preview-icon {
  font-size: 64px;
  opacity: 0.5;
}

.document-name {
  margin: 0;
  word-break: break-word;
}

.metadata-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.metadata-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.metadata-label {
  color: var(--md-sys-color-on-surface-variant);
  min-width: 60px;
}

.metadata-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 8px;
}

.preview-notes {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.notes-label {
  margin: 0;
}

.notes-input {
  width: 100%;
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
  border-width: 2px;
  padding: 11px; /* Adjust for thicker border */
}

.preview-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preview-actions .btn {
  width: 100%;
  justify-content: center;
  gap: 8px;
}

.empty-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 48px 24px;
  text-align: center;
  color: var(--md-sys-color-on-surface-variant);
}

.empty-icon {
  font-size: 64px;
  opacity: 0.3;
}
</style>
