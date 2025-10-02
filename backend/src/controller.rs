use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{get, post, FromFormField, Request, Response, State};
use serde::Serialize;
use sizes::cmd::Command;
use sizes::conf;
use sizes::conf::WatchDirectoryConfiguration;
use sizes::db::dirstat::{get_dir_stat_recursive, DirStat};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use crate::AppState;
use sizes::db::scanresult::{self, get_last_dir_scan_result};
use sizes::scan::{DirScanOverview, DirScanResult};
#[derive(Debug, PartialEq, FromFormField)]
pub(crate) enum Orderby {
    Block,
    FileNum,
    SubDirNum,
}

pub(crate) struct ResultResponder<T>(pub Result<T, String>);

impl<T> ResultResponder<T> {
    fn err(text: String) -> ResultResponder<T> {
        ResultResponder(Err(text))
    }
}

impl<T> From<T> for ResultResponder<T> {
    fn from(value: T) -> ResultResponder<T> {
        ResultResponder(Ok(value))
    }
}

impl<'r, T> Responder<'r, 'static> for ResultResponder<T>
where
    T: Debug + Serialize,
{
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        if self.0.is_err() {
            return Response::build_from(
                (Status::BadRequest, self.0.unwrap_err()).respond_to(req)?,
            )
            .raw_header("Access-Control-Allow-Origin", "*")
            .ok();
        }
        Response::build_from(Json(self.0.unwrap()).respond_to(req)?)
            .raw_header("Access-Control-Allow-Origin", "*")
            .ok()
    }
}

#[get("/api/scan?<path>")]
pub async fn scan_dir(app_state: &State<AppState>, path: &str) -> ResultResponder<String> {
    let cmd = Command::ScanDir(PathBuf::from(path));
    if let Err(err) = app_state.client.task_manager.send(cmd).await {
        return ResultResponder::err(err.to_string());
    }
    ResultResponder::from(format!("queued scan dir command for {} successfully", path))
}

#[get("/api/progress")]
pub async fn scan_dir_progress(
    app_state: &State<AppState>
) -> ResultResponder<HashMap<PathBuf, DirScanResult>> {
    let ongoings = app_state
        .client
        .task_manager
        .scan_progress()
        .await;

        ResultResponder::from(ongoings)
}

#[get("/api/result?<path>")]
pub fn scan_dir_results(
    app_state: &State<AppState>,
    path: &str,
) -> ResultResponder<Vec<DirScanResult>> {
    let result = scanresult::get_dir_scan_result(app_state.client.db, Path::new(path), 10)
        .unwrap();
    ResultResponder::from(result)
}

#[post("/api/results", data = "<paths>")]
pub fn dir_results(
    app_state: &State<AppState>,
    paths: Json<Vec<&str>>,
) -> ResultResponder<HashMap<String, DirScanResult>> {
    let mut m = HashMap::<String, DirScanResult>::new();
    for path in paths.iter() {
        let res = get_last_dir_scan_result(app_state.client.db, &Path::new(path));
        let v = res.unwrap_or_else(|| DirScanResult::new());
        m.insert(path.to_string(), v);
    }
    ResultResponder::from(m)
}

#[get("/api/watches")]
pub fn list_watch_dir(
    app_state: &State<AppState>,
) -> ResultResponder<Vec<WatchDirectoryConfiguration>> {
    let watches = conf::list_watch(app_state.client.db);
    ResultResponder::from(watches)
}

#[post("/api/watches/add", data = "<watch>")]
pub fn add_watch_dir(
    app_state: &State<AppState>,
    watch: Json<WatchDirectoryConfiguration>,
) -> ResultResponder<Vec<WatchDirectoryConfiguration>> {
    let db = app_state.client.db;
    conf::add_watch(db, &watch.0).unwrap();
    let watches = conf::list_watch(db);
    ResultResponder::from(watches)
}

#[post("/api/watches/delete", data = "<watch>")]
pub fn remove_watch_dir(
    app_state: &State<AppState>,
    watch: Json<WatchDirectoryConfiguration>,
) -> ResultResponder<Vec<WatchDirectoryConfiguration>> {
    let db = app_state.client.db;
    conf::remove_watch(db, &watch.0).unwrap();
    let watches = conf::list_watch(db);
    ResultResponder::from(watches)
}

#[get("/api/largest?<min>&<limit>&<offset>")]
pub fn get_largest(
    app_state: &State<AppState>,
    min: Option<u64>,
    limit: Option<u32>,
    offset: Option<u32>
) -> ResultResponder<Vec<DirStat>> {
    let db = app_state.client.db;
    let dirs = scanresult::get_largest_dirs(
        db,
        min.unwrap_or(0),
        limit.unwrap_or(10),
        offset.unwrap_or(0)
    ).unwrap();
    ResultResponder::from(dirs)
}

#[get("/api/stat?<path>")]
pub fn get_dir_stat(app_state: &State<AppState>, path: &str) -> ResultResponder<DirScanOverview> {
    get_dir_stat_recursive(app_state.client.db, Path::new(path))
        .unwrap_or(DirScanOverview::new())
        .into()
}
