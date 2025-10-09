<template>
  <div style="min-height: 100vh; background: #f3f4f6">
    <!-- Message de statut flottant -->
    <div
      v-if="showStatus"
      style="position: fixed; top: 20px; right: 20px; z-index: 100; background: white; padding: 1rem 1.5rem; border-radius: 0.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.1); max-width: 400px"
    >
      <p style="margin: 0; white-space: pre-line">{{ statusMessage }}</p>
    </div>

    <!-- Barre de progression multi-fichiers -->
    <div
      v-if="processingMultiple"
      style="position: fixed; bottom: 20px; right: 20px; z-index: 100; background: white; padding: 1rem 1.5rem; border-radius: 0.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.1); min-width: 300px"
    >
      <p style="margin: 0 0 0.5rem 0; font-weight: 600">
        Traitement en cours...
      </p>
      <div style="background: #e5e7eb; height: 8px; border-radius: 4px; overflow: hidden; margin-bottom: 0.5rem">
        <div
          style="background: #3b82f6; height: 100%; transition: width 0.3s"
          :style="{ width: `${(processedFiles + failedFiles) / totalFiles * 100}%` }"
        ></div>
      </div>
      <p style="margin: 0; font-size: 0.875rem; color: #6b7280">
        {{ processedFiles + failedFiles }} / {{ totalFiles }} fichiers
        <span v-if="failedFiles > 0" style="color: #ef4444">• {{ failedFiles }} échoué(s)</span>
      </p>
    </div>

    <div class="container" style="padding-top: 2rem; padding-bottom: 2rem">
      <!-- Header -->
      <div class="flex items-center justify-between mb-6">
        <div>
          <h1 style="font-size: 1.875rem; font-weight: bold; margin: 0">
            AATAA - An App To Archive All
          </h1>
          <p class="text-gray-500" style="margin-top: 0.25rem">
            Archivage automatique de documents avec OCR
          </p>
        </div>
        <button
          class="btn btn-secondary"
          @click="showSettings = true"
          style="padding: 0.5rem; width: 40px; height: 40px; font-size: 1.25rem"
          title="Paramètres"
        >
          ⚙️
        </button>
      </div>

      <!-- Import Section -->
      <div class="card mb-6">
        <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem">
          Importer un document
        </h2>

        <div
          class="upload-area"
          @click="selectFile"
          @dragover.prevent
          @drop.prevent="handleDrop"
        >
          <div style="font-size: 3rem; margin-bottom: 1rem">📄</div>
          <p class="text-gray-600">
            Cliquez pour sélectionner ou glissez-déposez un fichier
          </p>
          <p class="text-sm text-gray-500" style="margin-top: 0.5rem">
            PDF, PNG, JPG, JPEG supportés
          </p>
        </div>

        <div v-if="processing" style="text-align: center; margin-top: 1rem">
          <div style="font-size: 1.5rem; margin-bottom: 0.5rem">⏳</div>
          <p class="text-gray-600">
            Traitement en cours... (OCR + Classification)
          </p>
        </div>
      </div>

      <!-- Search -->
      <div class="card mb-6">
        <div style="position: relative; margin-bottom: 1rem">
          <input
            v-model="searchQuery"
            type="text"
            class="input"
            placeholder="🔍 Rechercher dans les documents..."
            style="padding-left: 2.5rem"
          />
        </div>

        <!-- Filters -->
        <div class="flex gap-2" style="flex-wrap: wrap">
          <div style="flex: 1; min-width: 200px">
            <label style="display: block; font-size: 0.75rem; font-weight: 500; margin-bottom: 0.25rem; color: #6b7280">Type de document</label>
            <select v-model="filterType" class="input" style="padding: 0.5rem">
              <option value="all">📂 Tous les types</option>
              <option v-for="type in documentTypes.filter(t => t !== 'all')" :key="type" :value="type">
                {{ type }}
              </option>
            </select>
          </div>

          <div style="flex: 1; min-width: 200px">
            <label style="display: block; font-size: 0.75rem; font-weight: 500; margin-bottom: 0.25rem; color: #6b7280">Tag</label>
            <select v-model="filterTag" class="input" style="padding: 0.5rem">
              <option value="all">🏷️ Tous les tags</option>
              <option v-for="tag in allTags.filter(t => t !== 'all')" :key="tag" :value="tag">
                {{ tag }}
              </option>
            </select>
          </div>

          <div style="display: flex; align-items: flex-end">
            <button
              @click="filterType = 'all'; filterTag = 'all'; searchQuery = ''"
              class="btn btn-secondary"
              style="padding: 0.5rem 1rem"
            >
              🔄 Réinitialiser
            </button>
          </div>
        </div>
      </div>

      <!-- Documents List -->
      <div class="card">
        <div class="flex items-center justify-between mb-6">
          <h2 style="font-size: 1.25rem; font-weight: 600; margin: 0">
            Documents archivés ({{ filteredDocuments.length }}{{ filteredDocuments.length !== documents.length ? ` / ${documents.length}` : '' }})
          </h2>
          <button
            class="btn btn-secondary"
            @click="loadDocuments"
            style="padding: 0.5rem; width: 36px; height: 36px"
            title="Actualiser"
          >
            🔄
          </button>
        </div>

        <div v-if="filteredDocuments.length === 0">
          <EmptyState
            v-if="documents.length === 0"
            icon="📄"
            title="Aucun document archivé"
            description="Commencez par importer votre premier document"
          />
          <EmptyState
            v-else
            icon="🔍"
            title="Aucun résultat"
            description="Aucun document ne correspond à vos filtres"
          />
        </div>

        <div v-else class="grid">
          <DocumentCard
            v-for="doc in filteredDocuments"
            :key="doc.id"
            :document="doc"
            @preview="openPreview(doc)"
            @open="openDocument(doc.file_path)"
            @delete="deleteDoc(doc.id)"
          />
        </div>
      </div>
    </div>

    <!-- Settings Modal -->
    <div
      v-if="showSettings"
      style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 50"
      @click.self="showSettings = false"
    >
      <div class="card" style="max-width: 500px; width: 90%; margin: 1rem">
        <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem">
          Paramètres
        </h2>

        <div style="margin-bottom: 1rem">
          <label style="display: block; font-size: 0.875rem; font-weight: 500; margin-bottom: 0.5rem">
            Dossier d'archivage
          </label>
          <div class="flex gap-2">
            <input
              v-model="archivePath"
              type="text"
              class="input"
              readonly
              style="flex: 1"
            />
            <button class="btn btn-primary" @click="selectArchivePath">
              Choisir
            </button>
          </div>
        </div>

        <div style="margin-bottom: 1rem">
          <label style="display: flex; align-items: center; gap: 0.5rem; cursor: pointer">
            <input
              type="checkbox"
              checked
              disabled
              style="width: 18px; height: 18px"
            />
            <span style="font-size: 0.875rem; font-weight: 500">
              Organisation automatique par dossiers
            </span>
          </label>
          <p style="font-size: 0.75rem; color: #6b7280; margin: 0.25rem 0 0 1.75rem">
            Les documents sont automatiquement classés dans des dossiers par type (Factures, Contrats, etc.)
          </p>
        </div>

        <div style="background: #dbeafe; border: 1px solid #93c5fd; border-radius: 0.375rem; padding: 0.75rem; margin-bottom: 1rem">
          <p style="font-size: 0.875rem; color: #1e40af; margin: 0">
            ℹ️ Tous les documents sont stockés localement et hors ligne.
          </p>
        </div>

        <button class="btn btn-primary" style="width: 100%" @click="showSettings = false">
          Fermer
        </button>
      </div>
    </div>

    <!-- Preview Modal -->
    <div
      v-if="showPreview && previewDocument"
      style="position: fixed; inset: 0; background: rgba(0, 0, 0, 0.8); display: flex; align-items: center; justify-content: center; z-index: 100"
      @click.self="closePreview"
    >
      <div class="card" style="max-width: 90vw; max-height: 90vh; width: 800px; overflow: auto; position: relative">
        <button
          @click="closePreview"
          style="position: absolute; top: 1rem; right: 1rem; background: #ef4444; color: white; border: none; border-radius: 50%; width: 32px; height: 32px; cursor: pointer; font-size: 1.25rem"
          title="Fermer"
        >
          ✕
        </button>

        <h2 style="font-size: 1.5rem; font-weight: 600; margin-bottom: 1rem; padding-right: 3rem">
          {{ previewDocument.new_name }}
        </h2>

        <div style="display: grid; gap: 1rem">
          <!-- Metadata -->
          <div style="background: #f3f4f6; padding: 1rem; border-radius: 0.5rem">
            <div class="flex gap-2" style="flex-wrap: wrap; margin-bottom: 0.5rem">
              <span :class="['badge', `badge-${typeColors[previewDocument.document_type] || 'gray'}`]">
                {{ previewDocument.document_type }}
              </span>
              <span
                v-for="tag in previewDocument.tags"
                :key="tag"
                class="badge badge-gray"
              >
                {{ tag }}
              </span>
            </div>
            <p class="text-sm text-gray-600" style="margin: 0.5rem 0">
              <strong>Fichier original:</strong> {{ previewDocument.original_name }}
            </p>
            <p class="text-sm text-gray-600" style="margin: 0.5rem 0">
              <strong>Date:</strong> {{ new Date(previewDocument.created_at).toLocaleString('fr-FR') }}
            </p>
            <p class="text-sm text-gray-600" style="margin: 0.5rem 0">
              <strong>Taille:</strong> {{ formatFileSize(previewDocument.file_size) }}
            </p>
          </div>

          <!-- OCR Text -->
          <div v-if="previewDocument.ocr_text">
            <h3 style="font-weight: 600; margin-bottom: 0.5rem">Texte extrait (OCR)</h3>
            <div style="background: white; border: 1px solid #d1d5db; border-radius: 0.5rem; padding: 1rem; max-height: 400px; overflow-y: auto; white-space: pre-wrap; font-family: monospace; font-size: 0.875rem">{{ previewDocument.ocr_text }}</div>
          </div>

          <!-- Actions -->
          <div class="flex gap-2">
            <button class="btn btn-primary" @click="openDocument(previewDocument.file_path)">
              📂 Ouvrir dans le système
            </button>
            <button class="btn btn-danger" @click="deleteDoc(previewDocument.id); closePreview()">
              🗑 Supprimer
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import DocumentCard from '../components/DocumentCard.vue'
import EmptyState from '../components/EmptyState.vue'

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

const documents = ref<Document[]>([])
const searchQuery = ref('')
const processing = ref(false)
const showSettings = ref(false)
const archivePath = ref('')
const statusMessage = ref('')
const showStatus = ref(false)
const previewDocument = ref<Document | null>(null)
const showPreview = ref(false)
const filterType = ref<string>('all')
const filterTag = ref<string>('all')

// Multi-file processing state
const processingMultiple = ref(false)
const totalFiles = ref(0)
const processedFiles = ref(0)
const failedFiles = ref(0)

onMounted(() => {
  loadDocuments()
  loadArchivePath()
})

function showMessage(msg: string, duration = 3000) {
  statusMessage.value = msg
  showStatus.value = true
  setTimeout(() => {
    showStatus.value = false
  }, duration)
}

async function loadDocuments() {
  try {
    console.log('Loading documents...')
    documents.value = await invoke<Document[]>('get_documents')
    console.log('Documents loaded:', documents.value.length)
    if (documents.value.length > 0) {
      console.log('First document sample:', documents.value[0])
    }
  } catch (error) {
    console.error('Error loading documents:', error)
    showMessage('❌ Erreur chargement documents: ' + error, 5000)
  }
}

async function loadArchivePath() {
  try {
    console.log('Loading archive path...')
    archivePath.value = await invoke<string>('get_archive_path')
    console.log('Archive path loaded:', archivePath.value)
  } catch (error) {
    console.error('Error loading archive path:', error)
  }
}

async function selectFile() {
  console.log('selectFile called!')

  try {
    const selected = await openDialog({
      multiple: true,
      filters: [{
        name: 'Documents',
        extensions: ['pdf', 'png', 'jpg', 'jpeg']
      }]
    })

    console.log('File(s) selected:', selected)

    if (!selected) {
      console.log('No file selected')
      return
    }

    // Handle single or multiple files
    const files = Array.isArray(selected) ? selected : [selected]

    if (files.length === 0) {
      console.log('No files to process')
      return
    }

    if (files.length > 100) {
      showMessage('❌ Maximum 100 fichiers à la fois', 5000)
      return
    }

    // Process multiple files
    await processMultipleFiles(files)

  } catch (error) {
    console.error('Error selecting file:', error)
    showMessage('❌ Erreur sélection: ' + error, 5000)
  }
}

async function processMultipleFiles(files: string[]) {
  processingMultiple.value = true
  totalFiles.value = files.length
  processedFiles.value = 0
  failedFiles.value = 0

  showMessage(`⏳ Traitement de ${files.length} fichier(s)...`, 30000)

  // Process files in batches of 5 to avoid overwhelming the system
  const batchSize = 5
  const results: Document[] = []

  for (let i = 0; i < files.length; i += batchSize) {
    const batch = files.slice(i, i + batchSize)

    const batchResults = await Promise.allSettled(
      batch.map(async (filePath) => {
        try {
          const doc = await invoke<Document>('process_file', { filePath })
          processedFiles.value++
          return doc
        } catch (error) {
          console.error('Error processing file:', filePath, error)
          failedFiles.value++
          throw error
        }
      })
    )

    // Collect successful results
    batchResults.forEach(result => {
      if (result.status === 'fulfilled') {
        results.push(result.value)
      }
    })

    // Update progress message
    showMessage(
      `⏳ Traitement: ${processedFiles.value}/${totalFiles.value} réussis, ${failedFiles.value} échoués`,
      30000
    )
  }

  // Add all processed documents to the list
  documents.value.unshift(...results)

  processingMultiple.value = false

  const summary = `✅ Terminé!\n${processedFiles.value} fichier(s) archivé(s)${failedFiles.value > 0 ? `\n❌ ${failedFiles.value} échec(s)` : ''}`
  showMessage(summary, 5000)
}

async function processFile(filePath: string) {
  processing.value = true
  console.log('Processing file:', filePath)
  showMessage('⏳ Traitement en cours...', 10000)

  try {
    const doc = await invoke<Document>('process_file', { filePath })
    console.log('Document processed:', doc)
    documents.value.unshift(doc)
    showMessage(`✅ Document archivé!\n${doc.document_type}: ${doc.new_name}`, 5000)
  } catch (error) {
    console.error('Error processing file:', error)
    showMessage('❌ Erreur traitement: ' + error, 5000)
  } finally {
    processing.value = false
  }
}

async function handleDrop(event: DragEvent) {
  event.preventDefault()
  console.log('Drop event triggered!')

  const files = event.dataTransfer?.files
  if (!files || files.length === 0) {
    console.log('No files dropped')
    return
  }

  console.log(`${files.length} file(s) dropped`)

  if (files.length > 100) {
    showMessage('❌ Maximum 100 fichiers à la fois', 5000)
    return
  }

  // Vérifier les types de fichiers
  const validTypes = ['application/pdf', 'image/png', 'image/jpeg', 'image/jpg']
  const invalidFiles = Array.from(files).filter(file =>
    !validTypes.includes(file.type) && !file.name.match(/\.(pdf|png|jpe?g)$/i)
  )

  if (invalidFiles.length > 0) {
    showMessage(`⚠️ ${invalidFiles.length} fichier(s) non supporté(s) ignoré(s). Utilisez PDF, PNG ou JPG.`, 4000)
  }

  const validFiles = Array.from(files).filter(file =>
    validTypes.includes(file.type) || file.name.match(/\.(pdf|png|jpe?g)$/i)
  )

  if (validFiles.length === 0) {
    return
  }

  // Pour Tauri, nous devons enregistrer les fichiers temporairement
  try {
    showMessage(`⏳ Préparation de ${validFiles.length} fichier(s)...`, 10000)

    const tempPaths: string[] = []

    for (const file of validFiles) {
      // Lire le fichier comme ArrayBuffer
      const arrayBuffer = await file.arrayBuffer()
      const uint8Array = new Uint8Array(arrayBuffer)

      // Créer un chemin temporaire
      const tempFileName = `aataa_temp_${Date.now()}_${file.name}`
      const tempPath = `/tmp/${tempFileName}`

      // Écrire le fichier via Tauri FS
      await invoke('write_temp_file', {
        path: tempPath,
        content: Array.from(uint8Array)
      })

      tempPaths.push(tempPath)
    }

    // Traiter tous les fichiers
    await processMultipleFiles(tempPaths)
    
  } catch (error) {
    console.error('Error handling drop:', error)
    showMessage('❌ Erreur glisser-déposer: ' + error, 5000)
  }
}

async function deleteDoc(id: string) {
  if (confirm('Êtes-vous sûr de vouloir supprimer ce document ?')) {
    try {
      await invoke('delete_document', { id })
      documents.value = documents.value.filter(d => d.id !== id)
    } catch (error) {
      console.error('Error deleting document:', error)
    }
  }
}

async function openDocument(filePath: string) {
  try {
    await invoke('open_file', { filePath })
  } catch (error) {
    console.error('Error opening file:', error)
  }
}

function openPreview(doc: Document) {
  previewDocument.value = doc
  showPreview.value = true
  console.log('Preview opened for:', doc.new_name)
  console.log('OCR text:', doc.ocr_text)
  console.log('Full document:', doc)
}

function closePreview() {
  showPreview.value = false
  previewDocument.value = null
}

const typeColors: Record<string, string> = {
  'Facture': 'blue',
  'Contrat': 'blue',
  'Relevé bancaire': 'green',
  'Bulletin de paie': 'yellow',
  'Document officiel': 'blue',
  'Reçu': 'yellow',
  'Unknown': 'gray'
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

// Computed pour les documents filtrés
const filteredDocuments = computed(() => {
  let result = documents.value

  // Filtre par type
  if (filterType.value !== 'all') {
    result = result.filter(doc => doc.document_type === filterType.value)
  }

  // Filtre par tag
  if (filterTag.value !== 'all') {
    result = result.filter(doc => doc.tags.includes(filterTag.value))
  }

  // Filtre par recherche
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(doc =>
      doc.new_name.toLowerCase().includes(query) ||
      doc.original_name.toLowerCase().includes(query) ||
      doc.ocr_text.toLowerCase().includes(query) ||
      doc.tags.some(tag => tag.toLowerCase().includes(query))
    )
  }

  return result
})

// Listes uniques pour les filtres
const documentTypes = computed(() => {
  const types = new Set(documents.value.map(doc => doc.document_type))
  return ['all', ...Array.from(types)]
})

const allTags = computed(() => {
  const tags = new Set(documents.value.flatMap(doc => doc.tags))
  return ['all', ...Array.from(tags)]
})

async function selectArchivePath() {
  try {
    const selected = await openDialog({
      directory: true,
      multiple: false
    })

    console.log('Directory selected:', selected)

    if (selected && typeof selected === 'string') {
      await invoke('set_archive_path', { path: selected })
      archivePath.value = selected
    } else if (!selected) {
      console.log('No directory selected')
    }
  } catch (error) {
    console.error('Error selecting archive path:', error)
    showMessage('❌ Erreur sélection dossier: ' + error, 5000)
  }
}
</script>
