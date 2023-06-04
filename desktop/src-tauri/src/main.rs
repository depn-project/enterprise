// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::{fs::File, io::Write};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_config(config: &str) -> Result<String, String> {
    let config_file = File::create("/opt/homebrew/etc/wireguard/depn.conf");

    match config_file {
        Ok(mut f) => match f.write_all(config.as_bytes()) {
            Ok(_) => Ok("Ok".into()),
            Err(_) => Err("Error".into()),
        },
        Err(_) => Err("Error".into()),
    }
}

#[tauri::command]
fn connect() -> Result<String, String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("do shell script \"sudo wg-quick up depn\" with administrator privileges")
        .output()
        .expect("Can't connect to WireGuard");

    if output.status.success() {
        Ok("Ok".into())
    } else {
        Err("Error".into())
    }
}

#[tauri::command]
fn disconnect() -> Result<String, String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("do shell script \"sudo wg-quick down depn\" with administrator privileges")
        .output()
        .expect("Can't disconnect from WireGuard");

    if output.status.success() {
        Ok("Ok".into())
    } else {
        Err("Error".into())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_config,
            connect,
            disconnect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
