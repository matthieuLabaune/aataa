<template>
  <nav class="navigation-rail">
    <div class="rail-header">
      <button class="app-logo" @click="showAbout = true" title="À propos de PaperVault">
        <span class="material-icons logo-icon">lock</span>
      </button>
    </div>

    <div class="rail-destinations">
      <router-link to="/" class="rail-item" :class="{ active: $route.path === '/' }">
        <div class="rail-item-container">
          <div class="rail-item-indicator"></div>
          <div class="rail-item-icon">
            <span class="material-icons">home</span>
          </div>
          <span class="rail-item-label">Accueil</span>
        </div>
      </router-link>

      <router-link to="/explorer" class="rail-item" :class="{ active: $route.path === '/explorer' }">
        <div class="rail-item-container">
          <div class="rail-item-indicator"></div>
          <div class="rail-item-icon">
            <span class="material-icons">folder_open</span>
          </div>
          <span class="rail-item-label">Explorateur</span>
        </div>
      </router-link>

      <router-link to="/trash" class="rail-item" :class="{ active: $route.path === '/trash' }">
        <div class="rail-item-container">
          <div class="rail-item-indicator"></div>
          <div class="rail-item-icon">
            <span class="material-icons">delete</span>
            <span v-if="deletedCount > 0" class="md-badge">{{ deletedCount }}</span>
          </div>
          <span class="rail-item-label">Corbeille</span>
        </div>
      </router-link>
    </div>

    <div class="rail-footer">
      <router-link to="/settings" class="md-icon-button" title="Paramètres">
        <span class="material-icons">settings</span>
      </router-link>
    </div>

    <!-- About Modal -->
    <div v-if="showAbout" class="md-modal-overlay" @click="showAbout = false">
      <div class="md-modal about-modal animate-scale-in" @click.stop>
        <div class="md-modal-header">
          <div class="about-header">
            <span class="material-icons about-icon">lock</span>
            <h2 class="title-large">PaperVault</h2>
          </div>
          <button @click="showAbout = false" class="md-icon-button">
            <span class="material-icons">close</span>
          </button>
        </div>

        <div class="md-modal-content about-content">
          <div class="about-section">
            <p class="body-large about-tagline">Coffre-fort documentaire intelligent</p>
          </div>

          <div class="about-info-grid">
            <div class="info-item">
              <span class="label-medium">Version</span>
              <span class="body-large">1.0.0</span>
            </div>
            <div class="info-item">
              <span class="label-medium">Éditeur</span>
              <span class="body-large">-</span>
            </div>
            <div class="info-item">
              <span class="label-medium">Licence</span>
              <span class="body-large">Gratuite</span>
            </div>
            <div class="info-item">
              <span class="label-medium">Numéro client</span>
              <span class="body-large">-</span>
            </div>
            <div class="info-item">
              <span class="label-medium">Numéro de série</span>
              <span class="body-large">-</span>
            </div>
            <div class="info-item">
              <span class="label-medium">Type</span>
              <span class="body-large">Version gratuite</span>
            </div>
          </div>

          <div class="about-section">
            <p class="body-small about-copyright">© 2025 PaperVault. Tous droits réservés.</p>
          </div>
        </div>

        <div class="md-modal-footer">
          <button @click="showAbout = false" class="md-filled-button">
            Fermer
          </button>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const deletedCount = ref(0);
const showAbout = ref(false);

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
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--md-sys-spacing-xs);
}

.app-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  background-color: var(--md-sys-color-primary);
  border-radius: var(--md-sys-shape-corner-medium);
  cursor: pointer;
  transition: transform var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.logo-icon {
  font-size: 28px;
  color: var(--md-sys-color-on-primary);
}

.app-name {
  color: var(--md-sys-color-on-surface-variant);
  font-weight: 500;
  text-align: center;
}

.rail-item-icon .material-icons {
  font-size: 24px;
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

/* About Modal */
.about-modal {
  max-width: 500px;
  width: 90%;
}

.about-header {
  display: flex;
  align-items: center;
  gap: var(--md-sys-spacing-md);
}

.about-icon {
  font-size: 32px;
  color: var(--md-sys-color-primary);
}

.about-content {
  padding: var(--md-sys-spacing-xl) var(--md-sys-spacing-lg);
}

.about-section {
  text-align: center;
  margin-bottom: var(--md-sys-spacing-lg);
}

.about-tagline {
  color: var(--md-sys-color-on-surface-variant);
  margin: 0;
}

.about-info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--md-sys-spacing-lg);
  margin: var(--md-sys-spacing-xl) 0;
  padding: var(--md-sys-spacing-lg);
  background-color: var(--md-sys-color-surface-container);
  border-radius: var(--md-sys-shape-corner-medium);
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: var(--md-sys-spacing-xs);
}

.info-item .label-medium {
  color: var(--md-sys-color-on-surface-variant);
  text-transform: uppercase;
  font-size: 11px;
  letter-spacing: 0.5px;
}

.info-item .body-large {
  color: var(--md-sys-color-on-surface);
  font-weight: 500;
}

.about-copyright {
  color: var(--md-sys-color-on-surface-variant);
  margin: 0;
  opacity: 0.7;
}
</style>
