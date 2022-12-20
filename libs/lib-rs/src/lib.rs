#![allow(clippy::unused_unit)]
extern crate js_sys;

use wasm_bindgen::prelude::*;

mod aoc_2021;
mod aoc_2022;
mod common;
pub mod utils;

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
    run_base(year, day, part, input)
}

pub fn run_base(year: usize, day: usize, part: Part, input: &str) -> String {
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
        (2021, 5, Part::First) => aoc_2021::day05::run_first(input),
        (2021, 5, Part::Second) => aoc_2021::day05::run_second(input),
        (2021, 6, Part::First) => aoc_2021::day06::run_first(input),
        (2021, 6, Part::Second) => aoc_2021::day06::run_second(input),
        (2021, 7, Part::First) => aoc_2021::day07::run_first(input),
        (2021, 7, Part::Second) => aoc_2021::day07::run_second(input),
        (2021, 8, Part::First) => aoc_2021::day08::run_first(input),
        (2021, 8, Part::Second) => aoc_2021::day08::run_second(input),
        (2021, 9, Part::First) => aoc_2021::day09::run_first(input),
        (2021, 9, Part::Second) => aoc_2021::day09::run_second(input),
        (2021, 10, Part::First) => aoc_2021::day10::run_first(input),
        (2021, 10, Part::Second) => aoc_2021::day10::run_second(input),
        (2021, 11, Part::First) => aoc_2021::day11::run_first(input),
        (2021, 11, Part::Second) => aoc_2021::day11::run_second(input),
        (2021, 12, Part::First) => aoc_2021::day12::run_first(input),
        (2021, 12, Part::Second) => aoc_2021::day12::run_second(input),
        (2021, 13, Part::First) => aoc_2021::day13::run_first(input),
        (2021, 13, Part::Second) => aoc_2021::day13::run_second(input),
        (2021, 14, Part::First) => aoc_2021::day14::run_first(input),
        (2021, 14, Part::Second) => aoc_2021::day14::run_second(input),
        (2021, 15, Part::First) => aoc_2021::day15::run_first(input),
        (2021, 15, Part::Second) => aoc_2021::day15::run_second(input),
        (2021, 16, Part::First) => aoc_2021::day16::run_first(input),
        (2021, 16, Part::Second) => aoc_2021::day16::run_second(input),
        (2021, 17, Part::First) => aoc_2021::day17::run_first(input),
        (2021, 17, Part::Second) => aoc_2021::day17::run_second(input),
        (2021, 18, Part::First) => aoc_2021::day18::run_first(input),
        (2021, 18, Part::Second) => aoc_2021::day18::run_second(input),

        // 2022
        (2022, 1, Part::First) => aoc_2022::day01::run_first(input),
        (2022, 1, Part::Second) => aoc_2022::day01::run_second(input),
        (2022, 2, Part::First) => aoc_2022::day02::run_first(input),
        (2022, 2, Part::Second) => aoc_2022::day02::run_second(input),
        (2022, 3, Part::First) => aoc_2022::day03::run_first(input),
        (2022, 3, Part::Second) => aoc_2022::day03::run_second(input),
        (2022, 4, Part::First) => aoc_2022::day04::run_first(input),
        (2022, 4, Part::Second) => aoc_2022::day04::run_second(input),
        (2022, 5, Part::First) => aoc_2022::day05::run_first(input),
        (2022, 5, Part::Second) => aoc_2022::day05::run_second(input),
        (2022, 6, Part::First) => aoc_2022::day06::run_first(input),
        (2022, 6, Part::Second) => aoc_2022::day06::run_second(input),
        (2022, 7, Part::First) => aoc_2022::day07::run_first(input),
        (2022, 7, Part::Second) => aoc_2022::day07::run_second(input),
        (2022, 8, Part::First) => aoc_2022::day08::run_first(input),
        (2022, 8, Part::Second) => aoc_2022::day08::run_second(input),
        (2022, 9, Part::First) => aoc_2022::day09::run_first(input),
        (2022, 9, Part::Second) => aoc_2022::day09::run_second(input),
        (2022, 10, Part::First) => aoc_2022::day10::run_first(input),
        (2022, 10, Part::Second) => aoc_2022::day10::run_second(input),
        (2022, 11, Part::First) => aoc_2022::day11::run_first(input),
        (2022, 11, Part::Second) => aoc_2022::day11::run_second(input),
        (2022, 12, Part::First) => aoc_2022::day12::run_first(input),
        (2022, 12, Part::Second) => aoc_2022::day12::run_second(input),
        (2022, 13, Part::First) => aoc_2022::day13::run_first(input),
        (2022, 13, Part::Second) => aoc_2022::day13::run_second(input),
        (2022, 14, Part::First) => aoc_2022::day14::run_first(input),
        (2022, 14, Part::Second) => aoc_2022::day14::run_second(input),
        (2022, 15, Part::First) => aoc_2022::day15::run_first(input),
        (2022, 15, Part::Second) => aoc_2022::day15::run_second(input),
        (2022, 16, Part::First) => aoc_2022::day16::run_first(input),
        (2022, 16, Part::Second) => aoc_2022::day16::run_second(input),
        (2022, 17, Part::First) => aoc_2022::day17::run_first(input),
        (2022, 17, Part::Second) => aoc_2022::day17::run_second(input),
        (2022, 18, Part::First) => aoc_2022::day18::run_first(input),
        (2022, 18, Part::Second) => aoc_2022::day18::run_second(input),
        (2022, 19, Part::First) => aoc_2022::day19::run_first(input),
        (2022, 19, Part::Second) => aoc_2022::day19::run_second(input),
        (2022, 20, Part::First) => aoc_2022::day20::run_first(input),
        (2022, 20, Part::Second) => aoc_2022::day20::run_second(input),
        (_, _, _) => panic!("Year {} Day {} Part {:?} not implemented", year, day, part),
    }
}
