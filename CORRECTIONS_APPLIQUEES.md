# ✅ CORRECTIONS APPLIQUÉES

## 🎯 Résumé des 3 problèmes et solutions

### 1️⃣ ✅ CORRIGÉ - Ouverture de fichiers

**Problème :** `invalid args 'filePath' for command open_file`

**Cause :** Incohérence entre camelCase (JS) et snake_case (Rust)

**Solution appliquée :**
```typescript
// app/app.vue ligne 846
await invoke('open_file', { file_path: filePath })  // ✅ Corrigé
```

**Test :**
1. Ouvrez l'app
2. Cliquez sur "Ouvrir le fichier" dans la modale de prévisualisation
3. Le fichier devrait s'ouvrir avec l'application par défaut

---

### 2️⃣ ✅ AJOUTÉ - Commandes de réinitialisation

**Problème :** Pas de moyen de réinitialiser l'application

**Solutions ajoutées :**

#### Option 1 : Vider la liste (garder fichiers)
```typescript
await invoke('clear_database')
```
- ✅ Vide la base de données
- ✅ **Garde** les fichiers sur le disque
- Utile pour : Tests, recréer l'index

#### Option 2 : Reset complet
```typescript
await invoke('reset_application')
```
- ✅ Vide la base de données
- ✅ **Supprime** tous les fichiers archivés
- ✅ Réinitialise les sous-catégories/mots-clés personnalisés
- ✅ **Garde** les paramètres prédéfinis
- Utile pour : Repartir à zéro proprement

**Utilisation dans l'UI :**

```vue
<!-- Ajouter dans app.vue (section Paramètres) -->
<div class="danger-zone" style="margin-top: 2rem; padding-top: 2rem; border-top: 2px solid #ef4444">
  <h3 style="color: #ef4444; margin-bottom: 1rem">⚠️ Zone dangereuse</h3>

  <button
    @click="clearDatabase"
    class="btn btn-secondary"
    style="width: 100%; margin-bottom: 0.5rem; background: #f59e0b; border-color: #f59e0b"
  >
    🗑️ Vider la liste (garder fichiers)
  </button>

  <button
    @click="resetApplication"
    class="btn btn-secondary"
    style="width: 100%; background: #ef4444; border-color: #ef4444; color: white"
  >
    🔄 Réinitialisation complète (TOUT supprimer)
  </button>
</div>

<script setup>
async function clearDatabase() {
  if (!confirm('Vider la liste des documents ?\n\nLes fichiers resteront sur le disque.')) {
    return
  }

  try {
    await invoke('clear_database')
    documents.value = []
    showMessage('✅ Liste vidée (fichiers conservés)', 3000)
  } catch (error) {
    showMessage('❌ Erreur: ' + error, 5000)
  }
}

async function resetApplication() {
  if (!confirm('⚠️ ATTENTION ⚠️\n\nCette action va :\n- Supprimer TOUS les documents de la liste\n- Supprimer TOUS les fichiers archivés\n- Réinitialiser les paramètres personnalisés\n\nCette action est IRRÉVERSIBLE !\n\nContinuer ?')) {
    return
  }

  try {
    await invoke('reset_application')
    documents.value = []
    showMessage('✅ Application réinitialisée', 3000)
    setTimeout(() => location.reload(), 1500)
  } catch (error) {
    showMessage('❌ Erreur: ' + error, 5000)
  }
}
</script>
```

---

### 3️⃣ ⚠️ EN COURS - Améliorer extraction de tags

**Problème :**
- Montant affiché : `24600.00€` au lieu de `24 600.00 €`
- Type : "Unknown" au lieu de "Facture"
- Tags manquants : Nom client, N° devis

**Solutions à venir :**

#### Court terme : Améliorer le classifier actuel
- Patterns plus flexibles pour détecter "Total" au lieu de "Total HT"
- Formatage des montants avec espaces

#### Moyen terme : Classification sémantique
- Utiliser `semantic_classifier.py` créé précédemment
- Compréhension du contexte au lieu de mots-clés stricts
- Robuste aux variations OCR

**Status :** Les scripts Python sont prêts, intégration à faire

---

## 🧪 Tests à effectuer

### Test 1 : Ouverture de fichiers
```bash
# Compiler l'app
cd src-tauri
cargo build

# Lancer l'app
npm run tauri dev

# Dans l'app :
1. Cliquer sur un document
2. Dans la modale, cliquer "Ouvrir le fichier"
3. ✅ Le PDF devrait s'ouvrir
```

### Test 2 : Vider la liste
```bash
# Dans l'app :
1. Aller dans Paramètres
2. Cliquer "Vider la liste"
3. Confirmer
4. ✅ Liste vide, mais fichiers toujours dans ~/Library/Application Support/aataa/archive
```

### Test 3 : Reset complet
```bash
# Dans l'app :
1. Aller dans Paramètres
2. Cliquer "Réinitialisation complète"
3. Confirmer (double confirmation recommandée)
4. ✅ Liste vide ET dossier archive vide
```

---

## 📂 Fichiers modifiés

- ✅ `app/app.vue` - Correction openDocument (ligne 846)
- ✅ `src-tauri/src/commands.rs` - Ajout clear_database + reset_application
- ✅ `src-tauri/src/lib.rs` - Enregistrement des nouvelles commandes

---

## 🔄 Prochaines étapes

1. **Compiler et tester** les corrections
   ```bash
   npm run tauri dev
   ```

2. **Ajouter l'UI de reset** dans la section Paramètres (code fourni ci-dessus)

3. **Optionnel : Améliorer l'extraction de tags**
   - Soit améliorer le classifier actuel
   - Soit intégrer la classification sémantique

4. **Tester la classification sémantique** (déjà prête)
   ```bash
   cd /Users/matt/Documents/sites/aataa
   ./demo_classification.sh
   ```

---

## ❓ Questions restantes

1. **Affichage des tags** : Voulez-vous les rendre plus visibles sur les cartes de documents ?
2. **Classification** : Préférez-vous améliorer le système actuel ou basculer vers la classification sémantique ?
3. **Montants** : Faut-il améliorer le formatage (espaces pour milliers) ?

---

**Résumé : L'ouverture de fichiers est corrigée, les commandes de reset sont ajoutées. Il reste à compiler et tester !** 🚀
