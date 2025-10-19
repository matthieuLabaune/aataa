# 🧪 Guide de test - Système de catégorisation à 3 niveaux

## ✅ Phase 6 - Tests finaux

### 🚀 1. Lancer l'application

```bash
cd /Users/matt/Documents/sites/aataa
npm run tauri dev
```

---

## 📋 Tests à effectuer

### Test 1 : Page Settings - Gestion des catégories

**Accès** : Cliquer sur ⚙️ Paramètres

**Vérifications** :
- [ ] Section "Catégories et sous-catégories" visible
- [ ] 8 catégories principales affichées avec icônes :
  - 📄 Administratif
  - 💼 Financier
  - 🏥 Santé
  - 💼 Professionnel
  - 🏠 Immobilier
  - 🎓 Académique
  - 👤 Personnel
  - 📁 Autre
- [ ] Sous-catégories prédéfinies visibles (gris clair)
- [ ] Bouton "+ Ajouter" présent par catégorie

**Actions** :
1. [ ] Cliquer sur "+ Ajouter" sous "Financier"
2. [ ] Taper "Facture Amazon"
3. [ ] Appuyer sur Entrée ou cliquer ✓
4. [ ] Vérifier que "Facture Amazon" apparaît en gris foncé
5. [ ] Cliquer sur ❌ pour supprimer
6. [ ] Vérifier que la suppression fonctionne
7. [ ] **Important** : Les sous-catégories prédéfinies n'ont pas de bouton ❌

---

### Test 2 : Import de documents

**Surveiller le terminal** pour voir les logs de classification :
```
✓ Catégorie: Financier (score: 87.5%)
  → Sous-catégorie suggérée: Facture énergie
```

#### Document A : Facture EDF
- [ ] Importer une facture EDF (ou créer une image de test)
- [ ] **Attendu** :
  - Catégorie : Financier (badge noir)
  - Sous-catégorie : "Facture énergie"
  - Tags : "2024", montant, "EDF"
  - Score > 70% dans les logs

#### Document B : Ordonnance
- [ ] Importer une ordonnance (ou texte contenant "ordonnance", "pharmacie")
- [ ] **Attendu** :
  - Catégorie : Santé (badge rouge)
  - Sous-catégorie : "Ordonnance"
  - Score > 60%

#### Document C : Fiche de paie
- [ ] Importer une fiche de paie (ou texte "fiche de paie", "salaire")
- [ ] **Attendu** :
  - Catégorie : Professionnel (badge gris foncé)
  - Sous-catégorie : "Fiche de paie"

#### Document D : Image quelconque
- [ ] Importer une photo simple sans texte pertinent
- [ ] **Attendu** :
  - Catégorie : Autre (badge gris clair)
  - Score < 60% → Logs montrent "Classification incertaine"

---

### Test 3 : Affichage dans Home

**Vérifications sur les cartes de documents** :
- [ ] Badge de catégorie visible en haut à gauche
- [ ] Icône Material correcte (description, account_balance, medical_services...)
- [ ] Nom de la catégorie affiché
- [ ] Couleur de fond du badge correspondant à la catégorie
- [ ] Sous-catégorie affichée en italique gris (si présente)
- [ ] Type de document (chip gris)
- [ ] Tags affichés en bas (max 3 + compteur)

**Actions** :
1. [ ] Cliquer sur un document pour ouvrir le modal
2. [ ] Vérifier que le modal affiche :
   - Catégorie avec icône
   - Sous-catégorie (si présente)
   - Type
   - Taille, Date
   - Liste complète des tags
   - Texte OCR

---

## 🔍 Vérification des détails

### Couleurs des badges par catégorie

| Catégorie     | Couleur         | Code    |
| ------------- | --------------- | ------- |
| Administratif | Gris            | #757575 |
| Financier     | Noir            | #000000 |
| Santé         | Rouge           | #D32F2F |
| Professionnel | Gris foncé      | #424242 |
| Immobilier    | Gris moyen      | #616161 |
| Académique    | Presque noir    | #212121 |
| Personnel     | Gris clair      | #9E9E9E |
| Autre         | Très gris clair | #BDBDBD |

### Icônes Material par catégorie

| Catégorie     | Icône            |
| ------------- | ---------------- |
| Administratif | description      |
| Financier     | account_balance  |
| Santé         | medical_services |
| Professionnel | work             |
| Immobilier    | home             |
| Académique    | school           |
| Personnel     | person           |
| Autre         | folder           |

---

## 🐛 Problèmes potentiels

### Si les sous-catégories ne s'affichent pas

```bash
# Vérifier la base de données
cd ~/Library/Application\ Support/com.aataa.app/
sqlite3 aataa.db "SELECT COUNT(*) FROM subcategories;"
```

**Devrait retourner** : 27

**Si 0** : Relancer l'app, la migration se fait au démarrage

### Si la classification ne fonctionne pas

- Vérifier les logs du terminal
- Score affiché devrait être entre 0% et 100%
- Si toujours "Autre" → Score < 60%
- Vérifier que le texte OCR est extrait (modal → Texte OCR)

### Si le badge ne s'affiche pas

- Vérifier que `doc.category` existe (modal → devrait afficher la catégorie)
- Console du navigateur (F12) → Vérifier les erreurs
- Vérifier que les fonctions `getCategoryIcon()` et `getCategoryColor()` existent dans Home.vue

---

## 📊 Rapport de test à compléter

### Documents testés

| Fichier | Catégorie obtenue | Score | Sous-catégorie | Tags | Badge visible | ✅/❌ |
| ------- | ----------------- | ----- | -------------- | ---- | ------------- | --- |
|         |                   |       |                |      |               |     |
|         |                   |       |                |      |               |     |
|         |                   |       |                |      |               |     |

### Fonctionnalités testées

- [ ] Import de document
- [ ] Classification automatique
- [ ] Affichage du badge de catégorie
- [ ] Affichage de la sous-catégorie
- [ ] Tags extraits et affichés
- [ ] Page Settings - liste des catégories
- [ ] Ajout de sous-catégorie personnalisée
- [ ] Suppression de sous-catégorie personnalisée
- [ ] Modal de détails complet
- [ ] Logs de classification dans terminal

### Bugs trouvés

-

### Points positifs

-

### Suggestions

-

---

## ✅ Critères de validation

Pour que le système soit validé, il faut :

- [ ] **Au moins 70% de bonnes classifications** sur documents tests
- [ ] **Badges visibles** et couleurs correctes
- [ ] **Sous-catégories** suggérées pertinentes (min 50%)
- [ ] **Tags extraits** pour 80% des documents
- [ ] **UI Settings** fonctionnelle (ajout/suppression)
- [ ] **Aucune erreur** en console ou terminal
- [ ] **Compilation** réussie (frontend + backend)

---

## 🎯 Si tout fonctionne

Vous êtes prêt pour créer le commit final ! 🎉

Le système de catégorisation à 3 niveaux est **entièrement fonctionnel** :
- ✅ 8 catégories principales
- ✅ Sous-catégories illimitées personnalisables
- ✅ Tags libres avec extraction automatique
- ✅ Classification intelligente avec scoring
- ✅ Interface utilisateur complète

---

## 🚀 Prochaines étapes (après validation)

1. Créer le commit final
2. Merger sur master
3. Tagger une release v2.0.0 (système de catégories)
4. Mettre à jour la documentation utilisateur
5. Préparer des captures d'écran pour le README

**Bon test !** 🧪
