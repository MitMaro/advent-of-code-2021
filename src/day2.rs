use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
pub enum Direction {
	Forward,
	Up,
	Down,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Direction, u32)> {
	input
		.lines()
		.map(|v| {
			if v.starts_with("forward") {
				(Direction::Forward, u32::from_str(&v[8..]).unwrap())
			}
			else if v.starts_with("down") {
				(Direction::Down, u32::from_str(&v[5..]).unwrap())
			}
			else {
				(Direction::Up, u32::from_str(&v[3..]).unwrap())
			}
		})
		.collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<(Direction, u32)>) -> u32 {
	let mut location: (u32, u32) = (0, 0);

	for (d, l) in input {
		match d {
			Direction::Forward => location.0 += l,
			Direction::Up => location.1 -= l,
			Direction::Down => location.1 += l,
		}
	}

	location.0 * location.1
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<(Direction, u32)>) -> u32 {
	let mut location: (u32, u32, u32) = (0, 0, 0);

	for (d, l) in input {
		match d {
			Direction::Forward => {
				location.2 += l;
				location.1 += location.0 * l;
			},
			Direction::Up => location.0 -= l,
			Direction::Down => location.0 += l,
		}
	}

	location.1 * location.2
}
