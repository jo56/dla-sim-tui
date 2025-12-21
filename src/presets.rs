use crate::settings::{
    BoundaryBehavior, NeighborhoodType, SimulationSettings, SpawnMode,
};
use crate::simulation::SeedPattern;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// A named preset containing simulation settings
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub description: String,
    pub settings: SimulationSettings,
    pub seed_pattern: SeedPattern,
    pub base_stickiness: f32,
    pub num_particles: usize,
}

#[allow(dead_code)]
impl Preset {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        settings: SimulationSettings,
        seed_pattern: SeedPattern,
        base_stickiness: f32,
        num_particles: usize,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            settings,
            seed_pattern,
            base_stickiness,
            num_particles,
        }
    }
}

/// Manager for loading and saving presets
#[allow(dead_code)]
pub struct PresetManager {
    /// Built-in presets that ship with the app
    pub builtin: Vec<Preset>,
    /// User-created presets loaded from disk
    pub user: Vec<Preset>,
}

#[allow(dead_code)]
impl Default for PresetManager {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl PresetManager {
    pub fn new() -> Self {
        let mut manager = Self {
            builtin: Vec::new(),
            user: Vec::new(),
        };
        manager.load_builtin_presets();
        manager.load_user_presets();
        manager
    }

    /// Load the built-in presets
    fn load_builtin_presets(&mut self) {
        self.builtin = vec![
            // Classic - default settings
            Preset::new(
                "Classic",
                "Standard DLA with default settings",
                SimulationSettings::default(),
                SeedPattern::Point,
                1.0,
                5000,
            ),
            // Dense - compact growth
            Preset::new(
                "Dense",
                "Compact structures with multiple contact requirement",
                SimulationSettings {
                    walk_step_size: 1.0,
                    multi_contact_min: 2,
                    neighborhood: NeighborhoodType::Moore,
                    ..Default::default()
                },
                SeedPattern::Point,
                1.0,
                5000,
            ),
            // Dendritic - thin branches
            Preset::new(
                "Dendritic",
                "Thin, branching dendrite patterns",
                SimulationSettings {
                    walk_step_size: 3.0,
                    tip_stickiness: 1.0,
                    side_stickiness: 0.3,
                    ..Default::default()
                },
                SeedPattern::Point,
                0.3,
                5000,
            ),
            // Snowflake - symmetric cross pattern
            Preset::new(
                "Snowflake",
                "Symmetric snowflake-like growth",
                SimulationSettings {
                    walk_step_size: 2.0,
                    neighborhood: NeighborhoodType::VonNeumann,
                    ..Default::default()
                },
                SeedPattern::Cross,
                0.8,
                5000,
            ),
            // Coral - thick organic growth
            Preset::new(
                "Coral",
                "Thick, coral-like structures",
                SimulationSettings {
                    walk_step_size: 1.5,
                    tip_stickiness: 0.5,
                    side_stickiness: 1.0,
                    neighborhood: NeighborhoodType::Moore,
                    ..Default::default()
                },
                SeedPattern::Ring,
                0.7,
                5000,
            ),
            // Wind-swept - directional growth
            Preset::new(
                "Wind-swept",
                "Asymmetric growth with directional bias",
                SimulationSettings {
                    walk_bias_angle: 45.0,
                    walk_bias_strength: 0.3,
                    ..Default::default()
                },
                SeedPattern::Point,
                0.8,
                5000,
            ),
            // Fractal Forest - multiple growing points
            Preset::new(
                "Fractal Forest",
                "Multiple growth centers competing",
                SimulationSettings {
                    walk_step_size: 2.5,
                    escape_multiplier: 3.0,
                    ..Default::default()
                },
                SeedPattern::Scatter,
                0.4,
                8000,
            ),
            // Edge Growth - particles from edges
            Preset::new(
                "Edge Growth",
                "Particles spawn from grid edges",
                SimulationSettings {
                    spawn_mode: SpawnMode::Edges,
                    boundary_behavior: BoundaryBehavior::Bounce,
                    ..Default::default()
                },
                SeedPattern::Point,
                0.9,
                5000,
            ),
            // Angular - Von Neumann creates angular patterns
            Preset::new(
                "Angular",
                "Sharp, angular growth patterns",
                SimulationSettings {
                    neighborhood: NeighborhoodType::VonNeumann,
                    walk_step_size: 1.5,
                    ..Default::default()
                },
                SeedPattern::Point,
                1.0,
                5000,
            ),
            // Blob - dense blob-like growth
            Preset::new(
                "Blob",
                "Dense, blob-like structures",
                SimulationSettings {
                    neighborhood: NeighborhoodType::Extended,
                    multi_contact_min: 3,
                    walk_step_size: 1.0,
                    ..Default::default()
                },
                SeedPattern::Block,
                1.0,
                5000,
            ),
            // Gradient - stickiness varies by distance
            Preset::new(
                "Gradient",
                "Dense core with sparse edges",
                SimulationSettings {
                    stickiness_gradient: -0.3,
                    ..Default::default()
                },
                SeedPattern::Point,
                1.0,
                5000,
            ),
            // Directional Rain - particles from top
            Preset::new(
                "Rain",
                "Particles fall from top edge",
                SimulationSettings {
                    spawn_mode: SpawnMode::Top,
                    radial_bias: 0.1,
                    ..Default::default()
                },
                SeedPattern::Line,
                0.8,
                5000,
            ),
        ];
    }

    /// Get the presets directory path
    fn presets_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("dla-sim-tui").join("presets"))
    }

    /// Load user presets from disk
    fn load_user_presets(&mut self) {
        if let Some(dir) = Self::presets_dir() {
            if dir.exists() {
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        if entry.path().extension().is_some_and(|e| e == "json") {
                            if let Ok(content) = fs::read_to_string(entry.path()) {
                                if let Ok(preset) = serde_json::from_str::<Preset>(&content) {
                                    self.user.push(preset);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Save a preset to disk
    pub fn save_preset(&mut self, preset: Preset) -> Result<(), String> {
        let dir = Self::presets_dir().ok_or("Could not determine config directory")?;

        // Create directory if it doesn't exist
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create presets directory: {}", e))?;

        // Sanitize filename
        let filename = preset
            .name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect::<String>();

        let path = dir.join(format!("{}.json", filename));

        let json = serde_json::to_string_pretty(&preset)
            .map_err(|e| format!("Failed to serialize preset: {}", e))?;

        fs::write(&path, json).map_err(|e| format!("Failed to write preset file: {}", e))?;

        // Add to user presets if not already present
        if !self.user.iter().any(|p| p.name == preset.name) {
            self.user.push(preset);
        }

        Ok(())
    }

    /// Delete a user preset
    pub fn delete_preset(&mut self, name: &str) -> Result<(), String> {
        let dir = Self::presets_dir().ok_or("Could not determine config directory")?;

        // Find and remove from user list
        if let Some(pos) = self.user.iter().position(|p| p.name == name) {
            self.user.remove(pos);
        }

        // Sanitize filename and delete file
        let filename = name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect::<String>();

        let path = dir.join(format!("{}.json", filename));
        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("Failed to delete preset file: {}", e))?;
        }

        Ok(())
    }

    /// Get all presets (builtin + user)
    pub fn all_presets(&self) -> impl Iterator<Item = &Preset> {
        self.builtin.iter().chain(self.user.iter())
    }

    /// Find a preset by name
    pub fn find(&self, name: &str) -> Option<&Preset> {
        self.all_presets().find(|p| p.name.eq_ignore_ascii_case(name))
    }

    /// Get preset names for display
    pub fn preset_names(&self) -> Vec<&str> {
        self.all_presets().map(|p| p.name.as_str()).collect()
    }
}
