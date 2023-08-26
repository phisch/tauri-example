#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub use tauri::{
    utils::assets::{AssetKey, EmbeddedAssets},
    Assets,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn get_context() -> tauri::Context<EmbeddedAssets> {
    tauri::generate_context!()
}

pub fn run() {
    let context = get_context();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .run(context)
        .expect("error while running tauri application");
}
