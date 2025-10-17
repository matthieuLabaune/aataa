import { ref, computed } from 'vue'
import type { MainCategory, Subcategory, Tag } from '../types/document'
import * as categoriesApi from '../api/categories'

const categories = ref<MainCategory[]>([])
const subcategories = ref<Subcategory[]>([])
const tags = ref<Tag[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

export function useCategories() {
    /**
     * Charge toutes les catégories principales
     */
    async function loadCategories() {
        loading.value = true
        error.value = null
        try {
            categories.value = await categoriesApi.getMainCategories()
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors du chargement des catégories:', e)
        } finally {
            loading.value = false
        }
    }

    /**
     * Charge les sous-catégories (optionnellement filtrées)
     */
    async function loadSubcategories(category?: MainCategory) {
        loading.value = true
        error.value = null
        try {
            subcategories.value = await categoriesApi.getSubcategories(category)
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors du chargement des sous-catégories:', e)
        } finally {
            loading.value = false
        }
    }

    /**
     * Charge tous les tags
     */
    async function loadTags() {
        loading.value = true
        error.value = null
        try {
            tags.value = await categoriesApi.getAllTags()
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors du chargement des tags:', e)
        } finally {
            loading.value = false
        }
    }

    /**
     * Ajoute une nouvelle sous-catégorie
     */
    async function addSubcategory(category: MainCategory, name: string) {
        loading.value = true
        error.value = null
        try {
            const id = await categoriesApi.addSubcategory(category, name)
            // Recharge les sous-catégories
            await loadSubcategories()
            return id
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors de l\'ajout de la sous-catégorie:', e)
            throw e
        } finally {
            loading.value = false
        }
    }

    /**
     * Supprime une sous-catégorie
     */
    async function deleteSubcategory(id: number) {
        loading.value = true
        error.value = null
        try {
            await categoriesApi.deleteSubcategory(id)
            // Recharge les sous-catégories
            await loadSubcategories()
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors de la suppression de la sous-catégorie:', e)
            throw e
        } finally {
            loading.value = false
        }
    }

    /**
     * Obtient les sous-catégories d'une catégorie spécifique
     */
    const getSubcategoriesForCategory = computed(() => {
        return (category: MainCategory) => {
            return subcategories.value.filter(sub => sub.category === category)
        }
    })

    /**
     * Obtient les sous-catégories prédéfinies
     */
    const predefinedSubcategories = computed(() => {
        return subcategories.value.filter(sub => sub.is_predefined)
    })

    /**
     * Obtient les sous-catégories créées par l'utilisateur
     */
    const userSubcategories = computed(() => {
        return subcategories.value.filter(sub => !sub.is_predefined)
    })

    /**
     * Tags les plus utilisés (top 10)
     */
    const topTags = computed(() => {
        return tags.value.slice(0, 10)
    })

    return {
        // State
        categories,
        subcategories,
        tags,
        loading,
        error,
        
        // Computed
        getSubcategoriesForCategory,
        predefinedSubcategories,
        userSubcategories,
        topTags,
        
        // Actions
        loadCategories,
        loadSubcategories,
        loadTags,
        addSubcategory,
        deleteSubcategory,
    }
}
