
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

    fn add_o(&self, other: &Number) -> Number {
        
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
        if overflow {
            result.push(1);
        }
        if other.chunks.len() > self.chunks.len() {
            for &val in &other.chunks[self.chunks.len()..] {
                result.push(val)
            }
        }
        Number{chunks: result}
    }
}

fn fib(n: i32) -> Number {
    if n < 2 {
        return Number::new(n)
    }
    let n_pos = n as usize;
    let mut result: Vec<Number> = vec![];
    result.push(Number::new(0));
    result.push(Number::new(1));
    for i in 1..n_pos {
        result.push(result[i-1].add_o(&result[i]))
    }
    result.pop().unwrap()
}

/* fails:
/ 1836311903 (46)
/ 1,823731426 (47)
/ 1,1,512559682 (48)
*/


// fn test_sum() -> Number{
//     let num1 = Number{
//         chunks: vec![2000000000]
//     };
//     let num2 = Number {
//         chunks: vec![2000000000]
//     };
//     num1 + num2
// }

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let n: i32 = input.trim().parse().unwrap();
    print!("{}\n", Number::format(fib(n)));
    // let x = test_sum();
    // print!("{}\n", Number::format(x));

}