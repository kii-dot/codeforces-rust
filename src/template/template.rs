// ---------- begin scanner -----------
#[allow(unused_imports)]
use std::io::{BufWriter, stdin, stdout, Read, Write, Stdout};
use std::str::FromStr;

pub struct Scanner<'a> {
    it: std::str::SplitWhitespace<'a>,
}
#[allow(dead_code)]
impl<'a> Scanner<'a> {
    pub fn new(s: &'a String) -> Scanner<'a> {
        Scanner {
            it: s.split_whitespace(),
        }
    }
    pub fn next<T: FromStr>(&mut self) -> T {
        self.it.next().expect("read EOF").parse::<T>().ok().expect("parse failed")
    }
    pub fn next_n<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
    pub fn next_n_index1<T: FromStr + Default>(&mut self, n: usize) -> Vec<T> {
        std::iter::once(T::default()).chain((0..n).map(|_| self.next())).collect()
    }
    pub fn next_chars(&mut self) -> Vec<char> { self.it.next().unwrap().chars().collect() }
    pub fn next_bytes(&mut self) -> Vec<u8> { self.it.next().unwrap().bytes().collect() }
    pub fn next_digits(&mut self) -> Vec<u8> {
        self.it.next().unwrap().bytes().map(|c|c-C0).collect()
    }
}
// ---------- end scanner -------------


// ---------- begin utils -------------
#[allow(dead_code)]
const C0: u8 = '0' as u8;

// ---------- end utils ---------------


// ---------- begin debugger ----------
#[cfg(feature = "my_debug")]
#[macro_export]
macro_rules! debug {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        eprintln!("[{}:{}] {} = {:?}", file!(), line!(), stringify!($val), &($val))
    };
    ($($val:expr),+ $(,)?) => {
        ($(debug!($val)),+,)
    };
}

#[cfg(not(feature = "my_debug"))]
#[macro_export]
macro_rules! debug {
    ($( $args:expr ),*) => {}
}
// ---------- end debugger ------------


fn main() {
    debug!("Ish: good luck and high rating!");

    #[cfg(not(feature = "my_debug"))]
        let s = {
        let mut s = String::new();
        stdin().read_to_string(&mut s).unwrap();
        s
    };
    #[cfg(feature = "my_debug")]
        let s = std::fs::read_to_string("in.txt").unwrap();

    let mut sc = Scanner::new(&s);
    let out = &mut BufWriter::new(stdout());

    let cases: u32 = sc.next();
    // let cases: u32 = 1;
    debug!(cases);
    for _case in 1..=cases { debug!(_case);
        run(&mut sc, out);
    }
}


/** Write your mind here.

  */
fn run(sc: &mut Scanner, out: &mut BufWriter<Stdout>) {

}