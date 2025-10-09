#!/bin/bash

# AATAA Quick Start Script

echo "🚀 AATAA - Auto Archive Documents"
echo "=================================="
echo ""

# Check Tesseract
echo "🔍 Vérification de Tesseract..."
if ! command -v tesseract &> /dev/null; then
    echo "❌ Tesseract n'est pas installé!"
    echo ""
    echo "Pour installer Tesseract:"
    echo "  macOS:   brew install tesseract tesseract-lang"
    echo "  Ubuntu:  sudo apt-get install tesseract-ocr tesseract-ocr-fra"
    echo "  Windows: Télécharger depuis https://github.com/UB-Mannheim/tesseract/wiki"
    exit 1
else
    echo "✅ Tesseract installé: $(tesseract --version | head -1)"
fi

echo ""
echo "📦 Installation des dépendances..."

# Install Node dependencies
if [ ! -d "node_modules" ]; then
    echo "   → Installation des packages npm..."
    npm install
else
    echo "   ✅ node_modules déjà présent"
fi

echo ""
echo "🔨 Compilation du backend Rust..."

# Build Rust backend
cd src-tauri
if cargo build 2>&1 | tail -5; then
    echo "   ✅ Backend compilé avec succès"
else
    echo "   ⚠️  Des warnings peuvent apparaître, c'est normal"
fi
cd ..

echo ""
echo "✅ Configuration terminée!"
echo ""
echo "Pour lancer l'application:"
echo "  npm run tauri:dev"
echo ""
echo "Pour compiler en production:"
echo "  npm run tauri:build"
echo ""
