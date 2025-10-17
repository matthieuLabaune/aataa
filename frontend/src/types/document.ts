export interface Document {
  id: string
  original_name: string
  new_name: string
  file_path: string
  document_type: string
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
