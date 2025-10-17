# TODO - AATAA Améliorations Futures

## 🔧 Corrections immédiates (Quick Wins)

- [ ] **Supprimer les warnings Rust**
  - Fichier : `src-tauri/src/commands.rs`
  - Action : Supprimer les imports inutilisés `DialogExt` et `MessageDialogKind`
  - Supprimer ou implémenter `select_file_dialog()`

- [ ] **Implémenter dialog natif**
  - Remplacer l'input HTML par le dialog Tauri natif
  - Utiliser `tauri-plugin-dialog` correctement
  - Supporter le drag & drop

- [ ] **Ajouter gestion d'erreurs UI**
  - Toasts pour les erreurs
  - Messages d'erreur clairs
  - Retry automatique

## 🎨 Interface Utilisateur

### Court terme (1-2 semaines)
- [ ] **Preview de documents**
  - Modal avec aperçu image
  - Zoom in/out
  - Navigation entre documents

- [ ] **Édition des métadonnées**
  - Modifier le nom du document
  - Ajouter/supprimer des tags manuellement
  - Changer le type de document

- [ ] **Filtres avancés**
  - Filtre par type
  - Filtre par date
  - Filtre par tags
  - Filtre par taille

- [ ] **Tri personnalisé**
  - Par nom
  - Par date
  - Par taille
  - Par type

- [ ] **Actions en masse**
  - Sélection multiple
  - Suppression en masse
  - Export en masse
  - Re-classification en masse

### Moyen terme (1 mois)
- [ ] **Dashboard/Statistiques**
  - Nombre de documents par type
  - Documents ce mois-ci
  - Taille totale d'archive
  - Graphiques

- [ ] **Thèmes personnalisés**
  - Couleurs personnalisables
  - Mode sombre amélioré
  - Disposition personnalisable

- [ ] **Raccourcis clavier**
  - Ctrl+N : Nouveau document
  - Ctrl+F : Rechercher
  - Ctrl+D : Supprimer
  - Échap : Fermer modal

## 🔍 OCR et Classification

### Court terme
- [ ] **Support multi-langues**
  - Détection automatique de langue
  - Français + Anglais combinés
  - Autres langues européennes

- [ ] **OCR amélioré pour PDF**
  - Conversion PDF → Images
  - OCR sur chaque page
  - Fusion des textes

- [ ] **Améliorer les patterns**
  - Tester avec vrais documents
  - Ajuster les regex
  - Ajouter nouveaux types

### Moyen terme
- [ ] **Patterns personnalisables**
  - Interface pour créer des patterns
  - Import/Export de règles
  - Partage communautaire

- [ ] **Apprentissage des patterns**
  - ML pour classification
  - Amélioration continue
  - Suggestions de patterns

### Long terme
- [ ] **Intégration HuggingFace**
  - olmOCR-7B-0825 (AllenAI)
  - TrOCR (Microsoft)
  - Donut (Naver)
  - Mode offline avec modèles locaux

- [ ] **Extraction intelligente**
  - Dates structurées
  - Montants et devises
  - Entités nommées (personnes, lieux)
  - Tables de données

## 💾 Stockage et Performance

### Court terme
- [ ] **Cache OCR**
  - Ne pas re-traiter les mêmes fichiers
  - Hash des fichiers
  - Invalidation cache

- [ ] **Optimisation DB**
  - Index SQLite
  - FTS5 pour recherche
  - Vacuum automatique

- [ ] **Compression**
  - Compresser les grandes images
  - Format WebP
  - Qualité configurable

### Moyen terme
- [ ] **Traitement en arrière-plan**
  - File d'attente
  - Traitement parallèle
  - Progress bar

- [ ] **Backup automatique**
  - Export automatique
  - Versioning
  - Restauration

- [ ] **Limite de taille**
  - Configuration max file size
  - Avertissement espace disque
  - Nettoyage automatique ancien docs

## 📤 Export et Intégration

### Court terme
- [ ] **Export CSV**
  - Métadonnées
  - Texte OCR
  - Configurable

- [ ] **Export JSON**
  - Format structuré
  - Import/Export complet

### Moyen terme
- [ ] **API REST locale**
  - Endpoints pour intégrations
  - Webhook notifications
  - Documentation OpenAPI

- [ ] **Intégrations**
  - Dropbox
  - Google Drive
  - OneDrive
  - Nextcloud

- [ ] **Templates d'export**
  - Excel
  - PDF rapport
  - HTML

## 🔒 Sécurité et Confidentialité

### Court terme
- [ ] **Chiffrement optionnel**
  - Chiffrement base de données
  - Chiffrement fichiers
  - Mot de passe maître

### Moyen terme
- [ ] **Audit logs**
  - Traçabilité des actions
  - Export des logs
  - Analyse de sécurité

- [ ] **Permissions granulaires**
  - Lecture seule
  - Import seulement
  - Admin

## 📱 Multi-plateforme

### Moyen terme
- [ ] **Mobile (Tauri Mobile)**
  - iOS
  - Android
  - Synchronisation

- [ ] **Web version**
  - PWA
  - Mode offline
  - Sync avec desktop

## 🧪 Tests et Qualité

### Court terme
- [ ] **Tests unitaires**
  - Backend Rust
  - Frontend Vue
  - Coverage > 80%

- [ ] **Tests d'intégration**
  - E2E avec Playwright
  - Tests de performance
  - Tests de régression

### Moyen terme
- [ ] **CI/CD**
  - GitHub Actions
  - Build automatique
  - Release automatique

- [ ] **Monitoring**
  - Crash reporting (optionnel)
  - Performance monitoring
  - Usage analytics (anonyme, opt-in)

## 📚 Documentation

### Court terme
- [ ] **Vidéo démo**
  - Screencast 2-3 min
  - Upload sur YouTube
  - Embed dans README

- [ ] **Guide utilisateur illustré**
  - Screenshots
  - Annotations
  - Cas d'usage

### Moyen terme
- [ ] **Documentation API**
  - Tauri commands
  - Types TypeScript
  - Exemples de code

- [ ] **Contribution guide**
  - Code style
  - PR template
  - Issue template

## 🌐 Communauté

### Moyen terme
- [ ] **Website**
  - Landing page
  - Documentation en ligne
  - Blog

- [ ] **Forum/Discord**
  - Support communautaire
  - Partage de patterns
  - Roadmap publique

- [ ] **Marketplace de patterns**
  - Partage de règles
  - Import facile
  - Rating système

## 🚀 Fonctionnalités avancées

### Long terme
- [ ] **OCR temps réel**
  - Scan de webcam
  - Documents physiques
  - Traitement instantané

- [ ] **Reconnaissance de formulaires**
  - Templates de formulaires
  - Extraction champs
  - Auto-remplissage

- [ ] **Génération de documents**
  - Templates
  - Mail merge
  - PDF generation

- [ ] **Assistant IA**
  - Résumé automatique
  - Q&A sur documents
  - Suggestions d'organisation

- [ ] **Collaboration**
  - Partage de documents
  - Commentaires
  - Workflow approval

## 📊 Priorités suggérées

### P0 - Critique (Cette semaine)
1. Supprimer warnings Rust
2. Dialog natif
3. Gestion d'erreurs UI

### P1 - Haute (Ce mois)
1. Preview de documents
2. Filtres avancés
3. Export CSV/JSON
4. OCR multi-langues

### P2 - Moyenne (2-3 mois)
1. Dashboard statistiques
2. Patterns personnalisables
3. Backup automatique
4. Tests unitaires

### P3 - Basse (3-6 mois)
1. Intégration HuggingFace
2. Mobile app
3. API REST
4. Marketplace patterns

---

## 🎯 Objectifs par version

### v0.2.0 (semaine 1-2)
- Corrections warnings
- Dialog natif
- Preview documents
- Export CSV/JSON

### v0.3.0 (mois 1)
- Filtres avancés
- Dashboard stats
- OCR multi-langues
- Tests unitaires

### v0.4.0 (mois 2-3)
- Patterns personnalisables
- Backup automatique
- Intégrations cloud
- CI/CD

### v1.0.0 (mois 4-6)
- ML classification
- Mobile app (beta)
- API REST
- Documentation complète

---

**Note** : Cette TODO est vivante et sera mise à jour au fil du développement.

*Dernière mise à jour : 9 octobre 2025*
