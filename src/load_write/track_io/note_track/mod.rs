mod note;
mod note_region;

use crate::load_write::AsBytes;
use knodiq_engine::track::note_track::NoteTrack;

impl AsBytes for NoteTrack {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Convert the regions into bytes
        let mut region_bytes = Vec::new();
        for (region_id, region) in self.get_all_regions() {
            region_bytes.extend((region_id.0 as u64).to_le_bytes());
            region.as_bytes(&mut region_bytes);
        }

        // Write the length of the regions
        bytes.extend((region_bytes.len() as u64).to_le_bytes());
        // Write the regions
        bytes.extend(region_bytes);
    }
}
