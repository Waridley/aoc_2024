use anyhow::{anyhow, ensure, Result};
use std::collections::HashSet;
use std::fmt::Display;

crate::decl_tests! {}

fn eval_pt_1(input: &str) -> Result<impl Display> {
	use Cell::*;
	use Direction::*;
	let mut start = (usize::MAX, usize::MAX);
	let mut direction = Up;
	let mut map = input
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
							Visited
						}
						'v' => {
							start = (line, col);
							direction = Down;
							Visited
						}
						'<' => {
							start = (line, col);
							direction = Left;
							Visited
						}
						'>' => {
							start = (line, col);
							direction = Right;
							Visited
						}
						c => return Err(anyhow!("Unexpected character {c}")),
					})
				})
				.collect::<Result<Vec<_>>>()
		})
		.collect::<Result<Vec<_>>>()?;
	let map_width = map[0].len();
	ensure!(start.0 != usize::MAX && start.1 != usize::MAX);
	loop {
		let next = match direction {
			Up => {
				if start.0 == 0 {
					break;
				} else {
					(-1, 0)
				}
			}
			Down => {
				if start.0 >= map.len() - 1 {
					break;
				} else {
					(1, 0)
				}
			}
			Left => {
				if start.1 == 0 {
					break;
				} else {
					(0, -1)
				}
			}
			Right => {
				if start.1 >= map_width - 1 {
					break;
				} else {
					(0, 1)
				}
			}
		};
		let (next_line, next_col) = (
			start.0.strict_add_signed(next.0),
			start.1.strict_add_signed(next.1),
		);
		let next = &mut map[next_line][next_col];
		match next {
			next @ Obstacle => direction = direction.turn_right(),
			_ => {
				*next = Visited;
				start = (next_line, next_col);
			}
		};
	}
	let num_visited = map
		.into_iter()
		.flatten()
		.filter(|cell| match cell {
			Visited => true,
			_ => false,
		})
		.count();
	Ok(num_visited)
}

fn eval_pt_2(input: &str) -> Result<impl Display> {
	Ok("todo")
}

enum Cell {
	Empty,
	Obstacle,
	Visited,
}

#[derive(Clone, Copy, Debug)]
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
