use std::str::FromStr;

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug)]
pub struct Cell {
	pub value: i32,
	pub selected: bool,
}

#[derive(Debug)]
pub struct Card {
	pub grid: [[Cell; 5]; 5],
	pub won: bool,
}

impl Card {
	fn mark_selected(&mut self, num: i32) -> bool {
		if self.won {
			return true;
		}
		for row in self.grid.as_mut() {
			for cell in row.as_mut() {
				if cell.value == num {
					cell.selected = true;
					return true;
				}
			}
		}
		false
	}

	fn is_win(&mut self) -> bool {
		if !self.won {
			self.won = self.has_complete_row() || self.has_complete_column();
		}
		self.won
	}

	fn get_score(&self) -> i32 {
		let mut score = 0;
		for row in &self.grid {
			for cell in row {
				if !cell.selected {
					score += cell.value
				}
			}
		}
		score
	}

	fn has_complete_row(&self) -> bool {
		self.grid.iter().any(|r| r.iter().all(|v| v.selected))
	}

	fn has_complete_column(&self) -> bool {
		'a: for i in 0..5 {
			for j in 0..5 {
				if !self.grid[j][i].selected {
					continue 'a;
				}
			}
			return true;
		}
		return false;
	}
}

pub fn input_generator(input: &str) -> (Vec<i32>, Vec<Card>) {
	let mut lines = input.lines().filter(|v| !v.is_empty());

	let nums = lines
		.next()
		.unwrap()
		.split(",")
		.map(|v| i32::from_str(v).unwrap())
		.collect();

	let grids = lines
		.map(|v| {
			v.split_whitespace()
				.map(|v| {
					Cell {
						value: i32::from_str(v).unwrap(),
						selected: false,
					}
				})
				.collect::<Vec<Cell>>()
				.try_into()
				.unwrap()
		})
		.chunks(5)
		.into_iter()
		.map(|chunks| {
			Card {
				grid: chunks.collect::<Vec<[Cell; 5]>>().try_into().unwrap(),
				won: false,
			}
		})
		.collect::<Vec<Card>>();
	(nums, grids)
}

#[aoc(day4, part1)]
pub fn part1(file: &str) -> i32 {
	let (values, mut grids) = input_generator(file);
	for v in values {
		for grid in grids.iter_mut() {
			if grid.mark_selected(v) && grid.is_win() {
				return grid.get_score() * v;
			}
		}
	}
	unreachable!();
}

#[aoc(day4, part2)]
pub fn part2(file: &str) -> i32 {
	let (values, mut grids) = input_generator(file);
	let mut last_index = None;
	for v in values {
		let mut won = 0;
		for grid in grids.iter_mut() {
			grid.mark_selected(v);

			if grid.is_win() {
				won += 1;
			}
		}
		if last_index.is_none() && won == grids.len() - 1 {
			for (i, grid) in grids.iter_mut().enumerate() {
				if !grid.is_win() {
					// eprintln!("{} {}", i, v);
					last_index = Some(i);
				}
			}
		}
		if won == grids.len() {
			if let Some(index) = last_index {
				let a: &Card = grids.get(index).unwrap();
				return a.get_score() * v;
			}
		}
	}
	unreachable!();
}
