mod app;
pub(crate) mod colors;
pub(crate) mod components;
mod ui;

use app::KnodiqApp;
use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Knodiq",
        options,
        Box::new(|cc| Ok(Box::new(KnodiqApp::new(cc)))),
    )
}
