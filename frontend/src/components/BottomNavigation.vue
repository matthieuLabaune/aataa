<template>
  <nav class="bottom-navigation">
    <router-link to="/" class="nav-item" :class="{ active: $route.path === '/' }">
      <div class="nav-item-container">
        <div class="nav-item-indicator"></div>
        <div class="nav-item-icon">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <polyline points="9 22 9 12 15 12 15 22" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <span class="nav-item-label">Accueil</span>
      </div>
    </router-link>

    <router-link to="/trash" class="nav-item" :class="{ active: $route.path === '/trash' }">
      <div class="nav-item-container">
        <div class="nav-item-indicator"></div>
        <div class="nav-item-icon">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <polyline points="3 6 5 6 21 6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span v-if="deletedCount > 0" class="md-badge">{{ deletedCount }}</span>
        </div>
        <span class="nav-item-label">Corbeille</span>
      </div>
    </router-link>

    <router-link to="/settings" class="nav-item" :class="{ active: $route.path === '/settings' }">
      <div class="nav-item-container">
        <div class="nav-item-indicator"></div>
        <div class="nav-item-icon">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <circle cx="12" cy="12" r="3" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M12 1v6m0 6v6m5.196-13.804l-4.243 4.243m0 5.122l-4.243 4.243M23 12h-6m-6 0H5m13.804 5.196l-4.243-4.243m0-5.122l-4.243-4.243" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <span class="nav-item-label">Réglages</span>
      </div>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const deletedCount = ref(0);

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
.bottom-navigation {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: var(--md-comp-navigation-bar-container-height);
  background-color: var(--md-sys-color-surface-container);
  display: flex;
  justify-content: space-around;
  align-items: center;
  padding: var(--md-sys-spacing-sm) 0;
  z-index: var(--md-sys-z-index-navigation);
  border-top: 1px solid var(--md-sys-color-outline-variant);
  box-shadow: var(--md-sys-elevation-level2);
}

.nav-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-decoration: none;
  color: var(--md-sys-color-on-surface-variant);
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
  transition: color var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
  position: relative;
}

.nav-item-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  width: 64px;
}

.nav-item-indicator {
  position: absolute;
  top: -8px;
  left: 50%;
  transform: translateX(-50%);
  width: 0;
  height: 3px;
  background-color: var(--md-sys-color-secondary-container);
  border-radius: var(--md-sys-shape-corner-full);
  transition: width var(--md-sys-motion-duration-medium2) var(--md-sys-motion-easing-emphasized);
}

.nav-item.active .nav-item-indicator {
  width: 64px;
}

.nav-item-icon {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 32px;
  border-radius: var(--md-sys-shape-corner-large);
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.nav-item:active .nav-item-icon {
  background-color: color-mix(in srgb, var(--md-sys-color-on-surface) 12%, transparent);
}

.nav-item.active {
  color: var(--md-sys-color-on-surface);
}

.nav-item.active .nav-item-icon {
  background-color: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
}

.nav-item-icon .md-badge {
  position: absolute;
  top: -4px;
  right: 12px;
  animation: scaleIn var(--md-sys-motion-duration-medium2) var(--md-sys-motion-easing-emphasized);
}

.nav-item-label {
  margin-top: var(--md-sys-spacing-xs);
  font-size: var(--md-sys-typescale-label-medium-font-size);
  font-weight: var(--md-sys-typescale-label-medium-font-weight);
  text-align: center;
}
</style>
