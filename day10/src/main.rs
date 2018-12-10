extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

fn main() {
	let mut input: Vec<Point> = std::fs::read_to_string("input.txt").unwrap().lines()
		.map(parse_str).collect();
	let mut i = 0;
	while input.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y -
		input.iter().min_by(|x, y| x.y.cmp(&y.y)).unwrap().y != 9 {
		input.iter_mut().for_each(|p| {
			p.x += p.delta_x;
			p.y += p.delta_y;
		});
		i += 1;
	}

	let max_x = input.iter().max_by(|x, y| x.x.cmp(&y.x)).unwrap().x;
	let min_x = input.iter().min_by(|x, y| x.x.cmp(&y.x)).unwrap().x;
	let max_y = input.iter().max_by(|x, y| x.y.cmp(&y.y)).unwrap().y;
	let min_y = input.iter().min_by(|x, y| x.y.cmp(&y.y)).unwrap().y;

	let (width, height) = (max_x + 1 - min_x, max_y + 1 - min_y);
	let mut grid = vec![' '; (width * height) as usize];
	input.iter().for_each(|p| grid[((p.y - min_y) * width + (p.x - min_x)) as usize] = '\u{2588}');

	println!("Part 1:");
	for j in 0..height {
		for i in 0..width {
			print!("{}", grid[(j * width + i) as usize]);
		}
		println!();
	}
	println!("Part 2: {}", i);
}

struct Point {
	x: i32,
	y: i32,
	delta_x: i32,
	delta_y: i32,
}

fn parse_str(s: &str) -> Point {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"position=<\s?(.+), \s?(.+)> velocity=<\s?(.+), \s?(.+)>")
			.unwrap();
	}
	let caps = RE.captures(s).unwrap();
	Point {
		x: caps[1].parse::<i32>().unwrap(),
		y: caps[2].parse::<i32>().unwrap(),
		delta_x: caps[3].parse::<i32>().unwrap(),
		delta_y: caps[4].parse::<i32>().unwrap(),
	}
}
