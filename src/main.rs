mod app;
mod generator;
mod types;
mod ui;

use app::QrApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([980.0, 720.0])
            .with_min_inner_size([760.0, 540.0])
            .with_title("QRust — Studio QR Code Generator"),
        ..Default::default()
    };

    eframe::run_native(
        "QRust",
        native_options,
        Box::new(|cc| Ok(Box::new(QrApp::new(cc)))),
    )
}
