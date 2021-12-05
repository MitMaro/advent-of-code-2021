use std::{
	cmp::{max, min},
	collections::HashMap,
	str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Vectors {
	pub start_x: u32,
	pub start_y: u32,
	pub end_x: u32,
	pub end_y: u32,
}

impl Vectors {
	fn new(values: &Vec<u32>) -> Self {
		let start_x = values[0];
		let start_y = values[1];
		let end_x = values[2];
		let end_y = values[3];

		Self {
			start_x,
			start_y,
			end_x,
			end_y,
		}
	}

	fn is_horizontal_or_vertical_line(&self) -> bool {
		self.start_x == self.end_x || self.start_y == self.end_y
	}

	fn is_45(&self) -> bool {
		i32::abs(self.end_x as i32 - self.start_x as i32) == i32::abs(self.end_y as i32 - self.start_y as i32)
	}
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Vectors> {
	input
		.replace(" -> ", ",")
		.lines()
		.map(|l| Vectors::new(&l.split(",").map(|v| u32::from_str(v).unwrap()).collect()))
		.collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Vectors>) -> usize {
	let mut visited: HashMap<(u32, u32), u32> = HashMap::new();
	for v in input.iter().filter(|v| v.is_horizontal_or_vertical_line()) {
		for x in min(v.start_x, v.end_x)..=max(v.start_x, v.end_x) {
			for y in min(v.start_y, v.end_y)..=max(v.start_y, v.end_y) {
				let l = visited.entry((x, y)).or_insert(0);
				*l += 1;
			}
		}
	}

	visited.values().filter(|v| **v > 1).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Vectors>) -> usize {
	let mut visited: HashMap<(u32, u32), u32> = HashMap::new();
	for v in input.iter().filter(|v| v.is_horizontal_or_vertical_line()) {
		for x in min(v.start_x, v.end_x)..=max(v.start_x, v.end_x) {
			for y in min(v.start_y, v.end_y)..=max(v.start_y, v.end_y) {
				let l = visited.entry((x, y)).or_insert(0);
				*l += 1;
			}
		}
	}

	for v in input.iter().filter(|v| v.is_45()) {
		let mut x = v.start_x as i32;
		let mut y = v.start_y as i32;
		let x_i = if v.start_x >= v.end_x { -1 } else { 1 };
		let y_i = if v.start_y >= v.end_y { -1 } else { 1 };
		while v.end_x as i32 != (x - x_i) {
			let l = visited.entry((x as u32, y as u32)).or_insert(0);
			*l += 1;
			x += x_i;
			y += y_i;
		}
	}

	visited.values().filter(|v| **v > 1).count()
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		0,9 -> 5,9
		8,0 -> 0,8
		9,4 -> 3,4
		2,2 -> 2,1
		7,0 -> 7,4
		6,4 -> 2,0
		0,9 -> 2,9
		3,4 -> 1,4
		0,0 -> 8,8
		5,5 -> 8,2
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&input_generator(EXAMPLE)), 5);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&input_generator(EXAMPLE)), 12);
	}
}
