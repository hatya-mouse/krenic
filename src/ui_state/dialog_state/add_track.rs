use std::fmt::Display;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TrackType {
    Audio,
    Note,
}

impl Display for TrackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Audio => write!(f, "Audio"),
            Self::Note => write!(f, "Note"),
        }
    }
}

pub struct AddTrackState {
    pub selected_track_type: TrackType,
    pub name: String,
}
