use crate::intcode;
use std::collections::HashMap;

pub fn part1(input: &str) -> isize {
    let mut network = HashMap::new();
    for address in 0..50 {
        network.insert(address, intcode::Computer::from(input));
    }
    for address in 0..50 {
        let computer = network.get_mut(&address).unwrap();
        computer.write_input(address);
    }
    let mut y_output = 0;
    while y_output == 0 {
        for address in 0..50 {
            let computer = network.get_mut(&address).unwrap();
            computer.run();
            if computer.is_waiting() {
                computer.write_input(-1);
                computer.run();
            }

            if computer.has_output() {
                computer.run();
                computer.run();
                let send_address = computer.pop_output().unwrap();
                let x = computer.pop_output().unwrap();
                let y = computer.pop_output().unwrap();
                if send_address == 255 {
                    y_output = y;
                    break;
                }

                let receiver = network.get_mut(&send_address).unwrap();
                receiver.write_input(x);
                receiver.write_input(y);
            }
        }
    }
    y_output
}

pub fn part2(input: &str) -> isize {
    let mut network = HashMap::new();
    for address in 0..50 {
        network.insert(address, intcode::Computer::from(input));
    }
    for address in 0..50 {
        let computer = network.get_mut(&address).unwrap();
        computer.write_input(address);
    }

    let mut nat = vec![];
    let mut ys = vec![];
    let mut loops = 0;
    loop {
        loops += 1;
        println!("loop {}", loops);
        let mut waiting = 0;
        for address in 0..50 {
            let computer = network.get_mut(&address).unwrap();
            computer.run();
            if computer.is_waiting() {
                waiting += 1;
                computer.write_input(-1);
                computer.run();
            }

            if computer.has_output() {
                waiting -= 1;
                computer.run();
                computer.run();
                let send_address = computer.pop_output().unwrap();
                let x = computer.pop_output().unwrap();
                let y = computer.pop_output().unwrap();
                print!(".");
                if send_address == 255 {
                    println!("Sent to NAT ({}, {})", x, y);
                    nat.push((x, y));
                    continue;
                }

                let receiver = network.get_mut(&send_address).unwrap();
                receiver.write_input(x);
                receiver.write_input(y);
            }
        }
        if waiting == 50 {
            // network is idle
            println!("network was idle");
            let (x, y) = nat.last().unwrap();
            let computer = network.get_mut(&0).unwrap();
            computer.write_input(*x);
            computer.write_input(*y);
            ys.push(*y);
            if ys.ends_with(&[*y, *y]) {
                break *y;
            }
        }
    }
}
