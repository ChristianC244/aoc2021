use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let input: Vec<usize> = buf.split(",").map(|x| x.parse::<usize>()
        .unwrap())
        .collect();

    let max_len = *input.clone()
        .iter()
        .max()
        .unwrap();

    let line = input.iter().fold(vec![0; max_len +1], |mut acc, x| {
        acc[*x] += 1;
        acc
    });

    part_one(line.clone());
    part_two(line);
    
}

fn fuel_calc(dist: i32) -> i32{
    // dist should be dist +1; hence the formula below
    dist*(dist+1)/2

}


fn part_one(line: Vec<i32>) {
    println!("PART ONE");

    let vec_len = line.len();
    //For every fish calculate how much it costs to move in each space and sum with others
    let fuel_cost = line.iter()
        .enumerate()
        .fold(vec![0i32;vec_len], | mut cost, (i, val) | {
        
        let i = i as i32;
        cost.iter_mut()
            .enumerate()
            .map(|(c_i, c_val)| {
                *c_val + ( c_i as i32 - i).abs() * *val
            }).collect()
    });

    println!("{}", fuel_cost.iter().min().unwrap());
}

fn part_two(line: Vec<i32>) {
    println!("PART TWO");

    let vec_len = line.len();
    //For every fish calculate how much it costs to move in each space and add to others
    let fuel_cost = line.iter()
        .enumerate()
        .fold(vec![0i32;vec_len], | mut cost, (i, val) | {
        
        let i = i as i32;
        cost.iter_mut()
            .enumerate()
            .map(|(c_i, c_val)| {
                *c_val + fuel_calc(( c_i as i32 - i).abs() ) * *val
            }).collect()
    });

    println!("{}", fuel_cost.iter().min().unwrap());

}

