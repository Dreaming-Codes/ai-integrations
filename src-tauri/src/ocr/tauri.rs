use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{Window, WindowBuilder, WindowEvent, WindowUrl};
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Debug, Clone, Serialize)]
pub struct ScreenArea {
    start: (u32, u32),
    size: (u32, u32),
    monitor_position: (i32, i32),
}

#[derive(Error, Debug)]
pub enum SelectAreaError {
    #[error("The selection was cancelled by the user")]
    Cancelled,
    #[error("Unable to parse selection from the frontend")]
    UnableToParseSelection,
    #[error("Unable to get available monitors")]
    UnableToGetAvailableMonitors(#[source] tauri::Error),
    #[error("Error while building window")]
    ErrorWhileBuildingWindow(#[source] tauri::Error),
    #[error("Error while getting window position")]
    ErrorWhileGettingWindowPosition(#[source] tauri::Error),
}

impl Serialize for SelectAreaError {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn select_area(app_handle: tauri::AppHandle) -> Result<ScreenArea, SelectAreaError> {
    let monitors = app_handle.available_monitors().map_err(SelectAreaError::UnableToGetAvailableMonitors)?;

    // We try to create a window for each monitor to get the selection area if one window creation fails we close all windows and return the error
    let mut windows: Vec<Window> = Vec::new();
    for (index, monitor) in monitors.iter().enumerate() {
        let monitor_position = monitor.position();
        let window_result = WindowBuilder::new(&app_handle, format!("area-selector-{}", index), WindowUrl::App("windows/area-selector".into()))
            .title("Select Area")
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
                let position = window.inner_position().map_err(SelectAreaError::ErrorWhileGettingWindowPosition);

                close_windows(&windows);

                let Ok(position) = position else {
                    send_error(&tx, position.unwrap_err());
                    return;
                };

                let area = event.payload().map_or(Err(SelectAreaError::UnableToParseSelection), |payload| {
                    serde_json::from_str::<ScreenAreaResponse>(&payload).map_err(|_| SelectAreaError::UnableToParseSelection)
                });

                let Ok(area) = area else {
                    send_error(&tx, area.unwrap_err());
                    return;
                };

                let x = area.start_x.min(area.end_x);
                let y = area.start_y.min(area.end_y);

                let width = (area.start_x.max(area.end_x) - x).max(1);
                let height = (area.start_y.max(area.end_y) - y).max(1);

                // Parse the selection from event and send it through the channel
                let screen_area = ScreenArea { start: (x, y), size: (width, height), monitor_position: (position.x, position.y) };
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
