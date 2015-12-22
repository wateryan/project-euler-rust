// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut max = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let curr_val = i * j;
            if is_palindrome(curr_val.to_string()) {
                if curr_val > max {
                    max = curr_val;
                }
            }
        }
    }
    println!("Max: {}", max);
}

fn is_palindrome(val: String) -> (bool) {
    let chars: Vec<char> = val.chars().collect();
    let rev_chars: Vec<char> = val.chars().rev().collect();
    return chars.eq(&rev_chars);
}
