#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Submenu, Menu, MenuItem, generate_context};
fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "离开");
  let close = CustomMenuItem::new("close".to_string(), "关闭");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);


  tauri::Builder::default()
    .menu(menu)
    .run(generate_context!())
    .expect("error while running tauri application");
}
