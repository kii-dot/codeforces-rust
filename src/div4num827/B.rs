use std::io::{BufWriter, stdin, stdout, Write};
use std::str;
use std::collections::HashSet;

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
        let n: usize = scan.next();

        let mut hs: HashSet<usize> = HashSet::new();
        let mut found_repeat: bool = false;
        for _ in 0..(n) {
            let cur = scan.next();
            if hs.contains(&(cur)) {
                found_repeat = true;
            }
            hs.insert(cur);
        }

        if found_repeat {
            writeln!(out, "NO").ok();
        } else {
            writeln!(out, "YES").ok();
        }
    }
}