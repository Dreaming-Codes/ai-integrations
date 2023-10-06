// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowBuilder, WindowEvent, WindowUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn select_area(app_handle: tauri::AppHandle) {
    let monitors = app_handle.available_monitors().expect("error while getting available monitors");

    let windows = monitors.iter().enumerate().map(|(index, monitor)| {
        let monitor_position = monitor.position();
        let window = WindowBuilder::new(&app_handle, format!("area-selector-{}", index), WindowUrl::App("windows/area-selector".into()))
            .title("Select Area")
            .transparent(true)
            .fullscreen(true)
            .focused(true)
            .always_on_top(true)
            .visible_on_all_workspaces(true)
            .closable(false)
            // This is done so that it can be used in a stealthy mode also when monitored by screen capturing software
            .content_protected(true)
            .decorations(false)
            .disable_file_drop_handler()
            .maximized(true)
            .skip_taskbar(true)
            // We need to set the window position to the monitor position, otherwise the window will be created on the primary monitor
            .position(monitor_position.x.into(), monitor_position.y.into())
            .build().expect("error while building window");

        window
    }).collect::<Vec<_>>();

    // When one window is closed, all windows get closed
    windows.iter().for_each(|window| {
        window.on_window_event({
            let windows = windows.clone();
            move |event| {
                if let WindowEvent::CloseRequested { .. } = event {
                    windows.iter().for_each(|window| {
                        window.close().expect("error while closing window");
                    });
                }
            }
        });
        window.listen("selection", {
            let windows = windows.clone();
            move |event| {
                windows.iter().for_each(|window| {
                    window.close().expect("error while closing window");
                });
                todo!("do something with the selection");
            }
        });
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![select_area])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
