use crate::ui::{EditorUi, editor::inspector::inspector_section};
use eframe::egui;
use knodiq_engine::mixer::TrackID;

impl EditorUi {
    pub(super) fn track_inspector(&mut self, ui: &mut egui::Ui, track_id: &TrackID) {
        let Some(track_meta) = self.project_meta.get_track_mut(track_id) else {
            return;
        };

        inspector_section(ui, "Track".to_string(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut track_meta.name);
            });

            if self.debug_mode {
                ui.label(format!("Track ID: {}", track_id.0));
            }
        });
    }
}
