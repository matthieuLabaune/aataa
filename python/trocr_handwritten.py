#!/usr/bin/env python3
"""
TrOCR Handwritten Text Extractor
Utilise Microsoft TrOCR pour extraire le texte manuscrit des images
"""

import sys
import json
from pathlib import Path
from PIL import Image
from transformers import TrOCRProcessor, VisionEncoderDecoderModel
import torch

# Configuration des modèles
HANDWRITTEN_MODEL = "microsoft/trocr-base-handwritten"
PRINTED_MODEL = "microsoft/trocr-base-printed"

class TrOCRExtractor:
    def __init__(self, model_name=HANDWRITTEN_MODEL):
        """Initialise TrOCR avec le modèle spécifié"""
        print(f"🔄 Chargement du modèle {model_name}...", file=sys.stderr)

        # Détection du device (GPU si disponible)
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        print(f"📱 Utilisation du device: {self.device}", file=sys.stderr)

        # Chargement du processeur et du modèle
        self.processor = TrOCRProcessor.from_pretrained(model_name)
        self.model = VisionEncoderDecoderModel.from_pretrained(model_name)
        self.model.to(self.device)
        self.model.eval()

        print("✅ Modèle chargé avec succès", file=sys.stderr)

    def extract_text(self, image_path):
        """Extrait le texte d'une image"""
        try:
            # Chargement de l'image
            image = Image.open(image_path).convert("RGB")
            print(f"📸 Image chargée: {image.size}", file=sys.stderr)

            # Préparation des données
            pixel_values = self.processor(
                images=image,
                return_tensors="pt"
            ).pixel_values.to(self.device)

            # Génération du texte
            print("🔮 Génération du texte...", file=sys.stderr)
            with torch.no_grad():
                generated_ids = self.model.generate(pixel_values)

            # Décodage
            generated_text = self.processor.batch_decode(
                generated_ids,
                skip_special_tokens=True
            )[0]

            return generated_text.strip()

        except Exception as e:
            print(f"❌ Erreur lors de l'extraction: {str(e)}", file=sys.stderr)
            raise

def main():
    if len(sys.argv) < 2:
        print(json.dumps({
            "error": "Usage: python trocr_handwritten.py <image_path> [handwritten|printed]"
        }))
        sys.exit(1)

    image_path = sys.argv[1]
    model_type = sys.argv[2] if len(sys.argv) > 2 else "handwritten"

    # Vérification du fichier
    if not Path(image_path).exists():
        print(json.dumps({
            "error": f"L'image n'existe pas: {image_path}"
        }))
        sys.exit(1)

    try:
        # Sélection du modèle
        model_name = HANDWRITTEN_MODEL if model_type == "handwritten" else PRINTED_MODEL

        # Extraction du texte
        extractor = TrOCRExtractor(model_name)
        text = extractor.extract_text(image_path)

        # Résultat JSON
        result = {
            "success": True,
            "text": text,
            "model": model_name,
            "device": extractor.device
        }

        print(json.dumps(result, ensure_ascii=False))

    except Exception as e:
        print(json.dumps({
            "success": False,
            "error": str(e)
        }))
        sys.exit(1)

if __name__ == "__main__":
    main()
