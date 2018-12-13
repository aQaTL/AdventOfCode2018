fn main() {
    let mut input: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();

    let mut cursors = Vec::new();
    input.iter_mut().enumerate().for_each(|(y_idx, y)| {
        y.iter_mut().enumerate().for_each(|(x_idx, x)| {
            let direction;
            match x {
                '<' => {
                    *x = '-';
                    direction = 3;
                }
                '>' => {
                    *x = '-';
                    direction = 1;
                }
                'v' => {
                    *x = '|';
                    direction = 2;
                }
                '^' => {
                    *x = '|';
                    direction = 0;
                }
                _ => return,
            }
            cursors.push(Cursor {
                x: x_idx,
                y: y_idx,
                direction,
                turn: 0,
                crashed: false,
            });
        })
    });

    let mut part_1: Option<Cursor> = None;
    let mut cursors_remaining = cursors.len();
    while cursors_remaining > 1 {
        cursors.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        for i in 0..cursors.len() {
            let cursor = &mut cursors[i];
            if cursor.crashed {
                continue;
            }

            match cursor.direction {
                0 => cursor.y -= 1,
                1 => cursor.x += 1,
                2 => cursor.y += 1,
                3 => cursor.x -= 1,
                _ => panic!(),
            }
            match input[cursor.y][cursor.x] {
                '/' => {
                    cursor.direction = match cursor.direction {
                        0 => 1,
                        2 => 3,
                        3 => 2,
                        1 => 0,
                        _ => panic!(),
                    }
                }
                '\\' => {
                    cursor.direction = match cursor.direction {
                        1 => 2,
                        2 => 1,
                        0 => 3,
                        3 => 0,
                        _ => panic!(),
                    }
                }
                '+' => {
                    match cursor.turn {
                        0 => {
                            if cursor.direction == 0 {
                                cursor.direction = 3;
                            } else {
                                cursor.direction -= 1;
                            }
                        }
                        1 => (),
                        2 => cursor.direction = (cursor.direction + 1) % 4,
                        _ => panic!(),
                    }
                    cursor.turn = (cursor.turn + 1) % 3;
                }
                _ => (),
            }

            'find_crash: for i in 0..cursors.len() {
                for j in (i + 1)..cursors.len() {
                    if cursors[i].x == cursors[j].x
                        && cursors[i].y == cursors[j].y
                        && !cursors[i].crashed
                        && !cursors[j].crashed
                    {
                        if part_1.is_none() {
                            part_1 = Some(Cursor { ..cursors[i] });
                        }
                        cursors[i].crashed = true;
                        cursors[j].crashed = true;
                        cursors_remaining -= 2;
                        break 'find_crash;
                    }
                }
            }
        }
    }

    let part_1 = part_1.unwrap();
    println!("Part 1: {},{}", part_1.x, part_1.y);
    let part_2 = cursors.iter().find(|x| !x.crashed).unwrap();
    println!("Part 2: {},{}", part_2.x, part_2.y);
}

struct Cursor {
    x: usize,
    y: usize,
    direction: u8, // 0 - up, 1 - right, 2 - down, 3 - left
    turn: u8,      // 0 - turn left, 1 - go straight, 2 - turn right
    crashed: bool,
}