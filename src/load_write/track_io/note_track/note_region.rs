use crate::load_write::AsBytes;
use knodiq_engine::track::note_track::NoteRegion;

impl AsBytes for NoteRegion {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Convert the notes into bytes
        let mut note_bytes = Vec::new();
        for (note_id, note) in &self.notes {
            note_bytes.extend((note_id.0 as u64).to_le_bytes());
            note.as_bytes(bytes);
        }

        // Write the length of the note bytes
        bytes.extend((note_bytes.len() as u64).to_le_bytes());
        // Write the note bytes
        bytes.extend(note_bytes);
    }
}
