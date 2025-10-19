<template>
  <div class="subcategory-manager">
    <div class="category-section" v-for="category in categories" :key="category">
      <div class="category-header">
        <span class="material-icons category-icon">{{ getCategoryIcon(category) }}</span>
        <h3>{{ category }}</h3>
      </div>

      <div class="subcategories-list">
        <!-- Sous-catégories existantes -->
        <div
          v-for="sub in getSubcategoriesFor(category)"
          :key="sub.id"
          class="subcategory-chip"
          :class="{ 'predefined': sub.is_predefined }"
        >
          <span>{{ sub.name }}</span>
          <button
            v-if="!sub.is_predefined"
            @click="handleDelete(sub)"
            class="delete-btn"
            :aria-label="`Supprimer ${sub.name}`"
          >
            <span class="material-icons">close</span>
          </button>
        </div>

        <!-- Bouton d'ajout -->
        <button
          v-if="!isAdding[category]"
          @click="startAdding(category)"
          class="add-subcategory-btn"
        >
          <span class="material-icons">add</span>
          Ajouter
        </button>

        <!-- Formulaire d'ajout -->
        <div v-else class="add-form">
          <input
            v-model="newSubcategoryName"
            @keyup.enter="handleAdd(category)"
            @keyup.escape="cancelAdding"
            type="text"
            placeholder="Nom de la sous-catégorie"
            class="input-field"
            ref="inputRef"
            autofocus
          />
          <button @click="handleAdd(category)" class="btn-primary">
            <span class="material-icons">check</span>
          </button>
          <button @click="cancelAdding" class="btn-secondary">
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import { useCategories } from '../composables/useCategories'
import { CATEGORY_ICONS, type MainCategory, type Subcategory } from '../types/document'

const {
  categories,
  subcategories,
  error,
  loadCategories,
  loadSubcategories,
  addSubcategory,
  deleteSubcategory,
} = useCategories()

const isAdding = ref<Record<MainCategory, boolean>>({} as Record<MainCategory, boolean>)
const newSubcategoryName = ref('')
const inputRef = ref<HTMLInputElement | null>(null)

onMounted(async () => {
  await loadCategories()
  await loadSubcategories()
})

function getCategoryIcon(category: MainCategory) {
  return CATEGORY_ICONS[category] || 'folder'
}

function getSubcategoriesFor(category: MainCategory) {
  return subcategories.value.filter(sub => sub.category === category)
}

function startAdding(category: MainCategory) {
  isAdding.value[category] = true
  newSubcategoryName.value = ''
  nextTick(() => {
    inputRef.value?.focus()
  })
}

function cancelAdding() {
  Object.keys(isAdding.value).forEach(key => {
    isAdding.value[key as MainCategory] = false
  })
  newSubcategoryName.value = ''
}

async function handleAdd(category: MainCategory) {
  const trimmedName = newSubcategoryName.value.trim()
  if (!trimmedName) return

  // Vérifier si la sous-catégorie existe déjà
  const existing = getSubcategoriesFor(category).find(
    sub => sub.name.toLowerCase() === trimmedName.toLowerCase()
  )

  if (existing) {
    error.value = `La sous-catégorie "${trimmedName}" existe déjà dans ${category}`
    setTimeout(() => error.value = null, 3000)
    return
  }

  try {
    await addSubcategory(category, trimmedName)
    cancelAdding()
  } catch (e) {
    const errorMessage = e instanceof Error ? e.message : String(e)
    if (errorMessage.includes('UNIQUE constraint')) {
      error.value = `La sous-catégorie "${trimmedName}" existe déjà`
    } else {
      error.value = `Erreur: ${errorMessage}`
    }
    setTimeout(() => error.value = null, 5000)
    console.error('Erreur lors de l\'ajout:', e)
  }
}

async function handleDelete(sub: Subcategory) {
  if (!confirm(`Supprimer "${sub.name}" ?`)) return

  try {
    await deleteSubcategory(sub.id)
  } catch (e) {
    console.error('Erreur lors de la suppression:', e)
  }
}
</script>

<style scoped>
.subcategory-manager {
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
}

.subcategories-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding-left: 32px;
}

.subcategory-chip {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  background: var(--md-sys-color-surface-container-high);
  border: 1px solid var(--md-sys-color-outline-variant);
  border-radius: 8px;
  font-size: 14px;
  color: var(--md-sys-color-on-surface);
  transition: all 0.2s ease;
}

.subcategory-chip.predefined {
  background: var(--md-sys-color-surface-container-highest);
  border-color: var(--md-sys-color-outline);
}

.subcategory-chip:hover {
  background: var(--md-sys-color-surface-container-highest);
}

.delete-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  padding: 0;
  background: none;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  color: var(--md-sys-color-on-surface-variant);
  transition: all 0.2s ease;
}

.delete-btn:hover {
  background: var(--md-sys-color-error-container);
  color: var(--md-sys-color-error);
}

.delete-btn .material-icons {
  font-size: 16px;
}

.add-subcategory-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 12px;
  background: none;
  border: 1px dashed var(--md-sys-color-outline);
  border-radius: 8px;
  font-size: 14px;
  color: var(--md-sys-color-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.add-subcategory-btn:hover {
  background: var(--md-sys-color-primary-container);
  border-color: var(--md-sys-color-primary);
}

.add-subcategory-btn .material-icons {
  font-size: 18px;
}

.add-form {
  display: flex;
  align-items: center;
  gap: 8px;
}

.input-field {
  flex: 1;
  padding: 8px 12px;
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

.btn-primary,
.btn-secondary {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
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
</style>
