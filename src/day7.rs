use std::{cmp, str::FromStr};

use aoc_runner_derive::aoc;

pub fn input_generator(input: &str) -> Vec<i32> {
	input
		.trim()
		.split(",")
		.map(|v| i32::from_str(v).unwrap())
		.collect::<Vec<i32>>()
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> i32 {
	let pos = input_generator(input);

	let mut buckets = vec![0; 2000];

	for p in pos {
		buckets[p as usize] += 1;
	}

	let mut minimum = i32::MAX;
	for i in 0..=2000 {
		let total = buckets
			.iter()
			.enumerate()
			.fold(0, |acc, (p, v)| acc + ((p as i32 - i).abs() * v));
		minimum = cmp::min(minimum, total);
	}

	minimum
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i32 {
	let pos = input_generator(input);

	let mut buckets = vec![0; 2000];

	for p in pos {
		buckets[p as usize] += 1;
	}

	let mut minimum = i32::MAX;
	'a: for i in 0..=2000 {
		let mut total = 0;
		for (p, t) in buckets.iter().enumerate() {
			if *t == 0 {
				continue;
			}

			let n = (p as i32 - i).abs();

			// for j in 1..=(p as i32 - i).abs() {
			// 	total += j * t;
			// }
			total += ((n * (n + 1)) / 2) * t;

			if total > minimum {
				continue 'a;
			}
		}
		minimum = cmp::min(minimum, total);
	}

	minimum
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		16,1,2,0,4,2,7,1,2,14
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&EXAMPLE), 37);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&EXAMPLE), 168);
	}
}
