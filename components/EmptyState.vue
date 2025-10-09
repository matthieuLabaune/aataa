<template>
  <div class="card empty-state">
    <div class="empty-state-icon">
      {{ icon }}
    </div>
    <h3 style="font-size: 1.125rem; font-weight: 600; margin-bottom: 0.5rem">
      {{ title }}
    </h3>
    <p class="text-gray-600">
      {{ description }}
    </p>
    <button
      v-if="action"
      class="btn btn-primary"
      style="margin-top: 1rem"
      @click="action"
    >
      {{ actionText }}
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  icon: string
  title: string
  description: string
  action?: () => void
  actionText?: string
}>()
</script>
