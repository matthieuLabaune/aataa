# 🔧 Fix : Erreur Export - Duplicate Filename

## ❌ Problème

**Erreur** : `Failed to add file to ZIP: invalid Zip archive: Duplicate filename`

L'export échouait lorsque plusieurs documents avaient le même nom dans le ZIP, ce qui arrive dans plusieurs cas :

1. **Même nom original** : Deux factures nommées `Facture.pdf`
2. **Même classification** : Plusieurs documents du même type/année
3. **Renommage identique** : Le classifier génère le même nom

Exemple :
```
documents/Finance/Facture_EDF_2024-06.pdf  ← Premier document
documents/Finance/Facture_EDF_2024-06.pdf  ← DOUBLON ! ❌
```

---

## ✅ Solution Implémentée

### Détection et résolution automatique des doublons

**Fichier** : `src-tauri/src/backup.rs` (lignes 100-140)

**Mécanisme** :
1. HashSet `added_files` pour tracker les chemins déjà utilisés
2. Détection de collision avant d'ajouter au ZIP
3. Ajout d'un suffixe numérique automatique : `_1`, `_2`, etc.
4. Préservation de l'extension de fichier

**Code** :
```rust
let mut added_files = std::collections::HashSet::new();

for doc in &active_docs {
    // ... lecture du fichier source ...

    // Gérer les noms dupliqués
    let mut zip_path = format!("documents/{}/{}", category, doc.new_name);
    let mut counter = 1;

    while added_files.contains(&zip_path) {
        // Extraire nom de base et extension
        let stem = path.file_stem().unwrap_or("file");
        let ext = path.extension().unwrap_or("");

        // Créer nouveau nom avec suffixe
        let new_name = format!("{}_{}.{}", stem, counter, ext);
        zip_path = format!("documents/{}/{}", category, new_name);
        counter += 1;
    }

    added_files.insert(zip_path.clone());
    // ... ajout au ZIP ...
}
```

---

## 📦 Résultat

Les doublons sont automatiquement renommés :

**Avant (erreur)** :
```
documents/Finance/Facture_EDF_2024-06.pdf
documents/Finance/Facture_EDF_2024-06.pdf  ❌ ERREUR
```

**Après (résolu)** :
```
documents/Finance/Facture_EDF_2024-06.pdf
documents/Finance/Facture_EDF_2024-06_1.pdf  ✅
documents/Finance/Facture_EDF_2024-06_2.pdf  ✅
```

---

## 🧪 Test

Pour tester le fix :

1. **Importer plusieurs documents avec le même nom**
   ```bash
   # Copier 3 fois le même fichier
   cp facture.pdf facture_copy1.pdf
   cp facture.pdf facture_copy2.pdf
   ```

2. **Lancer l'export** depuis les Paramètres

3. **Vérifier le ZIP** :
   ```bash
   unzip -l AATAA_Backup_2024-10-19.zip
   # Devrait afficher :
   # Facture_EDF_2024-06.pdf
   # Facture_EDF_2024-06_1.pdf
   # Facture_EDF_2024-06_2.pdf
   ```

---

## ✅ Statut

- [x] Bug identifié
- [x] Solution implémentée
- [x] Compilation réussie
- [ ] Test en conditions réelles

**Prêt à tester !** 🚀
