use crate::intcode;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::iter::FromIterator;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
struct Cell {
    loc: (isize, isize),
    content: char,
    weight: usize,
    is_explored: bool,
    distance: isize,
}

impl Cell {
    fn new(loc: (isize, isize), content: char, distance: isize) -> Self {
        Cell {
            loc: loc,
            content: content,
            weight: 1,
            is_explored: false,
            distance: distance,
        }
    }
    fn explored(&mut self) {
        self.is_explored = true
    }
}

fn print_lab(maze: &HashMap<(isize, isize), Cell>) {
    let min_x = maze.keys().min_by_key(|(x, _)| *x).unwrap().0;
    let max_x = maze.keys().max_by_key(|(x, _)| *x).unwrap().0;

    let min_y = maze.keys().min_by_key(|(_, y)| *y).unwrap().1;
    let max_y = maze.keys().max_by_key(|(_, y)| *y).unwrap().1;
    for y in (min_y - 5)..max_y + 5 {
        let row = ((min_x - 5)..max_x + 5)
            .map(|x| {
                maze.get(&(x, y))
                    .or(Some(&Cell::new((0, 0), '▒', 0)))
                    .unwrap()
                    .content
            })
            .collect::<Vec<char>>();
        println!("{}", String::from_iter(row));
    }
    sleep(Duration::from_millis(10));
}

#[derive(Debug, Clone)]
struct Drone {
    pos: (isize, isize),
    computer: intcode::Computer,
    maze: HashMap<(isize, isize), Cell>,
    distance: isize,
    oxygen_found: bool,
}

impl Drone {
    fn new(input: &str) -> Self {
        let pos = (0, 0);
        let mut maze = HashMap::new();
        maze.insert(pos, Cell::new(pos, 'D', 0));
        Drone {
            pos: pos,
            computer: intcode::Computer::from(input),
            maze: maze,
            distance: 0,
            oxygen_found: false,
        }
    }

    fn reset(&mut self) {
        // resets everything but not the computer.
        self.pos = (0, 0);
        self.maze = HashMap::new();
        self.maze.insert(self.pos, Cell::new(self.pos, 'D', 0));
        self.oxygen_found = false;
        self.distance = 0;
    }

    fn print(&self) {
        go_to_top();
        println!(
            "pos({}, {}); distance: {}    ",
            self.pos.0, self.pos.1, self.distance
        );
        print_lab(&self.maze)
    }

    fn explore(&mut self) -> Vec<isize> {
        let tmppos = self.pos;
        let mut next_directions = vec![];
        for direction in 1..5 {
            let nextpos = compute_pos(self.pos, direction);
            if self.maze.contains_key(&nextpos) {
                // already know that tile
                // this will cause a dead end later, which is fine because
                // we don't want to do backtracking.
                continue;
            }
            self.step(direction, 1);
            if tmppos == self.pos {
                // we didn't move (so a wall probably)
                continue;
            }
            // so we moved, since we have no backtracking though we should
            // always be in unexplored areas and there are no cycles
            if !self.maze.contains_key(&nextpos) {
                next_directions.push(direction);
            } else if !self.maze.get(&nextpos).unwrap().is_explored {
                next_directions.push(direction);
            }

            // Here we just bring back the drone to the starting position
            self.step(invert(direction), -1);
        }
        self.maze.entry(self.pos).and_modify(|o| o.explored());
        next_directions
    }

    fn step(&mut self, direction: isize, distance_incr: isize) {
        // take a step
        let nextpos = compute_pos(self.pos, direction);

        // used to write only when computer waiting.
        self.computer.write_input(direction);
        self.computer.run();

        if let Some(output) = self.computer.pop_output() {
            match output {
                0 => {
                    // wall, didn't move so don't update self.pos
                    add_cell(&mut self.maze, nextpos, '█', self.distance + distance_incr);
                }
                1 => {
                    // moved, update old tile at self.pos which is now old
                    // position, with old distance
                    if self.pos == (0, 0) {
                        add_cell(&mut self.maze, self.pos, 'S', self.distance);
                    } else {
                        add_cell(&mut self.maze, self.pos, '.', self.distance);
                    }

                    // this is new position now, update our pointer, new distance
                    self.distance += distance_incr;
                    self.pos = nextpos;
                    add_cell(&mut self.maze, nextpos, 'D', self.distance);
                }
                2 => {
                    // found it! we moved and our new position is oxygen, game
                    // is actually over.
                    self.oxygen_found = true;
                    // update old position
                    add_cell(&mut self.maze, self.pos, '.', self.distance);
                    self.distance += distance_incr;
                    self.pos = nextpos;
                    add_cell(&mut self.maze, nextpos, 'O', self.distance);
                }
                _ => {
                    println!("Error");
                }
            }
        }
        self.print();
    }
}

pub fn part12(input: &str) -> (isize, isize) {
    clear_screen();

    let mut paused_drones: VecDeque<Drone> = VecDeque::new();
    let mut drone = Drone::new(input);
    drone.print();
    let mut rng = thread_rng();
    let mut oxygen_drone = drone.clone();
    loop {
        let mut next_directions = drone.explore();
        if drone.oxygen_found {
            drone.print();
            println!("OXYGEN FOUND");
            oxygen_drone = drone.clone();
        }
        if next_directions.len() == 0 {
            // dead end, so "backtrack" to a previous known state
            if let Some(mut newdrone) = paused_drones.pop_back() {
                newdrone.maze.extend(drone.maze);
                drone = newdrone;
                continue;
            } else {
                // drones are done
                break;
            }
        }

        next_directions.shuffle(&mut rng);
        for next_direction in next_directions {
            let mut newdrone = drone.clone();
            newdrone.step(next_direction, 1);
            paused_drones.push_back(newdrone);
        }
        drone = paused_drones.pop_back().unwrap();
    }

    let oxygen_distance = oxygen_drone.distance;
    drone = oxygen_drone;
    let mut longest_drone_distance = 0;
    drone.reset(); // forget the map and redo this from the oxygen now
    loop {
        let mut next_directions = drone.explore();
        if next_directions.len() == 0 {
            // dead end, so "backtrack" to a previous known state
            if let Some(mut newdrone) = paused_drones.pop_back() {
                newdrone.maze.extend(drone.maze);
                if drone.distance > longest_drone_distance {
                    longest_drone_distance = drone.distance;
                }
                drone = newdrone;
                continue;
            } else {
                // drones are done
                break;
            }
        }

        next_directions.shuffle(&mut rng);
        for next_direction in next_directions {
            let mut newdrone = drone.clone();
            newdrone.step(next_direction, 1);
            paused_drones.push_back(newdrone);
        }
        drone = paused_drones.pop_back().unwrap();
    }
    (oxygen_distance, longest_drone_distance)
}

//
fn compute_pos(pos: (isize, isize), direction: isize) -> (isize, isize) {
    let mov = movement(direction);
    (pos.0 + mov.0, pos.1 + mov.1)
}

fn invert(direction: isize) -> isize {
    match direction {
        1 => 2, // north
        2 => 1, // south
        3 => 4, // west
        _ => 3, // east
    }
}

fn add_cell(
    lab: &mut HashMap<(isize, isize), Cell>,
    pos: (isize, isize),
    content: char,
    distance: isize,
) {
    let entry = lab.entry(pos).or_insert(Cell::new(pos, content, distance));
    entry.content = content;
}

fn movement(direction: isize) -> (isize, isize) {
    match direction {
        1 => (0, 1),  // north
        2 => (0, -1), // south
        3 => (1, 0),  // west
        4 => (-1, 0), // east
        _ => (0, 0),
    }
}
fn go_to_top() {
    print!("{}[0;0H", 27 as char);
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
