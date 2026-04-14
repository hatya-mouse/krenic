use crate::load_write::AsBytes;
use knodiq_engine::graph::Graph;

impl AsBytes for Graph {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write each nodes
        for (node_id, node) in self.get_node_map() {
            let node_bytes = node.as_bytes();
            // Write the ID of the node
            bytes.extend(&(node_id.0 as u64).to_le_bytes());
            // Write the length of the node data
            bytes.extend(&(node_bytes.len() as u64).to_le_bytes());
            // Write the node data
            bytes.extend(&node_bytes);
        }
    }
}
