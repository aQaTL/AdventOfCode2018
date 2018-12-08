#![feature(nll)]

fn main() {
	let input: Vec<_> = std::fs::read_to_string("input.txt").unwrap()
		.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

	let (mut i, mut metadata_sum, mut last_node_value) = (0usize, 0u32, 0u32);
	let mut queue: Vec<Node> = Vec::with_capacity(input.len());
	push_node(&input, &mut i, &mut queue);
	while !queue.is_empty() {
		let tail = queue.last_mut().unwrap();
		if tail.children as usize == tail.child_values.len() {
			if tail.children == 0 {
				for i in i..i + tail.metadata as usize {
					metadata_sum += input[i];
					tail.value += input[i];
				}
			} else {
				for i in i..i + tail.metadata as usize {
					metadata_sum += input[i];
					tail.value += tail.child_values.get((input[i] - 1) as usize).unwrap_or(&0);
				}
			}
			i += tail.metadata as usize;
			last_node_value = tail.value;
			queue.pop();
			if let Some(last_last) = queue.last_mut() {
				(*last_last).child_values.push(last_node_value);
			}
		} else {
			push_node(&input, &mut i, &mut queue);
		}
	}
	println!("Part 1: {}", metadata_sum);
	println!("Part 2: {}", last_node_value);
}

fn push_node(input: &Vec<u32>, i: &mut usize, queue: &mut Vec<Node>) {
	let children = input[*i];
	let metadata = input[*i + 1];
	let n = Node {
		children,
		metadata,
		value: 0,
		child_values: Vec::with_capacity(children as usize),
	};
	queue.push(n);
	*i += 2;
}

struct Node {
	children: u32,
	metadata: u32,
	value: u32,
	child_values: Vec<u32>,
}
