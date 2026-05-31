pub(crate) fn midi_note_name(pitch: f32) -> String {
    const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let n = pitch.round() as i32;
    format!("{}{}", NAMES[((n % 12) + 12) as usize % 12], n / 12 - 1)
}
