# 🎯 Must-Have #4 : Tests de Performance

## 📋 Objectif

S'assurer que l'app reste fluide avec **1000+ documents**.

**Critères de performance** :
- Démarrage app : < 2s
- Chargement liste 1000 docs : < 1s
- Recherche full-text : < 500ms
- Classification document : < 3s
- Mémoire idle : < 150MB
- Mémoire 1000 docs : < 300MB

---

## 🏗️ Script de Génération de Données de Test

### 1. Générateur Python

**Fichier** : `python/generate_test_data.py`

```python
#!/usr/bin/env python3
"""
Génère 1000+ documents de test avec variété réaliste
"""

import os
import random
from pathlib import Path
from datetime import datetime, timedelta
from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import letter
from PIL import Image, ImageDraw, ImageFont

# Catégories et templates
CATEGORIES = {
    "Financier": [
        "Facture {vendor} - {amount}€",
        "Relevé bancaire {month} {year}",
        "Note de frais {month} {year}"
    ],
    "Santé": [
        "Ordonnance Dr. {doctor}",
        "Résultats analyses {date}",
        "Décompte CPAM {month}"
    ],
    "Administratif": [
        "Attestation {type}",
        "Certificat {purpose}",
        "Carte {document_type}"
    ],
    "Professionnel": [
        "Bulletin salaire {month} {year}",
        "Contrat {type}",
        "Attestation employeur"
    ]
}

VENDORS = ["EDF", "SFR", "Orange", "Carrefour", "SNCF", "Amazon", "Apple"]
DOCTORS = ["Martin", "Dupont", "Bernard", "Petit", "Dubois"]
MONTHS = ["Janvier", "Février", "Mars", "Avril", "Mai", "Juin", 
          "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre"]

def generate_pdf(filepath: Path, content: str, category: str):
    """Génère un PDF simple avec du texte"""
    c = canvas.Canvas(str(filepath), pagesize=letter)
    c.setFont("Helvetica", 12)
    
    # Header
    c.drawString(100, 750, f"DOCUMENT - {category}")
    c.drawString(100, 730, f"Date: {datetime.now().strftime('%d/%m/%Y')}")
    c.drawString(100, 710, "-" * 60)
    
    # Content
    y = 680
    for line in content.split('\n'):
        c.drawString(100, y, line)
        y -= 20
    
    # Footer
    c.drawString(100, 100, f"Numéro: {random.randint(1000, 9999)}")
    
    c.save()

def generate_image(filepath: Path, text: str, category: str):
    """Génère une image avec du texte (simule scan)"""
    img = Image.new('RGB', (800, 600), color='white')
    d = ImageDraw.Draw(img)
    
    try:
        font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 20)
    except:
        font = ImageFont.load_default()
    
    # Title
    d.text((50, 50), f"{category}", fill='black', font=font)
    d.text((50, 80), "-" * 50, fill='black')
    
    # Content
    y = 120
    for line in text.split('\n'):
        d.text((50, y), line, fill='black', font=font)
        y += 30
    
    img.save(filepath)

def generate_test_documents(output_dir: Path, count: int = 1000):
    """Génère N documents de test"""
    output_dir.mkdir(parents=True, exist_ok=True)
    
    print(f"🔄 Génération de {count} documents de test...")
    
    for i in range(count):
        # Choisir catégorie et template
        category = random.choice(list(CATEGORIES.keys()))
        template = random.choice(CATEGORIES[category])
        
        # Générer contenu
        content = template.format(
            vendor=random.choice(VENDORS),
            amount=random.randint(10, 5000),
            doctor=random.choice(DOCTORS),
            month=random.choice(MONTHS),
            year=random.randint(2020, 2024),
            type=random.choice(["Résidence", "Travail", "Scolarité"]),
            purpose=random.choice(["Médical", "Sportif", "Vaccinal"]),
            document_type=random.choice(["Identité", "Vitale", "Transport"]),
            date=datetime.now().strftime('%d/%m/%Y')
        )
        
        # Contenu détaillé
        full_content = f"""
{content}

Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Montant: {random.randint(10, 1000)}€
Référence: REF-{random.randint(1000, 9999)}
Date: {(datetime.now() - timedelta(days=random.randint(0, 365))).strftime('%d/%m/%Y')}

Description du document...
Détails supplémentaires...
        """.strip()
        
        # Alterner PDF et images
        if random.random() > 0.5:
            filepath = output_dir / f"doc_{i:04d}.pdf"
            generate_pdf(filepath, full_content, category)
        else:
            filepath = output_dir / f"doc_{i:04d}.jpg"
            generate_image(filepath, full_content, category)
        
        if (i + 1) % 100 == 0:
            print(f"  ✓ {i + 1}/{count} documents générés")
    
    print(f"✅ {count} documents générés dans {output_dir}")

if __name__ == "__main__":
    import sys
    count = int(sys.argv[1]) if len(sys.argv) > 1 else 1000
    output = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("test_documents")
    
    # Installer dépendances si nécessaire
    try:
        import reportlab
        from PIL import Image
    except ImportError:
        print("📦 Installation des dépendances...")
        os.system("pip install reportlab pillow")
    
    generate_test_documents(output, count)
```

---

## 📊 Scripts de Benchmark

### 2. Benchmark Performances

**Fichier** : `python/benchmark.py`

```python
#!/usr/bin/env python3
"""
Benchmarks de performance AATAA
"""

import time
import json
import subprocess
from pathlib import Path

def measure_time(label: str):
    """Decorator pour mesurer le temps d'exécution"""
    def decorator(func):
        def wrapper(*args, **kwargs):
            start = time.time()
            result = func(*args, **kwargs)
            duration = time.time() - start
            print(f"⏱️  {label}: {duration:.3f}s")
            return result, duration
        return wrapper
    return decorator

@measure_time("Démarrage application")
def benchmark_startup():
    """Mesure temps de démarrage"""
    # Simuler lancement app et attendre qu'elle soit prête
    # En pratique, mesurer via logs ou API
    time.sleep(1.5)  # Simulation
    return True

@measure_time("Chargement 1000 documents")
def benchmark_load_documents(count=1000):
    """Mesure temps de chargement liste"""
    # Appeler API Tauri pour récupérer documents
    # Simulé ici
    docs = [{"id": i, "name": f"Doc {i}"} for i in range(count)]
    return docs

@measure_time("Recherche full-text")
def benchmark_search(query="facture"):
    """Mesure temps de recherche"""
    # Appeler API de recherche
    # Simulé ici
    time.sleep(0.3)
    return [{"id": 1}, {"id": 5}, {"id": 42}]

@measure_time("Classification document")
def benchmark_classification():
    """Mesure temps de classification"""
    text = "Facture prestation développement web 2400 euros"
    # Appeler classifier sémantique
    result = subprocess.run(
        ["python", "semantic_classifier.py", text],
        capture_output=True,
        text=True
    )
    return json.loads(result.stdout)

def measure_memory():
    """Mesure mémoire utilisée par l'app"""
    try:
        import psutil
        # Trouver le process de l'app
        for proc in psutil.process_iter(['pid', 'name', 'memory_info']):
            if 'aataa' in proc.info['name'].lower():
                memory_mb = proc.info['memory_info'].rss / 1024 / 1024
                print(f"💾 Mémoire utilisée: {memory_mb:.1f} MB")
                return memory_mb
    except:
        print("⚠️  psutil non installé, impossible de mesurer la mémoire")
    return None

def run_all_benchmarks():
    """Exécute tous les benchmarks"""
    print("=" * 60)
    print("🏁 BENCHMARKS DE PERFORMANCE AATAA")
    print("=" * 60)
    print()
    
    results = {}
    
    # Startup
    _, results['startup_time'] = benchmark_startup()
    
    # Load documents
    _, results['load_1000_docs'] = benchmark_load_documents(1000)
    
    # Search
    _, results['search_time'] = benchmark_search()
    
    # Classification
    _, results['classification_time'] = benchmark_classification()
    
    # Memory
    results['memory_mb'] = measure_memory()
    
    print()
    print("=" * 60)
    print("📊 RÉSULTATS")
    print("=" * 60)
    
    # Vérifier si les critères sont respectés
    criteria = {
        'Démarrage': (results['startup_time'], 2.0, 's'),
        'Chargement 1000 docs': (results['load_1000_docs'], 1.0, 's'),
        'Recherche': (results['search_time'], 0.5, 's'),
        'Classification': (results['classification_time'], 3.0, 's'),
    }
    
    if results['memory_mb']:
        criteria['Mémoire'] = (results['memory_mb'], 300, 'MB')
    
    for label, (value, threshold, unit) in criteria.items():
        status = "✅" if value <= threshold else "❌"
        print(f"{status} {label}: {value:.3f}{unit} (seuil: {threshold}{unit})")
    
    print()
    
    # Sauvegarder résultats
    with open('benchmark_results.json', 'w') as f:
        json.dump(results, f, indent=2)
    print("💾 Résultats sauvegardés dans benchmark_results.json")

if __name__ == "__main__":
    run_all_benchmarks()
```

---

## 🚀 Optimisations si Nécessaire

### 1. Pagination Backend

**Si le chargement est lent avec 1000+ docs** :

```rust
#[command]
pub async fn get_documents_paginated(
    page: usize,
    page_size: usize,
    state: tauri::State<'_, AppState>
) -> Result<PaginatedResult, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    let total = db.count_documents()?;
    let offset = page * page_size;
    
    let documents = db.get_documents_range(offset, page_size)?;
    
    Ok(PaginatedResult {
        documents,
        page,
        page_size,
        total,
        total_pages: (total + page_size - 1) / page_size
    })
}
```

### 2. Virtual Scrolling Frontend

**Si le rendu liste est lent** :

```vue
<template>
  <RecycleScroller
    :items="documents"
    :item-size="80"
    key-field="id"
    v-slot="{ item }"
  >
    <DocumentCard :document="item" />
  </RecycleScroller>
</template>

<script setup>
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
</script>
```

**Installation** :
```bash
npm install vue-virtual-scroller
```

### 3. Index Database

**Si la recherche est lente** :

```sql
-- Déjà présent : FTS5 pour recherche full-text
CREATE VIRTUAL TABLE documents_fts USING fts5(
    content, 
    new_name, 
    notes,
    content='documents'
);

-- Ajouter index sur catégories/dates si filtrage lent
CREATE INDEX idx_category ON documents(category, created_at DESC);
CREATE INDEX idx_date ON documents(created_at DESC);
```

### 4. Cache Frontend

**Cache catégories et stats** :

```typescript
// frontend/src/composables/useDocuments.ts

const documentsCache = ref<Document[]>([])
const categoriesCache = ref<string[]>([])
const lastFetch = ref<number>(0)
const CACHE_DURATION = 30000 // 30s

async function loadDocuments(forceRefresh = false) {
  const now = Date.now()
  
  if (!forceRefresh && now - lastFetch.value < CACHE_DURATION) {
    return documentsCache.value
  }
  
  const docs = await invoke('get_all_documents')
  documentsCache.value = docs
  lastFetch.value = now
  
  return docs
}
```

---

## 📋 Plan de Test

### Phase 1 : Génération Données (30 min)

```bash
# Générer 1000 documents
cd python
python generate_test_data.py 1000 ../test_documents

# Importer dans l'app
# Via UI : glisser-déposer tous les documents
```

### Phase 2 : Benchmarks (1h)

```bash
# Lancer benchmarks
python benchmark.py

# Vérifier résultats
cat benchmark_results.json
```

### Phase 3 : Optimisations si Nécessaire (1 jour)

**Si un critère échoue** :
1. Identifier le goulot (logs, profiling)
2. Appliquer optimisation ciblée
3. Re-benchmark
4. Itérer jusqu'à succès

### Phase 4 : Tests Utilisateur (2h)

**Scénarios** :
1. Démarrage avec 1000 docs
2. Scroll dans la liste
3. Recherche "facture" → mesurer temps réponse
4. Filtrer par catégorie
5. Importer nouveau document
6. Ouvrir document

**Critères subjectifs** :
- ✅ UI reste fluide (60 FPS)
- ✅ Pas de freeze
- ✅ Feedback instantané (<100ms perçu)

---

## ✅ Checklist

### Préparation

- [ ] Créer `generate_test_data.py`
- [ ] Installer dépendances : `pip install reportlab pillow psutil`
- [ ] Créer `benchmark.py`
- [ ] Générer 1000 documents de test

### Benchmarking

- [ ] Mesurer démarrage app
- [ ] Mesurer chargement 1000 docs
- [ ] Mesurer recherche full-text
- [ ] Mesurer classification
- [ ] Mesurer mémoire idle et chargée

### Optimisations (si nécessaire)

- [ ] Pagination backend
- [ ] Virtual scrolling frontend
- [ ] Index database supplémentaires
- [ ] Cache frontend
- [ ] Lazy loading des previews

### Validation

- [ ] Tous les critères respectés
- [ ] Tests utilisateur réussis
- [ ] Documentation résultats
- [ ] Commit final

---

## 📊 Critères de Réussite

| Métrique                  | Seuil    | Résultat | Status |
| ------------------------- | -------- | -------- | ------ |
| Démarrage app             | < 2s     | _TBD_    | ⏳      |
| Chargement 1000 docs      | < 1s     | _TBD_    | ⏳      |
| Recherche full-text       | < 500ms  | _TBD_    | ⏳      |
| Classification doc        | < 3s     | _TBD_    | ⏳      |
| Mémoire idle              | < 150MB  | _TBD_    | ⏳      |
| Mémoire avec 1000 docs    | < 300MB  | _TBD_    | ⏳      |
| Fluidité UI (subjective)  | 60 FPS   | _TBD_    | ⏳      |

---

## 📈 Estimation

**Temps total** : 2 jours

- Jour 1 : Scripts génération + benchmarks + tests initiaux
- Jour 2 : Optimisations si nécessaire + validation finale

**Si tout est déjà performant** : Gain de 1 jour ! 🎉

---

## 🎯 Résultat Attendu

**App scalable et performante** :
- ✅ 1000+ documents sans ralentissement
- ✅ Recherche instantanée
- ✅ UI fluide en permanence
- ✅ Mémoire sous contrôle
- ✅ Prête pour utilisateurs intensifs

**Preuve de qualité professionnelle** 🚀
