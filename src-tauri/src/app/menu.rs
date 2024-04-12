use crate::app::config::AppConf;
use crate::app::dns::IDNS;

use crate::utils;

use serde_json::json;
use sysdns::SysDNS;
use tauri::{
  AppHandle, CustomMenuItem, Menu, Submenu, SystemTray, SystemTrayEvent, SystemTrayMenu,
  WindowMenuEvent, Wry,
};
use tauri::{Manager, MenuItem};
use tauri_plugin_autostart::ManagerExt;

pub fn init_menu() -> Menu {
  let name = "idns";
  let app_menu = Submenu::new(
    name,
    Menu::with_items([
      CustomMenuItem::new("check_update", "Check for Updates").into(),
      MenuItem::Services.into(),
      MenuItem::Hide.into(),
      MenuItem::Separator.into(),
      MenuItem::Quit.into(),
    ]),
  );
  Menu::new().add_submenu(app_menu)
}

pub fn menu_handler(event: WindowMenuEvent<tauri::Wry>) {
  let win = Some(event.window()).unwrap();
  let app = win.app_handle();
  let menu_id = event.menu_item_id();
  match menu_id {
    "check_update" => {
      utils::run_check_update(app, false, None);
    }
    _ => (),
  }
}

pub fn init_tray_menu() -> SystemTrayMenu {
  let mut tray_menu = SystemTrayMenu::new();
  let configs = AppConf::read();
  let select_value = configs.select.clone();
  for item in configs.items.iter() {
    let mut cunstom_item = CustomMenuItem::new(item.value.clone(), item.label.clone());
    if item.value.as_str() == select_value.as_str() {
      cunstom_item = cunstom_item.selected();
      IDNS::set_dns(select_value.to_owned());
    }
    tray_menu = tray_menu.add_item(cunstom_item);
  }
  // 没有配置就使用获取到的dns
  match SysDNS::get_system_dns() {
    Ok(dns) => {
      if configs.items.len() <= 0 {
        tray_menu =
          tray_menu.add_item(CustomMenuItem::new(dns.server.clone(), dns.server.clone()).selected())
      }
    }
    Err(_) => (),
  }

  tray_menu = tray_menu.add_item(CustomMenuItem::new("config".to_owned(), "添加常用dns"));
  let mut auto_launch_menu = CustomMenuItem::new("auto_launch".to_owned(), "开机自启动");

  if configs.auto_launch {
    auto_launch_menu = auto_launch_menu.selected();
  }
  tray_menu = tray_menu.add_item(auto_launch_menu);
  tray_menu
}

pub fn init_tray() -> SystemTray {
  SystemTray::new().with_menu(init_tray_menu())
}

pub fn tray_handler(app: &AppHandle<Wry>, event: SystemTrayEvent) {
  if let SystemTrayEvent::MenuItemClick { id, tray_id, .. } = event {
    if id == "config" {
      match app.get_window("main") {
        Some(w) => {
          w.show().unwrap();
        }
        None => {
          let _ =
            tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
              .title("idns")
              .maximizable(false)
              .max_inner_size(600f64, 600f64)
              .build()
              .unwrap();
        }
      };
      return;
    }
    let ha = app.tray_handle_by_id(tray_id.as_str()).unwrap();

    match id.as_str() {
      "auto_launch" => {
        let autolaunch = app.autolaunch();
        let is_enabled = autolaunch.is_enabled().unwrap();
        if is_enabled {
          let _ = autolaunch.disable();
        } else {
          let _ = autolaunch.enable();
        }
        let _ = AppConf::read()
          .amend(json!({"auto_launch":!is_enabled}))
          .write();

        ha.get_item(id.as_str()).set_selected(!is_enabled).unwrap();
        return;
      }
      _ => (),
    }

    let configs = AppConf::read();
    let id = id.as_str();
    for item in configs.items.iter() {
      let value = item.value.as_str();
      if id != value {
        ha.get_item(value).set_selected(false).unwrap();
      }
    }
    ha.get_item(id).set_selected(true).unwrap();
    IDNS::set_dns(id.to_owned());
    configs.amend(json!({"select":id})).write();
  }
}
