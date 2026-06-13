use crate::{consts::RECENT_PROJCETS_PATH, ui::workspaces::splash::state::RecentProjData};
use std::{fs::File, io::Read, path::PathBuf};

pub(crate) fn load_recent_projects() -> Vec<RecentProjData> {
    let full_path = dirs::data_dir()
        .expect("Could not get data dir")
        .join(RECENT_PROJCETS_PATH);
    let Ok(mut file) = File::open(&full_path) else {
        return Vec::new();
    };

    // Load the JSON string and parse it
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).ok();

    let paths: Vec<PathBuf> = match serde_json::from_str(&json_string) {
        Ok(paths) => paths,
        Err(_) => {
            return Vec::new();
        }
    };

    // Create RecentProjData from paths
    paths
        .iter()
        .map(|path| RecentProjData {
            name: path
                .file_name()
                .map(|os_str| os_str.to_string_lossy().into_owned())
                .unwrap_or_else(|| "Untitled".to_string()),
            path_str: path.to_string_lossy().to_string(),
            path: path.clone(),
        })
        .collect()
}

pub(crate) fn save_recent_projects(recent_projects: &[RecentProjData]) {
    let full_path = dirs::data_dir()
        .expect("Could not get data dir")
        .join(RECENT_PROJCETS_PATH);
    std::fs::create_dir_all(&full_path).unwrap();

    // Write the JSON string to the path
    let paths: Vec<PathBuf> = recent_projects
        .iter()
        .map(|project| project.path.clone())
        .collect();
    let Ok(json_string) = serde_json::to_string(&paths) else {
        return;
    };
    std::fs::write(&full_path, json_string).ok();
}
