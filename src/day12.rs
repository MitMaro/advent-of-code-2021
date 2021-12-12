use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Clone, Debug)]
pub struct Cave {
	pub name: String,
	pub small: bool,
	pub linked_caves: Vec<String>,
}

impl Cave {
	fn new(name: &str) -> Self {
		let small = !name.chars().next().unwrap().is_ascii_uppercase();
		Self {
			name: String::from(name),
			small,
			linked_caves: vec![],
		}
	}
}

pub fn input_generator(input: &str) -> HashMap<String, Cave> {
	let mut map = HashMap::new();
	for line in input.trim().lines() {
		let mut split = line.split("-");
		let a = split.next().unwrap();
		let aa = String::from(a);
		let b = split.next().unwrap();
		let bb = String::from(b);

		let aaa = map.entry(aa.clone()).or_insert_with(|| Cave::new(a));
		if !aaa.linked_caves.contains(&bb) {
			aaa.linked_caves.push(bb.clone());
		}
		let bbb = map.entry(bb.clone()).or_insert_with(|| Cave::new(b));
		if !bbb.linked_caves.contains(&aa) {
			bbb.linked_caves.push(aa.clone());
		}
	}
	map
}

pub fn traverse<'a>(map: &'a HashMap<String, Cave>, visited: &mut Vec<&'a str>, name: &'a str) -> i32 {
	if name == "end" {
		return 1;
	}

	let cave = map.get(name).unwrap();

	if cave.small && visited.contains(&name) {
		return 0;
	}

	visited.push(name);

	let mut total = 0;
	for c in &cave.linked_caves {
		total += traverse(map, visited, c);
	}

	visited.pop();

	total
}

pub fn traverse_2<'a>(
	map: &'a HashMap<String, Cave>,
	visited: &mut Vec<&'a str>,
	name: &'a str,
	has_double: bool,
) -> i32 {
	if name == "end" {
		return 1;
	}

	let mut has_double = has_double;

	let cave = map.get(name).unwrap();

	if name == "start" && visited.contains(&"start") {
		return 0;
	}

	if cave.small && visited.contains(&name) {
		if has_double {
			return 0;
		}
		has_double = true;
	}

	visited.push(name);

	let mut total = 0;
	for c in &cave.linked_caves {
		total += traverse_2(map, visited, c, has_double);
	}

	visited.pop();

	total
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
	let caves = input_generator(input);

	let mut visited = vec![];
	traverse(&caves, &mut visited, "start")
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
	let caves = input_generator(input);

	let mut visited = vec![];
	traverse_2(&caves, &mut visited, "start", false)
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE_1: &'static str = indoc! {"
		start-A
		start-b
		A-c
		A-b
		b-d
		A-end
		b-end
	"};

	static EXAMPLE_2: &'static str = indoc! {"dc-end
		HN-start
		start-kj
		dc-start
		dc-HN
		LN-dc
		HN-end
		kj-sa
		kj-HN
		kj-dc
	"};

	static EXAMPLE_3: &'static str = indoc! {"
		fs-end
		he-DX
		fs-he
		start-DX
		pj-DX
		end-zg
		zg-sl
		zg-pj
		pj-he
		RW-he
		fs-DX
		pj-RW
		zg-RW
		start-pj
		he-WI
		zg-he
		pj-fs
		start-RW
	"};

	#[test]
	fn example_1_part1() {
		assert_eq!(part1(&EXAMPLE_1), 10);
	}

	#[test]
	fn example_2_part1() {
		assert_eq!(part1(&EXAMPLE_2), 19);
	}

	#[test]
	fn example_3_part1() {
		assert_eq!(part1(&EXAMPLE_3), 226);
	}

	#[test]
	fn example_1_part2() {
		assert_eq!(part2(&EXAMPLE_1), 36);
	}

	#[test]
	fn example_2_part2() {
		assert_eq!(part2(&EXAMPLE_2), 103);
	}

	#[test]
	fn example_3_part2() {
		assert_eq!(part2(&EXAMPLE_3), 3509);
	}
}
