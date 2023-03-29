// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn py_add(add_one: String, add_two: String) -> String {
    println!("Rust: {} + {}", add_one, add_two);
    let add_one = &add_one[..];
    let add_two = &add_two[..];
    use tauri::api::process::{Command, CommandEvent};
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
    use tauri::api::process::{Command, CommandEvent};
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
        .invoke_handler(tauri::generate_handler![py_add, py_ver])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
