<template>
  <div class="app-layout surface">
    <TopAppBar />

    <div class="app-content">
      <!-- Navigation Rail (Desktop & Tablet only) -->
      <NavigationRail v-if="!isMobile" />

      <!-- Main Content Area -->
      <main class="content-pane">
        <slot />
      </main>

      <!-- Supporting Pane (Desktop only, when document is selected) -->
      <SupportingPane v-if="isDesktop && selectedDocument" />
    </div>

    <!-- Bottom Navigation (Mobile only) -->
    <BottomNavigation v-if="isMobile" />

    <!-- Mobile Preview Modal (Mobile & Tablet when document is selected) -->
    <MobilePreviewModal
      v-if="!isDesktop"
      v-model="showMobilePreview"
      :document="selectedDocument"
      @open="handleOpenDocument"
      @edit="handleEditDocument"
      @delete="handleDeleteDocument"
      @updateNotes="handleUpdateNotes"
    />
  </div>
</template>

<script setup lang="ts">
import type { Document } from '~/types/document'

const selectedDocument = useState<Document | null>('selectedDocument', () => null)
const showMobilePreview = ref(false)

// Watch for document selection changes on mobile/tablet
watch(selectedDocument, (newDoc) => {
  if (newDoc && !isDesktop.value) {
    showMobilePreview.value = true
  }
})

// Close modal when switching to desktop
watch(isDesktop, (isNowDesktop) => {
  if (isNowDesktop) {
    showMobilePreview.value = false
  }
})

// Responsive breakpoints
const windowWidth = ref(typeof window !== 'undefined' ? window.innerWidth : 1200)

const isMobile = computed(() => windowWidth.value < 768)
const isTablet = computed(() => windowWidth.value >= 768 && windowWidth.value < 1200)
const isDesktop = computed(() => windowWidth.value >= 1200)

// Update window width on resize
if (typeof window !== 'undefined') {
  onMounted(() => {
    const handleResize = () => {
      windowWidth.value = window.innerWidth
    }
    window.addEventListener('resize', handleResize)
    onUnmounted(() => {
      window.removeEventListener('resize', handleResize)
    })
  })
}

// Document action handlers
function handleOpenDocument() {
  // TODO: Implement document opening logic
  console.log('Open document:', selectedDocument.value)
}

function handleEditDocument() {
  // TODO: Implement document editing logic
  console.log('Edit document:', selectedDocument.value)
}

function handleDeleteDocument() {
  // TODO: Implement document deletion logic
  console.log('Delete document:', selectedDocument.value)
  showMobilePreview.value = false
  selectedDocument.value = null
}

function handleUpdateNotes(notes: string) {
  // TODO: Implement notes update logic
  console.log('Update notes:', notes)
  if (selectedDocument.value) {
    selectedDocument.value.notes = notes
  }
}
</script>

<style scoped>
.app-layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--md-sys-color-background);
  color: var(--md-sys-color-on-background);
}

.app-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.content-pane {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

/* Mobile: Full width content, bottom padding for nav */
@media (max-width: 767px) {
  .content-pane {
    padding: 16px;
    padding-bottom: 80px; /* Space for bottom nav */
  }
}

/* Tablet: Navigation Rail + Content */
@media (min-width: 768px) and (max-width: 1199px) {
  .app-content {
    grid-template-columns: 72px 1fr;
  }
}

/* Desktop: Navigation Rail + Content + Supporting Pane */
@media (min-width: 1200px) {
  .app-content {
    grid-template-columns: 80px 1fr 360px;
  }
}
</style>
