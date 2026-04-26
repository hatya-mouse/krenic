mod node;
mod port;

// --- NODE LAYOUT CONSTANTS ---

pub(super) const NODE_WIDTH: f32 = 180.0;
pub(super) const HEADER_HEIGHT: f32 = 28.0;
pub(super) const PORT_ROW_HEIGHT: f32 = 22.0;
pub(super) const PORT_RADIUS: f32 = 8.0;
/// Padding on top and bottom of the node body.
pub(super) const NODE_PADDING: f32 = 4.0;

use crate::ui::EditorUi;
use eframe::egui;
use knodiq_engine::graph::node_id::NodeID;

impl EditorUi {
    pub(in crate::ui) fn node_graph(&mut self, ui: &mut egui::Ui) {
        let content_rect = ui.available_rect_before_wrap();

        // Allocate the full canvas rect for background interaction.
        // Must be before node allocations so node interactions take priority.
        let bg_response = ui.allocate_rect(content_rect, egui::Sense::drag());

        // Middle mouse drag to pan
        if bg_response.dragged_by(egui::PointerButton::Middle) {
            self.ui_state.node_graph_state.pan_offset += bg_response.drag_delta();
        }

        // view_transform converts canvas-space positions to screen-space each frame.
        // Adding content_rect.min ensures nodes follow panel moves and resizes automatically.
        let view_transform = content_rect.min.to_vec2() + self.ui_state.node_graph_state.pan_offset;

        // Collect node IDs from the selected track's graph meta.
        // Cloned up front to avoid holding a borrow on project_meta during draw.
        let Some(node_ids): Option<Vec<NodeID>> = self
            .ui_state
            .selected_track
            .and_then(|track_id| self.project_meta.get_track(&track_id))
            .map(|track| track.graph.nodes.keys().cloned().collect())
        else {
            return;
        };

        // Draw each node
        for node_id in node_ids {
            self.draw_node(ui, &node_id, view_transform);
        }
    }
}
