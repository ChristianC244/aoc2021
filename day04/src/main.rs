use std::io::Read;
use std::collections::HashMap;
use std::usize;
use regex::Regex;


type Matrix = Vec<Vec<bool>>;

#[derive(Clone)]
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

    part_one(&calls, &mut cards.clone());
    part_two(&calls, &mut cards);
    

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

fn print_result(card: &Card, win_card:usize ) {
    let res = card.numbers.iter()
    // Calculate sum of remaining numbers
    .fold(0,| mut acc, (k, (x,y)) | 
        {
            if card.grid[*x][*y]==false { acc += k}
            acc
        }
    );

    println!("{}",res*win_card);
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

    print_result(&cards[winner as usize], win_card)
    
}

fn part_two(calls: &Vec<usize>, cards: &mut Vec<Card>){

    let mut track_cards = vec![false; cards.len()];

    let mut last: isize = -1;
    let mut last_card: isize = -1;

    for n in calls {
        for (i,c) in cards.iter_mut().enumerate() {
            if check_call(n, c) {
                track_cards[i] = true;

                if track_cards.iter().filter(|x| **x == false).count() == 0 
                {
                    last = i as isize;
                    last_card = *n as isize;
                    break;
                } 
            }
        }
        if last != -1 {break;}
    }

    dbg!(last_card);
    print_result(&cards[last as usize], last_card as usize);

}