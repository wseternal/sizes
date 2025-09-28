use std::{path::PathBuf, sync::OnceLock};

use serde::{Deserialize, Serialize};

use crate::kvstore::KvStore;
use crate::{
    db::TABLE_CONF,
    home_dir,
};

static CONF_KEY_WATCHES: &str = "watches";

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub struct WatchDirectoryConfiguration {
    pub refresh_interval: String,
    pub label: String,
    pub path: String,
}

pub fn app_db_path() -> &'static PathBuf {
    static GLOBAL_CONF_PATH: OnceLock<PathBuf> = OnceLock::new();
    GLOBAL_CONF_PATH.get_or_init(|| {
        let mut path = home_dir();
        path.push(".config/sizes/app.db");
        path
    })
}

pub fn list_watch(db: &impl KvStore) -> Vec<WatchDirectoryConfiguration> {
    db.get_as(TABLE_CONF, CONF_KEY_WATCHES).unwrap_or(vec![])
}

pub fn add_watch(
    db: &impl KvStore,
    to_add: &WatchDirectoryConfiguration,
) -> crate::Result<()> {
    let mut watches = list_watch(db);
    if watches.contains(to_add) {
        return Ok(());
    }

    let mut updated = false;
    for elem in &mut watches {
        if elem.path != to_add.path {
            continue;
        }
        elem.label = to_add.label.clone();
        elem.refresh_interval = to_add.refresh_interval.clone();
        updated = true;
    }
    if !updated {
        watches.push(to_add.clone());
    }
    db.set_json(TABLE_CONF, CONF_KEY_WATCHES, &watches)
}

pub fn remove_watch(
    db: &impl KvStore,
    to_remove: &WatchDirectoryConfiguration,
) -> crate::Result<()> {
    let mut watches = list_watch(db);
    let orig_size = watches.len();
    watches.retain(|elem| elem.path != to_remove.path);
    if orig_size != watches.len() {
        db.set_json(TABLE_CONF, CONF_KEY_WATCHES, &watches)
    } else {
        Ok(())
    }
}
