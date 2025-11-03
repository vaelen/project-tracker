// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Tauri commands (IPC functions callable from frontend)
#[tauri::command]
async fn list_projects() -> Result<Vec<String>, String> {
    // TODO: Implement project listing
    Ok(vec![])
}

#[tauri::command]
async fn create_project(name: String) -> Result<String, String> {
    // TODO: Implement project creation
    Ok(format!("Created project: {}", name))
}

#[tauri::command]
async fn chat_with_claude(message: String) -> Result<String, String> {
    // TODO: Implement Claude chat
    Ok(format!("Claude response to: {}", message))
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_projects,
            create_project,
            chat_with_claude,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
