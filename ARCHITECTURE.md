# Architecture AATAA

## Vue d'ensemble

AATAA est une application desktop multi-plateforme construite avec :
- **Frontend**: Nuxt 3 + Vue 3 + Nuxt UI + Tailwind CSS
- **Backend**: Rust + Tauri 2
- **OCR**: Tesseract via tesseract-plumbing
- **Database**: SQLite via rusqlite

## Architecture technique

### Frontend (Nuxt 3)

```
app/
├── app.vue              # Application principale
├── app.config.ts        # Configuration UI
components/
├── DocumentCard.vue     # Carte d'affichage document
└── EmptyState.vue       # État vide
types/
└── tauri.d.ts          # Types TypeScript pour Tauri
```

**Fonctionnalités** :
- Interface utilisateur moderne et responsive
- Drag & drop de fichiers (préparé)
- Recherche en temps réel
- Modal de paramètres
- Affichage dynamique des documents

### Backend (Rust)

```
src-tauri/src/
├── main.rs           # Point d'entrée
├── lib.rs            # Configuration Tauri + State
├── models.rs         # Structures de données
├── database.rs       # Couche SQLite
├── ocr.rs            # Engine OCR Tesseract
├── classifier.rs     # Classification + Tagging
└── commands.rs       # API Tauri (bridge Frontend/Backend)
```

#### Modules détaillés

**models.rs** :
- `Document` : Structure principale de document
- `DocumentType` : Type de document avec pattern regex

**database.rs** :
- `Database::new()` : Initialisation SQLite
- `insert_document()` : Insertion
- `get_all_documents()` : Récupération tous
- `search_documents()` : Recherche full-text
- `delete_document()` : Suppression

**ocr.rs** :
- `OcrEngine::new()` : Initialisation Tesseract
- `extract_text_from_image()` : OCR sur images
- `extract_text_from_pdf()` : Extraction texte PDF

**classifier.rs** :
- `Classifier::new()` : Initialisation avec patterns
- `classify()` : Classification par regex
- `extract_tags()` : Extraction métadonnées
- `generate_filename()` : Génération nom fichier

**commands.rs** : API Tauri exposée au frontend
- `process_file()` : Pipeline complet traitement
- `get_documents()` : Liste documents
- `search_documents()` : Recherche
- `delete_document()` : Suppression
- `open_file()` : Ouvrir fichier OS
- `set_archive_path()` : Configurer dossier
- `get_archive_path()` : Récupérer dossier

## Flux de données

### Import de document

```
1. User sélectionne fichier → Frontend
2. invoke('process_file') → Backend Rust
3. OCR extraction → ocr.rs
4. Classification → classifier.rs
5. Génération nom → classifier.rs
6. Copie fichier → commands.rs
7. Insertion DB → database.rs
8. Retour Document → Frontend
9. Mise à jour UI → Vue
```

### Recherche

```
1. User tape recherche → Frontend
2. invoke('search_documents') → Backend
3. SQLite LIKE query → database.rs
4. Retour résultats → Frontend
5. Affichage filtré → Vue
```

## Patterns de classification

| Type              | Pattern Regex                            | Préfixe |
| ----------------- | ---------------------------------------- | ------- |
| Facture           | `facture\|invoice\|montant\|total`       | FACT    |
| Contrat           | `contrat\|contract\|agreement`           | CONT    |
| Relevé bancaire   | `relevé\|bank statement\|iban`           | BANK    |
| Bulletin de paie  | `bulletin de paie\|salaire\|payslip`     | PAIE    |
| Document officiel | `carte identité\|passeport\|attestation` | OFFI    |
| Reçu              | `reçu\|receipt\|ticket`                  | RECU    |

## Tags automatiques

- `contains_date` : Si date détectée (DD/MM/YYYY)
- `contains_amount` : Si montant détecté (XX.XX €)
- `company_document` : Si forme juridique détectée (SARL, SAS, etc.)
- `contains_email` : Si email détecté
- `contains_phone` : Si téléphone FR détecté

## Stockage des données

### Base de données
**Emplacement** :
- macOS: `~/Library/Application Support/com.aataa.app/aataa.db`
- Windows: `%APPDATA%/com.aataa.app/aataa.db`
- Linux: `~/.local/share/com.aataa.app/aataa.db`

**Schema** :
```sql
CREATE TABLE documents (
    id TEXT PRIMARY KEY,
    original_name TEXT NOT NULL,
    new_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    document_type TEXT NOT NULL,
    tags TEXT NOT NULL,        -- JSON array
    ocr_text TEXT,
    created_at TEXT NOT NULL,  -- RFC3339
    file_size INTEGER NOT NULL
)
```

### Fichiers archivés
**Emplacement par défaut** :
- `{APP_DATA_DIR}/archive/`

**Format noms** :
- `{PREFIX}_{YYYYMMDD_HHMMSS}.{ext}`
- Exemple: `FACT_20241009_143022.png`

## Communication Frontend ↔ Backend

### Via Tauri Commands (IPC)

Frontend (TypeScript) :
```typescript
import { invoke } from '@tauri-apps/api/core'

const doc = await invoke<Document>('process_file', {
  filePath: '/path/to/file.png'
})
```

Backend (Rust) :
```rust
#[command]
pub async fn process_file(
    file_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<Document, String> {
    // Implementation
}
```

## Sécurité

- ✅ Toutes les données sont stockées localement
- ✅ Pas de connexion réseau requise
- ✅ OCR traité en local
- ✅ Pas d'envoi de données vers cloud
- ✅ Chiffrement possible au niveau OS

## Performance

### OCR
- Temps moyen : 2-5s par page A4
- Dépend de la résolution image
- Multi-threading via Tokio

### Base de données
- SQLite en mémoire pour recherches rapides
- Index sur colonnes recherchées
- Transactions pour intégrité

## Évolutions futures

### Court terme
- [ ] Support drag & drop complet
- [ ] Preview documents dans l'app
- [ ] Export CSV/JSON
- [ ] Statistiques

### Moyen terme
- [ ] OCR multi-pages PDF
- [ ] Batch processing
- [ ] Règles de classification personnalisées
- [ ] Templates d'export

### Long terme
- [ ] Intégration modèles HuggingFace (olmOCR, TrOCR)
- [ ] Classification par ML
- [ ] Extraction entités nommées
- [ ] Synchronisation cloud optionnelle

## Dépendances principales

### Frontend
- `nuxt`: ^4.1.3
- `@nuxt/ui`: ^3.0.0
- `@tauri-apps/api`: ^2.8.0

### Backend
- `tauri`: 2.8.5
- `rusqlite`: 0.32
- `tesseract-plumbing`: 0.11
- `regex`: 1.10
- `image`: 0.25
- `pdf-extract`: 0.7

## Build

### Développement
```bash
npm run tauri:dev
```

### Production
```bash
npm run tauri:build
```

Génère :
- macOS: `.app` + `.dmg`
- Windows: `.exe` + `.msi`
- Linux: `.AppImage` + `.deb`
