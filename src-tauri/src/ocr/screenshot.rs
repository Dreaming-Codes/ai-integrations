use image::DynamicImage;
use screenshots::Screen;
use thiserror::Error;
use crate::ocr::tauri::ScreenArea;

#[derive(Error, Debug)]
pub enum ScreenshotError {
    #[error("Unable to get screen from position")]
    UnableToGetScreen(#[source] anyhow::Error),
    #[error("Unable to capture screen")]
    UnableToCaptureScreenshot(#[source] anyhow::Error),
}

pub fn take_screenshot(screen_area: &ScreenArea) -> Result<DynamicImage, ScreenshotError> {
    let screen = Screen::from_point(screen_area.monitor_position.0, screen_area.monitor_position.1).map_err(ScreenshotError::UnableToGetScreen)?;
    let image = screen.capture_area(screen_area.start.0 as i32, screen_area.start.1 as i32, screen_area.size.0, screen_area.size.1).map_err(ScreenshotError::UnableToCaptureScreenshot)?;
    Ok(image.into())
}
