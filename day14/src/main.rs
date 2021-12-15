use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut formulas = HashMap::<(char, char), char>::new();

    let input: Vec<char> = input.chars().filter(|c| c.is_alphabetic() ).collect();
    
    buf.lines().for_each(|x| {
        let l = x.chars().nth(0).unwrap();
        let r = x.chars().nth(1).unwrap();
        let k = x.chars().nth(6).unwrap();

        formulas.insert((l, r), k);
    });

    part_one(&input, &formulas );

    let mut input_map = HashMap::<(char, char), usize>::new();

    input.windows(2).for_each(|x| {
        *input_map.entry((x[0], x[1])).or_insert(0) += 1
    });

    part_two(&input_map, &formulas);


}

fn next_round(input: Vec<char>, lookup: &HashMap<(char, char), char>) -> Vec<char> {

    let mut res:Vec<char> = Vec::from( [input[0]] );
    input.windows(2).for_each(|x| {
        res.push( *lookup.get( &(x[0], x[1])).unwrap() );
        res.push(x[1]);
    });

    res
}

fn next_round_hash(input: HashMap<(char, char), usize>, lookup: &HashMap<(char, char), char>) -> HashMap<(char, char), usize> {

    let mut res = HashMap::<(char, char), usize>::new();

    for (k, val) in input {
        let p = *lookup.get(&(k.0, k.1)).unwrap();
        *res.entry((k.0, p)).or_insert(0) += val;
        *res.entry((p, k.1)).or_insert(0) += val;
    };

    res
}

fn part_one(polymer: &Vec<char>, lookup: &HashMap<(char, char), char>) {

    let mut polymer = polymer.clone();
    
    for _ in 0..10 {
        polymer = next_round(polymer, lookup);
    }

    let mut counter = HashMap::<char, usize>::new();

    polymer.iter().for_each(| x | {
        *counter.entry(*x).or_insert(0) += 1usize;
    });

    let max = counter.iter().max_by_key(|(_, v)| **v).unwrap().1;
    let min = counter.iter().min_by_key(|(_, v)| **v).unwrap().1;

    println!("Part one: {}", max - min)
}

fn part_two(polymer: &HashMap<(char, char), usize>, lookup: &HashMap<(char, char), char>) {
    
    let mut polymer = polymer.clone();

    for _ in 0..40 {
        polymer = next_round_hash(polymer, lookup);
    }

    let mut counter = HashMap::<char, usize>::new();
    
    for (k, val) in polymer {
        *counter.entry(k.1).or_insert(0) += val;
    }

    let max = counter.iter().max_by_key(|(_, v)| **v).unwrap().1;
    let min = counter.iter().min_by_key(|(_, v)| **v).unwrap().1;

    println!("Part two: {}", max - min)
}