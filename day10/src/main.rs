use std::io::Read;
use std::collections::VecDeque;


fn value_of(c: char) -> usize {

    let close = [')', ']', '}',  '>' ];
    let payoff = [3, 57, 1197, 25137];

    match close.iter().position(|x| *x==c) {
        Some(ind) => payoff[ind],
        None => 0,
    }
}

fn find_corrupted(line: &str) -> usize {

    let open = ['(', '[', '{', '<'];
    let close = [')', ']', '}', '>'];
    let mut mem: VecDeque<char> = VecDeque::new();
    
    for x in line.chars() {

        if open.contains(&x) {
            mem.push_back(x);
        }

        else if close.contains(&x) {
            let open_br =  match mem.pop_back() {
                Some(c) => c,
                None => panic!("No open bracket found!")
            };
            let index = open.iter().position(|c| *c == open_br).unwrap();
            
            if close[index] == x {continue}
            else {
                // println!("Expected {}, but found {} instead", &close[index], &x);
                return value_of(x)
            };
        };
    };
    return 0
}

fn find_incomplete(line: &str) -> String{

    let open = ['(', '[', '{', '<'];
    let close = [')', ']', '}', '>'];
    let mut mem: VecDeque<char> = VecDeque::new();

    for x in line.chars() {

        if open.contains(&x) {
            mem.push_front(x);
        }

        else if close.contains(&x) {
            mem.pop_front();
        };
    }

    mem.drain(0..).map(|x| {
        let i = open.iter().position(|b| *b == x).unwrap();
        close[i]
    }).collect::<String>()
}

fn count_value(input: &str) -> usize{

    let brackets = [')', ']', '}', '>'];

    input.chars().fold(0, |mut acc, x| {
        acc *= 5;
        acc + brackets.iter().position(|b| *b == x).unwrap() +1
    })
}

fn main() {

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let (corrupted, incomplete): (Vec<&str>, Vec<&str>) = buf.lines().partition(|x| find_corrupted(x) != 0);

    part_one(corrupted);
    part_two(incomplete)
}

fn part_one(input: Vec<&str>) {

    let res = input.iter().fold(0, |acc, x| acc + find_corrupted(x));
    println!("Part One: {}", res);
}

fn part_two(input: Vec<&str>){
    
    let mut res: Vec<usize> = input.iter().map(|x| count_value(&find_incomplete(*x)) ).collect();
    res.sort_unstable();

    let len = res.len();

    println!("Part two: {}", res[len/2])
}
