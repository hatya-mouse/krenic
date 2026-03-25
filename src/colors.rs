use eframe::egui::{self, Color32};

pub(crate) fn icon_tint(ui: &egui::Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(230, 230, 230)
    } else {
        Color32::from_rgb(40, 40, 40)
    }
}

pub(crate) fn button_bg(ui: &egui::Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgb(70, 70, 70)
    } else {
        Color32::from_rgb(230, 230, 230)
    }
}
