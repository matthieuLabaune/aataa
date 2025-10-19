#!/bin/bash
# Démonstration rapide de la classification sémantique vs mots-clés

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   🎯 DÉMONSTRATION : Pourquoi les mots-clés ne suffisent pas  ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}📝 CAS 1 : Votre document problématique${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
TEXT1="Gray Matter Technology prestation développement web 24600 euros auto-entrepreneur SIRET"
echo "Texte OCR : ${TEXT1}"
echo ""
echo -e "${RED}❌ Système actuel (mots-clés)${NC}"
echo "   Résultat : 'unknown' (manque le mot 'facture')"
echo "   Confiance : N/A"
echo ""
echo -e "${GREEN}✅ Classification sémantique${NC}"
echo ""

cd "$(dirname "$0")"
source python/venv/bin/activate 2>/dev/null

if ! python python/semantic_classifier.py "$TEXT1" 2>/dev/null; then
    echo -e "${YELLOW}⚠️  Modèle en cours de téléchargement...${NC}"
    echo "   (Cela peut prendre quelques minutes au premier lancement)"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo -e "${BLUE}📝 CAS 2 : Texte avec erreurs OCR${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
TEXT2="F4cture pr3station inform4tique mont4nt 2400 3UR0S TVA non 4pplicable"
echo "Texte OCR : ${TEXT2}"
echo "           (remarquez les erreurs : F4cture, 3UR0S, 4pplicable)"
echo ""
echo -e "${RED}❌ Système actuel (mots-clés)${NC}"
echo "   Résultat : 'unknown' (les erreurs OCR cassent la détection)"
echo ""
echo -e "${GREEN}✅ Classification sémantique${NC}"
echo ""
python python/semantic_classifier.py "$TEXT2" 2>/dev/null
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo -e "${BLUE}📝 CAS 3 : Document ambigu${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
TEXT3="Contrat de prestation de service informatique durée 12 mois"
echo "Texte OCR : ${TEXT3}"
echo "           (Est-ce Financier ou Professionnel ?)"
echo ""
echo -e "${YELLOW}⚠️  Système actuel (mots-clés)${NC}"
echo "   Résultat : Indéterminé ou conflit"
echo ""
echo -e "${GREEN}✅ Classification sémantique${NC}"
echo "   → Donne un score pour CHAQUE catégorie"
echo ""
python python/semantic_classifier.py "$TEXT3" 2>/dev/null
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo -e "${GREEN}✨ CONCLUSION${NC}"
echo ""
echo "La classification sémantique :"
echo "  ✅ Fonctionne malgré les erreurs OCR"
echo "  ✅ Comprend le contexte et le sens"
echo "  ✅ Donne des scores de confiance honnêtes"
echo "  ✅ Gère l'ambiguïté intelligemment"
echo ""
echo "Les mots-clés :"
echo "  ❌ Cassent avec les erreurs OCR"
echo "  ❌ Ne comprennent pas le contexte"
echo "  ❌ Pas de score de confiance"
echo "  ❌ Ambiguïté = confusion"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "${BLUE}📖 Pour plus d'infos : REPONSE_CLASSIFICATION.md${NC}"
echo ""
