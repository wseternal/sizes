// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rocket::routes;
use std::env;
use std::error::Error;
use rocket::fs::FileServer;
use tauri::{App, AppHandle};

use crate::controller::{
    add_watch_dir, dir_results, get_dir_stat, get_largest, list_watch_dir, remove_watch_dir,
    scan_dir, scan_dir_progress, scan_dir_results
};
use sizes::Client;

mod cmds;
mod controller;

struct AppState {
    client: Client,
    handle: AppHandle,
}

fn main() {
    tauri::Builder::default()
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        let client = sizes::init().await;
        let rocket_builder = rocket::build()
            .manage(AppState { client, handle })
            .mount(
                "/sizes",
                routes![
                    scan_dir,
                    scan_dir_progress,
                    scan_dir_results,
                    list_watch_dir,
                    add_watch_dir,
                    remove_watch_dir,
                    dir_results,
                    get_largest,
                    get_dir_stat
                ]
            );

        if tauri::is_dev() {
            rocket_builder.mount("/", FileServer::from("../frontend/composeApp/build/dist/wasmJs/developmentExecutable/"))
                .launch()
                .await
                .unwrap();
        } else {
            rocket_builder
                .launch()
                .await
                .unwrap();
        }
    });
    println!("setup finished");
    Ok(())
}
