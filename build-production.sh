#!/bin/bash
# Script de build de production pour AATAA

echo "🚀 Build de production AATAA"
echo "=============================="
echo ""

cd "$(dirname "$0")"

echo "📋 Étape 1/4 : Nettoyage des builds précédents..."
rm -rf src-tauri/target/release/bundle
echo "✅ Nettoyage terminé"
echo ""

echo "📦 Étape 2/4 : Build de production Tauri..."
npm run tauri build
BUILD_STATUS=$?

if [ $BUILD_STATUS -ne 0 ]; then
    echo "❌ Erreur lors du build"
    exit 1
fi
echo "✅ Build réussi"
echo ""

echo "📊 Étape 3/4 : Résumé des fichiers générés..."
echo ""

if [ -f "src-tauri/target/release/bundle/macos/AATAA.app.tar.gz" ]; then
    echo "✅ Application bundle:"
    ls -lh src-tauri/target/release/bundle/macos/AATAA.app.tar.gz
fi

if [ -f "src-tauri/target/release/bundle/dmg/"*.dmg ]; then
    echo "✅ Installeur DMG:"
    ls -lh src-tauri/target/release/bundle/dmg/*.dmg
fi

if [ -d "src-tauri/target/release/bundle/macos/AATAA.app" ]; then
    echo "✅ Application prête:"
    du -sh src-tauri/target/release/bundle/macos/AATAA.app
fi

echo ""
echo "🎉 Étape 4/4 : Installation des scripts Python dans l'app..."

# Copier les scripts Python dans l'app
if [ -d "src-tauri/target/release/bundle/macos/AATAA.app" ]; then
    APP_PATH="src-tauri/target/release/bundle/macos/AATAA.app/Contents/Resources"
    mkdir -p "$APP_PATH/python"
    cp python/*.py "$APP_PATH/python/"
    cp python/requirements.txt "$APP_PATH/python/"
    echo "✅ Scripts Python copiés dans l'app"
else
    echo "⚠️  App bundle non trouvée, scripts Python non copiés"
fi

echo ""
echo "=============================="
echo "✨ Build terminé avec succès !"
echo ""
echo "📍 Fichiers générés :"
echo "   - Application : src-tauri/target/release/bundle/macos/AATAA.app"
echo "   - Installeur  : src-tauri/target/release/bundle/dmg/AATAA_*.dmg"
echo ""
echo "🚀 Pour tester l'app :"
echo "   open src-tauri/target/release/bundle/macos/AATAA.app"
echo ""
echo "📦 Pour distribuer :"
echo "   Partagez le fichier .dmg"
echo ""
