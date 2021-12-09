use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
	input
		.trim()
		.lines()
		.map(|v| {
			v.chars()
				.map(|v| i32::from_str(v.to_string().as_str()).unwrap())
				.collect::<Vec<i32>>()
		})
		.collect::<Vec<Vec<i32>>>()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
	let mut low_points = vec![];
	let mut grid = input.clone();
	let row_length = grid[0].len();
	for row in grid.iter_mut() {
		row.insert(0, 9);
		row.push(9);
	}
	grid.insert(0, vec![9; row_length + 2]);
	grid.push(vec![9; row_length + 2]);

	for row in 1..(grid.len() - 1) {
		for col in 1..(grid[row].len() - 1) {
			let above = grid[row - 1][col];
			let below = grid[row + 1][col];
			let left = grid[row][col - 1];
			let right = grid[row][col + 1];
			let cell = grid[row][col];

			if cell < above && cell < below && cell < left && cell < right {
				low_points.push(cell + 1);
			}
		}
	}

	low_points.iter().sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
	let mut low_points = vec![];
	let mut grid = input.clone();
	let row_length = grid[0].len();
	for row in grid.iter_mut() {
		row.insert(0, 9);
		row.push(9);
	}
	grid.insert(0, vec![9; row_length + 2]);
	grid.push(vec![9; row_length + 2]);

	for row in 1..(grid.len() - 1) {
		for col in 1..(grid[row].len() - 1) {
			let above = grid[row - 1][col];
			let below = grid[row + 1][col];
			let left = grid[row][col - 1];
			let right = grid[row][col + 1];
			let cell = grid[row][col];

			if cell < above && cell < below && cell < left && cell < right {
				low_points.push((row, col));
			}
		}
	}

	let mut basin_sizes = vec![];

	for (r, c) in low_points {
		let mut stack = vec![(r, c)];
		let mut visited = vec![];

		while !stack.is_empty() {
			let (row, col) = stack.pop().unwrap();
			if visited.contains(&(row, col)) {
				continue;
			}

			visited.push((row, col));
			let above = grid[row - 1][col];
			let below = grid[row + 1][col];
			let left = grid[row][col - 1];
			let right = grid[row][col + 1];
			let cell = grid[row][col];

			if above != 9 && above >= cell + 1 {
				stack.push((row - 1, col));
			}
			if below != 9 && below >= cell + 1 {
				stack.push((row + 1, col));
			}
			if left != 9 && left >= cell + 1 {
				stack.push((row, col - 1));
			}
			if right != 9 && right >= cell + 1 {
				stack.push((row, col + 1));
			}
		}

		basin_sizes.push(visited.len() as i32);
	}

	let sorted = basin_sizes.iter().sorted().rev().map(|v| *v).collect::<Vec<i32>>();

	sorted[0] * sorted[1] * sorted[2]
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		2199943210
		3987894921
		9856789892
		8767896789
		9899965678
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&input_generator(EXAMPLE)), 15);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&input_generator(EXAMPLE)), 1134);
	}
}
