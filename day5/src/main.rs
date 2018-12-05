fn main() {
	let input = std::fs::read("input.txt").unwrap();
	println!("Part 1: {}", part_1(input.clone()));
	println!("Part 2: {}", part_2(input));
}

fn part_1(mut input: Vec<u8>) -> usize {
	react(&mut input);
	input.len()
}

fn part_2(mut input: Vec<u8>) -> usize {
	let mut best_react = usize::max_value();
	react(&mut input);
	for c in 65..=90 {
		let mut input_2 = input.iter()
			.filter(|&&x| x != c && x != c + 32)
			.map(|x| x.clone())
			.collect::<Vec<_>>();
		react(&mut input_2);
		if input_2.len() < best_react {
			best_react = input_2.len();
		}
	}
	best_react
}

fn react(input: &mut Vec<u8>) {
	'l: loop {
		for i in 0..input.len() - 1 {
			if input[i] == input[i + 1] + 32 || input[i] + 32 == input[i + 1] {
				input.remove(i);
				input.remove(i);
				continue 'l;
			}
		}
		break;
	}
}