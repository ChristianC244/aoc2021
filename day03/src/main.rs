use std::cmp::Ordering;
use std::io::Read;

type BitVec = Vec<bool>;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let buf: Vec<&str> = buf.lines().collect();
    
    part_one(&buf);
    

    let buf:Vec<BitVec> = buf.iter().map(|x| {
        x.chars().map(|c| c == '1').collect()
    }).collect();

    part_two(&buf);
    
}

fn binstr2dec(vec: Vec<usize>) -> usize {
    let numstr = vec.iter()
        .map(|x| x.to_string())
        .collect::<String>();
    
    usize::from_str_radix(&numstr, 2).unwrap()
}

fn most_common(vec: &Vec<BitVec>, index: usize) -> bool {

    assert!(index < vec[0].len());

    let len = vec.len();

    let sum: usize = vec.iter()
        .filter(|x| x[index])
        .count();

    2 * sum >= len 
    
}

fn bin2int(bin: &BitVec) -> usize {

    let numstr = bin.iter().map(| x | if *x {'1'} else {'0'}).collect::<String>();
    usize::from_str_radix(&numstr, 2).unwrap()

}


fn part_one(buf: &Vec<&str>) {
    
    let half_len: u32 = (buf.len()/ 2).try_into().unwrap();
    let vec_len = buf[0].len();
    
    let sumvector = buf.iter()
        .fold(vec![0; vec_len], |acc:Vec<u32>, x| {
           
            acc.iter()
                .zip(x.chars())
                .map(|(a,c)| a + c.to_digit(10).unwrap())
                .collect()

        });
    
    let gamma: Vec<usize> = sumvector
        .iter()
        .map(|x| {
            
            match x.cmp(&half_len){
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


fn part_two(vec: &Vec<BitVec>) {
    
    println!("PART TWO");
    let mut o2_vec = vec.clone();
    let mut co2_vec = vec.clone();
    let vec_len = vec[0].len();
    
    for i in 0..vec_len {

        let mc = most_common(&o2_vec, i);
        let lc = !most_common(&co2_vec, i);

        if o2_vec.len() != 1 {
            o2_vec = o2_vec.iter()
                .cloned()
                .filter(| x | x[i] == mc)
                .collect();
        }
        if  co2_vec.len() != 1 {
            co2_vec = co2_vec.iter()
            .cloned()
            .filter(| x | x[i] == lc)
            .collect();            
        }

        if o2_vec.len() == 1 && co2_vec.len() == 1 {break;}
    }   

    assert_eq!(o2_vec.len(), 1);
    assert_eq!(co2_vec.len(), 1);
    
    println!("Final Result: {}", bin2int(&o2_vec[0]) * bin2int(&co2_vec[0]));

}