use anyhow::Result;
use std::ascii::Char;
use std::fmt::Display;

crate::decl_tests! {}

pub fn eval_pt_1(input: &str) -> Result<impl Display> {
	let Some(input) = input.as_ascii() else {
		anyhow::bail!("expected ascii string");
	};
	let input = input
		.split(|c| *c == Char::LineFeed)
		.collect::<Vec<&[Char]>>();
	let mut count = 0;
	for line in 0..input.len() {
		for col in 0..input[line].len() {
			count += count_xmases(&input, line, col);
		}
	}
	Ok(count)
}

pub fn eval_pt_2(input: &str) -> Result<impl Display> {
	let Some(input) = input.as_ascii() else {
		anyhow::bail!("expected ascii string");
	};
	let input = input
		.split(|c| *c == Char::LineFeed)
		.collect::<Vec<&[Char]>>();
	let mut count = 0;
	for line in 1..input.len() - 1 {
		for col in 1..input[line].len() - 1 {
			count += count_x_mases(&input, line, col);
		}
	}
	Ok(count)
}

fn count_xmases(input: &Vec<&[Char]>, line: usize, col: usize) -> usize {
	if input[line][col] != Char::CapitalX {
		return 0;
	}

	let check_dir = |l_dir, c_dir| {
		input[line.strict_add_signed(l_dir)][col.strict_add_signed(c_dir)] == Char::CapitalM
			&& input[line.strict_add_signed(l_dir * 2)][col.strict_add_signed(c_dir * 2)]
				== Char::CapitalA
			&& input[line.strict_add_signed(l_dir * 3)][col.strict_add_signed(c_dir * 3)]
				== Char::CapitalS
	};

	let mut count = 0;
	// look up
	if line >= 3 && check_dir(-1, 0) {
		count += 1;
	}
	// look down
	if line <= input.len() - 4 && check_dir(1, 0) {
		count += 1;
	}
	// look left
	if col >= 3 && check_dir(0, -1) {
		count += 1;
	}
	// look right
	if col <= input[line].len() - 4 && check_dir(0, 1) {
		count += 1;
	}
	// look up-left
	if line >= 3 && col >= 3 && check_dir(-1, -1) {
		count += 1;
	}
	// look up-right
	if line >= 3 && col <= input[line].len() - 4 && check_dir(-1, 1) {
		count += 1;
	}
	// look down-right
	if line <= input.len() - 4 && col <= input[line].len() - 4 && check_dir(1, 1) {
		count += 1;
	}
	// look down-left
	if line <= input.len() - 4 && col >= 3 && check_dir(1, -1) {
		count += 1;
	}
	count
}

fn count_x_mases(input: &Vec<&[Char]>, line: usize, col: usize) -> usize {
	if input[line][col] != Char::CapitalA {
		return 0;
	}
	const TL: (isize, isize) = (-1, -1);
	const TR: (isize, isize) = (-1, 1);
	const BL: (isize, isize) = (1, -1);
	const BR: (isize, isize) = (1, 1);

	let mut count = 0;
	let mut dirs = vec![TL, TR, BL, BR];
	for _ in 0..2 {
		let mut i = 0;
		while i < dirs.len() {
			let dir = dirs[i];
			if input[line.strict_add_signed(dir.0)][col.strict_add_signed(dir.1)] == Char::CapitalM
			{
				dirs.remove(i);
				// This M is obviously not an S, so the opposite can't start a "MAS" either.
				let Some((opp, _)) = dirs
					.iter()
					.enumerate()
					.find(|(_, &it)| it == (-dir.0, -dir.1))
				else {
					unreachable!();
				};
				dirs.remove(opp);
				if input[line.strict_add_signed(-dir.0)][col.strict_add_signed(-dir.1)]
					== Char::CapitalS
				{
					count += 1;
				}
			}
			i += 1;
		}
		if count == 0 {
			// didn't find any "MAS" -- bail early.
			break;
		}
	}
	match count {
		0 => 0,
		1 => 0,
		2 => 1,
		_ => unreachable!(),
	}
}
