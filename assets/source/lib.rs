#![allow(clippy::unused_unit)]
extern crate js_sys;

use wasm_bindgen::prelude::*;

mod aoc_2021;
mod aoc_2022;
mod common;
mod utils;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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
pub enum Part {
    First = 0,
    Second = 1,
}

#[allow(unused)]
#[wasm_bindgen]
pub fn run(year: usize, day: usize, part: Part, input: &str) -> String {
    utils::set_panic_hook();
    match (year, day, part) {
        // 2021
        (2021, 1, Part::First) => aoc_2021::day01::run_first(input),
        (2021, 1, Part::Second) => aoc_2021::day01::run_second(input),
        (2021, 2, Part::First) => aoc_2021::day02::run_first(input),
        (2021, 2, Part::Second) => aoc_2021::day02::run_second(input),
        (2021, 3, Part::First) => aoc_2021::day03::run_first(input),
        (2021, 3, Part::Second) => aoc_2021::day03::run_second(input),
        (2021, 4, Part::First) => aoc_2021::day04::run_first(input),
        (2021, 4, Part::Second) => aoc_2021::day04::run_second(input),

        // 2022
        (2022, 1, Part::First) => aoc_2022::day01::run_first(input),
        (2022, 1, Part::Second) => aoc_2022::day01::run_second(input),
        (2022, 2, Part::First) => aoc_2022::day02::run_first(input),
        (2022, 2, Part::Second) => aoc_2022::day02::run_second(input),
        (2022, 3, Part::First) => aoc_2022::day03::run_first(input),
        (2022, 3, Part::Second) => aoc_2022::day03::run_second(input),
        (_, _, _) => panic!("Not Implemented"),
    }
}
