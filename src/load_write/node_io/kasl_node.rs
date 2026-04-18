use crate::{
    kasl_node::KaslNode,
    load_write::{AsBytes, FromBytes},
};
use std::path::PathBuf;

impl AsBytes for KaslNode {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        let path_str = self
            .get_file_path()
            .and_then(|p| p.to_str())
            .unwrap_or_default();
        bytes.extend_from_slice(path_str.as_bytes());
    }
}

impl FromBytes for KaslNode {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let path_str = String::from_utf8(bytes.to_vec())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let mut node = KaslNode::new();
        if !path_str.is_empty() {
            node.set_file_path(PathBuf::from(path_str));
        }
        Ok(node)
    }
}
