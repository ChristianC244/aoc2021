use std::io::Read;


fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let input: Vec<Vec<u32>> = buf.lines()
        .map(|x| {

            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    part_one(&input, 100);
    part_two(&input);
}


fn add_one_mut(grid: &mut Vec<Vec<u32>>) {

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            grid[y][x] += 1
        }
    }
}


fn flash(grid: &mut Vec<Vec<u32>>, x: usize, y: usize) {

    if grid[y][x] == 0 {}
    else if grid[y][x] > 9{
        // if this value is bigger than 9
        
        // this cell set to zero
        grid[y][x] = 0;


        // find window dimension
        let min_x = match x.checked_sub(1) {
            Some(val) => val,
            None => 0            
        };
        let max_x = usize::min(x+2, grid.len());

        let min_y = match y.checked_sub(1) {
            Some(val) => val,
            None => 0            
        };
        let max_y = usize::min(y+2, grid.len());


        for i in min_y..max_y {
            for j in min_x..max_x {
            // increase by 1 value of adj cells if != 0
            // call flash() to every adjiacent cell
                if grid[i][j] != 0 {
                    grid[i][j] += 1;
                    flash(grid, j, i)
                }

            }
        }
    }

}




fn part_one(grid: &Vec<Vec<u32>>, steps: u32) {
    let mut grid = grid.clone();
    let mut counter = 0usize;

    for _ in 0..steps {

        add_one_mut(&mut grid);

        for i in 0..grid.len() * grid.len() {
            let (x,y) = (i%grid.len(), i/grid.len());
            flash(&mut grid, x,y)
        }

        counter += grid.iter()
            .map(|row| {
                row.iter().filter(|x| **x == 0).count()
            })
            .sum::<usize>();

    };

    println!("Part One: {}", counter);
}

fn part_two(grid: &Vec<Vec<u32>>) {
    let mut grid = grid.clone();
    let mut i:usize = 0;

    loop {
        i +=1;

        add_one_mut(&mut grid);

        for i in 0..grid.len() * grid.len() {
            let (x,y) = (i%grid.len(), i/grid.len());
            flash(&mut grid, x,y)
        }

        let counter =  grid.iter()
            .map(|row| {
                row.iter().filter(|x| **x == 0).count()
            })
            .sum::<usize>();

        if counter == grid.len() * grid.len() {
            println!("Part One: {}",i );
            break;
        }

    };

}