use crate::AppState;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{get, post, routes, Build, FromFormField, Request, Response, Rocket, State};
use rocket_okapi::{openapi, openapi_get_routes};
use serde::Serialize;
use sizes::cmd::Command;
use sizes::conf;
use sizes::conf::WatchDirectoryConfiguration;
use sizes::db::dirstat::{get_dir_stat_recursive, DirStat};
use sizes::db::scanresult::{self, get_last_dir_scan_result};
use sizes::scan::{DirScanOverview, DirScanResult};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[openapi(tag = "scan")]
#[get("/api/scan?<path>")]
pub async fn scan_dir(app_state: &State<AppState>, path: &str) -> String {
    let cmd = Command::ScanDir(PathBuf::from(path));
    if let Err(err) = app_state.client.task_manager.send(cmd).await {
        return err.to_string();
    }
    format!("queued scan dir command for {} successfully", path)
}

#[openapi(tag = "scan")]
#[get("/api/progress")]
pub async fn scan_dir_progress(
    app_state: &State<AppState>,
) -> Json<HashMap<PathBuf, DirScanResult>> {
    Json(app_state.client.task_manager.scan_progress().await)
}

#[openapi(tag = "WathDirs")]
#[get("/api/watches")]
pub fn list_watch_dir(app_state: &State<AppState>) -> Json<Vec<WatchDirectoryConfiguration>> {
    let watches = conf::list_watch(app_state.client.db);
    Json(watches)
}

#[openapi(tag = "WathDirs")]
#[post("/api/watches/add", data = "<watch>")]
pub fn add_watch_dir(
    app_state: &State<AppState>,
    watch: Json<WatchDirectoryConfiguration>,
) -> Json<Vec<WatchDirectoryConfiguration>> {
    let db = app_state.client.db;
    conf::add_watch(db, &watch.0).unwrap();
    let watches = conf::list_watch(db);
    Json(watches)
}

#[openapi(tag = "WathDirs")]
#[post("/api/watches/delete", data = "<watch>")]
pub fn remove_watch_dir(
    app_state: &State<AppState>,
    watch: Json<WatchDirectoryConfiguration>,
) -> Json<Vec<WatchDirectoryConfiguration>> {
    let db = app_state.client.db;
    conf::remove_watch(db, &watch.0).unwrap();
    let watches = conf::list_watch(db);
    Json(watches)
}

#[openapi(tag = "result")]
#[get("/api/result?<path>")]
pub fn scan_dir_results(app_state: &State<AppState>, path: &str) -> Json<Vec<DirScanResult>> {
    let result = scanresult::get_dir_scan_result(app_state.client.db, Path::new(path), 10).unwrap();
    Json(result)
}

#[openapi(tag = "result")]
#[post("/api/results", data = "<paths>")]
pub fn dir_results(
    app_state: &State<AppState>,
    paths: Json<Vec<&str>>,
) -> Json<HashMap<String, DirScanResult>> {
    let mut m = HashMap::<String, DirScanResult>::new();
    for path in paths.iter() {
        let res = get_last_dir_scan_result(app_state.client.db, &Path::new(path));
        let v = res.unwrap_or_else(|| DirScanResult::new());
        m.insert(path.to_string(), v);
    }
    Json(m)
}

#[openapi(tag = "result")]
#[get("/api/largest?<min>&<limit>&<offset>")]
pub fn get_largest(
    app_state: &State<AppState>,
    min: Option<u64>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Json<Vec<DirStat>> {
    let db = app_state.client.db;
    let dirs = scanresult::get_largest_dirs(
        db,
        min.unwrap_or(0),
        limit.unwrap_or(10),
        offset.unwrap_or(0),
    )
    .unwrap();
    Json(dirs)
}

#[openapi(tag = "result")]
#[get("/api/stat?<path>")]
pub fn get_dir_stat(app_state: &State<AppState>, path: &str) -> Json<DirScanOverview> {
    get_dir_stat_recursive(app_state.client.db, Path::new(path))
        .unwrap_or(DirScanOverview::new())
        .into()
}

pub fn mount_routes(builder: Rocket<Build>) -> Rocket<Build> {
    builder.mount(
            "/sizes",
            openapi_get_routes![
                scan_dir,
                scan_dir_progress,
                scan_dir_results,
                list_watch_dir,
                add_watch_dir,
                remove_watch_dir,
                dir_results,
                get_largest,
                get_dir_stat
            ],
        )
        .mount(
            "/sizes/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/sizes/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}
