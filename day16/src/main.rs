use core::panic;
use std::io::Read;
use std::usize;

#[derive(Debug, Clone, Copy)]
struct Num {
    ver: u8,
    _t_id: u8,
    value: usize,
    len: usize,
}
impl Num {
    fn new(ver: u8, _t_id: u8, value: usize, len: usize) -> Num {
        Num {
            ver,
            _t_id,
            value,
            len,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Op {
    ver: u8,
    t_id: u8,
    b: bool, // length type id
    tail: usize,
    len: usize,
}
impl Op {
    fn new(ver: u8, t_id: u8, b: bool, tail: usize, len: usize) -> Op {
        Op {
            ver,
            t_id,
            b,
            tail,
            len,
        }
    }
}

#[derive(Debug)]
enum Cmd {
    Num(Num),
    Op(Op),
}

fn hex_to_bin(hex: &str) -> String {
    hex.chars()
        .map(|h| match h.to_digit(16) {
            Some(val) => format!("{:04b}", val),
            None => "".to_string(),
        })
        .collect::<String>()
}

fn command_finder(bin_str: &str) -> Option<Cmd> {
    // returns command parsed

    if bin_str.len() < 11 {
        return None;
    } // 11 is minimum length to have a Num

    let ver = u8::from_str_radix(&bin_str[0..3], 2).unwrap();
    let t_id = u8::from_str_radix(&bin_str[3..6], 2).unwrap();

    let cmd = match t_id {
        4 => {
            let mut check = true;
            let value_str: String = bin_str
                .char_indices()
                .skip(6)
                .step_by(5)
                .take_while(|(_, c)| {
                    if !check {
                        return check;
                    };
                    if c.eq(&'0') {
                        check = false;
                        return true;
                    };
                    check
                })
                .map(|(i, _)| &bin_str[i + 1..i + 5])
                .collect();
            let value = usize::from_str_radix(&value_str, 2).unwrap();
            let len = (5 * value_str.len() / 4 + 6) as usize;
            Cmd::Num(Num::new(ver, t_id, value, len))
        }
        _ => match &bin_str.chars().nth(6).unwrap() {
            '0' => {
                let len = (7 + 15) as usize;
                let tail = usize::from_str_radix(&bin_str[7..len], 2).unwrap();
                Cmd::Op(Op::new(ver, t_id, false, tail, len))
            }
            '1' => {
                let len = (7 + 11) as usize;
                let tail = usize::from_str_radix(&bin_str[7..len], 2).unwrap();
                Cmd::Op(Op::new(ver, t_id, true, tail, len))
            }
            _ => panic!(),
        },
    };
    Some(cmd)
}

fn command_parser(bin_str: &str) -> Vec<Cmd> {
    let len = bin_str.len();
    let mut curr = 0usize;
    let mut res: Vec<Cmd> = Vec::new();

    while curr < len {
        let cmd = match command_finder(&bin_str[curr..]) {
            Some(val) => val,
            None => break,
        };
        let skip = match &cmd {
            Cmd::Num(val) => val.len,
            Cmd::Op(val) => val.len,
        };
        curr += skip;
        res.push(cmd);
    }
    res
}

fn operate(op: &Op, members: Vec<Num>) -> Num {
    let op = *op;
    let len = members.iter().map(|x| x.len).sum::<usize>() + op.len;
    let res = match op.t_id {
        0 => {
            let sum = members.iter().map(|x| x.value).sum();
            Num::new(0, 4, sum, len)
        }
        1 => {
            let prod = members.iter().map(|x| x.value).product();
            Num::new(1, 4, prod, len)
        }
        2 => {
            let value = members.into_iter().min_by_key(|x| x.value).unwrap().value;
            Num::new(2, 4, value, len)
        }
        3 => {
            let value = members.into_iter().max_by_key(|x| x.value).unwrap().value;
            Num::new(3, 4, value, len)
        }
        5 => {
            if members.len() != 2 {
                panic!("Cannot compare, do not have 2 values")
            }
            if members[0].value > members[1].value {
                Num::new(5, 4, 1, len)
            } else {
                Num::new(5, 4, 0, len)
            }
        }
        6 => {
            if members.len() != 2 {
                panic!("Cannot compare, do not have 2 values")
            }
            if members[0].value < members[1].value {
                Num::new(6, 4, 1, len)
            } else {
                Num::new(6, 4, 0, len)
            }
        }
        7 => {
            if members.len() != 2 {
                panic!("Cannot compare, do not have 2 values")
            }
            if members[0].value == members[1].value {
                Num::new(7, 4, 1, len)
            } else {
                Num::new(7, 4, 0, len)
            }
        }
        _ => panic!("Wrong type id"),
    };
    res
}
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let input = hex_to_bin(&buf);
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let cmds = command_parser(input);
    let counter = cmds.iter().fold(0usize, |acc, cmd| {
        acc + match cmd {
            Cmd::Num(val) => val.ver as usize,
            Cmd::Op(val) => val.ver as usize,
        }
    });
    println!("Part One: {}", counter);
}

fn part_two(input: &str) {
    let cmds = command_parser(input);
    let mut cmds = cmds.into_iter().map(Some).collect::<Vec<Option<Cmd>>>();

    let mut curr_op_index = 0; // Index of Operation packet
    let mut curr_op = match cmds[curr_op_index].as_ref().unwrap() {
        Cmd::Op(val) => *val,
        Cmd::Num(_) => panic!(),
    };

    let mut i = 1; // index used to check Nums for Operation
    let mut vec_nums: Vec<Num> = Vec::new(); // Store for sub-packet of Operation
    let mut filling = match &cmds[curr_op_index].as_ref().unwrap() {
        // bits or packets required for Operation
        Cmd::Op(val) => val.tail,
        Cmd::Num(_) => panic!(),
    };

    loop {
        let res = match &cmds[i] {
            Some(pkt) => {
                match pkt {
                    Cmd::Num(val) => {
                        vec_nums.push(*val);
                        i += 1;

                        if !curr_op.b {
                            filling -= val.len
                        }
                        // need to take 'filling' bits
                        else {
                            filling -= 1
                        } // need to take 'filling' packets

                        if filling == 0 {
                            // We have all the sub-packet for the Operation
                            let ret = operate(&curr_op, vec_nums);
                            vec_nums = Vec::new();
                            Some(Cmd::Num(ret))
                        } else {
                            None
                        }
                    }
                    Cmd::Op(val) => {
                        vec_nums = Vec::new();
                        curr_op_index = i;
                        curr_op = *val;
                        filling = curr_op.tail;
                        i += 1;
                        None
                    }
                }
            }
            None => {
                i += 1;
                None
            }
        };

        if let Some(val) = res {
            // Update the cmds only if an operation has been made
            // Putting 'None' where Nums were and The result (Cmd::Num) where the operation were
                cmds[curr_op_index] = Some(val);
                cmds.iter_mut().take(i).skip(curr_op_index + 1).for_each(|x| *x = None);

                if curr_op_index == 0 {
                    break;
                }
                // Back to previous operation
                curr_op_index = cmds[0..curr_op_index]
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| match x {
                        Some(Cmd::Op(val)) => {
                            curr_op = *val;
                            true
                        }
                        _ => false,
                    })
                    .map(|(i, _)| i)
                    .last()
                    .unwrap();
                filling = curr_op.tail;
                i = curr_op_index + 1;
            }
        }
    
    println!("Part Two: {:?}", cmds[0].as_ref().unwrap());
}
