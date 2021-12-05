use std::io::Read;
use std::collections::HashMap;
use std::usize;
use regex::Regex;


type Matrix = Vec<Vec<bool>>;

#[derive(Debug)]
struct Card {
    numbers: HashMap<usize, (usize,usize)>,
    grid: Matrix,
}

fn main() {
    //Assuming height and witdh is same = 5

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let input:Vec<&str> = buf.split("\n\n").collect();

    let calls:Vec<usize> = input[0].split(",")
        .map(| x | x.parse::<usize>().unwrap())
        .collect();

    let mut cards: Vec<Card> = Vec::new();

    for i in 1..input.len() {
        cards.push(str_to_card(&input[i]))
    }

    part_one(&calls, &mut cards);
    

}

fn str_to_card(input: &str)  -> Card{

    let re = Regex::new(r"(\d+)").unwrap();
    let numbers: Vec<usize> = re.captures_iter(input)
        .map(| x | x[1].parse::<usize>().unwrap())
        .collect();

    let mut grid = Matrix::new();
    for _ in 0..5 {
        grid.push(vec![false; 5]);
    };

    let mut map = HashMap::new();
    numbers.iter()
    .zip(0..numbers.len() as usize)
    .for_each(|(val, i)| {
        map.insert(*val, (i / 5,i % 5));
    }); 
    
    Card {
        numbers : map, 
        grid : grid,
    }

} 


fn check_call(n: &usize, card: &mut Card) -> bool{
    // True if win
    if !card.numbers.contains_key(&n) {return false};
    let (x,y) = card.numbers.get(n).unwrap();

    card.grid[*x][*y] = true;

    check_win(x, y, card)
}

fn check_win(x: &usize, y: &usize, card: &Card) -> bool{
    
    let mut win = true;
    for i in 0..5 {

        win &= card.grid[*x][i];
    }
    if win {return true}
    
    win=true;
    for i in 0..5 {
        win &= card.grid[i][*y]; 
    }
    if win {return true}
    
    false

}

fn part_one(calls: &Vec<usize>, cards: &mut Vec<Card>) {
    let mut winner:isize = -1;
    let mut win_card= 0;
    for n in calls {
        for (i, c) in cards.iter_mut().enumerate() {
            if check_call(&n,  c) {
                winner = i as isize;
                win_card = *n;
                break;
            }
        }
        if winner != -1 {break;}
    }

    assert_ne!(winner,-1);

    let winner:usize = winner as usize;

    let res = cards[winner].numbers.iter()
    // Calculate sum
    .fold(0,| mut acc, (k, (x,y)) | 
        {
            if cards[winner].grid[*x][*y]==false { acc += k}
            acc
        }
    );

    println!("{}",res*win_card);
}