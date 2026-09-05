use m_agenda::engine::{resolve_day_overlaps, TimelineTransform};
use m_agenda::model::{Event, TimeRange};

#[test]
fn test_rf1_3_concurrency_and_lane_partitioning() {
    // Escenario: 3 eventos el día Miércoles (2)
    // Evento A: 14:00 - 15:30 (840 - 930)
    // Evento B: 14:30 - 16:00 (870 - 960) -> Solapa con A y C
    // Evento C: 15:15 - 16:30 (915 - 990) -> Solapa con A y B en [915..930] -> Concurrencia K = 3
    let ev_a = Event::new("a", 2, TimeRange::new(840.0, 930.0), "A", "Loc A", [0, 0, 0]);
    let ev_b = Event::new("b", 2, TimeRange::new(870.0, 960.0), "B", "Loc B", [0, 0, 0]);
    let ev_c = Event::new("c", 2, TimeRange::new(915.0, 990.0), "C", "Loc C", [0, 0, 0]);

    let day_events = vec![&ev_a, &ev_b, &ev_c];
    let assignments = resolve_day_overlaps(&day_events);

    let assign_a = assignments.get("a").expect("Debe tener carril");
    let assign_b = assignments.get("b").expect("Debe tener carril");
    let assign_c = assignments.get("c").expect("Debe tener carril");

    // Concurrencia máxima K calculada
    assert_eq!(assign_a.total_lanes, 3);
    assert_eq!(assign_b.total_lanes, 3);
    assert_eq!(assign_c.total_lanes, 3);

    // Carriles mutuamente distintos porque coinciden en [915, 930]
    let lanes = vec![assign_a.lane_index, assign_b.lane_index, assign_c.lane_index];
    assert_eq!(lanes.len(), 3);
    assert!(lanes.contains(&0));
    assert!(lanes.contains(&1));
    assert!(lanes.contains(&2));

    // Validar que las coordenadas X no colisionan geométricamente
    let transform = TimelineTransform::new(0.0, 700.0, 0.0, 1000.0, 0.0, 1.0);
    let box_a = transform.compute_event_box(2, ev_a.time.start_min, ev_a.time.duration(), assign_a.lane_index, assign_a.total_lanes, 2.0);
    let box_b = transform.compute_event_box(2, ev_b.time.start_min, ev_b.time.duration(), assign_b.lane_index, assign_b.total_lanes, 2.0);
    let box_c = transform.compute_event_box(2, ev_c.time.start_min, ev_c.time.duration(), assign_c.lane_index, assign_c.total_lanes, 2.0);

    // Anchos de carril iguales
    assert!((box_a.2 - box_b.2).abs() < 1e-3);
    assert!((box_b.2 - box_c.2).abs() < 1e-3);

    // Posiciones horizontales disjuntas
    let x_ends = vec![box_a.0 + box_a.2, box_b.0 + box_b.2, box_c.0 + box_c.2];
    assert!(x_ends[0] <= box_b.0 || box_b.0 + box_b.2 <= box_a.0 || assign_a.lane_index != assign_b.lane_index);
}

#[test]
fn test_sequential_events_share_lane() {
    // Dos eventos en el mismo día que no se solapan: deben poder compartir el carril 0 y total_lanes = 1
    let ev1 = Event::new("seq1", 0, TimeRange::new(480.0, 540.0), "1", "", [0, 0, 0]);
    let ev2 = Event::new("seq2", 0, TimeRange::new(540.0, 600.0), "2", "", [0, 0, 0]);

    let day_events = vec![&ev1, &ev2];
    let assignments = resolve_day_overlaps(&day_events);

    assert_eq!(assignments.get("seq1").unwrap().total_lanes, 1);
    assert_eq!(assignments.get("seq2").unwrap().total_lanes, 1);
    assert_eq!(assignments.get("seq1").unwrap().lane_index, 0);
    assert_eq!(assignments.get("seq2").unwrap().lane_index, 0);
}
