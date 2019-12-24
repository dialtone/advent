use std::collections::{HashMap, HashSet};
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
    for minute in 0..10 {
        println!("Minute {}", minute);
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
                    continue; // skip the center for this
                }

                let inner_mask = get_inner_mask(i);
                let outer_mask = get_outer_mask(i);
                let inner_adjacents = inner & inner_mask;
                let outer_adjacents = outer & outer_mask;
                let adjacents = current & mask;
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
            print!("{}: {:b} ", level, tmp);
            tmp_world[level] = tmp;
        }
        println!();
        println!("-----------");
        world = tmp_world;
    }

    world.iter().map(|x| x.count_ones()).sum::<u32>()
}

fn get_outer_mask(pos: usize) -> u32 {
    if pos < 5 {
        // upper row, pos 8
        1 << 7
    } else if pos > 19 {
        // bottom row, pos 18
        1 << 17
    } else if pos + 1 % 5 == 0 {
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
        0b00001_00001_00001_00001_00001
    } else if pos == 17 {
        0b11111_00000_00000_00000_00000
    } else if pos == 11 {
        0b10000_10000_10000_10000_10000
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

// if the mask here touches the inner cell then we need to deal
// with the masks there, that is if mask & 1<<13 is > 0
// if it touches the outer cell we need to deal with it here
// that is either pos < 5 or pos > 19 or pos+1%5==0 or pos%5==0
// let inner_mask = if mask & 1 << 13 > 0 {
//     // adjaciency with internal levels which go to the right
//     get_inner_mask(i)
// } else {
//     0b0
// };
// let outer_mask = if i < 5 || i > 19 || i + 1 % 5 == 0 || i % 5 == 0 {
//     // adjanciency with external levels which go to the left
//     get_outer_mask(i)
// } else {
//     0b0
// };
