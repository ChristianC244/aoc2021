use std::io::Read;
use std::collections::HashMap;
use std::ops::Index;

type HeightMap = Vec<usize>;
type PassedMap = Vec<usize>;
struct Map {
    pub map:HeightMap,
    pub len: usize,
    pub w: usize,
}
impl Map {
    
    pub fn new(m: &HeightMap, w: usize) -> Map{
        
        Map {
            len: m.len(),
            w,
            map: m.to_owned(),
        }
    }




    pub fn is_lower_point(&self, (coord_x, coord_y): (usize, usize)) -> bool {
        // low points := the locations that are lower than any of its adjacent locations

        let range_x = (coord_x as isize -1).max(0) as usize ..= (coord_x +1).min(self.w-1);
        let range_y = (coord_y as isize -1).max(0) as usize ..= (coord_y +1).min((self.len)/self.w -1);

        let curr = self[&(coord_x, coord_y)];
        let mut adj = vec![0usize; 0];
        
        for x in range_x {
            if x == coord_x {continue;}
            adj.push(self[&(x, coord_y)]);
        }
        
        for y in range_y {
            if y == coord_y {continue;}
            adj.push(self[&(coord_x, y)]);
        }


        if adj.iter().min().unwrap() <= &curr {false} else {true}
    }


}

impl Index<& (usize, usize)> for Map {
    type Output = usize;

    fn index(&self, (x, y): &(usize, usize) ) -> &usize {
        if  *x > self.w -1  {panic!("Index out of bound!")};
        if  *y > self.len / self.w -1 {panic!("Index out of bound!")};

        self.map.iter().skip(*x).step_by(self.w).collect::<Vec<&usize>>()[*y]
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
    part_two(&map, lp);

}


fn part_two(heightmap: &Map, low_points: HashMap<(usize, usize), usize>) {
   
    let mut track_map: PassedMap = heightmap.map.iter().map(|_|  0 ).collect();
    // let mut low_points: HashMap<(usize, usize), usize> = HashMap::new();
    let len = heightmap.len;
    let w = heightmap.w;
    let curr_basin = 1;
}

fn part_one(heightmap: &Map)  -> HashMap<(usize, usize), usize>{

    let mut low_points: HashMap<(usize, usize), usize> = HashMap::new();
    let len = heightmap.len;
    let w = heightmap.w;

    for i in 0..len {
        let (x, y) =  (i%w, i/w);

        if heightmap.is_lower_point((x,y)) {low_points.insert((x,y), heightmap[&(x,y)] );};
    }
    let res = low_points.iter().fold(0, |acc, x| acc + x.1 + 1 );

    println!("Part One: {}",res);
    
    low_points

} 
