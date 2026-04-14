use crate::load_write::{AsBytes, FromBytes};
use knodiq_engine::{
    graph::{Graph, node_id::NodeID},
    node::Node,
};
use std::io::{Cursor, Read};

impl AsBytes for Graph {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write each nodes
        let mut nodes_bytes = Vec::new();
        for (node_id, node) in self.get_node_map() {
            let mut node_bytes = Vec::new();
            node.as_bytes(&mut node_bytes);
            // Write the ID of the node
            nodes_bytes.extend((node_id.0 as u64).to_le_bytes());
            // Write the length of the node data
            nodes_bytes.extend((node_bytes.len() as u64).to_le_bytes());
            // Write the node data
            nodes_bytes.extend(node_bytes);
        }

        // Write the length of the nodes_bytes
        bytes.extend((nodes_bytes.len() as u64).to_le_bytes());
        // Write the nodes_bytes
        bytes.extend(nodes_bytes);
    }
}

impl FromBytes for Graph {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut graph = Graph::default();
        let mut cursor = Cursor::new(bytes);

        // Read the length of the nodes_bytes
        let mut length_bytes = [0u8; 8];
        cursor.read_exact(&mut length_bytes)?;
        let nodes_length = u64::from_le_bytes(length_bytes) as usize;

        // Read nodes_bytes
        let mut nodes_bytes = vec![0u8; nodes_length];
        cursor.read_exact(&mut nodes_bytes)?;
        let mut nodes_cursor = Cursor::new(nodes_bytes);

        // Read each node
        while nodes_cursor.position() < nodes_length as u64 {
            // Read node_id
            let mut buf = [0u8; 8];
            nodes_cursor.read_exact(&mut buf)?;
            let node_id = NodeID(u64::from_le_bytes(buf) as usize);

            // Read node data length
            nodes_cursor.read_exact(&mut buf)?;
            let node_length = u64::from_le_bytes(buf) as usize;

            // Read node data
            let mut node_bytes = vec![0u8; node_length];
            nodes_cursor.read_exact(&mut node_bytes)?;
            let node = <Box<dyn Node>>::from_bytes(&node_bytes)?;

            graph.add_node_with_id(node_id, node);
        }

        Ok(graph)
    }
}
