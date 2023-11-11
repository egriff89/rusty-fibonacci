# rusty-fibonacci
Basic Rust CLI utility that calculates the nth number of the Fibonacci sequence.

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
354,224,848,179,261,915,075
$ fibonacci -n 1000
43,466,557,686,937,456,435,688,527,675,040,625,802,564,660,517,371,780,402,481,729,089,536,555,417,949,051,890,403,879,840,079,255,169,295,922,593,080,322,634,775,209,689,623,239,873,322,471,161,642,996,440,906,533,187,938,298,969,649,928,516,003,704,476,137,795,166,849,228,875
```