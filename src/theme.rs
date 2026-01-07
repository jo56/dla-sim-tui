use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::color::ColorScheme;

/// Background mode for themes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BackgroundMode {
    /// Use terminal's default background (transparent)
    Transparent,
    /// Use a custom solid background color
    Solid(u8, u8, u8),
}

impl Default for BackgroundMode {
    fn default() -> Self {
        BackgroundMode::Transparent
    }
}

/// Complete theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    /// Human-readable theme name
    pub name: &'static str,
    /// Particle color gradient scheme
    pub color_scheme: ColorScheme,
    /// Border color for UI panels
    pub border_color: Color,
    /// Highlight color for focused elements
    pub highlight_color: Color,
    /// Primary text color
    pub text_color: Color,
    /// Dimmed text color for labels
    pub dim_text_color: Color,
    /// Background handling
    pub background: BackgroundMode,
}

/// Theme identifier enum for CLI selection and cycling
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum ThemeId {
    #[default]
    Default,
    // Terminal-integrated (transparent background)
    RosePine,
    RosePineMoon,
    Dracula,
    GruvboxDark,
    TokyoNight,
    Catppuccin,
    Nord,
    // Custom background themes
    DeepSpace,
    Sunset,
    Matrix,
    Amber,
}

impl ThemeId {
    /// Get all theme IDs in order
    const ALL: [ThemeId; 12] = [
        ThemeId::Default,
        ThemeId::RosePine,
        ThemeId::RosePineMoon,
        ThemeId::Dracula,
        ThemeId::GruvboxDark,
        ThemeId::TokyoNight,
        ThemeId::Catppuccin,
        ThemeId::Nord,
        ThemeId::DeepSpace,
        ThemeId::Sunset,
        ThemeId::Matrix,
        ThemeId::Amber,
    ];

    pub fn next(&self) -> Self {
        let idx = Self::ALL.iter().position(|t| t == self).unwrap_or(0);
        Self::ALL[(idx + 1) % Self::ALL.len()]
    }

    pub fn prev(&self) -> Self {
        let idx = Self::ALL.iter().position(|t| t == self).unwrap_or(0);
        Self::ALL[(idx + Self::ALL.len() - 1) % Self::ALL.len()]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ThemeId::Default => "Default",
            ThemeId::RosePine => "Rose Pine",
            ThemeId::RosePineMoon => "Rose Pine Moon",
            ThemeId::Dracula => "Dracula",
            ThemeId::GruvboxDark => "Gruvbox Dark",
            ThemeId::TokyoNight => "Tokyo Night",
            ThemeId::Catppuccin => "Catppuccin",
            ThemeId::Nord => "Nord",
            ThemeId::DeepSpace => "Deep Space",
            ThemeId::Sunset => "Sunset",
            ThemeId::Matrix => "Matrix",
            ThemeId::Amber => "Amber",
        }
    }

    /// Get the full theme configuration for this theme ID
    pub fn theme(&self) -> Theme {
        match self {
            ThemeId::Default => Theme {
                name: "Default",
                color_scheme: ColorScheme::Ice,
                border_color: Color::Cyan,
                highlight_color: Color::Yellow,
                text_color: Color::White,
                dim_text_color: Color::Gray,
                background: BackgroundMode::Transparent,
            },
            ThemeId::RosePine => Theme {
                name: "Rose Pine",
                color_scheme: ColorScheme::RosePine,
                border_color: Color::Rgb(49, 116, 143),  // #31748F foam
                highlight_color: Color::Rgb(246, 193, 119),  // #F6C177 gold
                text_color: Color::Rgb(224, 222, 244),  // #E0DEF4
                dim_text_color: Color::Rgb(144, 140, 170),  // #908CAA
                background: BackgroundMode::Transparent,
            },
            ThemeId::RosePineMoon => Theme {
                name: "Rose Pine Moon",
                color_scheme: ColorScheme::RosePine,
                border_color: Color::Rgb(62, 143, 176),  // #3E8FB0
                highlight_color: Color::Rgb(246, 193, 119),  // #F6C177 gold
                text_color: Color::Rgb(224, 222, 244),  // #E0DEF4
                dim_text_color: Color::Rgb(129, 124, 156),  // #817C9C
                background: BackgroundMode::Transparent,
            },
            ThemeId::Dracula => Theme {
                name: "Dracula",
                color_scheme: ColorScheme::Dracula,
                border_color: Color::Rgb(189, 147, 249),  // #BD93F9 purple
                highlight_color: Color::Rgb(241, 250, 140),  // #F1FA8C yellow
                text_color: Color::Rgb(248, 248, 242),  // #F8F8F2
                dim_text_color: Color::Rgb(98, 114, 164),  // #6272A4
                background: BackgroundMode::Transparent,
            },
            ThemeId::GruvboxDark => Theme {
                name: "Gruvbox Dark",
                color_scheme: ColorScheme::Gruvbox,
                border_color: Color::Rgb(131, 165, 152),  // #83A598 aqua
                highlight_color: Color::Rgb(250, 189, 47),  // #FABD2F yellow
                text_color: Color::Rgb(235, 219, 178),  // #EBDBB2
                dim_text_color: Color::Rgb(146, 131, 116),  // #928374
                background: BackgroundMode::Transparent,
            },
            ThemeId::TokyoNight => Theme {
                name: "Tokyo Night",
                color_scheme: ColorScheme::TokyoNight,
                border_color: Color::Rgb(122, 162, 247),  // #7AA2F7 blue
                highlight_color: Color::Rgb(224, 175, 104),  // #E0AF68 yellow
                text_color: Color::Rgb(169, 177, 214),  // #A9B1D6
                dim_text_color: Color::Rgb(86, 95, 137),  // #565F89
                background: BackgroundMode::Transparent,
            },
            ThemeId::Catppuccin => Theme {
                name: "Catppuccin",
                color_scheme: ColorScheme::Plasma,  // Uses existing Plasma gradient
                border_color: Color::Rgb(137, 180, 250),  // #89B4FA blue
                highlight_color: Color::Rgb(249, 226, 175),  // #F9E2AF yellow
                text_color: Color::Rgb(205, 214, 244),  // #CDD6F4
                dim_text_color: Color::Rgb(108, 112, 134),  // #6C7086
                background: BackgroundMode::Transparent,
            },
            ThemeId::Nord => Theme {
                name: "Nord",
                color_scheme: ColorScheme::Nord,
                border_color: Color::Rgb(136, 192, 208),  // #88C0D0 frost
                highlight_color: Color::Rgb(235, 203, 139),  // #EBCB8B yellow
                text_color: Color::Rgb(236, 239, 244),  // #ECEFF4
                dim_text_color: Color::Rgb(76, 86, 106),  // #4C566A
                background: BackgroundMode::Transparent,
            },
            ThemeId::DeepSpace => Theme {
                name: "Deep Space",
                color_scheme: ColorScheme::Neon,  // Uses existing Neon gradient
                border_color: Color::Rgb(88, 166, 255),  // #58A6FF
                highlight_color: Color::Rgb(255, 166, 87),  // #FFA657
                text_color: Color::Rgb(201, 209, 217),  // #C9D1D9
                dim_text_color: Color::Rgb(110, 118, 129),  // #6E7681
                background: BackgroundMode::Solid(13, 17, 23),  // #0D1117
            },
            ThemeId::Sunset => Theme {
                name: "Sunset",
                color_scheme: ColorScheme::Sunset,
                border_color: Color::Rgb(255, 107, 107),  // #FF6B6B
                highlight_color: Color::Rgb(255, 230, 109),  // #FFE66D
                text_color: Color::Rgb(247, 255, 247),  // #F7FFF7
                dim_text_color: Color::Rgb(180, 160, 180),  // muted lavender
                background: BackgroundMode::Solid(26, 20, 35),  // #1A1423
            },
            ThemeId::Matrix => Theme {
                name: "Matrix",
                color_scheme: ColorScheme::Matrix,
                border_color: Color::Rgb(0, 255, 65),  // #00FF41 bright green
                highlight_color: Color::Rgb(173, 255, 47),  // #ADFF2F green-yellow
                text_color: Color::Rgb(0, 255, 65),  // #00FF41
                dim_text_color: Color::Rgb(0, 128, 0),  // darker green
                background: BackgroundMode::Solid(10, 10, 10),  // #0A0A0A
            },
            ThemeId::Amber => Theme {
                name: "Amber",
                color_scheme: ColorScheme::Amber,
                border_color: Color::Rgb(255, 176, 0),  // #FFB000 amber
                highlight_color: Color::Rgb(255, 204, 0),  // #FFCC00 gold
                text_color: Color::Rgb(255, 200, 100),  // warm amber text
                dim_text_color: Color::Rgb(180, 120, 50),  // muted amber
                background: BackgroundMode::Solid(26, 26, 10),  // #1A1A0A
            },
        }
    }
}

/// Parse a theme name string into a ThemeId
pub fn parse_theme(s: &str) -> ThemeId {
    match s.to_lowercase().replace(['-', '_', ' '], "").as_str() {
        "default" => ThemeId::Default,
        "rosepine" => ThemeId::RosePine,
        "rosepinemoon" => ThemeId::RosePineMoon,
        "dracula" => ThemeId::Dracula,
        "gruvbox" | "gruvboxdark" => ThemeId::GruvboxDark,
        "tokyonight" => ThemeId::TokyoNight,
        "catppuccin" | "catppuccinmocha" => ThemeId::Catppuccin,
        "nord" => ThemeId::Nord,
        "deepspace" | "space" => ThemeId::DeepSpace,
        "sunset" => ThemeId::Sunset,
        "matrix" => ThemeId::Matrix,
        "amber" => ThemeId::Amber,
        _ => ThemeId::Default,
    }
}
