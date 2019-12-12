use std::collections::HashMap;
use std::fs;
use std::string::String;

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Hash, Eq, Copy, Clone)]
struct Coordinate {
    pos_x: i32,
    pos_y: i32,
}

#[derive(Hash, Eq, Copy, Clone)]
struct Wire {
    id: u32,
}

impl Direction {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        match self {
            Direction::U => "U".to_string(),
            Direction::D => "D".to_string(),
            Direction::L => "L".to_string(),
            Direction::R => "R".to_string(),
        }
    }
}

impl Coordinate {
    fn build_coordinate(x: i32, y: i32) -> Coordinate {
        Coordinate { pos_x: x, pos_y: y }
    }

    fn add_coordinate(&mut self, other: &Self) {
        self.pos_x += other.pos_x;
        self.pos_y += other.pos_y;
    }

    fn get_manhattan_distance(&self) -> u32 {
        (self.pos_x.abs() + self.pos_y.abs()) as u32
    }

    fn to_string(&self) -> String {
        format!("{}, {}", self.pos_x, self.pos_y)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        (self.pos_x == other.pos_x) & (self.pos_y == other.pos_y)
    }
}

impl Wire {
    fn build_wire(wire_id: u32) -> Wire {
        Wire { id: wire_id }
    }
}

impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        (self.id == other.id)
    }
}

fn add_wire_segment(
    start: &mut Coordinate,
    direction: Direction,
    length: u32,
    wireboard: &mut HashMap<Coordinate, Wire>,
    crossings: &mut Vec<Coordinate>,
    wire: &Wire,
) {
    let dir: Coordinate = match direction {
        Direction::U => Coordinate::build_coordinate(0, 1),
        Direction::D => Coordinate::build_coordinate(0, -1),
        Direction::L => Coordinate::build_coordinate(1, 0),
        Direction::R => Coordinate::build_coordinate(-1, 0),
    };

    for _ in 0..length {
        start.add_coordinate(&dir);
        if wireboard.contains_key(start) {
            if *(wireboard.get(start).unwrap()) != *wire {
                crossings.push(*start);
            }
        } else {
            wireboard.insert(*start, *wire);
        }
    }
}

fn find_shortest(crossings: &mut Vec<Coordinate>) -> u32 {
    let mut shortest: u32 = std::u32::MAX;

    crossings.iter().for_each(|cross| {
        let cross_manhattan_dist = cross.get_manhattan_distance();
        if shortest > cross_manhattan_dist {
            shortest = cross_manhattan_dist;
        }
    });

    shortest
}

fn check_in_between(coord_one: Coordinate, coord_two: Coordinate, test_coord : Coordinate, dir: Direction) -> bool {
    match dir {        
        Direction::U => {
            (coord_one.pos_x <= test_coord.pos_x) & (test_coord.pos_x <= coord_two.pos_x)
        },
        Direction::D => {
            (coord_one.pos_x >= test_coord.pos_x) & (test_coord.pos_x >= coord_two.pos_x)
        },
        Direction::L => {
            (coord_one.pos_y <= test_coord.pos_y) & (test_coord.pos_y <= coord_two.pos_y)
        },
        Direction::R => {
            (coord_one.pos_y >= test_coord.pos_y) & (test_coord.pos_y >= coord_two.pos_y)
        },
    }
}

fn get_cross_distance(mut instructions: String, target: Coordinate) -> u32 {
    let mut total: u32 = 0;

    let mut start: Coordinate = Coordinate::build_coordinate(0, 0);

    for command in instructions.split(",") {
        let dir: Direction = match &command[..1] {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => unreachable!("WHAT HAPPENED"),
        };

        let length: i32 = command[1..].to_string().parse::<i32>().unwrap();
        total += length as u32;

        let movement: Coordinate = match dir {
            Direction::U => Coordinate::build_coordinate(0, length),
            Direction::D => Coordinate::build_coordinate(0, -1*(length)),
            Direction::L => Coordinate::build_coordinate(length, 0),
            Direction::R => Coordinate::build_coordinate(-1*(length), 0),
        };

        let curr_coord: Coordinate = start;
        start.add_coordinate(&movement);
        if check_in_between(curr_coord, start, target, dir) {
            break;
        }
    }

    total
}

fn find_shortest_distance(wires: &mut Vec<String>, crossings: Vec<Coordinate>) -> u32 {
    let mut shortest_dist: u32 = std::u32::MAX;

    for cross in crossings {
        let mut curr_dist: u32 = 0;
        for mut wire_directions in wires.iter() {
            curr_dist += get_cross_distance(wire_directions.to_string(), cross);
        }

        if curr_dist < shortest_dist {
            shortest_dist = curr_dist;
            println!("{}", cross.to_string());
        }
    }

    shortest_dist
}

fn part_one() {
    let mut wireboard: HashMap<Coordinate, Wire> = HashMap::new();
    let mut crossings: Vec<Coordinate> = Vec::new();

    let mut wires: Vec<String> = Vec::new();
    collect_input(String::from("src/input.txt"), &mut wires);

    let mut index: u32 = 1;
    for wire_directions in wires.iter() {
        let mut position: Coordinate = Coordinate::build_coordinate(0, 0);
        let curr_wire = Wire::build_wire(index);
        for command in wire_directions.split(",") {
            let dir: Direction = match &command[..1] {
                "U" => Direction::U,
                "D" => Direction::D,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => unreachable!("WHAT HAPPENED"),
            };

            let length: u32 = command[1..].to_string().parse::<u32>().unwrap();
            add_wire_segment(
                &mut position,
                dir,
                length,
                &mut wireboard,
                &mut crossings,
                &curr_wire,
            );
        }

        index += 1;
    }

    crossings
        .iter()
        .for_each(|cross| println!("{}", cross.to_string()));

    println!("{}", find_shortest(&mut crossings));
}

fn part_two() {
    let mut wireboard: HashMap<Coordinate, Wire> = HashMap::new();
    let mut crossings: Vec<Coordinate> = Vec::new();

    let mut wires: Vec<String> = Vec::new();
    collect_input(String::from("src/input.txt"), &mut wires);

    let mut index: u32 = 1;
    for wire_directions in wires.iter() {
        let mut position: Coordinate = Coordinate::build_coordinate(0, 0);
        let curr_wire = Wire::build_wire(index);
        for command in wire_directions.split(",") {
            let dir: Direction = match &command[..1] {
                "U" => Direction::U,
                "D" => Direction::D,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => unreachable!("WHAT HAPPENED"),
            };

            let length: u32 = command[1..].to_string().parse::<u32>().unwrap();
            add_wire_segment(
                &mut position,
                dir,
                length,
                &mut wireboard,
                &mut crossings,
                &curr_wire,
            );
        }

        index += 1;
    }
    
    println!("FINAL NUM: {}", find_shortest_distance(&mut wires, crossings));
}

fn collect_input(filename: String, wires: &mut Vec<String>) {
    let contents: String = fs::read_to_string(filename).expect("Guess the filename is incorrect");
    for c in contents.lines() {
        wires.push(c.to_string());
    }
}

fn main() {
    part_two();
}
