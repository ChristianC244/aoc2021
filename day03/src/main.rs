use std::cmp::Ordering;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mean: u32 = 500; // 1000/2
    
    let sumvector = buf.lines()
        .fold(vec![0; 12], |acc:Vec<u32>, x| {
           
            acc.iter()
                .zip(x.chars())
                .map(|(a,c)| a + c.to_digit(10).unwrap())
                .collect()

        });
    
    let gamma: Vec<usize> = sumvector
        .iter()
        .map(|x| {
            
            match x.cmp(&mean){
            Ordering::Less => 0,
            Ordering::Greater => 1,
            _ => unreachable!()
            }
        })
        .collect();

    let epsilon: Vec<usize> = gamma.iter()
        .map(|x| (x + 1)%2)
        .collect();

        println!("gamma:   {:?}\nepsilon: {:?}", &gamma,&epsilon);
        println!("Final result: {}", binstr2dec(gamma) * binstr2dec(epsilon) )
    
}

fn binstr2dec(vec: Vec<usize>) -> usize {
    let numstr = vec.iter()
        .map(|x| x.to_string())
        .collect::<String>();
    
    usize::from_str_radix(&numstr, 2).unwrap()
}

