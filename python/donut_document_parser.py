#!/usr/bin/env python3
"""
Donut Document Understanding - Extraction structurée de documents
Remplace l'OCR classique par un modèle qui COMPREND les documents
"""

import sys
import json
from pathlib import Path
from PIL import Image
from transformers import DonutProcessor, VisionEncoderDecoderModel
import torch
import re

# Modèle Donut pour l'analyse de documents
DONUT_MODEL = "naver-clova-ix/donut-base-finetuned-docvqa"

class DonutDocumentParser:
    def __init__(self):
        """Initialise Donut pour l'analyse de documents"""
        print(f"🔄 Chargement du modèle Donut...", file=sys.stderr)

        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        print(f"📱 Device: {self.device}", file=sys.stderr)

        self.processor = DonutProcessor.from_pretrained(DONUT_MODEL)
        self.model = VisionEncoderDecoderModel.from_pretrained(DONUT_MODEL)
        self.model.to(self.device)
        self.model.eval()

        print("✅ Modèle Donut chargé", file=sys.stderr)

    def parse_document(self, image_path, task="<s_docvqa><s_question>What is the total amount?</s_question><s_answer>"):
        """
        Analyse un document et extrait les informations structurées

        Args:
            image_path: Chemin vers l'image
            task: Tâche spécifique (question pour DocVQA)

        Returns:
            dict avec les informations extraites
        """
        try:
            # Charger l'image
            image = Image.open(image_path).convert("RGB")

            # Préparer l'input
            pixel_values = self.processor(image, return_tensors="pt").pixel_values
            pixel_values = pixel_values.to(self.device)

            # Encoder la tâche
            decoder_input_ids = self.processor.tokenizer(
                task,
                add_special_tokens=False,
                return_tensors="pt"
            ).input_ids
            decoder_input_ids = decoder_input_ids.to(self.device)

            # Générer la réponse
            with torch.no_grad():
                outputs = self.model.generate(
                    pixel_values,
                    decoder_input_ids=decoder_input_ids,
                    max_length=self.model.decoder.config.max_position_embeddings,
                    pad_token_id=self.processor.tokenizer.pad_token_id,
                    eos_token_id=self.processor.tokenizer.eos_token_id,
                    use_cache=True,
                    bad_words_ids=[[self.processor.tokenizer.unk_token_id]],
                    return_dict_in_generate=True,
                )

            # Décoder la réponse
            sequence = self.processor.batch_decode(outputs.sequences)[0]
            sequence = sequence.replace(self.processor.tokenizer.eos_token, "").replace(self.processor.tokenizer.pad_token, "")

            return {
                "success": True,
                "result": sequence,
                "model": DONUT_MODEL,
                "device": self.device
            }

        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "model": DONUT_MODEL
            }

    def extract_invoice_data(self, image_path):
        """Extrait les données structurées d'une facture"""

        # Questions multiples pour extraire toutes les infos
        questions = {
            "amount": "What is the total amount?",
            "invoice_number": "What is the invoice number?",
            "date": "What is the invoice date?",
            "vendor": "Who is the vendor?",
            "client": "Who is the client?"
        }

        results = {}

        for key, question in questions.items():
            task = f"<s_docvqa><s_question>{question}</s_question><s_answer>"
            response = self.parse_document(image_path, task)

            if response["success"]:
                # Extraire la réponse
                answer = response["result"]
                # Parser la réponse structurée
                match = re.search(r'<s_answer>(.*?)</s_answer>', answer)
                if match:
                    results[key] = match.group(1).strip()
                else:
                    results[key] = answer.strip()

        return {
            "success": True,
            "data": results,
            "model": DONUT_MODEL,
            "device": self.device
        }


def main():
    if len(sys.argv) < 2:
        print(json.dumps({
            "success": False,
            "error": "Usage: python donut_document_parser.py <image_path> [extract_type]"
        }))
        sys.exit(1)

    image_path = sys.argv[1]
    extract_type = sys.argv[2] if len(sys.argv) > 2 else "invoice"

    if not Path(image_path).exists():
        print(json.dumps({
            "success": False,
            "error": f"Image not found: {image_path}"
        }))
        sys.exit(1)

    try:
        parser = DonutDocumentParser()

        if extract_type == "invoice":
            result = parser.extract_invoice_data(image_path)
        else:
            # Question générique
            result = parser.parse_document(image_path)

        print(json.dumps(result, ensure_ascii=False, indent=2))

    except Exception as e:
        print(json.dumps({
            "success": False,
            "error": str(e),
            "traceback": str(e.__traceback__)
        }), file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
