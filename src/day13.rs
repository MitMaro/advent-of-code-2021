use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone)]
pub enum Fold {
	X(i32),
	Y(i32),
}

pub fn input_generator(input: &str) -> (HashMap<(i32, i32), i32>, Vec<Fold>) {
	let mut map = HashMap::new();

	for l in input.trim().lines() {
		if l == "" {
			continue;
		}
		if l.starts_with("fold") {
			break;
		}
		let mut iter = l.split(",").map(|v| i32::from_str(v).unwrap());

		map.insert((iter.next().unwrap(), iter.next().unwrap()), 1);
	}
	(
		map,
		input
			.trim()
			.lines()
			.filter(|v| v.starts_with("fold"))
			.map(|v| {
				let v = v.replace("fold along ", "");
				let mut iter = v.split("=");

				let direction = iter.next().unwrap();
				let line = i32::from_str(iter.next().unwrap()).unwrap();

				if direction == "y" {
					Fold::Y(line)
				}
				else {
					Fold::X(line)
				}
			})
			.collect(),
	)
}

fn fold(paper: HashMap<(i32, i32), i32>, fold: Fold) -> HashMap<(i32, i32), i32> {
	match fold {
		Fold::X(line) => fold_x(paper, line),
		Fold::Y(line) => fold_y(paper, line),
	}
}

fn fold_x(paper: HashMap<(i32, i32), i32>, line: i32) -> HashMap<(i32, i32), i32> {
	let mut new = HashMap::new();
	for entry in paper {
		let (x, y) = entry.0;
		let key = if x > line { (line - (x - line), y) } else { (x, y) };
		*new.entry(key).or_insert(0) += entry.1;
	}
	new
}

fn fold_y(paper: HashMap<(i32, i32), i32>, line: i32) -> HashMap<(i32, i32), i32> {
	let mut new = HashMap::new();
	for entry in paper {
		let (x, y) = entry.0;
		let key = if y > line { (x, line - (y - line)) } else { (x, y) };
		*new.entry(key).or_insert(0) += entry.1;
	}
	new
}

fn print(paper: &HashMap<(i32, i32), i32>) -> String {
	let mut lines = vec![String::new()];
	let mut max_x = 0;
	let mut max_y = 0;
	for ((x, y), _) in paper {
		if *x > max_x {
			max_x = *x;
		}
		if *y > max_y {
			max_y = *y;
		}
	}
	for y in 0..=max_y {
		let mut line = vec![];
		for x in 0..=max_x {
			if paper.contains_key(&(x, y)) {
				line.push("#");
			}
			else {
				line.push(" ");
			}
		}
		lines.push(line.join(""));
	}
	lines.join("\n")
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
	let (mut paper, folds) = input_generator(input);

	for f in folds {
		paper = fold(paper, f);
		break;
	}
	paper.iter().map(|v| v.1).count() as i32
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> String {
	let (mut paper, folds) = input_generator(input);

	for f in folds {
		paper = fold(paper, f);
	}

	print(&paper)
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		6,10
		0,14
		9,10
		0,3
		10,4
		4,11
		6,0
		6,12
		4,1
		0,13
		10,12
		3,4
		3,0
		8,4
		1,10
		2,14
		8,10
		9,0

		fold along y=7
		fold along x=5
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&EXAMPLE), 17);
	}

	static PART2_OUT: &'static str = indoc! {"
		#####
		#   #
		#   #
		#   #
		#####
	"};

	#[test]
	fn example_part2() {
		assert_eq!(part2(&EXAMPLE).trim(), PART2_OUT.trim());
	}
}
