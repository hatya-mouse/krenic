use crate::metadata::RegionMeta;
use eframe::egui;
use knodiq_engine::track::RegionID;
use std::collections::HashMap;

pub(crate) struct TrackMeta {
    pub name: String,
    pub color: egui::Color32,
    pub regions: HashMap<RegionID, RegionMeta>,
}

impl TrackMeta {
    pub fn new(name: String, color: egui::Color32) -> Self {
        Self {
            name,
            color,
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

    pub fn get_region_meta(&mut self, id: &RegionID) -> Option<&mut RegionMeta> {
        self.regions.get_mut(id)
    }
}
