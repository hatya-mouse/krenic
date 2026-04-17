use crate::load_write::{FromBytes, LoadProjResult};
use eframe::egui;
use knodiq_engine::mixer::{Project, TrackID};
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

impl FromBytes for LoadProjResult {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the track metadata
        let mut tracks_meta_len_bytes = [0u8; 8];
        cursor.read_exact(&mut tracks_meta_len_bytes)?;
        let tracks_meta_len = u64::from_le_bytes(tracks_meta_len_bytes);
        let mut tracks_meta_bytes = vec![0u8; tracks_meta_len as usize];
        cursor.read_exact(&mut tracks_meta_bytes)?;

        // Parse the track metadata
        let mut track_names = HashMap::new();
        let mut track_colors = HashMap::new();
        let mut tracks_meta_cursor = Cursor::new(tracks_meta_bytes);
        while tracks_meta_cursor.position() < tracks_meta_len {
            // Read the track ID
            let mut track_id_bytes = [0u8; 8];
            tracks_meta_cursor.read_exact(&mut track_id_bytes)?;
            let track_id = TrackID(u64::from_le_bytes(track_id_bytes) as usize);

            // Read the color of the track (rgba)
            let mut color_bytes = [0u8; 4];
            tracks_meta_cursor.read_exact(&mut color_bytes)?;
            let color = egui::Color32::from_rgba_unmultiplied(
                color_bytes[0],
                color_bytes[1],
                color_bytes[2],
                color_bytes[3],
            );

            // Read the name of the track
            let mut name_len_bytes = [0u8; 8];
            tracks_meta_cursor.read_exact(&mut name_len_bytes)?;
            let name_len = u64::from_le_bytes(name_len_bytes) as usize;
            let mut name_bytes = vec![0u8; name_len];
            tracks_meta_cursor.read_exact(&mut name_bytes)?;
            let name = String::from_utf8(name_bytes)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

            // Add the track to the metadata maps
            track_names.insert(track_id, name);
            track_colors.insert(track_id, color);
        }

        // Read the rest of the file and parse the project
        let mut project_bytes = Vec::new();
        cursor.read_to_end(&mut project_bytes)?;
        let project = Project::from_bytes(&project_bytes)?;

        // Construct the new LoadProjResult
        let result = LoadProjResult {
            project,
            track_names,
            track_colors,
        };

        Ok(result)
    }
}
