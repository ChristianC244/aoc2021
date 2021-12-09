#[derive(Clone)]
pub struct Session {
    pub input: Vec<String>,
    pub output: Vec<String>,
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

// ------------------- PART TWO

#[derive(Clone, Debug)]
pub struct Single {
    //value: Option<char>,
    pub possibities: Vec<char>,
}
impl Single {
    fn new()-> Single {
        
        Single {
            //value: None,
            possibities: ['a', 'b', 'c', 'd', 'e', 'f', 'g'].to_vec()
        }
    }
}
#[derive(Debug)]
pub struct  Solve {
    vector: Vec<Single>,
}
impl Solve {
    pub fn new() -> Solve {
        
        Solve { vector: vec![Single::new(); 7] }
    }

    pub fn update_one(&mut self, one: &str ) {
        // 1 has segments in position 2,5
        self.vector.iter_mut()
            .enumerate()
            .for_each(| (i, val) |{

                let v: Vec<char> = match i {
                    2 | 5 =>  val.possibities.iter()
                        .filter(| x | **x == one.chars().nth(0).unwrap() || **x == one.chars().nth(1).unwrap() )
                        .map(|x| *x)
                        .collect(), // remove every char different from ones in one: &str

                    _ => val.possibities.iter()
                        .filter(| x | **x != one.chars().nth(0).unwrap() && **x != one.chars().nth(1).unwrap())
                        .map(|x| *x)
                        .collect(), // remove every char equal to ones in one: &str
                };
                val.possibities = v;
            });
    }

    pub fn update_seven(&mut self, seven: &str) {
        // 7 has segments 0,2,5
        self.vector.iter_mut()
            .enumerate()
            .for_each(| (i, val) |{

                let v: Vec<char> = match i {
                    0|2|5 =>  val.possibities.iter()
                        .filter(| x | **x == seven.chars().nth(0).unwrap() 
                            || **x == seven.chars().nth(1).unwrap() 
                            || **x == seven.chars().nth(2).unwrap())
                        .map(|x| *x)
                        .collect(), // remove every char different from ones in one: &str

                    _ => val.possibities.iter()
                        .filter(| x | **x != seven.chars().nth(0).unwrap() 
                            && **x != seven.chars().nth(1).unwrap()
                            && **x != seven.chars().nth(2).unwrap())
                        .map(|x| *x)
                        .collect(), // remove every char equal to ones in one: &str
                };
                val.possibities = v;
            });
    }

    pub fn update_four(&mut self, four: &str) {
        // 4 has segments 1,2,3,5
        self.vector.iter_mut()
            .enumerate()
            .for_each(| (i, val) |{

                let v: Vec<char> = match i {
                    1|2|3|5 =>  val.possibities.iter()
                        .filter(| x | **x == four.chars().nth(0).unwrap() 
                            || **x == four.chars().nth(1).unwrap() 
                            || **x == four.chars().nth(2).unwrap()
                            || **x == four.chars().nth(3).unwrap())
                        .map(|x| *x)
                        .collect(), // remove every char different from ones in one: &str

                    _ => val.possibities.iter()
                        .filter(| x | **x != four.chars().nth(0).unwrap() 
                            && **x != four.chars().nth(1).unwrap()
                            && **x != four.chars().nth(2).unwrap()
                            && **x != four.chars().nth(3).unwrap())
                        .map(|x| *x)
                        .collect(), // remove every char equal to ones in one: &str
                };
                val.possibities = v;
            });    
    }

    pub fn generate_all_poss(&self) -> Vec<Single>{

        let mut  v: Vec<Single> = Vec::new();
        for i in 0..64 {
           let x =  [self.vector[0].possibities[i / 64 % 2] ,
            self.vector[1].possibities[i / 32 % 2],
            self.vector[2].possibities[i / 16 % 2],
            self.vector[3].possibities[i / 8 % 2],
            self.vector[4].possibities[i / 4 % 2],
            self.vector[5].possibities[i / 2 % 2],
            self.vector[6].possibities[i % 2]].to_vec();

            v.push(Single { possibities: x.clone()});
        }

        return v
    }
}
