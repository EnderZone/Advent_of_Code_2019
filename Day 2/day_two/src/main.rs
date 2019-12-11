fn main() {
    println!("Hello, world!");
    let filename = ".\\assets\\Input_one.txt";
    let contents: String = fs::read_to_string(filename).expect("Guess the filename is incorrect");
    let mut total_cost: i32 = 0;

    let mut instructions: Vec<i32> = contents
        .split(",")
        .map(|input| input.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}
