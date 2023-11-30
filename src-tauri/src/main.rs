// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ocr;
mod status;
mod ai;
mod builder;
mod hyprland_compat;

use log::info;
use tauri::async_runtime::spawn;
use ocr::tauri::do_full_ocr;
use status::tauri::display_status;
use status::tauri::close_status;

fn main() {
    std::env::set_var("GDK_BACKEND", "x11");
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    tauri::Builder::default()
        .manage(status::tauri::StatusWindow::default())
        .invoke_handler(tauri::generate_handler![do_full_ocr, display_status, close_status])
        .setup(|app| {
            if hyprland_compat::is_hyprland() {
                info!("Running on Hyprland, setting window rules");
                spawn(async {
                    hyprland_compat::set_rules().await.expect("Unable to set Hyprland rules");
                });
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::Ready { .. } => {
                info!("Application started");
            }
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();

                if hyprland_compat::is_hyprland() {
                    info!("Running on Hyprland, unsetting window rules");
                    spawn(async {
                        hyprland_compat::unset_rules().await.expect("Unable to unset Hyprland rules");
                    });
                }

                std::process::exit(0);
            }
            _ => {}
        });
}
