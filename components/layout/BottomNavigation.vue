<template>
  <nav class="bottom-navigation surface elevation-3">
    <div
      v-for="item in navItems"
      :key="item.id"
      class="bottom-nav-item ripple"
      :class="{ active: activeItem === item.id }"
      @click="selectItem(item.id)"
      :aria-label="item.label"
    >
      <div class="nav-icon-wrapper">
        <span class="nav-icon">{{ item.icon }}</span>
        <span v-if="item.badge" class="nav-badge badge-error">{{ item.badge }}</span>
      </div>
      <span class="nav-label label-medium">{{ item.label }}</span>
    </div>
  </nav>
</template>

<script setup lang="ts">
const activeItem = ref('types')

const navItems = [
  { id: 'types', icon: '📂', label: 'Types', badge: null },
  { id: 'dashboard', icon: '📊', label: 'Stats', badge: null },
  { id: 'tags', icon: '🏷️', label: 'Tags', badge: null },
  { id: 'years', icon: '📅', label: 'Années', badge: null },
  { id: 'settings', icon: '⚙️', label: 'Config', badge: null },
]

function selectItem(id: string) {
  activeItem.value = id
}
</script>

<style scoped>
.bottom-navigation {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  justify-content: space-around;
  align-items: center;
  height: 56px;
  background-color: var(--md-sys-color-surface);
  border-top: 1px solid var(--md-sys-color-outline-variant);
  z-index: 100;
}

.bottom-nav-item {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  flex: 1;
  height: 100%;
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
  color: var(--md-sys-color-on-surface-variant);
}

.bottom-nav-item.active {
  color: var(--md-sys-color-primary);
  background-color: var(--md-sys-color-surface-container-highest);
}

.bottom-nav-item.active .nav-icon-wrapper {
  background-color: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
  border-radius: var(--md-sys-shape-corner-medium);
  padding: 4px 16px;
}

.nav-icon-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--md-sys-motion-duration-short2) var(--md-sys-motion-easing-standard);
}

.nav-icon {
  font-size: 24px;
  line-height: 1;
}

.nav-label {
  font-size: 12px;
  font-weight: 500;
}

.nav-badge {
  position: absolute;
  top: -4px;
  right: -4px;
  min-width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  font-weight: 600;
}
</style>
