use ggez::{
    conf, event, graphics, input, mint, nalgebra as na, timer, Context, ContextBuilder, GameResult,
};
use std::iter;

//window stuff
const HEIGHT: f32 = 720.0;
const WIDTH: f32 = HEIGHT * (16.0 / 9.0);

//algorithm stuff
const SPEED_LIMIT: f32 = 400.0; // Pixels per second
const VISUAL_RANGE: f32 = 32.0; // Pixels
const MIN_DISTANCE: f32 = 16.0; // Pixels

//drawing stuff
const NUM_BOIDS: usize = 1500; // n
const BOID_SIZE: f32 = 16.0; // Pixels

#[derive(Debug, Clone, Copy)]
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
                //rgb
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                (rand::random::<f32>() * 128.0 + 128.0) / 255.0,
                0.5,
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

    fn limit_speed(&mut self) {
        let speed = (self.dx * self.dx + self.dy * self.dy).sqrt();
        if speed > SPEED_LIMIT {
            self.dx = (self.dx / speed) * SPEED_LIMIT;
            self.dy = (self.dy / speed) * SPEED_LIMIT;
        }
    }

    fn keep_within_bounds(&mut self, cursor: mint::Point2<f32>) {
        let edge_buffer: f32 = 40.0;
        let turn_factor: f32 = 16.0;
        let mut x_bounded = true;
        let mut y_bounded = true;

        if self.x < WIDTH - edge_buffer {
            self.dx += turn_factor;
            x_bounded = !x_bounded;
        }
        if self.x > edge_buffer {
            self.dx -= turn_factor;
            x_bounded = !x_bounded;
        }
        if self.y < HEIGHT - edge_buffer {
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

fn get_boids() -> Vec<Boid> {
    iter::repeat_with(|| Boid::new()).take(NUM_BOIDS).collect()
}

struct State {
    dt: std::time::Duration,
    boids: Vec<Boid>,
    points: Vec<na::Point2<f32>>,
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        State {
            dt: std::time::Duration::new(0, 0),
            boids: get_boids(),
            points: vec![
                na::Point2::new(0.0, -BOID_SIZE / 2.0),
                na::Point2::new(BOID_SIZE / 4.0, BOID_SIZE / 2.0),
                na::Point2::new(0.0, BOID_SIZE / 3.0),
                na::Point2::new(-BOID_SIZE / 4.0, BOID_SIZE / 2.0),
            ],
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
            b.keep_within_bounds(input::mouse::position(ctx));
            b.limit_speed();

            //Convert new velocity to postion change
            b.x += b.dx * tick;
            b.y += b.dy * tick;

            self.boids[i] = b;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.15, 0.2, 0.22, 1.0].into());

        let mb = &mut graphics::MeshBuilder::new();
        for boid in &self.boids {
            let rot = na::Rotation2::new(boid.dx.atan2(-boid.dy));
            let pos = na::Vector2::new(boid.x, boid.y);
            mb.polygon(
                graphics::DrawMode::fill(),
                &[
                    (rot * self.points[0]) + pos,
                    (rot * self.points[1]) + pos,
                    (rot * self.points[2]) + pos,
                    (rot * self.points[3]) + pos,
                ],
                boid.color.into(),
            )?;
        }
        /*Highlight cursor..*/
        mb.circle(
            graphics::DrawMode::fill(),
            input::mouse::position(ctx),
            10.0,
            0.1,
            [1.0, 1.0, 1.0, 0.5].into(),
        );
        let m = mb.build(ctx)?;
        graphics::draw(ctx, &m, graphics::DrawParam::new())?;
        graphics::present(ctx)
    }
}

fn main() {
    let (mut ctx, mut events_loop) = ContextBuilder::new("GOL", "Daniel Eisen")
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
        .window_setup(conf::WindowSetup::default().samples(conf::NumSamples::Eight))
        .build()
        .expect("Failed to create context");

    let mut state = State::new(&mut ctx);

    match event::run(&mut ctx, &mut events_loop, &mut state) {
        Ok(_) => println!("Exited Cleanly "),
        Err(e) => println!("Error: {}", e),
    }
}
