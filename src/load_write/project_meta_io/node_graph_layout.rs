use crate::{
    load_write::{AsBytes, FromBytes},
    metadata::NodeGraphLayout,
};
use eframe::egui;
use knodiq_engine::graph::node_id::NodeID;
use std::io::{Cursor, Read};

pub struct StoredNodeGraphLayout {
    pan_x: f32,
    pan_y: f32,
    node_positions: Vec<(usize, f32, f32)>,
}

impl Default for StoredNodeGraphLayout {
    fn default() -> Self {
        Self {
            pan_x: 0.0,
            pan_y: 0.0,
            node_positions: Vec::new(),
        }
    }
}

impl StoredNodeGraphLayout {
    pub fn from_layout(layout: &NodeGraphLayout) -> Self {
        Self {
            pan_x: layout.pan_offset.x,
            pan_y: layout.pan_offset.y,
            node_positions: layout
                .node_positions
                .iter()
                .map(|(id, pos)| (id.0, pos.x, pos.y))
                .collect(),
        }
    }

    pub fn to_layout(&self) -> NodeGraphLayout {
        let mut layout = NodeGraphLayout {
            pan_offset: egui::vec2(self.pan_x, self.pan_y),
            ..Default::default()
        };
        for &(id, x, y) in &self.node_positions {
            layout.node_positions.insert(NodeID(id), egui::pos2(x, y));
        }
        layout
    }
}

impl AsBytes for StoredNodeGraphLayout {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.extend(self.pan_x.to_le_bytes());
        bytes.extend(self.pan_y.to_le_bytes());
        bytes.extend((self.node_positions.len() as u64).to_le_bytes());
        for &(id, x, y) in &self.node_positions {
            bytes.extend((id as u64).to_le_bytes());
            bytes.extend(x.to_le_bytes());
            bytes.extend(y.to_le_bytes());
        }
    }
}

impl FromBytes for StoredNodeGraphLayout {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        let mut pan_x_bytes = [0u8; 4];
        let mut pan_y_bytes = [0u8; 4];
        cursor.read_exact(&mut pan_x_bytes)?;
        cursor.read_exact(&mut pan_y_bytes)?;

        let mut count_bytes = [0u8; 8];
        cursor.read_exact(&mut count_bytes)?;
        let count = u64::from_le_bytes(count_bytes) as usize;

        let mut node_positions = Vec::with_capacity(count);
        for _ in 0..count {
            let mut id_bytes = [0u8; 8];
            let mut x_bytes = [0u8; 4];
            let mut y_bytes = [0u8; 4];
            cursor.read_exact(&mut id_bytes)?;
            cursor.read_exact(&mut x_bytes)?;
            cursor.read_exact(&mut y_bytes)?;
            node_positions.push((
                u64::from_le_bytes(id_bytes) as usize,
                f32::from_le_bytes(x_bytes),
                f32::from_le_bytes(y_bytes),
            ));
        }

        Ok(Self {
            pan_x: f32::from_le_bytes(pan_x_bytes),
            pan_y: f32::from_le_bytes(pan_y_bytes),
            node_positions,
        })
    }
}
