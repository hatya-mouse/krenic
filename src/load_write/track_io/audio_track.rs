use crate::load_write::AsBytes;
use knodiq_engine::track::audio_track::AudioTrack;

impl AsBytes for AudioTrack {
    fn as_bytes(&self, _bytes: &mut Vec<u8>) {}
}
