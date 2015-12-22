// The sum of the squares of the first ten natural numbers is,
// 12 + 22 + ... + 102 = 385
//
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)2 = 552 = 3025
//
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    let limit = 100;
    let sum: i64 = (0..limit + 1).fold(0, |sum, i| sum + i);
    let squared_sum = sum * sum;
    let sum_squares: i64 = (0..limit + 1).map(|i| i * i).fold(0, |sum, i| sum + i);
    let solution = squared_sum - sum_squares;
    println!("Solution: {}", solution);
}
