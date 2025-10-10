# 🚀 Plan d'Implémentation Material Design 3

## 📋 Résumé Exécutif

**Objectif**: Transformer AATAA en gestionnaire de documents Material 3 avec navigation multi-pane  
**Durée estimée**: 2-3 heures  
**Approche**: Vanilla CSS + Material Tokens (contrôle total, léger)  
**Stratégie**: Desktop-first → Tablet → Mobile

---

## 🎯 Phase 1: Foundation & Tokens (20 min)

### 1.1 Créer les Design Tokens
- [ ] `assets/css/tokens.css` - Variables CSS Material 3
  - Couleurs (primary, secondary, tertiary, surface, error)
  - Typography (display, headline, title, body, label)
  - Élévation (level0 à level5)
  - Shape (border radius)
  - Motion (easing, durations)

### 1.2 Créer la base CSS
- [ ] `assets/css/material.css` - Classes utilitaires Material 3
  - `.surface`, `.surface-variant`
  - `.elevation-1` à `.elevation-5`
  - `.rounded-small`, `.rounded-medium`, `.rounded-large`
  - `.transition-standard`, `.transition-emphasized`

### 1.3 Mettre à jour nuxt.config.ts
```ts
css: [
  '~/assets/css/tokens.css',
  '~/assets/css/material.css',
  '~/public/main.css' // Garder les styles existants pour compatibilité
]
```

---

## 🏗️ Phase 2: Layout Principal (45 min)

### 2.1 Créer AppLayout.vue
```vue
<template>
  <div class="app-layout">
    <TopAppBar />
    <div class="app-content">
      <NavigationRail v-if="isDesktopOrTablet" />
      <main class="content-pane">
        <slot />
      </main>
      <SupportingPane v-if="isDesktop && selectedDocument" />
    </div>
    <BottomNavigation v-if="isMobile" />
  </div>
</template>
```

**Fichiers à créer**:
- [ ] `components/layout/AppLayout.vue`
- [ ] `components/layout/TopAppBar.vue`
- [ ] `components/layout/NavigationRail.vue`
- [ ] `components/layout/BottomNavigation.vue`
- [ ] `components/layout/SupportingPane.vue`

### 2.2 TopAppBar.vue
- Logo AATAA à gauche
- Titre centré (mobile) ou gauche (desktop)
- Icône Settings à droite
- Élévation level2
- Height: 64px

### 2.3 NavigationRail.vue (Desktop/Tablet)
- Width: 80px (desktop), 72px (tablet)
- Position: fixed left
- Items:
  - 📂 Types (avec badge count)
  - 📊 Dashboard
  - 🏷️ Tags
  - 📅 Années
  - ⚙️ Settings
- Active indicator Material 3
- Ripple effect sur hover

### 2.4 BottomNavigation.vue (Mobile)
- Height: 56px
- Position: fixed bottom
- Mêmes items que NavigationRail
- Active state avec icône + label

### 2.5 SupportingPane.vue (Desktop only)
- Width: 360px
- Preview document (PDF thumbnail ou icône)
- Métadonnées affichées
- Actions: Ouvrir, Éditer, Supprimer
- Notes éditables inline
- Border-left separator

---

## 📱 Phase 3: Composants de Contenu (45 min)

### 3.1 DocumentGrid.vue
```vue
<template>
  <div class="document-grid">
    <DocumentCard
      v-for="doc in documents"
      :key="doc.id"
      :document="doc"
      @click="selectDocument(doc)"
    />
  </div>
</template>
```

**Grid responsive**:
- Desktop (1200px+): 3 colonnes
- Tablet (768-1199px): 2 colonnes
- Mobile (<768px): 1 colonne
- Gap: 16px

**Fichiers**:
- [ ] `components/documents/DocumentGrid.vue`
- [ ] `components/documents/DocumentList.vue` (alternative view)
- [ ] `components/documents/ViewToggle.vue` (switch Grid/List)

### 3.2 Adapter DocumentCard.vue au Material 3
```vue
<template>
  <div class="document-card surface elevation-1">
    <div class="card-media">
      <div class="document-icon">📄</div>
    </div>
    <div class="card-content">
      <h3 class="title-medium">{{ document.new_name }}</h3>
      <div class="card-metadata">
        <span class="body-small">{{ document.document_type }}</span>
        <span class="body-small">{{ formatDate(document.created_at) }}</span>
      </div>
      <div class="card-tags">
        <span v-for="tag in document.tags" class="chip-small">{{ tag }}</span>
      </div>
    </div>
    <div class="card-actions">
      <button class="icon-button">👁️</button>
      <button class="icon-button">✏️</button>
    </div>
  </div>
</template>
```

**Styles Material 3**:
- Surface elevation-1
- Rounded-medium (12px)
- Hover: elevation-2 + transition 200ms
- Active: elevation-3

### 3.3 DocumentPreview.vue (Supporting Pane)
```vue
<template>
  <div class="document-preview">
    <div class="preview-image">
      <!-- PDF thumbnail ou icône document -->
      <img v-if="thumbnailUrl" :src="thumbnailUrl" />
      <div v-else class="preview-placeholder">📄</div>
    </div>
    <div class="preview-metadata">
      <h2 class="title-large">{{ document.new_name }}</h2>
      <div class="metadata-row">
        <span class="label-medium">Type:</span>
        <span class="body-medium">{{ document.document_type }}</span>
      </div>
      <div class="metadata-row">
        <span class="label-medium">Date:</span>
        <span class="body-medium">{{ formatDate(document.created_at) }}</span>
      </div>
      <div class="metadata-row">
        <span class="label-medium">Taille:</span>
        <span class="body-medium">{{ formatFileSize(document.file_size) }}</span>
      </div>
      <div class="metadata-tags">
        <span v-for="tag in document.tags" class="chip">{{ tag }}</span>
      </div>
    </div>
    <div class="preview-notes">
      <label class="label-large">Notes</label>
      <textarea v-model="notes" class="notes-input" />
    </div>
    <div class="preview-actions">
      <button class="btn-primary">Ouvrir</button>
      <button class="btn-secondary">Éditer</button>
      <button class="btn-error">Supprimer</button>
    </div>
  </div>
</template>
```

---

## 🎨 Phase 4: Styles & Interactions (30 min)

### 4.1 Créer tokens.css
```css
:root {
  /* Primary */
  --md-sys-color-primary: #6750A4;
  --md-sys-color-on-primary: #FFFFFF;
  --md-sys-color-primary-container: #EADDFF;
  --md-sys-color-on-primary-container: #21005D;

  /* Surface */
  --md-sys-color-surface: #FEF7FF;
  --md-sys-color-on-surface: #1D1B20;
  --md-sys-color-surface-variant: #E7E0EC;

  /* Elevation */
  --md-sys-elevation-level1: 0 1px 2px rgba(0,0,0,0.3), 0 1px 3px rgba(0,0,0,0.15);
  --md-sys-elevation-level2: 0 1px 2px rgba(0,0,0,0.3), 0 2px 6px rgba(0,0,0,0.15);
  --md-sys-elevation-level3: 0 4px 8px rgba(0,0,0,0.3), 0 6px 20px rgba(0,0,0,0.15);

  /* Typography */
  --md-sys-typescale-title-large: 22px;
  --md-sys-typescale-title-medium: 16px;
  --md-sys-typescale-body-medium: 14px;
  --md-sys-typescale-label-medium: 12px;

  /* Shape */
  --md-sys-shape-corner-small: 8px;
  --md-sys-shape-corner-medium: 12px;
  --md-sys-shape-corner-large: 16px;

  /* Motion */
  --md-sys-motion-easing-standard: cubic-bezier(0.2, 0.0, 0, 1.0);
  --md-sys-motion-duration-medium1: 250ms;
}
```

### 4.2 Créer material.css (classes utilitaires)
```css
/* Surfaces */
.surface {
  background-color: var(--md-sys-color-surface);
  color: var(--md-sys-color-on-surface);
}

.surface-variant {
  background-color: var(--md-sys-color-surface-variant);
  color: var(--md-sys-color-on-surface-variant);
}

/* Elevation */
.elevation-1 {
  box-shadow: var(--md-sys-elevation-level1);
}

.elevation-2 {
  box-shadow: var(--md-sys-elevation-level2);
}

/* Shape */
.rounded-small { border-radius: var(--md-sys-shape-corner-small); }
.rounded-medium { border-radius: var(--md-sys-shape-corner-medium); }
.rounded-large { border-radius: var(--md-sys-shape-corner-large); }

/* Transitions */
.transition-standard {
  transition: all var(--md-sys-motion-duration-medium1) var(--md-sys-motion-easing-standard);
}

/* Typography */
.title-large { font-size: var(--md-sys-typescale-title-large); font-weight: 500; }
.title-medium { font-size: var(--md-sys-typescale-title-medium); font-weight: 500; }
.body-medium { font-size: var(--md-sys-typescale-body-medium); }
.label-medium { font-size: var(--md-sys-typescale-label-medium); font-weight: 500; }

/* Buttons */
.btn-primary {
  background-color: var(--md-sys-color-primary);
  color: var(--md-sys-color-on-primary);
  padding: 10px 24px;
  border-radius: var(--md-sys-shape-corner-full);
  border: none;
  cursor: pointer;
  font-size: var(--md-sys-typescale-label-large);
  font-weight: 500;
}

.btn-primary:hover {
  box-shadow: var(--md-sys-elevation-level1);
}
```

### 4.3 Animations & Interactions
```css
/* Card hover effect */
.document-card {
  transition: box-shadow 200ms, transform 200ms;
}

.document-card:hover {
  box-shadow: var(--md-sys-elevation-level2);
  transform: translateY(-2px);
}

/* Active state pour Navigation Rail */
.nav-rail-item.active {
  background-color: var(--md-sys-color-secondary-container);
  color: var(--md-sys-color-on-secondary-container);
}

.nav-rail-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  width: 3px;
  height: 56px;
  background-color: var(--md-sys-color-primary);
}

/* Ripple effect (simple CSS version) */
.ripple {
  position: relative;
  overflow: hidden;
}

.ripple::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  border-radius: 50%;
  background-color: rgba(255, 255, 255, 0.5);
  transform: translate(-50%, -50%);
  transition: width 0.6s, height 0.6s;
}

.ripple:active::after {
  width: 200px;
  height: 200px;
}
```

---

## 📱 Phase 5: Responsive & Mobile (30 min)

### 5.1 Responsive Layout
```css
/* Mobile (<768px) */
@media (max-width: 767px) {
  .app-layout {
    display: flex;
    flex-direction: column;
    padding-bottom: 56px; /* Height of bottom nav */
  }

  .navigation-rail {
    display: none;
  }

  .supporting-pane {
    display: none; /* Preview en modal sur mobile */
  }

  .content-pane {
    padding: 16px;
  }

  .document-grid {
    grid-template-columns: 1fr; /* 1 colonne */
  }
}

/* Tablet (768-1199px) */
@media (min-width: 768px) and (max-width: 1199px) {
  .app-content {
    display: grid;
    grid-template-columns: 72px 1fr; /* Rail + Content */
  }

  .bottom-navigation {
    display: none;
  }

  .supporting-pane {
    display: none;
  }

  .document-grid {
    grid-template-columns: repeat(2, 1fr); /* 2 colonnes */
  }
}

/* Desktop (1200px+) */
@media (min-width: 1200px) {
  .app-content {
    display: grid;
    grid-template-columns: 80px 1fr 360px; /* Rail + Content + Supporting */
  }

  .bottom-navigation {
    display: none;
  }

  .document-grid {
    grid-template-columns: repeat(3, 1fr); /* 3 colonnes */
  }
}
```

### 5.2 Mobile Preview Modal
```vue
<!-- MobilePreviewModal.vue -->
<template>
  <Teleport to="body">
    <div v-if="show" class="modal-overlay" @click.self="close">
      <div class="modal-content surface elevation-5">
        <div class="modal-header">
          <h2 class="title-large">{{ document.new_name }}</h2>
          <button @click="close" class="icon-button">✕</button>
        </div>
        <DocumentPreview :document="document" />
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: flex-end;
  z-index: 1000;
}

.modal-content {
  width: 100%;
  max-height: 80vh;
  border-radius: 28px 28px 0 0;
  padding: 24px;
  overflow-y: auto;
  animation: slideUp 300ms var(--md-sys-motion-easing-standard);
}

@keyframes slideUp {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}
</style>
```

---

## 🔄 Phase 6: State Management & Filtres (30 min)

### 6.1 Créer composable pour filtres
```ts
// composables/useDocumentFilters.ts
export const useDocumentFilters = () => {
  const activeFilters = useState('activeFilters', () => ({
    type: null as string | null,
    year: null as string | null,
    tags: [] as string[],
    search: '' as string,
  }))

  const viewMode = useState('viewMode', () => 'grid' as 'grid' | 'list')
  
  const selectedDocument = useState('selectedDocument', () => null as Document | null)

  const filteredDocuments = computed(() => {
    let docs = documents.value
    
    if (activeFilters.value.type) {
      docs = docs.filter(d => d.document_type === activeFilters.value.type)
    }
    
    if (activeFilters.value.year) {
      docs = docs.filter(d => d.created_at.includes(activeFilters.value.year!))
    }
    
    if (activeFilters.value.tags.length > 0) {
      docs = docs.filter(d => 
        activeFilters.value.tags.some(tag => d.tags.includes(tag))
      )
    }
    
    if (activeFilters.value.search) {
      docs = docs.filter(d => 
        d.new_name.toLowerCase().includes(activeFilters.value.search.toLowerCase()) ||
        d.ocr_text.toLowerCase().includes(activeFilters.value.search.toLowerCase())
      )
    }
    
    return docs
  })

  return {
    activeFilters,
    viewMode,
    selectedDocument,
    filteredDocuments,
  }
}
```

### 6.2 Intégrer dans NavigationRail
```vue
<template>
  <nav class="navigation-rail surface-variant elevation-2">
    <div
      v-for="item in navItems"
      :key="item.id"
      class="nav-rail-item ripple"
      :class="{ active: isActive(item) }"
      @click="selectFilter(item)"
    >
      <span class="nav-icon">{{ item.icon }}</span>
      <span class="nav-label body-small">{{ item.label }}</span>
      <span v-if="item.badge" class="nav-badge">{{ item.badge }}</span>
    </div>
  </nav>
</template>

<script setup lang="ts">
const { activeFilters } = useDocumentFilters()
const { data: documents } = await useFetch('/api/documents')

const navItems = computed(() => [
  {
    id: 'types',
    icon: '📂',
    label: 'Types',
    badge: new Set(documents.value?.map(d => d.document_type)).size
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
    badge: new Set(documents.value?.flatMap(d => d.tags)).size
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
</script>
```

---

## ✅ Checklist d'Implémentation

### Phase 1: Foundation ✅
- [ ] Créer `assets/css/tokens.css`
- [ ] Créer `assets/css/material.css`
- [ ] Mettre à jour `nuxt.config.ts`

### Phase 2: Layout ✅
- [ ] Créer `components/layout/AppLayout.vue`
- [ ] Créer `components/layout/TopAppBar.vue`
- [ ] Créer `components/layout/NavigationRail.vue`
- [ ] Créer `components/layout/BottomNavigation.vue`
- [ ] Créer `components/layout/SupportingPane.vue`

### Phase 3: Composants ✅
- [ ] Créer `components/documents/DocumentGrid.vue`
- [ ] Créer `components/documents/DocumentList.vue`
- [ ] Créer `components/documents/ViewToggle.vue`
- [ ] Adapter `components/DocumentCard.vue` Material 3
- [ ] Créer `components/documents/DocumentPreview.vue`

### Phase 4: Styles ✅
- [ ] Implémenter tokens Material 3
- [ ] Créer classes utilitaires
- [ ] Ajouter animations & transitions
- [ ] Implémenter ripple effects

### Phase 5: Responsive ✅
- [ ] Layout mobile (Bottom Nav)
- [ ] Layout tablet (Navigation Rail 2-col)
- [ ] Layout desktop (Navigation Rail + Supporting Pane 3-col)
- [ ] Créer `components/MobilePreviewModal.vue`

### Phase 6: Features ✅
- [ ] Créer `composables/useDocumentFilters.ts`
- [ ] Implémenter filtrage par type
- [ ] Implémenter filtrage par année
- [ ] Implémenter filtrage par tags
- [ ] Implémenter barre de recherche
- [ ] Implémenter toggle Grid/List

### Phase 7: Integration ✅
- [ ] Migrer page `pages/index.vue`
- [ ] Tester toutes les fonctionnalités existantes
- [ ] Valider responsive sur 3 breakpoints
- [ ] Accessibilité (keyboard nav, ARIA)
- [ ] Commit final

---

## 🎯 Ordre d'Exécution Recommandé

1. **Tokens & Base CSS** (20 min) → Foundation solide
2. **AppLayout + TopAppBar** (15 min) → Structure de base
3. **NavigationRail + BottomNav** (20 min) → Navigation principale
4. **DocumentGrid + Card Material 3** (30 min) → Contenu principal
5. **SupportingPane + Preview** (20 min) → Preview desktop
6. **Responsive breakpoints** (20 min) → Mobile/Tablet
7. **Filtres & State Management** (20 min) → Fonctionnalités
8. **Polish & Testing** (15 min) → Finalisation

**Total**: ~2h40 (marge de 20 min pour imprévus)

---

## 🚀 Commandes de Test

```bash
# Dev server
npm run dev

# Build
npm run build

# Preview production
npm run preview

# Tests (après implémentation)
npm run test
```

---

## 📝 Notes Importantes

1. **Garder la compatibilité**: Ne pas supprimer les composants existants, les adapter
2. **Progressive Enhancement**: Desktop d'abord, puis responsive
3. **Performance**: Lazy load SupportingPane, virtualiser si >100 documents
4. **Accessibilité**: Tester navigation clavier, screen readers
5. **Git**: Commits atomiques par phase pour faciliter rollback si besoin

---

## 🎨 Résultat Final Attendu

- ✅ Interface Material Design 3 moderne
- ✅ Navigation multi-pane intuitive
- ✅ Responsive mobile/tablet/desktop
- ✅ Filtrage avancé (type, année, tags, recherche)
- ✅ Preview avec métadonnées éditables
- ✅ Animations fluides Material Motion
- ✅ Toutes les fonctionnalités existantes préservées
