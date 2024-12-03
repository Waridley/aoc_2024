use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

pub mod day_01;
pub mod day_02;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
	pub ex_in: String,
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
			let (day, input) = $crate::load_day_input(file!())?;
			anyhow::ensure!(format!("{}", eval_pt_1(&input.ex_in)?) == input.ex_ans_1);
			println!("{day} pt 1 answer: {}", eval_pt_1(&input.input)?);
			Ok(())
		}

		#[test]
		fn part_2() -> anyhow::Result<()> {
			let (day, input) = $crate::load_day_input(file!())?;
			anyhow::ensure!(format!("{}", eval_pt_2(&input.ex_in)?) == input.ex_ans_2);
			println!("{day} pt 2 answer: {}", eval_pt_2(&input.input)?);
			Ok(())
		}
	};
}
