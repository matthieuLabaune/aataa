# Guide de test AATAA

## Test rapide avec des images de test

1. **Créer une image de test simple** :
   - Ouvrir un éditeur de texte
   - Écrire "FACTURE" et quelques montants
   - Faire une capture d'écran
   - Sauvegarder en PNG

2. **Tester l'application** :
   - Lancer `npm run tauri:dev`
   - Cliquer sur la zone d'import
   - Sélectionner l'image de test
   - Observer le traitement OCR + classification

## Types de documents à tester

### Facture
Créer une image contenant :
```
FACTURE N° 2024-001
Date: 15/10/2024
Montant HT: 100.00 €
TVA: 20.00 €
Total TTC: 120.00 €
```

### Relevé bancaire
Créer une image contenant :
```
RELEVÉ DE COMPTE
IBAN: FR76 1234 5678 9012 3456 7890 123
Solde: 1,234.56 €
```

### Bulletin de paie
Créer une image contenant :
```
BULLETIN DE PAIE
Salaire brut: 2,500.00 €
Cotisations: 500.00 €
Net à payer: 2,000.00 €
```

## Résultat attendu

L'application devrait :
1. Extraire le texte via OCR
2. Classifier automatiquement le document
3. Renommer le fichier (ex: FACT_20241009_143022.png)
4. Ajouter des tags (contains_date, contains_amount, etc.)
5. Stocker dans la base de données
6. Afficher dans la liste
