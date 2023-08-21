#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use tauri::State;

// Define a structure for a Mutex<i32> counter
struct Counter(Mutex<i32>);

// Increase or decrease the counter value and return the new value
#[tauri::command]
fn counter(count_val: i32, counter: State<Counter>) -> i32 {
    let mut ct = counter.0.lock().unwrap();
    *ct += count_val;
    *ct
}

// Display the received value in a message box
use std::path::PathBuf;
#[tauri::command]

fn get_exec_path() -> Result<PathBuf, String> {

match std::env::current_exe() {

Ok(path) => return Ok(path),

Err(error) => return Err(format!("{error}")),

}

}


fn main() {
    tauri::Builder::default()
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![
            counter,
            get_exec_path
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
