use std::io::Read;
use std::collections::{HashMap, VecDeque};
use std::ops::{Index, IndexMut};
use std::fmt;

type HeightMap = Vec<usize>;

#[derive(Clone)]
struct Map {
    pub map:HeightMap,
    pub len: usize,
    pub w: usize,
}
impl Map {
    
    pub fn new(m: &HeightMap, w: usize) -> Map {
        
        Map {
            len: m.len(),
            w,
            map: m.to_owned(),
        }
    }

    pub fn find_adj_min(&self,(coord_x, coord_y): (usize, usize)) -> Option<(usize, usize)> {

        let range_x = (coord_x as isize -1).max(0) as usize ..= (coord_x +1).min(self.w-1);
        let range_y = (coord_y as isize -1).max(0) as usize ..= (coord_y +1).min((self.len)/self.w -1);

        let mut min = self[(coord_x, coord_y)];
        let mut min_coord: (usize, usize) = (0, 0);
        let mut changed= false;
        
        for x in range_x {
            if self[(x, coord_y)] < min {
                min_coord = (x, coord_y);
                min = self[min_coord];
                changed = true;
            }
        }

        for y in range_y {
            if self[(coord_x, y)] < min {
                min_coord = (coord_x, y);
                min = self[min_coord];
                changed = true;
            }
        }
        if changed && min != 9 {Some(min_coord)} else {None}
    }

    pub fn is_lower_point(&self, (coord_x, coord_y): (usize, usize)) -> bool {
        // low points := the locations that are lower than any of its adjacent locations

        let range_x = (coord_x as isize -1).max(0) as usize ..= (coord_x +1).min(self.w-1);
        let range_y = (coord_y as isize -1).max(0) as usize ..= (coord_y +1).min((self.len)/self.w -1);

        let curr = self[(coord_x, coord_y)];
        let mut adj = vec![0usize; 0];
        
        for x in range_x {
            if x == coord_x {continue;}
            adj.push(self[(x, coord_y)]);
        }
        
        for y in range_y {
            if y == coord_y {continue;}
            adj.push(self[(coord_x, y)]);
        }

        if adj.iter().min().unwrap() <= &curr {false} else {true}
    }
}

impl Index< (usize, usize)> for Map {
    type Output = usize;

    fn index(&self, (x, y): (usize, usize) ) -> &usize {
        if  x > self.w -1  {panic!("Index out of bound!")};
        if  y > self.len / self.w -1 {panic!("Index out of bound!")};

        self.map.iter().skip(x).step_by(self.w).collect::<Vec<&usize>>()[y]
    }
}
impl IndexMut< (usize, usize)> for Map {

    fn index_mut (&mut self, (x, y): (usize, usize) ) -> &mut Self::Output {
        if  x > self.w -1  {panic!("Index out of bound!")};
        if  y > self.len / self.w -1 {panic!("Index out of bound!")};

        &mut self.map[x + self.w * y]
    }
}

impl fmt::Display for Map {
    //Implemented for debug porpouses
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self.map.windows(self.w)
            .step_by(self.w)
            .map(|w| 
                w.iter()
                .map(|c| c.to_string())
                .collect::<String>() + "\n"
            ).collect::<String>();
        
        write!(f, "{}", string)
    }
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let w = &buf.lines().collect::<Vec<&str>>()[0].len();
    let input: HeightMap = buf.chars()
        .filter_map(| x | x.to_digit(10u32) )
        .map(|x| x as usize)
        .collect::<Vec<usize>>();

    let map = Map::new(&input, *w);
    
    let lp = part_one(&map); 
    part_two(map, &lp);
}

fn part_two(heightmap: Map, low_points: &HashMap<(usize, usize), usize>) {
   
    let mut track_map: Map = heightmap.clone();
    track_map.map.iter_mut().for_each(|x| *x = 0usize);
    low_points.iter()
        .enumerate()
        .for_each(|(i, (k, _))| track_map[*k] = i+1);
        // Each path leading to a basin will end with the same index used for the basin
    
    let mut curr_path: VecDeque<(usize, usize)> = VecDeque::new();
    let len = heightmap.len;
    let w = heightmap.w;
    
    let mut i = 0;
    let mut coord =  (i, i);

    while i<len {

        if track_map[coord] != 0 {
            // We are in a path already connected to a basin or to a basin
            let basin = track_map[coord];
            curr_path.drain(0..)
                .filter(|x| heightmap[*x] != 9)
                .for_each(|x| track_map[x] = basin);

            i+=1;
            coord = (i%w, i/w);
            continue;
        }

        coord = match heightmap.find_adj_min(coord) {
            
            Some(min_coord) => {
                curr_path.push_back(coord);
                min_coord
            },
            None => {
                // We are in a height 9 cell
                curr_path.drain(0..);
                i+=1;
                (i%w, i/w)
            }
        };
    }

    let basins = low_points.len()+1;
    
    let mut counter = track_map.map.iter().fold(vec!(0usize; basins), |mut acc, x| {
        acc[*x] += 1;
        acc
    });
    
    counter[1..].sort_by_key(|x| usize::MAX - x);
    let max = &counter[1..4];
    //println!("{}", track_map);
    println!("Part two: {} * {} * {} = {}", max[0], max[1], max[2], max[0]*max[1]*max[2])
}

fn part_one(heightmap: &Map)  -> HashMap<(usize, usize), usize>{

    let mut low_points: HashMap<(usize, usize), usize> = HashMap::new();
    let len = heightmap.len;
    let w = heightmap.w;

    for i in 0..len {
        let (x, y) =  (i%w, i/w);

        if heightmap.is_lower_point((x,y)) {low_points.insert((x,y), heightmap[(x,y)] );};
    }
    let res = low_points.iter().fold(0, |acc, x| acc + x.1 + 1 );

    println!("Part One: {}",res);
    
    low_points
} 
