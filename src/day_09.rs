use anyhow::{anyhow, Result};
use log::debug;
use std::fmt::Display;
use std::fmt::Write;

crate::decl_tests! {}

fn eval_pt_1(input: &str) -> Result<impl Display> {
	let (mut disk, _) = parse_disk(input)?;

	let mut consolidated = Vec::new();

	let mut i = 0;
	loop {
		match disk.get(i) {
			Some(None) => loop {
				match disk.pop() {
					Some(None) => continue,
					Some(Some(id)) => {
						consolidated.push(id);
						break;
					}
					None => unreachable!(),
				}
			},
			Some(Some(id)) => consolidated.push(*id),
			None => break,
		}
		i += 1;
	}

	{
		// debug consolidated
		let mut s = String::new();
		for block in consolidated.iter().copied() {
			write!(&mut s, "{block}")?
		}
		debug!("{s}");
	}

	let mut checksum = 0u64;
	let mut pos = 0;

	for id in consolidated {
		checksum += pos as u64 * id as u64;
		pos += 1;
	}

	Ok(checksum)
}

fn eval_pt_2(input: &str) -> Result<impl Display> {
	let (_, mut file_list) = parse_disk(input)?;

	let mut last_id = i64::MAX;
	loop {
		if last_id <= 0 {
			break;
		}
		let Some((mut i, &(Span::File { id }, size))) =
			file_list
				.iter()
				.enumerate()
				.max_by_key(|(_, (span, _))| match span {
					&Span::File { id } if (id as i64) < last_id => id as i64,
					_ => i64::MIN,
				})
		else {
			break;
		};
		debug!("id: {id}");
		last_id = id as i64;

		let mut j = 0;
		while j < i {
			match file_list[j] {
				(Span::Free, free_space) => match free_space.checked_sub(size) {
					Some(rem) => {
						file_list[i] = (Span::Free, size);
						if rem > 0 {
							// reduce size of free space
							file_list[j] = (Span::Free, free_space - size);
						} else {
							// no free space will be remaining
							file_list.remove(j);
							i -= 1;
						}
						// Insert it before the free space
						file_list.insert(j, (Span::File { id }, size));

						// merge free space left
						loop {
							if let Some((Span::Free, adj_size)) = file_list.get_mut(i - 1) {
								*adj_size += size;
								file_list.remove(i);
								i -= 1;
							} else {
								break;
							}
						}
						// merge free space right
						while let Some((Span::Free, size)) = file_list.get(i).copied() {
							if let Some((Span::Free, adj_size)) = file_list.get_mut(i + 1) {
								*adj_size += size;
								file_list.remove(i);
							} else {
								break;
							}
						}

						break;
					}
					None => {}
				},
				_ => {}
			}
			j += 1;
		}
	}

	let mut defragged = Vec::new();

	for (span, size) in file_list {
		match span {
			Span::File { id } => {
				for i in 0..size {
					defragged.push(Some(id));
				}
			}
			Span::Free => {
				for _ in 0..size {
					defragged.push(None);
				}
			}
		}
	}

	{
		// debug defragged
		let mut s = String::new();
		for block in defragged.iter().copied() {
			match block {
				Some(id) => write!(&mut s, "{id}")?,
				None => s.push('.'),
			}
		}
		debug!("{s}");
	}

	let mut checksum = 0u64;

	for (pos, id) in defragged.into_iter().enumerate() {
		if let Some(id) = id {
			checksum += pos as u64 * id as u64;
		}
	}

	Ok(checksum)
}

fn parse_disk(input: &str) -> Result<(Vec<Option<u32>>, Vec<(Span, usize)>)> {
	let mut input = input.chars();

	let mut disk = Vec::new();
	let mut file_list = Vec::new();

	let mut id = 0u32;
	loop {
		let Some(file_size) = input.next() else { break };
		let file_size = file_size
			.to_digit(10)
			.ok_or_else(|| anyhow!("Invalid number {file_size}"))?
			.try_into()?;

		file_list.push((Span::File { id }, file_size));
		disk.extend(std::iter::repeat(Some(id)).take(file_size));
		id += 1;

		let Some(free_space) = input.next() else {
			break;
		};
		let free_space = free_space
			.to_digit(10)
			.ok_or_else(|| anyhow!("Invalid number {free_space}"))?
			.try_into()?;

		file_list.push((Span::Free, free_space));
		disk.extend(std::iter::repeat(None).take(free_space));
	}
	Ok((disk, file_list))
}

#[derive(Debug, Clone, Copy)]
enum Span {
	File { id: u32 },
	Free,
}
