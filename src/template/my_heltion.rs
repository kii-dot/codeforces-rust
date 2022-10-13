fn main() {
    for _ in 0..read_i32() {
        run();
    }
}

// Write Solution Here
//
// "When the crawfish sings, it's time to think"
fn run() {

}
// Write solution Ends

fn read_i32() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    return line.trim().parse::<i32>().unwrap();
}

fn read_i32s() -> Vec<i32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    return line.trim().split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    return line.trim().to_string();
}

fn gcd(a : i32, b : i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
