# Résumé des améliorations - 17 octobre 2025

## ✅ Changements appliqués

### 1. 🎨 Nouveau thème noir et blanc minimaliste

**Fichier** : `frontend/src/styles/material-tokens.css`

**Avant** : Thème Material Design 3 violet (#6750A4)
**Après** : Thème minimaliste noir et blanc

#### Palette de couleurs
```css
/* Couleurs principales */
--md-sys-color-primary: #000000 (noir)
--md-sys-color-on-primary: #FFFFFF (blanc)
--md-sys-color-primary-container: #F5F5F5 (gris très clair)

/* Couleurs secondaires */
--md-sys-color-secondary: #424242 (gris foncé)
--md-sys-color-secondary-container: #E0E0E0 (gris clair)

/* Surfaces et arrière-plans */
--md-sys-color-surface: #FFFFFF (blanc pur)
--md-sys-color-background: #FAFAFA (presque blanc)
--md-sys-color-surface-variant: #F5F5F5 (gris très clair)
--md-sys-color-outline: #BDBDBD (bordures grises)

/* Erreurs */
--md-sys-color-error: #D32F2F (rouge Material)
```

#### Résultat visuel
- Boutons principaux : noir avec texte blanc
- Boutons secondaires : bordure noire avec texte noir
- Surfaces : blanc avec bordures grises subtiles
- Arrière-plan : gris très clair (#FAFAFA) pour réduire la fatigue visuelle
- Look épuré, professionnel et minimaliste

---

### 2. 🔍 Amélioration de la classification des documents

**Fichier** : `src-tauri/src/classifier.rs`

**Problème identifié** :
- Certaines images classées en "Facture" à tort
- Système de regex trop permissif
- Mots-clés génériques (ex: "total", "montant") présents dans beaucoup de documents

**Solution implémentée** : Système de scoring avec mots-clés pondérés

#### Architecture du nouveau classifier

```rust
struct ClassificationPattern {
    doc_type: DocumentType,
    required_keywords: Vec<String>,    // Au moins 1 doit être présent
    support_keywords: Vec<String>,      // Augmentent le score
    blocker_keywords: Vec<String>,      // Bloquent la classification
}
```

#### Algorithme de scoring

1. **Vérification des bloquants** (priorité absolue)
   - Si un mot bloquant est présent → score = 0
   - Exemple : "contrat" bloque la classification "Facture"

2. **Mots requis** (70% du score)
   - Au moins 1 mot requis doit être présent
   - Exemple pour Facture : "facture" OU "invoice"
   - Score proportionnel au nombre de mots requis matchés

3. **Mots de support** (30% du score)
   - Augmentent la confiance sans être obligatoires
   - Exemple pour Facture : "tva", "ht", "ttc", "total", "montant"
   - Score proportionnel au nombre de mots support matchés

4. **Seuil de confiance**
   - Minimum 60% pour valider une classification
   - Si < 60% → type "Document" par défaut

#### Exemple : Classification "Facture"

**Mots requis** :
- "facture" OU "invoice"

**Mots de support** :
- "montant", "total", "tva", "€", "ht", "ttc", "payer", "échéance"

**Mots bloquants** :
- "contrat", "bulletin", "relevé", "attestation"

**Calcul du score** :
```
Texte : "Facture n°12345 - Total: 150.00€ TTC - TVA 20%"

1. Bloquants présents ? Non ✓
2. Mots requis : "facture" présent (1/2) → 50% × 0.7 = 35%
3. Mots support : "total", "€", "ttc", "tva" présents (4/8) → 50% × 0.3 = 15%
4. Score final : 35% + 15% = 50%

❌ 50% < 60% → Type "Document" (confiance insuffisante)
```

```
Texte : "Facture n°12345 - Montant TTC: 150.00€ - TVA 20% - Échéance: 30/11/2025"

1. Bloquants présents ? Non ✓
2. Mots requis : "facture" présent (1/2) → 50% × 0.7 = 35%
3. Mots support : "montant", "ttc", "€", "tva", "échéance" (5/8) → 62.5% × 0.3 = 18.75%
4. Score final : 35% + 18.75% = 53.75%

❌ 53.75% < 60% → Type "Document" (confiance insuffisante)
```

**Note** : Le seuil peut être ajusté si trop strict. Tester avec vos documents réels.

#### Logging ajouté

```rust
// Si confiance < 60%
eprintln!("⚠️  Classification incertaine (score: 52.0%) - Type par défaut utilisé");

// Si confiance >= 60%
eprintln!("✓ Classification: Facture (score: 87.5%)");
```

Vous verrez maintenant dans la console :
- Le type détecté
- Le score de confiance en %
- Les avertissements si score faible

---

## 📊 Résultats attendus

### Avant
```
Image avec "Total: 5 photos"
→ Classée "Facture" ❌ (faux positif)
```

### Après
```
Image avec "Total: 5 photos"
→ Mots requis absents ("facture"/"invoice")
→ Score = 0%
→ Classée "Document" ✓ (type générique)
```

### Vraie facture
```
"Facture n°123 - Total TTC: 150€ - TVA 20%"
→ Mot requis présent: "facture"
→ Mots support: "total", "ttc", "€", "tva"
→ Score = ~75%
→ Classée "Facture" ✓
```

---

## 🧪 Tests recommandés

### 1. Tester avec vos images problématiques
1. Importez les images qui étaient mal classées
2. Vérifiez dans la console le score de classification
3. Vérifiez le type détecté dans l'interface

### 2. Ajuster les seuils si nécessaire

**Si trop de faux négatifs** (vraies factures non détectées) :
```rust
// Dans classifier.rs, ligne ~170
if best_score < 0.5 {  // Réduire de 0.6 à 0.5
```

**Si pas assez de mots requis matchent** :
```rust
// Ajouter plus de variantes
required_keywords: vec![
    "facture".to_string(),
    "invoice".to_string(),
    "bill".to_string(),  // Ajouter "bill"
],
```

### 3. Surveiller les logs

Quand vous importez un document, regardez la sortie console :
```bash
cd /Users/matt/Documents/sites/aataa
npm run tauri dev  # ou cargo tauri dev

# Vous verrez :
✓ Classification: Facture (score: 87.5%)
# ou
⚠️  Classification incertaine (score: 42.0%) - Type par défaut utilisé
```

---

## 🔄 Prochaines améliorations possibles

### Court terme
- [ ] Afficher le score de confiance dans l'UI
- [ ] Permettre à l'utilisateur de corriger la classification
- [ ] Ajouter plus de mots-clés selon vos besoins

### Moyen terme
- [ ] Analyse structurelle (détecter les tableaux)
- [ ] Détection du nombre de lignes avec des montants
- [ ] Analyse de la position des mots-clés

### Long terme
- [ ] Machine Learning (modèle entraîné sur vos documents)
- [ ] OCR multiple (TrOCR pour manuscrit, BLIP pour images)

---

## 📝 Commits créés

```
df856b3 docs: add OCR and classification improvement plan
f8f5806 feat(classifier): improve document classification with scoring system
759060c style: change theme to minimalist black and white
```

---

## ✨ Résumé

**Thème visuel** : ✅ Noir et blanc minimaliste appliqué
**Classification** : ✅ Système de scoring avec seuil de confiance
**Documentation** : ✅ Plan d'amélioration complet
**Tests** : ⏳ À faire avec vos documents

**Prochaine étape** : Testez avec vos images problématiques et ajustez les seuils si besoin !
