use std::io::Cursor;
use image::DynamicImage;
use tesseract::InitializeError;
use tesseract::plumbing::leptonica_plumbing::PixReadMemError;
use tesseract::plumbing::TessBaseApiGetUtf8TextError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanTextError {
    #[error(transparent)]
    InitializeError(#[from] InitializeError),
    #[error(transparent)]
    DecodeError(#[from] image::ImageError),
    #[error(transparent)]
    ImageError(#[from] PixReadMemError),
    #[error(transparent)]
    TextScanError(#[from] TessBaseApiGetUtf8TextError),
}

pub async fn scan_text(image: &DynamicImage, language: &str) -> Result<String, ScanTextError> {
    let mut decoded_image: Vec<u8> = vec![];

    image.write_to(&mut Cursor::new(&mut decoded_image), image::ImageOutputFormat::Tiff)?;

    let text = tesseract::Tesseract::new(None, Some(language))?.set_image_from_mem(decoded_image.as_slice())?.get_text()?.trim().to_string();

    Ok(text)
}
