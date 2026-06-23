#[inline(never)]
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[inline(never)]
fn sum_primes(limit: u64) -> u64 {
    let mut sum = 0;
    for i in 2..limit {
        if is_prime(i) {
            sum += i;
        }
    }
    sum
}

fn main() {
    let limit = 200_000;
    let result = sum_primes(limit);
    println!("Sum = {}", result);
}
