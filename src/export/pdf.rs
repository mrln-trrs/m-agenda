use printpdf::{
    BuiltinFont, Color, Line, LinePoint, Mm, Op, PaintMode, PdfDocument, PdfFontHandle, PdfPage,
    PdfSaveOptions, Point, Polygon, PolygonRing, Pt, Rgb, TextItem, WindingOrder,
};
use crate::model::AgendaModel;
use crate::engine::{resolve_day_overlaps, SemanticLod};
use super::auto_frame::ExportFraming;
use super::svg::DAY_NAMES;

pub struct PdfExportOptions {
    pub dark_mode: bool,
    pub title: String,
}

impl Default for PdfExportOptions {
    fn default() -> Self {
        Self {
            dark_mode: false,
            title: "Chronovex Semanal 16:9".into(),
        }
    }
}

#[inline]
fn pt_point(x: f32, y: f32) -> Point {
    Point { x: Pt(x), y: Pt(y) }
}

#[inline]
fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::Rgb(Rgb::new(r, g, b, None))
}

/// Recorta a `max` caracteres sin partir un carácter UTF-8.
fn truncate_chars(text: &str, max: usize) -> &str {
    match text.char_indices().nth(max) {
        Some((idx, _)) => &text[..idx],
        None => text,
    }
}

/// Acumula las operaciones de dibujo de una página (printpdf ≥ 0.8 construye páginas con `Op`).
struct Canvas {
    ops: Vec<Op>,
}

impl Canvas {
    fn new() -> Self {
        Self { ops: Vec::new() }
    }

    fn set_fill_color(&mut self, col: Color) {
        self.ops.push(Op::SetFillColor { col });
    }

    fn set_outline_color(&mut self, col: Color) {
        self.ops.push(Op::SetOutlineColor { col });
    }

    fn set_outline_thickness(&mut self, pt: f32) {
        self.ops.push(Op::SetOutlineThickness { pt: Pt(pt) });
    }

    /// Rectángulo entre `(x0, y0)` (abajo a la izquierda) y `(x1, y1)` (arriba a la derecha), en pt de PDF.
    fn add_rect(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, mode: PaintMode) {
        let points = [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
            .iter()
            .map(|&(x, y)| LinePoint { p: pt_point(x, y), bezier: false })
            .collect();
        self.ops.push(Op::DrawPolygon {
            polygon: Polygon {
                rings: vec![PolygonRing { points }],
                mode,
                winding_order: WindingOrder::NonZero,
            },
        });
    }

    fn add_line(&mut self, from: (f32, f32), to: (f32, f32)) {
        self.ops.push(Op::DrawLine {
            line: Line {
                points: vec![
                    LinePoint { p: pt_point(from.0, from.1), bezier: false },
                    LinePoint { p: pt_point(to.0, to.1), bezier: false },
                ],
                is_closed: false,
            },
        });
    }

    /// Escribe `text` con la fuente estándar indicada; usa el color de relleno vigente.
    fn use_text(&mut self, text: &str, size: f32, x: f32, y: f32, font: BuiltinFont) {
        self.ops.extend([
            Op::StartTextSection,
            Op::SetFont { font: PdfFontHandle::Builtin(font), size: Pt(size) },
            Op::SetTextCursor { pos: pt_point(x, y) },
            Op::ShowText { items: vec![TextItem::Text(text.to_string())] },
            Op::EndTextSection,
        ]);
    }
}

/// Genera un documento PDF vectorial puro con printpdf (VAL-02: resolución infinita, cero bitmaps)
pub fn generate_pdf_16_9(model: &AgendaModel, options: &PdfExportOptions) -> Result<Vec<u8>, String> {
    let width_pt = 1920.0f32;
    let height_pt = 1080.0f32;

    let font_bold = BuiltinFont::HelveticaBold;
    let font_regular = BuiltinFont::Helvetica;
    let mut c = Canvas::new();

    let framing = ExportFraming::compute(model, 30.0);
    let transform = &framing.transform;

    // En PDF el origen (0, 0) está en la esquina inferior izquierda.
    // Función de conversión de Y de pantalla (origen arriba) a Y de PDF (origen abajo)
    let to_pdf_y = |screen_y: f32| -> f32 {
        height_pt - screen_y
    };

    // Fondo de la lámina
    let (bg_color, card_bg, text_primary, text_secondary, grid_color) = if options.dark_mode {
        (
            rgb(15.0 / 255.0, 23.0 / 255.0, 42.0 / 255.0),
            rgb(30.0 / 255.0, 41.0 / 255.0, 59.0 / 255.0),
            rgb(248.0 / 255.0, 250.0 / 255.0, 252.0 / 255.0),
            rgb(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0),
            rgb(51.0 / 255.0, 65.0 / 255.0, 85.0 / 255.0),
        )
    } else {
        (
            rgb(248.0 / 255.0, 250.0 / 255.0, 252.0 / 255.0),
            rgb(1.0, 1.0, 1.0),
            rgb(15.0 / 255.0, 23.0 / 255.0, 42.0 / 255.0),
            rgb(100.0 / 255.0, 116.0 / 255.0, 139.0 / 255.0),
            rgb(203.0 / 255.0, 213.0 / 255.0, 225.0 / 255.0),
        )
    };

    c.set_fill_color(bg_color);
    c.add_rect(0.0, 0.0, width_pt, height_pt, PaintMode::Fill);

    // 1. Cabeceras de días y separadores de columnas
    let w_col = transform.column_width();
    for d in 0..7 {
        let (x_left, x_right) = transform.day_column_x(d);

        // Caja de cabecera
        c.set_fill_color(card_bg.clone());
        c.set_outline_color(grid_color.clone());
        c.set_outline_thickness(1.0);
        let header_top_y = to_pdf_y(12.0);
        let header_bottom_y = to_pdf_y(60.0);
        c.add_rect(x_left + 2.0, header_bottom_y, x_right - 2.0, header_top_y, PaintMode::FillStroke);

        // Texto de cabecera
        c.set_fill_color(text_primary.clone());
        c.use_text(
            DAY_NAMES[d as usize],
            14.0,
            x_left + (w_col * 0.3),
            to_pdf_y(40.0),
            font_bold,
        );

        // Línea vertical separadora
        c.set_outline_color(grid_color.clone());
        c.set_outline_thickness(0.8);
        c.add_line(
            (x_left, to_pdf_y(height_pt - 35.0)),
            (x_left, to_pdf_y(75.0)),
        );
    }

    // Línea final vertical
    let (_, x_domingo_right) = transform.day_column_x(6);
    c.add_line(
        (x_domingo_right, to_pdf_y(height_pt - 35.0)),
        (x_domingo_right, to_pdf_y(75.0)),
    );

    // 2. Grilla horizontal y etiquetas horarias
    let first_hour = (framing.t_min / 60.0).floor() as i32;
    let last_hour = (framing.t_max / 60.0).ceil() as i32;

    for h in first_hour..=last_hour {
        let minute = (h * 60) as f32;
        let y_screen = transform.time_to_y(minute);
        if y_screen >= 75.0 && y_screen <= height_pt - 30.0 {
            let y_pdf = to_pdf_y(y_screen);
            c.set_outline_color(grid_color.clone());
            c.set_outline_thickness(1.0);
            c.add_line((95.0, y_pdf), (x_domingo_right, y_pdf));

            let label = format!("{:02}:00", h % 24);
            c.set_fill_color(text_secondary.clone());
            c.use_text(&label, 12.0, 40.0, y_pdf - 4.0, font_bold);
        }
    }

    // 3. Renderizado de tarjetas de eventos vectoriales
    for d in 0..7 {
        let day_events = model.events_for_day(d);
        if day_events.is_empty() {
            continue;
        }

        let lane_assignments = resolve_day_overlaps(&day_events);

        for ev in day_events {
            let assign = lane_assignments.get(&ev.id).copied().unwrap_or(crate::engine::LaneAssignment {
                lane_index: 0,
                total_lanes: 1,
            });

            let (ex, ey, ew, eh) = transform.compute_event_box(
                d,
                ev.time.start_min,
                ev.time.duration(),
                assign.lane_index,
                assign.total_lanes,
                3.0,
            );

            let y_top_pdf = to_pdf_y(ey);
            let y_bottom_pdf = to_pdf_y(ey + eh);

            let [r, g, b] = ev.color_rgb;
            let accent_color = rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

            // Rectángulo de tarjeta
            c.set_fill_color(card_bg.clone());
            c.set_outline_color(accent_color.clone());
            c.set_outline_thickness(1.5);
            c.add_rect(ex, y_bottom_pdf, ex + ew, y_top_pdf, PaintMode::FillStroke);

            // Barra vertical de acento (5 pt ancho)
            c.set_fill_color(accent_color.clone());
            c.add_rect(ex, y_bottom_pdf, ex + 5.0, y_top_pdf, PaintMode::Fill);

            // Textos según LOD
            let lod = SemanticLod::from_height(eh);
            let content_x = ex + 10.0;

            match lod {
                SemanticLod::Lod0Micro => {
                    c.set_fill_color(text_primary.clone());
                    let label = format!("{} {}", ev.time.format_interval(), ev.title);
                    c.use_text(truncate_chars(&label, 24), 10.0, content_x, y_top_pdf - 12.0, font_bold);
                }
                SemanticLod::Lod1Standard | SemanticLod::Lod2Detailed => {
                    // Tríada Esencial: CUÁNDO, QUÉ, DÓNDE
                    // Renglón 1: Cuándo
                    c.set_fill_color(text_secondary.clone());
                    c.use_text(&ev.when_text(), 10.0, content_x, y_top_pdf - 14.0, font_bold);

                    // Renglón 2: Qué
                    c.set_fill_color(text_primary.clone());
                    c.use_text(truncate_chars(&ev.title, 30), 12.0, content_x, y_top_pdf - 28.0, font_bold);

                    // Renglón 3: Dónde
                    if eh >= 48.0 {
                        c.set_fill_color(text_secondary.clone());
                        c.use_text(truncate_chars(&ev.location, 32), 10.0, content_x, y_top_pdf - 42.0, font_regular);
                    }
                }
            }
        }
    }

    let page = PdfPage::new(Mm::from(Pt(width_pt)), Mm::from(Pt(height_pt)), c.ops);
    let mut doc = PdfDocument::new(&options.title);
    let mut warnings = Vec::new();
    let pdf_bytes = doc
        .with_pages(vec![page])
        .save(&PdfSaveOptions::default(), &mut warnings);

    if pdf_bytes.is_empty() {
        return Err(format!("Error guardando PDF: {:?}", warnings));
    }

    Ok(pdf_bytes)
}
