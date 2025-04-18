// spatial_grid.rs
use crate::boid::Boid;

// Spatial grid for faster neighbor lookups
pub struct SpatialGrid {
    cells: Vec<Vec<usize>>,
    cell_size: f32,
    width: usize,
    height: usize,
}

impl SpatialGrid {
    pub fn new(window_width: f32, window_height: f32, cell_size: f32) -> Self {
        let width = (window_width / cell_size).ceil() as usize;
        let height = (window_height / cell_size).ceil() as usize;
        let cells = vec![Vec::new(); width * height];
        
        SpatialGrid {
            cells,
            cell_size,
            width,
            height,
        }
    }
    
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }
    
    pub fn insert(&mut self, boid_index: usize, boid: &Boid) {
        let cell_x = (boid.x / self.cell_size).floor() as usize;
        let cell_y = (boid.y / self.cell_size).floor() as usize;
        
        // Clamp to ensure we don't go out of bounds
        let cell_x = cell_x.min(self.width - 1);
        let cell_y = cell_y.min(self.height - 1);
        
        let idx = cell_y * self.width + cell_x;
        if idx < self.cells.len() {
            self.cells[idx].push(boid_index);
        }
    }
    
    pub fn get_neighbors(&self, boid: &Boid, range: f32) -> Vec<usize> {
        let mut neighbors = Vec::new();
        
        // Calculate the cell range to check
        let cell_range = (range / self.cell_size).ceil() as usize + 1;
        let cx = (boid.x / self.cell_size).floor() as isize;
        let cy = (boid.y / self.cell_size).floor() as isize;
        
        // Check all cells in range
        for y in (cy - cell_range as isize)..=(cy + cell_range as isize) {
            if y < 0 || y >= self.height as isize {
                continue;
            }
            
            for x in (cx - cell_range as isize)..=(cx + cell_range as isize) {
                if x < 0 || x >= self.width as isize {
                    continue;
                }
                
                let idx = (y as usize) * self.width + (x as usize);
                if idx < self.cells.len() {
                    neighbors.extend_from_slice(&self.cells[idx]);
                }
            }
        }
        
        neighbors
    }
}
