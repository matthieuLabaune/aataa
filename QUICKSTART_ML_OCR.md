# 🚀 Démarrage rapide - ML OCR

## Installation en 3 étapes

### 1️⃣ Installer les dépendances Python

```bash
./install_ml_dependencies.sh
```

**Ce qui sera installé :**
- Environnement virtuel Python dans `/python/venv`
- PyTorch, Transformers, Pillow
- ~200MB de dépendances

### 2️⃣ (Optionnel) Tester les scripts

```bash
cd python
source venv/bin/activate

# Test avec une image de votre choix
python trocr_handwritten.py /path/to/image.jpg handwritten
python blip_caption.py /path/to/image.jpg fr
```

### 3️⃣ Lancer l'application

```bash
npm run tauri dev
```

## 🎯 Utilisation

1. Dans l'interface, sélectionnez le type d'analyse :
   - **Standard** : Tesseract classique (rapide)
   - **Manuscrit** : Pour écriture à la main
   - **Imprimé ML** : IA pour texte imprimé
   - **Description** : Génère une description de l'image

2. Glissez-déposez votre fichier

3. Le traitement démarre automatiquement !

## ⚠️ Premier lancement

Au premier usage de chaque mode ML :
- Les modèles seront téléchargés (~3-4GB total)
- Peut prendre 5-10 minutes selon votre connexion
- Les modèles sont ensuite mis en cache

## 📚 Documentation complète

- `ML_OCR_SETUP.md` - Guide détaillé d'installation et dépannage
- `ML_OCR_IMPLEMENTATION.md` - Détails techniques de l'implémentation
- `python/README.md` - Documentation des scripts Python

## 🆘 Problème ?

**Python introuvable ?**
```bash
# macOS
brew install python3

# Ubuntu/Debian
sudo apt-get install python3 python3-pip
```

**Script Python introuvable ?**
- Vérifiez que vous lancez l'app depuis le dossier racine du projet

**Modèle trop lent ?**
- Utilisez le mode "Standard" pour les documents simples
- Fermez les autres applications
- 4GB RAM minimum recommandé

---

**Besoin d'aide ?** Consultez `ML_OCR_SETUP.md` pour le dépannage détaillé.
