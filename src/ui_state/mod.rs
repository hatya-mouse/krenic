pub mod dialog_state;
pub mod timeline_state;

use crate::ui_state::{dialog_state::DialogState, timeline_state::TimelineState};
use knodiq_engine::{mixer::TrackID, track::RegionID};
use std::time::Instant;

pub struct KnodiqUIState {
    /// The current dialog state.
    pub dialog_state: DialogState,

    /// The current timeline state.
    pub timeline_state: TimelineState,

    /// An instant to track the last edited time for project updating.
    pub last_edit_time: Option<Instant>,

    /// An ID of the currently selected region.
    pub selected_region: Option<(TrackID, RegionID)>,

    /// Pixels per beat.
    pub pixels_per_beat: f32,
}

impl Default for KnodiqUIState {
    fn default() -> Self {
        Self {
            dialog_state: DialogState::None,
            timeline_state: TimelineState::default(),
            last_edit_time: None,
            selected_region: None,
            pixels_per_beat: 80.0,
        }
    }
}
