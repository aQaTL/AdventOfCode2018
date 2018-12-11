fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap().parse::<usize>().unwrap();

	const GRID_SIDE: usize = 300;
	let mut grid = vec![0i32; GRID_SIDE.pow(2)];
	for j in 0..GRID_SIDE {
		for i in 0..GRID_SIDE {
			grid[j * GRID_SIDE + i] = (((((i + 1 + 10) * (j + 1) + input) * (i + 1 + 10)) as f64 / 100.0) as i32 % 10) - 5;
		}
	}

	let part_1 = solve(&grid, GRID_SIDE, 3..=3);
	println!("Part 1: {},{},{}", part_1.1, part_1.2, part_1.3);
	let part_2 = solve(&grid, GRID_SIDE, 1..=300);
	println!("Part 2: {},{},{}", part_2.1, part_2.2, part_2.3);
}

fn solve<T>(grid: &Vec<i32>, grid_side: usize, square_sizes: T) -> (i32, usize, usize, usize)
	where T: Iterator<Item=usize>
{
	let mut square: (i32, usize, usize, usize) = (0, 0, 0, 0);
	for square_size in square_sizes {
		for j in 0..grid_side - (square_size - 1) {
			for i in 0..grid_side - (square_size - 1) {
				let mut square_value = 0;
				for jj in 0..square_size {
					for ii in 0..square_size {
						square_value += grid[(j + jj) * grid_side + (i + ii)];
					}
				}
				if square_value > square.0 {
					square = (square_value, i + 1, j + 1, square_size);
				}
			}
		}
	}
	square
}