// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

#[cfg(debug_assertions)]
mod app;
mod utils;
use app::{
  commands,
  config::AppConf,
  menu::{self},
};
use log::info;
use tauri_plugin_autostart::MacosLauncher;

#[tauri::command(main)]
fn main() {
  let context = tauri::generate_context!();
  let app = tauri::Builder::default();
  let _app_conf = AppConf::read().write();
  app
    .setup(|_app| {
      info!("app running...");
      Ok(())
    })
    .system_tray(menu::init_tray())
    .on_system_tray_event(menu::tray_handler)
    .menu(menu::init_menu())
    .on_menu_event(menu::menu_handler)
    .invoke_handler(tauri::generate_handler![
      commands::get_dns,
      commands::add_dns,
      commands::set_dns,
      commands::del_dns,
      commands::refresh_tray_menu
    ])
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      None,
    ))
    .build(context)
    .expect("error while running idns application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        println!("last close");
        api.prevent_exit();
      }
      _ => {}
    })
}
