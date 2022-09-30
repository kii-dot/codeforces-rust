use crate::scanner::Scanner;

use std::cmp::min;
use std::io::{BufWriter, stdin, stdout, Write};
use std::str;
const BITS: usize = 19;

mod scanner;

fn main() {
    let mut scan = Scanner::default();

    let out = &mut BufWriter::new(stdout());
    let n = scan.next::<usize>();
    let q = scan.next::<usize>();
    let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let mut closest = vec![n; BITS];
    let mut next = vec![vec![n; BITS]; n+1];

    for (i, &ai) in a.iter().enumerate().rev() {
        let (zero_bits, one_bits): (Vec<usize>, Vec<usize>) =
            (0..BITS).partition(|b| (ai & (1usize << b)) == 0);

        for b in one_bits {
            next[i][b] = i;
            let j = closest[b];
            for &c in &zero_bits { // must borrow (not consume) zero_bits to reuse it!
                // inner loop's constant factor <= 1/4, so it may run 27 million times
                next[i][c] = min(next[i][c], next[j][c]);
            }
            closest[b] = i;
        }
    }

    for _ in 0..q {
        let x = scan.next::<usize>() - 1;
        let y = scan.next::<usize>() - 1;
        let success = (0..BITS).filter(|b| (a[y] & (1usize << b)) != 0)
            .any(|b| next[x][b] <= y);
        if success {
            writeln!(out, "Shi").ok();
        } else {
            writeln!(out, "Fou").ok();
        }
    }
}
