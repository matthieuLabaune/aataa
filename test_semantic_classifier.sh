#!/bin/bash
# Script de test de la classification sémantique

cd "$(dirname "$0")/.."

echo "🧪 Test de la Classification Sémantique"
echo "========================================"
echo ""

# Activer l'environnement virtuel
source python/venv/bin/activate

echo "📋 Test 1 : Facture freelance (votre cas problématique)"
echo "Texte: 'Facture Gray Matter Technology Prestation développement 24600 euros auto-entrepreneur'"
echo ""
python python/semantic_classifier.py "Facture Gray Matter Technology Prestation développement 24600 euros auto-entrepreneur"
echo ""
echo "---"
echo ""

echo "📋 Test 2 : Facture simple"
echo "Texte: 'Facture EDF électricité montant 150 euros'"
echo ""
python python/semantic_classifier.py "Facture EDF électricité montant 150 euros"
echo ""
echo "---"
echo ""

echo "📋 Test 3 : Bulletin de paie"
echo "Texte: 'Bulletin de salaire employeur net à payer cotisations'"
echo ""
python python/semantic_classifier.py "Bulletin de salaire employeur net à payer cotisations"
echo ""
echo "---"
echo ""

echo "📋 Test 4 : Document administratif"
echo "Texte: 'Carte nationale identité république française'"
echo ""
python python/semantic_classifier.py "Carte nationale identité république française"
echo ""
echo "---"
echo ""

echo "📋 Test 5 : Document avec erreurs OCR"
echo "Texte: 'F4cture pr3station inform4tique mont4nt 2400 3UR0S'"
echo ""
python python/semantic_classifier.py "F4cture pr3station inform4tique mont4nt 2400 3UR0S"
echo ""
echo "---"
echo ""

echo "✅ Tests terminés !"
