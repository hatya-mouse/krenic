use crate::{app::KnodiqApp, ui_state::dialog_state::TrackType};
use eframe::egui;
use knodiq_engine::{data_types::Beats, mixer::TrackID};

impl KnodiqApp {
    pub(super) fn track_row(
        &mut self,
        ui: &mut egui::Ui,
        track_id: &TrackID,
        row_rect: egui::Rect,
    ) {
        let response = ui.allocate_rect(row_rect, egui::Sense::click());
        let track_type = self.project_meta.get_track(track_id).map(|m| m.track_type);

        response.context_menu(|ui| {
            if ui.button("Add Region").clicked() {
                match track_type {
                    Some(TrackType::AudioTrack) => {
                        self.add_audio_region(track_id, "Region".to_string(), Beats(0.0));
                    }
                    Some(TrackType::NoteTrack) => {
                        self.add_note_region(track_id, "Region".to_string(), Beats(0.0));
                    }
                    None => (),
                }

                ui.close();
            }
        });
    }
}
