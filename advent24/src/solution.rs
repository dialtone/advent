use std::collections::HashSet;
use std::fmt::Display;

fn parse(input: &str) -> u32 {
    input
        .bytes()
        .filter(|&x| x != 10)
        .enumerate()
        .fold(0b0, |acc, (i, x)| {
            if x == b'#' {
                acc | 1 << i
            } else {
                acc | 0 << i
            }
        })
}

pub fn part1(input: &str) -> impl Display {
    let mut eris = parse(input);

    let mut masks = Vec::new();
    for pos in 0..25 {
        masks.push(get_adjacency_map(pos))
    }

    let mut done = HashSet::new();
    loop {
        let mut tmp = 0b0;
        for (i, mask) in masks.iter().enumerate() {
            let adjacents = eris & mask;
            let bugs_around = adjacents.count_ones();
            let has_bug = eris & (1 << i);
            if has_bug > 0 && bugs_around == 1 {
                tmp |= 1 << i;
            }
            if has_bug == 0 && (bugs_around == 1 || bugs_around == 2) {
                tmp |= 1 << i;
            }
        }
        eris = tmp;
        if done.contains(&eris) {
            break eris;
        } else {
            done.insert(eris);
        }
    }
}

fn print(name: &str, cell: u32) {
    println!("{}", name);
    for i in 0..25 {
        print!("{} ", cell >> i & 0b1);
        if (i + 1) % 5 == 0 {
            println!()
        }
    }
}

pub fn part2(input: &str) -> impl Display {
    let mut world: Vec<u32> = vec![0b0; 300];

    let mut same_level_masks = Vec::new();
    for pos in 0..25 {
        same_level_masks.push(get_adjacency_map(pos))
    }

    let level = 125; // assume this is level 0
    world[level] = parse(input); // set level 0 to the input

    // simulate 200 minutes
    let world_len = world.len() - 1;
    for minute in 1..201 {
        let mut tmp_world = vec![0b0; 300];
        for level in 0..world_len {
            let outer = if level > 0 { world[level - 1] } else { 0b0 };
            let inner = if level < world_len {
                world[level + 1]
            } else {
                0b0
            };
            let current = world[level];

            // no influece going on here.
            if outer == 0b0 && inner == 0b0 && current == 0b0 {
                continue;
            }

            let mut tmp = 0b0;
            for (i, mask) in same_level_masks.iter().enumerate() {
                if i == 12 {
                    continue; // skip the center spot
                }
                let inner_mask = get_inner_mask(i);
                let outer_mask = get_outer_mask(i);
                // this eliminates the center spot from bug counting
                // let current_mask = !(1<<13) & mask;
                let current_mask = 0b11111_11111_11011_11111_11111 & mask;
                let inner_adjacents = inner & inner_mask;
                let outer_adjacents = outer & outer_mask;
                let adjacents = current & current_mask;

                let bugs_around = adjacents.count_ones()
                    + inner_adjacents.count_ones()
                    + outer_adjacents.count_ones();
                let has_bug = current & (1 << i);
                if has_bug > 0 && bugs_around == 1 {
                    tmp |= 1 << i;
                }
                if has_bug == 0 && (bugs_around == 1 || bugs_around == 2) {
                    tmp |= 1 << i;
                }
            }
            // println!();
            // print(format!("Level {}", level).as_str(), tmp);
            tmp_world[level] = tmp;
        }
        // println!();
        // println!("-----------");
        world = tmp_world;
    }

    world.iter().map(|x| x.count_ones()).sum::<u32>()
}

fn get_outer_mask(pos: usize) -> u32 {
    if pos == 0 {
        let mut mask = 0b0;
        mask |= 1 << 7;
        mask |= 1 << 11;
        mask
    } else if pos == 4 {
        let mut mask = 0b0;
        mask |= 1 << 7;
        mask |= 1 << 13;
        mask
    } else if pos == 20 {
        let mut mask = 0b0;
        mask |= 1 << 11;
        mask |= 1 << 17;
        mask
    } else if pos == 24 {
        let mut mask = 0b0;
        mask |= 1 << 13;
        mask |= 1 << 17;
        mask
    } else if pos < 5 {
        // upper row, pos 8
        1 << 7
    } else if pos > 19 {
        // bottom row, pos 18
        1 << 17
    } else if (pos + 1) % 5 == 0 {
        // right row
        1 << 13
    } else if pos % 5 == 0 {
        // left row
        1 << 11
    } else {
        0b0
    }
}

fn get_inner_mask(pos: usize) -> u32 {
    if pos == 7 {
        0b11111
    } else if pos == 13 {
        0b10000_10000_10000_10000_10000
    } else if pos == 17 {
        0b11111_00000_00000_00000_00000
    } else if pos == 11 {
        0b00001_00001_00001_00001_00001
    } else {
        0b0
    }
}

fn get_adjacency_map(pos: usize) -> u32 {
    let mut mask = 0b0;
    if pos > 4 {
        // second row or more, then check above
        mask |= 1 << (pos - 5);
    }
    if pos < 20 {
        // not last row, then check below
        mask |= 1 << (pos + 5);
    }
    if pos % 5 != 0 {
        // check left
        mask |= 1 << (pos - 1);
    }

    if (pos + 1) % 5 != 0 {
        // check right
        mask |= 1 << (pos + 1);
    }
    mask
}
