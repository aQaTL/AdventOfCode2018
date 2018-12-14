fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap().parse::<u32>().unwrap();

	println!("Part 1: {}", part_1(input));
	println!("Part 2: {}", part_2(input));
}

fn part_1(input: u32) -> String {
	let mut recipes = vec![3u8, 7u8];
	let (mut elf_1, mut elf_2) = (0usize, 1usize);

	let mut recipes_created = 0;
	while recipes_created < (input + 10) {
		let new_recipe = recipes[elf_1] + recipes[elf_2];
		if new_recipe < 10 {
			recipes.push(new_recipe);
			recipes_created += 1;
		} else {
			recipes.push((new_recipe as f32 / 10.0) as u8);
			recipes.push(new_recipe % 10);
			recipes_created += 2;
		}
		elf_1 = (elf_1 + recipes[elf_1] as usize + 1) % recipes.len();
		elf_2 = (elf_2 + recipes[elf_2] as usize + 1) % recipes.len();
	}
	recipes[input as usize..input as usize + 10].iter().map(u8::to_string).collect::<String>()
}

fn part_2(input: u32) -> usize {
	let mut recipes = vec![3u8, 7u8];
	let (mut elf_1, mut elf_2) = (0usize, 1usize);
	let result: Vec<u8> = input.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

	'l: loop {
		let new_recipe = recipes[elf_1] + recipes[elf_2];
		let new_recipes;
		if new_recipe < 10 {
			recipes.push(new_recipe);
			new_recipes = 1;
		} else {
			recipes.push((new_recipe as f32 / 10.0) as u8);
			recipes.push(new_recipe % 10);
			new_recipes = 2;
		}
		elf_1 = (elf_1 + recipes[elf_1] as usize + 1) % recipes.len();
		elf_2 = (elf_2 + recipes[elf_2] as usize + 1) % recipes.len();
		if recipes.len() < result.len() {
			continue;
		}
		for (idx, i) in result.iter().rev().enumerate() {
			if recipes[recipes.len() - new_recipes - idx] != *i {
				continue 'l;
			}
		}
		break;
	}

	't: for i in 0..recipes.len() - result.len() {
		for (idx, j) in result.iter().enumerate() {
			if recipes[i + idx] != *j {
				continue 't;
			}
		}
		return i;
	}
	0
}