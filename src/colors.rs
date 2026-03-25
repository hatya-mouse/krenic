use eframe::egui::Color32;

pub(crate) fn primary_fg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(230, 230, 230)
    } else {
        Color32::from_rgb(40, 40, 40)
    }
}

pub(crate) fn button_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(70, 70, 70)
    } else {
        Color32::from_rgb(216, 218, 220)
    }
}

pub(crate) fn primary_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(38, 40, 50)
    } else {
        Color32::from_rgb(251, 253, 255)
    }
}

pub(crate) fn secondary_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(33, 35, 41)
    } else {
        Color32::from_rgb(240, 245, 248)
    }
}

pub(crate) fn tertiary_bg(dark_mode: bool) -> Color32 {
    if dark_mode {
        Color32::from_rgb(28, 30, 38)
    } else {
        Color32::from_rgb(234, 236, 238)
    }
}
