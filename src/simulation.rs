use rand::RngExt;
use rand::seq::SliceRandom;

struct Config {
    fish_breed_time: u32,
    shark_breed_time: u32,
    shark_start_energy: u32,
    initial_fish: usize,
    initial_sharks: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Fish { age: u32, last_update: u32 },
    Shark { age: u32, energy: u32, last_update: u32 },
}

struct World {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
    current_tick: u32,
    config: Config,
}

impl World {
    pub fn new(width: usize, height: usize, config: Config) -> Self {
        let grid = vec![Cell::Empty; width * height];
        let mut world = Self { width, height, grid, current_tick: 0, config };
        world.populate_initial_state();
        world
    }

    fn get_ix(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_coords(&self, ix: usize) -> (usize, usize) {
        (ix % self.width, ix / self.width)
    }

    fn get_wrapped_coord(&self, x: isize, max: usize) -> usize {
        let max_isize = max as isize;
        (((x % max_isize) + max_isize) % max_isize) as usize
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let ix = x as isize;
        let iy = y as isize;

        vec![
            (self.get_wrapped_coord(ix, self.width), self.get_wrapped_coord(iy - 1, self.height)),
            (self.get_wrapped_coord(ix, self.width), self.get_wrapped_coord(iy + 1, self.height)),
            (self.get_wrapped_coord(ix - 1, self.width), self.get_wrapped_coord(iy, self.height)),
            (self.get_wrapped_coord(ix + 1, self.width), self.get_wrapped_coord(iy, self.height)),
        ]
    }

    pub fn tick(&mut self) {
        self.current_tick += 1;

        for y in 0..self.height {
            for x in 0..self.width {
                let ix = self.get_ix(x, y);
                let cell = self.grid[ix];
                match cell {
                    Cell::Fish { .. } => self.process_fish(x, y),
                    Cell::Shark { age, energy, .. } => self.process_shark(x, y),
                    Cell::Empty => continue,
                }
            }
        }
    }

    fn process_fish(&mut self, x: usize, y: usize) {
        let empty_cells: Vec<(usize, usize)> = self.get_empty_neighbours(x, y);

        if !empty_cells.is_empty() {
            let mut rng = rand::rng();
            let new_ix = rng.random_range(0..empty_cells.len());
            let (new_x, new_y) = empty_cells[new_ix];
            let target_index = self.get_ix(new_x, new_y);
            let old_index = self.get_ix(x, y);
            self.grid[target_index] = self.grid[old_index];
            if let Cell::Fish { age, .. } = self.grid[old_index] && age >= self.config.fish_breed_time {
                self.grid[old_index] = Cell::Fish { age: age + 1, last_update: self.current_tick };
            } else {
                self.grid[old_index] = Cell::Empty;
            }
        } else {
            let ix = self.get_ix(x, y);
            if let Cell::Fish { age, .. } = self.grid[ix] {
                self.grid[ix] = Cell::Fish { age: age + 1, last_update: self.current_tick };
            }
        }
    }

    fn get_empty_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        self.get_neighbours(x, y)
            .into_iter()
            .filter(|&(x, y)| self.grid[self.get_ix(x, y)] == Cell::Empty)
            .collect()
    }

    fn process_shark(&mut self, x: usize, y: usize) {
        let fish_cells: Vec<(usize, usize)> = self.get_neighbours(x, y)
            .into_iter()
            .filter(|&(x, y)| matches!(self.grid[self.get_ix(x, y)], Cell::Fish { .. }))
            .collect();

        if !fish_cells.is_empty() {
            let mut rng = rand::rng();
            let new_ix = rng.random_range(0..fish_cells.len());
            let (new_x, new_y) = fish_cells[new_ix];
            let target_index = self.get_ix(new_x, new_y);
            let old_index = self.get_ix(x, y);
            if let Cell::Shark { age, .. } = self.grid[old_index] {
                self.grid[target_index] = Cell::Shark { energy: self.config.shark_start_energy, last_update: self.current_tick, age: age + 1};
                self.grid[old_index] = Cell::Empty;
            } else {
                panic!("Shark is not a shark");
            }
        } else {
            let empty_cells = self.get_empty_neighbours(x, y);

            if !empty_cells.is_empty() {
                let mut rng = rand::rng();
                let new_ix = rng.random_range(0..empty_cells.len());
                let (new_x, new_y) = empty_cells[new_ix];
                let target_index = self.get_ix(new_x, new_y);
                let old_index = self.get_ix(x, y);
                if let Cell::Shark { energy, age, .. } = self.grid[old_index] {
                    self.grid[target_index] = Cell::Shark { energy: energy - 1, last_update: self.current_tick, age: age + 1};
                    self.grid[old_index] = Cell::Empty;
                } else {
                    panic!("Shark is not a shark");
                }
            } else {
                let ix = self.get_ix(x, y);
                if let Cell::Shark { energy, age, .. } = self.grid[ix] {
                    self.grid[ix] = Cell::Shark { energy, last_update: self.current_tick, age: age + 1};
                }
            }
        }
    }

    fn populate_initial_state(&mut self) {
        let total_cells = self.width * self.height;
        let mut ixs: Vec<usize> = (0..total_cells).collect();

        let mut rng = rand::rng();
        ixs.shuffle(&mut rng);

        let n_fish = self.config.initial_fish.min(total_cells);
        let n_sharks = self.config.initial_sharks.min(total_cells - n_fish);

        for &ix in &ixs[0..n_fish] {
            let starting_age = rng.random_range(0..self.config.fish_breed_time);
            self.grid[ix] = Cell::Fish {
                age: starting_age,
                last_update: 0
            };
        }

        for &ix in &ixs[n_fish..(n_fish + n_sharks)] {
            let starting_age = rng.random_range(0..self.config.shark_breed_time);

            self.grid[ix] = Cell::Shark {
                age: starting_age,
                energy: self.config.shark_start_energy,
                last_update: 0
            };
        }
    }
}