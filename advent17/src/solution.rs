use crate::intcode;
use std::char;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::thread::sleep;
use std::time::Duration;

type Point = (isize, isize);

fn print_map(maze: &HashMap<Point, char>) {
    go_to_top();
    let min_x = maze.keys().min_by_key(|(x, _)| *x).unwrap().0;
    let max_x = maze.keys().max_by_key(|(x, _)| *x).unwrap().0;

    let min_y = maze.keys().min_by_key(|(_, y)| *y).unwrap().1;
    let max_y = maze.keys().max_by_key(|(_, y)| *y).unwrap().1;
    for y in (min_y - 5)..max_y + 5 {
        let row = ((min_x - 5)..max_x + 5)
            .map(|x| *maze.get(&(x, y)).or(Some(&'▒')).unwrap())
            .collect::<Vec<char>>();
        println!("{}", String::from_iter(row));
    }
    sleep(Duration::from_millis(200));
}

fn get_starting_pos(map: &HashMap<Point, char>, drone_char: char) -> Point {
    for (pos, cell) in map {
        if *cell == drone_char {
            return *pos;
        }
    }
    (-1, -1)
}

pub fn part2(input: &str) -> isize {
    // get the maze
    // generate the full list of instructions to walk the maze
    //   effectively walk straight until you run out of scaffold, then turn
    // then find the longest (from 2 up to 20) patterns that repeat
    // non-overlapping in the string.
    // the first pattern starts from the beginning.
    // you can keep a vector/hashmap with the currently found patterns
    // you can also keep a vector with the sequence in which they appear in the
    // string, if a pattern is duplicate it doesn't matter as long as it goes
    // in the instruction string.
    // at the end you should have a hashmap with the 3 patterns as keys and name
    // as value (to be inverted).
    // a vector with the name of the 3 patterns repeated as they happen in the
    // string.
    // all good.
    // let map = get_map(input);
    // let mut rotation: isize = 0;
    // let mut current_direction = get_current_direction(rotation);
    // let mut pos = get_starting_pos(&map, current_direction);

    // //let mut visited: HashSet<Point> = HashSet::new();
    // let mut moves = vec![];

    // "R,4,R,10,R,8,R,4,R,10,R,6,R,4,R,4,R,10,R,8,R,4,R,10,R,6,R,4,R,4,L,12,R,6,L,12,R,10,R,6,R,4,R,4,L,12,R,6,L,12,R,4,R,10,R,8,R,4,R,10,R,6,R,4,R,4,L,12,R,6,L,12"

    // A => R,4,R,10,R,8,R,4,
    // B => R,10,R,6,R,4,
    // A => R,4,R,10,R,8,R,4,
    // B => R,10,R,6,R,4,
    // C => R,4,L,12,R,6,L,12,
    // B => R,10,R,6,R,4,
    // C => R,4,L,12,R,6,L,12,
    // A => R,4,R,10,R,8,R,4,
    // B => R,10,R,6,R,4,
    // C => R,4,L,12,R,6,L,12

    let mut computer = intcode::Computer::from(input);
    computer.store_mem(0, 2);
    let mut map = HashMap::new();
    let mut pos = (0, 0);

    // this was easy by hand... I'll do it proper another time, today I don't
    // have time.
    computer.write_string_input("A,B,A,B,C,B,C,A,B,C\n");
    computer.write_string_input("R,4,R,10,R,8,R,4\n");
    computer.write_string_input("R,10,R,6,R,4\n");
    computer.write_string_input("R,4,L,12,R,6,L,12\n");
    computer.write_string_input("n\n");
    let mut result = 0;
    while !computer.has_halted() {
        if computer.is_waiting() {
            break;
        }
        computer.run();
        //result = computer.pop_output().unwrap();
        let output = computer.get_output();
        result = output[output.len() - 1];
        for dot in output {
            match *dot {
                10 => {
                    pos = next_row(pos);
                    continue;
                }
                _ => {
                    map.insert(pos, char::from_u32(*dot as u32).unwrap());
                }
            }
            pos = next_col(pos);
        }
        print_map(&map);
    }

    // loop {
    //     let new_rot = get_rotation(&map, pos, rotation);
    //     rotation += new_rot;
    //     if new_rot == 0 {
    //         // end of the road, we're at the end.
    //         break;
    //     }
    //     if new_rot > 0 {
    //         moves.push("R".to_owned())
    //     } else {
    //         moves.push("L".to_owned())
    //     }

    //     current_direction = get_current_direction(rotation);
    //     let mut forwards = 0;
    //     // now we have rotated, time to find the end of the straight
    //     loop {
    //         let next_pos = next_cell_in_direction(pos, current_direction);
    //         if is_scaffolding(&map, next_pos) {
    //             forwards += 1;
    //             pos = next_pos;
    //         } else {
    //             break;
    //         }
    //     }
    //     moves.push(forwards.to_string());
    // }
    // println!("{}", moves.join(","));
    result
}

fn is_scaffolding(map: &HashMap<Point, char>, pos: Point) -> bool {
    if *map.get(&pos).or(Some(&'.')).unwrap() == '#' {
        return true;
    }
    false
}

fn get_current_direction(rotation: isize) -> char {
    let directions = vec!['^', '>', 'v', '<'];
    directions[rotation as usize % 4]
}

fn next_cell_in_direction(pos: Point, direction: char) -> Point {
    match direction {
        '^' => (pos.0, pos.1 - 1),
        '>' => (pos.0 + 1, pos.1),
        'v' => (pos.0, pos.1 + 1),
        '<' => (pos.0 - 1, pos.1),
        _ => {
            // messy
            println!("Can't figure out the direction: {}", direction);
            pos
        }
    }
}

fn get_rotation(map: &HashMap<Point, char>, pos: Point, rotation: isize) -> isize {
    // strictly call this function ONLY when at the end of a straight
    let right_cell = next_cell_in_direction(pos, get_current_direction(rotation + 1));
    let left_cell = next_cell_in_direction(pos, get_current_direction(rotation - 1));
    if *map.get(&right_cell).or(Some(&'.')).unwrap() == '#' {
        return 1;
    }
    if *map.get(&left_cell).or(Some(&'.')).unwrap() == '#' {
        return -1;
    }

    return 0;
}

fn get_map(input: &str) -> HashMap<Point, char> {
    let mut pos = (0, 0);
    let mut computer = intcode::Computer::from(input);
    let mut map = HashMap::new();

    while !computer.has_halted() {
        if computer.is_waiting() {
            break;
        }
        computer.run();
        for dot in computer.get_output() {
            match *dot {
                10 => {
                    pos = next_row(pos);
                    continue;
                }
                _ => {
                    map.insert(pos, char::from_u32(*dot as u32).unwrap());
                }
            }
            pos = next_col(pos);
        }
    }

    map
}

pub fn part1(input: &str) -> isize {
    clear_screen();
    let map = get_map(input);
    print_map(&map);
    let mut calibration = 0;
    for (pos, value) in &map {
        if *value != '#' {
            continue;
        }

        if is_intersection(&map, *pos) {
            calibration += pos.0 * pos.1;
        }
    }

    calibration
}

fn is_intersection(map: &HashMap<Point, char>, pos: Point) -> bool {
    if *map.get(&(pos.0, pos.1 + 1)).or(Some(&' ')).unwrap() == '#'
        && *map.get(&(pos.0, pos.1 - 1)).or(Some(&' ')).unwrap() == '#'
        && *map.get(&(pos.0 + 1, pos.1)).or(Some(&' ')).unwrap() == '#'
        && *map.get(&(pos.0 - 1, pos.1)).or(Some(&' ')).unwrap() == '#'
    {
        return true;
    }
    false
}

fn next_col(pos: Point) -> Point {
    (pos.0 + 1, pos.1)
}

fn next_row(pos: Point) -> Point {
    (0, pos.1 + 1)
}

fn go_to_top() {
    print!("{}[0;0H", 27 as char);
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
