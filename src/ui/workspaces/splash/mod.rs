mod project_list;
mod splash_controls;
pub(crate) mod state;

use crate::{
    core::metadata::ProjectMeta,
    storage::project::{init_kasl_nodes, load_project_from_dir},
    ui::workspaces::splash::state::SplashUiState,
};
use eframe::egui;
use kadent_engine::{data_types::AudioContext, mixer::Project};
use std::path::PathBuf;

/// The splash screen of Kadent.
#[derive(Default)]
pub struct SplashUi {
    /// The current splash UI state.
    splash_state: SplashUiState,
}

/// A struct that contains the data passed to the editor UI.
pub struct EditorTransition {
    pub project_dir: PathBuf,
    pub audio_ctx: AudioContext,
    pub project: Project,
    pub project_meta: ProjectMeta,
}

impl SplashUi {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<EditorTransition> {
        let mut transition = None;

        egui::Panel::left("splash_controls").show_inside(ui, |ui| {
            transition = self.splash_controls(ui);
        });

        egui::Panel::right("recent_projects").show_inside(ui, |ui| {
            transition = self.project_list(ui);
        });

        transition
    }

    /// Opens the project at the given directory, returning the transition data if successful.
    fn open_project(&mut self, project_dir: PathBuf) -> Option<EditorTransition> {
        match load_project_from_dir(&project_dir) {
            Ok(mut proj_res) => match ProjectMeta::from_load_res(&proj_res) {
                Ok(project_meta) => {
                    init_kasl_nodes(
                        &mut proj_res.project,
                        &project_meta.kasl_search_paths,
                        &project_dir,
                    );

                    let audio_ctx = proj_res.project.audio_ctx.clone();
                    Some(EditorTransition {
                        project_dir,
                        audio_ctx,
                        project: proj_res.project,
                        project_meta,
                    })
                }
                Err(e) => {
                    eprintln!("Failed to extract project metadata: {:?}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("Failed to load project: {:?}", e);
                None
            }
        }
    }
}
