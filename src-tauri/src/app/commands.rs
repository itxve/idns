use crate::app::config::AppConf;
use sysdns::SysDNS;
use tauri::AppHandle;

use super::{config::DNSItem, menu};

pub fn _set_dns(server: String) -> () {
    let _ = SysDNS {
        enable: SysDNS::is_support(),
        server,
    }
    .set_system_dns();
}

#[tauri::command]
pub fn get_dns() -> (String, Vec<DNSItem>) {
    let configs = AppConf::read();
    let dns = SysDNS::get_system_dns().unwrap();
    (dns.server, configs.items)
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
    _set_dns(dns)
}

#[tauri::command]
pub fn del_dns(dns: String) -> () {
    let mut configs = AppConf::read();
    configs.items.retain(|x| x.value.as_str() != dns.as_str());
    if configs.select == dns.as_str() {
        configs.select = "".to_owned();
        _set_dns("Empty".to_owned())
    }
    configs.write();
}

#[tauri::command]
pub fn refresh_tray_menu(app: AppHandle) -> () {
    let _ = app.tray_handle().set_menu(menu::init_tray_menu());
}
