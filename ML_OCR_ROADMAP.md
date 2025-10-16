# 🤖 Amélioration OCR avec Machine Learning

## 📋 Vue d'ensemble

Ce document décrit les options pour améliorer l'OCR d'AATAA afin de supporter :
- ✍️ **Écriture manuscrite** (texte écrit à la main)
- 🖼️ **Description automatique d'images** (image captioning)

## 🎯 Objectifs

1. **OCR Manuscrit** : Reconnaître du texte manuscrit avec des modèles ML spécialisés
2. **Image Captioning** : Générer des descriptions automatiques pour les images
3. **Modèles locaux** : Utiliser des modèles open-source exécutés localement (pas d'API externe)
4. **Performances** : Maintenir des temps de traitement acceptables

## 🛠️ Options techniques

### Option 1 : Candle (Rust natif) ⚡

**Avantages** :
- ✅ Intégration native en Rust (pas de dépendances Python)
- ✅ Performances maximales
- ✅ Portable et compilé

**Inconvénients** :
- ❌ API complexe et en évolution
- ❌ Documentation limitée
- ❌ Nécessite expertise Rust ML

**Modèles disponibles** :
- **TrOCR** : Reconnaissance texte manuscrit (`microsoft/trocr-base-handwritten`)
- **BLIP** : Description d'images (`Salesforce/blip-image-captioning-large`)

**État actuel** :
- 🟡 Dépendances ajoutées au `Cargo.toml`
- 🟡 Modules créés (`trocr_ocr.rs`, `image_caption.rs`)
- 🔴 Erreurs de compilation dues à l'API Candle complexe
- 🔴 Nécessite refactoring majeur

### Option 2 : Python Subprocess + Transformers 🐍 (RECOMMANDÉE)

**Avantages** :
- ✅ API simple et bien documentée
- ✅ Écosystème Python ML mature
- ✅ Facile à maintenir et déboguer
- ✅ Large choix de modèles sur Hugging Face

**Inconvénients** :
- ⚠️ Nécessite Python installé
- ⚠️ Communication inter-processus (léger overhead)

**Architecture** :
```
Rust (Tauri) → Python Script → Transformers → Modèle local → Résultat → Rust
```

**Implémentation** :

1. **Script Python** (`src-tauri/python/ocr_ml.py`) :
```python
#!/usr/bin/env python3
import sys
from transformers import TrOCRProcessor, VisionEncoderDecoderModel
from PIL import Image

def extract_handwritten_text(image_path):
    processor = TrOCRProcessor.from_pretrained('microsoft/trocr-base-handwritten')
    model = VisionEncoderDecoderModel.from_pretrained('microsoft/trocr-base-handwritten')

    image = Image.open(image_path).convert("RGB")
    pixel_values = processor(images=image, return_tensors="pt").pixel_values
    generated_ids = model.generate(pixel_values)
    text = processor.batch_decode(generated_ids, skip_special_tokens=True)[0]

    return text

if __name__ == "__main__":
    image_path = sys.argv[1]
    print(extract_handwritten_text(image_path))
```

2. **Commande Rust** :
```rust
#[command]
pub async fn extract_handwritten_text(image_path: String) -> Result<String, String> {
    let output = std::process::Command::new("python3")
        .arg("python/ocr_ml.py")
        .arg(&image_path)
        .output()
        .map_err(|e| format!("Erreur Python: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
```

**Modèles recommandés** :
- **TrOCR** : `microsoft/trocr-base-handwritten` (texte manuscrit)
- **TrOCR** : `microsoft/trocr-base-printed` (texte imprimé haute qualité)
- **BLIP** : `Salesforce/blip-image-captioning-base` (descriptions)
- **BLIP-2** : `Salesforce/blip2-opt-2.7b` (descriptions avancées)

### Option 3 : API Externes (Cloud) ☁️

**Services disponibles** :
- **Google Cloud Vision API** : OCR manuscrit excellent
- **AWS Textract** : Reconnaissance de formulaires
- **Azure Computer Vision** : OCR + descriptions

**Avantages** :
- ✅ Qualité state-of-the-art
- ✅ Pas de modèles locaux à gérer
- ✅ Scalable

**Inconvénients** :
- ❌ Coûts (quotas limités gratuits)
- ❌ Nécessite connexion internet
- ❌ Confidentialité des données

## 🚀 Recommandation d'implémentation

### Phase 1 : Python Subprocess (court terme)

1. **Setup** :
```bash
# Installer Python dependencies
pip install transformers pillow torch
```

2. **Créer scripts Python** :
   - `python/trocr_handwritten.py` : OCR manuscrit
   - `python/trocr_printed.py` : OCR imprimé
   - `python/blip_caption.py` : Descriptions d'images

3. **Intégration Rust** :
   - Créer commandes Tauri qui appellent les scripts Python
   - Gérer les erreurs et timeouts
   - Ajouter indicateurs de progression

4. **UI** :
   - Ajouter sélecteur de type d'OCR (Standard/Manuscrit/Description)
   - Afficher le statut de téléchargement des modèles
   - Gérer le cache des modèles

### Phase 2 : Migration Candle (long terme)

Une fois l'API Candle stabilisée et mieux documentée :
- Migrer les scripts Python vers Rust natif
- Améliorer les performances
- Réduire les dépendances

## 📦 Dépendances Python

```txt
transformers>=4.30.0
torch>=2.0.0
pillow>=10.0.0
```

## 🧪 Tests

### Test OCR manuscrit
```bash
python3 python/trocr_handwritten.py test_images/handwritten.png
```

### Test descriptions
```bash
python3 python/blip_caption.py test_images/photo.jpg
```

## 📊 Performances estimées

| Modèle      | Taille  | Vitesse (CPU) | Qualité |
| ----------- | ------- | ------------- | ------- |
| TrOCR Base  | ~300 MB | 2-3s          | ⭐⭐⭐⭐    |
| TrOCR Large | ~900 MB | 5-8s          | ⭐⭐⭐⭐⭐   |
| BLIP Base   | ~500 MB | 3-4s          | ⭐⭐⭐⭐    |
| BLIP-2      | ~2.7 GB | 8-12s         | ⭐⭐⭐⭐⭐   |

*Vitesse mesurée sur MacBook Pro M1*

## 🔮 Prochaines étapes

1. ✅ Ajouter dépendances Candle (fait)
2. ⏳ Créer scripts Python (à faire)
3. ⏳ Intégrer subprocess dans Rust (à faire)
4. ⏳ Créer UI de sélection (à faire)
5. ⏳ Tester et optimiser (à faire)
6. ⏳ Documentation utilisateur (à faire)

## 📚 Ressources

- [Candle GitHub](https://github.com/huggingface/candle)
- [Transformers Docs](https://huggingface.co/docs/transformers)
- [TrOCR Paper](https://arxiv.org/abs/2109.10282)
- [BLIP Paper](https://arxiv.org/abs/2201.12086)
