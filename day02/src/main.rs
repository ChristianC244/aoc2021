use std::io::Read;
use regex::Regex;


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("stdin Reading Error");

    let re = Regex::new(r"(\w+) (\d+)")
    .expect("Regex Init Error");

    let vec : Vec<(String, u32)> = re.captures_iter(&input)
        .map(|x| (x[1].to_string(), x[2].parse::<u32>().unwrap()))
        .collect();

    part_one(&vec);

}

fn part_one(vec: &Vec<(String,u32)> ) {

    let mut coord = (0,0);

    for cmd in vec.iter() {
        match cmd.0.as_str() {
            "forward" => coord = (coord.0 + cmd.1, coord.1),
            "up" => coord = (coord.0, coord.1 - cmd.1),
            "down" => coord = (coord.0, coord.1 + cmd.1),
            _ => return,
            
        }
    }

    println!("PART ONE\nResult: {}", coord.0*coord.1)
}

