
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

    fn clone(&self) -> Number {
        Number {
            chunks: self.chunks.clone()
        }
    }

    fn format(n: Number) -> String {
        let chunks: Vec<_> = n.chunks.iter().map(|n| n.to_string()).collect();
        // FIXME
        let mut reversed = chunks.clone();
        reversed.reverse();
        reversed.join(",")
    }
}



impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        let mut result = vec![];
        let mut overflow = false;
        for (i, &val) in self.chunks.iter().enumerate() {
            let mut sum = val as u32;
            if let Some(other_val) = other.chunks.get(i) {
                sum = (val + other_val) as u32;
            }
            if overflow {
                sum += 1;
            }
            if sum > i32::MAX as u32 {
                sum -= i32::MAX as u32;
                overflow = true;
            } else {
                overflow = false;
            }
            result.push(sum as i32);
        }
        if other.chunks.len() > self.chunks.len() {
            for &val in &other.chunks[self.chunks.len()..] {
                result.push(val)
            }
        }
        Number{chunks: result}
    }
}

fn test_sum() -> Number{
    let num1 = Number{
        chunks: vec![0, 1, 2]
    };
    let num2 = Number {
        chunks: vec![10]
    };
    num1 + num2
}

fn main() {
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input);
    // let n: i32 = input.trim().parse().unwrap();
    // print!("{}\n", Number::format(fibonacci_fast(n)))
    let x = test_sum();
    print!("{}\n", Number::format(x));

}