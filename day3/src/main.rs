extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use regex::Regex;

fn main() {
	let claims = std::fs::read_to_string("input.txt").expect("input.txt not found")
		.lines().filter_map(|x| x.parse::<Claim>().ok()).collect::<Vec<Claim>>();

	let mut grid: Vec<u32> = Vec::with_capacity(1000 * 1000);
	(0..1000 * 1000).for_each(|_| grid.push(0));

	for claim in claims.iter() {
		for j in 0..claim.height {
			for i in 0..claim.width {
				grid[(claim.from_top + j) * 1000 + (claim.from_left + i)] += 1;
			}

		}
	}

	println!("{}", grid.iter().filter(|&&x| x > 1).count());

	'l: for (idx, claim) in claims.iter().enumerate() {
		for j in 0..claim.height {
			for i in 0..claim.width {
				if grid[(claim.from_top + j) * 1000 + (claim.from_left + i)] != 1 {
					continue 'l;
				}
			}
		}
		println!("{}", idx+1);
		break 'l;
	}
}

#[derive(Debug)]
struct Claim {
	from_left: usize,
	from_top: usize,
	width: usize,
	height: usize,
}

impl FromStr for Claim {
	type Err = (String);

	fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
		lazy_static! {
	        static ref RE: Regex = Regex::new(r"^#\d+\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();
	    }
		let caps = RE.captures(s).unwrap();

		Ok(Claim {
			from_left: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
			from_top: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
			width: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
			height: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
		})
	}
}
