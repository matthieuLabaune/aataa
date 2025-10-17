export interface Document {
    id: string
    original_name: string
    new_name: string
    file_path: string
    document_type: string
    category: string              // NEW: Catégorie principale
    subcategory?: string | null   // NEW: Sous-catégorie
    tags: string[]
    ocr_text: string
    created_at: string
    file_size: number
    notes?: string | null
    deleted_at?: string | null
}

export interface DocumentStats {
    total: number
    types: number
    tags: number
    size: number
}

// ===== NOUVEAUX TYPES POUR LE SYSTÈME DE CATÉGORIES =====

export type MainCategory = 
    | 'Administratif'
    | 'Financier'
    | 'Santé'
    | 'Professionnel'
    | 'Immobilier'
    | 'Académique'
    | 'Personnel'
    | 'Autre'

export interface Subcategory {
    id: number
    category: MainCategory
    name: string
    is_predefined: boolean
}

export interface Tag {
    name: string
    count: number
}

export interface ClassificationResult {
    category: MainCategory
    subcategory?: string | null
    suggested_tags: string[]
    confidence: number
}

export interface CategoryInfo {
    name: MainCategory
    icon: string
    prefix: string
}

export const CATEGORY_ICONS: Record<MainCategory, string> = {
    'Administratif': 'description',
    'Financier': 'account_balance',
    'Santé': 'medical_services',
    'Professionnel': 'work',
    'Immobilier': 'home',
    'Académique': 'school',
    'Personnel': 'person',
    'Autre': 'folder',
}

export const CATEGORY_COLORS: Record<MainCategory, string> = {
    'Administratif': '#757575',  // Gray
    'Financier': '#000000',      // Black
    'Santé': '#D32F2F',          // Red
    'Professionnel': '#424242',  // Dark gray
    'Immobilier': '#616161',     // Medium gray
    'Académique': '#212121',     // Almost black
    'Personnel': '#9E9E9E',      // Light gray
    'Autre': '#BDBDBD',          // Very light gray
}
