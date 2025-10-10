<template>
  <nav class="navigation-rail surface-variant elevation-1">
    <div class="rail-content">
      <div
        v-for="item in navItems"
        :key="item.id"
        class="nav-rail-item ripple state-layer"
        :class="{ active: activeItem === item.id }"
        @click="selectItem(item.id)"
        :aria-label="item.label"
      >
        <div class="nav-icon">{{ item.icon }}</div>
        <span class="nav-label label-small">{{ item.label }}</span>
        <span v-if="item.badge" class="nav-badge badge-primary">{{ item.badge }}</span>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { useRouter } from '#app'

const router = useRouter()
const { activeFilters } = useDocumentFilters()
const activeItem = ref('types')

const navItems = computed(() => [
  {
    id: 'types',
    icon: '📂',
    label: 'Types',
    badge: 6
  },
  {
    id: 'dashboard',
    icon: '📊',
    label: 'Stats',
  },
  {
    id: 'tags',
    icon: '🏷️',
    label: 'Tags',
    badge: 12
  },
  {
    id: 'years',
    icon: '📅',
    label: 'Années',
  },
  {
    id: 'settings',
    icon: '⚙️',
    label: 'Config',
  },
])

function selectItem(id: string) {
  activeItem.value = id
  
  // Navigate to appropriate view/filter
  if (id === 'types') {
    router.push('/types')
  } else if (id === 'dashboard') {
    // TODO: Navigate to dashboard view
    console.log('Navigate to dashboard')
  } else if (id === 'tags') {
    // TODO: Navigate to tags view
    console.log('Navigate to tags view')
  } else if (id === 'years') {
    // TODO: Navigate to years view
    console.log('Navigate to years view')
  } else if (id === 'settings') {
    // TODO: Open settings modal
    console.log('Open settings')
  }
}
</script>

<style scoped>
.navigation-rail {
  display: flex;
  flex-direction: column;
  width: 80px;
  background-color: var(--md-sys-color-surface-variant);
  border-right: 1px solid var(--md-sys-color-outline-variant);
}

.rail-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 12px 8px;
}

.nav-rail-item {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 12px 8px;
  border-radius: var(--md-sys-shape-corner-medium);
  cursor: pointer;
  transition: all var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
  color: var(--md-sys-color-on-surface-variant);
}

.nav-rail-item:hover {
  background-color: var(--md-sys-color-surface-container-high);
}

.nav-rail-item.active {
  background-color: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
}

.nav-rail-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 32px;
  background-color: var(--md-sys-color-primary);
  border-radius: 0 var(--md-sys-shape-corner-small) var(--md-sys-shape-corner-small) 0;
}

.nav-icon {
  font-size: 24px;
  line-height: 1;
}

.nav-label {
  text-align: center;
  font-size: 11px;
  font-weight: 500;
}

.nav-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  min-width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  font-weight: 600;
}

/* Tablet: Slightly smaller */
@media (min-width: 768px) and (max-width: 1199px) {
  .navigation-rail {
    width: 72px;
  }

  .nav-label {
    font-size: 10px;
  }
}
</style>
