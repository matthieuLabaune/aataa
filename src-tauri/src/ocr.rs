use std::path::Path;
use tesseract::Tesseract;

pub struct OcrEngine {}

impl OcrEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Just create the struct, Tesseract will be initialized per-use
        Ok(OcrEngine {})
    }

    pub fn extract_text_from_image(&mut self, image_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        // Perform OCR using tesseract crate with multilingual support (French + English + Spanish)
        let text = Tesseract::new(None, Some("fra+eng+spa"))?
            .set_image(image_path.to_str().ok_or("Invalid path")?)?
            .get_text()?;

        Ok(text)
    }

    pub fn extract_text_from_pdf(&mut self, pdf_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        // Extract text from PDF
        let bytes = std::fs::read(pdf_path)?;

        // Try to extract text directly first
        if let Ok(text) = pdf_extract::extract_text_from_mem(&bytes) {
            let trimmed = text.trim();
            if !trimmed.is_empty() && trimmed.len() > 50 {
                // If we got meaningful text, return it
                return Ok(text);
            }
        }

        // For scanned PDFs or PDFs without text, we'd need image conversion
        // For now, try to extract any text we can find
        let text = pdf_extract::extract_text_from_mem(&bytes).unwrap_or_default();

        if text.trim().is_empty() {
            // Return a placeholder for scanned PDFs
            Ok("Document scanné détecté. OCR complet à implémenter.".to_string())
        } else {
            Ok(text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_ocr_engine_creation() {
        let engine = OcrEngine::new();
        assert!(engine.is_ok(), "OcrEngine should be created successfully");
    }

    #[test]
    fn test_extract_text_from_pdf_empty() {
        // Create a minimal PDF structure (simplified for testing)
        let mut temp_file = NamedTempFile::new().unwrap();
        
        // Write minimal PDF header and trailer (this will be recognized as PDF but have no text)
        temp_file.write_all(b"%PDF-1.4\n").unwrap();
        temp_file.write_all(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n").unwrap();
        temp_file.write_all(b"2 0 obj\n<< /Type /Pages /Kids [] /Count 0 >>\nendobj\n").unwrap();
        temp_file.write_all(b"xref\n0 3\n0000000000 65535 f\n").unwrap();
        temp_file.write_all(b"0000000009 00000 n\n").unwrap();
        temp_file.write_all(b"0000000058 00000 n\n").unwrap();
        temp_file.write_all(b"trailer\n<< /Size 3 /Root 1 0 R >>\n").unwrap();
        temp_file.write_all(b"startxref\n109\n%%EOF\n").unwrap();
        temp_file.flush().unwrap();

        let mut engine = OcrEngine::new().unwrap();
        let result = engine.extract_text_from_pdf(temp_file.path());

        assert!(result.is_ok(), "PDF extraction should not fail");
        
        // Empty PDF should return the scanned document message
        let text = result.unwrap();
        assert!(
            text.contains("Document scanné") || text.trim().is_empty(),
            "Empty PDF should return placeholder or empty text"
        );
    }

    #[test]
    fn test_extract_text_from_invalid_path() {
        let mut engine = OcrEngine::new().unwrap();
        let result = engine.extract_text_from_pdf(Path::new("/nonexistent/file.pdf"));

        assert!(result.is_err(), "Should fail for nonexistent file");
    }

    // Note: Image OCR tests require tesseract to be installed and configured
    // These are integration tests that would need actual image files
    // For unit tests, we verify the error handling paths
    
    #[test]
    fn test_extract_text_from_invalid_image() {
        let mut engine = OcrEngine::new().unwrap();
        let result = engine.extract_text_from_image(Path::new("/nonexistent/image.png"));

        assert!(result.is_err(), "Should fail for nonexistent image file");
    }
}

