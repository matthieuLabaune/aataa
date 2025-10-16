#!/usr/bin/env python3
"""
BLIP Image Captioning
Génère des descriptions automatiques pour les images
"""

import sys
import json
from pathlib import Path
from PIL import Image
from transformers import BlipProcessor, BlipForConditionalGeneration
import torch

# Configuration des modèles
BLIP_MODEL = "Salesforce/blip-image-captioning-base"

class ImageCaptioner:
    def __init__(self):
        """Initialise BLIP pour la génération de descriptions"""
        print(f"🔄 Chargement du modèle BLIP...", file=sys.stderr)

        # Détection du device (GPU si disponible)
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        print(f"📱 Utilisation du device: {self.device}", file=sys.stderr)

        # Chargement du processeur et du modèle
        self.processor = BlipProcessor.from_pretrained(BLIP_MODEL)
        self.model = BlipForConditionalGeneration.from_pretrained(BLIP_MODEL)
        self.model.to(self.device)
        self.model.eval()

        print("✅ Modèle chargé avec succès", file=sys.stderr)

    def generate_caption(self, image_path, language="en", max_length=50):
        """Génère une description pour une image"""
        try:
            # Chargement de l'image
            image = Image.open(image_path).convert("RGB")
            print(f"📸 Image chargée: {image.size}", file=sys.stderr)

            # Préparation des données
            inputs = self.processor(image, return_tensors="pt").to(self.device)

            # Génération de la description
            print("🔮 Génération de la description...", file=sys.stderr)
            with torch.no_grad():
                # Configuration pour différentes langues
                if language == "fr":
                    # Pour le français, on peut utiliser un prompt
                    prompt_text = "une image de"
                    prompt_inputs = self.processor(
                        image,
                        text=prompt_text,
                        return_tensors="pt"
                    ).to(self.device)
                    generated_ids = self.model.generate(
                        **prompt_inputs,
                        max_length=max_length
                    )
                else:
                    # Description en anglais par défaut
                    generated_ids = self.model.generate(
                        **inputs,
                        max_length=max_length
                    )

            # Décodage
            caption = self.processor.decode(
                generated_ids[0],
                skip_special_tokens=True
            )

            return caption.strip()

        except Exception as e:
            print(f"❌ Erreur lors de la génération: {str(e)}", file=sys.stderr)
            raise

def main():
    if len(sys.argv) < 2:
        print(json.dumps({
            "error": "Usage: python blip_caption.py <image_path> [en|fr]"
        }))
        sys.exit(1)

    image_path = sys.argv[1]
    language = sys.argv[2] if len(sys.argv) > 2 else "en"

    # Vérification du fichier
    if not Path(image_path).exists():
        print(json.dumps({
            "error": f"L'image n'existe pas: {image_path}"
        }))
        sys.exit(1)

    try:
        # Génération de la description
        captioner = ImageCaptioner()
        caption = captioner.generate_caption(image_path, language)

        # Résultat JSON
        result = {
            "success": True,
            "caption": caption,
            "language": language,
            "model": BLIP_MODEL,
            "device": captioner.device
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
