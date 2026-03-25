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
}
