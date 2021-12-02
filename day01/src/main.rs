use std::io::Read;

fn main() {
 
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .unwrap();

    let data: Vec<u16> = parse_data(&input);
    part_one(&data);
    part_two(&data);
   
}

fn parse_data(data: &str) -> Vec<u16> {
    data.lines()
        .map(|e| e.parse::<u16>().unwrap())
        .collect()
}

fn count_increases(data: &Vec<u16>) -> usize {
    data.windows(2)
        .filter(|x| x[0] < x[1])
        .count()

}

fn windows_vector(v: &Vec<u16>) -> Vec<u16> {
    v.windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect()

}

fn part_one(v: &Vec<u16>){

    print!("PART ONE\n# increases: {}\n", count_increases(&v));
}

fn part_two(v: &Vec<u16>){
 
    let v3: Vec<u16> = windows_vector(&v);
    print!("PART TWO\n# increases: {}\n", count_increases(&v3));
}
