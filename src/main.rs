mod boid;

use ggez::{
    conf, event, graphics, input, nalgebra as na, timer, Context, ContextBuilder, GameResult,
};
use std::iter;

//window stuff
const HEIGHT: f32 = 720.0;
const WIDTH: f32 = HEIGHT * (16.0 / 9.0);

//drawing stuff
const NUM_BOIDS: usize = 64; // n
const BOID_SIZE: f32 = 16.0; // Pixels

fn get_boids() -> Vec<boid::Boid> {
    iter::repeat_with(|| boid::Boid::new(WIDTH, HEIGHT))
        .take(NUM_BOIDS)
        .collect()
}

struct State {
    dt: std::time::Duration,
    boids: Vec<boid::Boid>,
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
            b.keep_within_bounds(input::mouse::position(ctx), WIDTH, HEIGHT);
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
