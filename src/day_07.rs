use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::{debug, info, warn};
use std::fmt::Display;
use std::num::IntErrorKind;
use std::ptr::eq;
use std::str::FromStr;

crate::decl_tests! {}

fn eval_pt_1(input: &str) -> Result<impl Display> {
	let equations = Equation::parse_list(input)?;
	try_ops(equations.into_iter(), &[Op::Add, Op::Mul])
}

fn eval_pt_2(input: &str) -> Result<impl Display> {
	let equations = Equation::parse_list(input)?;
	try_ops(equations.into_iter(), &[Op::Add, Op::Mul, Op::Cat])
}

#[derive(Debug)]
struct Equation {
	test_value: u64,
	numbers: Vec<u64>,
}

impl Equation {
	fn parse_list(s: &str) -> Result<Vec<Self>> {
		s.lines()
			.map(|line| {
				let (val, nums) = line.split_once(": ").ok_or(anyhow!("invalid input"))?;
				let val = val.parse::<u64>()?;
				let nums = nums
					.split_whitespace()
					.map(|num| num.parse::<u64>().map_err(anyhow::Error::from))
					.collect::<Result<Vec<_>>>()?;
				Ok(Equation {
					test_value: val,
					numbers: nums,
				})
			})
			.collect::<Result<Vec<_>>>()
	}
}

#[derive(Debug, Copy, Clone)]
enum Op {
	Add,
	Mul,
	Cat,
}

fn try_ops(equations: impl Iterator<Item = Equation>, ops: &[Op]) -> Result<u64> {
	let mut sum = 0;
	'equation_loop: for equation in equations {
		info!("{equation:?}");
		let mut op_count = equation.numbers.len() - 1;
		for ops in (0..op_count)
			.map(|_| ops.iter().copied())
			.multi_cartesian_product()
		{
			let mut nums = equation.numbers.iter().copied();
			let mut val = nums
				.next()
				.ok_or_else(|| anyhow!("expected at least one number"))?;
			for (num, op) in nums.zip(ops.iter().copied()) {
				match op {
					Op::Add => val += num,
					Op::Mul => val *= num,
					Op::Cat => {
						val = match format!("{val}{num}").parse() {
							Ok(num) => num,
							Err(e) => match e.kind() {
								IntErrorKind::PosOverflow => {
									warn!("concatenating {val} and {num} would be too long");
									continue;
								}
								_ => return Err(e.into()),
							},
						}
					}
				}
			}
			if val == equation.test_value {
				info!("✔️ {ops:?} solves to {val}");
				sum += val;
				continue 'equation_loop;
			} else {
				debug!("❌ {ops:?} solves to {val}");
			}
		}
	}
	Ok(sum)
}
