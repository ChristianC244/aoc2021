use std::io::Read;
use lib::{Session, Solve, Single};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let input: Vec<Session> = buf.lines().map(| x | Session::parse(x)).collect();
    part_one(&input);
    part_two(&input);

}

fn all_unique(v: &Single) -> bool{
    // As soon as a duplicate is found returns !true
    !v.possibities
        .iter()
        .enumerate()
        .any(| (i, x) |  v.possibities.iter().skip(i+1).any(| y | *x==*y)  )
}

fn digitalize(v: &Single, num: &str) -> Option<usize> {
    
    let res = num.chars().fold(0, |acc, x| {
        acc + v.possibities
            .iter()
            .enumerate()
            .filter(|(_, val)| **val == x)
            .fold(0, |_, (i, _)| 2usize.pow(6u32 - i as u32))
    });

    match res {
        // Result of a Look-up table
        119 => Some(0),
        18 => Some(1),
        93 => Some(2),
        91 => Some(3),
        58 => Some(4),
        107 => Some(5),
        111 => Some(6),
        82 => Some(7),
        127 => Some(8),
        123 => Some(9),
        _ => None
    }
}

fn part_one(input: &Vec<Session>) {
    println!("PART ONE");

    let res = input.iter().fold(0, |acc, x| acc + x.count_ez_digit());

    println!("{}",res)
}

fn part_two(from: &Vec<Session>) {
    println!["PART TWO"];
    
    let sessions = from.clone();
    let mut res: usize = 0;

    for mut line in sessions {
        line.input.sort_unstable_by_key(| x | x.len());
        
        let input = line.input;
        let mut solver = Solve::new();

        // Filter len 2 -> 1
        solver.update_one(&input[0]);

        // Filter len 3 -> 7
        solver.update_seven(&input[1]);

        // Filter len 4 -> 4
        solver.update_four(&input[2]);
        
        // Generate all possible combination -> filter it
        let mut brute = solver.generate_all_poss();

        brute = brute.iter()
            .filter(|x| all_unique(x)).map(|x| x.clone())
            .collect();
        
        for key in brute.iter() {

            if input.iter().any(| x | digitalize(key, x) == None) {continue;} //Check if all number can be real
            
            res += line.output
            .iter()
            .map(| x | digitalize(key, x).unwrap())
            .map(| x | char::from_digit(x as u32, 10).unwrap())
            .collect::<String>()
            .parse::<usize>().unwrap();
            // more .map() more fun, no?
        }
    }
    println!("Final Result: {}", res);
    
}
