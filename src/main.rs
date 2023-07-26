use clap::Parser;
use num::BigInt;
use std::cmp::Ordering;
use anyhow::{Result, bail};

#[derive(Parser, Debug)]
struct Args {
    // #[arg(short = 'n', long = "number", allow_hyphen_values = true)]
    #[arg(allow_hyphen_values = true)]
    number: i32
}

fn fib(nth: i32) -> Result<BigInt> {
    
    match nth.cmp(&0) {
        Ordering::Equal   => Ok(BigInt::from(0)),
        Ordering::Less    => bail!("Number cannot be negative!"),
        Ordering::Greater => {
            let mut a: BigInt = BigInt::from(1);
            let mut b: BigInt = BigInt::from(1);

            for _ in 1..nth {
                let temp: BigInt = a;
                a = b.clone();
                b = b.checked_add(&temp).unwrap();
            }
            Ok(a)
        }
    }
}

fn main() {
    let nth = Args::parse().number;

    match fib(nth) {
        Ok(n)  => println!("fib({}): {:?}", nth, n),
        Err(e) => eprintln!("ERROR: {}", e)
    }
}
