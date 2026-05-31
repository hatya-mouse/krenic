mod audio_track;
mod note_track;

use crate::load_write::AsBytes;
use kreniq_engine::track::{Track, audio_track::AudioTrack, note_track::NoteTrack};

impl AsBytes for dyn Track {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the graph
        let graph = self.get_graph();
        graph.as_bytes(bytes);

        // Then write the contents of the track depending on the track type
        if let Some(audio_track) = self.as_any().downcast_ref::<AudioTrack>() {
            audio_track.as_bytes(bytes);
        } else if let Some(note_track) = self.as_any().downcast_ref::<NoteTrack>() {
            note_track.as_bytes(bytes);
        }
    }
}
