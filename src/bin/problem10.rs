// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

fn main() {
    let limit = 2_000_000;
    let mut sum = 0;
    for i in 2..limit + 1 {
        if is_prime(&i) {
            sum += i;
        }
    }
    println!("Solution: {}", sum);
}

fn is_prime(val: &i64) -> (bool) {
    if *val == 2 {
        return true;
    } else if *val == 3 {
        return true;
    } else if val % 2 == 0 {
        return false;
    } else if val % 3 == 0 {
        return false;
    }
    let mut i: i64 = 5;
    let mut w: i64 = 2;
    while i * i <= *val {
        if val % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }
    return true;
}
