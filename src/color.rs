use ratatui::style::Color;
use serde::{Deserialize, Serialize};

// Color gradient constants
/// First threshold for 3-stop gradients (33%)
const GRADIENT_STOP_1: f32 = 0.33;
/// Second threshold for 3-stop gradients (66%)
const GRADIENT_STOP_2: f32 = 0.66;
/// Remaining range after second stop (34%)
const GRADIENT_STOP_3_RANGE: f32 = 0.34;
/// Midpoint threshold for 2-stop gradients
const GRADIENT_MID: f32 = 0.5;

// Ice gradient color coefficients (dark blue -> cyan -> white)
const ICE_RED_BASE: f32 = 200.0;
const ICE_RED_QUADRATIC: f32 = 55.0;
const ICE_GREEN_BASE: f32 = 220.0;
const ICE_GREEN_LINEAR: f32 = 35.0;
const ICE_BLUE_BASE: f32 = 180.0;
const ICE_BLUE_LINEAR: f32 = 75.0;

// Plasma gradient phase offsets for sinusoidal color cycling
const PLASMA_PHASE_GREEN: f32 = 0.33;
const PLASMA_PHASE_BLUE: f32 = 0.67;
const PLASMA_MIN_RED: u8 = 50;

/// Pre-computed color lookup table (256 entries for fast gradient access)
pub type ColorLut = [Color; 256];

/// Fast color lookup from pre-computed LUT (t should be 0.0-1.0)
#[inline]
pub fn map_from_lut(lut: &ColorLut, t: f32) -> Color {
    let idx = (t.clamp(0.0, 1.0) * 255.0) as usize;
    lut[idx]
}

/// Color schemes for visualization
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum ColorScheme {
    #[default]
    Ice,
    Fire,
    Plasma,
    Viridis,
    Rainbow,
    Grayscale,
    Ocean,
    Neon,
    // Theme-specific gradients (renamed to match theme names)
    Lagoon,
    Violet,
    Harvest,
    Midnight,
    Frost,
    Sunset,
    Matrix,
    Amber,
}

impl ColorScheme {
    pub fn name(&self) -> &str {
        match self {
            ColorScheme::Ice => "Ice",
            ColorScheme::Fire => "Fire",
            ColorScheme::Plasma => "Plasma",
            ColorScheme::Viridis => "Viridis",
            ColorScheme::Rainbow => "Rainbow",
            ColorScheme::Grayscale => "Grayscale",
            ColorScheme::Ocean => "Ocean",
            ColorScheme::Neon => "Neon",
            ColorScheme::Lagoon => "Lagoon",
            ColorScheme::Violet => "Violet",
            ColorScheme::Harvest => "Harvest",
            ColorScheme::Midnight => "Midnight",
            ColorScheme::Frost => "Frost",
            ColorScheme::Sunset => "Sunset",
            ColorScheme::Matrix => "Matrix",
            ColorScheme::Amber => "Amber",
        }
    }

    pub fn next(&self) -> ColorScheme {
        match self {
            ColorScheme::Ice => ColorScheme::Fire,
            ColorScheme::Fire => ColorScheme::Plasma,
            ColorScheme::Plasma => ColorScheme::Viridis,
            ColorScheme::Viridis => ColorScheme::Rainbow,
            ColorScheme::Rainbow => ColorScheme::Grayscale,
            ColorScheme::Grayscale => ColorScheme::Ocean,
            ColorScheme::Ocean => ColorScheme::Neon,
            ColorScheme::Neon => ColorScheme::Lagoon,
            ColorScheme::Lagoon => ColorScheme::Violet,
            ColorScheme::Violet => ColorScheme::Harvest,
            ColorScheme::Harvest => ColorScheme::Midnight,
            ColorScheme::Midnight => ColorScheme::Frost,
            ColorScheme::Frost => ColorScheme::Sunset,
            ColorScheme::Sunset => ColorScheme::Matrix,
            ColorScheme::Matrix => ColorScheme::Amber,
            ColorScheme::Amber => ColorScheme::Ice,
        }
    }

    pub fn prev(&self) -> ColorScheme {
        match self {
            ColorScheme::Ice => ColorScheme::Amber,
            ColorScheme::Fire => ColorScheme::Ice,
            ColorScheme::Plasma => ColorScheme::Fire,
            ColorScheme::Viridis => ColorScheme::Plasma,
            ColorScheme::Rainbow => ColorScheme::Viridis,
            ColorScheme::Grayscale => ColorScheme::Rainbow,
            ColorScheme::Ocean => ColorScheme::Grayscale,
            ColorScheme::Neon => ColorScheme::Ocean,
            ColorScheme::Lagoon => ColorScheme::Neon,
            ColorScheme::Violet => ColorScheme::Lagoon,
            ColorScheme::Harvest => ColorScheme::Violet,
            ColorScheme::Midnight => ColorScheme::Harvest,
            ColorScheme::Frost => ColorScheme::Midnight,
            ColorScheme::Sunset => ColorScheme::Frost,
            ColorScheme::Matrix => ColorScheme::Sunset,
            ColorScheme::Amber => ColorScheme::Matrix,
        }
    }

    /// Map a value from 0.0-1.0 to a terminal color
    pub fn map(&self, t: f32) -> Color {
        let (r, g, b) = self.map_rgb(t);
        Color::Rgb(r, g, b)
    }

    /// Map a value from 0.0-1.0 to raw RGB values (for video recording)
    pub fn map_rgb(&self, t: f32) -> (u8, u8, u8) {
        let t = t.clamp(0.0, 1.0);
        match self {
            ColorScheme::Ice => Self::ice_gradient(t),
            ColorScheme::Fire => Self::fire_gradient(t),
            ColorScheme::Plasma => Self::plasma_gradient(t),
            ColorScheme::Viridis => Self::viridis_gradient(t),
            ColorScheme::Rainbow => Self::rainbow_gradient(t),
            ColorScheme::Grayscale => Self::grayscale_gradient(t),
            ColorScheme::Ocean => Self::ocean_gradient(t),
            ColorScheme::Neon => Self::neon_gradient(t),
            ColorScheme::Lagoon => Self::lagoon_gradient(t),
            ColorScheme::Violet => Self::violet_gradient(t),
            ColorScheme::Harvest => Self::harvest_gradient(t),
            ColorScheme::Midnight => Self::midnight_gradient(t),
            ColorScheme::Frost => Self::frost_gradient(t),
            ColorScheme::Sunset => Self::sunset_gradient(t),
            ColorScheme::Matrix => Self::matrix_gradient(t),
            ColorScheme::Amber => Self::amber_gradient(t),
        }
    }

    /// Build a 256-entry lookup table for fast color access
    /// Call this once when color scheme changes, then use map_from_lut() for rendering
    pub fn build_lut(&self) -> ColorLut {
        let mut lut = [Color::White; 256];
        for (i, color) in lut.iter_mut().enumerate() {
            *color = self.map(i as f32 / 255.0);
        }
        lut
    }

    fn ice_gradient(t: f32) -> (u8, u8, u8) {
        // Dark blue -> cyan -> white
        let r = (t * ICE_RED_BASE + ICE_RED_QUADRATIC * t * t) as u8;
        let g = (t * ICE_GREEN_BASE + ICE_GREEN_LINEAR * t) as u8;
        let b = (ICE_BLUE_BASE + ICE_BLUE_LINEAR * t) as u8;
        (r, g, b)
    }

    fn fire_gradient(t: f32) -> (u8, u8, u8) {
        // Black -> red -> orange -> yellow -> white
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            ((s * 200.0) as u8, 0, 0)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            (200 + (s * 55.0) as u8, (s * 150.0) as u8, 0)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            (255, 150 + (s * 105.0) as u8, (s * 200.0) as u8)
        }
    }

    fn plasma_gradient(t: f32) -> (u8, u8, u8) {
        // Purple -> pink -> orange -> yellow (sinusoidal color cycling)
        let r = ((0.5 + 0.5 * (std::f32::consts::TAU * t).sin()) * 255.0) as u8;
        let g = ((0.5 + 0.5 * (std::f32::consts::TAU * (t + PLASMA_PHASE_GREEN)).sin()) * 200.0) as u8;
        let b = ((0.5 + 0.5 * (std::f32::consts::TAU * (t + PLASMA_PHASE_BLUE)).sin()) * 255.0) as u8;
        (r.max(PLASMA_MIN_RED), g, b)
    }

    fn viridis_gradient(t: f32) -> (u8, u8, u8) {
        // Dark purple -> teal -> yellow-green
        let r = (68.0 + t * 185.0 * t) as u8;
        let g = (1.0 + t * 220.0) as u8;
        let b = (84.0 + 90.0 * (1.0 - t) * (1.0 - t * 0.5)) as u8;
        (r, g, b)
    }

    fn rainbow_gradient(t: f32) -> (u8, u8, u8) {
        // HSV rotation through the rainbow
        let h = t * 360.0;
        let s = 1.0;
        let v = 1.0;
        Self::hsv_to_rgb(h, s, v)
    }

    fn grayscale_gradient(t: f32) -> (u8, u8, u8) {
        let v = (t * 255.0) as u8;
        (v, v, v)
    }

    fn ocean_gradient(t: f32) -> (u8, u8, u8) {
        // Deep blue -> teal -> aqua
        let r = (t * 100.0) as u8;
        let g = (50.0 + t * 150.0) as u8;
        let b = (100.0 + t * 155.0) as u8;
        (r, g, b)
    }

    fn neon_gradient(t: f32) -> (u8, u8, u8) {
        // Bright neon colors: magenta -> cyan -> green
        if t < GRADIENT_MID {
            let s = t / GRADIENT_MID;
            (255 - (s * 255.0) as u8, (s * 255.0) as u8, 255)
        } else {
            let s = (t - GRADIENT_MID) / GRADIENT_MID;
            (0, 255, 255 - (s * 255.0) as u8)
        }
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    // Theme-specific gradients

    fn lagoon_gradient(t: f32) -> (u8, u8, u8) {
        // Deep purple -> teal -> gold -> rose (coastal lagoon colors)
        // #191724 -> #31748F -> #F6C177 -> #EBBCBA
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            Self::lerp_rgb((25, 23, 36), (49, 116, 143), s)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            Self::lerp_rgb((49, 116, 143), (246, 193, 119), s)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            Self::lerp_rgb((246, 193, 119), (235, 188, 186), s)
        }
    }

    fn violet_gradient(t: f32) -> (u8, u8, u8) {
        // Dark background -> purple -> pink -> cyan
        // #282A36 -> #BD93F9 -> #FF79C6 -> #8BE9FD
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            Self::lerp_rgb((40, 42, 54), (189, 147, 249), s)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            Self::lerp_rgb((189, 147, 249), (255, 121, 198), s)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            Self::lerp_rgb((255, 121, 198), (139, 233, 253), s)
        }
    }

    fn harvest_gradient(t: f32) -> (u8, u8, u8) {
        // Dark background -> orange -> yellow -> bright yellow (autumn harvest)
        // #282828 -> #D65D0E -> #D79921 -> #FABD2F
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            Self::lerp_rgb((40, 40, 40), (214, 93, 14), s)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            Self::lerp_rgb((214, 93, 14), (215, 153, 33), s)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            Self::lerp_rgb((215, 153, 33), (250, 189, 47), s)
        }
    }

    fn midnight_gradient(t: f32) -> (u8, u8, u8) {
        // Deep blue -> blue -> purple -> light lavender (midnight sky)
        // #1A1B26 -> #7AA2F7 -> #BB9AF7 -> #C0CAF5
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            Self::lerp_rgb((26, 27, 38), (122, 162, 247), s)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            Self::lerp_rgb((122, 162, 247), (187, 154, 247), s)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            Self::lerp_rgb((187, 154, 247), (192, 202, 245), s)
        }
    }

    fn frost_gradient(t: f32) -> (u8, u8, u8) {
        // Dark -> frost blue -> bright frost -> snow (icy frost)
        // #2E3440 -> #5E81AC -> #88C0D0 -> #ECEFF4
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            Self::lerp_rgb((46, 52, 64), (94, 129, 172), s)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            Self::lerp_rgb((94, 129, 172), (136, 192, 208), s)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            Self::lerp_rgb((136, 192, 208), (236, 239, 244), s)
        }
    }

    fn sunset_gradient(t: f32) -> (u8, u8, u8) {
        // Dark purple -> red -> orange -> yellow
        // #1A1423 -> #FF6B6B -> #FFA07A -> #FFE66D
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            Self::lerp_rgb((26, 20, 35), (255, 107, 107), s)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            Self::lerp_rgb((255, 107, 107), (255, 160, 122), s)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            Self::lerp_rgb((255, 160, 122), (255, 230, 109), s)
        }
    }

    fn matrix_gradient(t: f32) -> (u8, u8, u8) {
        // Black -> dark green -> bright green -> yellow-green
        // #0A0A0A -> #003B00 -> #00FF41 -> #ADFF2F
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            Self::lerp_rgb((10, 10, 10), (0, 59, 0), s)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            Self::lerp_rgb((0, 59, 0), (0, 255, 65), s)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            Self::lerp_rgb((0, 255, 65), (173, 255, 47), s)
        }
    }

    fn amber_gradient(t: f32) -> (u8, u8, u8) {
        // Dark -> dark amber -> amber -> gold
        // #1A1A0A -> #8B4000 -> #FFB000 -> #FFCC00
        if t < GRADIENT_STOP_1 {
            let s = t / GRADIENT_STOP_1;
            Self::lerp_rgb((26, 26, 10), (139, 64, 0), s)
        } else if t < GRADIENT_STOP_2 {
            let s = (t - GRADIENT_STOP_1) / GRADIENT_STOP_1;
            Self::lerp_rgb((139, 64, 0), (255, 176, 0), s)
        } else {
            let s = (t - GRADIENT_STOP_2) / GRADIENT_STOP_3_RANGE;
            Self::lerp_rgb((255, 176, 0), (255, 204, 0), s)
        }
    }

    /// Linear interpolation between two RGB colors
    #[inline]
    fn lerp_rgb(c1: (u8, u8, u8), c2: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
        let r = c1.0 as f32 + (c2.0 as f32 - c1.0 as f32) * t;
        let g = c1.1 as f32 + (c2.1 as f32 - c1.1 as f32) * t;
        let b = c1.2 as f32 + (c2.2 as f32 - c1.2 as f32) * t;
        (r as u8, g as u8, b as u8)
    }
}
