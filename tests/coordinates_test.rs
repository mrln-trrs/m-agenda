use m_agenda::engine::TimelineTransform;
use m_agenda::model::TimeRange;
use m_agenda::ui::canvas::GridViewport;

#[test]
fn test_val_03_temporal_coherence_adjacent_blocks() {
    // VAL-03: Si se concatenan un bloque de 45 min (08:00 - 08:45) y uno de 60 min (08:45 - 09:45),
    // no debe existir solapamiento visual ni brecha artificial de píxeles entre ambos bordes vectoriales.
    let transform = TimelineTransform::new(
        60.0,    // x_offset
        1400.0,  // available_width
        50.0,    // y_offset
        900.0,   // viewport_height
        480.0,   // origin_minute (08:00)
        1.2,     // sy (px/min)
    );

    let ev1_range = TimeRange::from_hm(8, 0, 8, 45);   // 45 min
    let ev2_range = TimeRange::from_hm(8, 45, 9, 45);  // 60 min

    let y1_top = transform.time_to_y(ev1_range.start_min);
    let h1 = transform.duration_to_height(ev1_range.duration());
    let y1_bottom = y1_top + h1;

    let y2_top = transform.time_to_y(ev2_range.start_min);
    let h2 = transform.duration_to_height(ev2_range.duration());
    let y2_bottom = y2_top + h2;

    // Contacto matemático exacto: y1_bottom == y2_top
    let delta = (y1_bottom - y2_top).abs();
    assert!(delta < 1e-5, "Debe existir contacto exacto sin brechas ni solapes: delta = {}", delta);

    // Duraciones escaladas exactamente
    assert!((h1 - 45.0 * 1.2).abs() < 1e-5);
    assert!((h2 - 60.0 * 1.2).abs() < 1e-5);
    assert!(((y2_bottom - y1_top) - (45.0 + 60.0) * 1.2).abs() < 1e-4);
}

#[test]
fn test_arbitrary_continuous_minutes_precision() {
    let transform = TimelineTransform::new(0.0, 700.0, 0.0, 1000.0, 0.0, 1.5);

    // Minutos arbitrarios: 08:17 (497.0 min) a 09:02 (542.0 min) = 45 min
    let start_min = 8.0 * 60.0 + 17.0;
    let end_min = 9.0 * 60.0 + 2.0;
    let range = TimeRange::new(start_min, end_min);

    assert_eq!(range.duration(), 45.0);
    assert_eq!(range.format_interval(), "08:17 - 09:02");

    let y = transform.time_to_y(start_min);
    let recovered_min = transform.y_to_time(y);
    assert!((start_min - recovered_min).abs() < 1e-5);
}

#[test]
fn test_horizontal_columns_normalization() {
    let transform = TimelineTransform::new(50.0, 700.0, 0.0, 1000.0, 0.0, 1.0);
    assert_eq!(transform.column_width(), 100.0);

    for d in 0..7 {
        let (left, right) = transform.day_column_x(d);
        assert_eq!(left, 50.0 + (d as f32) * 100.0);
        assert_eq!(right, left + 100.0);
        assert_eq!(transform.x_to_day(left + 50.0), Some(d));
    }
}

#[test]
fn test_dynamic_max_zoom_out_limit() {
    let mut viewport = GridViewport::new();
    let useful_height = 800.0f32;

    // min_sy para 800 px debe ser exactamente 800 / 1440
    let expected_min_sy = 800.0 / 1440.0;
    assert!((viewport.min_sy(useful_height) - expected_min_sy).abs() < 1e-5);

    // Si intentamos hacer zoom-out extremo (factor 0.01)
    viewport.zoom_at(400.0, 0.01, 0.0, useful_height);

    // Debe quedar clampado exactamente a min_sy
    assert!((viewport.sy - expected_min_sy).abs() < 1e-5);
    // Y el origen debe estar en 0.0 (00:00)
    assert_eq!(viewport.origin_minute, 0.0);

    // Intentar panear hacia arriba (minutos negativos)
    viewport.pan(-200.0, useful_height);
    assert_eq!(viewport.origin_minute, 0.0);

    // Intentar panear hacia abajo (más allá de medianoche)
    viewport.pan(200.0, useful_height);
    assert_eq!(viewport.origin_minute, 0.0); // Con min_sy, max_origin es 1440 - 1440 = 0.0
}
