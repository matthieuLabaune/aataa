# Stratégie de catégorisation - Analyse et recommandations

## 🤔 Question stratégique

**Dilemme** : Beaucoup de catégories (plus de cas d'usage) vs Peu de catégories (plus simple, mais moins d'acheteurs)

---

## 📊 Approche recommandée : **Système hybride à 3 niveaux**

### Niveau 1 : Catégories génériques (8-10 types de base)
### Niveau 2 : Sous-catégories configurables par l'utilisateur
### Niveau 3 : Tags personnalisés

**Avantages** :
- ✅ Application simple au départ (pas intimidante)
- ✅ Extensible selon les besoins de chaque utilisateur
- ✅ OCR intelligent peut pré-remplir niveau 1
- ✅ Utilisateur affine manuellement niveau 2 et 3

---

## 🎯 Proposition de catégories (Niveau 1)

### Option A : Catégories génériques (recommandé pour v1.0)

```rust
pub enum DocumentType {
    // 1. ADMINISTRATIF
    AdministratifOfficial,    // Passeport, carte identité, acte naissance
    AdministratifImpots,       // Déclaration, avis imposition

    // 2. FINANCIER
    FinancierFacture,          // Factures diverses
    FinancierBanque,           // Relevés, virements
    FinancierContrat,          // Contrats divers

    // 3. SANTÉ
    SanteMedical,              // Ordonnances, résultats analyses
    SanteRemboursement,        // Feuilles de soins, remboursements

    // 4. PROFESSIONNEL
    ProTravail,                // Contrat travail, fiche paie
    ProFacturation,            // Factures émises (si indépendant)

    // 5. IMMOBILIER
    ImmobilierBien,            // Actes propriété, diagnostics
    ImmobilierLocation,        // Baux, quittances loyer

    // 6. ACADÉMIQUE / RECHERCHE
    AcademiquePublication,     // Articles, papers
    AcademiqueCertificat,      // Diplômes, certifications

    // 7. PERSONNEL
    PersonnelTicket,           // Tickets, reçus achats
    PersonnelCorrespondance,   // Lettres, courriers

    // 8. AUTRE
    Document,                  // Type générique par défaut
}
```

**Total : 16 catégories** (assez pour couvrir 90% des cas)

---

### Option B : Catégories ultra-détaillées (risque de complexité)

```rust
// ADMINISTRATIF (7 types)
- Carte identité / Passeport
- Acte de naissance
- Livret de famille
- Carte vitale
- Permis de conduire
- Avis d'imposition
- Déclaration fiscale

// SANTÉ (6 types)
- Ordonnance médicale
- Résultat analyse
- Facture médicale
- Remboursement sécurité sociale
- Remboursement mutuelle
- Carnet de vaccination

// PROFESSIONNEL (8 types)
- Contrat de travail
- Fiche de paie
- Certificat de travail
- Attestation employeur
- Note de frais
- Facture client (émise)
- Devis client
- Bon de commande

// FINANCIER (7 types)
- Relevé bancaire
- Facture fournisseur (reçue)
- Facture énergie (EDF, gaz)
- Facture télécom (Internet, mobile)
- Facture eau
- Contrat bancaire
- Assurance (auto, habitation, santé)

// IMMOBILIER (5 types)
- Acte de propriété
- Bail location
- Quittance loyer
- Diagnostic immobilier (DPE, amiante)
- Facture travaux

// ACADÉMIQUE (6 types)
- Article de revue (peer-reviewed)
- Preprint / Working paper
- Thèse / Mémoire
- Diplôme / Certificat
- Relevé de notes
- Convention de stage

// AUTOMOBILE (4 types)
- Carte grise
- Assurance auto
- Contrôle technique
- Facture garage

// ACHATS PERSONNELS (4 types)
- Ticket de caisse
- Facture produit (garantie)
- Bon de livraison
- Bon de retour

// VOYAGE (3 types)
- Billet d'avion / train
- Réservation hôtel
- Visa

// AUTRE
- Document non classé
```

**Total : 50+ catégories** (très complet mais complexe)

---

## 🎨 Architecture technique recommandée

### 1. Base de données : Hiérarchie à 3 niveaux

```sql
-- Niveau 1 : Catégorie principale (prédéfinie)
CREATE TABLE document_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,              -- "Financier", "Santé", "Pro"...
    icon TEXT,                        -- Material icon
    color TEXT                        -- Couleur thème
);

-- Niveau 2 : Sous-catégorie (configurable par utilisateur)
CREATE TABLE document_subcategories (
    id INTEGER PRIMARY KEY,
    category_id INTEGER REFERENCES document_categories(id),
    name TEXT NOT NULL,              -- "Facture EDF", "Ordonnance", "Article Nature"
    is_predefined BOOLEAN DEFAULT 0, -- Fournie par défaut ou créée par user
    user_created BOOLEAN DEFAULT 0
);

-- Niveau 3 : Tags libres
CREATE TABLE document_tags (
    id INTEGER PRIMARY KEY,
    document_id INTEGER REFERENCES documents(id),
    tag_name TEXT NOT NULL           -- "urgent", "2024", "CHU Nantes"
);

-- Table principale documents
CREATE TABLE documents (
    id INTEGER PRIMARY KEY,
    category_id INTEGER REFERENCES document_categories(id),
    subcategory_id INTEGER REFERENCES document_subcategories(id),
    -- ... autres champs existants
);
```

### 2. Interface utilisateur : Configuration dynamique

```vue
<!-- Settings.vue : Gestion des catégories -->
<template>
  <section class="categories-settings">
    <h2>Catégories de documents</h2>

    <!-- Catégories principales (non modifiables) -->
    <div class="category-list">
      <div v-for="cat in mainCategories" :key="cat.id" class="category-item">
        <span class="category-icon">{{ cat.icon }}</span>
        <span>{{ cat.name }}</span>

        <!-- Sous-catégories (modifiables) -->
        <div class="subcategories">
          <chip v-for="sub in cat.subcategories" :key="sub.id">
            {{ sub.name }}
            <button @click="deleteSubcategory(sub.id)" v-if="sub.user_created">×</button>
          </chip>

          <!-- Ajouter sous-catégorie -->
          <button @click="addSubcategory(cat.id)">+ Ajouter</button>
        </div>
      </div>
    </div>
  </section>
</template>
```

### 3. OCR Classifier : Détection intelligente

```rust
// classifier.rs : Détection niveau 1 + suggestions niveau 2

pub struct ClassificationResult {
    pub category: MainCategory,           // Niveau 1 (OCR automatique)
    pub subcategory: Option<String>,      // Niveau 2 (suggestion)
    pub confidence: f32,
    pub suggested_tags: Vec<String>,      // Niveau 3 (extraction auto)
}

impl Classifier {
    pub fn classify_detailed(&self, text: &str) -> ClassificationResult {
        // 1. Catégorie principale (comme actuellement)
        let category = self.detect_main_category(text);

        // 2. Sous-catégorie suggérée
        let subcategory = match category {
            MainCategory::Financier => {
                if text.contains("EDF") || text.contains("électricité") {
                    Some("Facture énergie - EDF".to_string())
                } else if text.contains("SFR") || text.contains("Orange") {
                    Some("Facture télécom".to_string())
                } else {
                    None
                }
            },
            MainCategory::Sante => {
                if text.contains("CPAM") || text.contains("remboursement") {
                    Some("Remboursement sécu".to_string())
                } else if text.contains("ordonnance") {
                    Some("Ordonnance".to_string())
                } else {
                    None
                }
            },
            // ... autres catégories
            _ => None
        };

        // 3. Tags suggérés (extraction de dates, montants, entités)
        let tags = self.extract_tags(text);

        ClassificationResult {
            category,
            subcategory,
            confidence: self.calculate_confidence(text),
            suggested_tags: tags,
        }
    }

    fn extract_tags(&self, text: &str) -> Vec<String> {
        let mut tags = Vec::new();

        // Extraire année
        if let Some(year) = self.extract_year(text) {
            tags.push(year);
        }

        // Extraire montant
        if let Some(amount) = self.extract_amount(text) {
            tags.push(format!("{}€", amount));
        }

        // Extraire nom de société/revue
        if let Some(entity) = self.extract_entity(text) {
            tags.push(entity);
        }

        tags
    }
}
```

---

## 🎓 Cas d'usage : Chercheur académique

### Besoins spécifiques

```
Catégorie principale : "Académique / Recherche"

Sous-catégories prédéfinies :
- Article publié (peer-reviewed)
- Article soumis (en révision)
- Preprint
- Chapitre de livre
- Communication conférence
- Poster
- Rapport de recherche
- Thèse / Mémoire

Métadonnées personnalisées :
- Nom de la revue (Nature, Science, Cell...)
- Impact factor
- Année de publication
- Co-auteurs
- DOI
- Statut (publié / accepté / soumis)

Organisation suggérée :
1. Par revue → Par année
   - Nature/
     - 2024/
       - article1.pdf
       - article2.pdf
     - 2023/
   - Science/
     - 2024/

2. Par année → Par revue
   - 2024/
     - Nature/
     - Science/
   - 2023/
     - Cell/
```

### Implémentation pour chercheur

```rust
// models.rs : Métadonnées académiques

#[derive(Serialize, Deserialize)]
pub struct AcademicMetadata {
    pub journal_name: Option<String>,      // "Nature", "Science"
    pub impact_factor: Option<f32>,        // 43.07
    pub publication_year: Option<i32>,     // 2024
    pub doi: Option<String>,               // "10.1038/s41586-024-07123-x"
    pub coauthors: Vec<String>,            // ["Smith J", "Doe A"]
    pub status: PublicationStatus,         // Published, Submitted, InReview
}

pub enum PublicationStatus {
    Published,
    Accepted,
    InReview,
    Submitted,
    Rejected,
    Draft,
}

// Dans documents table
pub struct Document {
    // ... champs existants
    pub academic_metadata: Option<String>,  // JSON sérialisé
}
```

```vue
<!-- DocumentCard.vue : Affichage pour articles académiques -->
<template>
  <div v-if="doc.category === 'Académique'" class="academic-card">
    <div class="journal-badge" :style="{ background: journalColor }">
      {{ doc.metadata.journal_name }}
    </div>
    <h3>{{ doc.title }}</h3>
    <div class="academic-info">
      <span class="year">{{ doc.metadata.publication_year }}</span>
      <span class="impact-factor">IF: {{ doc.metadata.impact_factor }}</span>
      <span class="doi">DOI: {{ doc.metadata.doi }}</span>
    </div>
    <div class="coauthors">
      <chip v-for="author in doc.metadata.coauthors">{{ author }}</chip>
    </div>
  </div>
</template>
```

---

## 💡 Recommandations finales

### ✅ Pour la version 1.0 (maintenant)

**Stratégie : Simplicité + Extensibilité**

1. **8 catégories principales prédéfinies** (non modifiables)
   - Administratif
   - Financier
   - Santé
   - Professionnel
   - Immobilier
   - Académique
   - Personnel
   - Autre

2. **15-20 sous-catégories prédéfinies** (les plus courantes)
   - Utilisateur peut en ajouter à volonté

3. **Tags illimités** (libres)
   - Suggérés automatiquement par OCR
   - Personnalisables

4. **Métadonnées de base**
   - Titre, date, tags, notes
   - Extensible plus tard avec métadonnées spécialisées

### 🎯 Marketing / Positionnement

**Cible large** : "Organisez TOUS vos documents"

**Messages clés** :
- "Du particulier au chercheur"
- "S'adapte à VOS besoins"
- "Catégories personnalisables"

**Cas d'usage mis en avant** :
- 👤 Particulier : factures, impôts, santé
- 💼 Professionnel : contrats, paies, notes de frais
- 🎓 Étudiant : cours, stages, diplômes
- 🔬 Chercheur : publications, brevets, rapports
- 🏠 Propriétaire : actes, travaux, assurances

### 🚀 Roadmap évolutive

**Version 1.0** (maintenant)
- 8 catégories + sous-catégories configurables + tags

**Version 1.5** (dans 3 mois)
- Templates de métadonnées par catégorie
- Import automatique depuis DOI (pour chercheurs)
- Détection automatique de fournisseur (EDF, SFR...)

**Version 2.0** (dans 6 mois)
- Métadonnées spécialisées (académique, médical, immobilier)
- Intégrations externes (HAL, CrossRef pour chercheurs)
- Rappels automatiques (échéances, renouvellements)

---

## 📋 TODO : Implémentation

### Phase 1 : Refonte du modèle de catégories (2-3h)
- [ ] Créer enum MainCategory avec 8 types
- [ ] Créer table document_subcategories
- [ ] Créer table document_tags
- [ ] Migration base de données

### Phase 2 : UI de configuration (3-4h)
- [ ] Page Settings : gestion des sous-catégories
- [ ] Composant SubcategoryManager.vue
- [ ] Composant TagInput.vue
- [ ] Sélecteur hiérarchique (catégorie → sous-catégorie)

### Phase 3 : Classifier amélioré (2-3h)
- [ ] Implémenter detect_main_category() pour 8 catégories
- [ ] Ajouter suggestion de sous-catégorie
- [ ] Ajouter extraction de tags automatique
- [ ] Retourner ClassificationResult complet

### Phase 4 : Affichage enrichi (2-3h)
- [ ] DocumentCard avec sous-catégorie + tags
- [ ] Filtres par catégorie/sous-catégorie/tags
- [ ] Vue groupée par catégorie
- [ ] Couleurs par catégorie

**Temps total estimé : 10-15 heures de développement**

---

## 🤝 Ma recommandation

**Ne vous limitez PAS** - Adoptez le système hybride :

1. **Simple au premier démarrage** (8 catégories)
2. **Extensible selon l'utilisateur** (sous-catégories illimitées)
3. **Cas d'usage spécialisés supportés** (chercheur, médecin, entrepreneur...)

**Pourquoi ?**
- Vous ne perdez pas de clients (chacun peut adapter)
- Vous ne complexifiez pas l'onboarding (reste simple)
- Vous vous différenciez de la concurrence (ultra flexible)

**Message marketing** :
> "AATAA s'adapte à votre vie : étudiant, chercheur, entrepreneur ou particulier, organisez vos documents comme VOUS le voulez."

---

Qu'en pensez-vous ? Je peux commencer l'implémentation si vous validez cette approche ! 🚀
