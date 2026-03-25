use crate::{app::KnodiqApp, colors::button_bg};
use eframe::egui;

impl KnodiqApp {
    pub(super) fn toolbar_group(
        &mut self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) {
        egui::Frame::new()
            .fill(button_bg(ui))
            .corner_radius(6)
            .inner_margin(1)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(5.0, 0.0);
                add_contents(ui);
            });
    }
}
