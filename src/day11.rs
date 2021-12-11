use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, Copy)]
pub struct Octo {
	pub step: i32,
	pub level: i32,
}

impl Octo {
	fn sentinel() -> Self {
		Self { step: 9999, level: 0 }
	}

	fn is_sentinel(&self) -> bool {
		self.step == 9999
	}
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Octo>> {
	input
		.trim()
		.lines()
		.map(|v| {
			v.chars()
				.map(|v| {
					Octo {
						step: 0,
						level: i32::from_str(v.to_string().as_str()).unwrap(),
					}
				})
				.collect::<Vec<Octo>>()
		})
		.collect::<Vec<Vec<Octo>>>()
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<Vec<Octo>>) -> i32 {
	let mut octos = input.clone();
	let row_length = octos[0].len();
	for row in octos.iter_mut() {
		row.insert(0, Octo::sentinel());
		row.push(Octo::sentinel());
	}
	octos.insert(0, vec![Octo::sentinel(); row_length + 2]);
	octos.push(vec![Octo::sentinel(); row_length + 2]);

	let mut flashes = 0;
	for step in 1..=100 {
		for row in 1..(octos.len() - 1) {
			for col in 1..(octos[row].len() - 1) {
				let mut stack: Vec<(usize, usize)> = vec![(row, col)];

				while !stack.is_empty() {
					let (rr, cc) = stack.pop().unwrap();
					let mut cell = &mut octos[rr][cc];
					if cell.is_sentinel() {
						continue;
					}

					cell.level += 1;

					if cell.level == 10 {
						for (r, c) in [(-1, 0), (-1, 1), (-1, -1), (0, -1), (0, 1), (1, 1), (1, 0), (1, -1)] {
							stack.push(((rr as i32 + r) as usize, ((cc as i32) + c) as usize));
						}
					}
				}
			}
		}

		for row in 1..(octos.len() - 1) {
			for col in 1..(octos[row].len() - 1) {
				let mut cell = &mut octos[row][col];
				cell.step = step;
				if cell.level > 9 {
					flashes += 1;
					cell.level = 0;
				}
			}
		}
	}

	flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &Vec<Vec<Octo>>) -> i32 {
	let mut octos = input.clone();
	let row_length = octos[0].len();
	for row in octos.iter_mut() {
		row.insert(0, Octo::sentinel());
		row.push(Octo::sentinel());
	}
	octos.insert(0, vec![Octo::sentinel(); row_length + 2]);
	octos.push(vec![Octo::sentinel(); row_length + 2]);

	let mut step = 1;
	loop {
		for row in 1..(octos.len() - 1) {
			for col in 1..(octos[row].len() - 1) {
				let mut stack: Vec<(usize, usize)> = vec![(row, col)];

				while !stack.is_empty() {
					let (rr, cc) = stack.pop().unwrap();
					let mut cell = &mut octos[rr][cc];
					if cell.is_sentinel() {
						continue;
					}

					cell.level += 1;

					if cell.level == 10 {
						for (r, c) in [(-1, 0), (-1, 1), (-1, -1), (0, -1), (0, 1), (1, 1), (1, 0), (1, -1)] {
							// eprintln!("Push: {:?}", ((rr as i32 + r) as usize, ((cc as i32) + c) as usize));
							stack.push(((rr as i32 + r) as usize, ((cc as i32) + c) as usize));
						}
					}
				}
			}
		}

		let mut all = true;
		for row in 1..(octos.len() - 1) {
			for col in 1..(octos[row].len() - 1) {
				let mut cell = &mut octos[row][col];
				cell.step = step;
				if cell.level > 9 {
					cell.level = 0;
				}
				else {
					all = false;
				}
			}
		}
		if all {
			return step;
		}
		step += 1;
	}
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		5483143223
		2745854711
		5264556173
		6141336146
		6357385478
		4167524645
		2176841721
		6882881134
		4846848554
		5283751526
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&input_generator(EXAMPLE)), 1656);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&input_generator(EXAMPLE)), 195);
	}
}
