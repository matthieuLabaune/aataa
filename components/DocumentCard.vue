<template>
  <div class="card">
    <div class="flex items-start justify-between mb-4">
      <div style="flex: 1">
        <div class="flex items-center gap-2" style="flex-wrap: wrap">
          <span :class="['badge', `badge-${typeColor}`]">
            {{ document.document_type }}
          </span>
          <h3 class="font-semibold">
            {{ document.new_name }}
          </h3>
        </div>
        
        <p class="text-sm text-gray-500 mt-2">
          Original: {{ document.original_name }}
        </p>

        <div v-if="document.tags.length > 0" class="flex gap-2 mt-2" style="flex-wrap: wrap">
          <span
            v-for="tag in document.tags"
            :key="tag"
            class="badge badge-gray"
          >
            {{ tag }}
          </span>
        </div>

        <p class="text-xs text-gray-500 mt-2">
          {{ formattedDate }} • {{ formattedSize }}
        </p>

        <details v-if="document.ocr_text" style="margin-top: 0.5rem">
          <summary style="color: #3b82f6; cursor: pointer; font-size: 0.875rem">
            Voir le texte extrait
          </summary>
          <p class="text-sm text-gray-600 mt-2" style="padding: 0.5rem; background: #f3f4f6; border-radius: 0.375rem; max-height: 8rem; overflow-y: auto">
            {{ truncatedText }}
          </p>
        </details>
      </div>

      <div class="flex gap-2" style="margin-left: 1rem">
        <button
          class="btn btn-secondary"
          style="padding: 0.5rem; width: 36px; height: 36px"
          @click="$emit('open')"
          title="Ouvrir"
        >
          👁
        </button>
        <button
          class="btn btn-danger"
          style="padding: 0.5rem; width: 36px; height: 36px"
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
