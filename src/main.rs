use clap::Parser;
use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    // The number in the sequence to calculate
    #[arg(short, long, default_value_t = 0)]
    number: usize,
}

fn fib(nth: usize) -> String {
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();

    for _ in 0..nth {
        let temp: BigUint = a + &b;
        a = b;
        b = temp;
    }
    format_value(&a)
}

// Format a BigUint value to a String with a thousands separator
// Source: https://stackoverflow.com/a/67834588
fn format_value(n: &BigUint) -> String {
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
    println!("{}", fib(nth));
}
