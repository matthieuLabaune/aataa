use std::path::Path;
use tesseract::Tesseract;

pub struct OcrEngine {}

impl OcrEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Just create the struct, Tesseract will be initialized per-use
        Ok(OcrEngine {})
    }

    pub fn extract_text_from_image(&mut self, image_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        // Perform OCR using tesseract crate
        let text = Tesseract::new(None, Some("eng"))?
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
