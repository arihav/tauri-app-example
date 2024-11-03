// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod windows_service;


fn main() {
    // for windows-service
    windows_service::run();
    // for dev mode
    // tauri_app_lib::run();
}
