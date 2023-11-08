use num::BigUint;

fn increment_string(s: &str) -> String {
    let mut tx: String = String::new();
    let mut nu: String = String::new();
    let mut rev: String;
    let one: BigUint = BigUint::from(1 as u32);

    for i in s.chars().rev() {
        if i.is_digit(10) && tx.len() == 0 {
            nu.push(i);
        } else {
            tx.push(i);
        }
    }
    rev = tx.chars().rev().collect();
    tx = rev;
    rev = nu.chars().rev().collect();
    nu = rev;
    println!("tx = {}", tx);
    println!("nu = {}", nu);
    match nu.parse::<BigUint>() {
        Ok(n) => {
            let s1 = nu.len();
            let n1 = format!("{}", n.clone() + one.clone());
            if nu.chars().next() == Some('0') {
                nu = format!("{}{}", "0".repeat(s1 - n1.len()), n + one.clone());
            } else {
                nu = format!("{}", n + one);
            }
        }
        Err(_) => nu = "1".to_string(),
    }
    format!("{}{}", tx, nu)
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        dotest("foobar001", "foobar002");
        dotest("foo", "foo1");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}
