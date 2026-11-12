//! Windows registry read helpers for Oeksound Nimble Flutter.

pub const APP_KEY: &str = "Software\\Oeksound\\Oeksound Nimble Flutter";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Oeksound Nimble Flutter";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Oeksound Nimble Flutter") },
        RegValue { name: "Version",     value: String::from("v1.3.19") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Oeksound Nimble Flutter") },
        RegValue { name: "Publisher",       value: String::from("Oeksound") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Oeksound Nimble Flutter\\uninstall.exe\"") },
    ]
}
