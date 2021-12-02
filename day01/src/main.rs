use std::io::Read;

fn main() {
 
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .unwrap();

    part_one(input);
   
}

fn parse_data(data: String) -> Vec<u16> {
    data.lines()
        .map(|e| e.parse::<u16>().unwrap())
        .collect()
}

fn count_increases(data: &Vec<u16>) -> usize {
    data.windows(2)
        .filter(|x| x[0] < x[1])
        .count()

}

fn part_one(input: String){

    let v: Vec<u16> = parse_data(input);
    print!("PART ONE\n# increases: {}\n", count_increases(&v));
}
