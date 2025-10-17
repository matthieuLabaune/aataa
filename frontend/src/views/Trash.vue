<template>
  <div class="trash">
    <!-- Top App Bar -->
    <header class="md-top-app-bar">
      <div class="app-bar-content">
        <button @click="goBack" class="md-icon-button">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M19 12H5M12 19l-7-7 7-7" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
        <div class="app-bar-title">
          <h1 class="headline-medium">Corbeille</h1>
          <span class="body-small app-subtitle">{{ deletedDocuments.length }} documents</span>
        </div>
        <button 
          v-if="deletedDocuments.length > 0" 
          @click="showEmptyTrashConfirm = true" 
          class="md-outlined-button md-ripple"
        >
          <svg width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor">
            <polyline points="3 6 5 6 21 6" stroke-width="2" stroke-linecap="round"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" stroke-width="2" stroke-linecap="round"/>
          </svg>
          Vider la corbeille
        </button>
      </div>
    </header>

    <!-- Main Content -->
    <div class="trash-content">
      <!-- Loading State -->
      <div v-if="loading" class="loading-state">
        <div class="md-circular-progress"></div>
        <p class="body-large">Chargement...</p>
      </div>

      <!-- Empty State -->
      <div v-else-if="deletedDocuments.length === 0" class="empty-state animate-fade-in">
        <svg width="120" height="120" viewBox="0 0 120 120" fill="none">
          <circle cx="60" cy="60" r="50" :fill="`color-mix(in srgb, var(--md-sys-color-primary) 12%, transparent)`"/>
          <path d="M40 40 L60 60 M60 60 L80 40 M40 80 L60 60 M60 60 L80 80" stroke="var(--md-sys-color-on-surface-variant)" stroke-width="3" stroke-linecap="round"/>
          <circle cx="60" cy="60" r="35" stroke="var(--md-sys-color-on-surface-variant)" stroke-width="3" fill="none"/>
        </svg>
        <h3 class="title-medium">Corbeille vide</h3>
        <p class="body-medium">Les documents supprimés apparaîtront ici</p>
        <button @click="goBack" class="md-filled-button md-ripple">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
            <path d="M15 8H1M8 15L1 8l7-7" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          Retour à l'accueil
        </button>
      </div>

      <!-- Documents List -->
      <div v-else class="documents-list">
        <div 
          v-for="(doc, index) in deletedDocuments" 
          :key="doc.id"
          class="md-list-item document-item animate-slide-in-up"
          :style="{ animationDelay: `${index * 30}ms` }"
        >
          <!-- Document Icon -->
          <div class="doc-icon">
            <svg width="40" height="40" viewBox="0 0 40 40" fill="none">
              <rect width="40" height="40" rx="8" :fill="`color-mix(in srgb, var(--md-sys-color-error) 12%, transparent)`"/>
              <path d="M13 15h14M13 20h14M13 25h9" stroke="var(--md-sys-color-error)" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </div>

          <!-- Document Info -->
          <div class="doc-info">
            <h3 class="title-medium doc-name">{{ doc.new_name }}</h3>
            <div class="doc-meta body-small">
              <span class="meta-item">
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor">
                  <path d="M13 5v8a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h6M10 1v6h6" stroke-width="1.5" stroke-linecap="round"/>
                </svg>
                {{ doc.document_type }}
              </span>
              <span class="meta-item">
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor">
                  <path d="M5 1H3a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V9" stroke-width="1.5" stroke-linecap="round"/>
                </svg>
                {{ formatSize(doc.file_size) }}
              </span>
              <span class="meta-item">
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor">
                  <circle cx="7" cy="7" r="6" stroke-width="1.5"/>
                  <path d="M7 3v4l3 2" stroke-width="1.5" stroke-linecap="round"/>
                </svg>
                Supprimé {{ formatDeletedDate(doc.deleted_at!) }}
              </span>
            </div>
          </div>

          <!-- Actions -->
          <div class="doc-actions">
            <button @click="restoreDocument(doc.id)" class="md-filled-button md-ripple action-btn">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
                <path d="M1 4v4h4M15 12v-4h-4" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M3.51 9A6 6 0 1 0 2 6.5" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              Restaurer
            </button>
            <button @click="confirmDelete(doc)" class="md-outlined-button md-ripple action-btn error-btn">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
                <path d="M12 4L4 12M4 4l8 8" stroke-width="2" stroke-linecap="round"/>
              </svg>
              Supprimer définitivement
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Confirmation Modal: Delete One Document -->
    <div v-if="documentToDelete" class="md-modal-overlay" @click="documentToDelete = null">
      <div class="md-modal confirm-modal animate-scale-in" @click.stop>
        <div class="modal-icon error-icon">
          <svg width="48" height="48" viewBox="0 0 48 48" fill="none" stroke="currentColor">
            <circle cx="24" cy="24" r="22" stroke-width="3"/>
            <path d="M24 12v12M24 30v.01" stroke-width="3" stroke-linecap="round"/>
          </svg>
        </div>
        
        <div class="md-modal-content">
          <h2 class="title-large">Supprimer définitivement ?</h2>
          <p class="body-large modal-text">
            Êtes-vous sûr de vouloir supprimer définitivement 
            <strong>{{ documentToDelete.new_name }}</strong> ?
            Cette action est irréversible.
          </p>
        </div>

        <div class="md-modal-footer">
          <button @click="documentToDelete = null" class="md-text-button">
            Annuler
          </button>
          <button @click="permanentlyDelete(documentToDelete.id)" class="md-filled-button md-ripple error-button">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
              <polyline points="3 4 5 4 17 4" stroke-width="2" stroke-linecap="round"/>
              <path d="M15 4v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4" stroke-width="2" stroke-linecap="round"/>
            </svg>
            Supprimer définitivement
          </button>
        </div>
      </div>
    </div>

    <!-- Confirmation Modal: Empty Trash -->
    <div v-if="showEmptyTrashConfirm" class="md-modal-overlay" @click="showEmptyTrashConfirm = false">
      <div class="md-modal confirm-modal animate-scale-in" @click.stop>
        <div class="modal-icon error-icon">
          <svg width="48" height="48" viewBox="0 0 48 48" fill="none" stroke="currentColor">
            <circle cx="24" cy="24" r="22" stroke-width="3"/>
            <path d="M24 12v12M24 30v.01" stroke-width="3" stroke-linecap="round"/>
          </svg>
        </div>
        
        <div class="md-modal-content">
          <h2 class="title-large">Vider la corbeille ?</h2>
          <p class="body-large modal-text">
            Êtes-vous sûr de vouloir supprimer définitivement 
            <strong>{{ deletedDocuments.length }} documents</strong> ?
            Cette action est irréversible.
          </p>
        </div>

        <div class="md-modal-footer">
          <button @click="showEmptyTrashConfirm = false" class="md-text-button">
            Annuler
          </button>
          <button @click="emptyTrash" class="md-filled-button md-ripple error-button">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor">
              <polyline points="3 6 5 6 21 6" stroke-width="2" stroke-linecap="round"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" stroke-width="2" stroke-linecap="round"/>
            </svg>
            Vider la corbeille
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import type { Document } from '../types/document'

const router = useRouter()
const deletedDocuments = ref<Document[]>([])
const loading = ref(true)
const documentToDelete = ref<Document | null>(null)
const showEmptyTrashConfirm = ref(false)

// Load deleted documents
async function loadDeletedDocuments() {
  try {
    loading.value = true
    const docs = await invoke<Document[]>('get_deleted_documents')
    deletedDocuments.value = docs
  } catch (error) {
    console.error('Failed to load deleted documents:', error)
    alert(`Erreur lors du chargement de la corbeille: ${error}`)
  } finally {
    loading.value = false
  }
}

// Restore document
async function restoreDocument(id: string) {
  try {
    await invoke('restore_document', { id })
    await loadDeletedDocuments()
  } catch (error) {
    console.error('Failed to restore document:', error)
    alert(`Erreur lors de la restauration: ${error}`)
  }
}

// Confirm delete
function confirmDelete(doc: Document) {
  documentToDelete.value = doc
}

// Permanently delete document
async function permanentlyDelete(id: string) {
  try {
    await invoke('permanently_delete_document', { id })
    await loadDeletedDocuments()
    documentToDelete.value = null
  } catch (error) {
    console.error('Failed to permanently delete document:', error)
    alert(`Erreur lors de la suppression définitive: ${error}`)
  }
}

// Empty trash
async function emptyTrash() {
  try {
    // Delete all documents one by one
    for (const doc of deletedDocuments.value) {
      await invoke('permanently_delete_document', { id: doc.id })
    }
    await loadDeletedDocuments()
    showEmptyTrashConfirm.value = false
  } catch (error) {
    console.error('Failed to empty trash:', error)
    alert(`Erreur lors du vidage de la corbeille: ${error}`)
  }
}

// Go back to home
function goBack() {
  router.push('/')
}

// Format file size
function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' o'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' Ko'
  return (bytes / (1024 * 1024)).toFixed(1) + ' Mo'
}

// Format deleted date
function formatDeletedDate(dateStr: string): string {
  const date = new Date(dateStr)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMs / 3600000)
  const diffDays = Math.floor(diffMs / 86400000)

  if (diffMins < 1) return "à l'instant"
  if (diffMins < 60) return `il y a ${diffMins} min`
  if (diffHours < 24) return `il y a ${diffHours}h`
  if (diffDays < 7) return `il y a ${diffDays} jour${diffDays > 1 ? 's' : ''}`
  
  return date.toLocaleDateString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}

onMounted(() => {
  loadDeletedDocuments()
})
</script>

<style scoped>
.trash {
  min-height: 100vh;
  background-color: var(--md-sys-color-background);
}

/* Top App Bar */
.md-top-app-bar {
  position: sticky;
  top: 0;
  z-index: var(--md-sys-z-index-app-bar);
  background-color: var(--md-sys-color-surface);
  border-bottom: 1px solid var(--md-sys-color-outline-variant);
  padding: var(--md-sys-spacing-md) var(--md-sys-spacing-lg);
}

.app-bar-content {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-md);
}

.app-bar-title {
  flex: 1;
}

.app-bar-title h1 {
  color: var(--md-sys-color-on-surface);
  margin: 0;
}

.app-subtitle {
  color: var(--md-sys-color-on-surface-variant);
}

/* Main Content */
.trash-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: var(--md-sys-spacing-xl) var(--md-sys-spacing-lg);
}

/* Loading State */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: var(--md-sys-spacing-3xl);
  gap: var(--md-sys-spacing-lg);
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: var(--md-sys-spacing-3xl) var(--md-sys-spacing-lg);
  color: var(--md-sys-color-on-surface-variant);
}

.empty-state svg {
  margin-bottom: var(--md-sys-spacing-lg);
}

.empty-state h3 {
  margin-bottom: var(--md-sys-spacing-sm);
  color: var(--md-sys-color-on-surface);
}

.empty-state p {
  margin-bottom: var(--md-sys-spacing-xl);
}

/* Documents List */
.documents-list {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-md);
}

.document-item {
  background-color: var(--md-sys-color-surface-container-low);
  border-radius: var(--md-sys-shape-corner-medium);
  padding: var(--md-sys-spacing-lg);
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-md);
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
  border: 1px solid var(--md-sys-color-outline-variant);
}

.document-item:hover {
  box-shadow: var(--md-sys-elevation-level1);
  background-color: var(--md-sys-color-surface-container);
}

.doc-icon {
  flex-shrink: 0;
}

.doc-info {
  flex: 1;
  min-width: 0;
}

.doc-name {
  margin: 0 0 var(--md-sys-spacing-xs) 0;
  color: var(--md-sys-color-on-surface);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.doc-meta {
  display: flex;
  gap: var(--md-sys-spacing-md);
  color: var(--md-sys-color-on-surface-variant);
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-xs);
}

.doc-actions {
  display: flex;
  gap: var(--md-sys-spacing-sm);
  flex-shrink: 0;
}

.action-btn {
  white-space: nowrap;
}

.error-btn {
  color: var(--md-sys-color-error);
  border-color: var(--md-sys-color-error);
}

.error-btn:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-error) 8%, transparent);
}

/* Confirmation Modal */
.confirm-modal {
  max-width: 500px;
  text-align: center;
}

.modal-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto var(--md-sys-spacing-lg);
}

.error-icon {
  color: var(--md-sys-color-error);
}

.modal-text {
  color: var(--md-sys-color-on-surface-variant);
  text-align: center;
}

.error-button {
  background-color: var(--md-sys-color-error);
  color: var(--md-sys-color-on-error);
}

.error-button:hover {
  background-color: color-mix(in srgb, var(--md-sys-color-error) 92%, white);
}

/* Responsive */
@media (max-width: 768px) {
  .trash-content {
    padding: var(--md-sys-spacing-lg) var(--md-sys-spacing-md);
  }

  .document-item {
    flex-direction: column;
    align-items: flex-start;
  }

  .doc-actions {
    width: 100%;
    flex-direction: column;
  }

  .action-btn {
    width: 100%;
  }

  .app-bar-content {
    flex-wrap: wrap;
  }
}
</style>
