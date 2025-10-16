# Scripts Python pour ML OCR

Ce dossier contient les scripts Python pour l'OCR avancé et la description d'images.

## Installation

```bash
# Créer un environnement virtuel
python3 -m venv venv
source venv/bin/activate  # Sur macOS/Linux
# ou: venv\Scripts\activate  # Sur Windows

# Installer les dépendances
pip install -r requirements.txt
```

## Scripts disponibles

### 1. TrOCR - OCR Manuscrit
`trocr_handwritten.py` - Extrait le texte manuscrit des images

**Usage:**
```bash
python trocr_handwritten.py <image_path> [handwritten|printed]
```

**Exemples:**
```bash
# Texte manuscrit
python trocr_handwritten.py /path/to/handwritten.jpg handwritten

# Texte imprimé
python trocr_handwritten.py /path/to/printed.png printed
```

**Sortie JSON:**
```json
{
  "success": true,
  "text": "Texte extrait de l'image",
  "model": "microsoft/trocr-base-handwritten",
  "device": "cpu"
}
```

### 2. BLIP - Description d'Images
`blip_caption.py` - Génère des descriptions automatiques

**Usage:**
```bash
python blip_caption.py <image_path> [en|fr]
```

**Exemples:**
```bash
# Description en anglais
python blip_caption.py /path/to/photo.jpg en

# Description en français (expérimental)
python blip_caption.py /path/to/photo.jpg fr
```

**Sortie JSON:**
```json
{
  "success": true,
  "caption": "a cat sitting on a couch",
  "language": "en",
  "model": "Salesforce/blip-image-captioning-base",
  "device": "cpu"
}
```

## Modèles utilisés

- **TrOCR Manuscrit**: `microsoft/trocr-base-handwritten` (~1.4GB)
- **TrOCR Imprimé**: `microsoft/trocr-base-printed` (~1.4GB)
- **BLIP Caption**: `Salesforce/blip-image-captioning-base` (~990MB)

Les modèles sont téléchargés automatiquement au premier usage et mis en cache dans `~/.cache/huggingface/`.

## Performance

- **CPU**: ~2-5 secondes par image
- **GPU**: ~0.5-1 seconde par image

## Gestion des erreurs

Toutes les erreurs sont retournées en JSON:
```json
{
  "success": false,
  "error": "Description de l'erreur"
}
```

Les logs de débogage sont envoyés sur stderr.
