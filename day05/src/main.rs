use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone)]
struct  Coord {
    pub start: (usize, usize),
    pub end: (usize, usize),
}
impl Coord {

    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self{
        Self {
            start: (x1, y1),
            end: (x2, y2)
        }
    }
    
}

struct  VentMap {
    map: HashMap<(usize, usize), usize>,
}
impl VentMap {

    pub fn new() -> Self {
        VentMap {
            map: HashMap::new() 
        }
    }

    pub fn add_fissure(&mut self, coord: &Coord)  {
        // Given a Coord it creates the map, in case of entry already present increase it's value by 1; 
        // Returns the Lenght of the fissure or None
        let (mut x1, mut y1) = coord.start;
        let (mut x2, mut y2) = coord.end;

        if x1 == x2 {
            // Move vertically
            if y1 > y2 {
                let tmp = y2;
                y2 = y1;
                y1 = tmp;
            }

            for y in y1..y2+1 {
                self.add(x1, y);
            }

        } else if y1 == y2 {
            // Move horizontally
            if x1 > x2 {
                let tmp = x2;
                x2 = x1;
                x1 = tmp;
            }

            for x in x1..x2+1 {
                self.add(x, y1);
            }

        }

    }

    fn add(&mut self, x: usize, y: usize) {
        *self.map.entry((x, y)).or_insert(0) += 1;
    }

    pub fn print_result(&self) {
        // dbg!(&self.map);
        let res = self.map.iter().filter(| ((_, _), n) | **n > 1usize ).count();
        println!("Final result: {}", res);
    }

    
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let input: Vec<Coord> = re.captures_iter(&buf)
        .map(|x| {
            
            let x1 = x[1].parse::<usize>().unwrap();
            let y1 = x[2].parse::<usize>().unwrap();
            let x2 = x[3].parse::<usize>().unwrap();
            let y2 = x[4].parse::<usize>().unwrap();
            
            Coord::new(x1, y1, x2, y2)
        
        })
        .collect();

    part_one(input.clone());
    
}

fn part_one(input: Vec<Coord>) {
    let mut sea_bed = VentMap::new();

    input.iter().for_each(|c| sea_bed.add_fissure(c) );
    sea_bed.print_result();
}
