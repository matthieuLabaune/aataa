<template>
  <div style="min-height: 100vh;">
    <!-- Indicateur de traitement -->
    <ProcessingIndicator />

    <!-- Message de statut flottant -->
    <div
      v-if="showStatus"
      style="position: fixed; top: 20px; right: 20px; z-index: 100; background: white; padding: 1rem 1.5rem; border-radius: 0.75rem; box-shadow: 0 10px 25px rgba(0,0,0,0.15); max-width: 400px; border: 1px solid #e5e7eb; animation: slideInRight 0.3s ease;"
    >
      <p style="margin: 0; white-space: pre-line; font-weight: 500">{{ statusMessage }}</p>
    </div>

    <!-- Barre de progression multi-fichiers -->
    <div
      v-if="processingMultiple"
      style="position: fixed; bottom: 20px; right: 20px; z-index: 100; background: white; padding: 1.25rem 1.5rem; border-radius: 0.75rem; box-shadow: 0 10px 25px rgba(0,0,0,0.15); min-width: 320px; border: 1px solid #e5e7eb; animation: slideInRight 0.3s ease;"
    >
      <p style="margin: 0 0 0.75rem 0; font-weight: 700; color: #1f2937; font-size: 0.95rem">
        ⏳ Traitement en cours...
      </p>
      <div style="background: linear-gradient(135deg, #e5e7eb 0%, #f3f4f6 100%); height: 10px; border-radius: 9999px; overflow: hidden; margin-bottom: 0.75rem; box-shadow: inset 0 2px 4px rgba(0,0,0,0.06)">
        <div
          style="background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%); height: 100%; transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1); box-shadow: 0 0 10px rgba(59, 130, 246, 0.5);"
          :style="{ width: `${(processedFiles + failedFiles) / totalFiles * 100}%` }"
        ></div>
      </div>
      <p style="margin: 0; font-size: 0.875rem; color: #6b7280; font-weight: 600">
        {{ processedFiles + failedFiles }} / {{ totalFiles }} fichiers
        <span v-if="failedFiles > 0" style="color: #ef4444; font-weight: 700">• {{ failedFiles }} échoué(s)</span>
      </p>
    </div>

    <div class="container" style="padding-top: 2rem; padding-bottom: 2rem">
      <!-- Header -->
      <div class="flex items-center justify-between mb-6">
        <div>
          <h1 style="font-size: 2rem; font-weight: 800; margin: 0; background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;">
            📚 AATAA
          </h1>
          <p class="text-gray-500" style="margin-top: 0.25rem; font-weight: 500">
            An App To Archive All - Archivage intelligent avec OCR multilingue
          </p>
        </div>
        <div style="display: flex; gap: 0.5rem;">
          <NuxtLink
            to="/trash"
            class="btn btn-secondary icon-btn"
            title="Corbeille"
            style="text-decoration: none;"
          >
            🗑️
          </NuxtLink>
          <button
            class="btn btn-secondary icon-btn"
            @click="showSettings = true"
            title="Paramètres"
          >
            ⚙️
          </button>
        </div>
      </div>

      <!-- Import Section -->
      <div class="card mb-6">
        <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem">
          Importer un document
        </h2>

        <!-- OCR Type Selector -->
        <OcrTypeSelector v-model="selectedOcrType" style="margin-bottom: 1.5rem" />

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

        <div style="margin-top: 1rem; text-align: center">
          <button class="btn btn-secondary" @click="selectFolder" style="font-size: 0.875rem">
            📁 Ou sélectionner un dossier entier
          </button>
        </div>

        <div v-if="processing" style="text-align: center; margin-top: 1rem">
          <div style="font-size: 1.5rem; margin-bottom: 0.5rem">⏳</div>
          <p class="text-gray-600">
            {{ processingMessage }}
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

      <!-- Dashboard Stats -->
      <div v-if="documents.length > 0" class="card mb-6">
        <div class="flex items-center justify-between mb-4">
          <h2 style="font-size: 1.25rem; font-weight: 600; margin: 0">
            📊 Statistiques
          </h2>
          <button
            @click="showDashboard = !showDashboard"
            class="btn btn-secondary"
            style="padding: 0.5rem 1rem; font-size: 0.875rem"
          >
            {{ showDashboard ? '▼ Masquer' : '▶ Afficher' }}
          </button>
        </div>

        <div v-if="showDashboard" style="display: grid; gap: 1.5rem">
          <!-- Stats Overview -->
          <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem">
            <div style="background: #dbeafe; padding: 1rem; border-radius: 0.5rem; text-align: center">
              <div style="font-size: 2rem; font-weight: bold; color: #1e40af">{{ documents.length }}</div>
              <div style="font-size: 0.875rem; color: #1e40af; margin-top: 0.25rem">Documents totaux</div>
            </div>
            <div style="background: #dcfce7; padding: 1rem; border-radius: 0.5rem; text-align: center">
              <div style="font-size: 2rem; font-weight: bold; color: #15803d">{{ stats.documentTypes }}</div>
              <div style="font-size: 0.875rem; color: #15803d; margin-top: 0.25rem">Types différents</div>
            </div>
            <div style="background: #fef3c7; padding: 1rem; border-radius: 0.5rem; text-align: center">
              <div style="font-size: 2rem; font-weight: bold; color: #a16207">{{ stats.totalTags }}</div>
              <div style="font-size: 0.875rem; color: #a16207; margin-top: 0.25rem">Tags utilisés</div>
            </div>
            <div style="background: #f3e8ff; padding: 1rem; border-radius: 0.5rem; text-align: center">
              <div style="font-size: 2rem; font-weight: bold; color: #6b21a8">{{ stats.totalSize }}</div>
              <div style="font-size: 0.875rem; color: #6b21a8; margin-top: 0.25rem">Taille totale</div>
            </div>
          </div>

          <!-- Documents by Type -->
          <div>
            <h3 style="font-weight: 600; margin-bottom: 1rem">📁 Répartition par type</h3>
            <div style="display: grid; gap: 0.5rem">
              <div v-for="[type, count] in stats.byType" :key="type" style="display: flex; align-items: center; gap: 0.5rem">
                <div style="min-width: 150px; font-size: 0.875rem">{{ type }}</div>
                <div style="flex: 1; background: #e5e7eb; height: 24px; border-radius: 4px; overflow: hidden; position: relative">
                  <div
                    :style="{
                      width: `${(count / documents.length) * 100}%`,
                      background: typeColors[type] === 'blue' ? '#3b82f6' : typeColors[type] === 'green' ? '#10b981' : typeColors[type] === 'yellow' ? '#f59e0b' : '#6b7280',
                      height: '100%',
                      transition: 'width 0.3s'
                    }"
                  ></div>
                </div>
                <div style="min-width: 60px; text-align: right; font-size: 0.875rem; font-weight: 600">{{ count }} ({{ Math.round((count / documents.length) * 100) }}%)</div>
              </div>
            </div>
          </div>

          <!-- Documents by Month -->
          <div>
            <h3 style="font-weight: 600; margin-bottom: 1rem">📅 Documents par mois (6 derniers mois)</h3>
            <div style="display: grid; gap: 0.5rem">
              <div v-for="[month, count] in stats.byMonth" :key="month" style="display: flex; align-items: center; gap: 0.5rem">
                <div style="min-width: 120px; font-size: 0.875rem">{{ month }}</div>
                <div style="flex: 1; background: #e5e7eb; height: 24px; border-radius: 4px; overflow: hidden">
                  <div
                    :style="{
                      width: `${(count / Math.max(...stats.byMonth.map(m => m[1]))) * 100}%`,
                      background: '#8b5cf6',
                      height: '100%',
                      transition: 'width 0.3s'
                    }"
                  ></div>
                </div>
                <div style="min-width: 40px; text-align: right; font-size: 0.875rem; font-weight: 600">{{ count }}</div>
              </div>
            </div>
          </div>

          <!-- Top Tags -->
          <div>
            <h3 style="font-weight: 600; margin-bottom: 1rem">🏷️ Tags les plus utilisés</h3>
            <div class="flex gap-2" style="flex-wrap: wrap">
              <div
                v-for="[tag, count] in stats.topTags"
                :key="tag"
                class="badge badge-blue"
                style="font-size: 0.875rem; padding: 0.5rem 1rem"
              >
                {{ tag }} ({{ count }})
              </div>
              <div v-if="stats.topTags.length === 0" style="color: #6b7280; font-size: 0.875rem">
                Aucun tag utilisé
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Documents List -->
      <div class="card">
        <div class="flex items-center justify-between mb-6">
          <h2 style="font-size: 1.25rem; font-weight: 600; margin: 0">
            Documents archivés ({{ filteredDocuments.length }}{{ filteredDocuments.length !== documents.length ? ` / ${documents.length}` : '' }})
          </h2>
          <div class="flex gap-2">
            <button
              v-if="filteredDocuments.length > 0"
              class="btn btn-secondary"
              @click="exportToCSV"
              style="padding: 0.5rem 1rem; font-size: 0.875rem"
              title="Exporter en CSV"
            >
              📥 Export CSV
            </button>
            <button
              class="btn btn-secondary"
              @click="loadDocuments"
              style="padding: 0.5rem; width: 36px; height: 36px"
              title="Actualiser"
            >
              🔄
            </button>
          </div>
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
            <div v-if="!editMode">
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
              <button @click="editMode = true" class="btn btn-secondary" style="margin-top: 0.5rem; font-size: 0.875rem">
                ✏️ Modifier les métadonnées
              </button>
            </div>

            <!-- Edit Mode -->
            <div v-else>
              <div style="margin-bottom: 1rem">
                <label style="display: block; font-size: 0.875rem; font-weight: 500; margin-bottom: 0.5rem">Type de document</label>
                <select v-model="editedType" class="input">
                  <option value="Facture">Facture</option>
                  <option value="Contrat">Contrat</option>
                  <option value="Relevé bancaire">Relevé bancaire</option>
                  <option value="Bulletin de paie">Bulletin de paie</option>
                  <option value="Document officiel">Document officiel</option>
                  <option value="Reçu">Reçu</option>
                  <option value="Unknown">Autre</option>
                </select>
              </div>

              <div style="margin-bottom: 1rem">
                <label style="display: block; font-size: 0.875rem; font-weight: 500; margin-bottom: 0.5rem">Nom du fichier</label>
                <input v-model="editedName" type="text" class="input" />
              </div>

              <div style="margin-bottom: 1rem">
                <label style="display: block; font-size: 0.875rem; font-weight: 500; margin-bottom: 0.5rem">Tags (cliquez pour activer/désactiver)</label>
                <div class="flex gap-2" style="flex-wrap: wrap">
                  <button
                    v-for="tag in ['Urgent', 'Important', 'Personnel', 'Professionnel', 'Mensuel']"
                    :key="tag"
                    @click="toggleTag(tag)"
                    :class="['badge', editedTags.includes(tag) ? 'badge-blue' : 'badge-gray']"
                    style="cursor: pointer"
                  >
                    {{ tag }}
                  </button>
                </div>
              </div>

              <div class="flex gap-2">
                <button @click="saveMetadata" class="btn btn-primary">
                  💾 Sauvegarder
                </button>
                <button @click="editMode = false" class="btn btn-secondary">
                  Annuler
                </button>
              </div>
            </div>
          </div>

          <!-- Notes -->
          <div>
            <h3 style="font-weight: 600; margin-bottom: 0.5rem">📝 Notes personnelles</h3>
            <textarea
              v-model="editedNotes"
              class="input"
              style="min-height: 100px; font-family: inherit; resize: vertical"
              placeholder="Ajoutez vos notes ici..."
            ></textarea>
            <button @click="saveNotes" class="btn btn-primary" style="margin-top: 0.5rem; font-size: 0.875rem">
              💾 Sauvegarder les notes
            </button>
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
import ProcessingIndicator from '../components/ProcessingIndicator.vue'

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
  notes?: string
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

// ML OCR state
const selectedOcrType = ref<'standard' | 'handwritten' | 'printed-ml' | 'caption'>('standard')
const processingMessage = ref('Traitement en cours... (OCR + Classification)')

// Multi-file processing state
const processingMultiple = ref(false)
const totalFiles = ref(0)
const processedFiles = ref(0)
const failedFiles = ref(0)

// Edit mode state
const editMode = ref(false)
const editedNotes = ref('')
const editedType = ref('')
const editedTags = ref<string[]>([])
const editedName = ref('')

// Dashboard state
const showDashboard = ref(true)

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

// Fonction pour exécuter l'OCR ML selon le type sélectionné
async function runMlOcr(filePath: string): Promise<string | null> {
  try {
    if (selectedOcrType.value === 'handwritten') {
      processingMessage.value = '🖊️ Extraction du texte manuscrit avec TrOCR...'
      const text = await invoke<string>('extract_handwritten_text', {
        imagePath: filePath,
        ocrType: 'handwritten'
      })
      return text
    } else if (selectedOcrType.value === 'printed-ml') {
      processingMessage.value = '📰 Extraction du texte imprimé avec TrOCR ML...'
      const text = await invoke<string>('extract_handwritten_text', {
        imagePath: filePath,
        ocrType: 'printed'
      })
      return text
    } else if (selectedOcrType.value === 'caption') {
      processingMessage.value = '🎨 Génération de la description avec BLIP...'
      const caption = await invoke<string>('generate_image_caption', {
        imagePath: filePath,
        language: 'fr'
      })
      return caption
    }
    // Type 'standard' utilise Tesseract par défaut (pas d'appel ML)
    processingMessage.value = 'Traitement avec Tesseract OCR...'
    return null
  } catch (error) {
    console.error('Erreur ML OCR:', error)
    showMessage(`⚠️ Erreur ML OCR (retour à Tesseract): ${error}`, 5000)
    return null
  }
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

async function selectFolder() {
  console.log('selectFolder called!')

  try {
    const selected = await openDialog({
      directory: true,
      multiple: false
    })

    console.log('Folder selected:', selected)

    if (!selected || typeof selected !== 'string') {
      console.log('No folder selected')
      return
    }

    // Scan folder for supported files
    showMessage('🔍 Scan du dossier...', 5000)
    const files = await invoke<string[]>('scan_folder', { folderPath: selected })

    console.log(`Found ${files.length} files in folder`)

    if (files.length === 0) {
      showMessage('⚠️ Aucun fichier supporté trouvé dans ce dossier', 5000)
      return
    }

    if (files.length > 100) {
      showMessage(`❌ Trop de fichiers (${files.length}). Maximum 100 fichiers à la fois`, 5000)
      return
    }

    // Process all files
    await processMultipleFiles(files)

  } catch (error) {
    console.error('Error selecting folder:', error)
    showMessage('❌ Erreur sélection dossier: ' + error, 5000)
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
    await invoke('open_file', { file_path: filePath })
  } catch (error) {
    console.error('Error opening file:', error)
    showMessage('❌ Impossible d\'ouvrir le fichier: ' + error, 5000)
  }
}

function openPreview(doc: Document) {
  previewDocument.value = doc
  showPreview.value = true
  editMode.value = false
  editedNotes.value = doc.notes || ''
  editedType.value = doc.document_type
  editedTags.value = [...doc.tags]
  editedName.value = doc.new_name
  console.log('Preview opened for:', doc.new_name)
  console.log('OCR text:', doc.ocr_text)
  console.log('Full document:', doc)
}

function closePreview() {
  showPreview.value = false
  previewDocument.value = null
  editMode.value = false
}

async function saveNotes() {
  if (!previewDocument.value) return

  try {
    await invoke('update_notes', {
      id: previewDocument.value.id,
      notes: editedNotes.value || null
    })

    // Update local document
    previewDocument.value.notes = editedNotes.value
    const docIndex = documents.value.findIndex(d => d.id === previewDocument.value!.id)
    if (docIndex !== -1) {
      documents.value[docIndex].notes = editedNotes.value
    }

    showMessage('✅ Notes sauvegardées', 3000)
  } catch (error) {
    console.error('Error saving notes:', error)
    showMessage('❌ Erreur sauvegarde notes: ' + error, 5000)
  }
}

async function saveMetadata() {
  if (!previewDocument.value) return

  try {
    await invoke('update_metadata', {
      id: previewDocument.value.id,
      documentType: editedType.value,
      tags: editedTags.value,
      newName: editedName.value
    })

    // Update local document
    previewDocument.value.document_type = editedType.value
    previewDocument.value.tags = [...editedTags.value]
    previewDocument.value.new_name = editedName.value

    const docIndex = documents.value.findIndex(d => d.id === previewDocument.value!.id)
    if (docIndex !== -1) {
      documents.value[docIndex].document_type = editedType.value
      documents.value[docIndex].tags = [...editedTags.value]
      documents.value[docIndex].new_name = editedName.value
    }

    editMode.value = false
    showMessage('✅ Métadonnées mises à jour', 3000)
  } catch (error) {
    console.error('Error saving metadata:', error)
    showMessage('❌ Erreur sauvegarde: ' + error, 5000)
  }
}

function toggleTag(tag: string) {
  const index = editedTags.value.indexOf(tag)
  if (index === -1) {
    editedTags.value.push(tag)
  } else {
    editedTags.value.splice(index, 1)
  }
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
      doc.tags.some(tag => tag.toLowerCase().includes(query)) ||
      (doc.notes && doc.notes.toLowerCase().includes(query))
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

// Dashboard stats
const stats = computed(() => {
  // Total size
  const totalBytes = documents.value.reduce((sum, doc) => sum + doc.file_size, 0)
  const totalSize = formatFileSize(totalBytes)

  // Document types count
  const typeCount = new Map<string, number>()
  documents.value.forEach(doc => {
    typeCount.set(doc.document_type, (typeCount.get(doc.document_type) || 0) + 1)
  })
  const byType = Array.from(typeCount.entries()).sort((a, b) => b[1] - a[1])
  const documentTypes = typeCount.size

  // Tags count
  const tagCount = new Map<string, number>()
  documents.value.forEach(doc => {
    doc.tags.forEach(tag => {
      tagCount.set(tag, (tagCount.get(tag) || 0) + 1)
    })
  })
  const topTags = Array.from(tagCount.entries())
    .sort((a, b) => b[1] - a[1])
    .slice(0, 5)
  const totalTags = tagCount.size

  // Documents by month (last 6 months)
  const monthCount = new Map<string, number>()
  const now = new Date()
  const monthNames = ['Jan', 'Fév', 'Mar', 'Avr', 'Mai', 'Juin', 'Juil', 'Août', 'Sep', 'Oct', 'Nov', 'Déc']

  documents.value.forEach(doc => {
    const date = new Date(doc.created_at)
    const key = `${monthNames[date.getMonth()]} ${date.getFullYear()}`
    monthCount.set(key, (monthCount.get(key) || 0) + 1)
  })

  // Get last 6 months
  const last6Months: Array<[string, number]> = []
  for (let i = 5; i >= 0; i--) {
    const d = new Date(now.getFullYear(), now.getMonth() - i, 1)
    const key = `${monthNames[d.getMonth()]} ${d.getFullYear()}`
    last6Months.push([key, monthCount.get(key) || 0])
  }

  return {
    totalSize,
    documentTypes,
    totalTags,
    byType,
    byMonth: last6Months,
    topTags
  }
})

function exportToCSV() {
  try {
    // Préparer les données CSV
    const headers = ['ID', 'Nom', 'Nom original', 'Type', 'Tags', 'Date', 'Taille', 'Chemin', 'Notes']
    const rows = filteredDocuments.value.map(doc => [
      doc.id,
      doc.new_name,
      doc.original_name,
      doc.document_type,
      doc.tags.join('; '),
      new Date(doc.created_at).toLocaleString('fr-FR'),
      formatFileSize(doc.file_size),
      doc.file_path,
      doc.notes || ''
    ])

    // Échapper les guillemets et virgules pour CSV
    const escapeCSV = (value: string) => {
      if (value.includes(',') || value.includes('"') || value.includes('\n')) {
        return `"${value.replace(/"/g, '""')}"`
      }
      return value
    }

    // Créer le contenu CSV
    const csvContent = [
      headers.join(','),
      ...rows.map(row => row.map(cell => escapeCSV(String(cell))).join(','))
    ].join('\n')

    // Créer un blob et télécharger
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = `aataa_export_${new Date().toISOString().split('T')[0]}.csv`
    link.click()
    URL.revokeObjectURL(url)

    showMessage(`✅ Export réussi : ${filteredDocuments.value.length} document(s)`, 3000)
  } catch (error) {
    console.error('Error exporting CSV:', error)
    showMessage('❌ Erreur export CSV: ' + error, 5000)
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
    showMessage('❌ Erreur sélection dossier: ' + error, 5000)
  }
}
</script>

<style scoped>
@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(100px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
