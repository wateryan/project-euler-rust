// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//
// What is the 10 001st prime number?

fn main() {
    let limit = 10001;
    let mut values: Vec<i64> = Vec::new();
    let mut i = 1;
    while values.len() <= limit {
        if is_prime(&i) {
            values.push(i);
        }
        i += 1;
    }

    println!("Solution: {}", values.pop().unwrap());
}

fn is_prime(val: &i64) -> (bool) {
    for i in 2..*val {
        if val % i == 0 && *val != i {
            return false;
        }
    }
    return true;
}
