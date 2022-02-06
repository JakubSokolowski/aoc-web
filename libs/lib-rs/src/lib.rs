use wasm_bindgen::prelude::*;
extern crate fixedbitset;
extern crate js_sys;
use fixedbitset::FixedBitSet;

mod utils;
mod aoc_2021;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> usize {
        (self.height * self.width) as usize
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn new(w: u32, h: u32) -> Universe {
        let width = w;
        let height = h;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.1);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn random(&mut self, probability: f64) {
        let size = self.size();
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, js_sys::Math::random() < probability);
        }
        self.cells = cells;
    }

    pub fn reset(&mut self) {
        self.cells = FixedBitSet::with_capacity(self.size());
    }

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

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        let cell = self.cells[idx];
        self.cells.set(idx, !cell);
    }

    pub fn glider(&mut self, row: u32, column: u32) {
        if self.will_fit(row as i32, column as i32, 3, 3) {
            self.cells.set(self.get_index(row + 1, column), true);
            self.cells.set(self.get_index(row, column - 1), true);
            self.cells.set(self.get_index(row - 1, column - 1), true);
            self.cells.set(self.get_index(row - 1, column), true);
            self.cells.set(self.get_index(row - 1, column + 1), true);
        }
    }

    pub fn pulsar(&mut self, row: u32, column: u32) {
        if self.will_fit(row as i32, column as i32, 14, 14) {
            self.cells.set(self.get_index(row - 6, column - 4), true);
            self.cells.set(self.get_index(row - 6, column - 3), true);
            self.cells.set(self.get_index(row - 6, column - 2), true);
            self.cells.set(self.get_index(row - 6, column + 2), true);
            self.cells.set(self.get_index(row - 6, column + 3), true);
            self.cells.set(self.get_index(row - 6, column + 4), true);

            self.cells.set(self.get_index(row - 2, column - 6), true);
            self.cells.set(self.get_index(row - 3, column - 6), true);
            self.cells.set(self.get_index(row - 4, column - 6), true);
            self.cells.set(self.get_index(row + 2, column - 6), true);
            self.cells.set(self.get_index(row + 3, column - 6), true);
            self.cells.set(self.get_index(row + 4, column - 6), true);

            self.cells.set(self.get_index(row - 2, column + 6), true);
            self.cells.set(self.get_index(row - 3, column + 6), true);
            self.cells.set(self.get_index(row - 4, column + 6), true);
            self.cells.set(self.get_index(row + 2, column + 6), true);
            self.cells.set(self.get_index(row + 3, column + 6), true);
            self.cells.set(self.get_index(row + 4, column + 6), true);

            self.cells.set(self.get_index(row - 1, column - 2), true);
            self.cells.set(self.get_index(row - 1, column - 3), true);
            self.cells.set(self.get_index(row - 1, column - 4), true);
            self.cells.set(self.get_index(row - 1, column + 2), true);
            self.cells.set(self.get_index(row - 1, column + 3), true);
            self.cells.set(self.get_index(row - 1, column + 4), true);

            self.cells.set(self.get_index(row + 1, column - 2), true);
            self.cells.set(self.get_index(row + 1, column - 3), true);
            self.cells.set(self.get_index(row + 1, column - 4), true);
            self.cells.set(self.get_index(row + 1, column + 2), true);
            self.cells.set(self.get_index(row + 1, column + 3), true);
            self.cells.set(self.get_index(row + 1, column + 4), true);

            self.cells.set(self.get_index(row + 6, column - 4), true);
            self.cells.set(self.get_index(row + 6, column - 3), true);
            self.cells.set(self.get_index(row + 6, column - 2), true);
            self.cells.set(self.get_index(row + 6, column + 2), true);
            self.cells.set(self.get_index(row + 6, column + 3), true);
            self.cells.set(self.get_index(row + 6, column + 4), true);

            self.cells.set(self.get_index(row - 2, column - 1), true);
            self.cells.set(self.get_index(row - 3, column - 1), true);
            self.cells.set(self.get_index(row - 4, column - 1), true);
            self.cells.set(self.get_index(row + 2, column - 1), true);
            self.cells.set(self.get_index(row + 3, column - 1), true);
            self.cells.set(self.get_index(row + 4, column - 1), true);

            self.cells.set(self.get_index(row - 2, column + 1), true);
            self.cells.set(self.get_index(row - 3, column + 1), true);
            self.cells.set(self.get_index(row - 4, column + 1), true);
            self.cells.set(self.get_index(row + 2, column + 1), true);
            self.cells.set(self.get_index(row + 3, column + 1), true);
            self.cells.set(self.get_index(row + 4, column + 1), true);
        }
    }

    fn will_fit(&self, row: i32, column: i32, width: i32, height: i32) -> bool {
        let w2 = width / 2 + 1;
        let h2 = height / 2 + 1;
        row - h2 > 0
            && row + h2 < self.height as i32
            && column - w2 > 0
            && column + w2 < self.width as i32
    }

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
}

impl Universe {
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}
