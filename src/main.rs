extern crate permutator;

#[macro_use] extern crate scan_fmt;
//use permutator::{CartesianProduct, Combination, Permutation};
use permutator::copy::{CartesianProduct, Combination, Permutation};
use std::io;

fn gen_opers(operation: &[char]) -> Vec<Vec<char>> {
    let mut ret: Vec<Vec<char>> = Vec::new();
    let opts = vec![operation, operation, operation];

    opts.cart_prod().for_each(|item| {
        ret.push(item);
    });

    ret
}

fn apply_operation(a: i32, b: i32, o: char) -> i32 {
    match o {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            let c = a / b;
            if c * b == a {
                c
            } else {
                -1
            }
        },
        _ =>  panic!("oh no!"),
    }
}

fn apply_to_array(item: &[i32], operation: Vec<char>) {
    let mut ret = item[0];
    let mut valid = true;
    let iter = item.iter().skip(1);

    iter.enumerate().for_each(|(index, &item)| {
        ret = apply_operation(ret, item, operation[index]);
        if ret == -1 {
            valid = false;
        }
    });
    if valid && (ret == 24 || ret == -24) {
        println!("Found! {:?} {:?}", item, operation);
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
