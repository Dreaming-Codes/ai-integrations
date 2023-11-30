use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use tauri::{Window, WindowBuilder, WindowEvent, WindowUrl};
use thiserror::Error;
use tokio::sync::oneshot;
use macro_utils::SerializeError;
use crate::ocr::screenshot::{ScreenshotError, take_screenshot};
use crate::ocr::tesseract::{scan_text, ScanTextError};

#[derive(Debug, Clone, Serialize)]
pub struct ScreenArea {
    pub start: (u32, u32),
    pub size: (u32, u32),
    pub monitor_position: (i32, i32),
}

#[derive(Error, Debug, SerializeError)]
pub enum SelectAreaError {
    #[error("The selection was cancelled by the user")]
    Cancelled,
    #[error("Unable to parse selection from the frontend")]
    UnableToParseSelection(#[source] serde_json::Error),
    #[error("Unable to get available monitors")]
    UnableToGetAvailableMonitors(#[source] tauri::Error),
    #[error("Error while building window")]
    ErrorWhileBuildingWindow(#[source] tauri::Error),
    #[error("Error while getting window title")]
    ErrorWhileGettingWindowTitle(#[source] tauri::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScreenAreaResponse {
    #[serde(rename = "startX")]
    start_x: u32,
    #[serde(rename = "startY")]
    start_y: u32,
    #[serde(rename = "endX")]
    end_x: u32,
    #[serde(rename = "endY")]
    end_y: u32,
}

pub async fn select_area(app_handle: tauri::AppHandle) -> Result<ScreenArea, SelectAreaError> {
    let monitors = app_handle.available_monitors().map_err(SelectAreaError::UnableToGetAvailableMonitors)?;

    // We try to create a window for each monitor to get the selection area if one window creation fails we close all windows and return the error
    let mut windows: Vec<Window> = Vec::new();
    for (index, monitor) in monitors.iter().enumerate() {
        let monitor_position = monitor.position();
        let window_result = WindowBuilder::new(&app_handle, format!("area-selector-{}", index), WindowUrl::App("windows/area-selector".into()))
            .title(format!("Select Area {}x{}", monitor_position.x, monitor_position.y))
            .transparent(true)
            .fullscreen(true)
            .focused(true)
            .always_on_top(true)
            .visible_on_all_workspaces(true)
            .closable(false)
            .content_protected(true)
            .decorations(false)
            .disable_file_drop_handler()
            .maximized(true)
            .shadow(false)
            .skip_taskbar(true)
            // We need to set the window position to the monitor position,
            // otherwise the window will be created on the primary monitor
            .position(monitor_position.x.into(), monitor_position.y.into())
            .build().map_err(SelectAreaError::ErrorWhileBuildingWindow);

        match window_result {
            Ok(window) => {
                windows.push(window);
            }
            Err(err) => {
                // Close all windows so far on an error
                for window in &windows {
                    let _ = window.close();
                }
                return Err(err);  // Return the error afterwards
            },
        }
    }

    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));


    windows.iter().for_each(|window| {
        window.on_window_event({
            let windows = windows.clone();
            let tx = tx.clone();
            move |event| {
                if let WindowEvent::CloseRequested { .. } = event {
                    close_windows(&windows);
                    send_error(&tx, SelectAreaError::Cancelled);
                }
            }
        });
        window.listen("selection", {
            let windows = windows.clone();
            let window = window.clone();
            let tx = tx.clone();
            move |event| {
                let title = window.title().map_err(SelectAreaError::ErrorWhileGettingWindowTitle);

                close_windows(&windows);

                let Ok(title) = title else {
                    send_error(&tx, title.unwrap_err());
                    return;
                };

                let monitor_position = title.split(" ").nth(2).and_then(|position| {
                    let mut position = position.split("x");
                    let x = position.next().and_then(|x| x.parse::<i32>().ok());
                    let y = position.next().and_then(|y| y.parse::<i32>().ok());
                    x.and_then(|x| y.map(|y| (x, y)))
                }).unwrap_or((0, 0));

                let area = serde_json::from_str::<ScreenAreaResponse>(&event.payload()).map_err(SelectAreaError::UnableToParseSelection);

                let Ok(area) = area else {
                    send_error(&tx, area.unwrap_err());
                    return;
                };

                let x = area.start_x.min(area.end_x);
                let y = area.start_y.min(area.end_y);

                let width = (area.start_x.max(area.end_x) - x).max(1);
                let height = (area.start_y.max(area.end_y) - y).max(1);

                // Parse the selection from event and send it through the channel
                let screen_area = ScreenArea { start: (x, y), size: (width, height), monitor_position };
                let mut tx = tx.lock().expect("Unable to lock tx");
                if let Some(tx) = tx.take() {
                    tx.send(Ok(screen_area)).ok();
                }
            }
        });
    });

    // Wait for the selection to be made or cancelled
    match rx.await {
        Ok(selection) => selection,
        Err(_) => Err(SelectAreaError::Cancelled), // Receiver error implies that all senders were dropped, so we consider it as cancellation
    }
}

#[derive(Error, Debug, SerializeError)]
pub enum DoFullOcrError {
    #[error("Unable to select area")]
    SelectArea(#[from] SelectAreaError),
    #[error("Unable to take screenshot")]
    TakeScreenshot(#[from] ScreenshotError),
    #[error("Unable to OCR image")]
    ParseImage(#[from] ScanTextError)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// Since we are using some blocking functions like take_screenshot and scan_text marking this as async command will force Tauri to run it in a separate thread
#[tauri::command(async)]
pub async fn do_full_ocr(app_handle: tauri::AppHandle) -> Result<String, DoFullOcrError> {
    let area = select_area(app_handle).await?;
    println!("Selected area: {:?}", area);
    let image = take_screenshot(&area)?;

    let text = scan_text(&image, "eng").await?;

    Ok(text)
}


fn send_error(tx: &Arc<Mutex<Option<oneshot::Sender<Result<ScreenArea, SelectAreaError>>>>>, err: SelectAreaError) {
    let mut tx = tx.lock().expect("Unable to lock tx");
    if let Some(tx) = tx.take() {
        tx.send(Err(err)).ok();
    }
}

fn close_windows(windows: &[Window]) {
    windows.iter().for_each(|window| {
        let _ = window.close();
    });
}
