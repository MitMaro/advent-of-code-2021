use std::str::FromStr;

use aoc_runner_derive::aoc;
use petgraph::{
	algo::dijkstra::dijkstra,
	graph::{Graph, NodeIndex},
};

pub fn parse_input(
	input: &str,
	repeat: usize,
) -> ((NodeIndex, NodeIndex), Graph<i32, i32>, Vec<Vec<(NodeIndex, i32)>>) {
	let mut graph = Graph::new();
	let mut grid = vec![];

	let tile = input
		.trim()
		.lines()
		.map(|v| {
			v.chars()
				.map(|v| {
					let w = i32::from_str(v.to_string().as_str()).unwrap();
					w
				})
				.collect::<Vec<i32>>()
		})
		.collect::<Vec<Vec<i32>>>();

	for offset_row in 0..repeat {
		for r in &tile {
			let mut row = vec![];
			for offset_col in 0..repeat {
				for col in r {
					let value = *col + offset_row as i32 + offset_col as i32;
					let weight = if value > 18 {
						1
					}
					else if value > 9 {
						value - 9
					}
					else {
						value
					};

					row.push((graph.add_node(weight), weight))
				}
			}
			grid.push(row);
		}
	}

	for row in 0..grid.len() {
		for col in 0..grid[row].len() {
			let this_node = grid[row][col];
			for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
				let r = row as i32 + dr;
				let c = col as i32 + dc;
				if r < 0 || c < 0 || r >= grid.len() as i32 || c >= grid[row].len() as i32 {
					continue;
				}
				let node = grid[r as usize][c as usize];
				graph.add_edge(this_node.0, node.0, node.1);
			}
		}
	}

	((grid[0][0].0, grid[grid.len() - 1][grid[0].len() - 1].0), graph, grid)
}

fn solve(input: &str, repeats: usize) -> i32 {
	let ((start, end), graph, _) = parse_input(input, repeats);
	let path = dijkstra(&graph, start, Some(end), |e| *e.weight());
	*path.get(&end).unwrap()
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> i32 {
	solve(input, 1)
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i32 {
	solve(input, 5)
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		1163751742
		1381373672
		2136511328
		3694931569
		7463417111
		1319128137
		1359912421
		3125421639
		1293138521
		2311944581
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(EXAMPLE), 40);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(EXAMPLE), 315);
	}
}
