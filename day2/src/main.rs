use std::collections::HashMap;

fn main() {
	let ids = std::fs::read_to_string("input.txt").expect("input.txt not found")
		.lines().
		map(|x| x.chars().collect::<Vec<char>>()).
		collect::<Vec<Vec<char>>>();

	println!("Part 1: {}", part_1(&ids));
	println!("Part 2: {}", part_2(&ids));
}

fn part_1(ids: &Vec<Vec<char>>) -> i32 {
	let (mut twos, mut threes) = (0i32, 0i32);
	let mut hm = HashMap::new();
	for id in ids.iter() {
		hm.clear();
		for c in id {
			let stat = hm.entry(*c).or_insert(0);
			*stat += 1;
		}
		if let Some(_) = hm.values().find(|&&x| x == 2) {
			twos += 1;
		}
		if let Some(_) = hm.values().find(|&&x| x == 3) {
			threes += 1;
		}
	}
	twos * threes
}

fn part_2(ids: &Vec<Vec<char>>) -> String {
	for (idx, id) in ids.iter().enumerate() {
		let id2 = ids.iter()
			.skip(idx + 1)
			.find(|id2| id2.iter()
				.zip(id.iter())
				.filter(|(x, y)| x != y)
				.count() == 1);

		if let Some(id2) = id2 {
			return ids[idx].iter()
				.zip(id2.iter())
				.filter(|(x, y)| x == y)
				.map(|(x, _)| x)
				.collect::<String>();
		}
	}
	String::from("")
}
