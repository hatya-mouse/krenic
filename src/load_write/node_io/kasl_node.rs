use crate::{
    kasl_node::KaslNode,
    load_write::{AsBytes, FromBytes},
};

impl AsBytes for KaslNode {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Convert the code into bytes
        let code_bytes = self
            .get_code()
            .map(|code| code.as_bytes())
            .unwrap_or_default();
        // Write the code data
        bytes.extend_from_slice(code_bytes);
    }
}

impl FromBytes for KaslNode {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        // Get the code from the node
        let code = String::from_utf8(bytes.to_vec())
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        // Create a new node and set the code
        let mut node = KaslNode::new();
        node.set_code(code);

        Ok(node)
    }
}
