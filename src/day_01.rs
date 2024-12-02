use anyhow::{Context, Result};

crate::decl_tests! {}

pub fn eval_pt_1(input: &str) -> Result<u64> {
	let (mut left, mut right) = parse_lists(input)?;
	left.sort();
	right.sort();
	Ok(left
		.into_iter()
		.zip(right)
		.map(|(left, right)| left.abs_diff(right))
		.sum())
}

pub fn eval_pt_2(input: &str) -> Result<u64> {
	let (left, right) = parse_lists(input)?;

	Ok(left
		.into_iter()
		.map(|l| l * right.iter().copied().filter(|&r| l == r).count() as u64)
		.sum())
}

fn parse_lists(input: &str) -> Result<(Vec<u64>, Vec<u64>)> {
	let mut left = Vec::new();
	let mut right = Vec::new();
	for line in input.lines() {
		let mut line = line.split_whitespace();
		let l = line.next().context("left missing")?;
		let r = line.next().context("right missing")?;
		let (l, r) = l
			.parse::<u64>()
			.and_then(|left| r.parse::<u64>().map(|right| (left, right)))?;
		left.push(l);
		right.push(r);
	}
	Ok((left, right))
}
