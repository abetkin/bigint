
use std::io;
use std::i32;
use std::ops::Add;

struct Number {
    chunks: Vec<i32>
}

impl Number {

    fn new(x: i32) -> Number {
        Number{chunks: vec![x]}
    }

    fn format(n: Number) -> String {
        let chunks: Vec<_> = n.chunks.iter().map(|n| n.to_string()).collect();
        chunks.join("")
    }
}



impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        let pairs = self.chunks.iter().zip(other.chunks.iter());
        let mut result = vec![];
        let mut overflow = false;
        for (x1, x2) in pairs {
            let mut sum = (x1 + x2) as u32;
            sum = if overflow {
                sum + 1
            } else {
                sum
            };
            overflow = if sum > i32::MAX as u32 { true } else { false };
            sum = if overflow {
                sum - (i32::MAX as u32)
            } else {
                sum
            };
            result.push(sum as i32);
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

    let fib_raw = "array"; // FIXME
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
    print!("{}\n", Number::format(fibonacci_fast(n)))


}