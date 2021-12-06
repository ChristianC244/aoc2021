use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone)]
struct  Coord {
    pub start: (i32, i32),
    pub end: (i32, i32),
}
impl Coord {

    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self{
        Self {
            start: (x1, y1),
            end: (x2, y2)
        }
    }
    // pub fn is_ordered(&self) -> bool {
    //     let (x1, y1) = self.start;
    //     let (x2, y2) = self.end;

    //     if y1 < y2 { return true}
    //     else if y1 == y2 {return x1 < x2}
    //     false
    // }
    
}

struct  VentMap {
    map: HashMap<(i32, i32), i32>,
}
impl VentMap {

    pub fn new() -> Self {
        VentMap {
            map: HashMap::new() 
        }
    }

    pub fn add_fissure_one(&mut self, coord: &Coord)  {
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

    pub fn add_fissure_two(&mut self, coord: &Coord) {
        let (mut x1, mut y1) = coord.start;
        let (x2,  y2) = coord.end;

        if x1 == x2 || y1 == y2 {self.add_fissure_one(coord)}
        else {
            // Move diagonally
            let mut delta_x: i32 = x2 - x1;
            delta_x = delta_x / delta_x.abs();

            let mut delta_y: i32 = y2 - y1;
            delta_y = delta_y / delta_y.abs();

            self.add(x1, y1);
            while x1 != x2 {
                x1 += delta_x;
                y1 += delta_y;

                self.add(x1, y1);
            }
        }
    }

    fn add(&mut self, x: i32, y: i32) {
        *self.map.entry((x, y)).or_insert(0) += 1;
    }

    pub fn print_result(&self) {
        // dbg!(&self.map);
        let res = self.map.iter().filter(| ((_, _), n) | **n > 1i32 ).count();
        println!("Final result: {}", res);
    }   
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let input: Vec<Coord> = re.captures_iter(&buf)
        .map(|x| {
            
            let x1 = x[1].parse::<i32>().unwrap();
            let y1 = x[2].parse::<i32>().unwrap();
            let x2 = x[3].parse::<i32>().unwrap();
            let y2 = x[4].parse::<i32>().unwrap();
            
            Coord::new(x1, y1, x2, y2)
        
        })
        .collect();

    part_one(input.clone());
    part_two(input);
    
}

fn part_one(input: Vec<Coord>) {
    println!("PART ONE");
    let mut sea_bed = VentMap::new();

    input.iter().for_each(|c| sea_bed.add_fissure_one(c) );
    sea_bed.print_result();
}

fn part_two(input: Vec<Coord>) {
    println!("PART TWO");
    let mut sea_bed = VentMap::new();

    input.iter().for_each(|c| sea_bed.add_fissure_two(c) );
    sea_bed.print_result();
}
