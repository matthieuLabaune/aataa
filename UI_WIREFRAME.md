# 🎨 AATAA - Wireframe & Design System Material 3

## 🎯 Vision

Transformer AATAA d'une simple liste de documents en un **gestionnaire de documents professionnel** avec une navigation intuitive de type Finder/File Manager, basé sur Material Design 3.

### Problème actuel
- Navigation limitée : scroll infini dans une liste unique
- Pas de vue d'ensemble rapide
- Difficulté à naviguer entre différents types/années
- Pas de prévisualisation

### Solution proposée
- **Multi-pane layout** : Navigation + Contenu + Aperçu
- **Navigation Rail** : Accès rapide aux filtres
- **Material Design 3** : UI moderne et cohérente
- **Responsive** : Adapté mobile/tablet/desktop

---

## 📐 Architecture des écrans

### 🖥️ Desktop (1200px+)

```
┌─────────────────────────────────────────────────────────────────┐
│  [Logo] AATAA                                        [Settings] │ ← Top App Bar
├───────┬──────────────────────────────────────┬──────────────────┤
│       │                                      │                  │
│  📂   │  📄 Facture_2024_01_15.pdf          │  ┌────────────┐  │
│       │  📄 Contrat_ABC_2024.pdf             │  │            │  │
│  📊   │  📄 Releve_BANK_012024.pdf          │  │   PDF      │  │
│       │  📄 Bulletin_PAIE_012024.pdf         │  │  Preview   │  │
│  🏷️   │                                      │  │            │  │
│       │  ┌─────────────────────────────┐    │  │            │  │
│  📅   │  │ 🔍 Rechercher documents... │    │  └────────────┘  │
│       │  └─────────────────────────────┘    │                  │
│  ⚙️   │                                      │  Type: Facture   │
│       │  [Grid View] [List View]            │  Date: 15/01/24  │
│       │                                      │  Tags: montant   │
│       │  ┌────┐ ┌────┐ ┌────┐ ┌────┐       │                  │
│       │  │📄  │ │📄  │ │📄  │ │📄  │       │  📝 Notes...     │
│       │  │FAC │ │CON │ │BAN │ │PAI │       │                  │
│       │  └────┘ └────┘ └────┘ └────┘       │  [Ouvrir] [Edit] │
│       │                                      │                  │
└───────┴──────────────────────────────────────┴──────────────────┘
   ↑                    ↑                              ↑
Navigation Rail    Content Pane              Supporting Pane
  (80px)             (flex-grow)                  (360px)
```

### 📱 Mobile (< 768px)

```
┌─────────────────────────────────────┐
│  [☰] AATAA              [Settings]  │ ← Top App Bar
├─────────────────────────────────────┤
│  ┌─────────────────────────────┐    │
│  │ 🔍 Rechercher...           │    │
│  └─────────────────────────────┘    │
│                                     │
│  📄 Facture_2024_01_15.pdf         │
│  Type: Facture • 15/01/2024        │
│  ────────────────────────────────  │
│                                     │
│  📄 Contrat_ABC_2024.pdf           │
│  Type: Contrat • 10/01/2024        │
│  ────────────────────────────────  │
│                                     │
│  📄 Releve_BANK_012024.pdf         │
│  Type: Relevé • 05/01/2024         │
│  ────────────────────────────────  │
│                                     │
├─────────────────────────────────────┤
│  [📂] [📊] [🏷️] [📅] [⚙️]         │ ← Bottom Nav
└─────────────────────────────────────┘
```

### 💻 Tablet (768px - 1200px)

```
┌───────────────────────────────────────────────────┐
│  [Logo] AATAA                        [Settings]   │ ← Top App Bar
├─────┬─────────────────────────────────────────────┤
│     │  ┌─────────────────────────────┐            │
│ 📂  │  │ 🔍 Rechercher...           │            │
│     │  └─────────────────────────────┘            │
│ 📊  │                                             │
│     │  [Grid View] [List View]                   │
│ 🏷️  │                                             │
│     │  ┌──────────┐ ┌──────────┐ ┌──────────┐   │
│ 📅  │  │  📄      │ │  📄      │ │  📄      │   │
│     │  │ Facture  │ │ Contrat  │ │  Relevé  │   │
│ ⚙️  │  │ 15/01/24 │ │ 10/01/24 │ │ 05/01/24 │   │
│     │  └──────────┘ └──────────┘ └──────────┘   │
│     │                                             │
│     │  ┌──────────┐ ┌──────────┐                │
│     │  │  📄      │ │  📄      │                │
│     │  │  Paie    │ │ Document │                │
│     │  │ 01/01/24 │ │ 20/12/23 │                │
│     │  └──────────┘ └──────────┘                │
│     │                                             │
└─────┴─────────────────────────────────────────────┘
  ↑                       ↑
Navigation Rail      Content Pane
  (72px)              (flex-grow)
```

---

## 🎨 Material Design 3 Components

### Navigation Rail (Desktop/Tablet)
- **Position**: Fixed left
- **Width**: 80px (desktop) / 72px (tablet)
- **Items**:
  - 📂 Types de documents (avec badge count)
  - 📊 Dashboard/Stats
  - 🏷️ Tags
  - 📅 Années
  - ⚙️ Paramètres

### Bottom Navigation (Mobile)
- **Position**: Fixed bottom
- **Height**: 56px
- **Items**: Mêmes que Navigation Rail

### Top App Bar
- **Type**: Small (64px height)
- **Content**:
  - Logo/Title: "AATAA"
  - Actions: Settings, User menu
  - Elevation: 2dp

### Content Pane
- **Layout**: Grid ou Liste (toggle)
- **Grid**: Cards 3 colonnes (desktop), 2 (tablet), 1 (mobile)
- **Liste**: Expansion panels avec preview
- **Filters**: Chips pour filtrage rapide

### Supporting Pane (Desktop uniquement)
- **Width**: 360px
- **Content**:
  - Preview du document (PDF/Image thumbnail)
  - Métadonnées (type, date, tags)
  - Actions (Ouvrir, Éditer, Supprimer)
  - Notes éditable

### Cards
```
┌─────────────────────────┐
│  📄                     │
│  Facture_2024_01.pdf    │
│  ─────────────────────  │
│  Type: Facture          │
│  Date: 15/01/2024       │
│  Tags: montant, tva     │
│  ─────────────────────  │
│  [👁️ Voir] [✏️ Éditer]  │
└─────────────────────────┘
```

---

## 🎨 Design Tokens

### Couleurs (Material 3 Dynamic Color)
```css
/* Primary */
--md-sys-color-primary: #6750A4;
--md-sys-color-on-primary: #FFFFFF;
--md-sys-color-primary-container: #EADDFF;
--md-sys-color-on-primary-container: #21005D;

/* Secondary */
--md-sys-color-secondary: #625B71;
--md-sys-color-on-secondary: #FFFFFF;
--md-sys-color-secondary-container: #E8DEF8;
--md-sys-color-on-secondary-container: #1D192B;

/* Tertiary */
--md-sys-color-tertiary: #7D5260;
--md-sys-color-on-tertiary: #FFFFFF;
--md-sys-color-tertiary-container: #FFD8E4;
--md-sys-color-on-tertiary-container: #31111D;

/* Surface */
--md-sys-color-surface: #FEF7FF;
--md-sys-color-on-surface: #1D1B20;
--md-sys-color-surface-variant: #E7E0EC;
--md-sys-color-on-surface-variant: #49454F;

/* Background */
--md-sys-color-background: #FEF7FF;
--md-sys-color-on-background: #1D1B20;

/* Error */
--md-sys-color-error: #B3261E;
--md-sys-color-on-error: #FFFFFF;
--md-sys-color-error-container: #F9DEDC;
--md-sys-color-on-error-container: #410E0B;
```

### Typography
```css
/* Display */
--md-sys-typescale-display-large: 57px/64px;
--md-sys-typescale-display-medium: 45px/52px;
--md-sys-typescale-display-small: 36px/44px;

/* Headline */
--md-sys-typescale-headline-large: 32px/40px;
--md-sys-typescale-headline-medium: 28px/36px;
--md-sys-typescale-headline-small: 24px/32px;

/* Title */
--md-sys-typescale-title-large: 22px/28px;
--md-sys-typescale-title-medium: 16px/24px;
--md-sys-typescale-title-small: 14px/20px;

/* Body */
--md-sys-typescale-body-large: 16px/24px;
--md-sys-typescale-body-medium: 14px/20px;
--md-sys-typescale-body-small: 12px/16px;

/* Label */
--md-sys-typescale-label-large: 14px/20px;
--md-sys-typescale-label-medium: 12px/16px;
--md-sys-typescale-label-small: 11px/16px;
```

### Élévation
```css
--md-sys-elevation-level0: 0dp; /* Surface */
--md-sys-elevation-level1: 1dp; /* Cards */
--md-sys-elevation-level2: 3dp; /* App Bar */
--md-sys-elevation-level3: 6dp; /* FAB */
--md-sys-elevation-level4: 8dp; /* Navigation Drawer */
--md-sys-elevation-level5: 12dp; /* Modal */
```

### Rayon de bordure
```css
--md-sys-shape-corner-none: 0px;
--md-sys-shape-corner-extra-small: 4px;
--md-sys-shape-corner-small: 8px;
--md-sys-shape-corner-medium: 12px;
--md-sys-shape-corner-large: 16px;
--md-sys-shape-corner-extra-large: 28px;
--md-sys-shape-corner-full: 9999px;
```

---

## 📱 Responsive Breakpoints

```css
/* Mobile first */
$mobile: 0px;
$tablet: 768px;
$desktop: 1200px;
$wide: 1920px;

/* Usage */
@media (max-width: 767px) {
  /* Mobile: Bottom Nav, Full-width content, No preview pane */
}

@media (min-width: 768px) and (max-width: 1199px) {
  /* Tablet: Navigation Rail, Grid 2-col, No preview pane */
}

@media (min-width: 1200px) {
  /* Desktop: Navigation Rail, Grid 3-col, Preview pane */
}

@media (min-width: 1920px) {
  /* Wide: Same as desktop but wider content area */
}
```

---

## 🧩 Composants à créer

### 1. Layout Components
- [ ] `AppLayout.vue` - Layout principal avec Navigation Rail/Bottom Nav
- [ ] `NavigationRail.vue` - Rail de navigation (desktop/tablet)
- [ ] `BottomNavigation.vue` - Navigation bottom (mobile)
- [ ] `TopAppBar.vue` - Barre d'app top
- [ ] `SupportingPane.vue` - Panneau de droite avec preview

### 2. Content Components
- [ ] `DocumentGrid.vue` - Grille de documents
- [ ] `DocumentList.vue` - Liste de documents
- [ ] `DocumentCard.vue` - ✅ Déjà existe (à adapter Material 3)
- [ ] `DocumentPreview.vue` - Preview PDF/Image
- [ ] `FilterChips.vue` - Chips de filtrage

### 3. UI Components
- [ ] `SearchBar.vue` - Barre de recherche Material 3
- [ ] `ViewToggle.vue` - Toggle Grid/List
- [ ] `EmptyState.vue` - ✅ Déjà existe (à adapter Material 3)
- [ ] `LoadingState.vue` - État de chargement

### 4. Pages
- [ ] `index.vue` - Page principale avec nouveau layout
- [ ] `dashboard.vue` - Stats et analytics (optionnel)

---

## 🔄 Migration Path

### Phase 1: Setup & Layout (1h)
1. Créer tokens Material 3 dans `assets/css/tokens.css`
2. Créer `AppLayout.vue` avec structure multi-pane
3. Créer `NavigationRail.vue` et `BottomNavigation.vue`
4. Créer `TopAppBar.vue`

### Phase 2: Content Components (1h)
1. Adapter `DocumentCard.vue` au style Material 3
2. Créer `DocumentGrid.vue` avec responsive grid
3. Créer `DocumentList.vue` pour vue liste
4. Créer `SupportingPane.vue` avec preview

### Phase 3: Features (30min)
1. Implémenter filtrage par Navigation Rail
2. Ajouter toggle Grid/List view
3. Implémenter preview dans Supporting Pane
4. Ajouter animations Material Motion

### Phase 4: Polish & Responsive (30min)
1. Tester responsive sur 3 breakpoints
2. Ajuster spacing et élévations
3. Valider accessibilité (ARIA, keyboard nav)
4. Dark mode (optionnel)

---

## 🎯 Navigation Flow

### Filtrage par Type
1. Clic sur icône Type (📂) dans Navigation Rail
2. Affichage des types disponibles avec count
3. Sélection d'un type filtre la Content Pane
4. Chips actifs affichés en haut de Content Pane

### Filtrage par Année
1. Clic sur icône Année (📅) dans Navigation Rail
2. Affichage chronologique des années
3. Sélection filtre par année
4. Breadcrumb : "2024 > Factures"

### Filtrage par Tags
1. Clic sur icône Tags (🏷️) dans Navigation Rail
2. Affichage cloud de tags
3. Multi-sélection possible
4. Chips multiples actifs

### Preview & Actions
1. Clic sur document → affichage dans Supporting Pane
2. Preview PDF/Image thumbnail
3. Métadonnées éditables inline
4. Actions : Ouvrir, Éditer, Supprimer, Exporter

---

## ✨ Animations (Material Motion)

### Transitions
```css
/* Standard easing */
--md-sys-motion-easing-standard: cubic-bezier(0.2, 0.0, 0, 1.0);
--md-sys-motion-easing-emphasized: cubic-bezier(0.2, 0.0, 0, 1.0);

/* Durations */
--md-sys-motion-duration-short1: 50ms;
--md-sys-motion-duration-short2: 100ms;
--md-sys-motion-duration-medium1: 250ms;
--md-sys-motion-duration-medium2: 300ms;
--md-sys-motion-duration-long1: 400ms;
--md-sys-motion-duration-long2: 500ms;
```

### Effets
- **Card hover**: Élévation +2dp, transition 200ms
- **Navigation Rail item**: Ripple effect, active indicator
- **Document sélection**: Shared element transition
- **Pane toggle**: Slide in/out avec easing emphasized

---

## 🚀 Stack Technique Recommandé

### Option 1: Vanilla CSS + Material Tokens ⭐ RECOMMANDÉ
**Avantages**:
- Contrôle total sur le design
- Léger (pas de dépendances lourdes)
- Apprentissage Material 3 en profondeur
- Customisation facile

**Inconvénients**:
- Temps de développement plus long
- Besoin d'implémenter les composants from scratch

### Option 2: Vuetify 3
**Avantages**:
- Composants Material 3 ready-to-use
- Développement rapide
- Documentation complète

**Inconvénients**:
- Bundle plus lourd (~150KB)
- Customisation limitée
- Dépendance externe

### Option 3: Tailwind + Headless UI
**Avantages**:
- Utility-first, très flexible
- Petite taille avec PurgeCSS
- Composants accessibles

**Inconvénients**:
- Pas Material 3 native
- Besoin de créer les tokens Material manuellement
- Courbe d'apprentissage Tailwind

---

## 📝 Notes d'implémentation

1. **Priorité Desktop-first puis Mobile**: Commencer par la version desktop 3-pane, puis adapter responsive
2. **State Management**: Utiliser `useState` Nuxt pour filtres actifs, vue sélectionnée
3. **Performances**: Virtualisation pour listes longues (>100 items)
4. **Accessibilité**: 
   - Navigation clavier (Tab, Arrow keys)
   - ARIA labels sur Navigation Rail
   - Focus management sur modal/preview
5. **Dark Mode**: Préparer les tokens dark mais implémenter après v1

---

## 🎨 Inspiration & Références

- [Material Design 3](https://m3.material.io/)
- [Material You](https://material.io/blog/announcing-material-you)
- [macOS Finder](https://support.apple.com/guide/mac-help/finder-mchlp2605/mac)
- [Google Drive Web](https://drive.google.com/)
- [Notion Sidebar](https://www.notion.so/)
