use std::fs;
use std::string::String;

fn main() {
    let filename = ".\\assets\\Input_one.txt";
    let contents: String = fs::read_to_string(filename).expect("Guess the filename is incorrect");
    let mut total_cost : i32 = 0;
    for cost in contents.lines() {
        let mass: i32 = cost.parse::<i32>().unwrap();
        calculate_cost(mass, &mut total_cost);
    }

    println!("Total cost is: {}", total_cost);
}

fn calculate_cost(mass: i32, total: &mut i32) {
    let fuel_cost : i32 = (mass / 3) - 2;
    if fuel_cost > 0 {
        calculate_cost(fuel_cost, total);
        *total = *total + (mass/3) - 2;
    }
}