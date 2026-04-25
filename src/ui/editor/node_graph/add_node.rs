use crate::{kasl_node::KaslNode, metadata::ProjectMeta, ui::EditorUi};
use eframe::egui;
use knodiq_engine::{mixer::TrackID, node::Node};
use std::path::Path;

/// Node types that can be added from the context menu.
/// Add new variants here to extend the menu automatically.
#[derive(Clone, Copy)]
pub(super) enum AddableNodeKind {
    Kasl,
}

impl AddableNodeKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Kasl => "KASL Node",
        }
    }

    /// All variants in the order they appear in the context menu.
    pub fn all() -> &'static [Self] {
        &[Self::Kasl]
    }

    /// Instantiate the node. File/resource configuration is done separately by the user.
    pub fn create(self, project_meta: &ProjectMeta, project_dir: &Path) -> Box<dyn Node> {
        match self {
            Self::Kasl => {
                let mut node = KaslNode::new();
                node.set_search_paths(project_meta.kasl_search_paths.clone());
                node.set_project_dir(project_dir.to_path_buf());
                // No file path — the user will assign it elsewhere.
                Box::new(node)
            }
        }
    }
}

impl EditorUi {
    /// Add any node to the graph at the given canvas position.
    pub(super) fn add_node_to_graph(
        &mut self,
        track_id: TrackID,
        node: Box<dyn Node>,
        canvas_pos: egui::Pos2,
    ) {
        let Some(track) = self.project.tracks.get_mut(&track_id) else {
            return;
        };
        let node_id = track.get_graph_mut().add_node(node);

        if let Some(track_meta) = self.project_meta.get_track_mut(&track_id) {
            track_meta.node_graph.set_node_pos(node_id, canvas_pos);
        }

        self.modified_project();
    }
}
