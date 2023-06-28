// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(unused_variables)]

use tauri::{AppHandle};

mod app_modules;
use app_modules::{
    export_pdf::{
        runexport
    },
    calculation::{
        run_calculation
    }
};

#[tauri::command]
async fn export(handle: tauri::AppHandle) {
  let export_window = tauri::WindowBuilder::new(
    &handle,
    "Export", /* the unique window label */
    tauri::WindowUrl::App("export.html".into())
  ).build()
  .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![export, runexport, run_calculation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
