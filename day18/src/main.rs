#![feature(uniform_paths)]

use std::collections::HashMap;
use Acre::{*};

fn main() {
	let mut grid = std::fs::read_to_string("input.txt").unwrap().lines().map(|x| x.chars().map(|x| {
		match x {
			'.' => Open,
			'|' => Trees,
			'#' => Lumberyard,
			_ => panic!(format!("Unsupported symbol: {}", x))
		}
	}).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

	let mut hm = HashMap::new();
	let mut seen = Vec::new();
	let mut fin_i = 0;
	let mut p1 = Vec::new();
	for i in 1..=1_000_000_000 {
		let mut new_gen = grid.clone();
		for row_idx in 0..50 {
			for col_idx in 0..50 {
				let (trees, lumberyards) = count_adjacent_acres(&grid, row_idx, col_idx);
				match grid[row_idx][col_idx] {
					Open => if trees >= 3 { new_gen[row_idx][col_idx] = Trees; },
					Trees => if lumberyards >= 3 { new_gen[row_idx][col_idx] = Lumberyard; },
					Lumberyard => if lumberyards < 1 || trees < 1 { new_gen[row_idx][col_idx] = Open; },
				}
			}
		}
		grid = new_gen;

		let entry = hm.entry(grid.clone()).or_insert(0);
		*entry += 1;
		if *entry == 2 {
			seen.push(grid.clone());
		} else if *entry == 3 {
			fin_i = i;
			break;
		}

		if i == 10 {
			p1 = grid.clone();
		}
	}

	let calc_resource_value = |grid: &Vec<Vec<Acre>>|
		grid.iter().map(|row| row.iter().filter(|&&acre| acre == Trees).count()).sum::<usize>() *
			grid.iter().map(|row| row.iter().filter(|&&acre| acre == Lumberyard).count()).sum::<usize>();

	println!("Part 1: {}", calc_resource_value(&p1));
	println!("Part 2: {}", calc_resource_value(&seen[(1_000_000_000 - fin_i) % seen.len()]));
}

fn count_adjacent_acres(grid: &Vec<Vec<Acre>>, row_idx: usize, col_idx: usize) -> (u32, u32) {
	let (mut trees, mut lumberyards) = (0, 0);
	for r in ((row_idx as i64) - 1)..=(row_idx as i64 + 1) {
		if r < 0 {
			continue;
		}
		for c in ((col_idx as i64) - 1)..=(col_idx as i64 + 1) {
			if c < 0 || (col_idx as i64 == c && row_idx as i64 == r) {
				continue;
			}
			if let Some(r) = grid.get(r as usize) {
				if let Some(acre) = r.get(c as usize) {
					match acre {
						Open => (),
						Trees => trees += 1,
						Lumberyard => lumberyards += 1,
					}
				}
			}
		}
	}
	(trees, lumberyards)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Acre {
	Open,
	Trees,
	Lumberyard,
}
