use std::collections::{HashSet, HashMap};

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap().lines()
		.map(|x| x.split(", ")
			.map(|s| s.parse::<i32>().unwrap())
			.collect::<Vec<_>>())
		.collect::<Vec<Vec<_>>>();

	let max_x = input.iter().max_by(|x, y| x[0].cmp(&y[0])).unwrap()[0] + 2;
	let max_y = input.iter().max_by(|x, y| x[1].cmp(&y[1])).unwrap()[1] + 2;

	println!("Part 1: {}", part_1(&input, max_x, max_y));
	println!("Part 2: {}", part_2(&input, max_x, max_y));
}

fn part_1(input: &Vec<Vec<i32>>, max_x: i32, max_y: i32) -> usize {
	let mut grid = vec![GridCell::Unvisited; (max_x * max_y) as usize]; // (coord_idx, distance)
	for j in 0..max_y {
		for i in 0..max_x {
			for (coord_idx, coord) in input.iter().enumerate() {
				let len = manhattan(coord[0], coord[1], i, j);
				let cell = &mut grid[(j * max_x + i) as usize];
				match cell {
					GridCell::Unvisited => *cell = GridCell::Visited(coord_idx, len),
					GridCell::Visited(c_idx, c_len) => if len < *c_len {
						*cell = GridCell::Visited(coord_idx, len);
					} else if len == *c_len && *c_idx != coord_idx {
						*cell = GridCell::VisitedTwice(len);
					}
					GridCell::VisitedTwice(c_len) => if len < *c_len {
						*cell = GridCell::Visited(coord_idx, len);
					},
				}
			}
		}
	}

	let mut excluded: HashSet<usize> = HashSet::with_capacity(input.len());
	for j in 0..max_y {
		for i in 0..max_x {
			if j == 0 || j == (max_y - 1) || i == 0 || i == (max_x - 1) {
				if let GridCell::Visited(c_idx, _) = grid[(j * max_x + i) as usize] {
					excluded.insert(c_idx);
				}
			}
		}
	}

	let valid_locations = grid.iter()
		.filter_map(|cell| match cell {
			GridCell::Unvisited => None,
			GridCell::Visited(c_idx, _) => if excluded.contains(&c_idx) { None } else { Some(*c_idx) },
			GridCell::VisitedTwice(_) => None,
		})
		.collect::<Vec<usize>>();

	let mut location_areas = HashMap::with_capacity(valid_locations.len());
	valid_locations.iter().for_each(|loc| *location_areas.entry(loc).or_insert(0) += 1);
	*location_areas.values().max().unwrap()
}

fn part_2(input: &Vec<Vec<i32>>, max_x: i32, max_y: i32) -> u32 {
	let mut area = 0;
	for j in 0..max_y {
		for i in 0..max_x {
			if input.iter().fold(0, |acc, x| acc + manhattan(i, j, x[0], x[1])) < 10_000 {
				area += 1;
			}
		}
	}
	area
}

#[derive(Debug, Clone, Copy)]
enum GridCell {
	Unvisited,
	Visited(usize, i32),
	VisitedTwice(i32),
}

fn manhattan(p1: i32, p2: i32, q1: i32, q2: i32) -> i32 {
	(p1 - q1).abs() + (p2 - q2).abs()
}
