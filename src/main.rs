use egui::{Window, Context, DemoApp};
use crate::app::EmbroideryApp;
use egui_glium::storage;
use tokio::io::Error;

#[macro_use]
extern crate nom;

mod app;
mod pes;

type Unexpected<T> = Result<T, Error>;

#[tokio::main]
pub async fn main() -> Unexpected<()> {
    let title = "Embroidery Editor";

    let storage = storage::FileStorage::from_path(".embroidery_editor.json".to_string());
    let app: EmbroideryApp = egui::app::get_value(&storage, egui::app::APP_KEY).unwrap_or_default();
    egui_glium::run(title, Box::new(storage), app);
    Ok(())
}

