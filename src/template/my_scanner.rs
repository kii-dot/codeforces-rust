use std::io::{BufWriter, stdin, stdout, Stdout, Write};
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
        run(&mut scan, out);
    }
}

// solution here
fn run(scan: &mut Scanner, out: &mut BufWriter<Stdout>) {

}