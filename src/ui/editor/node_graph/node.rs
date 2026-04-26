use super::{HEADER_HEIGHT, NODE_PADDING, NODE_WIDTH, PORT_ROW_HEIGHT};
use crate::{colors, metadata::ProjectMeta, ui::EditorUi, ui_state::editor_state::EditorUiState};
use eframe::egui::{self, Sense};
use knodiq_engine::{graph::node_id::NodeID, mixer::TrackID};

impl EditorUi {
    pub(super) fn draw_node(
        &mut self,
        ui: &mut egui::Ui,
        node_id: &NodeID,
        view_transform: egui::Vec2,
    ) {
        let Some(track_id) = self.ui_state.selected_track else {
            return;
        };

        // Gather display data from the node and its meta.
        // Borrows end at the closing brace so we can take &mut self freely afterwards.
        let (input_names, output_names, pos, display_name) = {
            let Some(node) = self
                .project
                .get_track(&track_id)
                .and_then(|t| t.get_graph().get_node(node_id))
            else {
                return;
            };
            let Some(meta) = self
                .project_meta
                .get_track(&track_id)
                .and_then(|t| t.graph.get_node_meta(node_id))
            else {
                return;
            };
            (
                node.get_input_names(),
                node.get_output_names(),
                meta.pos,
                meta.display_name.clone(),
            )
        };

        // Calculate node geometry.
        // pos is in canvas space; view_transform converts it to screen space.
        let row_count = input_names.len().max(output_names.len()).max(1);
        let node_height =
            HEADER_HEIGHT + NODE_PADDING + PORT_ROW_HEIGHT * row_count as f32 + NODE_PADDING;
        let node_rect =
            egui::Rect::from_min_size(pos + view_transform, egui::vec2(NODE_WIDTH, node_height));
        let header_rect =
            egui::Rect::from_min_size(node_rect.min, egui::vec2(NODE_WIDTH, HEADER_HEIGHT));

        let dark_mode = ui.visuals().dark_mode;
        let painter = ui.painter();

        // Draw the node background
        painter.rect(
            node_rect,
            egui::CornerRadius::same(6),
            colors::secondary_bg(dark_mode),
            egui::Stroke::new(1.0, colors::border(dark_mode)),
            egui::StrokeKind::Outside,
        );

        // Draw the header background and its bottom border
        painter.rect_filled(
            header_rect,
            egui::CornerRadius {
                nw: 6,
                ne: 6,
                sw: 0,
                se: 0,
            },
            colors::tertiary_bg(dark_mode),
        );
        painter.line_segment(
            [header_rect.left_bottom(), header_rect.right_bottom()],
            egui::Stroke::new(1.0, colors::border(dark_mode)),
        );

        // Draw the node name in the header
        painter.text(
            header_rect.center(),
            egui::Align2::CENTER_CENTER,
            display_name.as_str(),
            egui::FontId::proportional(13.0),
            colors::primary_fg(dark_mode),
        );

        // Draw the input/output ports
        super::port::draw_ports(painter, node_rect, &input_names, &output_names, dark_mode);

        // Handle node gestures (click to select, drag to move)
        let response = ui.allocate_rect(node_rect, Sense::click_and_drag());
        Self::apply_node_gesture(
            response,
            node_id,
            &track_id,
            &mut self.project_meta,
            &mut self.ui_state,
        );
    }

    fn apply_node_gesture(
        response: egui::Response,
        node_id: &NodeID,
        track_id: &TrackID,
        project_meta: &mut ProjectMeta,
        ui_state: &mut EditorUiState,
    ) {
        // Click or drag to select the node
        if response.clicked() || response.dragged() {
            ui_state.set_selected_node(*node_id);
        }

        // Drag to move the node
        if response.dragged()
            && let Some(meta) = project_meta
                .get_track_mut(track_id)
                .and_then(|t| t.graph.get_node_meta_mut(node_id))
        {
            meta.pos += response.drag_delta();
        }
    }
}
