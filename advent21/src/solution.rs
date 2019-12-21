use crate::intcode;
use std::char;
use std::iter::FromIterator;

pub fn part1(input: &str) -> isize {
    let jumpscript = vec![
        "NOT C T\n", // hole 3 in front
        "AND D T\n", // no hole 4 and hole 3
        "NOT A J\n", //
        "OR T J\n",  // or hole in front
        "WALK\n",    // this basically means that any time there's a hole and it
                     // would land me on a tile, do it and check if you need to
                     // jump again right after immediately.
    ];
    run_computer(input, &jumpscript)
}

pub fn part2(input: &str) -> isize {
    let jumpscript = vec![
        "NOT C T\n", // hole 3 in front
        "AND D T\n", // full in 4
        "AND H T\n", // full in 8
        // OR
        "NOT A J\n", // hole in front
        "AND D J\n", // full in 4
        "OR T J\n",  // jump if there's a hole in 3 and landing spot in 4-8 or hole in front
        // OR
        "NOT B T\n", // no hole in 2
        "AND D T\n", // and landing in 4
        "OR T J\n",  // and even now is good
        "RUN\n",
    ];
    run_computer(input, &jumpscript)
}

fn run_computer(input: &str, jumpscript: &[&str]) -> isize {
    let mut computer = intcode::Computer::from(input);
    for line in jumpscript {
        computer.write_string_input(line);
    }
    computer.run();
    let mut output = computer.get_output().clone();
    let damage = output[output.len() - 1];
    println!(
        "{}",
        String::from_iter(
            output
                .drain(0..(output.len() - 1))
                .map(|x| char::from_u32(x as u32).unwrap())
                .collect::<Vec<_>>()
        )
    );
    damage
}

// fn go_to_top() {
//     print!("{}[0;0H", 27 as char);
// }

// fn clear_screen() {
//     print!("{}[2J", 27 as char);
// }
