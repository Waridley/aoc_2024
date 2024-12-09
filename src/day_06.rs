use anyhow::{anyhow, ensure, Result};
use std::collections::HashSet;
use std::fmt::Display;

crate::decl_tests! {}

fn eval_pt_1(input: &str) -> Result<impl Display> {
	use Cell::*;
	use Direction::*;
	let (mut map, pos, mut direction) = build_map(input)?;
	walk_path(&mut map, pos, direction);
	let num_visited = map
		.into_iter()
		.flatten()
		.filter(|cell| match cell {
			Visited { .. } => true,
			_ => false,
		})
		.count();
	Ok(num_visited)
}

fn eval_pt_2(input: &str) -> Result<impl Display> {
	use Cell::*;
	use Direction::*;
	let (map, pos, direction) = build_map(input)?;
	let mut orig_walked_map = map.clone();
	walk_path(&mut orig_walked_map, pos, direction);
	let mut poss_placements = orig_walked_map.into_iter()
		.enumerate()
		.flat_map(|(line, cells)| cells.into_iter().enumerate().map(move |(col, cell)| (line, col, cell)))
		.filter_map(|(line, col, cell)| match cell {
			Visited(_) => Some((line, col)),
			_ => None,
		});
	let mut count = 0;
	for (line, col) in poss_placements {
		let mut map = map.clone();
		map[line][col] = Obstacle;
		match walk_path(&mut map, pos, direction) {
			PathWalkResult::ExitedArea { .. } => {}
			PathWalkResult::Looping => count += 1,
		}
	}
	Ok(count)
}

#[derive(Debug, Clone)]
enum Cell {
	Empty,
	Obstacle,
	Visited(HashSet<Direction>),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn turn_right(self) -> Direction {
		use Direction::*;
		match self {
			Up => Right,
			Down => Left,
			Left => Up,
			Right => Down,
		}
	}
}

fn build_map(input: &str) -> Result<(Vec<Vec<Cell>>, (usize, usize), Direction)> {
	use Cell::*;
	use Direction::*;

	let mut start = (usize::MAX, usize::MAX);
	let mut direction = Up;
	let map = input
		.lines()
		.enumerate()
		.map(|(line, s)| {
			s.chars()
				.enumerate()
				.map(|(col, c)| {
					Ok(match c {
						'.' => Empty,
						'#' => Obstacle,
						'^' => {
							start = (line, col);
							direction = Up;
							Visited(HashSet::from([direction]))
						}
						'v' => {
							start = (line, col);
							direction = Down;
							Visited(HashSet::from([direction]))
						}
						'<' => {
							start = (line, col);
							direction = Left;
							Visited(HashSet::from([direction]))
						}
						'>' => {
							start = (line, col);
							direction = Right;
							Visited(HashSet::from([direction]))
						}
						c => return Err(anyhow!("Unexpected character {c}")),
					})
				})
				.collect::<Result<Vec<_>>>()
		})
		.collect::<Result<Vec<_>>>()?;
	ensure!(start.0 != usize::MAX && start.1 != usize::MAX);
	Ok((map, start, direction))
}

fn walk_path(
	map: &mut Vec<Vec<Cell>>,
	mut position: (usize, usize),
	mut direction: Direction,
) -> PathWalkResult {
	use Cell::*;
	use Direction::*;

	let map_width = map[0].len();
	loop {
		let next = match direction {
			Up => {
				if position.0 == 0 {
					break PathWalkResult::ExitedArea {
						last_pos: position,
						last_dir: direction,
					};
				} else {
					(-1, 0)
				}
			}
			Down => {
				if position.0 >= map.len() - 1 {
					break PathWalkResult::ExitedArea {
						last_pos: position,
						last_dir: direction,
					};
				} else {
					(1, 0)
				}
			}
			Left => {
				if position.1 == 0 {
					break PathWalkResult::ExitedArea {
						last_pos: position,
						last_dir: direction,
					};
				} else {
					(0, -1)
				}
			}
			Right => {
				if position.1 >= map_width - 1 {
					break PathWalkResult::ExitedArea {
						last_pos: position,
						last_dir: direction,
					};
				} else {
					(0, 1)
				}
			}
		};
		let (next_line, next_col) = (
			position.0.strict_add_signed(next.0),
			position.1.strict_add_signed(next.1),
		);
		let next = &mut map[next_line][next_col];
		match next {
			Obstacle => direction = direction.turn_right(),
			Visited(dirs) => {
				if !dirs.insert(direction) {
					return PathWalkResult::Looping;
				}
				position = (next_line, next_col);
			}
			Empty => {
				*next = Visited(HashSet::from([direction]));
				position = (next_line, next_col);
			}
		};
	}
}

enum PathWalkResult {
	ExitedArea {
		last_pos: (usize, usize),
		last_dir: Direction,
	},
	Looping,
}
