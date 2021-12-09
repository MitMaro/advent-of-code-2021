use std::str::FromStr;

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug)]
pub struct Entry {
	pub patterns: [String; 10],
	pub output: [String; 4],
}

#[derive(Copy, Clone, Debug)]
pub struct PossibleSegments {
	pub a: bool,
	pub b: bool,
	pub c: bool,
	pub d: bool,
	pub e: bool,
	pub f: bool,
	pub g: bool,
}

impl PossibleSegments {
	fn new() -> Self {
		Self {
			a: true,
			b: true,
			c: true,
			d: true,
			e: true,
			f: true,
			g: true,
		}
	}

	fn set_all_false(&mut self) {
		if self.is_resolved() {
			unreachable!();
		}
		self.set_false("abcdefg");
	}

	fn set_false(&mut self, i: &str) {
		if self.is_resolved() {
			return;
		}
		for v in i.chars() {
			match v {
				'a' => self.a = false,
				'b' => self.b = false,
				'c' => self.c = false,
				'd' => self.d = false,
				'e' => self.e = false,
				'f' => self.f = false,
				'g' => self.g = false,
				_ => unreachable!(),
			}
		}
	}

	fn set_true(&mut self, i: &str) {
		for v in i.chars() {
			match v {
				'a' => self.a = true,
				'b' => self.b = true,
				'c' => self.c = true,
				'd' => self.d = true,
				'e' => self.e = true,
				'f' => self.f = true,
				'g' => self.g = true,
				_ => unreachable!(),
			}
		}
	}

	fn is_resolved(&self) -> bool {
		[self.a, self.b, self.c, self.d, self.e, self.f, self.g]
			.iter()
			.fold(0, |a, v| {
				if *v {
					a + 1
				}
				else {
					a
				}
			}) == 1
	}

	fn get_value(&self) -> char {
		if !self.is_resolved() {
			unreachable!();
		}
		if self.a {
			'a'
		}
		else if self.b {
			'b'
		}
		else if self.c {
			'c'
		}
		else if self.d {
			'd'
		}
		else if self.e {
			'e'
		}
		else if self.f {
			'f'
		}
		else if self.g {
			'g'
		}
		else {
			unreachable!()
		}
	}

	fn get_options(&self) -> Vec<char> {
		let mut c = vec![];
		if self.a {
			c.push('a');
		}
		if self.b {
			c.push('b');
		}
		if self.c {
			c.push('c');
		}
		if self.d {
			c.push('d');
		}
		if self.e {
			c.push('e');
		}
		if self.f {
			c.push('f');
		}
		if self.g {
			c.push('g');
		}

		c
	}
}

#[derive(Debug)]
pub struct Display {
	pub segments: [PossibleSegments; 7],
}

impl Display {
	fn new() -> Self {
		Self {
			segments: [PossibleSegments::new(); 7],
		}
	}

	fn set_a(&mut self, segs_1: &str, segs_7: &str) {
		for s in segs_7.chars() {
			if !segs_1.contains(s) {
				let v = String::from(s);
				self.segments[0].set_all_false();
				self.segments[0].set_true(v.as_str());
				self.segments[1].set_false(v.as_str());
				self.segments[2].set_false(v.as_str());
				self.segments[3].set_false(v.as_str());
				self.segments[4].set_false(v.as_str());
				self.segments[5].set_false(v.as_str());
				self.segments[6].set_false(v.as_str());
				break;
			}
		}
	}

	// set_b solved from a and d

	fn set_c(&mut self, segs_1: &str, segs_5: &str) {
		for s in segs_1.chars() {
			if !segs_5.contains(s) {
				let v = String::from(s);
				self.segments[2].set_all_false();
				self.segments[2].set_true(v.as_str());

				self.segments[0].set_false(v.as_str());
				self.segments[1].set_false(v.as_str());
				self.segments[3].set_false(v.as_str());
				self.segments[4].set_false(v.as_str());
				self.segments[5].set_false(v.as_str());
				self.segments[6].set_false(v.as_str());
				break;
			}
		}
	}

	fn set_d(&mut self, segs_0: &str, segs_8: &str) {
		self.segments[3].set_all_false();
		for s in segs_8.chars() {
			if !segs_0.contains(s) {
				let v = String::from(s);
				self.segments[3].set_true(v.as_str());

				self.segments[0].set_false(v.as_str());
				self.segments[1].set_false(v.as_str());
				self.segments[2].set_false(v.as_str());
				self.segments[4].set_false(v.as_str());
				self.segments[5].set_false(v.as_str());
				self.segments[6].set_false(v.as_str());
				break;
			}
		}
	}

	fn set_e(&mut self, segs_5: &str, segs_6: &str) {
		for s in segs_6.chars() {
			if !segs_5.contains(s) {
				let v = String::from(s);
				self.segments[4].set_all_false();
				self.segments[4].set_true(v.as_str());

				self.segments[0].set_false(v.as_str());
				self.segments[1].set_false(v.as_str());
				self.segments[2].set_false(v.as_str());
				self.segments[3].set_false(v.as_str());
				self.segments[5].set_false(v.as_str());
				self.segments[6].set_false(v.as_str());
				break;
			}
		}
	}

	fn partial_resolve_b_and_d(&mut self, segs_1: &str, segs_4: &str) {
		self.segments[1].set_all_false();
		self.segments[3].set_all_false();
		for s in segs_4.chars() {
			if !segs_1.contains(s) {
				let v = String::from(s);
				self.segments[1].set_true(v.as_str());
				self.segments[3].set_true(v.as_str());

				self.segments[2].set_false(v.as_str());
				self.segments[4].set_false(v.as_str());
				self.segments[5].set_false(v.as_str());
				self.segments[6].set_false(v.as_str());
			}
		}
	}

	fn partial_resolve_c_and_f(&mut self, segs_1: &str) {
		self.segments[2].set_all_false();
		self.segments[5].set_all_false();
		for s in segs_1.chars() {
			let v = String::from(s);
			self.segments[2].set_true(v.as_str());
			self.segments[5].set_true(v.as_str());

			self.segments[0].set_false(v.as_str());
			self.segments[1].set_false(v.as_str());
			self.segments[3].set_false(v.as_str());
			self.segments[4].set_false(v.as_str());
			self.segments[6].set_false(v.as_str());
		}
	}

	fn find_zero(&self, items: &mut Vec<String>) -> String {
		let mut found = -1;
		let opts = self.segments[1].get_options();
		for (index, item) in items.iter().enumerate() {
			if item.len() != 6 {
				continue;
			}
			if (item.contains(opts[0]) && !item.contains(opts[1]))
				|| (!item.contains(opts[0]) && item.contains(opts[1]))
			{
				found = index as i32;
				break;
			}
		}
		if found == -1 {
			unreachable!();
		}
		items.remove(found as usize)
	}

	fn find_two(&self, items: &mut Vec<String>) -> String {
		let mut found = -1;

		let b = self.segments[1].get_value();
		let f = self.segments[5].get_value();
		for (index, item) in items.iter().enumerate() {
			if item.len() != 5 {
				continue;
			}

			if !item.contains(b) && !item.contains(f) {
				found = index as i32;
			}
		}
		if found == -1 {
			unreachable!();
		}
		items.remove(found as usize)
	}

	fn find_three(&self, items: &mut Vec<String>) -> String {
		let mut found = -1;

		let b = self.segments[1].get_value();
		let e = self.segments[4].get_value();
		for (index, item) in items.iter().enumerate() {
			if item.len() != 5 {
				continue;
			}

			if !item.contains(b) && !item.contains(e) {
				found = index as i32;
			}
		}
		if found == -1 {
			unreachable!();
		}
		items.remove(found as usize)
	}

	fn find_five(&self, items: &mut Vec<String>) -> String {
		let mut found = -1;
		let b = self.segments[1].get_value();
		for (index, item) in items.iter().enumerate() {
			if item.len() != 5 {
				continue;
			}

			if item.contains(b) {
				found = index as i32;
				break;
			}
		}
		if found == -1 {
			unreachable!();
		}
		items.remove(found as usize)
	}

	fn find_six(&self, items: &mut Vec<String>) -> String {
		let mut found = -1;
		let c = self.segments[2].get_value();
		for (index, item) in items.iter().enumerate() {
			if item.len() != 6 {
				continue;
			}

			if !item.contains(c) {
				found = index as i32;
			}
		}
		if found == -1 {
			unreachable!();
		}
		items.remove(found as usize)
	}
}

pub fn input_generator(input: &str) -> Vec<Entry> {
	input
		.trim()
		.lines()
		.map(|v| {
			let mut entry_values = v.split(" | ");

			let patterns = entry_values
				.next()
				.unwrap()
				.split(" ")
				.map(|v| v.chars().sorted().collect::<String>())
				.collect::<Vec<String>>();
			let output = entry_values
				.next()
				.unwrap()
				.split(" ")
				.map(|v| v.chars().sorted().collect::<String>())
				.collect::<Vec<String>>();

			Entry {
				patterns: patterns.try_into().unwrap(),
				output: output.try_into().unwrap(),
			}
		})
		.collect::<Vec<Entry>>()
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
	let notes = input_generator(input);

	let mut numbers = [0; 10];

	for entry in notes {
		for out in entry.output {
			match out.len() {
				2 => numbers[1] += 1,
				4 => numbers[4] += 1,
				3 => numbers[7] += 1,
				7 => numbers[8] += 1,
				_ => {},
			}
		}
	}

	numbers.iter().sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
	let notes = input_generator(input);

	let mut total = 0;
	for entry in &notes {
		let mut display = Display::new();

		let mut s1 = String::from("");
		let mut s4 = String::from("");
		let mut s7 = String::from("");
		let mut s8 = String::from("");
		let mut others = vec![];

		for pattern in &entry.patterns {
			let v = pattern.clone();
			match pattern.len() {
				2 => s1 = v,
				4 => s4 = v,
				3 => s7 = v,
				7 => s8 = v,
				_ => others.push(v),
			}
		}

		display.partial_resolve_c_and_f(s1.as_str());
		display.set_a(s1.as_str(), s7.as_str());
		display.partial_resolve_b_and_d(s1.as_str(), s4.as_str());
		let s0 = display.find_zero(&mut others);
		display.set_d(s0.as_str(), s8.as_str());
		let s5 = display.find_five(&mut others);
		display.set_c(s1.as_str(), s5.as_str());
		let s6 = display.find_six(&mut others);
		display.set_e(s5.as_str(), s6.as_str());
		let s2 = display.find_two(&mut others);
		let s3 = display.find_three(&mut others);
		let s9 = others.remove(0);

		let mut n: [String; 4] = Default::default();
		for (i, out) in entry.output.iter().enumerate() {
			match out.clone() {
				s if *s == s0 => n[i] = String::from("0"),
				s if *s == s1 => n[i] = String::from("1"),
				s if *s == s2 => n[i] = String::from("2"),
				s if *s == s3 => n[i] = String::from("3"),
				s if *s == s4 => n[i] = String::from("4"),
				s if *s == s5 => n[i] = String::from("5"),
				s if *s == s6 => n[i] = String::from("6"),
				s if *s == s7 => n[i] = String::from("7"),
				s if *s == s8 => n[i] = String::from("8"),
				s if *s == s9 => n[i] = String::from("9"),
				_ => {
					unreachable!()
				},
			}
		}
		total += i32::from_str(n.join("").as_str()).unwrap();
	}

	total
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
		edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
		fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
		fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
		aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
		fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
		dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
		bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
		egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
		gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&EXAMPLE), 26);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&EXAMPLE), 61229);
	}
}
