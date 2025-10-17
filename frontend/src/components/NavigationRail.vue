<template>
  <nav class="navigation-rail">
    <div class="rail-header">
      <div class="app-logo">
        <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
          <rect width="32" height="32" rx="8" :fill="currentColor" />
          <path d="M10 12L16 18L22 12" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M10 18L16 24L22 18" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
    </div>

    <div class="rail-destinations">
      <router-link to="/" class="rail-item" :class="{ active: $route.path === '/' }">
        <div class="rail-item-container">
          <div class="rail-item-indicator"></div>
          <div class="rail-item-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <polyline points="9 22 9 12 15 12 15 22" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <span class="rail-item-label">Accueil</span>
        </div>
      </router-link>

      <router-link to="/trash" class="rail-item" :class="{ active: $route.path === '/trash' }">
        <div class="rail-item-container">
          <div class="rail-item-indicator"></div>
          <div class="rail-item-icon">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <polyline points="3 6 5 6 21 6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span v-if="deletedCount > 0" class="md-badge">{{ deletedCount }}</span>
          </div>
          <span class="rail-item-label">Corbeille</span>
        </div>
      </router-link>
    </div>

    <div class="rail-footer">
      <router-link to="/settings" class="md-icon-button" title="Paramètres">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <circle cx="12" cy="12" r="3" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M12 1v6m0 6v6m5.196-13.804l-4.243 4.243m0 5.122l-4.243 4.243M23 12h-6m-6 0H5m13.804 5.196l-4.243-4.243m0-5.122l-4.243-4.243" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </router-link>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const deletedCount = ref(0);
const currentColor = 'var(--md-sys-color-primary)';

const loadDeletedCount = async () => {
  try {
    const documents = await invoke('get_deleted_documents');
    deletedCount.value = Array.isArray(documents) ? documents.length : 0;
  } catch (error) {
    console.error('Erreur lors du chargement des documents supprimés:', error);
  }
};

onMounted(() => {
  loadDeletedCount();
  // Refresh count every 30 seconds
  setInterval(loadDeletedCount, 30000);
});
</script>

<style scoped>
.navigation-rail {
  position: fixed;
  left: 0;
  top: 0;
  bottom: 0;
  width: var(--md-comp-navigation-rail-container-width);
  background-color: var(--md-sys-color-surface-container);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: var(--md-sys-spacing-md) 0;
  z-index: var(--md-sys-z-index-navigation);
  border-right: 1px solid var(--md-sys-color-outline-variant);
}

.rail-header {
  padding: var(--md-sys-spacing-md) 0;
  margin-bottom: var(--md-sys-spacing-lg);
}

.app-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: transform var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.app-logo:hover {
  transform: scale(1.1);
}

.rail-destinations {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-xs);
  width: 100%;
  padding: 0 var(--md-sys-spacing-xs);
}

.rail-item {
  text-decoration: none;
  color: var(--md-sys-color-on-surface-variant);
  display: flex;
  flex-direction: column;
  align-items: center;
  border-radius: var(--md-sys-shape-corner-large);
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
  position: relative;
}

.rail-item-container {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  padding: var(--md-sys-spacing-sm) 0;
}

.rail-item-indicator {
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 0;
  height: 32px;
  background-color: var(--md-sys-color-secondary-container);
  border-radius: 0 var(--md-sys-shape-corner-large) var(--md-sys-shape-corner-large) 0;
  transition: width var(--md-sys-motion-duration-medium2) var(--md-sys-motion-easing-emphasized);
}

.rail-item.active .rail-item-indicator {
  width: 4px;
}

.rail-item-icon {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 56px;
  height: 32px;
  border-radius: var(--md-sys-shape-corner-large);
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.rail-item:hover .rail-item-icon {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 8%, transparent);
}

.rail-item.active {
  color: var(--md-sys-color-on-secondary-container);
}

.rail-item.active .rail-item-icon {
  background-color: var(--md-sys-color-secondary-container);
}

.rail-item-icon .md-badge {
  position: absolute;
  top: -4px;
  right: 8px;
  animation: scaleIn var(--md-sys-motion-duration-medium2) var(--md-sys-motion-easing-emphasized);
}

.rail-item-label {
  margin-top: var(--md-sys-spacing-xs);
  font-size: var(--md-sys-typescale-label-medium-font-size);
  font-weight: var(--md-sys-typescale-label-medium-font-weight);
  text-align: center;
}

.rail-footer {
  padding: var(--md-sys-spacing-md) 0;
}
</style>
