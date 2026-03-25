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
        self.draw_regions(ui, track_id, row_rect);
        self.add_region_menu(ui, track_id, row_rect);
    }

    fn draw_regions(&self, ui: &mut egui::Ui, track_id: &TrackID, row_rect: egui::Rect) {
        let Some(track_meta) = self.project_meta.get_track(track_id) else {
            return;
        };
        let ppb = self.ui_state.pixels_per_beat;
        let painter = ui.painter();

        for region_meta in track_meta.regions.values() {
            let x = row_rect.min.x + region_meta.start.0 as f32 * ppb;
            let w = region_meta.duration.0 as f32 * ppb;
            let region_rect = egui::Rect::from_min_size(
                egui::pos2(x, row_rect.min.y + 2.0),
                egui::vec2(w, row_rect.height() - 4.0),
            );

            let region_painter = painter.with_clip_rect(region_rect);

            region_painter.rect_filled(region_rect, 0.0, track_meta.color.gamma_multiply(0.8));

            // Name label
            region_painter.text(
                egui::pos2(region_rect.min.x + 4.0, region_rect.min.y + 2.0),
                egui::Align2::LEFT_TOP,
                &region_meta.name,
                egui::FontId::proportional(11.0),
                egui::Color32::WHITE,
            );
        }
    }

    fn add_region_menu(&mut self, ui: &mut egui::Ui, track_id: &TrackID, row_rect: egui::Rect) {
        let area = ui.allocate_rect(row_rect, egui::Sense::click());
        let track_type = self.project_meta.get_track(track_id).map(|m| m.track_type);

        if area.double_clicked() {
            let click_pos_beats = area.interact_pointer_pos().map(|pos| {
                Beats(((pos.x - row_rect.min.x) / self.ui_state.pixels_per_beat) as f64)
            });

            let start = click_pos_beats.unwrap_or(Beats(0.0));
            match track_type {
                Some(TrackType::AudioTrack) => {
                    self.add_audio_region(track_id, "Region".to_string(), start);
                }
                Some(TrackType::NoteTrack) => {
                    self.add_note_region(track_id, "Region".to_string(), start);
                }
                None => (),
            }

            ui.close();
        }
    }
}
