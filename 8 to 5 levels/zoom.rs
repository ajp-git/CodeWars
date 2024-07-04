fn ascend_descend(length: usize, minimum: i32, maximum: i32) -> String {
    let s=(minimum..=maximum).chain(maximum..=minimum).cycle().for_each(|x| x.to_string).collect();
    println!("s: {}", s);
    s
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::ascend_descend;
    
    fn dotest(l: usize, a: i32, b: i32, expected: &str) {
        let actual = ascend_descend(l, a, b);
        assert!(actual == expected, 
            "With length = {l}, minimum = {a}, maximum = {b}\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn example_tests() {
        dotest(5, 1, 3, "12321");
        dotest(14, 0, 2, "01210121012101");
        dotest(11, 5, 9, "56789876567");
    }
}