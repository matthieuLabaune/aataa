#!/usr/bin/env python3
"""
Classification sémantique par embeddings
Remplace la classification par mots-clés par une vraie compréhension du contenu
"""

import sys
import json
from pathlib import Path
from sentence_transformers import SentenceTransformer, util
import numpy as np

# Modèle multilingue optimisé pour le français
EMBEDDING_MODEL = "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2"

# Définition des catégories avec descriptions riches
CATEGORIES = {
    "Financier": {
        "description": "Documents financiers comme factures, relevés bancaires, virements, notes de frais, reçus de paiement",
        "examples": [
            "Facture pour prestation de service informatique",
            "Relevé de compte bancaire mensuel",
            "Reçu de paiement par carte bancaire",
            "Facture d'électricité EDF",
            "Note de frais professionnelle"
        ]
    },
    "Administratif": {
        "description": "Documents officiels et administratifs comme cartes d'identité, passeports, attestations, certificats",
        "examples": [
            "Carte nationale d'identité",
            "Attestation de domicile",
            "Certificat de scolarité",
            "Avis d'imposition",
            "Permis de conduire"
        ]
    },
    "Santé": {
        "description": "Documents médicaux comme ordonnances, résultats d'analyses, remboursements CPAM, certificats médicaux",
        "examples": [
            "Ordonnance médicale du médecin",
            "Résultats d'analyses sanguines",
            "Décompte de remboursement CPAM",
            "Certificat médical d'aptitude",
            "Carnet de vaccination"
        ]
    },
    "Professionnel": {
        "description": "Documents liés au travail comme fiches de paie, contrats de travail, attestations employeur",
        "examples": [
            "Bulletin de salaire mensuel",
            "Contrat de travail CDI",
            "Attestation employeur Pôle Emploi",
            "Convention de stage",
            "Certificat de travail"
        ]
    },
    "Immobilier": {
        "description": "Documents immobiliers comme baux de location, actes de propriété, diagnostics",
        "examples": [
            "Bail de location d'appartement",
            "Acte de vente immobilier",
            "Diagnostic de performance énergétique",
            "Quittance de loyer",
            "État des lieux d'entrée"
        ]
    },
    "Académique": {
        "description": "Documents scolaires et universitaires comme diplômes, relevés de notes, thèses",
        "examples": [
            "Diplôme universitaire de Master",
            "Relevé de notes semestriel",
            "Attestation de réussite",
            "Thèse de doctorat",
            "Publication scientifique"
        ]
    },
    "Personnel": {
        "description": "Documents personnels variés comme courriers, tickets, photos de famille",
        "examples": [
            "Courrier postal personnel",
            "Ticket de caisse supermarché",
            "Lettre recommandée",
            "Reçu de livraison colis",
            "Note manuscrite personnelle"
        ]
    }
}


class SemanticClassifier:
    def __init__(self):
        """Initialise le modèle d'embeddings"""
        print(f"🔄 Chargement du modèle d'embeddings...", file=sys.stderr)

        self.model = SentenceTransformer(EMBEDDING_MODEL)

        # Pré-calculer les embeddings des catégories
        self.category_embeddings = {}

        for category, info in CATEGORIES.items():
            # Combiner description + exemples pour un embedding riche
            texts = [info["description"]] + info["examples"]
            embeddings = self.model.encode(texts, convert_to_tensor=True)
            # Moyenne des embeddings pour représenter la catégorie
            self.category_embeddings[category] = embeddings.mean(dim=0)

        print("✅ Modèle d'embeddings chargé", file=sys.stderr)

    def classify(self, text):
        """
        Classifie un texte par similarité sémantique

        Args:
            text: Texte extrait du document par OCR

        Returns:
            dict avec catégorie, confiance et scores détaillés
        """
        try:
            # Nettoyer le texte
            text = text.strip()
            if not text:
                return {
                    "success": False,
                    "error": "Texte vide"
                }

            # Encoder le texte
            text_embedding = self.model.encode(text, convert_to_tensor=True)

            # Calculer les similarités avec chaque catégorie
            scores = {}
            for category, category_embedding in self.category_embeddings.items():
                similarity = util.cos_sim(text_embedding, category_embedding).item()
                scores[category] = similarity

            # Trouver la meilleure catégorie
            best_category = max(scores, key=scores.get)
            confidence = scores[best_category]

            # Normaliser les scores en pourcentages
            total = sum(scores.values())
            normalized_scores = {k: (v / total) * 100 for k, v in scores.items()}

            # Suggérer une sous-catégorie basée sur le contenu
            subcategory = self._suggest_subcategory(best_category, text)

            return {
                "success": True,
                "category": best_category,
                "confidence": confidence,
                "subcategory": subcategory,
                "all_scores": normalized_scores,
                "model": EMBEDDING_MODEL
            }

        except Exception as e:
            return {
                "success": False,
                "error": str(e)
            }

    def _suggest_subcategory(self, category, text):
        """Suggère une sous-catégorie basée sur des mots-clés spécifiques"""
        text_lower = text.lower()

        subcategories = {
            "Financier": {
                "Facture freelance": ["auto-entrepreneur", "micro-entreprise", "prestation", "honoraires"],
                "Facture énergie": ["edf", "électricité", "gaz", "engie"],
                "Facture télécom": ["sfr", "orange", "free", "télécom", "mobile"],
                "Relevé bancaire": ["relevé", "iban", "compte", "banque"]
            },
            "Santé": {
                "Ordonnance": ["ordonnance", "prescription"],
                "Remboursement sécu": ["cpam", "remboursement", "décompte"],
                "Résultat analyse": ["analyse", "résultat", "laboratoire"]
            },
            "Professionnel": {
                "Fiche de paie": ["bulletin", "salaire", "paie", "net à payer"],
                "Contrat de travail": ["contrat", "cdi", "cdd", "employeur"]
            }
        }

        if category in subcategories:
            for subcat, keywords in subcategories[category].items():
                if any(kw in text_lower for kw in keywords):
                    return subcat

        return None


def main():
    if len(sys.argv) < 2:
        print(json.dumps({
            "success": False,
            "error": "Usage: python semantic_classifier.py <text>"
        }))
        sys.exit(1)

    # Le texte peut être passé directement ou via stdin
    if sys.argv[1] == "-":
        text = sys.stdin.read()
    else:
        text = " ".join(sys.argv[1:])

    try:
        classifier = SemanticClassifier()
        result = classifier.classify(text)

        print(json.dumps(result, ensure_ascii=False, indent=2))

        if result.get("success"):
            # Afficher un résumé lisible dans stderr
            print(f"\n✅ Catégorie: {result['category']}", file=sys.stderr)
            print(f"   Confiance: {result['confidence']*100:.1f}%", file=sys.stderr)
            if result.get('subcategory'):
                print(f"   Sous-catégorie: {result['subcategory']}", file=sys.stderr)
            print(f"\n📊 Distribution:", file=sys.stderr)
            for cat, score in sorted(result['all_scores'].items(), key=lambda x: x[1], reverse=True):
                print(f"   {cat}: {score:.1f}%", file=sys.stderr)

    except Exception as e:
        print(json.dumps({
            "success": False,
            "error": str(e)
        }), file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
