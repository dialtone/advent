use std::char;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::iter::FromIterator;
use std::thread::sleep;
use std::time::Duration;

type Point = (isize, isize);

fn parse(input: &str) -> HashMap<Point, char> {
    let mut pos = (0, 0);
    let mut maze = HashMap::new();

    for line in input.lines() {
        for dot in line.chars() {
            match dot {
                ' ' => maze.insert(pos, '#'),
                _ => maze.insert(pos, dot),
            };
            pos = next_col(pos);
        }
        pos = next_row(pos);
    }
    maze
}

fn edges(maze: &HashMap<Point, char>) -> (Point, Point) {
    let min_x = maze.keys().min_by_key(|(x, _)| *x).unwrap().0;
    let max_x = maze.keys().max_by_key(|(x, _)| *x).unwrap().0;
    let min_y = maze.keys().min_by_key(|(_, y)| *y).unwrap().1;
    let max_y = maze.keys().max_by_key(|(_, y)| *y).unwrap().1;

    ((min_x, min_y), (max_x, max_y))
}

fn print(maze: &HashMap<Point, char>) {
    go_to_top();

    let ((min_x, min_y), (max_x, max_y)) = edges(maze);

    for y in (min_y)..=max_y {
        let row = ((min_x)..=max_x)
            .map(|x| *maze.get(&(x, y)).or(Some(&'#')).unwrap())
            .collect::<Vec<char>>();
        println!("{}", String::from_iter(row));
    }
    sleep(Duration::from_millis(1000));
}

fn get_start_stop_and_portals(
    maze: &HashMap<Point, char>,
) -> (Point, Point, HashMap<Point, (Point, isize)>) {
    // new: isize
    let ((min_x, min_y), (max_x, max_y)) = edges(&maze);

    let mut tmp = HashMap::new();
    let mut portals = HashMap::new();

    for col in min_y..=max_y {
        for row in min_x..=max_x {
            match *maze.get(&(row, col)).or(Some(&' ')).unwrap() {
                first_letter @ 'A'..='Z' =>
                // maybe found a portal at this point it's either vertical or
                // horizontal and the location depends on where the '.' tile is
                // (before or after).
                {
                    let portal = match *maze.get(&(row + 1, col)).or(Some(&' ')).unwrap() {
                        second_letter @ 'A'..='Z' => {
                            // so it's horizontal
                            let third_letter = *maze.get(&(row + 2, col)).or(Some(&' ')).unwrap();
                            let loc = if third_letter == '.' {
                                // row+2, col is the portal location
                                (row + 2, col)
                            } else {
                                // row-1, col is the portal location
                                (row - 1, col)
                            };
                            ((first_letter, second_letter), loc)
                        }
                        _ => {
                            match *maze.get(&(row, col + 1)).or(Some(&' ')).unwrap() {
                                second_letter @ 'A'..='Z' => {
                                    // so it's vertical
                                    let third_letter =
                                        *maze.get(&(row, col + 2)).or(Some(&' ')).unwrap();
                                    let loc = if third_letter == '.' {
                                        // row, col+2 is the portal location
                                        (row, col + 2)
                                    } else {
                                        // row, col-1 is the portal location
                                        (row, col - 1)
                                    };
                                    ((first_letter, second_letter), loc)
                                }
                                _ => continue,
                            }
                        }
                    };
                    // println!("{:?}", portal);
                    if let Some((_, &loc1)) = tmp.get_key_value(&portal.0) {
                        // new
                        if is_outer(loc1, (min_x, min_y), (max_x, max_y)) {
                            // loc1 is an outer portal, going into it will
                            // decrease level by 1
                            portals.insert(loc1, (portal.1, -1));
                            // loc1 is an inner portal, going through it will
                            // increase level by 1
                            portals.insert(portal.1, (loc1, 1));
                        } else {
                            // vice versa
                            portals.insert(loc1, (portal.1, 1));
                            portals.insert(portal.1, (loc1, -1));
                        }
                    // /new
                    //
                    // portals.insert(loc1, portal.1);
                    // portals.insert(portal.1, loc1);
                    } else {
                        tmp.insert(portal.0, portal.1);
                    }
                }
                _ => continue,
            }
        }
    }
    let start = *tmp.get(&('A', 'A')).unwrap();
    let stop = *tmp.get(&('Z', 'Z')).unwrap();
    (start, stop, portals)
}

fn is_outer(loc: Point, topl: Point, botr: Point) -> bool {
    if loc.1 == topl.1 + 2 || loc.1 == botr.1 - 2 {
        // top row or bottom row
        return true;
    }
    if loc.0 == topl.0 + 2 || loc.0 == botr.0 - 2 {
        // left side or right side
        return true;
    }
    false
}

pub fn part1(input: &str) -> impl Display {
    clear_screen();
    let maze = parse(input);
    print(&maze);

    let (start, stop, portals) = get_start_stop_and_portals(&maze);
    distance(&maze, &portals, start, stop)
}

fn distance(
    maze: &HashMap<Point, char>,
    portals: &HashMap<Point, (Point, isize)>, // new isize
    from: Point,
    to: Point,
) -> usize {
    let mut visited = HashSet::new();
    let mut drones: VecDeque<(usize, Point, isize)> = VecDeque::new();

    drones.push_back((0, from, 0));

    let distance;
    loop {
        let mut visit = drones.pop_front().unwrap();

        match *maze.get(&visit.1).or(Some(&'#')).unwrap() {
            '#' => continue,
            'A'..='Z' => continue,
            _ => (),
        }

        if visited.contains(&(visit.1, visit.2)) {
            continue;
        }

        if visit.1 == to && visit.2 == 0 {
            distance = visit.0;
            break;
        }

        visited.insert((visit.1, visit.2));

        if let Some(&(dest, level_change)) = portals.get(&visit.1) {
            if level_change < 0 && visit.2 == 0 {
                continue; // negative levels apparently aren't good
            }
            // teleportation is a free move
            visit = (visit.0 + 1, dest, visit.2 + level_change);
        }

        drones.push_back((visit.0 + 1, (visit.1 .0, visit.1 .1 + 1), visit.2));
        drones.push_back((visit.0 + 1, (visit.1 .0, visit.1 .1 - 1), visit.2));
        drones.push_back((visit.0 + 1, (visit.1 .0 + 1, visit.1 .1), visit.2));
        drones.push_back((visit.0 + 1, (visit.1 .0 - 1, visit.1 .1), visit.2));
    }
    distance
}

pub fn part2(input: &str) -> impl Display {
    0
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
