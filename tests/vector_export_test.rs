use m_agenda::model::AgendaModel;
use m_agenda::export::{generate_svg_16_9, generate_pdf_16_9, SvgExportOptions, PdfExportOptions};

#[test]
fn test_pure_vector_svg_export() {
    let model = AgendaModel::create_sample_week();
    let svg = generate_svg_16_9(&model, &SvgExportOptions::default());

    // Validar encabezados y proporciones 16:9
    assert!(svg.contains(r#"viewBox="0 0 1920 1080""#));
    assert!(svg.contains(r#"width="1920""#));
    assert!(svg.contains(r#"height="1080""#));

    // Validar presencia de días y Tríada Esencial
    assert!(svg.contains("LUNES"));
    assert!(svg.contains("DOMINGO"));
    assert!(svg.contains("Arquitectura del Compilador")); // Qué
    assert!(svg.contains("08:15 - 09:00"));               // Cuándo
    assert!(svg.contains("Aula Magna 301"));             // Dónde

    // Validar clipPath y ausencia de tags de imágenes rasterizadas (VAL-02)
    assert!(svg.contains("<clipPath id="));
    assert!(!svg.contains("<image"), "El SVG vectorial puro no debe contener tags <image>");
}

#[test]
fn test_val_02_pure_vector_pdf_export() {
    let model = AgendaModel::create_sample_week();
    let pdf_bytes = generate_pdf_16_9(&model, &PdfExportOptions::default())
        .expect("La generación de PDF vectorial debe ser exitosa");

    // Verificar cabecera estándar de PDF
    assert!(pdf_bytes.starts_with(b"%PDF-1."), "Debe tener cabecera mágica de PDF válida");
    assert!(pdf_bytes.len() > 1000, "El PDF generado debe tener contenido vectorial");

    // Validar que no contiene objetos de tipo Image (cero rasterizado, pura resolución vectorial)
    let pdf_text = String::from_utf8_lossy(&pdf_bytes);
    assert!(!pdf_text.contains("/Subtype /Image"), "El PDF vectorial puro NO debe contener /Subtype /Image rasterizado");
}
