use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::color::ColorScheme;

/// Background mode for themes
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum BackgroundMode {
    /// Use terminal's default background (transparent)
    #[default]
    Transparent,
    /// Use a custom solid background color
    Solid(u8, u8, u8),
}

/// Complete theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    /// Human-readable theme name
    #[allow(dead_code)]
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
    /// Particle/dot color when gradient is disabled
    pub particle_color: Color,
    /// Background handling
    pub background: BackgroundMode,
}

/// Theme identifier enum for CLI selection and cycling
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum ThemeId {
    #[default]
    Default,
    // Descriptive theme names
    Lagoon,
    Bluemono,
    Violet,
    Harvest,
    Midnight,
    Rainbow,
    Frost,
    // Custom themes
    DeepSpace,
    Sunset,
    Matrix,
    Amber,
}

impl ThemeId {
    /// Get all theme IDs in order
    const ALL: [ThemeId; 12] = [
        ThemeId::Default,
        ThemeId::Lagoon,
        ThemeId::Bluemono,
        ThemeId::Violet,
        ThemeId::Harvest,
        ThemeId::Midnight,
        ThemeId::Rainbow,
        ThemeId::Frost,
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

    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            ThemeId::Default => "Default",
            ThemeId::Lagoon => "Lagoon",
            ThemeId::Bluemono => "Bluemono",
            ThemeId::Violet => "Violet",
            ThemeId::Harvest => "Harvest",
            ThemeId::Midnight => "Midnight",
            ThemeId::Rainbow => "Rainbow",
            ThemeId::Frost => "Frost",
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
                color_scheme: ColorScheme::Neon,  // High-contrast magenta->cyan->green
                border_color: Color::Rgb(0, 255, 255),     // Bright cyan
                highlight_color: Color::Rgb(255, 255, 0),  // Bright yellow
                text_color: Color::Rgb(255, 255, 255),     // Pure white
                dim_text_color: Color::Rgb(160, 160, 160), // Visible gray
                particle_color: Color::Rgb(0, 255, 255),   // Bright cyan
                background: BackgroundMode::Solid(8, 8, 12),  // Near-black #08080C
            },
            ThemeId::Lagoon => Theme {
                name: "Lagoon",
                color_scheme: ColorScheme::Lagoon,
                border_color: Color::Rgb(49, 116, 143),  // #31748F teal
                highlight_color: Color::Rgb(246, 193, 119),  // #F6C177 gold
                text_color: Color::Rgb(224, 222, 244),  // #E0DEF4
                dim_text_color: Color::Rgb(144, 140, 170),  // #908CAA
                particle_color: Color::Rgb(156, 207, 216),  // #9CCFD8 foam
                background: BackgroundMode::Solid(25, 23, 36),  // #191724
            },
            ThemeId::Bluemono => Theme {
                name: "Bluemono",
                color_scheme: ColorScheme::Ocean,  // Blue monochrome gradient
                border_color: Color::Rgb(0, 0, 0),  // Black borders
                highlight_color: Color::Rgb(0, 0, 0),  // Black highlights
                text_color: Color::Rgb(0, 0, 0),  // Black text
                dim_text_color: Color::Rgb(0, 0, 0),  // Black labels
                particle_color: Color::Rgb(30, 85, 130),  // Deep blue for contrast
                background: BackgroundMode::Solid(252, 246, 248),  // #FCF6F8 soft white
            },
            ThemeId::Violet => Theme {
                name: "Violet",
                color_scheme: ColorScheme::Violet,
                border_color: Color::Rgb(189, 147, 249),  // #BD93F9 purple
                highlight_color: Color::Rgb(241, 250, 140),  // #F1FA8C yellow
                text_color: Color::Rgb(248, 248, 242),  // #F8F8F2
                dim_text_color: Color::Rgb(98, 114, 164),  // #6272A4
                particle_color: Color::Rgb(139, 233, 253),  // #8BE9FD cyan
                background: BackgroundMode::Solid(40, 42, 54),  // #282a36
            },
            ThemeId::Harvest => Theme {
                name: "Harvest",
                color_scheme: ColorScheme::Harvest,
                border_color: Color::Rgb(131, 165, 152),  // #83A598 teal
                highlight_color: Color::Rgb(250, 189, 47),  // #FABD2F yellow
                text_color: Color::Rgb(235, 219, 178),  // #EBDBB2 cream
                dim_text_color: Color::Rgb(146, 131, 116),  // #928374
                particle_color: Color::Rgb(254, 128, 25),  // #FE8019 bright orange
                background: BackgroundMode::Solid(40, 40, 40),  // #282828
            },
            ThemeId::Midnight => Theme {
                name: "Midnight",
                color_scheme: ColorScheme::Midnight,
                border_color: Color::Rgb(122, 162, 247),  // #7AA2F7 blue
                highlight_color: Color::Rgb(224, 175, 104),  // #E0AF68 gold
                text_color: Color::Rgb(169, 177, 214),  // #A9B1D6
                dim_text_color: Color::Rgb(86, 95, 137),  // #565F89
                particle_color: Color::Rgb(122, 162, 247),  // #7AA2F7 blue
                background: BackgroundMode::Solid(26, 27, 38),  // #1a1b26
            },
            ThemeId::Rainbow => Theme {
                name: "Rainbow",
                color_scheme: ColorScheme::Plasma,  // Uses existing Plasma gradient
                border_color: Color::Rgb(137, 180, 250),  // #89B4FA blue
                highlight_color: Color::Rgb(249, 226, 175),  // #F9E2AF cream
                text_color: Color::Rgb(205, 214, 244),  // #CDD6F4
                dim_text_color: Color::Rgb(108, 112, 134),  // #6C7086
                particle_color: Color::Rgb(203, 166, 247),  // #CBA6F7 mauve
                background: BackgroundMode::Solid(30, 30, 46),  // #1e1e2e
            },
            ThemeId::Frost => Theme {
                name: "Frost",
                color_scheme: ColorScheme::Frost,
                border_color: Color::Rgb(136, 192, 208),  // #88C0D0 cyan
                highlight_color: Color::Rgb(235, 203, 139),  // #EBCB8B yellow
                text_color: Color::Rgb(236, 239, 244),  // #ECEFF4
                dim_text_color: Color::Rgb(76, 86, 106),  // #4C566A
                particle_color: Color::Rgb(136, 192, 208),  // #88C0D0 frost cyan
                background: BackgroundMode::Solid(46, 52, 64),  // #2e3440
            },
            ThemeId::DeepSpace => Theme {
                name: "Deep Space",
                color_scheme: ColorScheme::Neon,  // Uses existing Neon gradient
                border_color: Color::Rgb(88, 166, 255),  // #58A6FF
                highlight_color: Color::Rgb(255, 166, 87),  // #FFA657
                text_color: Color::Rgb(201, 209, 217),  // #C9D1D9
                dim_text_color: Color::Rgb(110, 118, 129),  // #6E7681
                particle_color: Color::Rgb(88, 166, 255),  // #58A6FF bright blue
                background: BackgroundMode::Solid(13, 17, 23),  // #0D1117
            },
            ThemeId::Sunset => Theme {
                name: "Sunset",
                color_scheme: ColorScheme::Sunset,
                border_color: Color::Rgb(255, 107, 107),  // #FF6B6B
                highlight_color: Color::Rgb(255, 230, 109),  // #FFE66D
                text_color: Color::Rgb(247, 255, 247),  // #F7FFF7
                dim_text_color: Color::Rgb(180, 160, 180),  // muted lavender
                particle_color: Color::Rgb(255, 160, 122),  // Light salmon
                background: BackgroundMode::Solid(26, 20, 35),  // #1A1423
            },
            ThemeId::Matrix => Theme {
                name: "Matrix",
                color_scheme: ColorScheme::Matrix,
                border_color: Color::Rgb(0, 255, 65),  // #00FF41 bright green
                highlight_color: Color::Rgb(173, 255, 47),  // #ADFF2F green-yellow
                text_color: Color::Rgb(0, 255, 65),  // #00FF41
                dim_text_color: Color::Rgb(0, 128, 0),  // darker green
                particle_color: Color::Rgb(0, 255, 65),  // #00FF41 matrix green
                background: BackgroundMode::Solid(10, 10, 10),  // #0A0A0A
            },
            ThemeId::Amber => Theme {
                name: "Amber",
                color_scheme: ColorScheme::Amber,
                border_color: Color::Rgb(255, 176, 0),  // #FFB000 amber
                highlight_color: Color::Rgb(255, 204, 0),  // #FFCC00 gold
                text_color: Color::Rgb(255, 200, 100),  // warm amber text
                dim_text_color: Color::Rgb(180, 120, 50),  // muted amber
                particle_color: Color::Rgb(255, 176, 0),  // #FFB000 amber
                background: BackgroundMode::Solid(26, 26, 10),  // #1A1A0A
            },
        }
    }
}

/// Parse a theme name string into a ThemeId
pub fn parse_theme(s: &str) -> ThemeId {
    match s.to_lowercase().replace(['-', '_', ' '], "").as_str() {
        "default" => ThemeId::Default,
        "lagoon" => ThemeId::Lagoon,
        "bluemono" => ThemeId::Bluemono,
        "violet" => ThemeId::Violet,
        "harvest" => ThemeId::Harvest,
        "midnight" => ThemeId::Midnight,
        "rainbow" => ThemeId::Rainbow,
        "frost" => ThemeId::Frost,
        "deepspace" | "space" => ThemeId::DeepSpace,
        "sunset" => ThemeId::Sunset,
        "matrix" => ThemeId::Matrix,
        "amber" => ThemeId::Amber,
        _ => ThemeId::Default,
    }
}
