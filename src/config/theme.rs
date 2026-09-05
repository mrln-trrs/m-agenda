use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
    HighContrast,
}

#[derive(Debug, Clone)]
pub struct AppTheme {
    pub bg_canvas: Color32,
    pub card_bg: Color32,
    pub card_border: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub grid_macro: Color32,
    pub grid_mid: Color32,
    pub grid_micro: Color32,
    pub time_label: Color32,
    pub selection: Color32,
    pub buffer_hatch: Color32,
}

impl AppTheme {
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self {
                bg_canvas: Color32::from_rgb(15, 23, 42),       // Slate 900
                card_bg: Color32::from_rgb(30, 41, 59),         // Slate 800
                card_border: Color32::from_rgb(51, 65, 85),     // Slate 700
                text_primary: Color32::from_rgb(248, 250, 252), // Slate 50 (contrast ~14:1)
                text_secondary: Color32::from_rgb(148, 163, 184),// Slate 400 (contrast ~6.5:1)
                grid_macro: Color32::from_rgb(71, 85, 105),     // Slate 600
                grid_mid: Color32::from_rgba_premultiplied(71, 85, 105, 120),
                grid_micro: Color32::from_rgba_premultiplied(71, 85, 105, 50),
                time_label: Color32::from_rgb(148, 163, 184),
                selection: Color32::from_rgb(96, 165, 250),     // Blue 400
                buffer_hatch: Color32::from_rgb(234, 179, 8),   // Yellow 500
            },
            ThemeMode::Light => Self {
                bg_canvas: Color32::from_rgb(248, 250, 252),    // Slate 50
                card_bg: Color32::from_rgb(255, 255, 255),      // White
                card_border: Color32::from_rgb(203, 213, 225),  // Slate 300
                text_primary: Color32::from_rgb(15, 23, 42),    // Slate 900 (contrast ~15:1)
                text_secondary: Color32::from_rgb(71, 85, 105), // Slate 600 (contrast ~6.0:1)
                grid_macro: Color32::from_rgb(203, 213, 225),   // Slate 300
                grid_mid: Color32::from_rgba_premultiplied(203, 213, 225, 140),
                grid_micro: Color32::from_rgba_premultiplied(203, 213, 225, 70),
                time_label: Color32::from_rgb(71, 85, 105),
                selection: Color32::from_rgb(37, 99, 235),      // Blue 600
                buffer_hatch: Color32::from_rgb(202, 138, 4),   // Yellow 600
            },
            ThemeMode::HighContrast => Self {
                bg_canvas: Color32::BLACK,
                card_bg: Color32::from_rgb(20, 20, 20),
                card_border: Color32::WHITE,
                text_primary: Color32::WHITE,                   // Contrast 21:1
                text_secondary: Color32::from_rgb(220, 220, 220),
                grid_macro: Color32::WHITE,
                grid_mid: Color32::from_rgb(120, 120, 120),
                grid_micro: Color32::from_rgb(80, 80, 80),
                time_label: Color32::WHITE,
                selection: Color32::YELLOW,
                buffer_hatch: Color32::YELLOW,
            },
        }
    }
}
