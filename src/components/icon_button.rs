use crate::colors::icon_tint;
use eframe::egui;

pub(crate) fn icon_button(ui: &mut egui::Ui, image: egui::Image) -> egui::Response {
    ui.add_sized(
        [40.0, 28.0],
        egui::Button::image(
            image
                .fit_to_exact_size(egui::vec2(24.0, 24.0))
                .tint(icon_tint(ui)),
        )
        .corner_radius(6),
    )
}
