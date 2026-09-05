use serde::{Deserialize, Serialize};

pub const LOD_0_THRESHOLD: f32 = 38.0;
pub const LOD_1_THRESHOLD: f32 = 85.0;
pub const HYSTERESIS_BAND: f32 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SemanticLod {
    /// LOD 0: h < 38 px. Barra de color, hora inicio y título condensado
    Lod0Micro,
    /// LOD 1: 38 px <= h < 85 px. Tríada Esencial: Qué + Cuándo + Dónde
    Lod1Standard,
    /// LOD 2: h >= 85 px. Detalle completo: Tríada + notas + checklists
    Lod2Detailed,
}

impl SemanticLod {
    /// Clasificación directa sin memoria de histéresis
    pub fn from_height(height: f32) -> Self {
        if height < LOD_0_THRESHOLD {
            SemanticLod::Lod0Micro
        } else if height < LOD_1_THRESHOLD {
            SemanticLod::Lod1Standard
        } else {
            SemanticLod::Lod2Detailed
        }
    }

    /// Clasificación con histéresis para evitar flickering en transiciones de zoom (+-3 px)
    pub fn from_height_with_hysteresis(height: f32, previous: Option<Self>) -> Self {
        match previous {
            None => Self::from_height(height),
            Some(prev) => match prev {
                SemanticLod::Lod0Micro => {
                    if height >= LOD_0_THRESHOLD + HYSTERESIS_BAND {
                        if height >= LOD_1_THRESHOLD + HYSTERESIS_BAND {
                            SemanticLod::Lod2Detailed
                        } else {
                            SemanticLod::Lod1Standard
                        }
                    } else {
                        SemanticLod::Lod0Micro
                    }
                }
                SemanticLod::Lod1Standard => {
                    if height < LOD_0_THRESHOLD - HYSTERESIS_BAND {
                        SemanticLod::Lod0Micro
                    } else if height >= LOD_1_THRESHOLD + HYSTERESIS_BAND {
                        SemanticLod::Lod2Detailed
                    } else {
                        SemanticLod::Lod1Standard
                    }
                }
                SemanticLod::Lod2Detailed => {
                    if height < LOD_0_THRESHOLD - HYSTERESIS_BAND {
                        SemanticLod::Lod0Micro
                    } else if height < LOD_1_THRESHOLD - HYSTERESIS_BAND {
                        SemanticLod::Lod1Standard
                    } else {
                        SemanticLod::Lod2Detailed
                    }
                }
            },
        }
    }
}
