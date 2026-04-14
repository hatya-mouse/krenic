mod graph_io;
mod project_io;
mod track_io;

pub trait AsBytes {
    /// Converts the self into bytes so it can be loaded later.
    fn as_bytes(&self, bytes: &mut Vec<u8>);
}

pub trait FromBytes {
    /// Loads the instance of self from bytes.
    fn from_bytes(bytes: &[u8]) -> Self;
}
