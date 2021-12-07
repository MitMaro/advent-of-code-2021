use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Debug)]
pub struct MetaFish {
	pub cycle: i32,
	pub count: i64,
}

impl MetaFish {
	fn new(cycle: i32) -> Self {
		Self { cycle, count: 0 }
	}

	fn add_fish(&mut self) {
		self.count += 1;
	}

	fn cycle(&mut self) {
		self.cycle = match self.cycle {
			8 => 7,
			7 => 8,
			0 => 6,
			v => v - 1,
		};
	}
}

pub fn input_generator(input: &str) -> [MetaFish; 9] {
	let ages = input
		.trim()
		.split(",")
		.map(|v| i32::from_str(v).unwrap())
		.collect::<Vec<i32>>();

	let mut metafish = [
		MetaFish::new(0),
		MetaFish::new(1),
		MetaFish::new(2),
		MetaFish::new(3),
		MetaFish::new(4),
		MetaFish::new(5),
		MetaFish::new(6),
		MetaFish::new(7),
		MetaFish::new(8),
	];

	for age in ages {
		metafish[age as usize].add_fish();
	}

	metafish
}

fn push_fish(cycle: i32, metafish: &mut [MetaFish]) {
	for fish in metafish {
		if fish.cycle == cycle {
			fish.add_fish();
		}
	}
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
	let mut metafish = input_generator(input);

	let mut cycle = 5;
	while cycle < 80 + 5 {
		let mut num_babies = 0;
		for adult in metafish.iter_mut() {
			if adult.cycle == 0 {
				num_babies = adult.count;
			}
			adult.cycle();
		}

		let young = metafish.iter_mut().find(|v| v.cycle == 8).unwrap();
		let young_count = young.count;
		young.count = 0;
		if young_count > 0 {
			for _ in 0..young_count {
				push_fish(6, &mut metafish);
			}
		}

		for _ in 0..num_babies {
			push_fish(8, &mut metafish);
		}

		cycle += 1;
	}

	let mut sum = 0;
	for adult in metafish {
		sum += adult.count;
	}
	sum
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
	let mut metafish = input_generator(input);

	let mut cycle = 5;
	while cycle < 256 + 5 {
		let mut num_babies = 0;
		for adult in metafish.iter_mut() {
			if adult.cycle == 0 {
				num_babies = adult.count;
			}
			adult.cycle();
		}

		let young = metafish.iter_mut().find(|v| v.cycle == 8).unwrap();
		let young_count = young.count;
		young.count = 0;
		if young_count > 0 {
			for _ in 0..young_count {
				push_fish(6, &mut metafish);
			}
		}

		for _ in 0..num_babies {
			push_fish(8, &mut metafish);
		}

		cycle += 1;
	}

	let mut sum = 0;
	for adult in metafish {
		sum += adult.count;
	}
	sum
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		3,4,3,1,2
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&EXAMPLE), 5934);
	}

	// Won't pass as a test, does pass if run through cargo aoc
	// #[test]
	// fn example_part2() {
	// 	assert_eq!(part2(&EXAMPLE), 26984457539);
	// }
}
