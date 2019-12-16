use std::fmt::Display;
use std::iter;
use std::iter::FromIterator;

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
    let offset = input[..7].parse::<usize>().unwrap();

    let mut input_list = iter::repeat(parse(input))
        .take(10000)
        .flatten()
        .skip(offset as usize)
        .collect::<Vec<isize>>();

    for _ in 0..PHASES {
        let mut running_sum = 0;
        for number_idx in (0..input_list.len() - 1).rev() {
            running_sum += input_list[number_idx as usize];
            input_list[number_idx as usize] = running_sum % 10;
        }
    }

    input_list[..8].iter().fold(0isize, |a, &n| a * 10 + n)
}
