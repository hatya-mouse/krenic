use crate::{
    app::KnodiqApp,
    components::icon_button::icon_button,
    load_write::{load_project, save_project},
    metadata::ProjectMeta,
    ui::toolbar::toolbar_group::toolbar_group,
};
use eframe::egui;

impl KnodiqApp {
    pub(super) fn file_control(&mut self, ui: &mut egui::Ui) {
        toolbar_group(ui, |ui| {
            if icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../assets/icons/save.svg")),
            )
            .clicked()
            {
                let files = rfd::FileDialog::new()
                    .add_filter("text", &["kasl"])
                    .save_file();

                if let Some(path) = files {
                    match save_project(&path, &self.project, &self.project_meta) {
                        Ok(()) => (),
                        Err(e) => {
                            eprintln!("Failed to save project: {:?}", e);
                        }
                    }
                }
            }

            if icon_button(
                ui,
                egui::Image::new(egui::include_image!("../../../assets/icons/open.svg")),
            )
            .clicked()
            {
                let files = rfd::FileDialog::new()
                    .add_filter("text", &["kasl"])
                    .pick_file();

                if let Some(path) = files {
                    match load_project(&path) {
                        Ok(proj_res) => {
                            self.project_meta = ProjectMeta::from_load_res(&proj_res);
                            self.project = proj_res.project;
                            println!("New Project Meta: {:#?}", self.project_meta);
                            self.update_project();
                        }
                        Err(e) => {
                            eprintln!("Failed to load project: {:?}", e);
                        }
                    }
                }
            }
        });
    }
}
