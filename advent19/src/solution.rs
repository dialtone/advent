use crate::intcode;
use std::char;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::thread::sleep;
use std::time::Duration;

type Point = (isize, isize);

fn print_map(map: &HashMap<Point, char>) {
    go_to_top();
    let min_x = map.keys().min_by_key(|(x, _)| *x).unwrap().0;
    let max_x = map.keys().max_by_key(|(x, _)| *x).unwrap().0;

    let min_y = map.keys().min_by_key(|(_, y)| *y).unwrap().1;
    let max_y = map.keys().max_by_key(|(_, y)| *y).unwrap().1;
    for y in (min_y - 5)..max_y + 5 {
        let row = ((min_x - 5)..max_x + 5)
            .map(|x| *map.get(&(x, y)).or(Some(&'▒')).unwrap())
            .collect::<Vec<char>>();
        println!("{}", String::from_iter(row));
    }
    sleep(Duration::from_millis(200));
}

pub fn part2(input: &str) -> isize {
    clear_screen();
    let mut map = HashMap::new();
    let start_x = 1523;
    let start_y = 1022;
    for x in start_x..start_x + 100 {
        for y in start_y..start_y + 100 {
            let mut computer = intcode::Computer::from(input);
            computer.write_input(x);
            computer.write_input(y);
            computer.run();
            let dot = match computer.pop_output().unwrap() {
                0 => '.',
                _ => '#',
            };
            map.insert((x, y), dot);
        }
    }
    start_x * 10000 + start_y
}

pub fn part1(input: &str) -> usize {
    //clear_screen();
    let mut map = HashMap::new();
    for x in 0..50 {
        for y in 0..50 {
            let mut computer = intcode::Computer::from(input);
            computer.write_input(x);
            computer.write_input(y);
            computer.run();
            let dot = match computer.pop_output().unwrap() {
                0 => '.',
                _ => '#',
            };
            map.insert((x, y), dot);
        }
    }
    //print_map(&map);
    map.iter().filter(|(_, &v)| v == '#').count()
}

fn go_to_top() {
    print!("{}[0;0H", 27 as char);
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
