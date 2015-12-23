// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//
// a2 + b2 = c2
// For example, 32 + 42 = 9 + 16 = 25 = 52.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    let limit = 1000;
    let mut solution = 0;
    for a in 1..limit / 3 {
        for b in a..limit / 2 {
            for c in 0..limit {
                if a * a + b * b == c * c && a + b + c == 1000 {
                    solution = a * b * c;
                }
            }
        }
    }
    println!("Solution: {}", solution);
}
