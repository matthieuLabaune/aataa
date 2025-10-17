import { invoke } from '@tauri-apps/api/core'
import type { MainCategory, Subcategory, Tag } from '../types/document'

/**
 * Récupère toutes les catégories principales
 */
export async function getMainCategories(): Promise<MainCategory[]> {
    return await invoke<MainCategory[]>('get_main_categories')
}

/**
 * Récupère les sous-catégories (optionnellement filtrées par catégorie)
 */
export async function getSubcategories(category?: MainCategory): Promise<Subcategory[]> {
    return await invoke<Subcategory[]>('get_subcategories', { category })
}

/**
 * Ajoute une nouvelle sous-catégorie
 */
export async function addSubcategory(category: MainCategory, name: string): Promise<number> {
    return await invoke<number>('add_subcategory', { category, name })
}

/**
 * Supprime une sous-catégorie (uniquement celles créées par l'utilisateur)
 */
export async function deleteSubcategory(id: number): Promise<void> {
    await invoke('delete_subcategory', { id })
}

/**
 * Récupère tous les tags avec leur nombre d'occurrences
 */
export async function getAllTags(): Promise<Tag[]> {
    return await invoke<Tag[]>('get_all_tags')
}
