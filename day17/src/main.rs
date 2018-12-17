#![feature(uniform_paths)]

extern crate regex;

use regex::Regex;
use Orientation::{*};
use Ground::{*};

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let re = Regex::new(r"(.)=(\d+), (.)=(\d+)..(\d+)").unwrap();
	let mut clay_veins = Vec::with_capacity(input.lines().count());
	for cap in re.captures_iter(&input) {
		match &cap[1] {
			"x" => {
				clay_veins.push(ClayVein {
					orientation: Vertical,
					x: (&cap[2]).parse::<usize>().unwrap(),
					y: (&cap[4]).parse::<usize>().unwrap(),
					end: (&cap[5]).parse::<usize>().unwrap(),
				});
			}
			"y" => {
				clay_veins.push(ClayVein {
					orientation: Horizontal,
					x: (&cap[4]).parse::<usize>().unwrap(),
					y: (&cap[2]).parse::<usize>().unwrap(),
					end: (&cap[5]).parse::<usize>().unwrap(),
				});
			}
			_ => panic!(format!("invalid orientation: {}", &cap[1])),
		};
	}

	let min_y = clay_veins.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
	let max_y = clay_veins.iter().filter(|x| x.orientation == Vertical).map(|x| x.end).max().unwrap();
	let width = clay_veins.iter().filter(|x| x.orientation == Horizontal).map(|x| x.end).max().unwrap() + 10;
	let height = max_y + 1;

	let mut grid = vec![Sand; width * height];

	for vein in &clay_veins {
		match vein.orientation {
			Horizontal => (vein.x..=vein.end).for_each(|i| grid[vein.y * width + i] = Clay),
			Vertical => (vein.y..=vein.end).for_each(|j| grid[j * width + vein.x] = Clay),
		}
	}

	let mut points = Vec::new();
	points.push(Point { x: 500, y: 0 });
	grid[points[0].y * width + points[0].x] = WaterRunning;

	'l: while points.len() != 0 {
		for p_idx in 0..points.len() {
			let mut p = points[p_idx].clone();

			if grid[p.y * width + p.x] == WaterStill {
				p.y -= 1;
				points[p_idx] = p;
			} else if p.y == height - 1 || grid[p.y * width + p.x] != WaterRunning {
				points.remove(p_idx);
				continue 'l;
			} else if grid[(p.y + 1) * width + p.x] == Sand {
				while (p.y + 1) < height && grid[(p.y + 1) * width + p.x] == Sand {
					p.y += 1;
					grid[p.y * width + p.x] = WaterRunning;
				}
				points[p_idx] = p;
				if p.y != (height - 1) {
					if grid[(p.y + 1) * width + p.x] == WaterRunning {
						points.remove(p_idx);
						continue 'l;
					}
				} else {
					points.remove(p_idx);
					continue 'l;
				}
			} else {
				let org_x = p.x;
				let mut went_down = false;
				loop {
					if grid[(p.y + 1) * width + p.x] == Sand {
						went_down = true;
						points[p_idx] = p;
						break;
					}
					if grid[p.y * width + (p.x - 1)] == Clay {
						break;
					}
					if grid[(p.y + 1) * width + p.x] == WaterRunning {
						points.remove(p_idx);
						continue 'l;
					}
					p.x -= 1;
					grid[p.y * width + p.x] = WaterRunning;
				}

				if went_down {
					p.x += 1;
				}
				loop {
					if grid[(p.y + 1) * width + p.x] == Sand {
						if went_down {
							points.push(p);
						} else {
							points[p_idx] = p;
						}
						went_down = true;
						break;
					}
					if grid[p.y * width + (p.x + 1)] == Clay {
						break;
					}
					if grid[(p.y + 1) * width + p.x] == WaterRunning {
						points.remove(p_idx);
						continue 'l;
					}
					p.x += 1;
					grid[p.y * width + p.x] = WaterRunning;
				}

				if !went_down {
					loop {
						grid[p.y * width + p.x] = WaterStill;
						if grid[p.y * width + (p.x - 1)] == Clay {
							break;
						}
						p.x -= 1;
					}
					points[p_idx] = Point {
						x: org_x,
						y: p.y - 1,
					};
					grid[(p.y - 1) * width + org_x] = WaterRunning;
				}
			}
		}
	}

//	for j in 0..height {
//		for i in 0..width {
//			match grid[j * width + i] {
//				Sand => print!("."),
//				Clay => print!("#"),
//				WaterRunning => print!("|"),
//				WaterStill => print!("~"),
//			}
//		}
//		println!();
//	}

	let mut water = 0;
	let mut still_water = 0;
	for j in min_y..=max_y {
		for i in 0..width {
			match grid[j * width + i] {
				WaterStill => {
					still_water += 1;
					water += 1;
				}
				WaterRunning => water += 1,
				_ => (),
			}
		}
	}
	println!("Part 1: {}", water);
	println!("Part 2: {}", still_water);
}

#[derive(Copy, Clone)]
struct Point {
	x: usize,
	y: usize,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Ground {
	Sand,
	Clay,
	WaterRunning,
	WaterStill,
}

#[derive(Debug)]
struct ClayVein {
	orientation: Orientation,
	x: usize,
	y: usize,
	end: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
	Horizontal,
	Vertical,
}
