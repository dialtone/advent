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

pub fn part2(input: &str) -> impl Display {
    0
}

fn get_adjacency_map(pos: usize) -> u32 {
    let mut mask = 0b0;
    // map[pos] = 1;
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
