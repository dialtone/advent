use num::range_step;
use std::fmt::Display;
use std::iter;
use std::iter::FromIterator;
//use prefix_sum::PrefixSum;

const PHASES: usize = 100;

fn parse(input: &str) -> Vec<isize> {
    input
        .trim()
        .bytes()
        .map(|digit| (digit - b'0') as isize)
        .collect()
}

pub fn part1(input: &str) -> impl Display {
    let mut input_list = parse(input);
    for i in 0..PHASES {
        for number_idx in 0..input_list.len() {
            let repeats = number_idx + 1;
            input_list[number_idx] = multiply(&input_list, repeats);
        }
    }
    String::from_iter(input_list[0..8].iter().map(|&x| x.to_string()))
}

fn multiply(number: &Vec<isize>, repeats: usize) -> isize {
    let pattern = iter::repeat(0)
        .take(repeats)
        .chain(iter::repeat(1).take(repeats))
        .chain(iter::repeat(0).take(repeats))
        .chain(iter::repeat(-1).take(repeats))
        .cycle()
        .skip(1);
    let mut result = 0;

    for (i, number) in pattern.zip(number) {
        result += i * number;
    }
    (result % 10).abs()
}

pub fn part2(input: &str) -> impl Display {
    let input_list = iter::repeat(parse(input))
        .take(10000)
        .flatten()
        .collect::<Vec<isize>>();

    let offset = String::from_iter(input_list[0..7].iter().map(|&x| x.to_string()))
        .parse::<usize>()
        .unwrap();

    let mut actual_start = input_list
        .iter()
        .skip(offset)
        .map(|&x| x)
        .collect::<Vec<isize>>();

    for _ in 0..PHASES {
        let mut running_sum = 0;
        for number_idx in range_step((actual_start.len() - 1) as isize, -1 as isize, -1) {
            running_sum += actual_start[number_idx as usize];
            actual_start[number_idx as usize] = running_sum % 10;
        }
    }

    String::from_iter(actual_start[0..8].iter().map(|&x| x.to_string()))
}
