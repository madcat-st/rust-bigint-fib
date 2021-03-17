use std::io;
use std::io::Write;
use std::str::FromStr;

use num_bigint::*;

fn fib_big_simple(n: u32) -> BigInt {
    if n < 2 {
        n.to_bigint().unwrap()
    } else {

        let mut a = 0.to_bigint().unwrap();
        let mut b = 1.to_bigint().unwrap();
        let mut c = 0.to_bigint().unwrap();

        for _ in 2..n {
            c = a + b.clone();
            a = b;
            b = c.clone();
        }
        return c;
    }
}


fn fib(n: u32, memo: &mut Vec<u64>) ->  u64 {
    if n < 2 {
        return n as u64;
    }
    else if memo[(n-1) as usize] != 0 {
        return memo[(n-1) as usize];
    }

    memo[(n-1) as usize] = fib(n - 1, memo) + fib(n - 2, memo);
    memo[(n-1) as usize]
}

fn main() {
    loop {
        print!("n = ");
        io::stdout().flush().unwrap();

        let mut str: String = String::new();
        io::stdin().read_line(&mut str)
            .expect("Error getting number");

        str.pop();
        match u32::from_str(&str) {
            Ok(n) => {
                // let mut memo: Vec<u64> = vec![0; n as usize];
                // println!("fib({})={}", n, fib(n, &mut memo));
                println!("fib({})={}", n, fib_big_simple(n));
                break;
            }
            Err(_e) => {
                println!("Error parsing number.");
                println!("Try again");
            }
        };
    }
}
