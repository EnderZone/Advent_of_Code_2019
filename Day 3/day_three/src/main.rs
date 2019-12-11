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

#[derive(Hash, Eq)]
struct Coordinate {
    pos_x: i32,
    pos_y: i32,
}

#[derive(Hash, Eq)]
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
        Coordinate {
            pos_x: x,
            pos_y: y,
        }
    }

    fn add_coordinate(&self, x: i32, y: i32) {
        self.pos_x += x;
        self.pos_y += y;
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        (self.pos_x == other.pos_x) & (self.pos_y == other.pos_y)
    }
}

impl Wire {
    fn build_wire(wire_id: u32) -> Wire {
        Wire {
            id: wire_id,
        }
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
    crossings: &mut Vec<Coordinate>
) {
    let new_crosses : Vec<Coordinate> = Vec::new();


}

fn part_one() {
    let mut wireboard: HashMap<Coordinate, Wire> = HashMap::new();
    let mut crossings: Vec<Coordinate> = Vec::new();

    let mut wires: Vec<String> = Vec::new();
    collect_input(String::from("src/input.txt"), &mut wires);

    for wire_directions in wires.iter() {
        let position : Coordinate = Coordinate::build_coordinate(0, 0);
        for command in wire_directions.split(",") {
            let dir: Direction = match &command[..1] {
                "U" => Direction::U,
                "D" => Direction::D,
                "L" => Direction::L,
                "R" => Direction::R,
                _ => unreachable!("WHAT HAPPENED"),
            };

            let length: u32 = command[1..].to_string().parse::<u32>().unwrap();
            add_wire_segment(&mut position, dir, length, &mut wireboard, &mut crossings);
        }
    }
}

fn collect_input(filename: String, wires: &mut Vec<String>) {
    let contents: String = fs::read_to_string(filename).expect("Guess the filename is incorrect");
    for c in contents.lines() {
        wires.push(c.to_string());
    }
}

fn main() {
    part_one();
}
