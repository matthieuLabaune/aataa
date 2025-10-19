# 🧹 Guide : Nettoyer les Doublons Existants

## 🎯 Objectif

Supprimer les documents dupliqués qui ont été créés avant l'implémentation de la détection de doublons.

---

## ⚠️ Avant de Commencer

**IMPORTANT** : Fais un backup AVANT de nettoyer !

```
Paramètres → Sauvegarde & Export → Exporter
→ Sauvegarde sur le Bureau : AATAA_Backup_Avant_Nettoyage.zip
```

---

## Option 1 : Nettoyage Manuel (Recommandé) 🖱️

### Avantages
- ✅ Contrôle total sur ce qui est supprimé
- ✅ Aucun risque de perte de données
- ✅ Pas besoin de compétences techniques

### Étapes

1. **Identifier les doublons** :
   - Ouvre l'app AATAA
   - Regarde la liste des documents
   - Repère les noms identiques (exemple: 3x "Facture_EDF.pdf")

2. **Trier pour faciliter** :
   - Clique sur "Nom" pour trier alphabétiquement
   - Les doublons seront côte à côte

3. **Supprimer les copies** :
   - Pour chaque groupe de doublons :
     - Garde le **plus récent** (date de création)
     - Ou celui avec les **meilleures métadonnées** (catégorie, notes)
   - Supprime les autres (glisse vers la corbeille)

4. **Vider la corbeille** :
   - Onglet "Corbeille"
   - Sélectionne les doublons
   - "Supprimer définitivement"

**Temps estimé** : 5-10 minutes pour 50 documents

---

## Option 2 : Reset Complet (Rapide) 🔄

### ⚠️ Attention
Cette méthode **supprime TOUT** et réimporte depuis le backup.

### Quand l'utiliser
- Tu as beaucoup de doublons (>100)
- Le nettoyage manuel prendrait trop de temps
- Tu as un backup récent et fiable

### Étapes

1. **Exporter un backup final** :
   ```
   Paramètres → Export → AATAA_Backup_Final.zip
   ```

2. **Réinitialiser l'app** :
   ```
   Paramètres → Zone dangereuse → Réinitialiser l'application
   → Confirmer
   ```

3. **Réimporter le backup** :
   ```
   Paramètres → Importer → Sélectionner AATAA_Backup_Final.zip
   → Message : "X documents imported, 0 duplicates skipped" ✅
   ```

4. **Vérifier** :
   - Compte le nombre de documents
   - Devrait correspondre au nombre original (avant doublons)

**Temps estimé** : 2-3 minutes

---

## Option 3 : Script Automatique (Avancé) 💻

### Prérequis
- Connaissances en ligne de commande
- SQLite installé

### Script Shell

Créer un fichier `clean_duplicates.sh` :

```bash
#!/bin/bash

# Chemin vers la base de données AATAA
DB_PATH="$HOME/Library/Application Support/com.aataa.app/documents.db"

# Backup de la DB
cp "$DB_PATH" "$DB_PATH.backup"

# SQL pour trouver et supprimer les doublons
sqlite3 "$DB_PATH" <<EOF
-- Afficher les doublons
SELECT new_name, COUNT(*) as count, GROUP_CONCAT(id) as ids
FROM documents
WHERE deleted_at IS NULL
GROUP BY new_name
HAVING count > 1;

-- Supprimer les doublons (garder le plus récent = MAX(created_at))
DELETE FROM documents
WHERE id IN (
    SELECT d1.id
    FROM documents d1
    WHERE deleted_at IS NULL
    AND EXISTS (
        SELECT 1
        FROM documents d2
        WHERE d2.new_name = d1.new_name
        AND d2.deleted_at IS NULL
        AND d2.created_at > d1.created_at
    )
);

-- Afficher le résultat
SELECT 'Cleaned! Remaining documents:' as message, COUNT(*) as count
FROM documents
WHERE deleted_at IS NULL;
EOF

echo "✅ Duplicates cleaned! Backup saved at $DB_PATH.backup"
```

### Exécution

```bash
chmod +x clean_duplicates.sh
./clean_duplicates.sh
```

**⚠️ ATTENTION** : Cette méthode modifie directement la base de données. Fais un backup d'abord !

---

## Option 4 : Détection Intelligente en SQL (Technique) 🔍

### Voir les Doublons

```bash
# Ouvrir la DB
sqlite3 "$HOME/Library/Application Support/com.aataa.app/documents.db"
```

```sql
-- Lister les doublons avec détails
SELECT
    new_name,
    COUNT(*) as count,
    GROUP_CONCAT(created_at, ', ') as dates
FROM documents
WHERE deleted_at IS NULL
GROUP BY new_name
HAVING count > 1
ORDER BY count DESC;
```

### Supprimer Sélectivement

```sql
-- Exemple : Supprimer les doublons de "Facture_EDF.pdf"
-- Garde celui avec le created_at le plus récent

DELETE FROM documents
WHERE new_name = 'Facture_EDF.pdf'
AND id != (
    SELECT id
    FROM documents
    WHERE new_name = 'Facture_EDF.pdf'
    AND deleted_at IS NULL
    ORDER BY created_at DESC
    LIMIT 1
);
```

---

## 🧪 Vérification Post-Nettoyage

### Checklist

1. **Compte les documents** :
   ```sql
   SELECT COUNT(*) FROM documents WHERE deleted_at IS NULL;
   ```
   Devrait être environ **la moitié** du nombre avant nettoyage (si doublons 2x)

2. **Vérifie les doublons restants** :
   ```sql
   SELECT new_name, COUNT(*) as count
   FROM documents
   WHERE deleted_at IS NULL
   GROUP BY new_name
   HAVING count > 1;
   ```
   Devrait retourner **0 résultats** ✅

3. **Ouvre quelques documents** :
   - Teste 5-10 documents au hasard
   - Vérifie qu'ils s'ouvrent correctement
   - Vérifie le contenu

---

## 📊 Résultats Attendus

**Avant nettoyage** :
```
Total documents : 47
Uniques : 24
Doublons : 23 (47 - 24)
```

**Après nettoyage** :
```
Total documents : 24 ✅
Uniques : 24 ✅
Doublons : 0 ✅
```

---

## 🚨 En Cas de Problème

### "J'ai supprimé le mauvais document !"

**Solution** :
1. Corbeille → Restaurer le document
2. Supprimer le bon

### "Tous mes documents ont disparu !"

**Solution** :
1. Restaurer depuis le backup :
   ```
   Paramètres → Importer → AATAA_Backup_Avant_Nettoyage.zip
   ```

### "La base de données est corrompue"

**Solution** :
1. Quitter l'app
2. Restaurer le backup de la DB :
   ```bash
   cp "$HOME/Library/Application Support/com.aataa.app/documents.db.backup" \
      "$HOME/Library/Application Support/com.aataa.app/documents.db"
   ```
3. Relancer l'app

---

## 💡 Prévention Future

Avec le fix appliqué, les doublons ne seront **plus créés** :

```
Import backup avec doublons :
→ "10 documents imported, 5 duplicates skipped" ✅
→ Pas de nouvelle duplication
```

---

## 🎯 Recommandation

**Pour la majorité des utilisateurs** : Option 1 (Nettoyage Manuel)
- Sûr
- Simple
- Contrôle total

**Si beaucoup de doublons** : Option 2 (Reset + Réimport)
- Rapide
- Propre
- Nécessite un bon backup

**Si très technique** : Option 3 ou 4
- Précis
- Flexible
- Risqué si mal utilisé

---

**Choisis l'option qui te convient et nettoie tes doublons !** 🧹✨
