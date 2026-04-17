mod region_meta;
mod track_meta;

pub use region_meta::StoredRegionMeta;
pub use track_meta::StoredTrackMeta;

use crate::{
    load_write::{AsBytes, FromBytes},
    metadata::ProjectMeta,
};
use knodiq_engine::mixer::TrackID;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

pub struct StoredProjMeta {
    pub track_metas: HashMap<TrackID, StoredTrackMeta>,
}

impl StoredProjMeta {
    pub fn from_proj_meta(proj_meta: &ProjectMeta) -> Self {
        let track_metas = proj_meta
            .tracks
            .iter()
            .map(|(track_id, track_meta)| (*track_id, StoredTrackMeta::from_track_meta(track_meta)))
            .collect();

        Self { track_metas }
    }
}

impl AsBytes for StoredProjMeta {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the track metadatas
        let mut track_metas_bytes = Vec::new();
        for (track_id, track_meta) in &self.track_metas {
            // Get the length of the track metadata in bytes
            let mut track_meta_bytes = Vec::new();
            track_meta.as_bytes(&mut track_meta_bytes);

            // Write the track ID and the length of the track metadata
            track_metas_bytes.extend(&(track_id.0 as u64).to_le_bytes());
            track_metas_bytes.extend(&(track_meta_bytes.len() as u64).to_le_bytes());
            track_metas_bytes.extend(track_meta_bytes);
        }

        // Write the length of all track metadatas and the track metadatas
        bytes.extend(&(track_metas_bytes.len() as u64).to_le_bytes());
        bytes.extend(track_metas_bytes);
    }
}

impl FromBytes for StoredProjMeta {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the length of the track metadatas
        let mut track_metas_len_bytes = [0u8; 8];
        cursor.read_exact(&mut track_metas_len_bytes)?;
        let track_metas_len = u64::from_le_bytes(track_metas_len_bytes);

        // Read the track metadatas
        let mut track_metas_bytes = vec![0u8; track_metas_len as usize];
        cursor.read_exact(&mut track_metas_bytes)?;

        // Loop over the track metadatas and read each track metadata
        let mut track_metas = HashMap::new();
        let mut track_metas_cursor = Cursor::new(track_metas_bytes);
        while track_metas_cursor.position() < track_metas_len {
            // Read the track ID and the length of the track metadata
            let mut track_id_bytes = [0u8; 8];
            track_metas_cursor.read_exact(&mut track_id_bytes)?;
            let track_id = TrackID(u64::from_le_bytes(track_id_bytes) as usize);

            let mut track_meta_len_bytes = [0u8; 8];
            track_metas_cursor.read_exact(&mut track_meta_len_bytes)?;
            let track_meta_len = u64::from_le_bytes(track_meta_len_bytes) as usize;

            // Read the track metadata
            let mut track_meta_bytes = vec![0u8; track_meta_len];
            track_metas_cursor.read_exact(&mut track_meta_bytes)?;

            // Convert the track metadata bytes to a StoredTrackMeta and insert it into the HashMap
            let track_meta = StoredTrackMeta::from_bytes(&track_meta_bytes)?;
            track_metas.insert(track_id, track_meta);
        }

        // Construct the StoredProjMeta
        let proj_meta = StoredProjMeta { track_metas };

        Ok(proj_meta)
    }
}
