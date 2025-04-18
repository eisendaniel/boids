mod boid;
mod spatial_grid;

use boid::{Boid, VISUAL_RANGE};
use spatial_grid::SpatialGrid;

use ggez::{
    conf,
    event,
    graphics,
    input,
    timer,
    Context,
    ContextBuilder,
    GameResult,
};
use std::time::Instant;

// Window dimensions
const HEIGHT: f32 = 720.0;
const WIDTH: f32 = HEIGHT * (16.0 / 9.0);

// Boid settings
const NUM_BOIDS: usize = 1000; // Starting with 1000 boids
const BOID_SIZE: f32 = 32.0;   // Pixels

// Performance settings
const CELL_SIZE: f32 = VISUAL_RANGE; // Cell size for spatial partitioning
const FPS_TARGET: u32 = 30;          // Target fps

// Rendering settings
const DRAW_SPATIAL_GRID: bool = false; // Set to true to visualize the spatial grid

fn get_boids(count: usize) -> Vec<Boid> {
    std::iter::repeat_with(|| Boid::new(WIDTH, HEIGHT))
        .take(count)
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
    boids: Vec<Boid>,
    spatial_grid: SpatialGrid,
    points: Vec<glam::Vec2>,
    fps_display: graphics::Text,
    frames: usize,
    frame_time: std::time::Duration,
    boid_count: usize,
    mesh_cache: Option<graphics::Mesh>, // Cache for static parts of the mesh
    last_update_time: Instant,          // For measuring time spent in update
    last_draw_time: Instant,            // For measuring time spent in draw
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        // Create initial boids
        let boids = get_boids(NUM_BOIDS);
        
        // Create spatial grid for efficient neighbor lookups
        let spatial_grid = SpatialGrid::new(WIDTH, HEIGHT, CELL_SIZE);
        
        State {
            state: PlayState::Setup,
            dt: std::time::Duration::new(0, 0),
            boids,
            spatial_grid,
            points: vec![
                glam::vec2(0.0, -BOID_SIZE / 2.0),
                glam::vec2(BOID_SIZE / 4.0, BOID_SIZE / 2.0),
                glam::vec2(0.0, BOID_SIZE / 3.0),
                glam::vec2(-BOID_SIZE / 4.0, BOID_SIZE / 2.0),
            ],
            fps_display: graphics::Text::new(graphics::TextFragment {
                text: "FPS: 0".to_string(),
                color: Some(graphics::Color::WHITE),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::PxScale::from(20.0)),
            }),
            frames: 0,
            frame_time: std::time::Duration::new(0, 0),
            boid_count: NUM_BOIDS,
            mesh_cache: None,
            last_update_time: Instant::now(),
            last_draw_time: Instant::now(),
        }
    }
    
    // Helper function to adjust the number of boids
    fn adjust_boid_count(&mut self, increase: bool, _ctx: &mut Context) {
        if increase {
            self.boid_count += 500; // Increase by 500 instead of 100
        } else if self.boid_count > 500 {
            self.boid_count -= 500; // Decrease by 500 instead of 100
        }
        
        // Update boids
        self.boids = get_boids(self.boid_count);
            
        println!("Boid count: {}", self.boid_count);
    }
    
    // Update the spatial grid with current boid positions
    fn update_spatial_grid(&mut self) {
        self.spatial_grid.clear();
        
        for (i, boid) in self.boids.iter().enumerate() {
            self.spatial_grid.insert(i, boid);
        }
    }
    
    // Get neighbor lists for all boids using spatial partitioning
    // Fixed to not use parallelism due to Sync trait issues
    fn get_all_neighbor_lists(&self) -> Vec<Vec<usize>> {
        self.boids.iter()
            .map(|boid| self.spatial_grid.get_neighbors(boid, VISUAL_RANGE))
            .collect()
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let _update_start = Instant::now();
        self.dt = timer::delta(ctx);
        let tick = (self.dt.subsec_millis() as f32) / 1000.0;
        let pressed_keys = input::keyboard::pressed_keys(ctx);
        
        // Update frame counter for FPS calculation
        self.frames += 1;
        self.frame_time += self.dt;
        
        // Update FPS display every second
        if self.frame_time.as_secs_f32() >= 1.0 {
            let fps = self.frames as f32 / self.frame_time.as_secs_f32();
            let update_time = self.last_update_time.elapsed().as_micros() as f32 / self.frames as f32;
            let draw_time = self.last_draw_time.elapsed().as_micros() as f32 / self.frames as f32;
            
            self.fps_display = graphics::Text::new(graphics::TextFragment {
                text: format!(
                    "FPS: {:.1} | Boids: {} | Update: {:.1}μs | Draw: {:.1}μs", 
                    fps, self.boid_count, update_time, draw_time
                ),
                color: Some(graphics::Color::WHITE),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::PxScale::from(20.0)),
            });
            
            self.frames = 0;
            self.frame_time = std::time::Duration::new(0, 0);
            self.last_update_time = Instant::now();
            self.last_draw_time = Instant::now();
            
            // Print performance information
            println!("Current FPS: {:.1} with {} boids | Update: {:.1}μs | Draw: {:.1}μs", 
                     fps, self.boid_count, update_time, draw_time);
        }

        match self.state {
            PlayState::Setup => {
                self.boids.drain(..);
                if pressed_keys.contains(&event::KeyCode::Space) {
                    self.boids = get_boids(self.boid_count);
                    self.state = PlayState::Play;
                }
            }

            PlayState::Pause => {
                if pressed_keys.contains(&event::KeyCode::Space) {
                    self.state = PlayState::Play;
                } else if pressed_keys.contains(&event::KeyCode::R) {
                    self.state = PlayState::Setup;
                } else if pressed_keys.contains(&event::KeyCode::Up) {
                    self.adjust_boid_count(true, ctx);
                } else if pressed_keys.contains(&event::KeyCode::Down) {
                    self.adjust_boid_count(false, ctx);
                }
            }

            PlayState::Play => {
                if pressed_keys.contains(&event::KeyCode::P) {
                    self.state = PlayState::Pause;
                } else if pressed_keys.contains(&event::KeyCode::R) {
                    self.state = PlayState::Setup;
                } else if pressed_keys.contains(&event::KeyCode::Up) {
                    self.adjust_boid_count(true, ctx);
                } else if pressed_keys.contains(&event::KeyCode::Down) {
                    self.adjust_boid_count(false, ctx);
                }

                // Update spatial grid
                self.update_spatial_grid();
                
                // Get neighbor lists for all boids
                let neighbor_lists = self.get_all_neighbor_lists();
                
                // Update boids movement - non-parallel version
                for i in 0..self.boids.len() {
                    // Make a copy of the boid to work with
                    let mut boid = self.boids[i];
                    boid.calculate_behaviors(&neighbor_lists[i], &self.boids);
                    boid.limit_speed();
                    boid.update_position(tick);
                    // Store the modified boid back in the collection
                    self.boids[i] = boid;
                }
                
                // Handle boundary checks and mouse interactions
                let mouse_pos = input::mouse::position(ctx);
                for boid in &mut self.boids {
                    boid.keep_within_bounds(mouse_pos, WIDTH, HEIGHT);
                }
            }
        };
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let draw_start = Instant::now();
        graphics::clear(ctx, [0.15, 0.2, 0.22, 1.0].into());

        match self.state {
            PlayState::Setup => {
                let menu_text = graphics::Text::new(graphics::TextFragment {
                    text: "play : <space>\npause : <p>\nreset : <r>\nadd boids : <up>\nreduce boids : <down>".to_string(),
                    color: Some(graphics::Color::WHITE),
                    font: Some(graphics::Font::default()),
                    scale: Some(graphics::PxScale::from(60.0)),
                });

                let text_pos = glam::vec2(
                    (WIDTH - menu_text.width(ctx) as f32) / 2.0,
                    (HEIGHT - menu_text.height(ctx) as f32) / 2.0,
                );

                graphics::draw(ctx, &menu_text,
                               graphics::DrawParam::default().dest(text_pos))?;
            }

            _ => {
                let mb = &mut graphics::MeshBuilder::new();
                
                // Draw boids using instanced rendering if possible, otherwise fallback to individual draws
                if self.boids.len() > 0 {
                    // For each boid, compute its transform matrix and add it to the mesh
                    for boid in &self.boids {
                        let rot = glam::Mat2::from_angle(boid.dx.atan2(-boid.dy));
                        let pos = glam::vec2(boid.x, boid.y);
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
                }
                
                // Draw spatial grid for debugging if enabled
                if DRAW_SPATIAL_GRID {
                    for x in 0..=(WIDTH / CELL_SIZE) as usize {
                        let x_pos = x as f32 * CELL_SIZE;
                        mb.line(
                            &[
                                glam::vec2(x_pos, 0.0),
                                glam::vec2(x_pos, HEIGHT),
                            ],
                            1.0,
                            [0.5, 0.5, 0.5, 0.3].into(),
                        )?;
                    }
                    
                    for y in 0..=(HEIGHT / CELL_SIZE) as usize {
                        let y_pos = y as f32 * CELL_SIZE;
                        mb.line(
                            &[
                                glam::vec2(0.0, y_pos),
                                glam::vec2(WIDTH, y_pos),
                            ],
                            1.0,
                            [0.5, 0.5, 0.5, 0.3].into(),
                        )?;
                    }
                }
                
                // Draw cursor highlight
                mb.circle(
                    graphics::DrawMode::fill(),
                    input::mouse::position(ctx),
                    10.0,
                    0.1,
                    [1.0, 1.0, 1.0, 0.5].into(),
                )?;
                
                // Build and draw the mesh
                let m = mb.build(ctx)?;
                graphics::draw(ctx, &m, graphics::DrawParam::new())?;
                
                // Draw the FPS display in the top-left corner
                graphics::draw(
                    ctx,
                    &self.fps_display,
                    graphics::DrawParam::default().dest(glam::vec2(10.0, 10.0)),
                )?;
            }
        };

        // Track time spent in draw
        self.last_draw_time = draw_start;
        
        graphics::present(ctx)
    }
}

fn main() {
    // Create a context with MSAA anti-aliasing
    let (mut ctx, events_loop) = ContextBuilder::new("Boids", "Daniel Eisen")
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
        .window_setup(conf::WindowSetup::default()
            .title("Optimized Boids")
            .samples(conf::NumSamples::Four)) // Reduced from Eight to Four for performance
        .build()
        .expect("Failed to create context");

    let state = State::new(&mut ctx);
    event::run(ctx, events_loop, state);
}
