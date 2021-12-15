use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Debug)]
pub struct Instructions {
	template: String,
	insertions: HashMap<(char, char), char>,
}

pub fn parse_input(input: &str) -> Instructions {
	let mut iter = input.trim().lines().filter(|v| !v.is_empty()).map(String::from);

	let template = iter.next().unwrap();
	let insertions = iter
		.map(|v| {
			let mut parts = v.split(" -> ");
			let mut key = parts.next().unwrap().chars();
			(
				(key.next().unwrap(), key.next().unwrap()),
				parts.next().unwrap().chars().next().unwrap(),
			)
		})
		.collect::<Vec<((char, char), char)>>();

	Instructions {
		template,
		insertions: HashMap::from_iter(insertions),
	}
}

fn char_to_index(c: char) -> usize {
	c as usize - 'A' as usize
}

fn index_to_char(i: usize) -> char {
	(i + 'A' as usize) as u8 as char
}

fn solve(inst: Instructions, iterations: usize) -> i64 {
	let mut next = [[0; 26]; 26];

	let mut chars = inst.template.chars();
	let mut prev = char_to_index(chars.next().unwrap());
	for char in chars {
		let index = char_to_index(char);
		next[prev][index] += 1;
		prev = index;
	}

	for _ in 0..iterations {
		let curr = next.clone();
		next = [[0; 26]; 26];
		for (first, seconds) in curr.iter().enumerate() {
			for (second, value) in seconds.iter().enumerate() {
				if *value > 0 {
					let f = index_to_char(first);
					let s = index_to_char(second);
					if let Some(r) = inst.insertions.get(&(f, s)) {
						let replace = char_to_index(*r);
						next[first][replace] += value;
						next[replace][second] += value;
					}
					else {
						next[first][second] += value;
					}
				}
			}
		}
	}

	let mut counts = [0; 26];
	for (first, seconds) in next.iter().enumerate() {
		for (second, value) in seconds.iter().enumerate() {
			counts[first] += value;
			counts[second] += value;
		}
	}

	// start and end characters are only added once, so add them a second time so the division below
	// will work.
	let first = inst.template.chars().next().unwrap();
	let last = inst.template.chars().skip(inst.template.len() - 1).next().unwrap();

	counts[char_to_index(first)] += 1;
	counts[char_to_index(last)] += 1;

	let max = counts.iter().max().unwrap();
	let min = counts.iter().filter(|v| **v != 0).min().unwrap();
	(max / 2) - (min / 2)
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> i64 {
	solve(parse_input(input), 10)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i64 {
	solve(parse_input(input), 40)
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		NNCB

		CH -> B
		HH -> N
		CB -> H
		NH -> C
		HB -> C
		HC -> B
		HN -> C
		NN -> C
		BH -> H
		NC -> B
		NB -> B
		BN -> B
		BB -> N
		BC -> B
		CC -> N
		CN -> C
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(EXAMPLE), 1588);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(EXAMPLE), 2188189693529);
	}
}
