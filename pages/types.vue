<template>
  <AppLayout>
    <div class="types-page">
      <h1 class="headline-large on-surface">Types de documents</h1>
      
      <!-- Type Categories -->
      <div class="type-categories">
        <div
          v-for="type in documentTypes"
          :key="type.name"
          class="type-card surface-container-low rounded-large elevation-1"
          @click="selectType(type.name)"
        >
          <div class="type-header">
            <div class="type-icon">{{ type.icon }}</div>
            <div class="type-info">
              <h3 class="title-large on-surface">{{ type.name }}</h3>
              <p class="body-medium on-surface-variant">{{ type.count }} document{{ type.count > 1 ? 's' : '' }}</p>
            </div>
          </div>
          
          <div class="type-preview">
            <div
              v-for="doc in type.recentDocs.slice(0, 3)"
              :key="doc.id"
              class="preview-item body-small on-surface-variant"
            >
              📄 {{ doc.new_name }}
            </div>
          </div>

          <div class="type-footer">
            <button class="button button-text">
              <span class="label-large">Voir tout</span>
            </button>
          </div>
        </div>

        <EmptyState
          v-if="documentTypes.length === 0"
          message="Aucun document importé"
          icon="📂"
        />
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from '#app'
import type { Document } from '~/types/document'

const router = useRouter()
const { activeFilters } = useDocumentFilters()

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

// Group documents by type with stats
const documentTypes = computed(() => {
  const typeMap = new Map<string, Document[]>()
  
  documents.value.forEach(doc => {
    const type = doc.type || 'Non classifié'
    if (!typeMap.has(type)) {
      typeMap.set(type, [])
    }
    typeMap.get(type)!.push(doc)
  })

  const typeIcons: Record<string, string> = {
    'Facture': '🧾',
    'Contrat': '📜',
    'Relevé': '📊',
    'Bulletin': '📋',
    'Officiel': '🏛️',
    'Non classifié': '📄'
  }

  return Array.from(typeMap.entries())
    .map(([name, docs]) => ({
      name,
      icon: typeIcons[name] || '📄',
      count: docs.length,
      recentDocs: docs.sort((a, b) => 
        new Date(b.date).getTime() - new Date(a.date).getTime()
      ).slice(0, 5)
    }))
    .sort((a, b) => b.count - a.count)
})

function selectType(typeName: string) {
  activeFilters.value.type = typeName
  router.push('/')
}
</script>

<style scoped>
.types-page {
  display: flex;
  flex-direction: column;
  gap: 32px;
  padding: 24px;
}

.types-page h1 {
  margin: 0;
}

.type-categories {
  display: grid;
  gap: 24px;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
}

.type-card {
  display: flex;
  flex-direction: column;
  padding: 24px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid transparent;
}

.type-card:hover {
  transform: translateY(-4px);
  border-color: var(--md-sys-color-primary);
  box-shadow: 
    0 4px 8px rgba(0, 0, 0, 0.1),
    0 1px 3px rgba(0, 0, 0, 0.08);
}

.type-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
}

.type-icon {
  font-size: 48px;
  line-height: 1;
}

.type-info h3 {
  margin: 0 0 4px 0;
}

.type-info p {
  margin: 0;
}

.type-preview {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 16px;
  min-height: 80px;
}

.preview-item {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.type-footer {
  display: flex;
  justify-content: flex-end;
  padding-top: 12px;
  border-top: 1px solid var(--md-sys-color-outline-variant);
}

/* Mobile: Single column */
@media (max-width: 767px) {
  .types-page {
    padding: 16px;
    gap: 24px;
  }

  .type-categories {
    grid-template-columns: 1fr;
    gap: 16px;
  }

  .type-card {
    padding: 16px;
  }
}
</style>
