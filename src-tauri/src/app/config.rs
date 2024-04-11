#![allow(unused)]
use crate::utils::{app_root, create_file, exists};
use log::{error, info};
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};
use sysdns::SysDNS;
use tauri::Manager;

pub const APP_CONFIG_FILE: &str = "dns.conf.json";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AppConf {
    pub select: String,
    pub auto_launch: bool,
    pub items: Vec<DNSItem>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DNSItem {
    pub value: String,
    pub label: String,
}

impl AppConf {
    pub fn new() -> Self {
        Self {
            select: "".to_owned(),
            auto_launch: false,
            items: vec![],
        }
    }

    pub fn file_path() -> PathBuf {
        app_root().join(APP_CONFIG_FILE)
    }

    pub fn read() -> Self {
        match std::fs::read_to_string(Self::file_path()) {
            Ok(v) => {
                if let Ok(v2) = serde_json::from_str::<AppConf>(&v) {
                    v2
                } else {
                    error!("conf_read_parse_error");
                    Self::default()
                }
            }
            Err(err) => {
                error!("conf_read_error: {}", err);
                Self::default()
            }
        }
    }

    pub fn write(self) -> Self {
        let path = &Self::file_path();
        if !exists(path) {
            create_file(path).unwrap();
            info!("conf_create");
        }
        if let Ok(v) = serde_json::to_string_pretty(&self) {
            std::fs::write(path, v).unwrap_or_else(|err| {
                error!("conf_write: {}", err);
                Self::default().write();
            });
        } else {
            error!("conf_ser");
        }
        self
    }

    pub fn amend_str(self, json: String) -> Self {
        let value: Value =
            serde_json::from_str(json.as_str()).expect("JSON was not well-formatted");
        self.amend(value)
    }

    pub fn amend(self, json: Value) -> Self {
        let val = serde_json::to_value(&self).unwrap();
        let mut config: BTreeMap<String, Value> = serde_json::from_value(val).unwrap();
        let new_json: BTreeMap<String, Value> = serde_json::from_value(json).unwrap();

        for (k, v) in new_json {
            config.insert(k, v);
        }

        match serde_json::to_string_pretty(&config) {
            Ok(v) => match serde_json::from_str::<AppConf>(&v) {
                Ok(v) => v,
                Err(err) => {
                    error!("conf_amend_parse: {}", err);
                    self
                }
            },
            Err(err) => {
                error!("conf_amend_str: {}", err);
                self
            }
        }
    }

    pub fn restart(self, app: tauri::AppHandle) {
        tauri::api::process::restart(&app.env());
    }
}

impl Default for AppConf {
    fn default() -> Self {
        Self::new()
    }
}
