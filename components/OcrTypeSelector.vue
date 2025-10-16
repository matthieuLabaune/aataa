<template>
  <div class="ocr-selector">
    <label class="label-medium on-surface">
      Type d'analyse :
    </label>
    <div class="ocr-options">
      <button
        v-for="option in ocrOptions"
        :key="option.value"
        :class="['ocr-option', { active: modelValue === option.value }]"
        @click="$emit('update:modelValue', option.value)"
      >
        <span class="ocr-icon">{{ option.icon }}</span>
        <div class="ocr-content">
          <span class="ocr-title title-small">{{ option.label }}</span>
          <span class="ocr-description label-small">{{ option.description }}</span>
        </div>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
export type OcrType = 'standard' | 'handwritten' | 'printed-ml' | 'caption'

interface OcrOption {
  value: OcrType
  label: string
  description: string
  icon: string
}

const props = defineProps<{
  modelValue: OcrType
}>()

defineEmits<{
  'update:modelValue': [value: OcrType]
}>()

const ocrOptions: OcrOption[] = [
  {
    value: 'standard',
    label: 'Standard (Tesseract)',
    description: 'Rapide • Texte imprimé • Multilingue',
    icon: '📄'
  },
  {
    value: 'handwritten',
    label: 'Manuscrit (TrOCR)',
    description: 'Écriture manuelle • Plus lent • IA',
    icon: '🖊️'
  },
  {
    value: 'printed-ml',
    label: 'Imprimé ML (TrOCR)',
    description: 'Texte imprimé • IA • Haute précision',
    icon: '📰'
  },
  {
    value: 'caption',
    label: 'Description (BLIP)',
    description: 'Génère une description de l\'image',
    icon: '🎨'
  }
]
</script>

<style scoped>
.ocr-selector {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.ocr-options {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 12px;
}

.ocr-option {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 16px;
  background-color: var(--md-sys-color-surface-container);
  border: 2px solid var(--md-sys-color-outline-variant);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  text-align: left;
}

.ocr-option:hover {
  background-color: var(--md-sys-color-surface-container-high);
  border-color: var(--md-sys-color-outline);
  transform: translateY(-2px);
  box-shadow: var(--md-sys-elevation-1);
}

.ocr-option.active {
  background-color: var(--md-sys-color-primary-container);
  border-color: var(--md-sys-color-primary);
  color: var(--md-sys-color-on-primary-container);
}

.ocr-icon {
  font-size: 32px;
  line-height: 1;
  flex-shrink: 0;
}

.ocr-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.ocr-title {
  font-weight: 500;
  color: var(--md-sys-color-on-surface);
}

.ocr-option.active .ocr-title {
  color: var(--md-sys-color-on-primary-container);
}

.ocr-description {
  color: var(--md-sys-color-on-surface-variant);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.ocr-option.active .ocr-description {
  color: var(--md-sys-color-on-primary-container);
  opacity: 0.8;
}

/* Mobile: Stack vertically */
@media (max-width: 767px) {
  .ocr-options {
    grid-template-columns: 1fr;
  }

  .ocr-option {
    padding: 12px;
  }

  .ocr-icon {
    font-size: 24px;
  }
}
</style>
