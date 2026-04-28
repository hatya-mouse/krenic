use crate::ui::EditorUi;
use knodiq_engine::{mixer::TrackID, track::RegionID};

impl EditorUi {
    pub(crate) fn remove_region(&mut self, track_id: &TrackID, region_id: &RegionID) {
        if let Some(track) = self.project.get_track_mut(track_id) {
            track.remove_region(region_id);
        }
        if let Some(track_meta) = self.project_meta.get_track_mut(track_id) {
            track_meta.remove_region(region_id);
        }

        if self.ui_state.selected_region == Some((*track_id, *region_id)) {
            self.ui_state.deselect_all();
        }

        self.modified_project();
    }
}
