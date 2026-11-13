//! Windows registry read helpers for Comsol Multiphysics.

pub const APP_KEY: &str = "Software\\Comsol\\Comsol Multiphysics";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Comsol Multiphysics";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Comsol Multiphysics") },
        RegValue { name: "Version",     value: String::from("v3.2.23") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Comsol Multiphysics") },
        RegValue { name: "Publisher",       value: String::from("Comsol") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Comsol Multiphysics\\uninstall.exe\"") },
    ]
}
