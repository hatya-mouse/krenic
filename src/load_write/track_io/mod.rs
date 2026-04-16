mod audio_track;
mod note_track;

use crate::load_write::{AsBytes, FromBytes};
use knodiq_engine::track::{Track, audio_track::AudioTrack, note_track::NoteTrack};
use std::io::{Cursor, Read};

#[repr(u8)]
enum TrackKind {
    Audio = 0,
    Note = 1,
}

impl AsBytes for dyn Track {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the graph
        let graph = self.get_graph();
        graph.as_bytes(bytes);

        // Then write the contents of the track depending on the track type
        if let Some(audio_track) = self.as_any().downcast_ref::<AudioTrack>() {
            // Write the track kind
            bytes.push(TrackKind::Audio as u8);

            // Get the size of the track
            let mut track_bytes = Vec::new();
            audio_track.as_bytes(&mut track_bytes);
            // Write the size of the track and its contents
            bytes.extend((track_bytes.len() as u64).to_le_bytes());
            bytes.extend(track_bytes);
        } else if let Some(note_track) = self.as_any().downcast_ref::<NoteTrack>() {
            // Write the track kind
            bytes.push(TrackKind::Note as u8);

            // Get the size of the track
            let mut track_bytes = Vec::new();
            note_track.as_bytes(&mut track_bytes);
            // Write the size of the track and its contents
            bytes.extend((track_bytes.len() as u64).to_le_bytes());
            bytes.extend(track_bytes);
        }
    }
}

impl FromBytes for Box<dyn Track> {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        // Get the first one byte and get the type of the track
        let mut type_byte = [0u8; 1];
        let mut track_len_bytes = [0u8; 8];
        cursor.read_exact(&mut type_byte)?;
        cursor.read_exact(&mut track_len_bytes)?;
        let track_len = u64::from_le_bytes(track_len_bytes) as usize;

        // Get the content bytes
        let mut track_data_bytes = vec![0u8; track_len];
        cursor.read_exact(&mut track_data_bytes)?;

        match type_byte[0] {
            0 => Ok(Box::new(AudioTrack::from_bytes(&track_data_bytes)?)),
            1 => Ok(Box::new(NoteTrack::from_bytes(&track_data_bytes)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid track kind",
            )),
        }
    }
}
