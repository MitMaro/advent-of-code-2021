use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
	input.trim().lines().map(String::from).collect::<Vec<String>>()
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<String>) -> u64 {
	let mut score = 0;

	for line in input {
		let mut stack = vec![];
		for c in line.chars() {
			if c == '(' || c == '[' || c == '{' || c == '<' {
				stack.push(c);
			}
			else {
				let o = stack.pop().unwrap();
				if c == ')' && o != '(' {
					score += 3;
					break;
				}
				else if c == ']' && o != '[' {
					score += 57;
					break;
				}
				else if c == '}' && o != '{' {
					score += 1197;
					break;
				}
				else if c == '>' && o != '<' {
					score += 25137;
					break;
				}
			}
		}
	}

	score
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<String>) -> u64 {
	let mut scores = vec![];

	'a: for line in input {
		let mut stack = vec![];
		for c in line.chars() {
			if c == '(' || c == '[' || c == '{' || c == '<' {
				stack.push(c);
			}
			else {
				let o = stack.pop().unwrap();
				if (c == ')' && o != '(') || (c == ']' && o != '[') || (c == '}' && o != '{') || (c == '>' && o != '<')
				{
					continue 'a;
				}
			}
		}

		let mut score = 0;
		for s in stack.iter().rev() {
			let p = match s {
				'(' => 1,
				'[' => 2,
				'{' => 3,
				'<' => 4,
				_ => unreachable!(),
			};
			score = score * 5 + p;
		}
		scores.push(score);
	}

	*scores.iter().sorted().skip(scores.len() / 2).next().unwrap()
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		[({(<(())[]>[[{[]{<()<>>
		[(()[<>])]({[<{<<[]>>(
		{([(<{}[<>[]}>{[]{[(<()>
		(((({<>}<{<{<>}{[]{[]{}
		[[<[([]))<([[{}[[()]]]
		[{[{({}]{}}([{[{{{}}([]
		{<[[]]>}<{[{[{[]{()[[[]
		[<(<(<(<{}))><([]([]()
		<{([([[(<>()){}]>(<<{{
		<{([{{}}[<[[[<>{}]]]>[]]
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&input_generator(EXAMPLE)), 26397);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&input_generator(EXAMPLE)), 288957);
	}
}
