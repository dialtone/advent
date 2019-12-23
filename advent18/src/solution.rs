use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::iter::FromIterator;
use std::thread::sleep;
use std::time::Duration;

type Point = (isize, isize);
type Quadrant = ((isize, isize), (isize, isize));

fn parse(input: &str) -> HashMap<Point, char> {
    let mut pos = (0, 0);
    let mut maze = HashMap::new();

    for line in input.lines() {
        for dot in line.chars() {
            maze.insert(pos, dot);
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

    print_quadrant(maze, ((min_x, min_y), (max_x, max_y)))
}

fn print_visited(maze: &HashMap<Point, char>, visited: &HashSet<Point>) {
    go_to_top();

    let ((min_x, min_y), (max_x, max_y)) = edges(maze);

    for y in min_y..=max_y {
        let row = (min_x..=max_x)
            .map(|x| {
                if visited.contains(&(x, y)) {
                    *maze.get(&(x, y)).or(Some(&'▒')).unwrap()
                } else {
                    '#'
                }
            })
            .collect::<Vec<char>>();
        println!("{}", String::from_iter(row));
    }
    sleep(Duration::from_millis(50));
}

fn print_quadrant(maze: &HashMap<Point, char>, ((min_x, min_y), (max_x, max_y)): Quadrant) {
    for y in min_y..=max_y {
        let row = (min_x..=max_x)
            .map(|x| *maze.get(&(x, y)).or(Some(&'▒')).unwrap())
            .collect::<Vec<char>>();
        println!("{}", String::from_iter(row));
    }
    sleep(Duration::from_millis(200));
}

pub fn part1(input: &str) -> impl Display {
    // clear_screen();
    let maze = parse(input);
    // print(&maze);

    let pos = *maze.iter().filter(|(&_, &v)| v == '@').collect::<Vec<_>>()[0].0;
    let mut keys = HashMap::new();

    for (k, v) in &maze {
        if let 'a'..='z' = v {
            keys.insert(v, k);
        }
    }
    // println!("Hi from {:?}", pos);

    let mut distances = HashMap::new();

    for (&&key, &&kpos) in &keys {
        let dist_keys = distance(&maze, pos, kpos);
        distances.insert((pos, kpos), dist_keys.clone());
        distances.insert((kpos, pos), dist_keys);

        for (&&otherkey, &&otherkpos) in &keys {
            if otherkey == key {
                continue;
            }
            let dist_keys = distance(&maze, kpos, otherkpos);
            distances.insert((kpos, otherkpos), dist_keys.clone());
            distances.insert((otherkpos, kpos), dist_keys);
        }
    }
    //println!("all distances {:?}", distances);

    let mut shortest_path = (usize::max_value(), vec![]);
    // one could avoid the binary heap if you kept around a mapping
    // between the best length and the set of keys and discard the paths
    // that have a worse off length there. The binary heap is easier to deal
    // with since it bubbles up the lowest distance paths automatically.
    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();
    q.push((Reverse(0), pos, vec![]));

    while !q.is_empty() {
        let visit = q.pop().unwrap();

        let mut sorted_keys = visit.2.clone();
        sorted_keys.sort();

        if visited.contains(&(visit.1, sorted_keys.clone())) {
            // sometimes this drops the wrong set of keys
            continue;
        }

        if visit.2.len() == keys.len() {
            //println!("Paths {:?}", visit);
            if shortest_path.0 > visit.0 .0 {
                shortest_path = (visit.0 .0, visit.2.clone());
            } else {
                continue;
            }
        }

        visited.insert((visit.1, sorted_keys));

        for (&&k, &&kpos) in &keys {
            if visit.2.contains(&k) {
                continue; // already collected
            }
            let dist_keys = distances.get(&(visit.1, kpos)).unwrap();
            if dist_keys.1.iter().all(|k| visit.2.contains(k)) {
                let mut kcollected = visit.2.clone();
                kcollected.push(k);
                let next_visit = (Reverse(visit.0 .0 + dist_keys.0), kpos, kcollected);
                //println!("Next Step from {:?} to {:?}", visit, next_visit);
                q.push(next_visit);
            }
        }
    }
    // println!("Path: {:?}", shortest_path.1);
    shortest_path.0
}

fn distance(maze: &HashMap<Point, char>, from: Point, to: Point) -> (usize, Vec<char>) {
    let mut visited = HashSet::new();
    let mut drones = VecDeque::new();

    drones.push_back((0, from, vec![]));

    let distance_and_keys;
    loop {
        let mut visit = drones.pop_front().unwrap();

        if visited.contains(&visit.1) {
            continue;
        }

        if visit.1 == to {
            distance_and_keys = (visit.0, visit.2);
            break;
        }

        visited.insert(visit.1);
        //print_visited(&maze, &visited);

        match *maze.get(&visit.1).or(Some(&'#')).unwrap() {
            '#' => continue, // wall
            '.' | 'a'..='z' | '@' => (),
            door @ 'A'..='Z' => visit.2.push(door.to_lowercase().next().unwrap()),
            _ => (),
        }

        drones.push_back((visit.0 + 1, (visit.1 .0, visit.1 .1 + 1), visit.2.clone()));
        drones.push_back((visit.0 + 1, (visit.1 .0, visit.1 .1 - 1), visit.2.clone()));
        drones.push_back((visit.0 + 1, (visit.1 .0 + 1, visit.1 .1), visit.2.clone()));
        drones.push_back((visit.0 + 1, (visit.1 .0 - 1, visit.1 .1), visit.2.clone()));
    }
    distance_and_keys
}

pub fn part2(input: &str) -> impl Display {
    // clear_screen();

    let mut maze = parse(input);
    let pos = *maze.iter().filter(|(&_, &v)| v == '@').collect::<Vec<_>>()[0].0;

    // turn the map into 4 quadrants
    maze.insert(pos, '#');
    maze.insert((pos.0, pos.1 + 1), '#');
    maze.insert((pos.0, pos.1 - 1), '#');
    maze.insert((pos.0 + 1, pos.1), '#');
    maze.insert((pos.0 - 1, pos.1), '#');
    maze.insert((pos.0 + 1, pos.1 + 1), '@');
    maze.insert((pos.0 + 1, pos.1 - 1), '@');
    maze.insert((pos.0 - 1, pos.1 - 1), '@');
    maze.insert((pos.0 - 1, pos.1 + 1), '@');

    // print(&maze);

    let ((min_x, min_y), (max_x, max_y)) = edges(&maze);

    let starts = vec![
        (pos.0 - 1, pos.1 - 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
    ];
    let quadrants = vec![
        ((min_x, min_y), pos),
        ((pos.0, min_y), (max_x, pos.1)),
        ((min_x, pos.1), (pos.0, max_y)),
        (pos, (max_x, max_y)),
    ];

    let mut keys = HashMap::new();
    for (k, v) in &maze {
        if let 'a'..='z' = v {
            keys.insert(v, k);
        }
    }

    let mut doors = HashMap::new();
    for (k, v) in &maze {
        if let 'A'..='Z' = v {
            doors.insert(v.to_lowercase().next().unwrap(), k);
        }
    }

    // clear_screen();
    // go_to_top();
    // println!("Original start {:?}", pos);
    // println!("Starts {:?}", starts);
    // println!("Quadrants {:?}", quadrants);
    let mut total_distance = 0;
    for (i, &quadrant) in quadrants.iter().enumerate() {
        let start = starts[i];
        // println!("Starts at {:?}", start);
        let in_quadrant_keys = keys
            .iter()
            .filter(|&(_k, &&kpos)| in_quadrant(kpos, quadrant))
            .collect::<Vec<_>>();
        // println!("Keys {:?}", in_quadrant_keys);
        // print_quadrant(&maze, quadrant);

        let mut distances = HashMap::new();
        for (&&key, &&kpos) in &in_quadrant_keys {
            // println!("{} {:?}", key, kpos);
            let dist_keys = distance(&maze, start, kpos);
            distances.insert((start, kpos), dist_keys.clone());
            distances.insert((kpos, start), dist_keys);

            for (&&otherkey, &&otherkpos) in &in_quadrant_keys {
                if otherkey == key {
                    continue;
                }
                // println!("otherkey {} {:?}", otherkey, otherkpos);
                let dist_keys = distance(&maze, kpos, otherkpos);
                distances.insert((kpos, otherkpos), dist_keys.clone());
                distances.insert((otherkpos, kpos), dist_keys);
            }
        }

        // println!("Distances: {:?}", distances);

        // Here each quadrant can be solved independently since you can assume
        // that the other robots will unlock the keys you need when you need
        // so you just need to do bfs on the 4 quadrants independently given
        // they have no keys in common.

        let mut shortest_path = (usize::max_value(), vec![]);
        let mut q = BinaryHeap::new();
        let mut visited = HashSet::new();
        q.push((Reverse(0), start, vec![]));

        while !q.is_empty() {
            let visit = q.pop().unwrap();

            let mut sorted_keys = visit.2.clone();
            sorted_keys.sort();

            if visited.contains(&(visit.1, sorted_keys.clone())) {
                continue;
            }

            if visit.2.len() == in_quadrant_keys.len() {
                // println!("Paths {:?}", visit);
                if shortest_path.0 > visit.0 .0 {
                    shortest_path = (visit.0 .0, visit.2.clone());
                    break;
                }
            }

            visited.insert((visit.1, sorted_keys));

            for (&&k, &&kpos) in &in_quadrant_keys {
                if visit.2.contains(&k) {
                    // println!(">>>>>  contains");
                    continue; // already collected
                }
                let dist_keys = distances.get(&(visit.1, kpos)).unwrap();
                let mut kcollected = visit.2.clone();
                kcollected.push(k);
                let next_visit = (Reverse(visit.0 .0 + dist_keys.0), kpos, kcollected);
                q.push(next_visit);
            }
        }
        total_distance += shortest_path.0;
    }

    total_distance
}

fn in_quadrant(pos: Point, ((min_x, min_y), (max_x, max_y)): Quadrant) -> bool {
    pos.0 >= min_x && pos.0 <= max_x && pos.1 >= min_y && pos.1 <= max_y
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
