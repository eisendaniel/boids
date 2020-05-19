use piston_window::*;
use std::iter;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 640.0;

const SPEED_LIMIT: f64 = 300.0;
const VISUAL_RANGE: f64 = 32.0;
const NUM_BOIDS: usize = 256;

#[derive(Clone, Copy)]
struct Boid {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    color: [f32; 4],
}

impl Boid {
    pub fn new() -> Boid {
        Boid {
            x: (rand::random::<f64>() * WIDTH / 2.0 + WIDTH / 4.0),
            y: (rand::random::<f64>() * HEIGHT / 2.0 + HEIGHT / 4.0),
            dx: (rand::random::<f64>() - 0.5) * 320.0,
            dy: (rand::random::<f64>() - 0.5) * 320.0,
            color: [
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                1.0,
            ],
        }
    }

    fn avoid_others(&mut self, boids: &Vec<Boid>) {
        let min_distance = 16.0;
        let avoid_factor = 0.5;
        let mut move_x = 0.0;
        let mut move_y = 0.0;
        for other in boids {
            let dist = self.distance(other);
            if dist < min_distance && dist > 0.0 {
                move_x += self.x - other.x;
                move_y += self.y - other.y;
            }
        }
        self.dx += move_x * avoid_factor;
        self.dy += move_y * avoid_factor;
    }

    fn fly_towards_center(&mut self, boids: &Vec<Boid>) {
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

    fn match_velocity(&mut self, boids: &Vec<Boid>) {
        let matching_factor = 0.05;
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
    fn flock(&mut self, boids: &Vec<Boid>) {
        let centering_factor = 0.05; // adjust velocity by this %
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        let mut num_neighbors_c = 0.0;
        let min_distance = 16.0;
        let avoid_factor = 0.5;
        let mut move_x = 0.0;
        let mut move_y = 0.0;
        let matching_factor = 0.05;
        let mut avg_dx = 0.0;
        let mut avg_dy = 0.0;
        let mut num_neighbors_v = 0.0;
        for other in boids {
            let dist = self.distance(other);

            if dist < VISUAL_RANGE {
                center_x += other.x;
                center_y += other.y;
                num_neighbors_c += 1.0;
            }
            if dist < min_distance && dist > 0.0 {
                move_x += self.x - other.x;
                move_y += self.y - other.y;
            }
            if dist < VISUAL_RANGE {
                avg_dx += other.dx;
                avg_dy += other.dy;
                num_neighbors_v += 1.0;
            }
        }
        if num_neighbors_c > 0.0 {
            center_x = center_x / num_neighbors_c;
            center_y = center_y / num_neighbors_c;

            self.dx += (center_x - self.x) * centering_factor;
            self.dy += (center_y - self.y) * centering_factor;
        }
        self.dx += move_x * avoid_factor;
        self.dy += move_y * avoid_factor;
        if num_neighbors_v > 0.0 {
            avg_dx = avg_dx / num_neighbors_v;
            avg_dy = avg_dy / num_neighbors_v;

            self.dx += (avg_dx - self.dx) * matching_factor;
            self.dy += (avg_dy - self.dy) * matching_factor;
        }
    }

    fn limit_speed(&mut self) {
        let speed = (self.dx * self.dx + self.dy * self.dy).sqrt();
        if speed > SPEED_LIMIT {
            self.dx = (self.dx / speed) * SPEED_LIMIT;
            self.dy = (self.dy / speed) * SPEED_LIMIT;
        }
    }

    fn keep_within_bounds(&mut self, cursor: &[f64; 2]) {
        let margin: f64 = 600.0;
        let turn_factor: f64 = 8.0;
        if self.x < margin {
            self.dx += turn_factor;
        }
        if self.x > WIDTH - margin {
            self.dx -= turn_factor
        }
        if self.y < margin {
            self.dy += turn_factor;
        }
        if self.y > HEIGHT - margin {
            self.dy -= turn_factor;
        }
        if ((self.x - cursor[0]).powi(2) + (self.y - cursor[1]).powi(2)).sqrt() < 20.0 {
            self.dx += (self.x - cursor[0]) * 0.7;
            self.dy += (self.y - cursor[1]) * 0.7;
        }
    }
    fn distance(&self, boid: &Boid) -> f64 {
        ((self.x - boid.x).powi(2) + (self.y - boid.y).powi(2)).sqrt()
    }
}

fn get_boids() -> Vec<Boid> {
    iter::repeat_with(|| Boid::new()).take(NUM_BOIDS).collect()
}

fn main() {
    let bg_color = [38.0 / 255.0, 50.0 / 255.0, 56.0 / 255.0, 1.0];
    let mut boids: Vec<Boid> = get_boids();
    let mut cursor = [0.0, 0.0];

    let mut window: PistonWindow = WindowSettings::new("Graphics!", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        event.mouse_cursor(|pos| {
            cursor = pos;
        });

        if let Some(_) = event.render_args() {
            let triangle = [[0.0, -8.0], [-4.0, 4.0], [4.0, 4.0]];
            window.draw_2d(&event, |context, graphics, _device| {
                clear(bg_color, graphics); //clear white
                for b in &boids {
                    let angle = b.dy.atan2(b.dx) + 3.14159 / 2.0;
                    let transform = context.transform.trans(b.x, b.y).rot_rad(angle);
                    polygon(b.color, &triangle, transform, graphics);
                    //uncomment to show COM
                    // rectangle(
                    //     [1.0, 0.0, 0.0, 1.0],
                    //     rectangle::centered_square(b.x, b.y, 1.0),
                    //     context.transform,
                    //     graphics,
                    // );
                }
            });
        }
        if let Some(u) = event.update_args() {
            for i in 0..boids.len() {
                let mut b = boids[i];
                // b.fly_towards_center(&boids);
                // b.avoid_others(&boids);
                // b.match_velocity(&boids);
                b.flock(&boids);
                b.limit_speed();
                b.keep_within_bounds(&cursor);
                b.x += b.dx * u.dt;
                b.y += b.dy * u.dt;
                boids[i] = b;
            }
        }
    }
}
