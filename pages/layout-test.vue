<template>
  <AppLayout>
    <div class="test-content">
      <h1 class="headline-medium">Nouveau Layout Material Design 3</h1>
      <p class="body-large">
        Le layout multi-pane est maintenant actif ! 🎉
      </p>

      <div class="test-cards">
        <div
          v-for="i in 6"
          :key="i"
          class="card elevation-1 rounded-medium"
          @click="selectDocument(i)"
          style="padding: 24px; cursor: pointer;"
        >
          <h3 class="title-medium">Document {{ i }}</h3>
          <p class="body-medium">Cliquez pour voir dans le panneau de droite (desktop uniquement)</p>
        </div>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
const { selectedDocument } = useDocumentFilters()

function selectDocument(id: number) {
  selectedDocument.value = {
    id: `doc-${id}`,
    new_name: `Document_Test_${id}.pdf`,
    document_type: 'Facture',
    created_at: new Date().toISOString(),
    file_size: 1024 * 500, // 500 KB
    tags: ['test', 'demo'],
    notes: `Notes pour le document ${id}`
  }
}
</script>

<style scoped>
.test-content {
  max-width: 1200px;
  margin: 0 auto;
}

.test-content h1 {
  margin-bottom: 16px;
}

.test-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
  margin-top: 32px;
}

.card {
  transition: transform var(--md-sys-motion-duration-short4) var(--md-sys-motion-easing-standard);
}

.card:hover {
  transform: translateY(-4px);
  box-shadow: var(--md-sys-elevation-level2);
}
</style>
