# 🚀 Démarrage Rapide AATAA

## Prérequis ✅

### 1. Installer Tesseract OCR

**macOS** :
```bash
brew install tesseract tesseract-lang
```

**Ubuntu/Debian** :
```bash
sudo apt-get update
sudo apt-get install tesseract-ocr tesseract-ocr-fra tesseract-ocr-eng
```

**Windows** :
- Télécharger depuis : https://github.com/UB-Mannheim/tesseract/wiki
- Ajouter au PATH système

### 2. Vérifier l'installation
```bash
tesseract --version
# Devrait afficher: tesseract 5.x.x
```

## Installation AATAA 📦

### Méthode 1 : Script automatique
```bash
./quickstart.sh
```

### Méthode 2 : Manuel

1. **Installer les dépendances Node.js** :
```bash
npm install
```

2. **Compiler le backend Rust** :
```bash
cd src-tauri
cargo build
cd ..
```

## Lancement 🎯

### Mode développement
```bash
npm run tauri:dev
```

L'application s'ouvrira automatiquement.

### Mode production
```bash
npm run tauri:build
```

Le fichier exécutable sera dans `src-tauri/target/release/bundle/`

## Premier test 🧪

### 1. Créer une image de test

Ouvrez un éditeur de texte et créez un document avec :
```
FACTURE N° 2024-001
Date: 09/10/2024
Montant: 120.00 €
```

Faites une capture d'écran et sauvegardez en PNG.

### 2. Importer dans AATAA

1. Cliquez sur la zone d'import
2. Sélectionnez votre image
3. Attendez 2-5 secondes (OCR en cours)
4. Le document apparaît classifié automatiquement !

### 3. Vérifier le résultat

- **Type détecté** : "Facture" (badge bleu)
- **Nom généré** : `FACT_20241009_HHMMSS.png`
- **Tags** : `contains_date`, `contains_amount`
- **Texte OCR** : Visible en cliquant sur "Voir le texte extrait"

## Fonctionnalités disponibles 🎨

### Interface principale
- ✅ **Import** : Cliquer ou glisser-déposer (préparé)
- ✅ **Recherche** : Barre de recherche full-text
- ✅ **Liste** : Affichage de tous les documents
- ✅ **Actions** : Ouvrir, supprimer chaque document
- ✅ **Paramètres** : Configurer le dossier d'archivage

### Types détectés automatiquement
| Type                | Mots-clés                        | Préfixe |
| ------------------- | -------------------------------- | ------- |
| 📄 Facture           | facture, invoice, montant, total | FACT    |
| 📝 Contrat           | contrat, contract, agreement     | CONT    |
| 🏦 Relevé bancaire   | relevé, IBAN, solde              | BANK    |
| 💰 Bulletin de paie  | salaire, cotisation, net à payer | PAIE    |
| 🆔 Document officiel | carte identité, passeport        | OFFI    |
| 🧾 Reçu              | reçu, receipt, ticket            | RECU    |

### Tags automatiques
- 📅 `contains_date` : Date trouvée
- 💶 `contains_amount` : Montant trouvé
- 🏢 `company_document` : Entreprise détectée
- 📧 `contains_email` : Email trouvé
- 📞 `contains_phone` : Téléphone trouvé

## Dépannage 🔧

### Erreur : "Tesseract not found"
→ Tesseract n'est pas installé ou pas dans le PATH
→ Suivez les instructions d'installation ci-dessus

### Erreur : "Failed to initialize OCR"
→ Vérifiez que les données de langue sont installées :
```bash
# macOS
brew install tesseract-lang

# Ubuntu
sudo apt-get install tesseract-ocr-fra tesseract-ocr-eng
```

### L'OCR ne détecte rien
→ Assurez-vous que :
- L'image est de bonne qualité (min 300 DPI)
- Le texte est bien contrasté
- Le fichier est bien une image (PNG, JPG) ou PDF

### La compilation Rust échoue
→ Vérifiez que Rust est installé :
```bash
rustc --version
# Si pas installé : curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### npm install échoue
→ Vérifiez votre version Node.js :
```bash
node --version  # Devrait être >= 18
```

## Architecture des données 💾

### Base de données
- **Emplacement** : `~/Library/Application Support/com.aataa.app/aataa.db` (macOS)
- **Type** : SQLite
- **Données** : Métadonnées documents + texte OCR

### Fichiers archivés
- **Emplacement par défaut** : `~/Library/Application Support/com.aataa.app/archive/`
- **Format** : `{PREFIX}_{YYYYMMDD_HHMMSS}.{ext}`

### Confidentialité
- ✅ 100% offline
- ✅ Aucune donnée envoyée sur Internet
- ✅ Stockage local uniquement
- ✅ Contrôle total de vos données

## Prochaines étapes 📈

1. **Testez avec vos vrais documents**
2. **Configurez votre dossier d'archivage** (⚙️ en haut à droite)
3. **Explorez la recherche** (essayez de chercher des montants, dates, etc.)
4. **Consultez** ARCHITECTURE.md pour comprendre le fonctionnement

## Raccourcis utiles ⌨️

- **Ouvrir document** : Clic sur l'icône 👁️
- **Supprimer** : Clic sur l'icône 🗑️
- **Rechercher** : Tapez dans la barre de recherche
- **Actualiser** : Icône ↻ à côté du titre

## Support & Contribution 🤝

- 📖 Lire : [ARCHITECTURE.md](./ARCHITECTURE.md)
- 🧪 Tester : [TEST_GUIDE.md](./TEST_GUIDE.md)
- 📋 Issues : Créer une issue GitHub
- 🔧 Contribuer : Pull requests bienvenues !

---

**Bon archivage ! 📚**
