extern crate permutator;

#[macro_use] extern crate scan_fmt;
//use permutator::{CartesianProduct, Combination, Permutation};
use permutator::copy::{CartesianProduct, Permutation};
use std::io;

fn gen_opers(operation: &[char]) -> Vec<Vec<char>> {
    let mut ret: Vec<Vec<char>> = Vec::new();
    let opts = vec![operation, operation, operation];

    opts.cart_prod().for_each(|item| {
        ret.push(item);
    });

    ret
}

fn apply_operation(a: i32, b: i32, o: char) -> Result<i32, i32> {
    match o {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            let c = a / b;
            if c * b == a {
                Ok(c)
            } else {
                Err(-1)
            }
        },
        _ =>  panic!("oh no!"),
    }
}

fn apply_to_array(item: &[i32], operation: Vec<char>) {
    let acc = item[0];
    let nums = item.iter().skip(1);

    let ret = nums.enumerate().try_fold(acc, |acc, (index, &item)| {
        apply_operation(acc, item, operation[index])
    });
    if let Ok(sum)  = ret {
        if sum == 24 || sum == -24 {
            println!("{:?} => {:?}", item, operation);
        }
    }
}

fn main() {
    let operation = ['+', '-', '*', '/'];
    let ops = gen_opers(&operation);
    let mut input = String::new();

    println!("Plese input 4 numbers:");
    io::stdin().read_line(&mut input).unwrap();

    let (a,b,c,d) = scan_fmt!(&input, "{d} {d} {d} {d}", i32, i32, i32, i32);
    let mut list: Vec<i32> = vec![a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap()];

    ops.iter().for_each(|op| {
        apply_to_array(&list[..], op.to_vec());
    });
    list.permutation().for_each(|p| {
        ops.iter().for_each(|op| {
            apply_to_array(&p, op.to_vec());
        });
    });
}
