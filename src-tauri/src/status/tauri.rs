use std::sync::{Mutex};
use tauri::{State, WindowBuilder, WindowUrl};
use thiserror::Error;
use macro_utils::SerializeError;

#[derive(Error, Debug, SerializeError)]
pub enum DisplayStatusError {
    #[error("Error while building window")]
    ErrorWhileBuildingWindow(#[source] tauri::Error),
    #[error("Error while updating status")]
    ErrorWhileUpdatingStatus(#[source] tauri::Error),
}

#[derive(Default)]
pub struct StatusWindow(Mutex<Option<tauri::Window>>);

#[tauri::command]
pub fn display_status(app_handle: tauri::AppHandle, status_window: State<StatusWindow>, status: &str) -> Result<(), DisplayStatusError> {
    let status_window_lock = status_window.0.lock().expect("Failed to lock status window");

    match status_window_lock.as_ref() {
        Some(window) => {
            window.emit("update-status", Some(status.to_string())).map_err(DisplayStatusError::ErrorWhileUpdatingStatus)?;
        }
        None => {
            // Drop the lock first to prevent deadlock
            drop(status_window_lock);
            const SIZE: f64 = 40_f64;

            let window = WindowBuilder::new(&app_handle, "status", WindowUrl::App(format!("windows/status-semaphore?status={}", status).into()))
                .title("Status")
                .transparent(true)
                .decorations(false)
                .skip_taskbar(true)
                .closable(false)
                .visible_on_all_workspaces(true)
                .always_on_top(true)
                .disable_file_drop_handler()
                .content_protected(true)
                .inner_size(SIZE, SIZE)
                .shadow(false)
                .center()
                .build()
                .map_err(DisplayStatusError::ErrorWhileBuildingWindow)?;

            let _ = window.set_ignore_cursor_events(true);

            // After building the window, lock the status_window again to replace the content.
            let mut status_window_lock = status_window.0.lock().expect("Failed to lock status window");
            *status_window_lock = Some(window);
        }
    };

    Ok(())
}

#[derive(Error, Debug, SerializeError)]
pub enum CloseStatusError {
    #[error("Error while closing window")]
    ErrorWhileClosingWindow(#[source] tauri::Error),
    #[error("No window to close")]
    NoWindow,
}

#[tauri::command]
pub fn close_status(status_window: State<StatusWindow>) -> Result<(), CloseStatusError> {
    let mut status_window_lock = status_window.0.lock().expect("Failed to lock status window");

    status_window_lock.take().map_or(Err(CloseStatusError::NoWindow), |window| {
        window.close().map_err(CloseStatusError::ErrorWhileClosingWindow)
    })
}
