<template>
  <div style="min-height: 100vh; background: #f3f4f6">
    <!-- Message de statut flottant -->
    <div
      v-if="showStatus"
      style="position: fixed; top: 20px; right: 20px; z-index: 100; background: white; padding: 1rem 1.5rem; border-radius: 0.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.1); max-width: 400px"
    >
      <p style="margin: 0; white-space: pre-line">{{ statusMessage }}</p>
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
        <div style="position: relative">
          <input
            v-model="searchQuery"
            type="text"
            class="input"
            placeholder="🔍 Rechercher dans les documents..."
            @input="handleSearch"
            style="padding-left: 2.5rem"
          />
        </div>
      </div>

      <!-- Documents List -->
      <div class="card">
        <div class="flex items-center justify-between mb-6">
          <h2 style="font-size: 1.25rem; font-weight: 600; margin: 0">
            Documents archivés ({{ documents.length }})
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

        <div v-if="documents.length === 0">
          <EmptyState
            icon="📄"
            title="Aucun document archivé"
            description="Commencez par importer votre premier document"
          />
        </div>

        <div v-else class="grid">
          <DocumentCard
            v-for="doc in documents"
            :key="doc.id"
            :document="doc"
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open as openDialog } from '@tauri-apps/plugin-dialog'

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
      multiple: false,
      filters: [{
        name: 'Documents',
        extensions: ['pdf', 'png', 'jpg', 'jpeg']
      }]
    })

    console.log('File selected:', selected)
    
    if (selected && typeof selected === 'string') {
      await processFile(selected)
    } else if (!selected) {
      console.log('No file selected')
    }
  } catch (error) {
    console.error('Error selecting file:', error)
    showMessage('❌ Erreur sélection: ' + error, 5000)
  }
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
  
  const file = files[0]
  console.log('File dropped:', file.name, file.type)
  
  // Vérifier le type de fichier
  const validTypes = ['application/pdf', 'image/png', 'image/jpeg', 'image/jpg']
  if (!validTypes.includes(file.type) && !file.name.match(/\.(pdf|png|jpe?g)$/i)) {
    showMessage('⚠️ Type de fichier non supporté. Utilisez PDF, PNG ou JPG.', 4000)
    return
  }
  
  // Pour Tauri, nous devons enregistrer le fichier temporairement
  // car nous avons besoin du chemin du fichier système, pas du blob
  try {
    showMessage('⏳ Préparation du fichier...', 3000)
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
    
    console.log('Temp file written:', tempPath)
    
    // Traiter le fichier
    await processFile(tempPath)
  } catch (error) {
    console.error('Error handling drop:', error)
    showMessage('❌ Erreur glisser-déposer: ' + error, 5000)
  }
}

async function handleSearch() {
  if (searchQuery.value.trim()) {
    try {
      documents.value = await invoke<Document[]>('search_documents', {
        query: searchQuery.value
      })
    } catch (error) {
      console.error('Error searching:', error)
    }
  } else {
    loadDocuments()
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
    alert('Erreur lors de la sélection du dossier: ' + error)
  }
}
</script>
