import { invoke } from '@tauri-apps/api/core'

export interface ClassificationKeyword {
    id: number
    category: string
    subcategory: string | null
    keyword: string
    weight: number
}

/**
 * Récupère les mots-clés de classification (optionnellement filtrés par catégorie)
 */
export async function getClassificationKeywords(category?: string): Promise<ClassificationKeyword[]> {
    return await invoke<ClassificationKeyword[]>('get_classification_keywords', { category })
}

/**
 * Ajoute un nouveau mot-clé de classification
 */
export async function addClassificationKeyword(
    category: string,
    subcategory: string | null,
    keyword: string,
    weight: number
): Promise<number> {
    return await invoke<number>('add_classification_keyword', {
        category,
        subcategory,
        keyword,
        weight,
    })
}

/**
 * Met à jour un mot-clé de classification existant
 */
export async function updateClassificationKeyword(
    id: number,
    keyword: string,
    weight: number
): Promise<void> {
    await invoke('update_classification_keyword', { id, keyword, weight })
}

/**
 * Supprime un mot-clé de classification
 */
export async function deleteClassificationKeyword(id: number): Promise<void> {
    await invoke('delete_classification_keyword', { id })
}
