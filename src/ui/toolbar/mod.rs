mod playback_control;
mod toolbar_group;

use crate::app::KnodiqApp;
use eframe::egui::{self, Color32, Stroke};

impl KnodiqApp {
    pub(super) fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);

            // Set the button hover and clicker color
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;

            ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                Color32::from_rgba_unmultiplied(150, 150, 150, 50);
            ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;

            ui.style_mut().visuals.widgets.active.weak_bg_fill =
                Color32::from_rgba_unmultiplied(150, 150, 150, 100);
            ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;

            // Draw the playback control buttons
            self.playback_control(ui);
        });
    }
}
