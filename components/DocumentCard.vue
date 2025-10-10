<template>
  <div class="document-card">
    <div class="flex items-start justify-between mb-4">
      <div style="flex: 1">
        <div class="flex items-center gap-2" style="flex-wrap: wrap; margin-bottom: 0.5rem">
          <span :class="['badge', `badge-${typeColor}`]">
            {{ document.document_type }}
          </span>
          <h3 style="font-weight: 700; font-size: 1.05rem; color: #1f2937; margin: 0">
            {{ document.new_name }}
          </h3>
        </div>

        <p class="text-sm text-gray-500 mt-2" style="font-weight: 500">
          📄 {{ document.original_name }}
        </p>

        <div v-if="document.tags.length > 0" class="flex gap-2 mt-2" style="flex-wrap: wrap">
          <span
            v-for="tag in document.tags"
            :key="tag"
            class="badge badge-gray"
            style="font-size: 0.7rem"
          >
            🏷️ {{ tag }}
          </span>
        </div>

        <p class="text-xs text-gray-500 mt-2" style="font-weight: 600">
          📅 {{ formattedDate }} • 💾 {{ formattedSize }}
        </p>

        <details v-if="document.ocr_text" style="margin-top: 0.75rem">
          <summary style="color: #3b82f6; cursor: pointer; font-size: 0.875rem; font-weight: 600; user-select: none">
            📜 Voir le texte extrait
          </summary>
          <p class="text-sm text-gray-600 mt-2" style="padding: 0.75rem; background: linear-gradient(135deg, #f9fafb 0%, #f3f4f6 100%); border-radius: 0.5rem; max-height: 10rem; overflow-y: auto; border: 1px solid #e5e7eb; font-family: 'SF Mono', 'Monaco', 'Courier New', monospace; line-height: 1.6">
            {{ truncatedText }}
          </p>
        </details>
      </div>

      <div class="flex gap-2" style="margin-left: 1rem; flex-shrink: 0">
        <button
          class="icon-btn"
          @click="$emit('preview')"
          title="Prévisualiser"
        >
          🔍
        </button>
        <button
          class="icon-btn"
          @click="$emit('open')"
          title="Ouvrir dans le système"
        >
          📂
        </button>
        <button
          class="icon-btn icon-btn-danger"
          @click="$emit('delete')"
          title="Supprimer"
        >
          🗑
        </button>
      </div>
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
  'Facture': 'blue',
  'Contrat': 'purple',
  'Relevé bancaire': 'green',
  'Bulletin de paie': 'orange',
  'Document officiel': 'red',
  'Reçu': 'yellow',
  'Unknown': 'gray'
}

const typeColor = computed(() => typeColors[props.document.document_type] || 'gray')

const formattedDate = computed(() => {
  const date = new Date(props.document.created_at)
  return date.toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
})

const formattedSize = computed(() => {
  const bytes = props.document.file_size
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
})

const truncatedText = computed(() => {
  const text = props.document.ocr_text
  return text.length > 500 ? text.substring(0, 500) + '...' : text
})
</script>

<style scoped>
.document-card {
  background: white;
  border-radius: 0.75rem;
  padding: 1.5rem;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  margin-bottom: 1.5rem;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid #f3f4f6;
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
  background: linear-gradient(90deg, #3b82f6 0%, #8b5cf6 100%);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.document-card:hover {
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
  border-color: #e5e7eb;
}

.document-card:hover::before {
  opacity: 1;
}

.icon-btn {
  padding: 0.5rem;
  width: 40px;
  height: 40px;
  border: none;
  background: transparent;
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 1.25rem;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-btn:hover {
  background: linear-gradient(135deg, #f9fafb 0%, #f3f4f6 100%);
  transform: scale(1.1);
}

.icon-btn-danger:hover {
  background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
  transform: scale(1.1);
}
</style>
