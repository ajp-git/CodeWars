fn show_bits(n: i32) -> [u8; 32] {
    let mut abs_n = n.abs() as u32;
    let mut bits: [u8; 32] = [0; 32];

    if n < 0 {
        abs_n = !abs_n + 1; // Convert to two's complement
    }

    for i in 0..32 {
        bits[i] = ((abs_n >> i) & 1) as u8;
    }

    bits.reverse(); // Reverse the order to get MSB at index 0
    bits
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::show_bits;

    fn dotest(n: i32, expected: &[u8; 32]) {
        let actual = &show_bits(n);
        assert!(
            actual == expected,
            "With n = {n}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        for (n, expected) in [
            (
                -2147483648,
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
            ),
            (
                701,
                &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0,
                    1, 1, 1, 1, 0, 1,
                ],
            ),
            (
                1,
                &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
            ),
            (
                -1,
                &[
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1,
                ],
            ),
            (
                -245,
                &[
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                    0, 0, 1, 0, 1, 1,
                ],
            ),
            (
                12_336,
                &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
                    1, 1, 0, 0, 0, 0,
                ],
            ),
            (
                -15,
                &[
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 0, 0, 0, 1,
                ],
            ),
            (
                0,
                &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
            ),
        ] {
            dotest(n, expected);
        }
    }
}
