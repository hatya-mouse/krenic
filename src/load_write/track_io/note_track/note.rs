use crate::load_write::AsBytes;
use kreniq_engine::track::note_track::Note;

impl AsBytes for Note {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        bytes.extend(self.start.0.to_le_bytes());
        bytes.extend(self.duration.0.to_le_bytes());
        bytes.extend(self.pitch.to_le_bytes());
        bytes.extend(self.velocity.to_le_bytes());
    }
}
