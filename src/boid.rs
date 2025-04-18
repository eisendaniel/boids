use ggez::mint;

// Algorithm constants - exposed for easy tuning
pub const SPEED_LIMIT: f32 = 400.0; // Pixels per second
pub const VISUAL_RANGE: f32 = 32.0; // Pixels
pub const MIN_DISTANCE: f32 = 16.0; // Pixels
pub const AVOID_FACTOR: f32 = 0.5;
pub const CENTERING_FACTOR: f32 = 0.05;
pub const MATCHING_FACTOR: f32 = 0.1;
pub const TURN_FACTOR: f32 = 16.0;
pub const EDGE_BUFFER: f32 = 40.0;

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub color: [f32; 4],
}

impl Boid {
    pub fn new(win_width: f32, win_height: f32) -> Boid {
        Boid {
            x: (rand::random::<f32>() * win_width / 2.0 + win_width / 4.0),
            y: (rand::random::<f32>() * win_height / 2.0 + win_height / 4.0),
            dx: (rand::random::<f32>() - 0.5) * SPEED_LIMIT,
            dy: (rand::random::<f32>() - 0.5) * SPEED_LIMIT,
            color: [
                //rgb
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                0.5,
            ],
        }
    }

    // Combined behavior calculation - reduces redundant distance calculations
    // and neighbor finding operations
    pub fn calculate_behaviors(&mut self, neighbor_indices: &[usize], boids: &[Boid]) {
        // Initialize accumulators
        let mut avoid_x = 0.0;
        let mut avoid_y = 0.0;
        
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        
        let mut avg_dx = 0.0;
        let mut avg_dy = 0.0;
        
        let mut num_neighbors = 0.0;
        let mut num_close = 0;
        
        // Process all neighbors in a single pass
        for &idx in neighbor_indices {
            let other = &boids[idx];
            
            // Don't process itself
            if self.x == other.x && self.y == other.y {
                continue;
            }
            
            // Fast squared distance check for performance
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            let squared_dist = dx * dx + dy * dy;
            
            // Avoidance (close range)
            if squared_dist < MIN_DISTANCE * MIN_DISTANCE {
                avoid_x += dx;
                avoid_y += dy;
                num_close += 1;
            }
            
            // Attraction and velocity matching (visual range)
            if squared_dist < VISUAL_RANGE * VISUAL_RANGE {
                center_x += other.x;
                center_y += other.y;
                avg_dx += other.dx;
                avg_dy += other.dy;
                num_neighbors += 1.0;
            }
        }
        
        // Apply avoidance behavior
        if num_close > 0 {
            self.dx += avoid_x * AVOID_FACTOR;
            self.dy += avoid_y * AVOID_FACTOR;
        }
        
        // Apply centering behavior
        if num_neighbors > 0.0 {
            center_x /= num_neighbors;
            center_y /= num_neighbors;
            self.dx += (center_x - self.x) * CENTERING_FACTOR;
            self.dy += (center_y - self.y) * CENTERING_FACTOR;
            
            // Apply velocity matching
            avg_dx /= num_neighbors;
            avg_dy /= num_neighbors;
            self.dx += (avg_dx - self.dx) * MATCHING_FACTOR;
            self.dy += (avg_dy - self.dy) * MATCHING_FACTOR;
        }
    }

    // Legacy methods kept for compatibility, but they delegate to calculate_behaviors
    // in the optimized implementation
    pub fn avoid_others(&mut self, _boids: &[Boid]) {
        // This is now handled by calculate_behaviors
    }

    pub fn fly_towards_center(&mut self, _boids: &[Boid]) {
        // This is now handled by calculate_behaviors
    }

    pub fn match_velocity(&mut self, _boids: &[Boid]) {
        // This is now handled by calculate_behaviors
    }

    // Optimized speed limit check with fast square root approximation
    pub fn limit_speed(&mut self) {
        let squared_speed = self.dx * self.dx + self.dy * self.dy;
        if squared_speed > SPEED_LIMIT * SPEED_LIMIT {
            let ratio = SPEED_LIMIT / squared_speed.sqrt();
            self.dx *= ratio;
            self.dy *= ratio;
        }
    }

    // Optimized boundary check with early returns
    pub fn keep_within_bounds(
        &mut self,
        cursor: mint::Point2<f32>,
        win_width: f32,
        win_height: f32,
    ) {
        let mut x_bounded = true;
        let mut y_bounded = true;

        // Check and adjust for x boundaries
        if self.x < EDGE_BUFFER {
            self.dx += TURN_FACTOR;
            x_bounded = false;
        } else if self.x > win_width - EDGE_BUFFER {
            self.dx -= TURN_FACTOR;
            x_bounded = false;
        }
        
        // Check and adjust for y boundaries
        if self.y < EDGE_BUFFER {
            self.dy += TURN_FACTOR;
            y_bounded = false;
        } else if self.y > win_height - EDGE_BUFFER {
            self.dy -= TURN_FACTOR;
            y_bounded = false;
        }
        
        // Apply damping if needed
        if !x_bounded {
            self.dx *= 0.8;
        }
        if !y_bounded {
            self.dy *= 0.8;
        }
        
        // Avoid mouse cursor with fast squared distance
        let dx_cursor = self.x - cursor.x;
        let dy_cursor = self.y - cursor.y;
        let squared_dist_cursor = dx_cursor * dx_cursor + dy_cursor * dy_cursor;
        
        if squared_dist_cursor < 400.0 { // 20.0^2 = 400.0
            self.dx += dx_cursor * 1.0;
            self.dy += dy_cursor * 1.0;
        }
    }
    
    // Fast squared distance calculation for performance
    #[inline]
    pub fn squared_distance(&self, other: &Boid) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    
    // Legacy distance method for compatibility
    #[inline]
    fn distance(&self, boid: &Boid) -> f32 {
        self.squared_distance(boid).sqrt()
    }
    
    // Update position based on velocity
    #[inline]
    pub fn update_position(&mut self, tick: f32) {
        self.x += self.dx * tick;
        self.y += self.dy * tick;
    }
}

// This function has been removed due to borrowing issues - we'll use a sequential approach in main.rs
// pub fn update_boids_parallel(boids: &mut [Boid], neighbor_lists: &[Vec<usize>], tick: f32) {
//     // Implementation removed
// }
