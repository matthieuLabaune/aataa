# Changelog

All notable changes to AATAA will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### To Add
- Native file dialog
- Document preview
- Advanced filters
- CSV/JSON export

## [0.1.0] - 2024-10-09

### Added
- **Core Features**
  - File import (PNG, JPG, JPEG, PDF)
  - OCR via Tesseract
  - Automatic document classification (6 types)
  - Intelligent renaming
  - Automatic tagging (5 types)
  - SQLite local storage
  - Local archiving

- **Document Types Detection**
  - Invoices (FACT)
  - Contracts (CONT)
  - Bank statements (BANK)
  - Payslips (PAIE)
  - Official documents (OFFI)
  - Receipts (RECU)

- **Automatic Tags**
  - contains_date
  - contains_amount
  - company_document
  - contains_email
  - contains_phone

- **User Interface**
  - Modern Nuxt UI interface
  - Full-text search
  - Document list with metadata
  - Actions (open, delete)
  - Settings (archive path)
  - Light/dark mode

- **Backend (Rust)**
  - `models.rs` - Data structures
  - `database.rs` - SQLite layer
  - `ocr.rs` - Tesseract OCR engine
  - `classifier.rs` - Classification & tagging
  - `commands.rs` - Tauri API (7 commands)
  - `lib.rs` - App initialization

- **Frontend (Vue/TypeScript)**
  - `app.vue` - Main application
  - `DocumentCard.vue` - Document display component
  - `EmptyState.vue` - Empty state component
  - Type definitions

- **Documentation**
  - README.md - User guide
  - ARCHITECTURE.md - Technical documentation
  - QUICKSTART.md - Quick start guide
  - TEST_GUIDE.md - Testing guide
  - CLASSIFICATION_PATTERNS.md - Pattern documentation
  - BUILD_SUMMARY.md - Build summary
  - FINAL_STATUS.md - Final status
  - TODO.md - Future improvements
  - CHANGELOG.md - This file

### Technical Details
- **Dependencies**
  - Tauri 2.8.5
  - Nuxt 4.1.3
  - Nuxt UI 3.0.0
  - Rust Tesseract 0.14
  - rusqlite 0.32
  - regex 1.10

- **Build**
  - Compiles successfully on macOS
  - Backend: ~519 lines of Rust
  - Frontend: ~438 lines of Vue/TS
  - Documentation: ~800+ lines

### Known Issues
- File dialog uses HTML input (temporary)
- PDF OCR limited to text extraction
- English OCR only (configurable)
- 2 unused import warnings in commands.rs

### Security
- 100% offline operation
- Local data storage only
- No telemetry
- No external network calls

---

## Version History

### [0.1.0] - 2024-10-09
- Initial MVP release
- Core functionality implemented
- Full documentation

---

## Future Releases

### Planned for 0.2.0
- Native file dialog
- Document preview
- Error handling UI
- Multi-language OCR

### Planned for 0.3.0
- Advanced filters
- Dashboard statistics
- CSV/JSON export
- Unit tests

### Planned for 0.4.0
- Custom patterns
- Automatic backup
- Cloud integrations
- CI/CD

### Planned for 1.0.0
- ML classification
- Mobile app (beta)
- REST API
- Complete documentation

---

[Unreleased]: https://github.com/yourusername/aataa/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/aataa/releases/tag/v0.1.0
