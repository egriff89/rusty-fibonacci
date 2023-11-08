# rusty-fibonacci
Basic Rust CLI utility that calculates the nth number of the Fibonacci sequence.

## Todo
- [x] Implement without recursion (dynamic).
- [x] Correctly handle negative numbers without causing a panic.


## Installation
```shell
git clone https://github.com/egriff89/rusty-fibonacci.git
cd rusty-fibonacci
cargo install --path .
```
## Running

```shell
$ fibonacci -h
Quickly calculate the Nth number of the Fibonacci sequence.

Usage: fibonacci [OPTIONS]

Options:
  -n, --number <NUMBER>  [default: 0]
  -h, --help             Print help

$ fibonacci
0
$ fibonacci -n 10
55
$ fibonacci -n 100
354224848179261915075
$ fibonacci -n 1000
43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875
```