mod region_meta;
mod track_meta;

pub(crate) use region_meta::RegionMeta;
pub(crate) use track_meta::TrackMeta;

use crate::load_write::LoadProjResult;
use eframe::egui;
use knodiq_engine::mixer::TrackID;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub(crate) struct ProjectMeta {
    pub tracks: HashMap<TrackID, TrackMeta>,
    pub track_order: Vec<TrackID>,
}

impl ProjectMeta {
    pub fn from_load_res(proj_res: &LoadProjResult) -> Self {
        let mut new_meta = ProjectMeta::default();

        // Initialize the tracks
        for (id, track) in &proj_res.project.tracks {
            let track_name = proj_res
                .track_names
                .get(id)
                .cloned()
                .unwrap_or(format!("Track({})", id.0));
            let track_color = proj_res
                .track_colors
                .get(id)
                .copied()
                .unwrap_or(egui::Color32::WHITE);
            let track_meta = TrackMeta::from_track(track.as_ref(), track_name, track_color);
            new_meta.add_track(*id, track_meta);
        }

        new_meta
    }

    // --- TRACK MANAGEMENT ---

    /// Adds a new track to the project with the given ID.
    pub fn add_track(&mut self, id: TrackID, track: TrackMeta) {
        self.tracks.insert(id, track);
        self.track_order.push(id);
    }

    /// Removes the track with the given ID.
    pub fn remove_track(&mut self, id: &TrackID) {
        self.tracks.remove(id);
    }

    /// Returns a reference to the track with the given ID.
    pub fn get_track(&self, id: &TrackID) -> Option<&TrackMeta> {
        self.tracks.get(id)
    }

    /// Returns a mutable reference to the track with the given ID.
    pub fn get_track_mut(&mut self, id: &TrackID) -> Option<&mut TrackMeta> {
        self.tracks.get_mut(id)
    }
}
