#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(strict_overflow_ops)]
#![feature(array_windows)]
#![feature(unsigned_signed_diff)]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
// pub mod day_10;
// pub mod day_11;
// pub mod day_12;
// pub mod day_13;
// pub mod day_14;
// pub mod day_15;
// pub mod day_16;
// pub mod day_17;
// pub mod day_18;
// pub mod day_19;
// pub mod day_20;
// pub mod day_21;
// pub mod day_22;
// pub mod day_23;
// pub mod day_24;
// pub mod day_25;

#[cfg(test)]
fn init() {
	let _ = env_logger::builder().is_test(true).try_init();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
	pub ex_in: String,
	pub ex_in_2: Option<String>,
	pub ex_ans_1: String,
	pub ex_ans_2: String,
	pub input: String,
}

pub fn load_day_input(rs_filename: &str) -> anyhow::Result<(&str, Input)> {
	let rs_file_path = std::path::Path::new(rs_filename);
	let stem = rs_file_path.file_stem().unwrap().to_str().unwrap();
	let file = File::open(format!("inputs/{}.ron", stem))?;
	let reader = BufReader::new(file);
	Ok((stem, ron::de::from_reader(reader)?))
}

#[macro_export]
macro_rules! decl_tests {
	() => {
		#[test]
		fn part_1() -> anyhow::Result<()> {
			crate::init();
			let (day, input) = $crate::load_day_input(file!())?;
			anyhow::ensure!(format!("{}", eval_pt_1(&input.ex_in)?) == input.ex_ans_1);
			log::info!("example passed part 1");
			println!("{day} pt 1 answer: {}", eval_pt_1(&input.input)?);
			Ok(())
		}

		#[test]
		fn part_2() -> anyhow::Result<()> {
			crate::init();
			let (day, input) = $crate::load_day_input(file!())?;
			let ex_in = if let Some(ex_in_2) = &input.ex_in_2 {
				ex_in_2
			} else {
				&input.ex_in
			};
			anyhow::ensure!(format!("{}", eval_pt_2(ex_in)?) == input.ex_ans_2);
			log::info!("example passed part 2");
			println!("{day} pt 2 answer: {}", eval_pt_2(&input.input)?);
			Ok(())
		}
	};
}
