<template>
  <div class="view-toggle">
    <button
      class="toggle-button"
      :class="{ active: modelValue === 'grid' }"
      @click="$emit('update:modelValue', 'grid')"
      title="Vue en grille"
    >
      <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor">
        <rect x="2" y="2" width="6" height="6" rx="1"/>
        <rect x="12" y="2" width="6" height="6" rx="1"/>
        <rect x="2" y="12" width="6" height="6" rx="1"/>
        <rect x="12" y="12" width="6" height="6" rx="1"/>
      </svg>
    </button>
    
    <button
      class="toggle-button"
      :class="{ active: modelValue === 'list' }"
      @click="$emit('update:modelValue', 'list')"
      title="Vue en liste"
    >
      <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor">
        <rect x="2" y="3" width="16" height="3" rx="1"/>
        <rect x="2" y="8.5" width="16" height="3" rx="1"/>
        <rect x="2" y="14" width="16" height="3" rx="1"/>
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  modelValue: 'grid' | 'list'
}>()

defineEmits<{
  'update:modelValue': [value: 'grid' | 'list']
}>()
</script>

<style scoped>
.view-toggle {
  display: inline-flex;
  gap: 4px;
  padding: 4px;
  background-color: var(--md-sys-color-surface-container);
  border-radius: var(--md-sys-shape-corner-small);
}

.toggle-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border: none;
  background-color: transparent;
  color: var(--md-sys-color-on-surface-variant);
  border-radius: var(--md-sys-shape-corner-extra-small);
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
}

.toggle-button:hover {
  background-color: var(--md-sys-color-surface-container-highest);
}

.toggle-button.active {
  background-color: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
}
</style>
