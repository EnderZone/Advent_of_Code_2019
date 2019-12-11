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
    crossings: &mut HashMap<Coordinate, u32>,
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
                crossings.insert(*start, get_total_cost(wireboard, wire));
            }
        } else {
            wireboard.insert(*start, *wire);
        }
    }
}

fn get_total_cost(wireboard: &mut HashMap<Coordinate, Wire>, curr_wire: &Wire) -> u32 {
    let mut total_cost = 0;

    for (coordinate, wire) in &*wireboard {
        if wire == curr_wire {
            total_cost += 1;
        }
    }

    total_cost
}

fn find_shortest(crossings: &mut HashMap<Coordinate, u32>) -> u32 {
    let mut shortest: u32 = std::u32::MAX;

    for (cross, dist) in &*crossings {
        let cross_manhattan_dist = cross.get_manhattan_distance();
        if shortest > cross_manhattan_dist {
            shortest = cross_manhattan_dist;
        }
    }

    shortest
}

fn part_one() {
    let mut wireboard: HashMap<Coordinate, Wire> = HashMap::new();
    let mut crossings: HashMap<Coordinate, u32> = HashMap::new();

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
    let mut crossings: HashMap<Coordinate, u32> = HashMap::new();

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

fn collect_input(filename: String, wires: &mut Vec<String>) {
    let contents: String = fs::read_to_string(filename).expect("Guess the filename is incorrect");
    for c in contents.lines() {
        wires.push(c.to_string());
    }
}

fn main() {
    part_two();
}
