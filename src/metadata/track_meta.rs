use crate::{metadata::RegionMeta, ui_state::dialog_state::TrackType};
use eframe::egui;
use knodiq_engine::track::{
    RegionID, Track,
    audio_track::{AudioRegion, AudioTrack},
    note_track::{NoteRegion, NoteTrack},
};
use std::collections::HashMap;

#[derive(Debug)]
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

    pub fn from_track(track: &dyn Track, name: String, color: egui::Color32) -> Self {
        // Determine track type based on the track's type
        if let Some(audio_region) = track.as_any().downcast_ref::<AudioTrack>() {
            // Get the audio regions from the track
            let regions = audio_region
                .get_all_regions()
                .iter()
                .map(|(region_id, note_region)| {
                    // TODO: Get the region name
                    (
                        *region_id,
                        RegionMeta::new(
                            name.clone(),
                            note_region.start,
                            note_region.duration,
                            None,
                        ),
                    )
                })
                .collect();

            Self {
                name,
                color,
                track_type: TrackType::Audio,
                regions,
            }
        } else if let Some(note_region) = track.as_any().downcast_ref::<NoteTrack>() {
            // Get the note regions from the track
            let regions = note_region
                .get_all_regions()
                .iter()
                .map(|(region_id, note_region)| {
                    // TODO: Get the region name
                    (
                        *region_id,
                        RegionMeta::new(
                            name.clone(),
                            note_region.start,
                            note_region.duration,
                            None,
                        ),
                    )
                })
                .collect();

            Self {
                name,
                color,
                track_type: TrackType::Note,
                regions,
            }
        } else {
            unreachable!("There must be no tracks other than AudioTrack and NoteTrack");
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
