use clap::Parser;
use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    // The number in the sequence to calculate
    number: usize,
}

fn fib(nth: usize) -> BigUint {
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();

    for _ in 0..nth {
        let temp = a + &b;
        a = b;
        b = temp;
    }
    a
}

fn main() {
    let nth = Args::parse().number;
    println!("{}", fib(nth));
}
