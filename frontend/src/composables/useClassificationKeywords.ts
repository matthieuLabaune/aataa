import { ref } from 'vue'
import type { ClassificationKeyword } from '../api/keywords'
import * as keywordsApi from '../api/keywords'

const keywords = ref<ClassificationKeyword[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

export function useClassificationKeywords() {
    /**
     * Charge tous les mots-clés (optionnellement filtrés par catégorie)
     */
    async function loadKeywords(category?: string) {
        loading.value = true
        error.value = null
        try {
            keywords.value = await keywordsApi.getClassificationKeywords(category)
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors du chargement des mots-clés:', e)
        } finally {
            loading.value = false
        }
    }

    /**
     * Ajoute un nouveau mot-clé
     */
    async function addKeyword(
        category: string,
        subcategory: string | null,
        keyword: string,
        weight: number
    ) {
        loading.value = true
        error.value = null
        try {
            const id = await keywordsApi.addClassificationKeyword(category, subcategory, keyword, weight)
            // Recharge les mots-clés
            await loadKeywords()
            return id
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors de l\'ajout du mot-clé:', e)
            throw e
        } finally {
            loading.value = false
        }
    }

    /**
     * Met à jour un mot-clé existant
     */
    async function updateKeyword(id: number, keyword: string, weight: number) {
        loading.value = true
        error.value = null
        try {
            await keywordsApi.updateClassificationKeyword(id, keyword, weight)
            // Recharge les mots-clés
            await loadKeywords()
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors de la mise à jour du mot-clé:', e)
            throw e
        } finally {
            loading.value = false
        }
    }

    /**
     * Supprime un mot-clé
     */
    async function deleteKeyword(id: number) {
        loading.value = true
        error.value = null
        try {
            await keywordsApi.deleteClassificationKeyword(id)
            // Recharge les mots-clés
            await loadKeywords()
        } catch (e) {
            error.value = e instanceof Error ? e.message : String(e)
            console.error('Erreur lors de la suppression du mot-clé:', e)
            throw e
        } finally {
            loading.value = false
        }
    }

    /**
     * Obtient les mots-clés d'une catégorie spécifique
     */
    function getKeywordsForCategory(category: string) {
        return keywords.value.filter(kw => kw.category === category)
    }

    return {
        // State
        keywords,
        loading,
        error,
        
        // Actions
        loadKeywords,
        addKeyword,
        updateKeyword,
        deleteKeyword,
        getKeywordsForCategory,
    }
}
