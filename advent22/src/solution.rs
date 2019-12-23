use std::fmt::Display;

// part 1 I did, but eventually part 2 was just beyond my reach and I copied it.

#[derive(Debug)]
enum Technique {
    Increment(i128),
    NewStack,
    Cut(i128),
}

fn parse(input: &str) -> Vec<Technique> {
    let mut ts = vec![];
    for line in input.lines() {
        let mut parsed_line: Vec<&str> = line.rsplitn(2, ' ').collect();
        parsed_line.reverse();
        let t = match parsed_line[0] {
            "cut" => Technique::Cut(parsed_line[1].parse().unwrap()),
            "deal into new" => Technique::NewStack,
            "deal with increment" => Technique::Increment(parsed_line[1].parse().unwrap()),
            _ => continue,
        };
        ts.push(t);
    }
    ts
}

pub fn part1(input: &str) -> impl Display {
    let operations = parse(input);
    const LEN: i128 = 10007;
    operations.iter().fold(2019, |pos, cmd| match cmd {
        Technique::NewStack => LEN - 1 - pos,
        Technique::Cut(n) => (pos - n) % LEN,
        Technique::Increment(n) => (pos * n) % LEN,
    })
}

pub fn part2(input: &str) -> impl Display {
    const M: i128 = 119_315_717_514_047;
    const N: i128 = 101_741_582_076_661;

    // Convert the whole process to a linear equation: ax + b
    let (a, b) = parse(input).iter().rev().fold((1, 0), |(a, b), cmd| {
        let (a_new, b_new) = match cmd {
            Technique::NewStack => (-a, -b - 1),
            Technique::Cut(n) => (a, b + n),
            Technique::Increment(n) => {
                let n = mod_pow(*n, M - 2, M);
                (a * n, b * n)
            }
        };
        (a_new % M, b_new % M)
    });

    // Applying the function n times simplifies to:
    // x * a^n + b * (a^n - 1) / (a-1)
    let term1 = 2020 * mod_pow(a, N, M) % M;
    let tmp = (mod_pow(a, N, M) - 1) * mod_pow(a - 1, M - 2, M) % M;
    let term2 = b * tmp % M;
    (term1 + term2) % M
}

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}
