use anyhow::{anyhow, ensure, Result};
use log::{info, warn};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;

crate::decl_tests! {}

fn eval_pt_1(input: &str) -> Result<impl Display> {
	let input = input.parse()?;
	let rule_map = Rules::build_map(&input)?;
	let updates = input.updates;
	let mut sum = 0;
	for update in &updates {
		if is_update_valid(update, &rule_map) {
			let mid = update.len() / 2;
			let mid = update[mid];
			info!("({update:?}) passed all rules -- midpoint = {mid}");
			sum += mid;
		}
	}
	Ok(sum)
}

fn eval_pt_2(input: &str) -> Result<impl Display> {
	let input = input.parse()?;
	let rule_map = Rules::build_map(&input)?;
	let mut updates = input.updates;
	let mut sum = 0;
	for update in &mut updates {
		if !is_update_valid(update, &rule_map) {
			info!("({update:?} is not valid, sorting...");
			update.sort_by(|a, b| {
				let mut ordering = None;
				if let Some(a_rules) = rule_map.get(a) {
					if a_rules.before.contains(b) {
						ordering = Some(Ordering::Less);
					}
					if a_rules.after.contains(b) {
						debug_assert!(ordering.is_none(), "contradicting rules");
						ordering = Some(Ordering::Greater);
					}
				}
				if let Some(b_rules) = rule_map.get(b) {
					if b_rules.before.contains(b) {
						debug_assert!(
							ordering.is_none_or(|o| o == Ordering::Greater),
							"contradicting rules"
						);
						ordering = Some(Ordering::Greater);
					}
					if b_rules.after.contains(b) {
						debug_assert!(
							ordering.is_none_or(|o| o == Ordering::Less),
							"contradicting rules"
						);
						ordering = Some(Ordering::Less);
					}
				}
				ordering.unwrap_or_else(|| {
					warn!("No explicit ordering for ({a}, {b})");
					Ordering::Equal
				})
			});
			let mid = update.len() / 2;
			let mid = update[mid];
			info!("sorted: ({update:?}) -- midpoint = {mid}");
			sum += mid;
		}
	}
	Ok(sum)
}

struct Input {
	rules: Vec<(u64, u64)>,
	updates: Vec<Vec<u64>>,
}

impl FromStr for Input {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		let mut rules = Vec::new();
		let mut updates = Vec::new();
		let mut input = s.lines();
		loop {
			let rule = input
				.next()
				.ok_or(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
			if rule.is_empty() {
				break;
			}
			let (before, after) = rule
				.split_once('|')
				.ok_or(std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
			let before = before.parse::<u64>()?;
			let after = after.parse::<u64>()?;
			rules.push((before, after));
		}

		for update in input {
			updates.push(
				update
					.split(',')
					.map(str::parse::<u64>)
					.collect::<Result<Vec<_>, _>>()?,
			)
		}

		Ok(Input { rules, updates })
	}
}

#[derive(Debug, Default)]
struct Rules {
	before: HashSet<u64>,
	after: HashSet<u64>,
}

impl Rules {
	fn build_map(Input { rules, updates }: &Input) -> Result<HashMap<u64, Self>> {
		let mut rule_map = HashMap::<u64, Rules>::new();
		for &(first, second) in rules {
			let rules = rule_map.entry(first).or_default();
			if rules.after.contains(&second) {
				anyhow::bail!("contradictory rules")
			}
			rules.before.insert(second);
			let rules = rule_map.entry(second).or_default();
			if rules.before.contains(&first) {
				anyhow::bail!("contradictory rules")
			}
			rules.after.insert(first);
		}
		Ok(rule_map)
	}
}

fn is_update_valid(update: &Vec<u64>, rule_map: &HashMap<u64, Rules>) -> bool {
	for i in 0..update.len() {
		let curr_page = update[i];
		let Some(rules) = rule_map.get(&curr_page) else {
			warn!("no rules for {curr_page}");
			continue;
		};
		for j in 0..i {
			let checking = update[j];
			if rules.before.contains(&checking) {
				info!("{update:?} invalid because {curr_page} comes after {checking}");
				return false;
			}
		}
		for j in i + 1..update.len() {
			let checking = update[j];
			if rules.after.contains(&checking) {
				info!("({update:?} invalid because {curr_page} comes before {checking}");
				return false;
			}
		}
	}
	true
}
