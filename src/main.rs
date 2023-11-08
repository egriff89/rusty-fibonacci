use clap::Parser;
use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::cmp::Ordering;
use anyhow::bail;

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    // The number in the sequence to calculate
    #[arg(short, long, default_value_t = 0, allow_hyphen_values = true)]
    number: i32
}

fn fib(nth: i32) -> anyhow::Result<BigInt> {
    match nth.cmp(&0) {
        Ordering::Equal   => Ok(Zero::zero()),
        Ordering::Less    => bail!("Number cannot be negative!"),
        Ordering::Greater => {
            let mut a: BigInt = One::one();
            let mut b: BigInt = One::one();

            for _ in 1..nth {
                let temp: BigInt = a + &b;
                a = b;
                b = temp;
            }
            Ok(a)
        }
    }
}

fn main() {
    let nth = Args::parse().number;

    match fib(nth) {
        Ok(n)  => println!("{n}"),
        Err(e) => eprintln!("ERROR: {e}")
    }
}
