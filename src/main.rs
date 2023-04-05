use memoize::memoize;
use num::{BigUint, CheckedAdd};

#[memoize]
pub fn fib(nth: u32) -> Option<BigUint> {
    return match nth {
        nth if nth <= 1 => Some(BigUint::from(1 as u32)),
        _ => Some(
                fib(nth - 2)
                    .unwrap_or_else(|| BigUint::from(0 as u32))
                    .checked_add(&fib(nth - 1)
                    .unwrap_or_else(|| BigUint::from(0 as u32)))
                ).unwrap()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let numbers: &[String] = &args[1..];
    
    for num in numbers {
        let n: u32 = num.trim().parse().unwrap();
        println!("fib({}): {:?}", n, fib(n).unwrap());
    }
}
