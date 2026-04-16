use crate::load_write::{AsBytes, FromBytes, error::LoadError, safe_read};
use knodiq_engine::{
    data_types::{AudioContext, Beats},
    mixer::{Project, TempoMap, TrackID},
    track::Track,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read, Write},
    path::Path,
};

/// Saves the given project to the given path. Returns an error if the file cannot be created or written to.
pub(crate) fn save_project(path: &Path, project: &Project) -> std::io::Result<()> {
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

    // Write the project
    let mut project_bytes = Vec::new();
    project.as_bytes(&mut project_bytes);

    file.write_all(&project_bytes)?;
    file.flush()?;

    Ok(())
}

/// Loads a project from the given path. Returns an error if the file is not a Knodiq Project file or if the file is corrupted.
pub(crate) fn load_project(path: &Path) -> Result<Project, LoadError> {
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
    let project = Project::from_bytes(&project_bytes).map_err(LoadError::FileCorrupted)?;

    Ok(project)
}

impl AsBytes for Project {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the audio configurations
        self.audio_ctx.as_bytes(bytes);

        // Write the range of the song
        bytes.extend(&self.range_start.0.to_le_bytes());
        bytes.extend(&self.range_duration.0.to_le_bytes());

        // Write the tempo map and its size
        let mut tempo_map_bytes = Vec::new();
        self.tempo_map.as_bytes(&mut tempo_map_bytes);
        bytes.extend(&(tempo_map_bytes.len() as u64).to_le_bytes());
        bytes.extend(tempo_map_bytes);

        // Write the tracks to the temporary buffer
        let mut tracks_bytes = Vec::new();
        for (track_id, track) in &self.tracks {
            // Get the size of the track data
            let mut track_bytes = Vec::new();
            track.as_bytes(&mut track_bytes);

            // Write the track ID, the size of the track and its contents
            tracks_bytes.extend(&(track_id.0 as u64).to_le_bytes());
            tracks_bytes.extend(&(track_bytes.len() as u64).to_le_bytes());
            tracks_bytes.extend(track_bytes);
        }

        // Write the size of all tracks and the tracks themselves
        bytes.extend(&(tracks_bytes.len() as u64).to_le_bytes());
        bytes.extend(tracks_bytes);
    }
}

impl FromBytes for Project {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the audio configurations from the bytes
        let mut audio_ctx_bytes = [0u8; 32];
        cursor.read_exact(&mut audio_ctx_bytes)?;
        let audio_ctx = AudioContext::from_bytes(&audio_ctx_bytes)?;

        // Read the range of the song
        let mut range_start_bytes = [0u8; 8];
        let mut range_duration_bytes = [0u8; 8];
        cursor.read_exact(&mut range_start_bytes)?;
        cursor.read_exact(&mut range_duration_bytes)?;
        let range_start = Beats(f64::from_le_bytes(range_start_bytes));
        let range_duration = Beats(f64::from_le_bytes(range_duration_bytes));

        // Read the tempo map from the bytes
        let mut tempo_map_len_bytes = [0u8; 8];
        cursor.read_exact(&mut tempo_map_len_bytes)?;
        let tempo_map_len = u64::from_le_bytes(tempo_map_len_bytes) as usize;
        let tempo_map_bytes = safe_read(&mut cursor, tempo_map_len)?;
        let tempo_map = TempoMap::from_bytes(&tempo_map_bytes)?;

        // Read the length of all tracks
        let mut tracks_len_bytes = [0u8; 8];
        cursor.read_exact(&mut tracks_len_bytes)?;
        let tracks_len = u64::from_le_bytes(tracks_len_bytes);

        // Read the tracks data
        let tracks_bytes = safe_read(&mut cursor, tracks_len as usize)?;

        let mut tracks = HashMap::new();
        let mut tracks_cursor = Cursor::new(tracks_bytes.as_slice());
        while tracks_cursor.position() < tracks_len {
            // Get the ID and the length of the track contents
            let mut track_id_bytes = [0u8; 8];
            let mut data_len_bytes = [0u8; 8];
            tracks_cursor.read_exact(&mut track_id_bytes)?;
            tracks_cursor.read_exact(&mut data_len_bytes)?;
            let track_id = TrackID(u64::from_le_bytes(track_id_bytes) as usize);
            let data_len = u64::from_le_bytes(data_len_bytes) as usize;

            // Parse the track contents
            let track_data_bytes = safe_read(&mut tracks_cursor, data_len)?;
            let track = <Box<dyn Track>>::from_bytes(&track_data_bytes)?;

            tracks.insert(track_id, track);
        }

        // Construct the new project
        let mut project =
            Project::with_tempo_map(audio_ctx, tempo_map, range_start, range_duration);
        project.tracks = tracks;

        Ok(project)
    }
}
