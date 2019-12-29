use crate::intcode;
use std::char;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self, BufRead};

pub fn part1(input: &str) -> isize {
    let mut computer = intcode::Computer::from(input);

    // - ornament
    // - astrolabe
    // - sand
    // - shell

    //                             astrolabe (supply measurements)
    //                             |
    //                             Pod
    //                             |
    //                             Sand->no power->mutex
    //                             |
    //                             |
    //                             |   observatory(shell)
    //                             |     |
    //                             S->holodeck
    //                                 |
    // engineering(photons no) <-sick bay (giant electromagnet) -> hot chocolate fountain (ornament) -> gift
    //                                                                                              wrapping
    //                                                                                                 |
    //                                                                                              security

    let stdin = io::stdin();
    while !computer.has_halted() {
        if computer.has_output() {
            print!(
                "{}",
                char::from_u32(computer.pop_output().unwrap() as u32).unwrap()
            );
        }
        if computer.is_waiting() {
            // write input
            let mut cmd = String::new();
            stdin.lock().read_line(&mut cmd).unwrap();
            computer.write_string_input(&cmd);
        }
        computer.run();
    }

    0
}

pub fn part2(input: &str) -> isize {
    let mut computer = intcode::Computer::from(input);
    0
}
