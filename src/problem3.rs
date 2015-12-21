// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?
fn main() {
    let max = 600851475143usize;
    let mut curr_max = max;
    let mut fact = 0;
    let mut i = 2;

    while i * i <= curr_max {
        if curr_max % i == 0 {
            curr_max  = curr_max / i;
            fact = i;
        }
        i += 1;
    }
    if curr_max > fact {
        fact = curr_max;
    }
    println!("Largest prime factor: {}", fact);
}
