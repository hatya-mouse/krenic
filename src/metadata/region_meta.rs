use knodiq_engine::data_types::Beats;

pub(crate) struct RegionMeta {
    pub name: String,
    pub start: Beats,
    pub duration: Beats,
}

impl RegionMeta {
    pub fn new(name: String, start: Beats, duration: Beats) -> Self {
        Self {
            name,
            start,
            duration,
        }
    }

    // --- REGION MODIFICATION ---

    pub fn move_region(&mut self, new_start: Beats) {
        self.start = new_start;
    }

    pub fn set_duration(&mut self, new_duration: Beats) {
        self.duration = new_duration;
    }
}
