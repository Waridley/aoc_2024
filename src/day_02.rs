use anyhow::Result;
use std::fmt::Display;
use std::str::FromStr;

crate::decl_tests! {}

pub fn eval_pt_1(input: &str) -> Result<usize> {
	Ok(input
		.lines()
		.filter(|line| {
			list_is_safe(
				line.split_whitespace()
					.map(i64::from_str)
					.map(Result::unwrap),
			)
		})
		.count())
}

pub fn eval_pt_2(input: &str) -> Result<usize> {
	Ok(input
		.lines()
		.filter_map(|line| {
			let list = line
				.split_whitespace()
				.map(i64::from_str)
				.map(ok_but_print_err)
				.collect::<Option<Vec<_>>>()?;

			if list_is_safe(list.iter().copied()) {
				return Some(());
			} else {
				for i in 0..list.len() {
					if list_is_safe(
						list[0..i]
							.iter()
							.copied()
							.chain(list[(i + 1)..].iter().copied()),
					) {
						return Some(());
					}
				}
			}
			None

			// // This did not get the right answer. Might be more efficient if it can be fixed.
			//
			// #[derive(Copy, Clone, Debug)]
			// struct Node {
			// 	next: Option<Direction>,
			// 	skip: Option<Direction>,
			// }
			//
			// let mut graph = Vec::with_capacity(list.len() - 1);
			//
			// for i in 0..(list.len() - 1) {
			// 	let next = Direction::from_diff(list[i + 1] - list[i]);
			// 	let skip = list.get(i + 2)
			// 		.map(|skip| *skip - list[i])
			// 		.and_then(Direction::from_diff);
			// 	graph.push(Node {
			// 		next,
			// 		skip,
			// 	});
			// }
			// fn check_paths(graph: &Vec<Node>, i: usize, dir: Option<Direction>) -> Option<usize> {
			// 	let Some(node) = graph.get(i) else { return Some(0) };
			// 	let next = if let Some(next) = node.next {
			// 		match dir {
			// 			Some(dir) if dir == next => {
			// 				check_paths(graph, i + 1, Some(next))
			// 			},
			// 			None => check_paths(graph, i + 1, Some(next)),
			// 			_ => None,
			// 		}
			// 	} else { None };
			// 	let skip = if let Some(skip) = node.skip {
			// 		match dir {
			// 			Some(dir) if dir == skip => {
			// 				check_paths(graph, i + 2, Some(skip))
			// 			},
			// 			None => check_paths(graph, i + 2, Some(skip)),
			// 			_ => None,
			// 		}.map(|n| n + 1)
			//
			// 	} else { None };
			// 	match (next, skip) {
			// 		(Some(next), Some(skip)) => Some(usize::min(next, skip)),
			// 		(Some(next), None) => Some(next),
			// 		(None, Some(skip)) => Some(skip),
			// 		(None, None) => None,
			// 	}
			// }
			//
			// check_paths(&graph, 0, None).filter(|dampened| *dampened <= 1)
		})
		.count())
}

fn list_is_safe(mut list: impl Iterator<Item = i64>) -> bool {
	let mut prev = list.next().unwrap();
	let next = list.next().unwrap();
	let diff = next - prev;
	let (min, max) = if diff >= 1 && diff <= 3 {
		(1, 3)
	} else if diff <= -1 && diff >= -3 {
		(-3, -1)
	} else {
		return false;
	};
	prev = next;
	for next in list {
		let diff = next - prev;
		if diff < min || diff > max {
			return false;
		}
		prev = next;
	}
	true
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// enum Direction {
// 	Increasing,
// 	Decreasing,
// }
//
// impl Direction {
// 	fn from_diff(diff: i64) -> Option<Self> {
// 		use Direction::*;
// 		if diff >= 1 && diff <= 3 {
// 			Some(Increasing)
// 		} else if diff >= -3 && diff <= -1 {
// 			Some(Decreasing)
// 		} else {
// 			None
// 		}
// 	}
// }

fn ok_but_print_err<T, E: Display>(result: Result<T, E>) -> Option<T> {
	match result {
		Ok(val) => Some(val),
		Err(e) => {
			eprintln!("{e}");
			None
		}
	}
}
