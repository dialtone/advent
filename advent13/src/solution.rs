include!("intcode.rs");

use std::collections::HashMap;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

pub fn part1(input: &str) -> impl Display {
    let mut board = HashMap::new();

    let mut computer = Computer::from(input);

    while !computer.has_halted() {
        if computer.is_waiting() {
            computer.resume();
        } else {
            computer.run();
        }
    }
    loop {
        let x_out = computer.pop_output();
        let x;
        match x_out {
            Some(value) => x = value,
            _ => break, // output finished
        }
        let y = computer.pop_output().unwrap();
        let tile_id = computer.pop_output().unwrap();
        board.insert((x, y), tile_id);
    }
    board.values().filter(|&&c| c == 2).count()
}

fn board_size(input: &str) -> (usize, usize) {
    let mut computer = Computer::from(input);
    let mut max_x = 0;
    let mut max_y = 0;
    while !computer.has_halted() {
        if computer.is_waiting() {
            computer.resume();
        } else {
            computer.run();
        }
    }

    loop {
        let x_out = computer.pop_output();
        let x;
        match x_out {
            Some(value) => x = value,
            _ => break, // output finished
        }
        let y = computer.pop_output().unwrap();
        let _ = computer.pop_output().unwrap();
        if x > max_x {
            max_x = x
        }
        if y > max_y {
            max_y = y
        }
    }
    max_x += 1;
    max_y += 1;
    (max_x as usize, max_y as usize)
}

pub fn part2(input: &str) -> impl Display {
    //clear_screen();
    //let (max_x, max_y) = board_size(input);

    let mut computer = Computer::from(input);
    computer.store_mem(0, 2);

    //let mut board = vec![vec![0; max_x]; max_y];
    let mut joystick_position = 0;
    let mut current_ball_position = (0, 0);
    let mut current_paddle_position = (0, 0);
    let mut score = 0;
    while !computer.has_halted() {
        if computer.is_waiting() {
            computer.write_input(joystick_position);
            computer.resume();
        } else {
            computer.run();
        }
        loop {
            let x_out = computer.pop_output();
            let x;
            match x_out {
                Some(value) => x = value,
                _ => break, // output finished
            }
            let y = computer.pop_output().unwrap();
            let tile_id = computer.pop_output().unwrap();
            if (x as isize, y as isize) == (-1, 0) {
                score = tile_id;
                continue;
            }

            if tile_id == 4 {
                current_ball_position = (x, y);
            }
            if tile_id == 3 {
                current_paddle_position = (x, y);
            }
            // initial value is just placeholder
            if current_ball_position.0 > current_paddle_position.0 {
                joystick_position = 1;
            } else if current_ball_position.0 == current_paddle_position.0 {
                joystick_position = 0;
            } else {
                joystick_position = -1
            }
            //board[y as usize][x as usize] = tile_id;
        }
        // go_to_top();
        // println!("score {} input {}     ", score, joystick_position);
        // print_board(&board);
        // sleep(Duration::from_millis(25));
    }
    score
}

fn tile_to_string(c: &isize) -> String {
    match c {
        0 => " ".to_owned(),
        1 => "█".to_owned(),
        2 => "▒".to_owned(),
        3 => "▬".to_owned(),
        4 => "●".to_owned(),
        _ => "".to_owned(),
    }
}

fn go_to_top() {
    print!("{}[0;0H", 27 as char);
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn print_board(board: &Vec<Vec<isize>>) {
    for row in board {
        println!(
            "{}",
            row.iter()
                .map(tile_to_string)
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
