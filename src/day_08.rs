use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::debug;
use std::ascii::Char;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

crate::decl_tests! {}

fn eval_pt_1(input: &str) -> Result<impl Display> {
	let (map, height, width) = parse_map(input)?;

	let mut antinodes = HashSet::new();
	for (_, info) in map.iter() {
		for (&(l1, c1), &(l2, c2)) in info.coords.iter().tuple_combinations() {
			let line_diff = l2.checked_signed_diff(l1).unwrap();
			let col_diff = c2.checked_signed_diff(c1).unwrap();
			let al1 = l1.checked_add_signed(-line_diff).filter(|l| *l < height);
			let al2 = l2.checked_add_signed(line_diff).filter(|l| *l < height);
			let ac1 = c1.checked_add_signed(-col_diff).filter(|c| *c < width);
			let ac2 = c2.checked_add_signed(col_diff).filter(|c| *c < width);
			if let Some(a1) = al1.zip(ac1) {
				antinodes.insert(a1);
			}
			if let Some(a2) = al2.zip(ac2) {
				antinodes.insert(a2);
			}
		}
	}
	debug_antinodes(&antinodes, width, height);
	Ok(antinodes.len())
}

fn eval_pt_2(input: &str) -> Result<impl Display> {
	let (map, height, width) = parse_map(input)?;

	let mut antinodes = HashSet::new();
	for (_, info) in map.iter() {
		for (&(l1, c1), &(l2, c2)) in info.coords.iter().tuple_combinations() {
			let line_diff = l2.checked_signed_diff(l1).unwrap();
			let col_diff = c2.checked_signed_diff(c1).unwrap();

			let mut al = Some(l1);
			let mut ac = Some(c1);

			while let Some(a) = al.zip(ac) {
				antinodes.insert(a);
				al = a.0.checked_add_signed(-line_diff).filter(|l| *l < height);
				ac = a.1.checked_add_signed(-col_diff).filter(|c| *c < width);
			}

			al = Some(l2);
			ac = Some(c2);

			while let Some(a) = al.zip(ac) {
				antinodes.insert(a);
				al = a.0.checked_add_signed(line_diff).filter(|l| *l < height);
				ac = a.1.checked_add_signed(col_diff).filter(|c| *c < width);
			}
		}
	}
	debug_antinodes(&antinodes, width, height);
	Ok(antinodes.len())
}

fn parse_map(input: &str) -> Result<(HashMap<Char, FrequencyInfo>, usize, usize)> {
	let mut map = HashMap::<Char, FrequencyInfo>::new();
	let mut height = 0;
	let mut width = 0;
	for (line, s) in input.lines().enumerate() {
		height = line + 1;
		width = s.len();
		for (col, c) in s.chars().enumerate() {
			if c.is_ascii_alphanumeric() {
				let c = c.as_ascii().unwrap();
				let mut info = map.entry(c).or_default();
				info.coords.push((line, col));
			}
		}
	}
	Ok((map, height, width))
}

#[derive(Debug, Default)]
struct FrequencyInfo {
	coords: Vec<(usize, usize)>,
}

fn debug_antinodes(antinodes: &HashSet<(usize, usize)>, width: usize, height: usize) {
	let mut s = String::new();
	for i in 0..height {
		for j in 0..width {
			if antinodes.contains(&(i, j)) {
				s.push('#');
			} else {
				s.push('.');
			}
		}
		s.push('\n');
	}
	debug!("\n\n{s}\n");
}
