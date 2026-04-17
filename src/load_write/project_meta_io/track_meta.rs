use crate::load_write::{AsBytes, FromBytes, project_meta_io::StoredRegionMeta};
use eframe::egui;
use knodiq_engine::track::RegionID;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

pub struct StoredTrackMeta {
    pub name: String,
    pub color: egui::Color32,
    pub region_metas: HashMap<RegionID, StoredRegionMeta>,
}

impl StoredTrackMeta {
    pub fn from_track_meta(track_meta: &crate::metadata::TrackMeta) -> Self {
        let region_metas = track_meta
            .regions
            .iter()
            .map(|(region_id, region_meta)| {
                (*region_id, StoredRegionMeta::from_region_meta(region_meta))
            })
            .collect();

        Self {
            name: track_meta.name.clone(),
            color: track_meta.color,
            region_metas,
        }
    }
}

impl AsBytes for StoredTrackMeta {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the color of the track to the bytes vector
        bytes.push(self.color.r());
        bytes.push(self.color.g());
        bytes.push(self.color.b());
        bytes.push(self.color.a());

        // Write the length of the name, and the name itself
        let name_bytes = self.name.as_bytes();
        bytes.extend((name_bytes.len() as u64).to_le_bytes());
        bytes.extend(name_bytes);

        // Write the region metadatas
        let mut region_metas_bytes = Vec::new();
        for (region_id, region_meta) in &self.region_metas {
            // Get the length of the region metadata
            let mut region_meta_bytes = Vec::new();
            region_meta.as_bytes(&mut region_meta_bytes);

            // Write the region ID, the length of the region metadata, and the region metadata itself
            region_metas_bytes.extend(region_id.0.to_le_bytes());
            region_metas_bytes.extend((region_meta_bytes.len() as u64).to_le_bytes());
            region_metas_bytes.extend(region_meta_bytes);
        }

        // Write the length of the region metadatas, and the region metadatas themselves
        bytes.extend((region_metas_bytes.len() as u64).to_le_bytes());
        bytes.extend(region_metas_bytes);
    }
}

impl FromBytes for StoredTrackMeta {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Read the first four bytes to get the color of the track
        let mut color_bytes = [0u8; 4];
        cursor.read_exact(&mut color_bytes)?;
        let color = egui::Color32::from_rgba_premultiplied(
            color_bytes[0],
            color_bytes[1],
            color_bytes[2],
            color_bytes[3],
        );

        // Read the length of the name
        let mut name_len_bytes = [0u8; 8];
        cursor.read_exact(&mut name_len_bytes)?;
        let name_len = u64::from_le_bytes(name_len_bytes) as usize;
        // Read the name itself
        let mut name_bytes = vec![0u8; name_len];
        cursor.read_exact(&mut name_bytes)?;
        let name = String::from_utf8(name_bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // Read the length of the region metadatas
        let mut region_metas_len_bytes = [0u8; 8];
        cursor.read_exact(&mut region_metas_len_bytes)?;
        let region_metas_len = u64::from_le_bytes(region_metas_len_bytes);

        // Read the region metadatas
        let mut region_metas_bytes = vec![0u8; region_metas_len as usize];
        cursor.read_exact(&mut region_metas_bytes)?;

        let mut region_metas = HashMap::new();
        let mut region_metas_cursor = Cursor::new(region_metas_bytes);
        while region_metas_cursor.position() < region_metas_len {
            // Read the region ID
            let mut region_id_bytes = [0u8; 8];
            region_metas_cursor.read_exact(&mut region_id_bytes)?;
            let region_id = RegionID(u64::from_le_bytes(region_id_bytes) as usize);

            // Read the length of the region metadata
            let mut region_meta_len_bytes = [0u8; 8];
            region_metas_cursor.read_exact(&mut region_meta_len_bytes)?;
            let region_meta_len = u64::from_le_bytes(region_meta_len_bytes) as usize;

            // Read the region metadata itself
            let mut region_meta_bytes = vec![0u8; region_meta_len];
            region_metas_cursor.read_exact(&mut region_meta_bytes)?;
            let region_meta = StoredRegionMeta::from_bytes(&region_meta_bytes)?;

            // Inser the region metadata into the hashmap
            region_metas.insert(region_id, region_meta);
        }

        // Construct the StoredTrackMeta
        let track_meta = Self {
            name,
            color,
            region_metas,
        };

        Ok(track_meta)
    }
}
