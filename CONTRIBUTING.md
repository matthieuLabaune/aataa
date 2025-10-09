# Contributing to AATAA

Thank you for considering contributing to AATAA! 🎉

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

This project adheres to a simple code of conduct:
- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report, include:
- **Clear title and description**
- **Steps to reproduce**
- **Expected vs actual behavior**
- **Screenshots** (if applicable)
- **System information** (OS, version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:
- **Use a clear and descriptive title**
- **Provide detailed description** of the proposed feature
- **Explain why** this enhancement would be useful
- **Include mockups** if applicable

### Your First Code Contribution

Unsure where to begin? Look for issues labeled:
- `good first issue` - Simple issues for beginners
- `help wanted` - Issues where we need assistance

## Development Setup

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (>= 18)
# Visit https://nodejs.org/

# Install Tesseract OCR
# macOS
brew install tesseract tesseract-lang

# Ubuntu/Debian
sudo apt-get install tesseract-ocr tesseract-ocr-fra
```

### Setup Project
```bash
# Clone repository
git clone https://github.com/yourusername/aataa.git
cd aataa

# Install dependencies
npm install

# Run development server
npm run tauri:dev
```

## Coding Standards

### Rust Code
- Follow Rust official style guide
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new features
- Document public APIs

```rust
// Good example
/// Processes a file and returns document metadata
pub async fn process_file(
    file_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<Document, String> {
    // Implementation
}
```

### TypeScript/Vue Code
- Use TypeScript strict mode
- Follow Vue 3 Composition API best practices
- Use meaningful variable names
- Add JSDoc comments for complex functions

```typescript
// Good example
/**
 * Loads all documents from the database
 * @returns Promise resolving to array of documents
 */
async function loadDocuments(): Promise<Document[]> {
  return await invoke<Document[]>('get_documents')
}
```

### File Organization
```
src-tauri/src/
  ├── models.rs      # Data structures
  ├── database.rs    # Database operations
  ├── ocr.rs         # OCR functionality
  ├── classifier.rs  # Classification logic
  └── commands.rs    # Tauri commands

app/
  └── app.vue        # Main application

components/
  ├── DocumentCard.vue
  └── EmptyState.vue
```

## Commit Guidelines

### Commit Message Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(ocr): add multi-language support

Add support for French and German OCR detection.
Tesseract can now be initialized with multiple languages.

Closes #42

---

fix(ui): correct search input behavior

Search now properly updates when query is cleared.
```

## Pull Request Process

1. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Write code
   - Add tests
   - Update documentation

3. **Test your changes**
   ```bash
   # Run tests
   cargo test
   npm test

   # Build project
   cargo build
   npm run build
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: your feature description"
   ```

5. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create Pull Request**
   - Go to GitHub repository
   - Click "New Pull Request"
   - Fill in the template
   - Link related issues

### PR Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests passing
- [ ] No new warnings
- [ ] Commit messages follow guidelines

## Adding New Document Types

To add a new document type:

1. **Update classifier.rs**
```rust
DocumentType {
    name: "New Type".to_string(),
    pattern: r"(?i)(keyword1|keyword2)".to_string(),
    prefix: "NEWT".to_string(),
}
```

2. **Update app.vue**
```typescript
const typeColors: Record<string, string> = {
  'New Type': 'cyan',
  // ... existing types
}
```

3. **Update documentation**
- Add to CLASSIFICATION_PATTERNS.md
- Update README.md

4. **Add tests**
```rust
#[test]
fn test_classify_new_type() {
    let classifier = Classifier::new();
    let doc_type = classifier.classify("keyword1 test");
    assert_eq!(doc_type.name, "New Type");
}
```

## Adding New Tags

1. **Update classifier.rs**
```rust
let new_tag_re = Regex::new(r"pattern").unwrap();
if new_tag_re.is_match(text) {
    tags.push("new_tag".to_string());
}
```

2. **Update documentation**
- Add to CLASSIFICATION_PATTERNS.md

3. **Add tests**

## Testing

### Rust Tests
```bash
cd src-tauri
cargo test
```

### Frontend Tests
```bash
npm test
```

### Manual Testing
- Test on different OSes
- Test with various document types
- Test edge cases
- Test performance with large files

## Documentation

When adding features:
- Update relevant .md files
- Add code comments
- Update CHANGELOG.md
- Add examples if needed

## Questions?

- Create an issue for discussion
- Ask in pull request comments
- Check existing documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to AATAA! 🙏
