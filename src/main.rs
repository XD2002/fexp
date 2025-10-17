use eframe::{self};
use app::FExpApp;

pub mod file_ops;
pub mod navigation;
pub mod icon;
pub mod sort;
pub mod app;
pub mod conversion;
pub mod config;
pub mod settings;
pub mod search;

fn main() -> Result<(), eframe::Error> {
    // Define native options for the application
    let native_options = eframe::NativeOptions::default();
    // Run the application
    eframe::run_native(
        "File explorer",
        native_options,
        Box::new(|_cc| Ok(Box::new(FExpApp::default()))),
    )
}
