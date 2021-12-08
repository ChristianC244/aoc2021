use std::io::Read;

struct Session {
    input: Vec<String>,
    output: Vec<String>,
}

impl Session {
    pub fn parse(line: &str) -> Session{
        let line: Vec<&str> = line.clone()
            .split(" | ")
            .collect();

        assert_eq!(line.len(),2);

        let input: Vec<String> = line[0].split(" ")
            .map(| x | x.to_string())
            .collect();
        let output: Vec<String> = line[1].split(" ")
            .map(| x | x.to_string())
            .collect();

        Session {
            input: input,
            output: output
        }
    }
    pub fn count_ez_digit(&self) -> usize {
        
        self.output.iter().filter(| x | {
            match x.len() {
                2 => true,
                3 => true,
                4 => true,
                7 => true,
                _ => false,
            }
        }).count()

    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let input: Vec<Session> = buf.lines().map(| x | Session::parse(x)).collect();
    part_one(&input);

}

fn part_one(input: &Vec<Session>) {
    println!("PART ONE");

    let res = input.iter().fold(0, |acc, x| acc + x.count_ez_digit());

    println!("{}",res)

}
