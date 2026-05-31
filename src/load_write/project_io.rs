use crate::load_write::AsBytes;
use kreniq_engine::mixer::Project;
use std::{fs::File, io::Write, path::Path};

pub(crate) fn save_project(path: &Path, project: &Project) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // Write the project data to the file
    // First write "KNODIQ" to check if the file is a Kreniq Project file
    file.write_all("KNODIQ".as_bytes())?;

    // Then write the version of Kreniq
    let major_ver: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor_ver: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch_ver: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
    file.write_all(&major_ver.to_le_bytes())?;
    file.write_all(&minor_ver.to_le_bytes())?;
    file.write_all(&patch_ver.to_le_bytes())?;

    // Write the project
    let mut project_bytes = Vec::new();
    project.as_bytes(&mut project_bytes);
    file.write_all(&project_bytes)?;

    Ok(())
}

impl AsBytes for Project {
    fn as_bytes(&self, bytes: &mut Vec<u8>) {
        // Write the audio configuration to the file
        bytes.extend(&(self.audio_ctx.channels as u64).to_le_bytes());
        bytes.extend(&(self.audio_ctx.sample_rate as u64).to_le_bytes());
        bytes.extend(&(self.audio_ctx.buffer_size as u64).to_le_bytes());
        bytes.extend(&(self.audio_ctx.max_voices as u64).to_le_bytes());

        // Write the tracks to the file
        for (track_id, track) in &self.tracks {
            // Write the track ID
            bytes.extend(&(track_id.0 as u64).to_le_bytes());

            // Write the track data
            track.as_bytes(bytes);
        }
    }
}
