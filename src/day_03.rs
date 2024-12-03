use anyhow::Result;
use std::fmt::Display;

crate::decl_tests! {}

pub fn eval_pt_1(mut input: &str) -> Result<u64> {
	let mut sum = 0;
	loop {
		let Some(i) = input.find("mul(") else { break };
		input = &input[i + 4..];
		let (args, rem) = parse_mul_args(input);
		input = rem;
		if let Some((lhs, rhs)) = args {
			sum += lhs * rhs;
		}
	}
	Ok(sum)
}

pub fn eval_pt_2(mut input: &str) -> Result<impl Display> {
	const DO_PAT: [char; 4] = ['d', 'o', '(', ')'];
	const DONT_PAT: [char; 7] = ['d', 'o', 'n', '\'', 't', '(', ')'];
	const MUL_PAT: [char; 4] = ['m', 'u', 'l', '('];
	let mut muls_enabled = true;
	let mut do_progress = 0;
	let mut dont_progress = 0;
	let mut mul_progress = 0;
	let mut sum = 0;
	loop {
		let Some((c, rem)) = input.split_at_checked(1) else {
			break;
		};
		input = rem;
		let c = c.parse::<char>()?;
		if c == DO_PAT[do_progress] {
			if do_progress == DO_PAT.len() - 1 {
				muls_enabled = true;
				do_progress = 0;
			} else {
				do_progress += 1;
			}
		} else {
			do_progress = 0;
		}
		if c == DONT_PAT[dont_progress] {
			if dont_progress == DONT_PAT.len() - 1 {
				muls_enabled = false;
				dont_progress = 0;
			} else {
				dont_progress += 1;
			}
		} else {
			dont_progress = 0;
		}
		if muls_enabled && c == MUL_PAT[mul_progress] {
			if mul_progress == MUL_PAT.len() - 1 {
				let (args, rem) = parse_mul_args(input);
				if let Some((lhs, rhs)) = args {
					input = rem;
					sum += lhs * rhs;
				}
				mul_progress = 0;
			} else {
				mul_progress += 1;
			}
		} else {
			mul_progress = 0;
		}
	}
	Ok(sum)
}

fn parse_mul_args(mut input: &str) -> (Option<(u64, u64)>, &str) {
	let mut lhs = String::new();
	loop {
		let (digit, rem) = input.split_at(1);
		let Ok(digit) = digit.parse::<char>() else {
			return (None, rem);
		};
		match digit {
			',' => {
				input = rem;
				break;
			}
			'0'..='9' => {
				if lhs.len() >= 3 {
					// args must be between 1 and 3 digits
					return (None, rem);
				} else {
					lhs.push(digit)
				}
			}
			_ => {
				// invalid mul instruction
				return (None, input);
			}
		}
		input = rem;
	}
	let Ok(lhs) = lhs.parse::<u64>() else {
		return (None, input);
	};
	let mut rhs = String::new();
	loop {
		let (digit, rem) = input.split_at(1);
		let Ok(digit) = digit.parse::<char>() else {
			return (None, rem);
		};
		match digit {
			')' => {
				input = rem;
				break;
			}
			'0'..='9' => {
				if rhs.len() >= 3 {
					// args must be between 1 and 3 digits
					return (None, rem);
				} else {
					rhs.push(digit)
				}
			}
			_ => {
				// invalid mul instruction
				return (None, input);
			}
		}
		input = rem;
	}
	let Ok(rhs) = rhs.parse::<u64>() else {
		return (None, input);
	};
	(Some((lhs, rhs)), input)
}
