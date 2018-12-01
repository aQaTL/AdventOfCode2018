use std::collections::HashMap;

fn main() {
	let input = std::fs::read_to_string("input.txt").expect("input.txt not found").lines()
		.filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();

	println!("Part 1: {}", input.iter().sum::<i32>());

	let mut sum = 0i32;
	let mut seen = HashMap::with_capacity(input.len());
	for n in input.iter().cycle() {
		seen.insert(sum, ());
		sum += n;
		if seen.contains_key(&sum) {
			break;
		}
	}

	println!("Part 2: {}", sum);
}
