use m_agenda::engine::{SemanticLod, LOD_0_THRESHOLD};

#[test]
fn test_val_01_invariant_triad_in_45_min_block() {
    // VAL-01: En lámina completa 16:9 que cubre hasta 16 horas de jornada (960 min),
    // con una altura útil de 960 px en 1080p, Sy = 960 / 960 = 1.0 px/min.
    // Para un bloque de 45 min, h = 45 * 1.0 = 45 px >= 38 px (LOD 1).
    let sy = 1.0f32;
    let duration_45 = 45.0f32;
    let height = duration_45 * sy;

    assert!(height >= LOD_0_THRESHOLD, "La altura ({}) debe ser >= umbral LOD 1 (38 px)", height);
    assert_eq!(SemanticLod::from_height(height), SemanticLod::Lod1Standard);
}

#[test]
fn test_val_04_micro_event_degradation() {
    // VAL-04: Si existe un evento menor a 30 minutos (ej. 15 min),
    // con Sy = 1.0 px/min, h = 15 px < 38 px, se degrada correctamente a LOD 0
    let sy = 1.0f32;
    let duration_15 = 15.0f32;
    let height = duration_15 * sy;

    assert!(height < LOD_0_THRESHOLD);
    assert_eq!(SemanticLod::from_height(height), SemanticLod::Lod0Micro);
}

#[test]
fn test_rnf3_2_lod_hysteresis_deadband() {
    // Banda muerta de +- 3 px en límites
    let prev = SemanticLod::Lod0Micro;
    // Si sube a 39 px (justo arriba de 38, pero dentro de 38 + 3 = 41), se mantiene en Lod0
    assert_eq!(
        SemanticLod::from_height_with_hysteresis(39.0, Some(prev)),
        SemanticLod::Lod0Micro
    );
    // Al superar 41 px, transiciona a Lod1
    assert_eq!(
        SemanticLod::from_height_with_hysteresis(41.5, Some(prev)),
        SemanticLod::Lod1Standard
    );

    // Ahora estando en Lod1:
    let prev = SemanticLod::Lod1Standard;
    // Si baja a 36 px (debajo de 38, pero arriba de 38 - 3 = 35), se mantiene en Lod1
    assert_eq!(
        SemanticLod::from_height_with_hysteresis(36.0, Some(prev)),
        SemanticLod::Lod1Standard
    );
    // Al caer debajo de 35 px, transiciona a Lod0
    assert_eq!(
        SemanticLod::from_height_with_hysteresis(34.0, Some(prev)),
        SemanticLod::Lod0Micro
    );
}
