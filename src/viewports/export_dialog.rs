use egui::{Context, Window};
use crate::export::{
    generate_pdf_16_9, generate_svg_16_9, PdfExportOptions, SvgExportOptions,
};
use crate::model::AgendaModel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    PdfVector,
    Svg16x9,
}

pub struct ExportDialogState {
    pub is_open: bool,
    pub format: ExportFormat,
    pub dark_mode: bool,
    pub show_buffers: bool,
    pub status_message: Option<String>,
}

impl Default for ExportDialogState {
    fn default() -> Self {
        Self {
            is_open: false,
            format: ExportFormat::PdfVector,
            dark_mode: false, // Default claro para impresión/documento
            show_buffers: true,
            status_message: None,
        }
    }
}

pub fn show_export_dialog(
    ctx: &Context,
    state: &mut ExportDialogState,
    model: &AgendaModel,
) {
    if !state.is_open {
        return;
    }

    let mut is_open = state.is_open;
    Window::new("📄 Exportar Lámina Semanal 16:9")
        .open(&mut is_open)
        .collapsible(false)
        .resizable(false)
        .default_size([420.0, 320.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading("Exportación Vectorial Pura (Resolución Infinita)");
            ui.label("La salida conserva vectores nativos y tipografías embebidas (cero mapas de bits).");
            ui.separator();

            ui.add_space(8.0);
            ui.label(egui::RichText::new("Formato de Salida:").strong());
            ui.horizontal(|ui| {
                ui.radio_value(&mut state.format, ExportFormat::PdfVector, "PDF Vectorial Puro (.pdf)");
                ui.radio_value(&mut state.format, ExportFormat::Svg16x9, "SVG 16:9 Nominal (.svg)");
            });

            ui.add_space(8.0);
            ui.label(egui::RichText::new("Opciones de Renderizado:").strong());
            ui.checkbox(&mut state.dark_mode, "Exportar con Tema Oscuro");
            ui.checkbox(&mut state.show_buffers, "Incluir áreas achuradas de traslado (< 30 min)");

            ui.add_space(12.0);
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("💾 Guardar Archivo...").clicked() {
                    match state.format {
                        ExportFormat::PdfVector => {
                            let file_opt = rfd::FileDialog::new()
                                .add_filter("Documento PDF", &["pdf"])
                                .set_file_name("agenda_semanal_16x9.pdf")
                                .save_file();

                            if let Some(path) = file_opt {
                                let opts = PdfExportOptions {
                                    dark_mode: state.dark_mode,
                                    title: "Agenda Semanal Chronovex 16:9".into(),
                                };
                                match generate_pdf_16_9(model, &opts) {
                                    Ok(bytes) => {
                                        if let Err(e) = std::fs::write(&path, bytes) {
                                            state.status_message = Some(format!("Error escribiendo archivo: {}", e));
                                        } else {
                                            state.status_message = Some(format!("¡PDF vectorial guardado con éxito en {:?}!", path.file_name().unwrap()));
                                        }
                                    }
                                    Err(e) => {
                                        state.status_message = Some(format!("Error generando PDF: {}", e));
                                    }
                                }
                            }
                        }
                        ExportFormat::Svg16x9 => {
                            let file_opt = rfd::FileDialog::new()
                                .add_filter("Gráfico Vectorial SVG", &["svg"])
                                .set_file_name("agenda_semanal_16x9.svg")
                                .save_file();

                            if let Some(path) = file_opt {
                                let opts = SvgExportOptions {
                                    dark_mode: state.dark_mode,
                                    show_travel_buffers: state.show_buffers,
                                    title: "Agenda Semanal Chronovex 16:9".into(),
                                };
                                let svg_content = generate_svg_16_9(model, &opts);
                                if let Err(e) = std::fs::write(&path, svg_content) {
                                    state.status_message = Some(format!("Error escribiendo archivo: {}", e));
                                } else {
                                    state.status_message = Some(format!("¡SVG 16:9 guardado con éxito en {:?}!", path.file_name().unwrap()));
                                }
                            }
                        }
                    }
                }

                if ui.button("Cerrar").clicked() {
                    state.is_open = false;
                }
            });

            if let Some(ref msg) = state.status_message {
                ui.add_space(8.0);
                ui.colored_label(egui::Color32::from_rgb(34, 197, 94), msg);
            }
        });

    state.is_open = is_open;
}
