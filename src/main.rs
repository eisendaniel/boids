use ggez::*;
use std::{f32::consts::PI, iter};

//window stuff
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = WIDTH;

//algorithm stuff
const SPEED_LIMIT: f32 = 300.0; // Pixels per second
const VISUAL_RANGE: f32 = 32.0; // Pixels
const MIN_DISTANCE: f32 = 12.0; // Pixels

//drawing stuff
const NUM_BOIDS: usize = 1; // n
const BOID_SIZE: f32 = 12.0; // Pixels

#[derive(Clone, Copy)]
struct Boid {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: [f32; 4],
}

impl Boid {
    pub fn new() -> Boid {
        Boid {
            x: (rand::random::<f32>() * WIDTH / 2.0 + WIDTH / 4.0),
            y: (rand::random::<f32>() * HEIGHT / 2.0 + HEIGHT / 4.0),
            dx: (rand::random::<f32>() - 0.5) * SPEED_LIMIT,
            dy: (rand::random::<f32>() - 0.5) * SPEED_LIMIT,
            color: [
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                1.0,
            ],
        }
    }
    fn avoid_others(&mut self, boids: &Vec<Boid>) {
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

    fn limit_speed(&mut self) {
        let speed = (self.dx * self.dx + self.dy * self.dy).sqrt();
        if speed > SPEED_LIMIT {
            self.dx = (self.dx / speed) * SPEED_LIMIT;
            self.dy = (self.dy / speed) * SPEED_LIMIT;
        }
    }

    fn keep_within_bounds(&mut self /*, cursor: &[f32; 2]*/) {
        let margin: f32 = WIDTH - 40.0;
        let turn_factor: f32 = 8.0;
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
        /*Dorment Cursor Code from piston*/
        // if ((self.x - cursor[0]).powi(2) + (self.y - cursor[1]).powi(2)).sqrt() < 20.0 {
        //     self.dx += (self.x - cursor[0]) * 0.7;
        //     self.dy += (self.y - cursor[1]) * 0.7;
        // }
    }
    fn distance(&self, boid: &Boid) -> f32 {
        ((self.x - boid.x).powi(2) + (self.y - boid.y).powi(2)).sqrt()
    }
}

fn get_boids() -> Vec<Boid> {
    iter::repeat_with(|| Boid::new()).take(NUM_BOIDS).collect()
}

struct State {
    dt: std::time::Duration,
    boids: Vec<Boid>,
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        State {
            dt: std::time::Duration::new(0, 0),
            boids: get_boids(),
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        let tick = (self.dt.subsec_millis() as f32) / 1000.0;

        for i in 0..(self.boids).len() {
            let mut b = self.boids[i];
            b.fly_towards_center(&self.boids);
            b.avoid_others(&self.boids);
            b.match_velocity(&self.boids);
            b.limit_speed();
            b.keep_within_bounds();

            b.x += b.dx * tick;
            b.y += b.dy * tick;

            self.boids[i] = b;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.15, 0.2, 0.22, 1.0].into());
        for boid in &self.boids {
            let rect = graphics::Rect::new(
                boid.x - BOID_SIZE / 2.0,
                boid.y - BOID_SIZE / 2.0,
                BOID_SIZE,
                BOID_SIZE,
            );
            let r1 = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                boid.color.into(),
            )?;
            graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
        }
        /*Highlight cursor..*/

        graphics::present(ctx)
    }
}

fn main() {
    let (mut ctx, mut events_loop) = ContextBuilder::new("GOL", "eisendaniel")
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
        .build()
        .expect("Failed to create context");

    let mut state = State::new(&mut ctx);

    match event::run(&mut ctx, &mut events_loop, &mut state) {
        Ok(_) => println!("Exited Cleanly "),
        Err(e) => println!("Error: {}", e),
    }
}
