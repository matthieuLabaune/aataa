#!/bin/bash

# Script d'installation des dépendances Python pour ML OCR

echo "🚀 Installation des dépendances Python pour ML OCR..."

# Vérifier si Python 3 est installé
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 n'est pas installé. Veuillez l'installer d'abord."
    exit 1
fi

echo "✅ Python 3 trouvé: $(python3 --version)"

# Créer le dossier python s'il n'existe pas
mkdir -p python
cd python

# Créer un environnement virtuel
echo "📦 Création de l'environnement virtuel..."
python3 -m venv venv

# Activer l'environnement virtuel
echo "🔧 Activation de l'environnement virtuel..."
source venv/bin/activate

# Mettre à jour pip
echo "⬆️ Mise à jour de pip..."
pip install --upgrade pip

# Installer les dépendances
echo "📥 Installation des dépendances..."
echo "   - torch (peut prendre quelques minutes...)"
echo "   - transformers"
echo "   - Pillow"
echo "   - accelerate"
echo ""

pip install torch>=2.0.0 transformers>=4.30.0 Pillow>=10.0.0 accelerate>=0.20.0

echo ""
echo "✅ Installation terminée!"
echo ""
echo "Les modèles seront téléchargés automatiquement au premier usage."
echo "Taille approximative des modèles:"
echo "  - TrOCR Manuscrit: ~1.4GB"
echo "  - TrOCR Imprimé: ~1.4GB"
echo "  - BLIP Caption: ~990MB"
echo ""
echo "Emplacement du cache: ~/.cache/huggingface/"
echo ""
echo "Pour tester, exécutez:"
echo "  cd python"
echo "  source venv/bin/activate"
echo "  python trocr_handwritten.py <chemin_image> handwritten"
echo ""
