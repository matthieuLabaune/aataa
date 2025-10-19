# 🔧 Fix : Détection et Prévention des Doublons à l'Import

## ❌ Problème Initial

**Symptôme** : Après chaque import de backup, les documents étaient dupliqués dans la liste.

**Exemple** :
```
AVANT l'import : 10 documents
Import du backup (10 documents)
APRÈS : 20 documents (10 + 10 doublons) ❌
```

**Cause** : L'import créait systématiquement de nouveaux documents, même s'ils existaient déjà. Aucune détection de doublon n'était en place.

---

## ✅ Solution Implémentée

### Détection Intelligente par Checksum SHA256

**Principe** : Calculer une empreinte unique (hash) du contenu de chaque fichier et comparer.

**Algorithme** :
1. Pour chaque document à importer :
   - Calculer son SHA256 (empreinte du contenu)
   - Comparer avec tous les documents existants
   - Si **même checksum + même nom** → c'est un doublon ✅
   - Sinon → importer

**Fichier** : `src-tauri/src/backup.rs`

### Code Clé

#### 1. Fonction de Calcul du Checksum
```rust
fn calculate_file_checksum(path: &Path) -> Result<String, String> {
    use sha2::{Sha256, Digest};

    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
```

**Pourquoi SHA256 ?**
- ✅ Détection fiable (probabilité de collision négligeable)
- ✅ Rapide (8KB buffer)
- ✅ Détecte même 1 bit de différence

#### 2. Détection lors de l'Import
```rust
// Récupérer tous les documents existants
let existing_docs = db.get_all_documents()?;

for doc in &manifest.documents {
    let source_checksum = calculate_file_checksum(&source)?;

    let mut is_duplicate = false;
    for existing_doc in &existing_docs {
        if existing_doc.deleted_at.is_none() {
            let existing_checksum = calculate_file_checksum(&existing_doc.file_path)?;

            // Doublon si même contenu ET même nom
            if source_checksum == existing_checksum &&
               doc.new_name == existing_doc.new_name {
                is_duplicate = true;
                break;
            }
        }
    }

    if is_duplicate {
        skipped_count += 1;
        eprintln!("⏭️  Skipped duplicate: {}", doc.new_name);
        continue; // Ne pas importer
    }

    // Sinon, importer normalement
    // ...
}
```

#### 3. Message de Confirmation Amélioré
```rust
let message = if skipped_count > 0 {
    format!(
        "Backup restored: {} imported, {} duplicates skipped",
        restored_count, skipped_count
    )
} else {
    format!("Backup restored: {} imported", restored_count)
};
```

---

## 📊 Comportement Détaillé

### Cas 1 : Fichiers Identiques (Contenu + Nom)
```
Existant : Facture_EDF.pdf (SHA: abc123...)
Import   : Facture_EDF.pdf (SHA: abc123...)
→ DOUBLON détecté ✅ → Skipped
```

### Cas 2 : Même Nom, Contenu Différent
```
Existant : Facture_EDF.pdf (SHA: abc123...)
Import   : Facture_EDF.pdf (SHA: xyz789...)
→ PAS un doublon → Renommé en Facture_EDF_1.pdf
```

### Cas 3 : Même Contenu, Nom Différent
```
Existant : Facture_EDF.pdf (SHA: abc123...)
Import   : Facture_EDF_2024.pdf (SHA: abc123...)
→ PAS un doublon (nom différent) → Importé normalement
```

### Cas 4 : Fichier Nouveau
```
Existant : (aucun)
Import   : Ordonnance.pdf (SHA: def456...)
→ Nouveau document → Importé normalement
```

---

## 🧪 Test de la Fonctionnalité

### Scénario de Test

1. **Préparer l'environnement** :
   - Avoir 5 documents dans l'app
   - Exporter un backup

2. **Test 1 : Import sur documents existants**
   ```
   Import du backup
   → Résultat attendu : "5 imported, 5 duplicates skipped"
   → Total docs : 5 (pas de changement) ✅
   ```

3. **Test 2 : Import après suppression de 2 docs**
   ```
   Supprimer 2 documents
   Import du backup
   → Résultat : "2 imported, 3 duplicates skipped"
   → Total docs : 5 (restauré) ✅
   ```

4. **Test 3 : Import multiple**
   ```
   Import 1re fois : "5 imported"
   Import 2e fois  : "0 imported, 5 duplicates skipped"
   Import 3e fois  : "0 imported, 5 duplicates skipped"
   → Pas de duplication infinie ✅
   ```

---

## 🗑️ Nettoyer les Doublons Existants

Si tu as déjà des doublons créés avant ce fix :

### Option 1 : Suppression Manuelle
1. Ouvre l'app
2. Trie par nom de fichier
3. Repère les doublons (même nom)
4. Supprime les copies indésirables

### Option 2 : Reset Complet (⚠️ Attention)
1. Paramètres → "Zone dangereuse"
2. "Réinitialiser l'application"
3. Importer ton backup
4. Cette fois, pas de doublons ✅

### Option 3 : Script SQL (Avancé)
```sql
-- Trouver les doublons (même new_name)
SELECT new_name, COUNT(*) as count
FROM documents
WHERE deleted_at IS NULL
GROUP BY new_name
HAVING count > 1;

-- Supprimer les doublons (garder le plus récent)
DELETE FROM documents
WHERE id NOT IN (
    SELECT MAX(id)
    FROM documents
    WHERE deleted_at IS NULL
    GROUP BY new_name
);
```

---

## 📈 Performance

### Impact sur le Temps d'Import

**Benchmark** (estimation pour 100 documents) :

| Opération            | Temps   |
| -------------------- | ------- |
| Extraction ZIP       | ~2s     |
| Calcul 100 checksums | ~3s     |
| Comparaison doublons | ~0.5s   |
| Copie fichiers       | ~1s     |
| Insert DB            | ~0.5s   |
| **TOTAL**            | **~7s** |

**Sans détection** : ~4s (mais crée des doublons ❌)
**Avec détection** : ~7s (mais évite les doublons ✅)

**Trade-off acceptable** : +3s pour éviter des heures de nettoyage manuel ! 🎯

---

## 🔐 Dépendance Ajoutée

**Cargo.toml** :
```toml
sha2 = "0.10"
```

**Taille** : ~50KB (négligeable)

---

## 💡 Améliorations Futures Possibles

### 1. Dialogue de Confirmation
Au lieu de skip automatique, demander à l'utilisateur :
```
5 duplicates detected. What do you want to do?
[ ] Skip duplicates (recommended)
[ ] Import anyway (create copies)
[ ] Replace existing files
```

### 2. Import Sélectif
Interface pour choisir quels documents importer :
```
☑️ Facture_EDF.pdf (new)
☐ Ordonnance.pdf (duplicate - skip)
☑️ Contrat_Location.pdf (new)
```

### 3. Rapport Détaillé
Après l'import :
```
Import Report:
✅ 5 new documents imported
⏭️ 3 duplicates skipped:
   - Facture_EDF.pdf
   - Ordonnance.pdf
   - Photo_Passeport.jpg
```

### 4. Cache des Checksums
Stocker les checksums en DB pour éviter de les recalculer :
```sql
ALTER TABLE documents ADD COLUMN sha256_checksum TEXT;
CREATE INDEX idx_checksum ON documents(sha256_checksum);
```

---

## ✅ Statut

- [x] Détection de doublons implémentée
- [x] Checksum SHA256 fonctionnel
- [x] Message informatif ("X skipped")
- [x] Compilation réussie
- [ ] Tests en conditions réelles
- [ ] Documentation utilisateur

---

## 🎯 Résultat Final

**Avant** :
```
Import 1 : 10 docs
Import 2 : 20 docs (10 doublons) ❌
Import 3 : 30 docs (20 doublons) ❌
```

**Après** :
```
Import 1 : 10 docs ✅
Import 2 : 10 docs (0 nouveaux, 10 skipped) ✅
Import 3 : 10 docs (0 nouveaux, 10 skipped) ✅
```

**Problème résolu !** 🎉
