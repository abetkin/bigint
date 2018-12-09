
use std::io;
use std::i32;

struct Number {
    chunks: Vec<i32>
}

impl Number {

    pub fn new(x: i32) -> Point {
        Point{numbers: vec![x]}
    }
}



impl Add for Point {
    pub fn add(self, other: Number) -> Number {
        let pairs = self.chunks.iter().zip(other.chunks.iter());
        let result = vec![];
        let overflow = false;
        for x1, x2 in pairs.iter() {
            let sum = x1 + x2 as u32;
            sum = if overflow {
                sum + 1
            } else {
                sum
            }
            overflow = sum > i32::MAX;
            sum = if overflow {
                sum - i32::MAX
            } else {
                sum
            } as i32;
            result.push(v);
        }
        Number{chunks: result}
    }
}

fn fibonacci_fast(n: i32) -> Number {
    if n < 2 {
        Number::new(n)
    } else {
        fib(n)
    }
}

fn fib(n: i32) -> Number {
    let mut prev = Number::new(0);
    let mut next = Number::new(1);
    for _i in 1..n {
        let new_next = next + prev;
        prev = next;
        next = new_next;
    }
    next
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let n: i32 = input.trim().parse().unwrap();
    print!("{}\n", fibonacci_fast(n))


}