use crate::app::KnodiqApp;
use eframe::egui;
use knodiq_engine::mixer::TrackID;

impl KnodiqApp {
    pub(super) fn track_row(&mut self, ui: &mut egui::Ui, track_id: TrackID, row_rect: egui::Rect) {
        let response = ui.allocate_rect(row_rect, egui::Sense::click());
        response.context_menu(|ui| {
            if ui.button("Add Region").clicked() {
                // リージョン追加処理
                ui.close_menu();
            }
        });
    }
}
