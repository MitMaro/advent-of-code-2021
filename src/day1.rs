use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
	input.lines().map(|v| i32::from_str(v).unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
	let mut iter = input.iter();
	let mut previous = *iter.next().unwrap();
	let mut increased = 0;
	for next in iter {
		if *next > previous {
			increased += 1;
		}
		previous = *next;
	}
	increased
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
	let mut iter = input.iter();
	let mut previous_a = *iter.next().unwrap();
	let mut previous_b = *iter.next().unwrap();
	let mut previous_c = *iter.next().unwrap();
	let mut increased = 0;
	for d in iter {
		let next = previous_b + previous_c + *d;
		if next > previous_b + previous_c + previous_a {
			increased += 1;
		}
		previous_a = previous_b;
		previous_b = previous_c;
		previous_c = *d;
	}
	increased
}
