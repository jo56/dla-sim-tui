use crate::color::ColorScheme;
use crate::simulation::DlaSimulation;
use ratatui::style::Color;

/// Braille character rendering for high-resolution terminal graphics.
/// Each Braille character represents a 2x4 grid of dots (8 dots total).
///
/// Dot positions and their bit values:
/// ```
/// (0,0)=0x01  (1,0)=0x08
/// (0,1)=0x02  (1,1)=0x10
/// (0,2)=0x04  (1,2)=0x20
/// (0,3)=0x40  (1,3)=0x80
/// ```
///
/// Unicode Braille patterns: U+2800 to U+28FF (256 patterns)

const BRAILLE_BASE: u32 = 0x2800;

/// Dot position to bit mapping for Braille characters
const BRAILLE_DOTS: [[u8; 4]; 2] = [
    [0x01, 0x02, 0x04, 0x40], // Left column (x=0): rows 0,1,2,3
    [0x08, 0x10, 0x20, 0x80], // Right column (x=1): rows 0,1,2,3
];

/// A single rendered Braille cell with position and color
#[derive(Clone)]
pub struct BrailleCell {
    pub x: u16,
    pub y: u16,
    pub char: char,
    pub color: Color,
}

/// Render the simulation grid to Braille characters
pub fn render_to_braille(
    simulation: &DlaSimulation,
    canvas_width: u16,
    canvas_height: u16,
    color_scheme: &ColorScheme,
    color_by_age: bool,
) -> Vec<BrailleCell> {
    let mut cells = Vec::with_capacity((canvas_width * canvas_height) as usize);

    // Each Braille character covers 2x4 simulation pixels
    // Calculate scale factors to map simulation grid to canvas
    let sim_width = simulation.grid_width;
    let sim_height = simulation.grid_height;

    // Braille effective resolution
    let braille_width = canvas_width as usize * 2;
    let braille_height = canvas_height as usize * 4;

    // Scale factors
    let scale_x = sim_width as f32 / braille_width as f32;
    let scale_y = sim_height as f32 / braille_height as f32;

    for cy in 0..canvas_height {
        for cx in 0..canvas_width {
            let mut pattern: u8 = 0;
            let mut total_age: f32 = 0.0;
            let mut dot_count: usize = 0;

            // Sample the 2x4 dots for this Braille character
            for dx in 0..2 {
                for dy in 0..4 {
                    // Calculate simulation grid position
                    let braille_x = cx as usize * 2 + dx;
                    let braille_y = cy as usize * 4 + dy;

                    let sim_x = (braille_x as f32 * scale_x) as usize;
                    let sim_y = (braille_y as f32 * scale_y) as usize;

                    // Check if this simulation cell is occupied
                    if let Some(age) = simulation.get_cell(sim_x, sim_y) {
                        pattern |= BRAILLE_DOTS[dx][dy];
                        total_age += age as f32;
                        dot_count += 1;
                    }
                }
            }

            // Only emit cells that have at least one dot
            if pattern != 0 {
                let braille_char = char::from_u32(BRAILLE_BASE + pattern as u32).unwrap_or(' ');

                let color = if color_by_age && dot_count > 0 {
                    // Average age for color mapping
                    let avg_age = total_age / dot_count as f32;
                    let t = avg_age / simulation.num_particles as f32;
                    color_scheme.map(t)
                } else {
                    Color::White
                };

                cells.push(BrailleCell {
                    x: cx,
                    y: cy,
                    char: braille_char,
                    color,
                });
            }
        }
    }

    cells
}

/// Calculate optimal simulation grid size for a given canvas size
/// Returns (width, height) for the simulation grid
pub fn calculate_simulation_size(canvas_width: u16, canvas_height: u16) -> (usize, usize) {
    // Braille gives 2x4 resolution per character
    // We want the simulation grid to match this resolution
    let width = (canvas_width as usize * 2).max(64);
    let height = (canvas_height as usize * 4).max(64);
    (width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille_pattern() {
        // Test that single dot patterns work correctly
        assert_eq!(BRAILLE_DOTS[0][0], 0x01); // Top-left
        assert_eq!(BRAILLE_DOTS[1][0], 0x08); // Top-right
        assert_eq!(BRAILLE_DOTS[0][3], 0x40); // Bottom-left
        assert_eq!(BRAILLE_DOTS[1][3], 0x80); // Bottom-right

        // All dots should give 0xFF
        let all_dots: u8 = BRAILLE_DOTS[0].iter().sum::<u8>() + BRAILLE_DOTS[1].iter().sum::<u8>();
        assert_eq!(all_dots, 0xFF);
    }

    #[test]
    fn test_braille_char_generation() {
        // Empty pattern
        let empty = char::from_u32(BRAILLE_BASE).unwrap();
        assert_eq!(empty, '\u{2800}');

        // Full pattern (all 8 dots)
        let full = char::from_u32(BRAILLE_BASE + 0xFF).unwrap();
        assert_eq!(full, '\u{28FF}');
    }
}
