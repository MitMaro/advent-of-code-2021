use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Operation {
	Sum,
	Product,
	Minimum,
	Maximum,
	GreaterThan,
	LessThan,
	EqualTo,
}

#[derive(Debug)]
enum PacketType {
	Literal(u64),
	Operator(Operation),
}

#[derive(Debug)]
struct Packet {
	version: u8,
	type_id: u8,
	packet_type: PacketType,
	sub_packets: Vec<Packet>,
}

fn decode_hex(c: char) -> u8 {
	if c >= 'A' {
		c as u8 - 'A' as u8 + 0b1010
	}
	else {
		c as u8 - '0' as u8
	}
}

fn convert_to_binary(input: &str) -> Vec<u8> {
	let mut iter = input.trim().chars();
	let mut values = vec![];
	while let Some(v1) = iter.next() {
		let v2 = iter.next().map(decode_hex).unwrap_or(0);
		values.push((decode_hex(v1) << 4) + v2);
	}
	values
}

fn read_data(data: &Vec<u8>, pointer: &mut usize, amount: usize) -> u64 {
	let mut value = 0;
	let p = *pointer;
	for i in 0..amount {
		let data_index = (p + i) / 8;
		let d = data[data_index];
		let offset = (p + i) - (data_index * 8);

		let mask = 0b1000_0000 >> offset;
		if (d & mask) == mask {
			value = (value << 1) + 0b1;
		}
		else {
			value <<= 1;
		}
	}
	*pointer += amount;
	value
}

fn parse_multiple_number(data: &Vec<u8>, pointer: &mut usize, num_packets: usize) -> Vec<Packet> {
	let mut packets = vec![];

	for _ in 0..num_packets {
		packets.push(parse(data, pointer));
	}

	packets
}

fn parse_multiple_length(data: &Vec<u8>, pointer: &mut usize, remaining_sub_packet_length: usize) -> Vec<Packet> {
	let mut packets = vec![];

	let mut remaining = remaining_sub_packet_length;

	while remaining > 0 {
		let pointer_start = *pointer;
		packets.push(parse(data, pointer));
		remaining -= *pointer - pointer_start;
	}

	packets
}

fn parse(data: &Vec<u8>, mut pointer: &mut usize) -> Packet {
	let version = read_data(&data, &mut pointer, 3) as u8;
	let type_id = read_data(&data, &mut pointer, 3) as u8;

	if type_id == 4 {
		let mut value = 0;
		loop {
			let v = read_data(&data, &mut pointer, 5);
			value = (value << 4) + (v & 0b1111);
			if v & 0b1_0000 == 0b1_0000 {
				continue;
			}
			return Packet {
				version,
				type_id,
				packet_type: PacketType::Literal(value),
				sub_packets: vec![],
			};
		}
	}
	else {
		let length_type = read_data(&data, &mut pointer, 1) as u8;
		let packet_length = read_data(&data, &mut pointer, if length_type == 0 { 15 } else { 11 }) as usize;
		let operation = match type_id {
			0 => Operation::Sum,
			1 => Operation::Product,
			2 => Operation::Minimum,
			3 => Operation::Maximum,
			5 => Operation::GreaterThan,
			6 => Operation::LessThan,
			7 => Operation::EqualTo,
			_ => unreachable!(),
		};
		return Packet {
			version,
			type_id,
			packet_type: PacketType::Operator(operation),
			sub_packets: if length_type == 0 {
				parse_multiple_length(data, pointer, packet_length)
			}
			else {
				parse_multiple_number(data, pointer, packet_length)
			},
		};
	}
}

fn parse_input(input: &str) -> Packet {
	let data = convert_to_binary(input);

	let mut pointer = 0;

	parse(&data, &mut pointer)
}

fn sum_versions(packet: &Packet) -> i32 {
	let mut t = packet.version as i32;
	for p in &packet.sub_packets {
		t += sum_versions(p);
	}
	t
}
fn eval(packet: &Packet) -> u64 {
	if let PacketType::Literal(v) = &packet.packet_type {
		*v
	}
	else if let PacketType::Operator(op) = &packet.packet_type {
		match op {
			Operation::Sum => packet.sub_packets.iter().map(|p| eval(p)).sum(),
			Operation::Product => packet.sub_packets.iter().map(|p| eval(p)).product(),
			Operation::Minimum => packet.sub_packets.iter().map(|p| eval(p)).min().unwrap(),
			Operation::Maximum => packet.sub_packets.iter().map(|p| eval(p)).max().unwrap(),
			Operation::GreaterThan => {
				let a = eval(&packet.sub_packets[0]);
				let b = eval(&packet.sub_packets[1]);
				if a > b {
					1
				}
				else {
					0
				}
			},
			Operation::LessThan => {
				let a = eval(&packet.sub_packets[0]);
				let b = eval(&packet.sub_packets[1]);
				if a < b {
					1
				}
				else {
					0
				}
			},
			Operation::EqualTo => {
				let a = eval(&packet.sub_packets[0]);
				let b = eval(&packet.sub_packets[1]);
				if a == b {
					1
				}
				else {
					0
				}
			},
		}
	}
	else {
		unreachable!()
	}
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> i32 {
	let packet = parse_input(input);
	sum_versions(&packet)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u64 {
	let packet = parse_input(input);
	eval(&packet)
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	#[test]
	fn example_1_part1() {
		assert_eq!(part1("8A004A801A8002F478"), 16);
	}

	#[test]
	fn example_2_part1() {
		assert_eq!(part1("620080001611562C8802118E34"), 12);
	}

	#[test]
	fn example_3_part1() {
		assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
	}

	#[test]
	fn example_4_part1() {
		assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
	}

	#[test]
	fn example_1_part2() {
		assert_eq!(part2("C200B40A82"), 3);
	}

	#[test]
	fn example_2_part2() {
		assert_eq!(part2("04005AC33890"), 54);
	}

	#[test]
	fn example_3_part2() {
		assert_eq!(part2("880086C3E88112"), 7);
	}

	#[test]
	fn example_4_part2() {
		assert_eq!(part2("CE00C43D881120"), 9);
	}

	#[test]
	fn example_5_part2() {
		assert_eq!(part2("D8005AC2A8F0"), 1);
	}

	#[test]
	fn example_6_part2() {
		assert_eq!(part2("F600BC2D8F"), 0);
	}

	#[test]
	fn example_7_part2() {
		assert_eq!(part2("9C005AC2F8F0"), 0);
	}

	#[test]
	fn example_8_part2() {
		assert_eq!(part2("9C0141080250320F1802104A08"), 1);
	}
}
