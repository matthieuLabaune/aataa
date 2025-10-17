<template>
  <div v-if="items.length > 0" class="processing-indicator">
    <div v-for="item in items" :key="item.id" class="processing-item" :class="item.status">
      <div class="item-header">
        <span class="material-icons">
          {{ item.type === 'folder' ? 'folder' : 'description' }}
        </span>
        <span class="item-name">{{ item.name }}</span>
        <span class="material-icons status-icon">
          {{ getStatusIcon(item.status) }}
        </span>
      </div>
      
      <div v-if="item.status === 'processing'" class="progress-bar">
        <div class="progress-fill" :style="{ width: item.progress + '%' }"></div>
      </div>
      
      <div v-if="item.status === 'error' && item.error" class="error-message">
        {{ item.error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useProcessingState } from '../composables/useProcessingState'

const { getProcessingItems } = useProcessingState()

const items = computed(() => getProcessingItems())

const getStatusIcon = (status: string) => {
  switch (status) {
    case 'processing':
      return 'hourglass_empty'
    case 'completed':
      return 'check_circle'
    case 'error':
      return 'error'
    default:
      return 'help'
  }
}
</script>

<style scoped>
.processing-indicator {
  position: fixed;
  bottom: 20px;
  right: 20px;
  z-index: 1000;
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-width: 400px;
}

.processing-item {
  background: var(--md-sys-color-surface);
  border: 1px solid var(--md-sys-color-outline);
  border-radius: 12px;
  padding: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  animation: slideIn 0.3s ease-out;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(100%);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.processing-item.completed {
  border-color: var(--md-sys-color-primary);
  background: var(--md-sys-color-primary-container);
}

.processing-item.error {
  border-color: var(--md-sys-color-error);
  background: var(--md-sys-color-error-container);
}

.item-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.item-header .material-icons {
  color: var(--md-sys-color-on-surface);
  font-size: 20px;
}

.item-name {
  flex: 1;
  font-size: 14px;
  font-weight: 500;
  color: var(--md-sys-color-on-surface);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.status-icon {
  font-size: 20px;
}

.processing .status-icon {
  color: var(--md-sys-color-primary);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.completed .status-icon {
  color: var(--md-sys-color-primary);
}

.error .status-icon {
  color: var(--md-sys-color-error);
}

.progress-bar {
  height: 4px;
  background: var(--md-sys-color-surface-variant);
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--md-sys-color-primary);
  transition: width 0.3s ease;
}

.error-message {
  margin-top: 8px;
  font-size: 12px;
  color: var(--md-sys-color-error);
}
</style>
