mod boid;

use ggez::{
    conf, event, graphics, input, nalgebra as na, timer, Context, ContextBuilder, GameResult,
};
use std::iter;

//window stuff
const HEIGHT: f32 = 720.0;
const WIDTH: f32 = HEIGHT * (16.0 / 9.0);

//drawing stuff
const NUM_BOIDS: usize = 1024; // n
const BOID_SIZE: f32 = 16.0; // Pixels

fn get_boids() -> Vec<boid::Boid> {
    iter::repeat_with(|| boid::Boid::new(WIDTH, HEIGHT))
        .take(NUM_BOIDS)
        .collect()
}

enum PlayState {
    Setup,
    Play,
    Pause,
}

struct State {
    state: PlayState,
    dt: std::time::Duration,
    boids: Vec<boid::Boid>,
    points: Vec<na::Point2<f32>>,
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        State {
            state: PlayState::Setup,
            dt: std::time::Duration::new(0, 0),
            boids: Vec::new(),
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
        let pressed_keys = ggez::input::keyboard::pressed_keys(ctx);

        match self.state {
            PlayState::Setup => {
                self.boids.drain(..);
                if pressed_keys.contains(&ggez::event::KeyCode::Space) {
                    self.boids = get_boids();
                    self.state = PlayState::Play;
                }
            }

            PlayState::Pause => {
                let pressed_keys = ggez::input::keyboard::pressed_keys(ctx);

                if pressed_keys.contains(&ggez::event::KeyCode::Space) {
                    self.state = PlayState::Play;
                } else if pressed_keys.contains(&ggez::event::KeyCode::R) {
                    self.state = PlayState::Setup;
                }
            }

            PlayState::Play => {
                if pressed_keys.contains(&ggez::event::KeyCode::P) {
                    self.state = PlayState::Pause;
                } else if pressed_keys.contains(&ggez::event::KeyCode::R) {
                    self.state = PlayState::Setup;
                }

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
            }
        };

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.15, 0.2, 0.22, 1.0].into());

        match self.state {
            PlayState::Setup => {
                let menu_text = graphics::Text::new(graphics::TextFragment {
                    text: "play : <space>\npause : <p>\nreset : <r>".to_string(),
                    color: Some(graphics::WHITE),
                    font: Some(graphics::Font::default()),
                    scale: Some(graphics::Scale::uniform(100.0)),
                });

                let text_pos = na::Point2::new(
                    (WIDTH - menu_text.width(ctx) as f32) / 2.0,
                    (HEIGHT - menu_text.height(ctx) as f32) / 2.0,
                );

                graphics::draw(ctx, &menu_text, (text_pos,))?;
            }

            _ => {
                // let fps = timer::fps(ctx);
                // let fps_display = Text::new(format!("FPS: {}", fps));
                // graphics::draw(
                //     ctx,
                //     &fps_display,
                //     (na::Point2::new(0.0, 0.0), graphics::WHITE),
                // )?;

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
            }
        };

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
