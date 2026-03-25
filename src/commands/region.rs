use crate::{app::KnodiqApp, metadata::RegionMeta};
use knodiq_engine::{
    data_types::Beats,
    mixer::TrackID,
    track::{
        audio_track::{AudioRegion, AudioTrack},
        note_track::{NoteRegion, NoteTrack},
    },
};

impl KnodiqApp {
    /// Adds a new empty audio region to the given audio track.
    pub(crate) fn add_audio_region(&mut self, track_id: &TrackID, name: String, start: Beats) {
        let sample_rate = self.project.audio_ctx.sample_rate;
        let channels = self.project.audio_ctx.channels;

        // Get the target track
        let Some(track) = self.project.get_track_mut(track_id) else {
            return;
        };

        // Cast the track to AudioTrack
        if let Some(audio_track) = track.as_any_mut().downcast_mut::<AudioTrack>() {
            // Create a region and add it to the audio track
            let duration = Beats(1.0);
            let base_bpm = 120.0;
            let frames = (duration.0 / base_bpm) as usize * 60 * sample_rate;
            let audio_region = AudioRegion::zeros(
                frames,
                sample_rate as u32,
                channels as u16,
                base_bpm,
                start,
                duration,
            );
            let region_id = audio_track.add_region(audio_region);

            // Add a region to the project meta
            if let Some(track_meta) = self.project_meta.get_track_mut(track_id) {
                let region_meta = RegionMeta::new(name, start, duration);
                track_meta.add_region(region_id, region_meta);
            }

            // Update the project on the audio thread
            self.update_project();
        }
    }

    /// Adds a new empty audio region to the given audio track.
    pub(crate) fn add_note_region(&mut self, track_id: &TrackID, name: String, start: Beats) {
        // Get the target track
        let Some(track) = self.project.get_track_mut(track_id) else {
            return;
        };

        // Cast the track to AudioTrack
        if let Some(audio_track) = track.as_any_mut().downcast_mut::<NoteTrack>() {
            // Create a region and add it to the audio track
            let duration = Beats(1.0);
            let note_region = NoteRegion::new(start, duration);
            let region_id = audio_track.add_region(note_region);

            // Add a region to the project meta
            if let Some(track_meta) = self.project_meta.get_track_mut(track_id) {
                let region_meta = RegionMeta::new(name, start, duration);
                track_meta.add_region(region_id, region_meta);
            }

            // Update the project on the audio thread
            self.update_project();
        }
    }
}
