import { ref } from 'vue'

interface ProcessingItem {
  id: string
  name: string
  type: 'file' | 'folder'
  progress: number // 0-100
  status: 'processing' | 'completed' | 'error'
  error?: string
}

const processingItems = ref<Map<string, ProcessingItem>>(new Map())

export function useProcessingState() {
  const startProcessing = (id: string, name: string, type: 'file' | 'folder') => {
    processingItems.value.set(id, {
      id,
      name,
      type,
      progress: 0,
      status: 'processing'
    })
  }

  const updateProgress = (id: string, progress: number) => {
    const item = processingItems.value.get(id)
    if (item) {
      item.progress = Math.min(100, Math.max(0, progress))
      processingItems.value.set(id, item)
    }
  }

  const completeProcessing = (id: string) => {
    const item = processingItems.value.get(id)
    if (item) {
      item.progress = 100
      item.status = 'completed'
      processingItems.value.set(id, item)
      
      // Retirer après 2 secondes
      setTimeout(() => {
        processingItems.value.delete(id)
      }, 2000)
    }
  }

  const errorProcessing = (id: string, error: string) => {
    const item = processingItems.value.get(id)
    if (item) {
      item.status = 'error'
      item.error = error
      processingItems.value.set(id, item)
      
      // Retirer après 5 secondes
      setTimeout(() => {
        processingItems.value.delete(id)
      }, 5000)
    }
  }

  const removeProcessing = (id: string) => {
    processingItems.value.delete(id)
  }

  const getProcessingItems = () => {
    return Array.from(processingItems.value.values())
  }

  const isProcessing = (id: string) => {
    return processingItems.value.has(id)
  }

  const clearAll = () => {
    processingItems.value.clear()
  }

  return {
    processingItems,
    startProcessing,
    updateProgress,
    completeProcessing,
    errorProcessing,
    removeProcessing,
    getProcessingItems,
    isProcessing,
    clearAll
  }
}
