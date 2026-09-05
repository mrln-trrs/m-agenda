use egui::{Context, ViewportBuilder, ViewportId};
use crate::config::{AgendaConfig, ThemeMode};

pub fn show_settings_viewport(
    ctx: &Context,
    open: &mut bool,
    config: &mut AgendaConfig,
) {
    if !*open {
        return;
    }

    let viewport_id = ViewportId::from_hash_of("chronovex_settings_window");
    let builder = ViewportBuilder::default()
        .with_title("Chronovex - Opciones y Configuración")
        .with_inner_size([440.0, 520.0])
        .with_resizable(true);

    ctx.show_viewport_immediate(viewport_id, builder, |ctx, _class| {
        // Si el usuario hace clic en la X de la ventana nativa del SO
        if ctx.input(|i| i.viewport().close_requested()) {
            *open = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading("⚙ Configuración de Chronovex");
            ui.label("Esta ventana es independiente del SO: puedes moverla o minimizarla libremente.");
            ui.separator();

            ui.add_space(8.0);
            ui.label(egui::RichText::new("🎨 Apariencia y Contraste (WCAG 2.1 AA)").strong());
            ui.horizontal(|ui| {
                ui.selectable_value(&mut config.theme_mode, ThemeMode::Dark, "🌙 Oscuro");
                ui.selectable_value(&mut config.theme_mode, ThemeMode::Light, "☀️ Claro");
                ui.selectable_value(&mut config.theme_mode, ThemeMode::HighContrast, "👁 Alto Contraste");
            });

            ui.add_space(12.0);
            ui.separator();
            ui.label(egui::RichText::new("📐 Espacio Continuo & Diagramación").strong());

            ui.checkbox(
                &mut config.show_travel_buffers,
                "Graficar áreas de descanso / traslado (< 30 min)",
            );

            ui.checkbox(
                &mut config.use_lod_hysteresis,
                "Banda de histéresis anti-parpadeo (±3 px en LOD)",
            );

            ui.checkbox(
                &mut config.live_preview,
                "⚡ Reactividad en tiempo real (aplicar al instante a la agenda)",
            );

            ui.add_space(8.0);
            ui.label("Alineación / Snapping al crear bloques:");
            ui.horizontal(|ui| {
                ui.radio_value(&mut config.snap_minutes, 0.0, "Continuo puro (1 min)");
                ui.radio_value(&mut config.snap_minutes, 5.0, "5 min");
                ui.radio_value(&mut config.snap_minutes, 15.0, "15 min");
            });

            ui.add_space(12.0);
            ui.separator();
            ui.label(egui::RichText::new("🔍 Escala Tipográfica").strong());
            ui.add(egui::Slider::new(&mut config.font_scale, 0.8..=1.4).text("Multiplicador de texto"));

            ui.add_space(20.0);
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Restablecer Valores").clicked() {
                    *config = AgendaConfig::default();
                }
                if ui.button("Cerrar Ventana").clicked() {
                    *open = false;
                }
            });
        });
    });
}
