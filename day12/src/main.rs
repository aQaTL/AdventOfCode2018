fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let input: Vec<_> = input.lines().collect();

	let mut plants: Vec<bool> = input[0].as_bytes()[15..].iter().map(|x| *x == '#' as u8).collect();
	let notes: Vec<Note> = input[2..].iter().map(|s| str_to_note(*s)).collect();

	(0..10).for_each(|_| plants.insert(0, false));
	(0..200).for_each(|_| plants.push(false));

	let (mut last_gen_value, mut last_diff, mut generation) = (0, 0, 0);
	loop {
		generation += 1;
		let mut new_gen_plants = plants.clone();
		for i in 2..new_gen_plants.len() - 2 {
			for note in &notes {
				let mut note_match = true;
				for (idx, plant) in (i - 2..=i + 2).enumerate() {
					if note.pattern[idx] != plants[plant] {
						note_match = false;
						break;
					}
				}
				if note_match {
					new_gen_plants[i] = note.result;
					break;
				}
			}
		}

		let curr_gen_value = new_gen_plants.iter().enumerate().filter(|(_, x)| **x)
			.map(|(i, _)| (i as i64) - 10).sum::<i64>();

		if generation == 20 {
			println!("Part 1: {}", curr_gen_value);
		}
		plants = new_gen_plants;

		let curr_diff = curr_gen_value - last_gen_value;
		last_gen_value = curr_gen_value;
		if curr_diff == last_diff {
			break;
		}
		last_diff = curr_diff;
	}

	println!("Part 2: {}", last_gen_value as u64 + (50000000000u64 - generation) * last_diff as u64);
}

struct Note {
	pattern: Vec<bool>,
	result: bool,
}

fn str_to_note(s: &str) -> Note {
	Note {
		pattern: s.as_bytes()[..5].iter().map(|x| *x == '#' as u8).collect::<Vec<bool>>(),
		result: s.as_bytes()[9] == '#' as u8,
	}
}
