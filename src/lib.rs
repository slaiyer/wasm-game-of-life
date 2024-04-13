mod utils;

use std::str::FromStr;

use wasm_bindgen::prelude::*;

extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

extern crate js_sys;
use js_sys::Math;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

#[wasm_bindgen]
#[derive(Default)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

enum Pattern {
    Glider,
    Pulsar,
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GLIDER" => Ok(Pattern::Glider),
            "PULSAR" => Ok(Pattern::Pulsar),
            _ => Err(()),
        }
    }
}

const GLIDER_ALIVE_OFFSET: [(i32, i32); 5] = [(-1, 1), (0, -1), (0, 1), (1, 0), (1, 1)];
const PULSAR_ALIVE_OFFSET: [(i32, i32); 48] = [
    // NW
    (-6, -4),
    (-6, -3),
    (-6, -2),
    (-4, -6),
    (-3, -6),
    (-2, -6),
    (-4, -1),
    (-3, -1),
    (-2, -1),
    (-1, -4),
    (-1, -3),
    (-1, -2),
    // NE
    (-6, 4),
    (-6, 3),
    (-6, 2),
    (-4, 6),
    (-3, 6),
    (-2, 6),
    (-4, 1),
    (-3, 1),
    (-2, 1),
    (-1, 4),
    (-1, 3),
    (-1, 2),
    // SW
    (6, -4),
    (6, -3),
    (6, -2),
    (4, -6),
    (3, -6),
    (2, -6),
    (4, -1),
    (3, -1),
    (2, -1),
    (1, -4),
    (1, -3),
    (1, -2),
    // SE
    (6, 4),
    (6, 3),
    (6, 2),
    (4, 6),
    (3, 6),
    (2, 6),
    (4, 1),
    (3, 1),
    (2, 1),
    (1, 4),
    (1, 3),
    (1, 2),
];

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
    }

    pub fn new(chance_of_life: Option<f64>) -> Universe {
        utils::set_panic_hook();

        let width = 256;
        let height = 256;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for idx in 0..width * height {
            cells.set(
                idx as usize,
                match chance_of_life {
                    Some(chance) => Math::random() < chance,
                    None => Math::random() < 0.1,
                },
            );
        }

        log! {
            "Universe created with width: {}, height: {}, alive cells: {}",
            width,
            height,
            cells.count_ones(0..cells.len())
        };

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr() as *const u32
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (self.width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
        self.cells.clear();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
        self.cells.clear();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn deploy(&mut self, pattern: &str, row: u32, column: u32) {
        let pattern = match pattern.parse::<Pattern>() {
            Ok(pattern) => match pattern {
                Pattern::Glider => GLIDER_ALIVE_OFFSET.iter(),
                Pattern::Pulsar => PULSAR_ALIVE_OFFSET.iter(),
            },
            Err(_) => return,
        };

        let alive_cells = pattern
            .map(|(delta_row, delta_col)| {
                let row = (row as i32 + delta_row) as u32;
                let col = (column as i32 + delta_col) as u32;
                (row % self.height, col % self.width)
            })
            .collect::<Vec<_>>();

        self.set_cells(&alive_cells);
    }
}
