use crate::{metadata::RegionMeta, ui_state::dialog_state::TrackType};
use eframe::egui;
use kreniq_engine::track::RegionID;
use std::collections::HashMap;

pub(crate) struct TrackMeta {
    pub name: String,
    pub color: egui::Color32,
    pub track_type: TrackType,
    pub regions: HashMap<RegionID, RegionMeta>,
}

impl TrackMeta {
    pub fn new(name: String, color: egui::Color32, track_type: TrackType) -> Self {
        Self {
            name,
            color,
            track_type,
            regions: HashMap::new(),
        }
    }

    // --- REGION MANAGEMENT ---

    pub fn add_region(&mut self, id: RegionID, region: RegionMeta) {
        self.regions.insert(id, region);
    }

    pub fn remove_region(&mut self, id: &RegionID) {
        self.regions.remove(id);
    }

    pub fn get_region(&self, id: &RegionID) -> Option<&RegionMeta> {
        self.regions.get(id)
    }

    pub fn get_region_mut(&mut self, id: &RegionID) -> Option<&mut RegionMeta> {
        self.regions.get_mut(id)
    }
}
