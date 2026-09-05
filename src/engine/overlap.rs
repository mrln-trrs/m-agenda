use std::collections::HashMap;
use crate::model::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaneAssignment {
    /// Carril asignado: 0 .. total_lanes - 1
    pub lane_index: usize,
    /// Grado de concurrencia K del clúster de solape
    pub total_lanes: usize,
}

/// Resuelve los solapes de eventos para un conjunto de eventos de un mismo día.
/// Retorna un mapa del ID del evento a su asignación de carril (k, K).
pub fn resolve_day_overlaps(events: &[&Event]) -> HashMap<String, LaneAssignment> {
    let mut result = HashMap::new();
    if events.is_empty() {
        return result;
    }

    // 1. Ordenar eventos por hora de inicio ascendente, y por duración descendente
    let mut sorted_indices: Vec<usize> = (0..events.len()).collect();
    sorted_indices.sort_by(|&a, &b| {
        events[a]
            .time
            .start_min
            .partial_cmp(&events[b].time.start_min)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                events[b]
                    .time
                    .duration()
                    .partial_cmp(&events[a].time.duration())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    // 2. Agrupar en clústeres conexos de eventos que se solapan transitivamente
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    let mut current_cluster: Vec<usize> = Vec::new();
    let mut current_cluster_end = -1.0f32;

    for &idx in &sorted_indices {
        let ev = events[idx];
        if current_cluster.is_empty() {
            current_cluster.push(idx);
            current_cluster_end = ev.time.end_min;
        } else if ev.time.start_min < current_cluster_end {
            // Se solapa con el clúster actual
            current_cluster.push(idx);
            if ev.time.end_min > current_cluster_end {
                current_cluster_end = ev.time.end_min;
            }
        } else {
            // No se solapa: cerramos el clúster actual y abrimos uno nuevo
            clusters.push(std::mem::take(&mut current_cluster));
            current_cluster.push(idx);
            current_cluster_end = ev.time.end_min;
        }
    }

    if !current_cluster.is_empty() {
        clusters.push(current_cluster);
    }

    // 3. Para cada clúster conexo, asignar carriles mediante coloración de intervalos
    for cluster in clusters {
        // lane_end_times guarda el minuto en que cada carril queda libre
        let mut lane_end_times: Vec<f32> = Vec::new();
        let mut cluster_assignments: Vec<(usize, usize)> = Vec::with_capacity(cluster.len());

        for &event_idx in &cluster {
            let ev = events[event_idx];
            let start = ev.time.start_min;
            let end = ev.time.end_min;

            // Buscar el primer carril disponible
            let mut assigned_lane = None;
            for (lane_idx, &lane_end) in lane_end_times.iter().enumerate() {
                if lane_end <= start {
                    assigned_lane = Some(lane_idx);
                    break;
                }
            }

            let lane = match assigned_lane {
                Some(l) => {
                    lane_end_times[l] = end;
                    l
                }
                None => {
                    // Abrir un nuevo carril
                    let l = lane_end_times.len();
                    lane_end_times.push(end);
                    l
                }
            };

            cluster_assignments.push((event_idx, lane));
        }

        let total_lanes = lane_end_times.len().max(1);

        for (event_idx, lane) in cluster_assignments {
            let ev_id = events[event_idx].id.clone();
            result.insert(
                ev_id,
                LaneAssignment {
                    lane_index: lane,
                    total_lanes,
                },
            );
        }
    }

    result
}
