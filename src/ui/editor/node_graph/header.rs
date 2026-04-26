use crate::{colors, components::icon_button::icon_button, ui::EditorUi};
use eframe::egui;
use std::fmt::Display;

enum AddibleNodes {
    Kasl,
}

impl Display for AddibleNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddibleNodes::Kasl => write!(f, "KASL Node"),
        }
    }
}

impl EditorUi {
    pub(super) fn draw_node_graph_header(&mut self, ui: &mut egui::Ui) {
        let mut node_to_add: Option<AddibleNodes> = None;

        let response = egui::Frame::new()
            .fill(colors::tertiary_bg(ui.visuals().dark_mode))
            .inner_margin(egui::Margin::symmetric(8, 4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let btn = icon_button(
                        ui,
                        egui::Image::new(egui::include_image!("../../../../assets/icons/plus.svg")),
                    );
                    egui::Popup::menu(&btn).show(|ui| {
                        if ui.button(AddibleNodes::Kasl.to_string()).clicked() {
                            node_to_add = Some(AddibleNodes::Kasl);
                        }
                    });
                });
            });

        let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
        let rect = response.response.rect;
        ui.painter()
            .line_segment([rect.left_bottom(), rect.right_bottom()], stroke);

        // Add a new node if the node is clicked on the add list
        if let Some(node_type) = node_to_add {
            // Get the currently selected track
            let Some(track_id) = self.ui_state.selected_track else {
                return;
            };

            let pan = self.ui_state.node_graph_state.pan_offset;
            let pos = egui::pos2(-pan.x + 20.0, -pan.y + 20.0);
            match node_type {
                AddibleNodes::Kasl => self.add_kasl_node(&track_id, pos),
            }
        }
    }
}
