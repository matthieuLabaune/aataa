<template>
  <div class="document-card card surface-container-low rounded-medium elevation-1 transition-standard">
    <div class="card-header">
      <div class="card-icon">📄</div>
      <div class="card-type-badge">
        <span :class="['chip', 'chip-small', `chip-${typeColor}`]">
          {{ document.document_type }}
        </span>
      </div>
    </div>

    <div class="card-content">
      <h3 class="title-medium card-title">{{ document.new_name }}</h3>
      
      <div class="card-meta body-small">
        <span>📅 {{ formatDateShort(document.created_at) }}</span>
        <span v-if="document.file_size">• 💾 {{ formattedSize }}</span>
      </div>

      <div v-if="document.tags && document.tags.length > 0" class="card-tags">
        <span v-for="tag in document.tags.slice(0, 3)" :key="tag" class="chip chip-small">
          {{ tag }}
        </span>
        <span v-if="document.tags.length > 3" class="chip chip-small">
          +{{ document.tags.length - 3 }}
        </span>
      </div>
    </div>

    <div class="card-actions">
      <button class="icon-button" @click="$emit('preview')" title="Prévisualiser">
        �️
      </button>
      <button class="icon-button" @click="$emit('open')" title="Ouvrir">
        📂
      </button>
      <button class="icon-button" @click="$emit('delete')" title="Supprimer">
        🗑️
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

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
}

const props = defineProps<{
  document: Document
}>()

defineEmits<{
  preview: []
  open: []
  delete: []
}>()

const typeColors: Record<string, string> = {
  'Facture': 'primary',
  'Contrat': 'secondary',
  'Relevé bancaire': 'tertiary',
  'Bulletin de paie': 'primary',
  'Document officiel': 'error',
  'Reçu': 'secondary',
  'Unknown': 'surface-variant'
}

const typeColor = computed(() => typeColors[props.document.document_type] || 'surface-variant')

const formattedSize = computed(() => {
  const bytes = props.document.file_size
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
})

function formatDateShort(dateString: string) {
  return new Date(dateString).toLocaleDateString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}
</script>

<style scoped>
.document-card {
  display: flex;
  flex-direction: column;
  padding: 16px;
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

.document-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, var(--md-sys-color-primary) 0%, var(--md-sys-color-secondary) 100%);
  opacity: 0;
  transition: opacity var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
}

.document-card:hover::before {
  opacity: 1;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.card-icon {
  font-size: 32px;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--md-sys-color-surface-variant);
  border-radius: var(--md-sys-shape-corner-medium);
}

.card-content {
  flex: 1;
  margin-bottom: 12px;
}

.card-title {
  margin: 0 0 8px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  color: var(--md-sys-color-on-surface);
}

.card-meta {
  display: flex;
  gap: 8px;
  color: var(--md-sys-color-on-surface-variant);
  margin-bottom: 8px;
}

.card-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.card-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding-top: 8px;
  border-top: 1px solid var(--md-sys-color-outline-variant);
}

/* Type-specific chip colors */
.chip-primary {
  background-color: var(--md-sys-color-primary-container);
  color: var(--md-sys-color-on-primary-container);
}

.chip-secondary {
  background-color: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
}

.chip-tertiary {
  background-color: var(--md-sys-color-tertiary-container);
  color: var(--md-sys-color-on-tertiary-container);
}

.chip-error {
  background-color: var(--md-sys-color-error-container);
  color: var(--md-sys-color-on-error-container);
}
</style>
