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
            if !text.trim().is_empty() {
                return Ok(text);
            }
        }

        // If no text or OCR needed, return message
        Ok("PDF OCR not yet fully implemented. Use image files for now.".to_string())
    }
}
