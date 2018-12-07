use std::collections::{HashMap, HashSet};

fn main() {
	let steps = std::fs::read_to_string("input.txt").unwrap()
		.lines()
		.map(str::as_bytes)
		.map(|b| (b[5], b[36]))
		.collect::<Vec<_>>();

	let mut graph: HashMap<u8, Vec<u8>> = HashMap::new();
	steps.iter().for_each(|(step_a, step_b)| (*graph.entry(*step_a).or_insert(Vec::new())).push(*step_b));
	let mut rev_graph: HashMap<u8, Vec<u8>> = HashMap::new();
	steps.iter().for_each(|(step_a, step_b)| (*rev_graph.entry(*step_b).or_insert(Vec::new())).push(*step_a));

	let root = steps.iter().filter(|(x, _)| !steps.iter().any(|(_, y2)| x == y2)).map(|(x, _)| *x).collect::<HashSet<u8>>();

	println!("Part 1: {}", part_1(&graph, &rev_graph, &root));
	println!("Part 2: {}", part_2(&graph, &rev_graph, &root));
}

fn part_1(graph: &HashMap<u8, Vec<u8>>, rev_graph: &HashMap<u8, Vec<u8>>, root: &HashSet<u8>) -> String {
	let mut search_queue = root.iter().map(|x| *x).collect::<Vec<u8>>();
	let mut result: Vec<u8> = Vec::new();

	while result.len() != graph.len() + 1 {
		search_queue.sort_by(|x, y| y.cmp(&x));
		if let Some(last) = search_queue.pop() {
			result.push(last);
			if let Some(nodes) = graph.get(&last) {
				nodes.iter()
					.filter(|node| !search_queue.contains(node) &&
						rev_graph[node].iter().all(|step_before| result.contains(step_before)))
					.collect::<Vec<_>>().iter()
					.for_each(|&&node| search_queue.push(node));
			}
		}
	}
	result.iter().map(|x| *x as char).collect::<String>()
}

fn part_2(graph: &HashMap<u8, Vec<u8>>, rev_graph: &HashMap<u8, Vec<u8>>, root: &HashSet<u8>) -> u32 {
	let mut clock = 0u32;
	let (mut workers, mut workers_working) = ([(0u8, 0u32); 5], 0);

	let mut search_queue = root.iter().map(|x| *x).collect::<Vec<u8>>();
	let mut result: Vec<u8> = Vec::new();

	while result.len() != graph.len() + 1 {
		if workers_working < workers.len() {
			search_queue.sort_by(|x, y| y.cmp(&x));
			while workers_working != workers.len() && search_queue.len() != 0 {
				if let Some(last) = search_queue.pop() {
					*workers.iter_mut().find(|x| x.0 == 0).unwrap() = (last, last as u32 - 4);
					workers_working += 1;
				}
			}
		}
		for worker in workers.iter_mut().filter(|worker| worker.0 != 0) {
			(*worker).1 -= 1;
			if worker.1 == 0 {
				result.push(worker.0);
				if let Some(nodes) = graph.get(&worker.0) {
					nodes.iter()
						.filter(|node| !search_queue.contains(node) &&
							rev_graph[node].iter().all(|step_before| result.contains(step_before)))
						.collect::<Vec<_>>().iter()
						.for_each(|&&node| search_queue.push(node));
				}
				(*worker).0 = 0;
				workers_working -= 1;
			}
		}
		clock += 1;
	}
	clock
}