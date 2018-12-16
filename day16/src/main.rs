fn main() {
	let input = std::fs::read_to_string("input1.txt").unwrap();
	let input: Vec<&str> = input.lines().collect();

	let ops = [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];
	let mut part_1 = 0;
	for i in (0..input.len()).step_by(4) {
		let before = input[i][9..19].split(", ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
		let instr = input[i + 1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
		let after = input[i + 2][9..19].split(", ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

		let mut results = 0;
		for op in &ops {
			let mut before_copy = before.clone();
			op(&instr, &mut before_copy);
			if before_copy == after {
				results += 1;
			}
		}
		if results >= 3 {
			part_1 += 1;
		}
	}
	println!("Part 1: {}", part_1);

	let input = std::fs::read_to_string("input2.txt").unwrap()
		.lines()
		.map(|x| x.split(" ")
			.map(|x| x.parse::<usize>().unwrap())
			.collect::<Vec<_>>())
		.collect::<Vec<Vec<usize>>>();

	/*
		banr 5
		bani 1

		setr 8
		seti 2

		gtir 12
		gtrr 13
		gtri 14

		eqrr 10
		eqir 4
		eqri 0

		borr 6
		bori 3
		addi 11
		mulri 7

		mulr 15
		addr 9
	*/
	let mut mem = vec![0usize; 4];
	for i in input {
		match i[0] {
			0 => eqri(&i, &mut mem),
			1 => bani(&i, &mut mem),
			2 => seti(&i, &mut mem),
			3 => bori(&i, &mut mem),
			4 => eqir(&i, &mut mem),
			5 => banr(&i, &mut mem),
			6 => borr(&i, &mut mem),
			7 => muli(&i, &mut mem),
			8 => setr(&i, &mut mem),
			9 => addr(&i, &mut mem),
			10 => eqrr(&i, &mut mem),
			11 => addi(&i, &mut mem),
			12 => gtir(&i, &mut mem),
			13 => gtrr(&i, &mut mem),
			14 => gtri(&i, &mut mem),
			15 => mulr(&i, &mut mem),
			_ => panic!("unknown opcode"),
		}
	}
	println!("Part 2: {}", mem[0]);
}

fn addr(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]] + mem[instr[2]];
}

fn addi(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]] + instr[2];
}

fn mulr(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]] * mem[instr[2]];
}

fn muli(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]] * instr[2];
}

fn banr(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]] & mem[instr[2]];
}

fn bani(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]] & instr[2];
}

fn borr(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]] | mem[instr[2]];
}

fn bori(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]] | instr[2];
}

fn setr(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = mem[instr[1]];
}

fn seti(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = instr[1];
}

fn gtir(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = if instr[1] > mem[instr[2]] { 1 } else { 0 }
}

fn gtri(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = if mem[instr[1]] > instr[2] { 1 } else { 0 }
}

fn gtrr(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = if mem[instr[1]] > mem[instr[2]] { 1 } else { 0 }
}

fn eqir(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = if instr[1] == mem[instr[2]] { 1 } else { 0 }
}

fn eqri(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = if mem[instr[1]] == instr[2] { 1 } else { 0 }
}

fn eqrr(instr: &Vec<usize>, mem: &mut Vec<usize>) {
	mem[instr[3]] = if mem[instr[1]] == mem[instr[2]] { 1 } else { 0 }
}
