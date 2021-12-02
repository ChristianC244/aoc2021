use std::io::Read;
use regex::Regex;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("stdin Reading Error");

    let re = Regex::new(r"(\w+) (\d+)")
    .expect("Regex Init Error");

    let vec : Vec<(String, i32)> = re.captures_iter(&input)
        .map(|x| (x[1].to_string(), x[2].parse::<i32>().unwrap()))
        .collect();

    part_one(&vec);
    part_two(&vec);

}

fn part_one(vec: &Vec<(String,i32)> ) {
    
    let coord = vec.iter().fold((0,0), |acc:(i32, i32), cmd| {
        match cmd.0.as_str() {
            "forward" => (acc.0 + cmd.1, acc.1),
            "up" => (acc.0, acc.1 - cmd.1),
            "down" => (acc.0, acc.1 + cmd.1),
            _ => unreachable!(), 
            }
        });

    println!("PART ONE\nResult: {}", coord.0*coord.1)
}

fn part_two(vec: &Vec<(String, i32)>) {

    let coord = vec.iter().fold((0,0,0), |(x,y,aim), cmd| {
        match cmd.0.as_str() {
            "forward" => (x+cmd.1, y+aim*cmd.1, aim),
            "up" => (x, y,aim - cmd.1),
            "down" => (x, y, aim + cmd.1),
            _ => unreachable!() 
        }
    });

    println!("PART TWO\nResult: {}", coord.0*coord.1)
}

