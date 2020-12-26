use egui::app::{App, Storage, IntegrationContext};
use egui::{Context, Ui, Window, Id, TopPanel, menu, Hyperlink, Label, TextStyle, Layout, Align};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use nfd2::{open_file_dialog, Response, open_save_dialog};
use crate::pes::{PesData, PesFile};
use std::fs;
use chrono::{DateTime, Local};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EmbroideryApp {
    #[serde(skip)]
    files: Vec<PesFile>,
    #[serde(skip)]
    curr_pes: Option<PesFile>
}

impl EmbroideryApp {

    fn show_tools(&self, ui: &mut Ui, integration_context: &mut IntegrationContext<'_>) {
        if ui.button("Select").clicked {

        }

        if ui.button("Copy").clicked {

        }

        if ui.button("Cut").clicked {

        }

        if ui.button("Paste").clicked {

        }
    }

    fn show_open_projects(&mut self, ui: &mut Ui, integration_context: &mut IntegrationContext<'_>) {
        menu::bar(ui, |ui| {
            for file in &self.files {
                if ui.button(&*file.file_path.file_name().unwrap().to_str().unwrap()).clicked {
                    self.curr_pes = Some(file.clone());
                }
            }
        });

        ui.separator();

        if let Some(pes) = &self.curr_pes {
            ui.label(&*format!("Identification: {}", String::from_utf8(pes.pes_data.identification.clone()).unwrap()));
            ui.label(&*format!("Version: {}", String::from_utf8(pes.pes_data.version.clone()).unwrap()));
            ui.separator();

            ui.label(&*format!("PEC Section: {:?}", pes.pes_data.pec_section.clone()));
            ui.label(&*format!("Hoop Size: {}", pes.pes_data.hoop_size_indicator));
            ui.label(&*format!("Subversion: {}", String::from_utf8(pes.pes_data.subversion.clone()).unwrap()));

            ui.separator();
            ui.label(&*format!("Design: {}", String::from_utf8(pes.pes_data.description.design.clone()).unwrap()));
            ui.label(&*format!("Author: {}", String::from_utf8(pes.pes_data.description.author.clone()).unwrap()));
            ui.label(&*format!("Category: {}", String::from_utf8(pes.pes_data.description.category.clone()).unwrap()));
            ui.label(&*format!("Keywords: {}", String::from_utf8(pes.pes_data.description.keywords.clone()).unwrap()));
            ui.label(&*format!("Comments: {}", String::from_utf8(pes.pes_data.description.comments.clone()).unwrap()));

            ui.separator();
            ui.label(&*format!("Debug: {:?}", pes.pes_data));

        }


    }

    fn show_menu_bar(&mut self, ui: &mut Ui) {
        menu::bar(ui, |ui| {
            menu::menu(ui, "File", |ui| {
                if ui.button("Load file").on_hover_text("Load file from disk").clicked {
                    match open_file_dialog(None, None).expect("oh no") {
                        Response::Okay(file_path) => {
                            println!("File path = {:?}", file_path);
                            let byts = &*fs::read(file_path.clone()).unwrap();
                            let (left, pes): (&[u8], PesData) = PesData::parse(byts).unwrap();
                            self.files.push(PesFile { file_path, pes_data: pes });
                        },
                        Response::OkayMultiple(files) => println!("Files {:?}", files),
                        Response::Cancel => println!("User canceled"),
                    }
                }

                if ui.button("Save file").on_hover_text("Save file to disk").clicked {
                    match open_save_dialog(None, None).expect("oh no") {
                        Response::Okay(file_path) => {
                            println!("File path = {:?}", file_path);

                        },
                        Response::OkayMultiple(files) => println!("Files {:?}", files),
                        Response::Cancel => println!("User canceled"),
                    }
                }

                if ui.button("Insert file").on_hover_text("Insert file into current project").clicked {
                    match open_file_dialog(None, None).expect("oh no") {
                        Response::Okay(file_path) => {
                            println!("File path = {:?}", file_path);

                        },
                        Response::OkayMultiple(files) => println!("Files {:?}", files),
                        Response::Cancel => println!("User canceled"),
                    }
                }

                if ui.button("Organize windows").clicked {
                    ui.ctx().memory().reset_areas();
                }

                if ui.button("Clear memory").on_hover_text("Forget scroll, collapsing headers etc").clicked {
                    *ui.ctx().memory() = Default::default();
                }
            });

            menu::menu(ui, "About", |ui| {
                ui.label("Embroidery Editor");
                ui.label("Copyright 2020");
                ui.label("by Marco");
                ui.add(Hyperlink::new("https://embroidery.editor/").text("Homepage"));
            });

            let local: DateTime<Local> = Local::now();
            ui.with_layout(Layout::horizontal(Align::Center).reverse(), |ui| {
                ui.add(Label::new(local.to_rfc2822()).text_style(TextStyle::Monospace));
            });
        });
    }

}

impl App for EmbroideryApp {

    fn ui(&mut self, ctx: &Arc<Context>, integration_context: &mut IntegrationContext<'_>) {
        TopPanel::top(Id::new("menu_bar")).show(ctx, |ui| {
            self.show_menu_bar(ui);
        });

        Window::new("Tools")
            .min_width(320.0)
            .scroll(true)
            .show(ctx, |ui| {
                self.show_tools(ui, integration_context);
            });

        Window::new("Projects")
            .min_width(360.0)
            .scroll(true)
            .resizable(true)
            .show(ctx, |ui| {
                self.show_open_projects(ui, integration_context);
            });
    }

    fn on_exit(&mut self, _storage: &mut dyn Storage) {
        egui::app::set_value(_storage, egui::app::APP_KEY, self);
    }

}