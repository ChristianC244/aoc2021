use std::io::Read;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let input: Vec<i32> = buf.split(",").map(|x| x.parse::<i32>()
        .unwrap())
        .collect();

    part_one_opt(&input);
    part_two_opt(&input);
}
fn fuel_calc(target: i32, crabs: &Vec<i32>) -> i32{
    
    let crabs = crabs.clone();
    crabs.iter()
        .map(| x | {
        ((target - x) * (target - x) + (target - x).abs() ) /2
        })
        .sum()
}

fn part_one_opt(line: &Vec<i32>) {
    // Calculates median, then sums distances from it. That is the fuel cost
    println!("PART ONE OPTIMIZED");
    
    let mut crab_pos = line.clone();
    crab_pos.sort_unstable();
    
    let median = crab_pos[(crab_pos.len()/2)];
    
    let fuel = crab_pos.iter().fold(0, |acc, x| {
        acc + (*x - median).abs()
    });

    println!("Min fuel cost: {}",fuel);    
}

fn part_two_opt(line: &Vec<i32>) {
    // The target position is the mean of all the starting positions ± 1/2 

    let crab_pos = line.clone();
    let num_crab = line.len() as i32;

    let mean = crab_pos.iter().fold(0, |acc, x| acc + *x) / num_crab;

    let fuel1 = fuel_calc(mean + 1, &crab_pos);
    let fuel2 = fuel_calc(mean , &crab_pos);
    let fuel3 = fuel_calc(mean - 1, &crab_pos);

    let fuel = vec![fuel1,fuel2,fuel3];
    println!("Min fuel cost: {}", fuel.iter().min().unwrap());
}
