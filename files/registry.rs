//! Windows registry read helpers for Surfcam 2024.

pub const APP_KEY: &str = "Software\\Surfcam\\Surfcam 2024";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Surfcam 2024";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Surfcam 2024") },
        RegValue { name: "Version",     value: String::from("v4.7.7") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Surfcam 2024") },
        RegValue { name: "Publisher",       value: String::from("Surfcam") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Surfcam 2024\\uninstall.exe\"") },
    ]
}
