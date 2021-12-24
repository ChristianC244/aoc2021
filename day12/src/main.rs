use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut cave_map: HashMap<&str, Vec<&str>> = HashMap::new();

    buf.lines().for_each(|x|{
        let cave: Vec<&str> = x.split('-').collect();

        cave_map.entry(cave[0]).or_insert(Vec::new()).push(cave[1]);
        cave_map.entry(cave[1]).or_insert(Vec::new()).push(cave[0]);
    });

    part_one(&cave_map);
    part_two(&cave_map);
}

fn pass_one(cave_map: &HashMap<&str, Vec<&str>>, current: &str, passed: &Vec<&str>) -> u32{
    
    if current.eq("end") {return 1}

    // add current to current passed caves if lowercase
    let mut passed = passed.clone();
    if current.chars().nth(0).unwrap().is_lowercase() {passed.push(current)}
    
    // caves to access
    let go = cave_map.get(current).unwrap();
    let go = go.clone()
        .into_iter()
        .filter(|x| !passed.contains(x) )
        .collect::<Vec<&str>>();

    // now pass thorough caves
    let mut counter = 0;
    for cave in go {
        counter += pass_one(cave_map, cave, &passed);
    }
    counter

}

fn part_one(cave_map: &HashMap<&str, Vec<&str>>) {
    
    let mut counter = 0;
    let passed = vec!["start"];
    for cave in cave_map.get("start").unwrap() {
        counter += pass_one(cave_map, cave, &passed);
    }
    println!("Part One:\nPossible ways to move: {}", &counter)
}

fn pass_two(cave_map: &HashMap<&str, Vec<&str>>, current: &str, passed_once: &Vec<&str>, passed_twice: Option<&str>) -> u32{
    
    if current.eq("end") {return 1}
    if current.eq("start") {return 0}

    // add current to current passed caves if lowercase
    let mut passed_once = passed_once.clone();
    let mut passed_twice = passed_twice;
    
    if current.chars().nth(0).unwrap().is_lowercase() && !passed_once.contains(&current) {passed_once.push(current)}
    else if current.chars().nth(0).unwrap().is_lowercase() && passed_twice.is_none() && passed_once.contains(&current) {passed_twice = Some(current)}
    else if current.chars().nth(0).unwrap().is_lowercase() && passed_twice.is_some() && passed_once.contains(&current) {return 0};

    // caves to access
    let go = cave_map.get(current).unwrap();
    let go = go.clone()
        .into_iter()
        .collect::<Vec<&str>>();

    // now pass thorough caves
    let mut counter = 0;
    for cave in go {
        counter += pass_two(cave_map, cave, &passed_once, passed_twice);
    }
    counter
}

fn part_two(cave_map: &HashMap<&str, Vec<&str>>) {
    
    let mut counter = 0;
    let passed_once: Vec<&str> = Vec::new();

    for cave in cave_map.get("start").unwrap() {
        counter += pass_two(cave_map, cave, &passed_once, None);
    }
    println!("Part Two:\nPossible ways to move: {}", &counter)
}

