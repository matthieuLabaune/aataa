<template>
  <header class="top-app-bar surface-container elevation-2">
    <div class="app-bar-content">
      <!-- Logo & Title -->
      <div class="app-bar-leading">
        <div class="app-logo">📚</div>
        <h1 class="title-large app-title">AATAA</h1>
      </div>

      <!-- Actions -->
      <div class="app-bar-actions">
        <button class="icon-button ripple" aria-label="Paramètres">
          <span>⚙️</span>
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
// No props needed for now
</script>

<style scoped>
.top-app-bar {
  position: sticky;
  top: 0;
  z-index: 100;
  background-color: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
}

.app-bar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 64px;
  padding: 0 16px;
  max-width: 100%;
}

.app-bar-leading {
  display: flex;
  align-items: center;
  gap: 12px;
}

.app-logo {
  font-size: 32px;
  line-height: 1;
}

.app-title {
  margin: 0;
  font-weight: 700;
  background: linear-gradient(135deg, var(--md-sys-color-primary) 0%, var(--md-sys-color-tertiary) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.app-bar-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* Mobile: Smaller title */
@media (max-width: 767px) {
  .app-title {
    font-size: 20px;
  }

  .app-logo {
    font-size: 24px;
  }
}
</style>
