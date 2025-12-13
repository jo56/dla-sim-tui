use rand::Rng;

/// Seed pattern types for initial structure
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SeedPattern {
    #[default]
    Point,
    Line,
    Cross,
    Circle,
}

impl SeedPattern {
    pub fn name(&self) -> &str {
        match self {
            SeedPattern::Point => "Point",
            SeedPattern::Line => "Line",
            SeedPattern::Cross => "Cross",
            SeedPattern::Circle => "Circle",
        }
    }

    pub fn all() -> &'static [SeedPattern] {
        &[
            SeedPattern::Point,
            SeedPattern::Line,
            SeedPattern::Cross,
            SeedPattern::Circle,
        ]
    }

    pub fn next(&self) -> SeedPattern {
        match self {
            SeedPattern::Point => SeedPattern::Line,
            SeedPattern::Line => SeedPattern::Cross,
            SeedPattern::Cross => SeedPattern::Circle,
            SeedPattern::Circle => SeedPattern::Point,
        }
    }

    pub fn prev(&self) -> SeedPattern {
        match self {
            SeedPattern::Point => SeedPattern::Circle,
            SeedPattern::Line => SeedPattern::Point,
            SeedPattern::Cross => SeedPattern::Line,
            SeedPattern::Circle => SeedPattern::Cross,
        }
    }
}

/// DLA simulation state
pub struct DlaSimulation {
    pub grid_width: usize,
    pub grid_height: usize,
    grid: Vec<Option<usize>>,
    pub num_particles: usize,
    pub stickiness: f32,
    pub particles_stuck: usize,
    max_radius: f32,
    pub paused: bool,
    pub seed_pattern: SeedPattern,
}

impl DlaSimulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sim = Self {
            grid_width: width,
            grid_height: height,
            grid: vec![None; width * height],
            num_particles: 5000,
            stickiness: 1.0,
            particles_stuck: 0,
            max_radius: 1.0,
            paused: false,
            seed_pattern: SeedPattern::Point,
        };
        sim.reset();
        sim
    }

    /// Execute one particle simulation step
    /// Returns true if simulation should continue, false if complete
    pub fn step(&mut self) -> bool {
        if self.paused || self.particles_stuck >= self.num_particles {
            return false;
        }

        let mut rng = rand::thread_rng();

        // Spawn radius - outside the structure
        let spawn_radius = (self.max_radius + 10.0).max(50.0);
        let center_x = self.grid_width as f32 / 2.0;
        let center_y = self.grid_height as f32 / 2.0;

        // Spawn particle on a circle
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let mut x = center_x + spawn_radius * angle.cos();
        let mut y = center_y + spawn_radius * angle.sin();

        // Random walk until it sticks or escapes
        for _ in 0..10000 {
            // Check if we've gone too far
            let dx = x - center_x;
            let dy = y - center_y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > spawn_radius * 2.0 {
                // Escaped, restart
                return true;
            }

            // Check if next to a stuck particle
            let ix = x as usize;
            let iy = y as usize;

            if ix > 0 && ix < self.grid_width - 1 && iy > 0 && iy < self.grid_height - 1 {
                let mut should_stick = false;

                // Check 8 neighbors
                'outer: for ndy in -1..=1i32 {
                    for ndx in -1..=1i32 {
                        if ndx == 0 && ndy == 0 {
                            continue;
                        }

                        let nx = (ix as i32 + ndx) as usize;
                        let ny = (iy as i32 + ndy) as usize;
                        let nidx = ny * self.grid_width + nx;

                        if self.grid[nidx].is_some() {
                            // Neighbor is stuck, maybe stick here
                            if rng.gen::<f32>() < self.stickiness {
                                should_stick = true;
                                break 'outer;
                            }
                        }
                    }
                }

                if should_stick {
                    // Stick here
                    let idx = iy * self.grid_width + ix;
                    self.grid[idx] = Some(self.particles_stuck);
                    self.particles_stuck += 1;

                    // Update max radius
                    let dx = ix as f32 - center_x;
                    let dy = iy as f32 - center_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    self.max_radius = self.max_radius.max(dist);

                    return true;
                }
            }

            // Random walk step
            let walk_angle = rng.gen_range(0.0..std::f32::consts::TAU);
            x += 2.0 * walk_angle.cos();
            y += 2.0 * walk_angle.sin();

            // Clamp to bounds
            x = x.clamp(1.0, self.grid_width as f32 - 2.0);
            y = y.clamp(1.0, self.grid_height as f32 - 2.0);
        }

        true
    }

    /// Reset the simulation with the current seed pattern
    pub fn reset(&mut self) {
        self.reset_with_seed(self.seed_pattern);
    }

    /// Reset with a specific seed pattern
    pub fn reset_with_seed(&mut self, pattern: SeedPattern) {
        // Resize grid if dimensions changed
        let required_size = self.grid_width * self.grid_height;
        if self.grid.len() != required_size {
            self.grid = vec![None; required_size];
        } else {
            self.grid.fill(None);
        }

        self.seed_pattern = pattern;

        match pattern {
            SeedPattern::Point => {
                // Single center point
                let center_idx = self.grid_height / 2 * self.grid_width + self.grid_width / 2;
                self.grid[center_idx] = Some(0);
                self.particles_stuck = 1;
                self.max_radius = 1.0;
            }
            SeedPattern::Line => {
                // Horizontal line seed
                let cy = self.grid_height / 2;
                let half_len = 20.min(self.grid_width / 4);
                let start_x = self.grid_width / 2 - half_len;
                let end_x = self.grid_width / 2 + half_len;
                for x in start_x..end_x {
                    self.grid[cy * self.grid_width + x] = Some(0);
                }
                self.particles_stuck = (end_x - start_x) as usize;
                self.max_radius = half_len as f32;
            }
            SeedPattern::Cross => {
                // Cross seed
                let cx = self.grid_width / 2;
                let cy = self.grid_height / 2;
                let arm_len = 10.min(self.grid_width / 8).min(self.grid_height / 8);
                let mut count = 0;
                for i in 0..arm_len {
                    if cx >= i && cy >= i {
                        self.grid[cy * self.grid_width + (cx - i)] = Some(0);
                        self.grid[cy * self.grid_width + (cx + i)] = Some(0);
                        self.grid[(cy - i) * self.grid_width + cx] = Some(0);
                        self.grid[(cy + i) * self.grid_width + cx] = Some(0);
                        count += 4;
                    }
                }
                self.particles_stuck = count;
                self.max_radius = arm_len as f32;
            }
            SeedPattern::Circle => {
                // Circle seed
                let cx = self.grid_width as f32 / 2.0;
                let cy = self.grid_height as f32 / 2.0;
                let radius = 15.0_f32.min((self.grid_width / 8) as f32).min((self.grid_height / 8) as f32);
                let mut count = 0;
                for angle_deg in 0..360 {
                    let angle = (angle_deg as f32).to_radians();
                    let x = (cx + radius * angle.cos()) as usize;
                    let y = (cy + radius * angle.sin()) as usize;
                    if x < self.grid_width && y < self.grid_height {
                        let idx = y * self.grid_width + x;
                        if self.grid[idx].is_none() {
                            self.grid[idx] = Some(0);
                            count += 1;
                        }
                    }
                }
                self.particles_stuck = count;
                self.max_radius = radius;
            }
        }

        self.paused = false;
    }

    /// Get cell state at (x, y)
    pub fn get_cell(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.grid_width && y < self.grid_height {
            self.grid[y * self.grid_width + x]
        } else {
            None
        }
    }

    /// Get simulation progress as a ratio (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        self.particles_stuck as f32 / self.num_particles as f32
    }

    /// Check if simulation is complete
    pub fn is_complete(&self) -> bool {
        self.particles_stuck >= self.num_particles
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Resize the simulation grid
    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        if new_width != self.grid_width || new_height != self.grid_height {
            self.grid_width = new_width;
            self.grid_height = new_height;
            self.reset();
        }
    }

    /// Adjust num_particles (clamped to 100-10000)
    pub fn adjust_particles(&mut self, delta: i32) {
        let new_val = (self.num_particles as i32 + delta).clamp(100, 10000) as usize;
        self.num_particles = new_val;
    }

    /// Adjust stickiness (clamped to 0.1-1.0)
    pub fn adjust_stickiness(&mut self, delta: f32) {
        self.stickiness = (self.stickiness + delta).clamp(0.1, 1.0);
    }
}
