use std::io::Read;
use std::ops::{Index, IndexMut};
use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Cell {
    value: usize,
    from: (usize, usize),
    coord: (usize, usize)
}

impl Cell {
    fn new(value: usize, from: (usize, usize), coord: (usize, usize)) -> Cell {
        Cell {
            value,
            from,
            coord
        }
    }
}

#[derive(Clone, Debug)]
struct Cave {
    pub chitons: Vec<Cell>,
    width: usize,

}

impl Index<(usize, usize)> for Cave {
    type Output = Cell;
    
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.chitons[x + self.width*y]
    }
}

impl IndexMut<(usize, usize)> for Cave {

    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.chitons[x + self.width*y]
    }
}


impl Cave {
    
    fn new(w: usize) -> Cave{
        // Initialize structure with max distance for everything except first cell

        //let mut v = vec![Cell::new(usize::MAX, (0,0)); w * w];
        let mut v: Vec<Cell> = Vec::new();
        for i in 0..w*w {
            v.push(Cell::new(usize::MAX, (0,0), (i%w, i/w)))
        }
        v[0] = Cell::new(0, (0,0), (0,0));

        Cave {
            chitons: v,
            width: w,
        }
    }

    fn parse(buffer: String) -> Cave {
        // Create Cave structure from unaltered buffer

        let w =  buffer.lines().nth(0).unwrap().len(); // length of a line
        let mut i = 0usize;

        let vector: Vec<Cell> = buffer.lines()
            .map(|row| {

                row.chars().map(|c| {
                    let  c = Cell::new(c.to_digit(10).unwrap() as usize, ( i%w, i/w), ( i%w, i/w));
                    i += 1;
                    c
                }).collect::<Vec<Cell>>()

            }).flatten()
            .collect();

        Cave {
            chitons: vector,
            width: w,
        }
    }
    
    fn neighbors(&self, cell: Cell) -> HashSet<Cell> {
        let (x, y) = cell.coord;
        
        let min_x = match x.checked_sub(1) {
            Some(val) => val,
            None => 0
        };
        let max_x = self.width.min(x+2);

        let min_y = match y.checked_sub(1) {
            Some(val) => val,
            None => 0
        };
        let max_y = self.width.min(y+2);

        let mut res: HashSet<Cell> = HashSet::new();
        
        for j in min_y..max_y {
            res.insert(self[(x,j)]);
        }
        for i in min_x..max_x {
            res.insert(self[(i,y)]);
        }

        res
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let cave = Cave::parse(buf);
    part_one(&cave);
}


fn dijkstra(actual_cave: Cave) -> Cave{
    
    let mut target_cave = Cave::new(actual_cave.width);
    let mut passed: HashSet<Cell> = HashSet::new();

    for cell in &actual_cave.chitons {
        passed.insert(*cell);
        let neighbors = actual_cave.neighbors(*cell);
        let neighbors: HashSet<Cell> = neighbors.into_iter().filter(|x| !passed.contains(x)).collect();

        for n in neighbors {
            let n_coord = n.coord;
            let dist = target_cave[cell.coord].value + actual_cave[n_coord].value;
            
            if target_cave[n_coord].value > dist {
                target_cave[n_coord].value = dist;
                target_cave[n_coord].from = cell.from;
            }
        }
    }

    target_cave
}


fn part_one(actual_cave: &Cave) {
    
    let actual_cave = actual_cave.clone();
    let res = dijkstra(actual_cave);
    let res = res.chitons.last().unwrap();

    println!("Part One: {:?}", res.value);
}