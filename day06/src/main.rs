use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let init_fishes  = buf.split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .fold(vec![0;9], |mut acc: Vec<usize>, x| {
            acc[x] += 1;
            acc
        });
    
    let fishes = VecDeque::from(init_fishes);

    print!("PART ONE: ");
    final_result(&mut fishes.clone(), 80usize);
    print!("PART TWO: ");
    final_result(&mut fishes.clone(), 256usize);
}

fn final_result(fishes: &mut VecDeque<usize>, days: usize) {
    
    for _ in 0..days {
        let parents = fishes.pop_front().unwrap();
        fishes[6] += parents;
        fishes.push_back(parents);
    }
    let res: usize = fishes.iter().sum();

    println!("{}",res);
}
