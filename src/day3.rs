use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (usize, Vec<u16>) {
	(
		input.lines().next().unwrap().len(),
		input.lines().map(|v| u16::from_str_radix(v, 2).unwrap()).collect(),
	)
}

#[aoc(day3, part1)]
pub fn part1(input: &(usize, Vec<u16>)) -> u32 {
	let mut gamma_rate = 0;
	let mut epsilon_rate = 0;

	// assume 16 bits maximum
	let mut counts = [0; 16];

	for v in &input.1 {
		let mut v = *v;
		for i in 0..input.0 {
			if v & 0b1 == 1 {
				counts[i] += 1;
			}
			v = v >> 1;
		}
	}

	for i in (0..input.0).rev() {
		gamma_rate = gamma_rate << 1;
		epsilon_rate = epsilon_rate << 1;
		if counts[i] > (input.1.len() / 2) {
			gamma_rate = gamma_rate | 0b1;
		}
		else {
			epsilon_rate = epsilon_rate | 0b1;
		}
	}

	gamma_rate * epsilon_rate
}

fn count_leading_ones(inp: &Vec<u16>, pos: usize) -> u16 {
	let mask = 0b1 << pos;

	let mut count = 0;

	for v in inp {
		if v & mask == mask {
			count += 1;
		}
	}
	count
}

#[aoc(day3, part2)]
pub fn part2(input: &(usize, Vec<u16>)) -> u32 {
	let mut nums = input.1.clone();

	let mut mask = 0b1 << (input.0 - 1);

	let mut oxygen = 0;
	for i in (0..input.0).rev() {
		let leading_ones = count_leading_ones(&nums, i);
		let more_ones = leading_ones * 2 > nums.len() as u16;
		let equal = leading_ones * 2 == nums.len() as u16;

		nums = nums
			.iter()
			.filter(|v| {
				let v = **v;
				let is_one = v & mask == mask;
				if equal {
					return is_one;
				}
				(is_one && more_ones) || (!is_one && !more_ones)
			})
			.map(|v| *v)
			.collect();
		if nums.len() == 1 {
			oxygen = nums[0];
			break;
		}
		mask = mask >> 1;
	}

	nums = input.1.clone();
	let mut mask = 0b1 << (input.0 - 1);
	let mut co2 = 0;
	for i in (0..input.0).rev() {
		let leading_ones = count_leading_ones(&nums, i);
		let more_ones = leading_ones * 2 > nums.len() as u16;
		let equal = leading_ones * 2 == nums.len() as u16;
		nums = nums
			.iter()
			.filter(|v| {
				let v = **v;
				let is_one = v & mask == mask;
				if equal {
					return !is_one;
				}
				(is_one && !more_ones) || (!is_one && more_ones)
			})
			.map(|v| *v)
			.collect();
		if nums.len() == 1 {
			co2 = nums[0];
			break;
		}
		mask = mask >> 1;
	}

	oxygen as u32 * co2 as u32
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		00100
		11110
		10110
		10111
		10101
		01111
		00111
		11100
		10000
		11001
		00010
		01010
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&input_generator(EXAMPLE)), 198);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&input_generator(EXAMPLE)), 230);
	}
}
