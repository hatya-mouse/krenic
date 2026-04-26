mod edge;
mod node;
mod port;

// --- NODE LAYOUT CONSTANTS ---

pub(super) const NODE_WIDTH: f32 = 180.0;
pub(super) const HEADER_HEIGHT: f32 = 28.0;
pub(super) const PORT_ROW_HEIGHT: f32 = 22.0;
pub(super) const PORT_RADIUS: f32 = 8.0;
/// Padding on top and bottom of the node body.
pub(super) const NODE_PADDING: f32 = 4.0;
/// Thinkness of the edge lines.
pub(super) const EDGE_WIDTH: f32 = 4.0;

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

        let Some(track_id) = self.ui_state.selected_track else {
            return;
        };

        // Collect what we need up front to avoid holding borrows during drawing
        let node_ids: Vec<NodeID> = self
            .project_meta
            .get_track(&track_id)
            .map(|t| t.graph.nodes.keys().cloned().collect())
            .unwrap_or_default();

        let edges = self
            .project
            .get_track(&track_id)
            .map(|t| t.get_graph().get_edges().clone())
            .unwrap_or_default();

        // Copy ghost/dragged edge before borrowing project_meta below
        let ghost_edge = self.ui_state.node_graph_state.ghost_edge;
        let dragged_edge = self.ui_state.node_graph_state.dragged_edge;

        // Draw edges behind nodes
        if let Some(track_meta) = self.project_meta.get_track(&track_id) {
            let painter = ui.painter();
            edge::draw_edges(ui, painter, &edges, &track_meta.graph, view_transform, dragged_edge);
            if let Some(ghost) = ghost_edge {
                edge::draw_ghost_edge(ui, painter, &ghost, &track_meta.graph, view_transform);
            }
        }

        // Draw each node (on top of edges)
        for node_id in node_ids {
            self.draw_node(ui, &node_id, view_transform);
        }
    }
}
