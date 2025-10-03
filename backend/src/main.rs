// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use std::env;
use std::error::Error;
use tauri::{App, AppHandle};

use crate::controller::mount_routes;
use sizes::Client;

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
        let mut rocket_builder = rocket::build()
            .manage(AppState { client, handle })
            .attach(AdHoc::on_response("cors", |_, res| {
                Box::pin(async move {
                    res.set_raw_header("Access-Control-Allow-Origin", "*");
                })
            }));

        rocket_builder = mount_routes(rocket_builder);

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
