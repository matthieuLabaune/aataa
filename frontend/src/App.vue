<template>
  <div id="app" class="app-container">
    <NavigationRail v-if="!isMobile" />
    <main class="main-content">
      <router-view />
    </main>
    <BottomNavigation v-if="isMobile" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import NavigationRail from './components/NavigationRail.vue';
import BottomNavigation from './components/BottomNavigation.vue';

const isMobile = ref(false);

const checkMobile = () => {
  isMobile.value = window.innerWidth < 768;
};

onMounted(() => {
  checkMobile();
  window.addEventListener('resize', checkMobile);
});

onUnmounted(() => {
  window.removeEventListener('resize', checkMobile);
});
</script>

<style>
@import './styles/material-tokens.css';
@import './styles/material-components.css';

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Roboto', -apple-system, BlinkMacSystemFont, 'Segoe UI', Oxygen, Ubuntu, Cantarell, sans-serif;
  background: var(--md-sys-color-background);
  color: var(--md-sys-color-on-background);
  line-height: 1.6;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#app {
  min-height: 100vh;
}

.app-container {
  display: flex;
  min-height: 100vh;
}

.main-content {
  flex: 1;
  margin-left: 0;
  margin-bottom: 0;
  transition: margin-left var(--md-sys-motion-duration-medium2) var(--md-sys-motion-easing-standard);
}

@media (min-width: 768px) {
  .main-content {
    margin-left: var(--md-comp-navigation-rail-container-width);
  }
}

@media (max-width: 767px) {
  .main-content {
    margin-bottom: var(--md-comp-navigation-bar-container-height);
  }
}
</style>
