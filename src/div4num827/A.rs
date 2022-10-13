use std::io::{BufWriter, stdin, stdout, Write};
use std::str;

#[derive(Default)]
pub struct Scanner {
    buffer: Vec<String>
}

impl Scanner {
    pub fn next<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let cases = scan.next::<usize>();

    for _ in 0..cases {
        let one: usize = scan.next();
        let two: usize = scan.next();
        let three: usize = scan.next();

        if one + two == three || one + three == two || two + three == one {
            writeln!(out, "YES").ok();
        } else {
            writeln!(out, "NO").ok();
        }
    }
}