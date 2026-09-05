use eframe::egui;
use m_agenda::ChronovexApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Chronovex - Planificador Semanal Vectorial")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([960.0, 540.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Chronovex",
        native_options,
        Box::new(|cc| Ok(Box::new(ChronovexApp::new(cc)))),
    )
}
