use crate::app::config::AppConf;
use crate::app::dns::IDNS;
use tauri::AppHandle;

use super::{config::DNSItem, menu};

#[tauri::command]
pub fn get_dns() -> (String, Vec<DNSItem>) {
  let configs = AppConf::read();
  let server = IDNS::get_system_dns();
  (server, configs.items)
}

#[tauri::command]
pub fn add_dns(dns: String) -> () {
  let mut configs = AppConf::read();
  configs.items.insert(
    0,
    DNSItem {
      value: dns.clone(),
      label: dns.clone(),
    },
  );
  configs.write();
}

#[tauri::command]
pub fn set_dns(dns: String) -> () {
  let mut configs = AppConf::read();
  configs.select = dns.clone();
  configs.write();
  IDNS::set_dns(dns)
}

#[tauri::command]
pub fn del_dns(dns: String) -> () {
  let mut configs = AppConf::read();
  configs.items.retain(|x| x.value.as_str() != dns.as_str());
  if configs.select == dns.as_str() {
    configs.select = "".to_owned();
    IDNS::empty_dns();
  }
  configs.write();
}

#[tauri::command]
pub fn refresh_tray_menu(app: AppHandle) -> () {
  let _ = app.tray_handle().set_menu(menu::init_tray_menu());
}
