# 🧪 Guide de Test - Export/Import Corrigé

## 🎯 Objectif
Vérifier que les corrections des dates (1980) et des IDs dupliqués fonctionnent correctement.

---

## ✅ Test 1 : Dates Correctes dans le ZIP

### Étapes
1. **Ouvre l'app AATAA** (déjà lancée)
2. **Va dans Paramètres** → Section "Sauvegarde & Export"
3. **Clique sur "Exporter"**
4. **Choisis un emplacement** : par exemple `~/Desktop/AATAA_Test.zip`
5. **Attends la confirmation** : "Backup created successfully"

### Vérification des Dates

**Dans le Finder** :
- Ouvre le ZIP (double-clic)
- Navigue dans `documents/Autre/` ou `documents/Finance/`
- **Vérifie les dates** des fichiers

**Résultat attendu** ✅ :
```
DOC_20251012_190332.pdf     19 oct. 2025 à 18:05  ✅
Facture_EDF_2024-06.pdf     15 juin 2024 à 14:30  ✅
```

**Résultat INCORRECT** ❌ (bug ancien) :
```
DOC_20251012_190332.pdf     1 janv. 1980 à 00:00  ❌
```

---

## ✅ Test 2 : Import Sans Erreur UNIQUE Constraint

### Étapes
1. **Garde le ZIP créé** au Test 1
2. **Importe-le une première fois** :
   - Paramètres → "Importer une sauvegarde"
   - Sélectionne le ZIP
   - **Résultat attendu** : "Backup restored successfully: X documents imported" ✅

3. **Importe-le UNE DEUXIÈME FOIS** (même fichier) :
   - Paramètres → "Importer une sauvegarde"
   - Sélectionne le même ZIP
   - **Résultat attendu** : "Backup restored successfully: X documents imported" ✅
   - **PAS d'erreur** "UNIQUE constraint failed" ✅

### Vérification dans la Liste

**Après le 2e import**, tu devrais voir :
- **Doublons** des mêmes documents (nom identique)
- **IDs différents** (invisibles mais présents en base)

**Exemple** :
```
📄 Facture_EDF_2024-06.pdf (original)
📄 Facture_EDF_2024-06.pdf (importé 1re fois)
📄 Facture_EDF_2024-06.pdf (importé 2e fois)
```

Tous ont des **UUIDs différents** → pas de conflit ! ✅

---

## ✅ Test 3 : Intégrité des Données

### Vérifier le Contenu
1. **Ouvre un document** importé (double-clic dans la liste)
2. **Compare avec l'original**
3. **Résultat attendu** : Contenu identique, aucune corruption ✅

### Vérifier les Métadonnées
1. **Sélectionne un document** importé
2. **Vérifie** :
   - Catégorie : correcte ✅
   - Sous-catégorie : correcte ✅
   - Tags : corrects ✅
   - Notes : correctes ✅

---

## 🐛 Si Problème Rencontré

### Erreur "Dates toujours en 1980"
**Diagnostic** : La correction des dates n'a pas fonctionné.

**Action** :
```bash
# Vérifier que le code est bien compilé
cd ~/Documents/sites/aataa
cargo build --manifest-path=src-tauri/Cargo.toml
```

Rechercher dans le code :
```rust
// Devrait être présent dans backup.rs
.last_modified_time(
    zip::DateTime::from_date_and_time(...)
)
```

---

### Erreur "UNIQUE constraint failed"
**Diagnostic** : La génération de nouveaux UUIDs n'a pas fonctionné.

**Vérifier** dans `backup.rs` ligne ~265 :
```rust
// DOIT contenir cette ligne :
restored_doc.id = Uuid::new_v4().to_string();
```

**Si absent**, relancer la compilation.

---

### Erreur "Failed to add file to ZIP: Duplicate filename"
**Diagnostic** : Fix des doublons pas appliqué.

**Solution** : Cette erreur a été corrigée dans le commit précédent. Vérifier la présence de :
```rust
let mut added_files = std::collections::HashSet::new();
// ... gestion des suffixes _1, _2, etc.
```

---

## 📊 Résultats Attendus - Récapitulatif

| Test           | Objectif        | Résultat Attendu           |
| -------------- | --------------- | -------------------------- |
| 1. Export      | Dates correctes | ✅ Dates réelles (pas 1980) |
| 2. Import 1x   | Restauration    | ✅ "X documents imported"   |
| 3. Import 2x   | Pas de conflit  | ✅ Pas d'erreur UNIQUE      |
| 4. Contenu     | Intégrité       | ✅ Fichiers identiques      |
| 5. Métadonnées | Conservation    | ✅ Catégories/tags OK       |

---

## 🎉 Si Tous les Tests Passent

**Félicitations !** Les bugs sont corrigés. Tu peux maintenant :

1. **Créer des backups réguliers** en toute confiance
2. **Importer plusieurs fois** sans erreur
3. **Migrer vers un nouveau Mac** facilement

### Utilisation Recommandée

**Backup mensuel** :
```
AATAA_Backup_2025-01.zip
AATAA_Backup_2025-02.zip
AATAA_Backup_2025-03.zip
```

**Backup avant modifications majeures** :
```
AATAA_Backup_Before_Migration.zip
AATAA_Backup_Before_Cleanup.zip
```

---

## 📝 Rapporter les Résultats

Après tes tests, note :
- ✅ ou ❌ pour chaque test
- Toute erreur rencontrée
- Comportement inattendu

Cela aidera à identifier s'il reste des edge cases à gérer !

---

**Bon test !** 🚀
