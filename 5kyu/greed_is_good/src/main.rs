fn main() {
    println!("Hello, world!");
}
use std::collections::HashMap;

fn score(dice: [u8; 5]) -> u32 {
    let mut counts = HashMap::new();
    for &d in &dice {
        *counts.entry(d).or_insert(0) += 1;
    }

    let mut score = 0;
    for (&d, &val) in &counts {
        match d {
            1 => {
                score += (val / 3) * 1000 + (val % 3) * 100;
            }
            5 => {
                score += (val / 3) * 500 + (val % 3) * 50;
            }
            _ => {
                score += (val / 3) * 100 * d as u32;
            }
        }
    }
    score
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::score;

    fn dotest(dice: [u8; 5], expected: u32) {
        let actual = score(dice);
        assert!(
            actual == expected,
            "Expected score with dice {dice:?} to be {expected}, but was {actual}\n"
        );
    }

    #[test]
    fn sample_tests() {
        dotest([2, 3, 4, 6, 2], 0);
        dotest([4, 4, 4, 3, 3], 400);
        dotest([2, 4, 4, 5, 4], 450);
    }
}
