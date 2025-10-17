<template>
  <div class="keywords-manager">
    <div class="category-section" v-for="category in categories" :key="category">
      <div class="category-header">
        <span class="material-icons category-icon">{{ getCategoryIcon(category) }}</span>
        <h3>{{ category }}</h3>
        <span class="keyword-count body-small">{{ getKeywordsForCategory(category).length }} mots-clés</span>
      </div>

      <div class="keywords-list">
        <!-- Mots-clés existants -->
        <div 
          v-for="kw in getKeywordsForCategory(category)" 
          :key="kw.id"
          class="keyword-item"
        >
          <div class="keyword-content">
            <span class="keyword-text">{{ kw.keyword }}</span>
            <span class="keyword-weight" :class="getWeightClass(kw.weight)">
              {{ getWeightLabel(kw.weight) }}
            </span>
          </div>
          <div class="keyword-actions">
            <button 
              @click="startEditing(kw)"
              class="btn-icon"
              title="Modifier"
            >
              <span class="material-icons">edit</span>
            </button>
            <button 
              @click="handleDelete(kw)"
              class="btn-icon btn-delete"
              title="Supprimer"
            >
              <span class="material-icons">delete</span>
            </button>
          </div>
        </div>

        <!-- Bouton d'ajout -->
        <button 
          v-if="!isAdding[category]"
          @click="startAdding(category)"
          class="add-keyword-btn"
        >
          <span class="material-icons">add</span>
          Ajouter un mot-clé
        </button>

        <!-- Formulaire d'ajout/édition -->
        <div v-else class="keyword-form">
          <input
            v-model="formData.keyword"
            @keyup.enter="handleSave(category)"
            @keyup.escape="cancelEditing"
            type="text"
            placeholder="Mot-clé (ex: facture, impôt...)"
            class="input-field"
            ref="inputRef"
            autofocus
          />
          <select v-model="formData.weight" class="weight-select">
            <option :value="0.5">Faible</option>
            <option :value="1.0">Normal</option>
            <option :value="1.5">Important</option>
            <option :value="2.0">Très important</option>
          </select>
          <button @click="handleSave(category)" class="btn-primary" title="Enregistrer">
            <span class="material-icons">check</span>
          </button>
          <button @click="cancelEditing" class="btn-secondary" title="Annuler">
            <span class="material-icons">close</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Message d'erreur -->
    <div v-if="error" class="error-message animate-fade-in">
      <span class="material-icons">error</span>
      {{ error }}
    </div>

    <!-- Message de succès -->
    <div v-if="successMessage" class="success-message animate-fade-in">
      <span class="material-icons">check_circle</span>
      {{ successMessage }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import { useClassificationKeywords } from '../composables/useClassificationKeywords'
import { CATEGORY_ICONS, type MainCategory } from '../types/document'
import type { ClassificationKeyword } from '../api/keywords'

const {
  keywords,
  error,
  loadKeywords,
  addKeyword,
  updateKeyword,
  deleteKeyword,
  getKeywordsForCategory: getKeywordsFor,
} = useClassificationKeywords()

const categories: MainCategory[] = [
  'Administratif',
  'Financier',
  'Santé',
  'Professionnel',
  'Immobilier',
  'Académique',
  'Personnel',
  'Autre',
]

const isAdding = ref<Record<MainCategory, boolean>>({} as Record<MainCategory, boolean>)
const editingKeyword = ref<ClassificationKeyword | null>(null)
const formData = ref({
  keyword: '',
  weight: 1.0,
})
const inputRef = ref<HTMLInputElement | null>(null)
const successMessage = ref<string | null>(null)

onMounted(async () => {
  await loadKeywords()
})

function getCategoryIcon(category: MainCategory) {
  return CATEGORY_ICONS[category] || 'folder'
}

function getKeywordsForCategory(category: MainCategory) {
  return getKeywordsFor(category)
}

function getWeightLabel(weight: number): string {
  if (weight >= 2.0) return 'Très important'
  if (weight >= 1.5) return 'Important'
  if (weight >= 1.0) return 'Normal'
  return 'Faible'
}

function getWeightClass(weight: number): string {
  if (weight >= 2.0) return 'weight-very-high'
  if (weight >= 1.5) return 'weight-high'
  if (weight >= 1.0) return 'weight-normal'
  return 'weight-low'
}

function startAdding(category: MainCategory) {
  isAdding.value[category] = true
  editingKeyword.value = null
  formData.value = {
    keyword: '',
    weight: 1.0,
  }
  nextTick(() => {
    inputRef.value?.focus()
  })
}

function startEditing(kw: ClassificationKeyword) {
  const category = kw.category as MainCategory
  isAdding.value[category] = true
  editingKeyword.value = kw
  formData.value = {
    keyword: kw.keyword,
    weight: kw.weight,
  }
  nextTick(() => {
    inputRef.value?.focus()
  })
}

function cancelEditing() {
  Object.keys(isAdding.value).forEach(key => {
    isAdding.value[key as MainCategory] = false
  })
  editingKeyword.value = null
  formData.value = {
    keyword: '',
    weight: 1.0,
  }
}

async function handleSave(category: MainCategory) {
  const trimmedKeyword = formData.value.keyword.trim().toLowerCase()
  if (!trimmedKeyword) return

  try {
    if (editingKeyword.value) {
      // Mode édition
      await updateKeyword(editingKeyword.value.id, trimmedKeyword, formData.value.weight)
      successMessage.value = `Mot-clé "${trimmedKeyword}" mis à jour`
    } else {
      // Mode ajout
      await addKeyword(category, null, trimmedKeyword, formData.value.weight)
      successMessage.value = `Mot-clé "${trimmedKeyword}" ajouté`
    }
    
    cancelEditing()
    setTimeout(() => successMessage.value = null, 3000)
  } catch (e) {
    const errorMessage = e instanceof Error ? e.message : String(e)
    if (errorMessage.includes('UNIQUE constraint')) {
      error.value = `Le mot-clé "${trimmedKeyword}" existe déjà dans ${category}`
    } else {
      error.value = `Erreur: ${errorMessage}`
    }
    setTimeout(() => error.value = null, 5000)
    console.error('Erreur lors de la sauvegarde:', e)
  }
}

async function handleDelete(kw: ClassificationKeyword) {
  if (!confirm(`Supprimer le mot-clé "${kw.keyword}" ?`)) return

  try {
    await deleteKeyword(kw.id)
    successMessage.value = `Mot-clé "${kw.keyword}" supprimé`
    setTimeout(() => successMessage.value = null, 3000)
  } catch (e) {
    console.error('Erreur lors de la suppression:', e)
  }
}
</script>

<style scoped>
.keywords-manager {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.category-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.category-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.category-icon {
  font-size: 24px;
  color: var(--md-sys-color-on-surface-variant);
}

.category-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
  color: var(--md-sys-color-on-surface);
  flex: 1;
}

.keyword-count {
  color: var(--md-sys-color-on-surface-variant);
  background: var(--md-sys-color-surface-container-highest);
  padding: 4px 12px;
  border-radius: 12px;
}

.keywords-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-left: 32px;
}

.keyword-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--md-sys-color-surface-container);
  border: 1px solid var(--md-sys-color-outline-variant);
  border-radius: 8px;
  transition: all 0.2s ease;
}

.keyword-item:hover {
  background: var(--md-sys-color-surface-container-high);
  border-color: var(--md-sys-color-outline);
}

.keyword-content {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.keyword-text {
  font-size: 14px;
  font-weight: 500;
  color: var(--md-sys-color-on-surface);
}

.keyword-weight {
  font-size: 12px;
  padding: 4px 8px;
  border-radius: 4px;
  font-weight: 500;
}

.weight-low {
  background: var(--md-sys-color-surface-container-highest);
  color: var(--md-sys-color-on-surface-variant);
}

.weight-normal {
  background: var(--md-sys-color-primary-container);
  color: var(--md-sys-color-on-primary-container);
}

.weight-high {
  background: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
}

.weight-very-high {
  background: var(--md-sys-color-tertiary-container);
  color: var(--md-sys-color-on-tertiary-container);
}

.keyword-actions {
  display: flex;
  gap: 4px;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  padding: 0;
  background: none;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  color: var(--md-sys-color-on-surface-variant);
  transition: all 0.2s ease;
}

.btn-icon:hover {
  background: var(--md-sys-color-surface-container-highest);
  color: var(--md-sys-color-on-surface);
}

.btn-delete:hover {
  background: var(--md-sys-color-error-container);
  color: var(--md-sys-color-error);
}

.btn-icon .material-icons {
  font-size: 18px;
}

.add-keyword-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: none;
  border: 1px dashed var(--md-sys-color-outline);
  border-radius: 8px;
  font-size: 14px;
  color: var(--md-sys-color-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.add-keyword-btn:hover {
  background: var(--md-sys-color-primary-container);
  border-color: var(--md-sys-color-primary);
}

.add-keyword-btn .material-icons {
  font-size: 18px;
}

.keyword-form {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: var(--md-sys-color-surface-container);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: 8px;
}

.input-field {
  flex: 1;
  padding: 10px 12px;
  background: var(--md-sys-color-surface);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: 8px;
  font-size: 14px;
  color: var(--md-sys-color-on-surface);
  outline: none;
  transition: border-color 0.2s ease;
}

.input-field:focus {
  border-color: var(--md-sys-color-primary);
}

.weight-select {
  padding: 10px 12px;
  background: var(--md-sys-color-surface);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: 8px;
  font-size: 14px;
  color: var(--md-sys-color-on-surface);
  cursor: pointer;
  min-width: 140px;
}

.weight-select:focus {
  outline: 2px solid var(--md-sys-color-primary);
  outline-offset: 2px;
}

.btn-primary,
.btn-secondary {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  padding: 0;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary {
  background: var(--md-sys-color-primary);
  color: var(--md-sys-color-on-primary);
}

.btn-primary:hover {
  background: var(--md-sys-color-primary-container);
  color: var(--md-sys-color-on-primary-container);
}

.btn-secondary {
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface-variant);
}

.btn-secondary:hover {
  background: var(--md-sys-color-surface-container-high);
}

.btn-primary .material-icons,
.btn-secondary .material-icons {
  font-size: 18px;
}

.error-message {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: var(--md-sys-color-error-container);
  border-radius: 8px;
  border-left: 4px solid var(--md-sys-color-error);
  color: var(--md-sys-color-on-error-container);
  font-size: 14px;
  font-weight: 500;
  box-shadow: var(--md-sys-elevation-level2);
}

.error-message .material-icons {
  color: var(--md-sys-color-error);
  font-size: 20px;
}

.success-message {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: var(--md-sys-color-tertiary-container);
  border-radius: 8px;
  border-left: 4px solid var(--md-sys-color-tertiary);
  color: var(--md-sys-color-on-tertiary-container);
  font-size: 14px;
  font-weight: 500;
  box-shadow: var(--md-sys-elevation-level2);
}

.success-message .material-icons {
  color: var(--md-sys-color-tertiary);
  font-size: 20px;
}
</style>
