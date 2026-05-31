mod region_meta;
mod track_meta;

pub(crate) use region_meta::RegionMeta;
pub(crate) use track_meta::TrackMeta;

use kreniq_engine::mixer::TrackID;
use std::collections::HashMap;

pub(crate) struct ProjectMeta {
    pub tracks: HashMap<TrackID, TrackMeta>,
    pub track_order: Vec<TrackID>,
}

impl ProjectMeta {
    pub fn new() -> Self {
        Self {
            tracks: HashMap::new(),
            track_order: Vec::new(),
        }
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
