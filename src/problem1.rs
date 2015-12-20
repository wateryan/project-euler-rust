/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    println!("Finding the sum of all the multiples of 3 or 5 below 1000");
    let limit = 1000;
    iterative_solution(limit);
    hof_solution(limit);
}

/// Functional solution taking advantage of Rust's higher order functions.
fn hof_solution(limit: i32) {
    let sum: i32 = (0..limit).filter(|n|  n % 3 == 0 || n % 5 == 0).fold(0, |sum, i| sum + i);
    println!("Functional solution: {}", sum);
}

/// Standard iterative solution.
fn iterative_solution(limit: i32) {
    let mut sum = 0;
    for n in 0..limit {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    println!("Iterative Solution: {}", sum);
}
