# AATAA - Auto Archive Documents

Application de bureau (Tauri + Nuxt + Rust) pour l'archivage automatique de documents avec OCR, classification et renommage intelligent.

## 🚀 Fonctionnalités

- **Import de documents** : PDF, PNG, JPG, JPEG
- **OCR avec Tesseract** : Extraction de texte automatique (FR + EN)
- **Classification intelligente** : Détection automatique du type de document (facture, contrat, relevé bancaire, etc.)
- **Renommage automatique** : Génération de noms de fichiers structurés
- **Tagging automatique** : Extraction d'informations (dates, montants, emails, téléphones)
- **Stockage local** : Base de données SQLite hors ligne
- **Recherche** : Recherche full-text dans les documents
- **Interface moderne** : UI avec Nuxt UI et Tailwind CSS

## 📋 Prérequis

### macOS
```bash
# Installer Tesseract OCR
brew install tesseract tesseract-lang

# Vérifier l'installation
tesseract --version
```

### Windows
- Télécharger et installer Tesseract : https://github.com/UB-Mannheim/tesseract/wiki
- Ajouter Tesseract au PATH système

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install tesseract-ocr tesseract-ocr-fra tesseract-ocr-eng
```

## 🛠️ Installation

```bash
# Installer les dépendances Node.js
npm install

# Installer les dépendances Rust (automatique avec Tauri)
cd src-tauri
cargo build
cd ..
```

## 🏃 Développement

```bash
# Lancer en mode développement
npm run tauri dev
```

## 📦 Build

```bash
# Build pour production
npm run tauri build
```

Le fichier exécutable sera dans `src-tauri/target/release/bundle/`

## 📁 Structure du projet

```
aataa/
├── app/                    # Frontend Nuxt
│   └── app.vue            # Application principale
├── src-tauri/             # Backend Rust
│   └── src/
│       ├── main.rs        # Point d'entrée
│       ├── lib.rs         # Configuration Tauri
│       ├── models.rs      # Structures de données
│       ├── database.rs    # SQLite
│       ├── ocr.rs         # Tesseract OCR
│       ├── classifier.rs  # Classification & tagging
│       └── commands.rs    # API Tauri
├── nuxt.config.ts         # Configuration Nuxt
└── package.json
```

## 🔧 Configuration

Au premier lancement, configurez le dossier d'archivage via l'icône ⚙️ en haut à droite.

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
