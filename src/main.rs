use anyhow::bail;
use clap::Parser;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::cmp::Ordering;

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    // The number in the sequence to calculate
    #[arg(short, long, default_value_t = 0, allow_hyphen_values = true)]
    number: i32,
}

fn fib(nth: i32) -> anyhow::Result<BigInt> {
    match nth.cmp(&0) {
        Ordering::Equal => Ok(Zero::zero()),
        Ordering::Less => bail!("Number cannot be negative!"),
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

// Format a BigInt value to a String with a thousands separator
// Source: https://stackoverflow.com/a/67834588
fn format_value(n: &BigInt) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

fn main() {
    let nth = Args::parse().number;

    match fib(nth) {
        Ok(n) => println!("{}", format_value(&n)),
        Err(e) => eprintln!("ERROR: {e}"),
    }
}
