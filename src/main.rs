use anyhow::bail;
use num::BigInt;

fn fib(nth: i32) -> anyhow::Result<BigInt> {
    
    if nth == 0 {
        return Ok(BigInt::from(0))
    } else if nth < 0 {
        bail!("")
    }

    let mut a: BigInt = BigInt::from(1);
    let mut b: BigInt = BigInt::from(1);

    for _ in (1..nth).enumerate() {
        let temp: BigInt = a;
        a = b.clone();
        b = b.checked_add(&temp).unwrap();
    }

    return Ok(a);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let numbers: &[String] = &args[1..];
    
    for num in numbers {
        let nth: i32 = num.trim().parse().unwrap();
        
        match fib(nth) {
            Ok(n)  => println!("fib({}): {:?}", nth, n),
            Err(_) => eprintln!("Cannot be negative!")
        }
    }
}
