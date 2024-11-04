use std::thread;
use std::time::Duration;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_localhost::Builder::new(1420).build())
        .setup(move |app|{

            let handle = app.handle().clone();
            tauri::async_runtime::spawn_blocking(move || {
                thread::sleep(Duration::from_secs(10)); // Wait for 10 seconds
                handle.exit(0);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .any_thread()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
