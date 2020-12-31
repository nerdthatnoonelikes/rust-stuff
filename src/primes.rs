// Original problem link: https://code.golf/prime-numbers#rust

pub fn run() {
    for x in 1..101 {
        if is_prime(x) {
            println!("{}", x)
        }
    }
}
fn is_prime(n: i64) -> bool {
    for x in 2..n {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}