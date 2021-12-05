#![deny(
	warnings,
	nonstandard_style,
	unused,
	future_incompatible,
	rust_2018_idioms,
	unsafe_code
)]

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

aoc_lib! { year = 2021 }
