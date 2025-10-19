<template>
  <div v-if="showOnboarding" class="onboarding-overlay">
    <div class="onboarding-container md-card animate-scale-in">
      <!-- Progress Indicator -->
      <div class="onboarding-progress">
        <div
          v-for="stepNum in 4"
          :key="stepNum"
          class="progress-dot"
          :class="{ active: stepNum === currentStep, completed: stepNum < currentStep }"
        />
      </div>

      <!-- Step 1: Bienvenue -->
      <div v-if="currentStep === 1" class="onboarding-step animate-fade-in">
        <div class="step-icon">
          <svg width="64" height="64" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M32 8v48M8 32h48" stroke-linecap="round"/>
            <circle cx="32" cy="32" r="24"/>
          </svg>
        </div>
        <h1 class="headline-large">Bienvenue dans AATAA</h1>
        <p class="body-large step-description">
          Votre assistant intelligent de gestion documentaire. <br>
          AATAA organise, recherche et classe automatiquement tous vos documents importants.
        </p>

        <div class="features-grid">
          <div class="feature-item">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <path d="M6 12h20M6 20h14" stroke-width="2" stroke-linecap="round"/>
              <rect x="4" y="6" width="24" height="20" rx="2" stroke-width="2"/>
            </svg>
            <h3 class="title-medium">OCR Intelligent</h3>
            <p class="body-small">Reconnaissance de texte même manuscrit</p>
          </div>
          <div class="feature-item">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <path d="M4 8h24M4 16h24M4 24h24" stroke-width="2" stroke-linecap="round"/>
              <circle cx="8" cy="8" r="2" fill="currentColor"/>
            </svg>
            <h3 class="title-medium">Classification IA</h3>
            <p class="body-small">Catégorisation automatique précise</p>
          </div>
          <div class="feature-item">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <circle cx="14" cy="14" r="10" stroke-width="2"/>
              <path d="M21 21l7 7" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <h3 class="title-medium">Recherche Rapide</h3>
            <p class="body-small">Trouvez n'importe quel document en secondes</p>
          </div>
        </div>
      </div>

      <!-- Step 2: Premier import -->
      <div v-if="currentStep === 2" class="onboarding-step animate-fade-in">
        <div class="step-icon">
          <svg width="64" height="64" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M32 48V16m0 0l-12 12m12-12l12 12" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M12 32v16a4 4 0 0 0 4 4h32a4 4 0 0 0 4-4V32" stroke-linecap="round"/>
          </svg>
        </div>
        <h1 class="headline-large">Importez votre premier document</h1>
        <p class="body-large step-description">
          Glissez-déposez n'importe quel PDF, image ou photo de document.<br>
          AATAA l'analysera automatiquement et l'organisera pour vous.
        </p>

        <div
          @click="selectFile"
          @dragover.prevent="isDragging = true"
          @dragleave="isDragging = false"
          @drop.prevent="handleDrop"
          class="drop-zone md-card"
          :class="{ dragging: isDragging, 'has-file': selectedFile }"
        >
          <svg v-if="!selectedFile" width="48" height="48" viewBox="0 0 48 48" fill="none" stroke="currentColor">
            <path d="M24 36V12m0 0l-9 9m9-9l9 9" stroke-width="2" stroke-linecap="round"/>
            <path d="M8 24v16a2 2 0 0 0 2 2h28a2 2 0 0 0 2-2V24" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <div v-if="selectedFile" class="selected-file-info">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none" stroke="currentColor">
              <path d="M18 2H8a2 2 0 0 0-2 2v24a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V10l-8-8z" stroke-width="2"/>
              <circle cx="16" cy="24" r="6" fill="var(--md-sys-color-primary)" stroke="none"/>
              <path d="M14 24l2 2 4-4" stroke="white" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <span class="label-large">{{ selectedFile.name }}</span>
          </div>
          <span v-else class="body-large">
            Cliquez ou glissez un document ici
          </span>
        </div>

        <p v-if="uploadError" class="error-message body-small">{{ uploadError }}</p>
      </div>

      <!-- Step 3: Démonstration recherche -->
      <div v-if="currentStep === 3" class="onboarding-step animate-fade-in">
        <div class="step-icon">
          <svg width="64" height="64" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="28" cy="28" r="20" stroke-linecap="round"/>
            <path d="M42 42l14 14" stroke-linecap="round"/>
            <path d="M20 28h16M28 20v16" stroke-linecap="round"/>
          </svg>
        </div>
        <h1 class="headline-large">Recherche ultra-rapide</h1>
        <p class="body-large step-description">
          Trouvez n'importe quel document en tapant simplement quelques mots.<br>
          La recherche inclut le contenu, les catégories, les dates et les notes.
        </p>

        <div class="search-demo md-card">
          <div class="search-box">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor">
              <circle cx="9" cy="9" r="7" stroke-width="2"/>
              <path d="M14 14l4 4" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <input
              type="text"
              v-model="searchQuery"
              placeholder="Essayez : facture, edf, 2024, contrat..."
              class="body-large"
            />
          </div>
          <div v-if="searchQuery" class="search-results">
            <div class="result-item">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6z" stroke-width="2"/>
              </svg>
              <div>
                <span class="label-large">Résultat correspondant à "{{ searchQuery }}"</span>
                <span class="body-small">Finance • 2024</span>
              </div>
            </div>
          </div>
        </div>

        <div class="tips-list">
          <h3 class="title-medium">Astuces de recherche :</h3>
          <ul>
            <li class="body-medium">🔍 Cherchez par mots-clés du contenu</li>
            <li class="body-medium">📅 Filtrez par année (ex: "2024")</li>
            <li class="body-medium">📁 Trouvez par catégorie (ex: "finance")</li>
          </ul>
        </div>
      </div>

      <!-- Step 4: Configuration catégories -->
      <div v-if="currentStep === 4" class="onboarding-step animate-fade-in">
        <div class="step-icon">
          <svg width="64" height="64" viewBox="0 0 64 64" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M8 16h48M8 32h48M8 48h48" stroke-linecap="round"/>
            <circle cx="16" cy="16" r="4" fill="currentColor"/>
            <circle cx="16" cy="32" r="4" fill="currentColor"/>
            <circle cx="16" cy="48" r="4" fill="currentColor"/>
          </svg>
        </div>
        <h1 class="headline-large">Personnalisez vos catégories</h1>
        <p class="body-large step-description">
          AATAA classe automatiquement vos documents en 9 catégories.<br>
          Vous pouvez ajouter des sous-catégories personnalisées à tout moment.
        </p>

        <div class="categories-preview">
          <div v-for="category in mainCategories" :key="category" class="category-chip md-chip">
            {{ category }}
          </div>
        </div>

        <div class="final-tips md-card">
          <h3 class="title-medium">Vous êtes prêt ! 🎉</h3>
          <p class="body-medium">
            Commencez à importer vos documents et laissez l'IA faire le travail.<br>
            Vous pouvez toujours modifier les catégories dans les paramètres.
          </p>
        </div>
      </div>

      <!-- Navigation -->
      <div class="onboarding-actions">
        <button
          v-if="currentStep > 1"
          @click="previousStep"
          class="md-text-button"
        >
          Précédent
        </button>
        <div class="spacer"></div>
        <button
          v-if="currentStep < 4"
          @click="nextStep"
          class="md-filled-button md-ripple"
          :disabled="currentStep === 2 && !selectedFile"
        >
          Suivant
        </button>
        <button
          v-else
          @click="completeOnboarding"
          class="md-filled-button md-ripple"
        >
          Commencer
        </button>
      </div>

      <!-- Skip button -->
      <button @click="completeOnboarding" class="skip-button md-text-button">
        Passer l'introduction
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const showOnboarding = ref(false)
const currentStep = ref(1)
const isDragging = ref(false)
const selectedFile = ref<File | null>(null)
const uploadError = ref('')
const searchQuery = ref('')
const mainCategories = ref([
  'Finance', 'Santé', 'Admin', 'Juridique',
  'Logement', 'Emploi', 'Assurance', 'Personnel', 'Autre'
])

const emit = defineEmits<{
  complete: []
}>()

onMounted(() => {
  // Check if onboarding was already completed
  const onboardingCompleted = localStorage.getItem('onboarding_completed')
  if (!onboardingCompleted) {
    showOnboarding.value = true
  }
})

function nextStep() {
  if (currentStep.value < 4) {
    currentStep.value++
  }
}

function previousStep() {
  if (currentStep.value > 1) {
    currentStep.value--
  }
}

async function selectFile() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.pdf,.png,.jpg,.jpeg'
  input.onchange = async (e) => {
    const file = (e.target as HTMLInputElement).files?.[0]
    if (file) {
      selectedFile.value = file
      uploadError.value = ''
      // Optionally process the file here
      await processFile(file)
    }
  }
  input.click()
}

function handleDrop(e: DragEvent) {
  isDragging.value = false
  const file = e.dataTransfer?.files[0]
  if (file) {
    selectedFile.value = file
    uploadError.value = ''
    processFile(file)
  }
}

async function processFile(file: File) {
  try {
    // This is a demo - actual implementation would process the file
    console.log('Processing file:', file.name)
    // You can add actual file processing here if needed
  } catch (error) {
    uploadError.value = `Erreur: ${error}`
  }
}

function completeOnboarding() {
  localStorage.setItem('onboarding_completed', 'true')
  showOnboarding.value = false
  emit('complete')
}

// Expose method to restart onboarding
defineExpose({
  restart: () => {
    currentStep.value = 1
    showOnboarding.value = true
  }
})
</script>

<style scoped>
.onboarding-overlay {
  position: fixed;
  inset: 0;
  z-index: var(--md-sys-z-index-modal);
  background-color: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--md-sys-spacing-lg);
}

.onboarding-container {
  max-width: 700px;
  width: 100%;
  padding: var(--md-sys-spacing-xxl);
  text-align: center;
  position: relative;
}

/* Progress Indicator */
.onboarding-progress {
  display: flex;
  justify-content: center;
  gap: var(--md-sys-spacing-sm);
  margin-bottom: var(--md-sys-spacing-xl);
}

.progress-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: var(--md-sys-color-outline);
  transition: all var(--md-sys-motion-duration-medium1) var(--md-sys-motion-easing-standard);
}

.progress-dot.active {
  width: 24px;
  border-radius: 4px;
  background-color: var(--md-sys-color-primary);
}

.progress-dot.completed {
  background-color: var(--md-sys-color-primary);
}

/* Step Content */
.onboarding-step {
  min-height: 400px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--md-sys-spacing-lg);
}

.step-icon {
  color: var(--md-sys-color-primary);
  margin-bottom: var(--md-sys-spacing-md);
}

.onboarding-step h1 {
  color: var(--md-sys-color-on-surface);
  margin: 0;
}

.step-description {
  color: var(--md-sys-color-on-surface-variant);
  max-width: 500px;
  line-height: 1.6;
}

/* Features Grid */
.features-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--md-sys-spacing-lg);
  margin-top: var(--md-sys-spacing-lg);
  width: 100%;
}

.feature-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--md-sys-spacing-sm);
  padding: var(--md-sys-spacing-md);
  text-align: center;
}

.feature-item svg {
  color: var(--md-sys-color-primary);
}

.feature-item h3 {
  color: var(--md-sys-color-on-surface);
  margin: 0;
}

.feature-item p {
  color: var(--md-sys-color-on-surface-variant);
  margin: 0;
}

/* Drop Zone */
.drop-zone {
  width: 100%;
  padding: var(--md-sys-spacing-xxl);
  border: 2px dashed var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-large);
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--md-sys-spacing-md);
  min-height: 200px;
  justify-content: center;
}

.drop-zone:hover {
  border-color: var(--md-sys-color-primary);
  background-color: color-mix(in srgb, var(--md-sys-color-primary) 4%, transparent);
}

.drop-zone.dragging {
  border-color: var(--md-sys-color-primary);
  background-color: color-mix(in srgb, var(--md-sys-color-primary) 8%, transparent);
  transform: scale(1.02);
}

.drop-zone.has-file {
  border-color: var(--md-sys-color-primary);
  background-color: color-mix(in srgb, var(--md-sys-color-primary) 12%, transparent);
}

.drop-zone svg {
  color: var(--md-sys-color-on-surface-variant);
}

.selected-file-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--md-sys-spacing-md);
}

.error-message {
  color: var(--md-sys-color-error);
  margin-top: var(--md-sys-spacing-sm);
}

/* Search Demo */
.search-demo {
  width: 100%;
  padding: var(--md-sys-spacing-lg);
}

.search-box {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-sm);
  padding: var(--md-sys-spacing-md);
  background-color: var(--md-sys-color-surface-container-high);
  border-radius: var(--md-sys-shape-corner-full);
}

.search-box svg {
  color: var(--md-sys-color-on-surface-variant);
}

.search-box input {
  flex: 1;
  border: none;
  background: none;
  color: var(--md-sys-color-on-surface);
  outline: none;
}

.search-results {
  margin-top: var(--md-sys-spacing-md);
  padding-top: var(--md-sys-spacing-md);
  border-top: 1px solid var(--md-sys-color-outline-variant);
}

.result-item {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-md);
  padding: var(--md-sys-spacing-md);
  background-color: var(--md-sys-color-surface-container);
  border-radius: var(--md-sys-shape-corner-medium);
  text-align: left;
}

.result-item svg {
  color: var(--md-sys-color-primary);
  flex-shrink: 0;
}

.result-item > div {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-xs);
}

/* Tips List */
.tips-list {
  text-align: left;
  width: 100%;
  padding: var(--md-sys-spacing-lg);
  background-color: var(--md-sys-color-surface-container-low);
  border-radius: var(--md-sys-shape-corner-medium);
}

.tips-list h3 {
  color: var(--md-sys-color-on-surface);
  margin: 0 0 var(--md-sys-spacing-md) 0;
}

.tips-list ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-sm);
}

.tips-list li {
  color: var(--md-sys-color-on-surface-variant);
}

/* Categories Preview */
.categories-preview {
  display: flex;
  flex-wrap: wrap;
  gap: var(--md-sys-spacing-sm);
  justify-content: center;
  width: 100%;
}

.category-chip {
  background-color: var(--md-sys-color-primary-container);
  color: var(--md-sys-color-on-primary-container);
}

/* Final Tips */
.final-tips {
  width: 100%;
  padding: var(--md-sys-spacing-xl);
  background-color: color-mix(in srgb, var(--md-sys-color-primary) 8%, transparent);
  text-align: center;
}

.final-tips h3 {
  color: var(--md-sys-color-on-surface);
  margin: 0 0 var(--md-sys-spacing-md) 0;
}

.final-tips p {
  color: var(--md-sys-color-on-surface-variant);
  margin: 0;
  line-height: 1.6;
}

/* Navigation */
.onboarding-actions {
  display: flex;
  gap: var(--md-sys-spacing-md);
  margin-top: var(--md-sys-spacing-xl);
  width: 100%;
}

.spacer {
  flex: 1;
}

.skip-button {
  position: absolute;
  top: var(--md-sys-spacing-md);
  right: var(--md-sys-spacing-md);
  opacity: 0.7;
}

.skip-button:hover {
  opacity: 1;
}

/* Responsive */
@media (max-width: 768px) {
  .onboarding-container {
    padding: var(--md-sys-spacing-lg);
  }

  .features-grid {
    grid-template-columns: 1fr;
  }

  .categories-preview {
    flex-direction: column;
  }
}
</style>
