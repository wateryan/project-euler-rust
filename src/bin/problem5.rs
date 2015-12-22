// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    let val: i64 = find_solution();
    println!("Solution: {}", val);
}

fn find_solution() -> (i64) {
    // filter by values which meet solution criteria, .next() grabs the first value as an Option<i64>
    // the unwrap that and return the value.
    return (1..).filter(|i| can_divide(*i)).next().unwrap();
}

fn can_divide(val: i64) -> (bool) {
    // filters by values which cannot be evenly divided, if none are present, returns true
    return (1..20).filter(|i| val % *i != 0).next().is_none();
}
