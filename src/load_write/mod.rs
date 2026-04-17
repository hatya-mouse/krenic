mod audio_ctx_io;
mod error;
mod graph_io;
mod load_proj_res;
mod node_io;
mod project_io;
mod tempo_map_io;
mod track_io;
mod traits;

use crate::{load_write::error::LoadError, metadata::ProjectMeta};
pub(crate) use traits::{AsBytes, FromBytes};

use crate::load_write::traits::safe_read;
use eframe::egui;
use knodiq_engine::mixer::{Project, TrackID};
use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read, Write},
    path::Path,
};

pub(crate) struct LoadProjResult {
    pub(crate) project: Project,
    pub(crate) track_names: HashMap<TrackID, String>,
    pub(crate) track_colors: HashMap<TrackID, egui::Color32>,
}

/// Saves the given project to the given path. Returns an error if the file cannot be created or written to.
pub(crate) fn save_project(
    path: &Path,
    project: &Project,
    project_meta: &ProjectMeta,
) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // Write the project data to the file
    // First write "KNODIQ" to check if the file is a Knodiq Project file
    file.write_all("KNODIQ".as_bytes())?;

    // Then write the version of Knodiq
    let major_ver: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor_ver: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch_ver: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    file.write_all(&major_ver.to_le_bytes())?;
    file.write_all(&minor_ver.to_le_bytes())?;
    file.write_all(&patch_ver.to_le_bytes())?;

    // Write the track metadata
    let mut tracks_meta_bytes = Vec::new();
    for (id, track_meta) in &project_meta.tracks {
        // Write the name of the track
        let track_name_bytes = track_meta.name.as_bytes();

        // Write the size of the track metadata and the metadata itself
        tracks_meta_bytes.extend((id.0 as u64).to_le_bytes());
        tracks_meta_bytes.push(track_meta.color.r());
        tracks_meta_bytes.push(track_meta.color.g());
        tracks_meta_bytes.push(track_meta.color.b());
        tracks_meta_bytes.push(track_meta.color.a());
        tracks_meta_bytes.extend((track_name_bytes.len() as u64).to_le_bytes());
        tracks_meta_bytes.extend(track_name_bytes);
    }
    // Write the total size of the track metadata
    file.write_all(&(tracks_meta_bytes.len() as u64).to_le_bytes())?;
    file.write_all(&tracks_meta_bytes)?;

    // Write the project
    let mut project_bytes = Vec::new();
    project.as_bytes(&mut project_bytes);

    file.write_all(&project_bytes)?;
    file.flush()?;

    Ok(())
}

/// Loads a project from the given path. Returns an error if the file is not a Knodiq Project file or if the file is corrupted.
pub(crate) fn load_project(path: &Path) -> Result<LoadProjResult, LoadError> {
    // Load the file from the path
    let mut file = File::open(path).map_err(LoadError::IoError)?;

    // Read the first 6 bytes to check if it's a Knodiq Project file
    let mut header_bytes = [0u8; 6];
    file.read_exact(&mut header_bytes)
        .map_err(LoadError::IoError)?;

    if &header_bytes != b"KNODIQ" {
        return Err(LoadError::NotAProjectFile);
    }

    // Read the next 12 bytes to get the version of Knodiq that created the project
    let mut major_bytes = [0u8; 4];
    let mut minor_bytes = [0u8; 4];
    let mut patch_bytes = [0u8; 4];
    file.read_exact(&mut major_bytes)
        .map_err(LoadError::IoError)?;
    file.read_exact(&mut minor_bytes)
        .map_err(LoadError::IoError)?;
    file.read_exact(&mut patch_bytes)
        .map_err(LoadError::IoError)?;
    let file_major_ver = u32::from_le_bytes(major_bytes);
    let file_minor_ver = u32::from_le_bytes(minor_bytes);
    let file_patch_ver = u32::from_le_bytes(patch_bytes);

    // Read the rest of the file and parse the project
    let mut project_bytes = Vec::new();
    file.read_to_end(&mut project_bytes)
        .map_err(LoadError::IoError)?;
    let result = LoadProjResult::from_bytes(&project_bytes).map_err(LoadError::FileCorrupted)?;

    Ok(result)
}
