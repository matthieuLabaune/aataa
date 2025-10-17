# PaperVault 🔒

**Coffre-fort documentaire intelligent** — Application de bureau pour l'archivage automatique de documents avec OCR multilingue, classification par IA et recherche avancée.

![PaperVault](https://img.shields.io/badge/version-0.1.0-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.8.5-orange)
![Vue](https://img.shields.io/badge/Vue-3.5.22-green)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-red)

---

## � Table des matières

- [Fonctionnalités](#-fonctionnalités)
- [Prérequis](#-prérequis)
- [Installation](#-installation)
- [Développement](#-développement)
- [Documentation](#-documentation)
- [Architecture](#-architecture)
- [Contribuer](#-contribuer)
- [Licence](#-licence)

---

## ✨ Fonctionnalités

### 🎯 Traitement intelligent de documents
- **Import multi-format** : PDF, PNG, JPG, JPEG
- **OCR multilingue** : Tesseract (FR, EN) + TrOCR pour l'écriture manuscrite
- **Classification automatique** : Système de mots-clés personnalisables par catégorie
- **Extraction de métadonnées** : N° facture, montants, dates, noms de clients, entreprises
- **Renommage intelligent** : Génération automatique de noms de fichiers structurés

### 📊 Organisation et recherche
- **8 catégories principales** : Administratif, Financier, Santé, Professionnel, Immobilier, Académique, Personnel, Autre
- **27+ sous-catégories** : Personnalisables et extensibles
- **Tags automatiques** : Extraction de dates, montants, emails, téléphones, entreprises
- **Recherche full-text** : Recherche instantanée dans tout le contenu OCR
- **Filtres avancés** : Par catégorie, sous-catégorie, tags, date, type de document

### 🎨 Interface moderne
- **Material Design 3** : Interface minimaliste noir/blanc/gris
- **Navigation intuitive** : Rail de navigation + barre d'applications
- **Vue Explorer** : Gestion avancée des documents avec filtres
- **Indicateurs de progression** : Suivi en temps réel du traitement
- **Corbeille** : Suppression sécurisée avec restauration

### 🔧 Personnalisation
- **Mots-clés éditables** : Personnalisation du système de classification
- **Poids ajustables** : Contrôle de l'importance de chaque mot-clé (Faible → Très important)
- **Sous-catégories personnalisées** : Création de vos propres catégories
- **Dossier d'archive configurable** : Choisissez où stocker vos documents

---

## 📋 Prérequis

### Tesseract OCR

#### macOS
```bash
brew install tesseract tesseract-lang
tesseract --version  # Vérifier l'installation
```

#### Windows
1. Télécharger depuis : https://github.com/UB-Mannheim/tesseract/wiki
2. Installer et ajouter au PATH système
3. Redémarrer le terminal

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install tesseract-ocr tesseract-ocr-fra tesseract-ocr-eng
```

### Technologies requises
- **Node.js** : v18+ (LTS recommandé)
- **Rust** : 1.75+ (installé automatiquement par Tauri)
- **Tesseract** : 5.0+ (voir ci-dessus)

---

## 🛠️ Installation

```bash
# Cloner le dépôt
git clone https://github.com/votre-repo/papervault.git
cd papervault

# Installer les dépendances Node.js
npm install

# Les dépendances Rust seront installées automatiquement lors du build
```

---

## 🏃 Développement

```bash
# Lancer en mode développement (hot-reload)
npm run tauri dev

# Le frontend démarre sur http://localhost:3000
# L'application Tauri s'ouvre automatiquement
```

### Scripts disponibles
- `npm run dev` : Lance le serveur Nuxt uniquement
- `npm run tauri dev` : Lance l'application complète (recommandé)
- `npm run tauri build` : Compile l'application pour production
- `cargo test --manifest-path=src-tauri/Cargo.toml` : Lance les tests Rust

---

## 📦 Build de production

```bash
# Build optimisé pour votre plateforme
npm run tauri build

# Fichiers générés :
# macOS : src-tauri/target/release/bundle/macos/PaperVault.app
# Windows : src-tauri/target/release/bundle/msi/PaperVault.msi
# Linux : src-tauri/target/release/bundle/deb/papervault.deb
```

---

## 📚 Documentation

### 📖 Guides utilisateur
- **[🚀 Démarrage rapide](docs/QUICKSTART.md)** — Guide d'installation et premiers pas
- **[✅ Tests et validation](docs/TEST_GUIDE.md)** — Tests fonctionnels et scénarios d'utilisation
- **[📊 Résultats des tests](docs/TEST_RESULTS.md)** — Rapports de validation

### 🛠️ Documentation technique
- **[🏗️ Architecture](docs/ARCHITECTURE.md)** — Vue d'ensemble de l'architecture système
- **[🎨 Material Design](docs/MATERIAL_DESIGN_RECAP.md)** — Spécifications du design
- **[🎭 Wireframes UI](docs/UI_WIREFRAME.md)** — Maquettes et flows d'interface
- **[🔍 Patterns de classification](docs/CLASSIFICATION_PATTERNS.md)** — Système de classification documentaire

### 📝 Gestion de projet
- **[📋 TODO & Roadmap](docs/TODO.md)** — Tâches en cours et prévues
- **[✔️ Checklist](docs/CHECKLIST.md)** — État d'avancement du projet
- **[📈 Plan d'implémentation](docs/IMPLEMENTATION_PLAN.md)** — Phases de développement
- **[🔄 Changelog](docs/CHANGELOG.md)** — Historique des versions
- **[📦 Résumés de build](docs/BUILD_SUMMARY.md)** — Synthèses des builds
- **[📊 Résumés de phase](docs/PHASE_SUMMARY.md)** — Récapitulatifs des phases

### 🤝 Contribution
- **[🤝 Guide de contribution](docs/CONTRIBUTING.md)** — Comment contribuer au projet
- **[✅ Statut final](docs/FINAL_STATUS.md)** — État du projet et objectifs atteints
- **[📦 Livraison](docs/DELIVERY.md)** — Processus de release et déploiement

---

## 🏗️ Architecture

### Stack technique

**Frontend**
- **Nuxt 3.15.3** : Framework Vue.js avec auto-imports et SSR
- **Vue 3.5.22** : Framework réactif avec Composition API
- **TypeScript** : Typage statique pour plus de robustesse
- **Vite 7.1.7** : Build tool ultra-rapide
- **Material Design 3** : Design system moderne et minimaliste

**Backend**
- **Tauri 2.8.5** : Framework cross-platform léger (Rust + WebView)
- **Rust 1.75+** : Langage système performant et sécurisé
- **SQLite (rusqlite 0.32)** : Base de données locale sans serveur
- **Tesseract 5.0+** : OCR open-source multilingue
- **TrOCR** : Modèle IA pour l'écriture manuscrite

**Architecture**
```
┌─────────────────────────────────────────┐
│          Vue 3 + Nuxt UI                │
│    (Pages, Components, Composables)     │
└────────────────┬────────────────────────┘
                 │
                 │ Tauri IPC
                 ▼
┌─────────────────────────────────────────┐
│         Rust Backend (Tauri)            │
│  ┌───────────────────────────────────┐  │
│  │  Commands API (Tauri handlers)   │  │
│  └────────────┬──────────────────────┘  │
│               │                         │
│  ┌────────────▼──────────────────────┐  │
│  │  Core Services                    │  │
│  │  - Database (SQLite)              │  │
│  │  - Classifier (mots-clés + DB)    │  │
│  │  - OCR Engine (Tesseract)         │  │
│  │  - File Manager                   │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────┐
│     Système de fichiers local          │
│  - Base SQLite (documents, keywords)   │
│  - Dossier archive (PDFs, images)      │
└─────────────────────────────────────────┘
```

### Flux de traitement de document

1. **Import** : Utilisateur sélectionne un fichier (PDF/PNG/JPG)
2. **OCR** : Tesseract extrait le texte (multilingue)
3. **Classification** : 
   - Chargement des mots-clés depuis DB
   - Scoring par catégorie selon les poids
   - Sélection de la meilleure catégorie
4. **Extraction métadonnées** :
   - N° facture, montant, client, entreprise
   - Dates, emails, téléphones
5. **Stockage** :
   - Enregistrement en base de données
   - Copie dans dossier d'archive
6. **Affichage** : Document disponible dans Explorer avec filtres

---

## 🔧 Configuration

**Premier lancement** : Configurez le dossier d'archivage via ⚙️ Paramètres

**Personnalisation** :
- Ajoutez vos propres mots-clés de classification
- Créez des sous-catégories personnalisées
- Ajustez les poids des mots-clés pour améliorer la précision

---

## 🧪 Tests

### Lancer les tests

```bash
# Tests Rust
cargo test --manifest-path=src-tauri/Cargo.toml

# Tests avec sortie détaillée
cargo test --manifest-path=src-tauri/Cargo.toml -- --nocapture
```

### Tests manuels
Consultez [docs/TEST_GUIDE.md](docs/TEST_GUIDE.md) pour les scénarios de test complets.

---

## 🤝 Contribuer

Les contributions sont les bienvenues ! Consultez [CONTRIBUTING.md](docs/CONTRIBUTING.md) pour les guidelines.

### Workflow de contribution
1. Fork le projet
2. Créer une branche feature (`git checkout -b feature/AmazingFeature`)
3. Commit vos changements (`git commit -m 'Add AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

---

## � Licence

Ce projet est sous licence MIT. Voir [LICENSE](LICENSE) pour plus de détails.

---

## 👥 Auteurs

- **Matthieu Labaune** — Développement initial

---

## 🙏 Remerciements

- [Tauri](https://tauri.app/) — Framework cross-platform
- [Tesseract OCR](https://github.com/tesseract-ocr/tesseract) — Moteur OCR
- [Material Design](https://m3.material.io/) — Design system
- [Nuxt](https://nuxt.com/) — Framework Vue.js

---

## 📞 Support

Pour toute question ou problème :
- Ouvrir une [issue](https://github.com/votre-repo/papervault/issues)
- Consulter la [documentation](docs/)

---

**PaperVault** — Votre coffre-fort documentaire intelligent 🔒

## 🎯 Types de documents détectés

- **Factures** : Détection de montants, totaux, mentions "facture"
- **Contrats** : Termes contractuels, signatures
- **Relevés bancaires** : IBAN, soldes, opérations
- **Bulletins de paie** : Salaire, cotisations
- **Documents officiels** : Cartes d'identité, passeports, attestations
- **Reçus** : Tickets de caisse

## 📊 Base de données

Les données sont stockées localement dans :
- macOS : `~/Library/Application Support/com.aataa.app/aataa.db`
- Windows : `%APPDATA%/com.aataa.app/aataa.db`
- Linux : `~/.local/share/com.aataa.app/aataa.db`

## 🔮 Évolution future

### Intégration de modèles OCR avancés (HuggingFace)

Pour améliorer la précision de l'OCR, possibilité d'intégrer :
- **olmOCR-7B-0825** : https://huggingface.co/allenai/olmOCR-7B-0825
- **TrOCR** : https://huggingface.co/microsoft/trocr-base-printed
- **Donut** : https://huggingface.co/naver-clova-ix/donut-base

Implémentation via :
```rust
// Ajouter au Cargo.toml :
// candle-core = "0.3"
// candle-transformers = "0.3"
// tokenizers = "0.15"

// Puis créer un module hf_ocr.rs pour l'inférence
```

## 🤝 Contribution

Les contributions sont les bienvenues ! Voir les issues pour les fonctionnalités planifiées.

## 📄 Licence

MIT
