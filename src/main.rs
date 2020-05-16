use piston_window::*;
use std::iter;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 640.0;
const NUMBOIDS: usize = 64;

struct Boid {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    color: [f32; 4],
}

impl Boid {
    pub fn new() -> Boid {
        let b: Boid = Boid {
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
        };
        b
    }
    fn keep_within_bounds(&mut self) {
        let margin: f64 = 620.0;
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
    }
}

fn get_boids() -> Vec<Boid> {
    iter::repeat_with(|| Boid::new()).take(NUMBOIDS).collect()
}

fn main() {
    let bg_color = [38.0 / 255.0, 50.0 / 255.0, 56.0 / 255.0, 1.0];
    let mut boids: Vec<Boid> = get_boids();

    let mut window: PistonWindow = WindowSettings::new("Graphics!", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear(bg_color, graphics); //clear white
                for b in &boids {
                    rectangle(
                        b.color,
                        [b.x - 4.0, b.y - 4.0, 8.0, 8.0],
                        context.transform,
                        graphics,
                    );
                }
            });
        }
        if let Some(u) = event.update_args() {
            for b in &mut boids {
                b.keep_within_bounds();
                b.x += b.dx * u.dt;
                b.y += b.dy * u.dt;
            }
        }
    }
}
