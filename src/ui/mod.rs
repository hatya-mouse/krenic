mod toolbar;

use crate::app::KnodiqApp;
use eframe::{App, egui};

impl App for KnodiqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar")
            .exact_height(44.0)
            .show(ctx, |ui| {
                self.toolbar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Knodiq");
        });
    }
}
