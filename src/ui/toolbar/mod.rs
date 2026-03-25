mod playback_control;
mod toolbar_group;

use crate::app::KnodiqApp;
use eframe::egui;

impl KnodiqApp {
    pub(super) fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);

            // Set the button hover and clicker color
            ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
            ui.visuals_mut().widgets.inactive.bg_stroke = egui::Stroke::NONE;

            ui.visuals_mut().widgets.hovered.weak_bg_fill =
                egui::Color32::from_rgba_unmultiplied(150, 150, 150, 50);
            ui.visuals_mut().widgets.hovered.bg_stroke = egui::Stroke::NONE;

            ui.visuals_mut().widgets.active.weak_bg_fill =
                egui::Color32::from_rgba_unmultiplied(150, 150, 150, 100);
            ui.visuals_mut().widgets.active.bg_stroke = egui::Stroke::NONE;

            // Draw the playback control buttons
            self.playback_control(ui);
        });
    }
}
