extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

use std::collections::HashMap;
use std::str::FromStr;

fn main() {
	let table = parse_input();
	println!("Part 1: {}", part_1(&table));
	println!("Part 2: {}", part_2(&table));
}

fn parse_input() -> HashMap<u32, Vec<u32>> {
	let mut logs = std::fs::read_to_string("input.txt").expect("input.txt not found")
		.lines().filter_map(|x| x.parse::<Log>().ok()).collect::<Vec<_>>();
	logs.sort_by(|x, y| x.year.cmp(&y.year)
		.then(x.month.cmp(&y.month))
		.then(x.day.cmp(&y.day))
		.then(x.hour.cmp(&y.hour))
		.then(x.minute.cmp(&y.minute)));

	let mut table: HashMap<u32, Vec<u32>> = HashMap::new(); //<id, [hour_table_of_sleeps]>
	let mut curr_id = 0u32;
	let mut went_sleep = 0u32;
	for log in logs {
		match log.action {
			Action::BeginShift(id) => {
				curr_id = id;
				table.entry(id).or_insert(vec![0u32; 60]);
			}
			Action::FallsAsleep => went_sleep = log.minute,
			Action::WakesUp => {
				(went_sleep..log.minute).for_each(|i|
					table.get_mut(&curr_id).unwrap()[i as usize] += 1);
			}
		}
	}
	table
}

fn part_1(table: &HashMap<u32, Vec<u32>>) -> u32 {
	let laziest_guard_id = table.iter()
		.map(|(k, v)| (k, v.iter().sum::<u32>()))
		.max_by(|x, y| x.1.cmp(&y.1))
		.unwrap().0;
	let most_freq_min = table.get(laziest_guard_id).unwrap().iter().enumerate()
		.max_by(|x, y| x.1.cmp(&y.1))
		.unwrap().0;
	laziest_guard_id * most_freq_min as u32
}

fn part_2(table: &HashMap<u32, Vec<u32>>) -> u32 {
	let (mut minute, mut guard_id, mut most_freq_minute) = (0, 0, 0);
	for (k, v) in table {
		let (day, most_freq_minute_in_day) = v.iter().enumerate()
			.max_by(|x, y| x.1.cmp(&y.1))
			.unwrap();
		if most_freq_minute_in_day > &most_freq_minute {
			most_freq_minute = *most_freq_minute_in_day;
			guard_id = *k;
			minute = day;
		}
	}
	minute as u32 * guard_id
}

struct Log {
	year: u32,
	month: u32,
	day: u32,
	hour: u32,
	minute: u32,
	action: Action,
}

enum Action {
	BeginShift(u32), //guard id
	FallsAsleep,
	WakesUp,
}

impl FromStr for Log {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"^\[(\d+)-(\d+)-(\d+)\s(\d+):(\d+)\]\s(.+)$").unwrap();
		}
		let caps = RE.captures(s).unwrap();

		let words = &caps[6].split(" ").collect::<Vec<_>>();
		let action = match words[0] {
			"Guard" => {
				Action::BeginShift(words[1][1..].parse::<u32>().unwrap())
			}
			"falls" => Action::FallsAsleep,
			"wakes" => Action::WakesUp,
			_ => panic!("action not found"),
		};

		Ok(Log {
			year: caps[1].parse::<u32>().unwrap(),
			month: caps[2].parse::<u32>().unwrap(),
			day: caps[3].parse::<u32>().unwrap(),
			hour: caps[4].parse::<u32>().unwrap(),
			minute: caps[5].parse::<u32>().unwrap(),
			action,
		})
	}
}
