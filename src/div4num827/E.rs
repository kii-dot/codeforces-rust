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
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let cases = sc.next::<usize>();

    for _ in 0..cases {
        run(&mut sc, out);
    }
}

// solution here
fn run(scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
    let steps: usize = scan.next();
    let leg_length: usize = scan.next();

    let mut step_arr = vec![0; steps];
    let mut cum_step_arr = vec![0; steps];
    let mut total = 0;
    for i in 0..steps {
        let cur_step: usize = scan.next();
        step_arr[i] = cur_step;
        total = total + cur_step;
        cum_step_arr[i] = total;
    }

    for _ in 0..leg_length {
        let cur_leg_length = scan.next::<usize>();
        let mut i = 0;
        while i < steps {
            if step_arr[i] > cur_leg_length {
                break;
            }
            i += 1;
        }

        let final_step = if i == 0 {
            0
        } else {
            cum_step_arr[i-1]
        };
        write!(out, "{} ", final_step).ok();
    }
    writeln!(out).ok();
}