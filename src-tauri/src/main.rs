// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::{Command, CommandEvent};
use tauri::Manager;
use window_shadows::set_shadow;
use window_vibrancy::{apply_blur, apply_mica, apply_vibrancy, NSVisualEffectMaterial};

#[tauri::command]
async fn py_ingest() -> String {
    println!("Rust: Ingest Data");
    let (mut rx, mut child) = Command::new("backend")
        .args(["ingest"])
        .spawn()
        .expect("Failed to spawn backend");
    let mut result = String::new();
    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line) = event {
            result = line.clone();
            println!("Python: {}", line);
            break; // Only lets python send one line. This is not ideal
        }
    }
    /*
    child
        .kill()
        .expect("Failed to kill child process. May already be dead?");
     */
    result
}

#[tauri::command]
async fn py_add(add_one: String, add_two: String) -> String {
    println!("Rust: {} + {}", add_one, add_two);
    let add_one = &add_one[..];
    let add_two = &add_two[..];
    let (mut rx, mut child) = Command::new("backend")
        .args(["addition", &add_one, &add_two])
        .spawn()
        .expect("Failed to spawn backend");
    let mut result = String::new();
    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line) = event {
            result = line.clone();
            println!("Python: {}", line);
            break; // Only lets python send one line. This is not ideal
        }
    }
    /*
    child
        .kill()
        .expect("Failed to kill child process. May already be dead?");
     */
    result
}

#[tauri::command]
async fn py_ver() -> String {
    println!("Rust: Get Version");
    let (mut rx, mut child) = Command::new("backend")
        .args(["version"])
        .spawn()
        .expect("Failed to spawn backend");
    let mut result = String::new();
    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line) = event {
            result = line.clone();
            println!("Python: {}", line);
            break; // Only lets python send one line. This is not ideal
        }
    }
    /*
    child
        .kill()
        .expect("Failed to kill child process. May already be dead?");
     */
    result
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_mica(&window)
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![py_add, py_ver, py_ingest])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
