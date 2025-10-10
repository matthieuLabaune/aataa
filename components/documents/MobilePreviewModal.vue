<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="modelValue && document" class="modal-overlay" @click.self="close">
        <div class="modal-container surface-container-high rounded-extra-large-top">
          <!-- Drag Handle -->
          <div class="modal-handle">
            <div class="handle-bar surface-variant"></div>
          </div>

          <!-- Header -->
          <div class="modal-header">
            <h2 class="headline-small on-surface">{{ document.new_name }}</h2>
            <button class="icon-button" @click="close" aria-label="Fermer">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
              </svg>
            </button>
          </div>

          <!-- Content -->
          <div class="modal-content">
            <!-- Thumbnail -->
            <div class="preview-thumbnail surface-container-highest rounded-large">
              <div class="file-icon">
                <span class="display-medium on-surface-variant">{{ fileExtension }}</span>
              </div>
            </div>

            <!-- Metadata -->
            <div class="metadata-section">
              <h3 class="title-medium on-surface">Informations</h3>
              
              <div class="metadata-row">
                <span class="label-large on-surface-variant">Type</span>
                <span :class="['chip', 'chip-small', `chip-${typeColor}`]">{{ document.type }}</span>
              </div>

              <div class="metadata-row">
                <span class="label-large on-surface-variant">Date</span>
                <span class="body-medium on-surface">{{ formattedDate }}</span>
              </div>

              <div class="metadata-row">
                <span class="label-large on-surface-variant">Taille</span>
                <span class="body-medium on-surface">{{ fileSize }}</span>
              </div>

              <div v-if="document.tags && document.tags.length > 0" class="metadata-row">
                <span class="label-large on-surface-variant">Tags</span>
                <div class="tags-list">
                  <span v-for="tag in document.tags" :key="tag" class="chip chip-small chip-outline">{{ tag }}</span>
                </div>
              </div>
            </div>

            <!-- Notes -->
            <div class="notes-section">
              <h3 class="title-medium on-surface">Notes</h3>
              <textarea 
                v-model="localNotes"
                class="notes-input body-large surface-container-highest rounded-medium on-surface"
                placeholder="Ajouter des notes..."
                rows="4"
              ></textarea>
              <button 
                v-if="notesChanged"
                @click="saveNotes"
                class="button button-filled"
              >
                <span class="label-large">Enregistrer les notes</span>
              </button>
            </div>

            <!-- OCR Text (Collapsible) -->
            <div v-if="document.ocr_text" class="ocr-section">
              <button @click="ocrExpanded = !ocrExpanded" class="ocr-toggle">
                <span class="title-medium on-surface">Texte OCR</span>
                <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor" :style="{transform: ocrExpanded ? 'rotate(180deg)' : 'rotate(0deg)', transition: 'transform 0.2s'}">
                  <path d="M7 10l5 5 5-5z"/>
                </svg>
              </button>
              <div v-if="ocrExpanded" class="ocr-content body-medium surface-container-highest rounded-medium on-surface-variant">
                {{ document.ocr_text }}
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="modal-actions">
            <button @click="$emit('open')" class="button button-filled">
              <span class="label-large">Ouvrir</span>
            </button>
            <button @click="$emit('edit')" class="button button-outlined">
              <span class="label-large">Éditer</span>
            </button>
            <button @click="$emit('delete')" class="button button-text error">
              <span class="label-large">Supprimer</span>
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { Document } from '~/types/document'

const props = defineProps<{
  modelValue: boolean
  document: Document | null
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'open'): void
  (e: 'edit'): void
  (e: 'delete'): void
  (e: 'updateNotes', notes: string): void
}>()

const localNotes = ref(props.document?.notes || '')
const ocrExpanded = ref(false)

watch(() => props.document, (newDoc) => {
  if (newDoc) {
    localNotes.value = newDoc.notes || ''
    ocrExpanded.value = false
  }
}, { immediate: true })

const notesChanged = computed(() => {
  return localNotes.value !== (props.document?.notes || '')
})

const fileExtension = computed(() => {
  if (!props.document?.original_name) return 'PDF'
  const ext = props.document.original_name.split('.').pop()?.toUpperCase()
  return ext || 'PDF'
})

const formattedDate = computed(() => {
  if (!props.document?.date) return '-'
  return new Date(props.document.date).toLocaleDateString('fr-FR', {
    day: '2-digit',
    month: 'long',
    year: 'numeric'
  })
})

const fileSize = computed(() => {
  if (!props.document?.file_size) return '-'
  const size = props.document.file_size
  if (size < 1024) return `${size} B`
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`
  return `${(size / (1024 * 1024)).toFixed(1)} MB`
})

const typeColor = computed(() => {
  const type = props.document?.type?.toLowerCase()
  if (type === 'facture') return 'primary'
  if (type === 'contrat') return 'secondary'
  if (type === 'relevé') return 'tertiary'
  if (type === 'bulletin') return 'primary'
  if (type === 'officiel') return 'error'
  return 'primary'
})

function close() {
  emit('update:modelValue', false)
}

function saveNotes() {
  emit('updateNotes', localNotes.value)
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 1000;
  display: flex;
  align-items: flex-end;
}

.modal-container {
  width: 100%;
  max-height: 90vh;
  overflow-y: auto;
  padding: 8px 16px 24px;
  animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes slideUp {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}

.modal-handle {
  display: flex;
  justify-content: center;
  padding: 8px 0;
  margin-bottom: 8px;
}

.handle-bar {
  width: 32px;
  height: 4px;
  border-radius: 2px;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 16px;
}

.modal-header h2 {
  flex: 1;
  margin: 0;
}

.modal-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.preview-thumbnail {
  width: 100%;
  aspect-ratio: 1.414;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.file-icon {
  text-align: center;
}

.metadata-section,
.notes-section,
.ocr-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.metadata-section h3,
.notes-section h3 {
  margin: 0;
}

.metadata-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 8px 0;
}

.tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.notes-input {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--md-sys-color-outline);
  resize: vertical;
  font-family: 'Roboto', sans-serif;
}

.notes-input:focus {
  outline: 2px solid var(--md-sys-color-primary);
  outline-offset: -1px;
}

.ocr-toggle {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 12px;
  background: none;
  border: none;
  cursor: pointer;
  border-radius: var(--md-sys-shape-corner-medium);
}

.ocr-toggle:hover {
  background: var(--md-sys-color-surface-container-highest);
}

.ocr-content {
  padding: 16px;
  max-height: 200px;
  overflow-y: auto;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.modal-actions {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-top: 24px;
}

.modal-actions .button {
  width: 100%;
}

/* Modal Transitions */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-container,
.modal-leave-active .modal-container {
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: translateY(100%);
}

/* Tablet and up: Center modal instead of bottom sheet */
@media (min-width: 768px) {
  .modal-overlay {
    align-items: center;
    justify-content: center;
  }

  .modal-container {
    max-width: 600px;
    max-height: 80vh;
    border-radius: var(--md-sys-shape-corner-extra-large);
  }

  .modal-handle {
    display: none;
  }

  .modal-actions {
    flex-direction: row;
    justify-content: flex-end;
  }

  .modal-actions .button {
    width: auto;
  }

  @keyframes slideUp {
    from {
      transform: scale(0.9);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }
}
</style>
