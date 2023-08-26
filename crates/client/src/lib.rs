#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub use tauri::{
    utils::assets::{AssetKey, EmbeddedAssets},
    Assets,
};

pub fn get_context() -> tauri::Context<EmbeddedAssets> {
    tauri::generate_context!()
}

pub fn run() {
    let context = get_context();
    tauri::Builder::default()
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .run(context)
        .expect("error while running tauri application");
}
