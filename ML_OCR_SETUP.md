# Guide d'installation et d'utilisation du ML OCR

## 📋 Prérequis

- **Python 3.8+** installé sur votre système
- **3-4 GB d'espace disque** pour les modèles
- **Connexion Internet** pour télécharger les modèles

## 🚀 Installation

### 1. Installer les dépendances Python

```bash
# Méthode automatique (recommandée)
chmod +x install_ml_dependencies.sh
./install_ml_dependencies.sh
```

**OU**

```bash
# Méthode manuelle
cd python
python3 -m venv venv
source venv/bin/activate  # macOS/Linux
pip install -r requirements.txt
```

### 2. Téléchargement des modèles

Les modèles sont téléchargés automatiquement au premier usage depuis Hugging Face Hub.

**Modèles utilisés :**
- `microsoft/trocr-base-handwritten` (~1.4GB) - OCR manuscrit
- `microsoft/trocr-base-printed` (~1.4GB) - OCR imprimé ML
- `Salesforce/blip-image-captioning-base` (~990MB) - Description d'images

**Cache :** Les modèles sont stockés dans `~/.cache/huggingface/`

## 🎯 Utilisation

### Dans l'application

1. Lancez l'application Tauri
2. Dans la section "Importer un document", choisissez le type d'analyse :
   - **Standard (Tesseract)** : Rapide, texte imprimé classique
   - **Manuscrit (TrOCR)** : Pour l'écriture manuelle
   - **Imprimé ML (TrOCR)** : Texte imprimé avec IA (meilleure précision)
   - **Description (BLIP)** : Génère une description de l'image

3. Importez votre document normalement

### En ligne de commande (tests)

```bash
# Activer l'environnement virtuel
cd python
source venv/bin/activate

# Test OCR manuscrit
python trocr_handwritten.py /path/to/handwritten.jpg handwritten

# Test OCR imprimé
python trocr_handwritten.py /path/to/printed.png printed

# Test description d'image (anglais)
python blip_caption.py /path/to/photo.jpg en

# Test description d'image (français - expérimental)
python blip_caption.py /path/to/photo.jpg fr
```

## ⚙️ Configuration

### GPU vs CPU

- **GPU (CUDA)** : Installation automatique si NVIDIA GPU disponible
  - Performance : ~0.5-1 seconde par image
  - Nécessite CUDA Toolkit installé

- **CPU** : Fonctionne partout
  - Performance : ~2-5 secondes par image
  - Pas de dépendance matérielle

### Optimisation de la mémoire

Les modèles sont chargés en lazy loading :
- Un seul modèle en mémoire à la fois
- ~2-3 GB de RAM nécessaire
- Les modèles sont mis en cache après le premier chargement

## 🐛 Dépannage

### Erreur "Python 3 not found"

```bash
# macOS : Installer via Homebrew
brew install python3

# Ubuntu/Debian
sudo apt-get install python3 python3-pip python3-venv

# Windows
# Télécharger depuis python.org
```

### Erreur "Script Python introuvable"

Vérifiez que vous lancez l'application depuis le répertoire racine du projet.

### Modèles trop lents

1. Vérifiez que vous avez assez de RAM (4GB minimum)
2. Fermez les autres applications
3. Utilisez le mode Standard (Tesseract) pour les documents simples

### Erreur de téléchargement des modèles

```bash
# Vérifier la connexion Internet
ping huggingface.co

# Nettoyer le cache si nécessaire
rm -rf ~/.cache/huggingface/
```

## 📊 Comparaison des modes

| Mode                     | Vitesse         | Précision        | Cas d'usage                                      |
| ------------------------ | --------------- | ---------------- | ------------------------------------------------ |
| **Standard (Tesseract)** | ⚡⚡⚡ Très rapide | ⭐⭐⭐ Bonne        | Texte imprimé standard, documents scannés        |
| **Manuscrit (TrOCR)**    | ⚡ Lent          | ⭐⭐⭐⭐ Très bonne  | Notes manuscrites, formulaires remplis à la main |
| **Imprimé ML (TrOCR)**   | ⚡ Lent          | ⭐⭐⭐⭐⭐ Excellente | Texte imprimé complexe, polices non standard     |
| **Description (BLIP)**   | ⚡ Lent          | ⭐⭐⭐⭐ Très bonne  | Photos, images sans texte, catalogage visuel     |

## 🔧 Développement

### Ajouter un nouveau modèle

1. Modifiez `python/trocr_handwritten.py` ou `python/blip_caption.py`
2. Changez la constante `MODEL_NAME`
3. Testez en ligne de commande
4. Ajoutez l'option dans `components/OcrTypeSelector.vue`
5. Mettez à jour `commands.rs` pour exposer la nouvelle fonctionnalité

### Structure des réponses JSON

**TrOCR :**
```json
{
  "success": true,
  "text": "Texte extrait",
  "model": "microsoft/trocr-base-handwritten",
  "device": "cpu"
}
```

**BLIP :**
```json
{
  "success": true,
  "caption": "a cat sitting on a couch",
  "language": "en",
  "model": "Salesforce/blip-image-captioning-base",
  "device": "cpu"
}
```

**Erreur :**
```json
{
  "success": false,
  "error": "Description de l'erreur"
}
```

## 📚 Ressources

- [TrOCR Paper](https://arxiv.org/abs/2109.10282)
- [BLIP Paper](https://arxiv.org/abs/2201.12086)
- [Hugging Face Transformers](https://huggingface.co/docs/transformers)
- [Candle Framework](https://github.com/huggingface/candle) (alternative Rust, non utilisée ici)

## 🆘 Support

En cas de problème :
1. Vérifiez les logs dans la console Rust (stderr)
2. Testez les scripts Python manuellement
3. Vérifiez que l'environnement virtuel est activé
4. Consultez `python/README.md` pour plus de détails
