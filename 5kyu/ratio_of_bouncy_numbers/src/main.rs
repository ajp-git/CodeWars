fn main() {
    println!("Hello, world!");
}

fn bouncy_ratio(ratio: f64) -> Option<u32> {
    if ratio == 0f64 {
        return Some(1);
    }
    if ratio < 0f64 || ratio > 0.99 {
        return None;
    }
    let mut bouncies = 0;
    let mut current_numbers: u32 = 100;
    loop {
        if is_bouncy(current_numbers) {
            bouncies += 1;
            if (bouncies as f64 / current_numbers as f64) >= ratio {
                return Some(current_numbers);
            }
        }
        current_numbers += 1;
    }
}

fn is_bouncy(number: u32) -> bool {
    let number_text = number.to_string();
    let mut number_text_iter = number_text.chars().peekable();
    let mut bgreater = false;
    let mut bsmaller = false;

    while let Some(digit) = number_text_iter.next() {
        if let Some(nex) = number_text_iter.peek() {
            if &digit < nex {
                bsmaller = true
            }
            if &digit > nex {
                bgreater = true
            }
        }
    }

    bsmaller && bgreater
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::{bouncy_ratio, is_bouncy};

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p: f64, expected: Option<u32>) {
        assert_eq!(bouncy_ratio(p), expected, "{ERR_MSG} with ratio = {p}")
    }

    #[test]
    fn fixed_tests() {
        dotest(0.0, Some(1));
        dotest(0.999, None);
        dotest(0.15, Some(160));
        dotest(0.5, Some(538));
        dotest(0.75, Some(3088));
        dotest(0.9, Some(21780));
    }
    #[test]
    fn bouncy_test() {
        assert_eq!(is_bouncy(121), true)
    }
}
