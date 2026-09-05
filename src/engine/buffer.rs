use crate::model::event::Event;
use crate::model::time_range::TimeRange;
use super::overlap::LaneAssignment;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct BufferArea {
    pub day: u8,
    pub time: TimeRange,
    pub lane_index: usize,
    pub total_lanes: usize,
    pub duration_min: f32,
}

/// Detecta huecos intermedios menores a 30 minutos entre eventos consecutivos del mismo carril
pub fn detect_travel_buffers(
    day: u8,
    events: &[&Event],
    lane_map: &HashMap<String, LaneAssignment>,
    max_gap_minutes: f32,
) -> Vec<BufferArea> {
    let mut buffers = Vec::new();
    if events.len() < 2 {
        return buffers;
    }

    // Agrupar eventos por carril asignado
    let mut lanes: HashMap<usize, Vec<&Event>> = HashMap::new();
    for &ev in events {
        if let Some(assignment) = lane_map.get(&ev.id) {
            lanes.entry(assignment.lane_index).or_default().push(ev);
        }
    }

    // Para cada carril, ordenar cronológicamente y detectar brechas 0 < gap < max_gap_minutes
    for (&lane_idx, lane_events) in lanes.iter_mut() {
        lane_events.sort_by(|a, b| {
            a.time
                .start_min
                .partial_cmp(&b.time.start_min)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for window in lane_events.windows(2) {
            let prev = window[0];
            let next = window[1];
            let gap = next.time.start_min - prev.time.end_min;

            if gap > 0.0 && gap <= max_gap_minutes {
                let total_lanes = lane_map
                    .get(&prev.id)
                    .map(|a| a.total_lanes)
                    .unwrap_or(1);

                buffers.push(BufferArea {
                    day,
                    time: TimeRange::new(prev.time.end_min, next.time.start_min),
                    lane_index: lane_idx,
                    total_lanes,
                    duration_min: gap,
                });
            }
        }
    }

    buffers
}
