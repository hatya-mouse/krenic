use super::{HEADER_HEIGHT, PORT_RADIUS, PORT_ROW_HEIGHT};
use crate::{colors, ui::editor::node_graph::NODE_PADDING};
use eframe::egui;

/// Draws the input and output ports of a node.
/// Ports are laid out in rows below the header; inputs on the left, outputs on the right.
pub(super) fn draw_ports(
    painter: &egui::Painter,
    node_rect: egui::Rect,
    input_names: &[String],
    output_names: &[String],
    dark_mode: bool,
) {
    let row_count = input_names.len().max(output_names.len());

    for current_row in 0..row_count {
        // Vertical center of this port row
        let y = calc_port_y(node_rect, current_row);

        // Input port on the left edge
        if let Some(name) = input_names.get(current_row) {
            let center = egui::pos2(node_rect.min.x, y);
            painter.circle_filled(center, PORT_RADIUS, colors::node_port_input());
            painter.text(
                egui::pos2(center.x + PORT_RADIUS + 4.0, y),
                egui::Align2::LEFT_CENTER,
                name,
                egui::FontId::default(),
                colors::primary_fg(dark_mode),
            );
        }

        // Output port on the right edge
        if let Some(name) = output_names.get(current_row) {
            let center = egui::pos2(node_rect.max.x, y);
            painter.circle_filled(center, PORT_RADIUS, colors::node_port_output());
            painter.text(
                egui::pos2(center.x - PORT_RADIUS - 4.0, y),
                egui::Align2::RIGHT_CENTER,
                name,
                egui::FontId::default(),
                colors::primary_fg(dark_mode),
            );
        }
    }
}

pub(super) fn calc_port_y(node_rect: egui::Rect, port_index: usize) -> f32 {
    node_rect.min.y
        + HEADER_HEIGHT
        + NODE_PADDING
        + PORT_ROW_HEIGHT * port_index as f32
        + PORT_ROW_HEIGHT / 2.0
}
