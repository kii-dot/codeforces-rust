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
    let t = scan.next::<usize>();

    for _ in 0..t {
        let n = scan.next::<usize>();
        let k = scan.next::<usize>();
        let bpc = n / k;
        let str = scan.next::<String>();


        // create map and fill with letters available
        // in the string
        let mut alphamap = vec![0; 26];
        for letter in str.chars() {
            alphamap[letter as usize - 'a' as usize] =
                alphamap[letter as usize - 'a' as usize] + 1;
        }



        let mut shelf: Vec<u8> = vec![];
        // go through the entire possible string to fill
        // up the slots
        for _ in 0..k {
            // we fill up the slots by getting the amount
            // of numbers out
            let mut val = 0;

            let mut row_count = bpc;
            for index in 0..alphamap.len() {
                let to_subtract = alphamap[index] > 0;

                if to_subtract {
                    if index == val {
                        val = val + 1;
                    }
                    alphamap[index] = alphamap[index] - 1;
                    row_count = row_count - 1;
                    if row_count == 0 {
                        break;
                    }
                }

                // we make sure we exit the while loop
            }

            shelf.push((val +'a' as usize).try_into().unwrap());
        }

        writeln!(out, "{}", String::from_utf8(shelf).expect("not ascii")).ok();
    }
}