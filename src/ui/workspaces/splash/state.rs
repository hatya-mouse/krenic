use crate::spawn_background_init;
use crate::storage::app_state::load_recent_projects;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub(crate) struct RecentProjData {
    pub name: String,
    pub path_str: String,
    pub path: PathBuf,
}

pub(super) struct SplashUiState {
    /// Recently opened projects.
    pub recent_projects: Arc<Mutex<Vec<RecentProjData>>>,
}

impl Default for SplashUiState {
    fn default() -> Self {
        Self {
            recent_projects: spawn_background_init!({ load_recent_projects() }),
        }
    }
}
