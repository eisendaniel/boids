use ggez::mint;

//algorithm stuff
const SPEED_LIMIT: f32 = 400.0; // Pixels per second
const VISUAL_RANGE: f32 = 32.0; // Pixels
const MIN_DISTANCE: f32 = 16.0; // Pixels

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
    pub fn avoid_others(&mut self, boids: &Vec<Boid>) {
        let avoid_factor = 0.5;
        let mut move_x = 0.0;
        let mut move_y = 0.0;
        for other in boids {
            let dist = self.distance(other);
            if dist < MIN_DISTANCE && dist > 0.0 {
                move_x += self.x - other.x;
                move_y += self.y - other.y;
            }
        }
        self.dx += move_x * avoid_factor;
        self.dy += move_y * avoid_factor;
    }

    pub fn fly_towards_center(&mut self, boids: &Vec<Boid>) {
        let centering_factor = 0.05; // adjust velocity by this %
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        let mut num_neighbors = 0.0;
        for other in boids {
            if self.distance(other) < VISUAL_RANGE {
                center_x += other.x;
                center_y += other.y;
                num_neighbors += 1.0;
            }
        }
        if num_neighbors > 0.0 {
            center_x = center_x / num_neighbors;
            center_y = center_y / num_neighbors;

            self.dx += (center_x - self.x) * centering_factor;
            self.dy += (center_y - self.y) * centering_factor;
        }
    }

    pub fn match_velocity(&mut self, boids: &Vec<Boid>) {
        let matching_factor = 0.1;
        let mut avg_dx = 0.0;
        let mut avg_dy = 0.0;
        let mut num_neighbors = 0.0;
        for other in boids {
            if self.distance(other) < VISUAL_RANGE {
                avg_dx += other.dx;
                avg_dy += other.dy;
                num_neighbors += 1.0;
            }
        }
        if num_neighbors > 0.0 {
            avg_dx = avg_dx / num_neighbors;
            avg_dy = avg_dy / num_neighbors;

            self.dx += (avg_dx - self.dx) * matching_factor;
            self.dy += (avg_dy - self.dy) * matching_factor;
        }
    }

    pub fn limit_speed(&mut self) {
        let speed = (self.dx * self.dx + self.dy * self.dy).sqrt();
        if speed > SPEED_LIMIT {
            self.dx = (self.dx / speed) * SPEED_LIMIT;
            self.dy = (self.dy / speed) * SPEED_LIMIT;
        }
    }

    pub fn keep_within_bounds(&mut self, cursor: mint::Point2<f32>, win_width: f32, win_height: f32) {
        let edge_buffer: f32 = 40.0;
        let turn_factor: f32 = 16.0;
        let mut x_bounded = true;
        let mut y_bounded = true;

        if self.x < win_width - edge_buffer {
            self.dx += turn_factor;
            x_bounded = !x_bounded;
        }
        if self.x > edge_buffer {
            self.dx -= turn_factor;
            x_bounded = !x_bounded;
        }
        if self.y < win_height - edge_buffer {
            self.dy += turn_factor;
            y_bounded = !y_bounded
        }
        if self.y > edge_buffer {
            self.dy -= turn_factor;
            y_bounded = !y_bounded
        }
        if !x_bounded {
            self.dx *= 0.8;
        }
        if !y_bounded {
            self.dy *= 0.8;
        }
        if ((self.x - cursor.x).powi(2) + (self.y - cursor.y).powi(2)).sqrt() < 20.0 {
            self.dx += (self.x - cursor.x) * 1.0;
            self.dy += (self.y - cursor.y) * 1.0;
        }
    }
    fn distance(&self, boid: &Boid) -> f32 {
        ((self.x - boid.x).powi(2) + (self.y - boid.y).powi(2)).sqrt()
    }
}
