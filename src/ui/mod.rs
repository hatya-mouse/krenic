mod editor;
mod splash;

use crate::{app::EditorUi, colors};
use eframe::{App, egui};
use splash::{SplashTransition, SplashUi};

pub enum KnodiqApp {
    Splash(Box<SplashUi>),
    Editor(Box<EditorUi>),
}

impl KnodiqApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        EditorUi::setup_fonts(&cc.egui_ctx);
        EditorUi::base_style(&cc.egui_ctx);
        KnodiqApp::Splash(Box::new(SplashUi))
    }
}

impl App for KnodiqApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Compute any splash→editor transition before mutating self.
        let transition = if let KnodiqApp::Splash(splash) = self {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .fill(colors::primary_bg(ui.visuals().dark_mode))
                        .inner_margin(0),
                )
                .show_inside(ui, |ui| splash.ui(ui))
                .inner
        } else if let KnodiqApp::Editor(app) = self {
            app.editor_ui(ui, frame);
            None
        } else {
            None
        };

        if let Some(transition) = transition {
            let (project_dir, audio_ctx, project, project_meta) = match transition {
                SplashTransition::NewProject {
                    project_dir,
                    audio_ctx,
                    project,
                    project_meta,
                }
                | SplashTransition::OpenProject {
                    project_dir,
                    audio_ctx,
                    project,
                    project_meta,
                } => (project_dir, audio_ctx, project, project_meta),
            };
            *self = KnodiqApp::Editor(Box::new(EditorUi::new(
                project_dir,
                audio_ctx,
                project,
                project_meta,
            )));
        }
    }
}
