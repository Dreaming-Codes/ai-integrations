use tesseract_native::leptonica_plumbing::PixReadMemError;
use tesseract_native::tesseract::{InitializeError, Tesseract};
use tesseract_native::tesseract::tesseract_plumbing::TessBaseApiGetUtf8TextError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanTextError {
    #[error("Error while initializing tesseract")]
    InitializeError(#[from] InitializeError),
    #[error("Error while reading image")]
    ImageError(#[from] PixReadMemError),
    #[error("Error while scanning text")]
    OCRFailed(#[from] TessBaseApiGetUtf8TextError)
}

pub fn scan_text(image: &[u8], language: &str) -> Result<String, ScanTextError> {
    let mut tesseract = Tesseract::new(None, Some(language))?
        .set_image_from_mem(image)?;

    let text = tesseract.get_text()?;

    Ok(text)
}
