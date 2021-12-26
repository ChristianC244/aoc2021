use std::io::Read;
use regex::Regex;
use itertools::Itertools;
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut dots: Vec<[usize; 2]> = Vec::new();
    let mut commands: Vec<(char, usize)> = Vec::new();
    
    let dots_finder = Regex::new( r"(\d+),(\d+)").unwrap();
    let commands_finder = Regex::new(r"fold along (\w)=(\d+)").unwrap();

    for cap in dots_finder.captures_iter(&buf) {        
        dots.push( [cap[1].parse::<usize>().unwrap(), 
            cap[2].parse::<usize>().unwrap() ]);
    }
    
    for cap in commands_finder.captures_iter(&buf) {        
        commands.push( (cap[1].to_string().chars().nth(0).unwrap(), 
            cap[2].parse::<usize>().unwrap() ));
    }

    part_one(&dots, commands[0]);
    part_two(&dots, commands);
}

fn fold(dots: Vec<[usize; 2]>, command: (char, usize)) ->  Vec<[usize; 2]>{
    let mut dots = dots.clone();
    
    let index = match command.0 {  
        'x' => 0,
        'y' => 1,
        _ => panic!()
    };

    dots.iter_mut()
        .for_each(|x| if x[index] > command.1 { x[index] = command.1 - ( x[index] - command.1) });
    
    dots.into_iter().unique().collect()
}

fn print_grid(dots: Vec<[usize; 2]>) {
    let max_x = dots.iter().max_by_key(|x| x[0]).unwrap()[0];
    let max_y = dots.iter().max_by_key(|x| x[1]).unwrap()[1];

    let row = vec!['.'; max_x + 1];
    let mut grid: Vec<Vec<char>> = vec![row.clone(); max_y+1];

    dots.into_iter().for_each(|[x,y]| grid[y][x] = '#' );

    for line in grid {
        for c in line {
            print!("{}",c);
        }
        println!();
    }

}


fn part_one(dots: &Vec<[usize; 2]>, command: (char, usize)) {
    
    let res = fold(dots.clone(), command);
    println!("Part One: {}", res.iter().count());
}

fn part_two(dots: &Vec<[usize; 2]>, commands: Vec<(char, usize)>) {
    
    let mut dots = dots.clone();

    for c in commands {
        dots = fold(dots, c);
    }

    print_grid(dots)
}