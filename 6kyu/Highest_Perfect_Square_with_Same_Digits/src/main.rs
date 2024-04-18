fn next_perfectsq_perm(lower_limit: u32, k: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::next_perfectsq_perm;
        
    fn dotest(n: u32, k: u32, expected: u32) {
        let actual = next_perfectsq_perm(n, k);
        assert!(actual == expected, 
            "With n = {n}, k = {k}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn example_tests() {
        dotest(100, 2, 441);
        dotest(100, 3, 961);
        dotest(100, 4, 81796);
        dotest(500, 2, 625);
        dotest(1000, 3, 9216);
        dotest(100000, 4, 298116);
    }

    #[test]
    fn some_edge_cases() {
        dotest(144, 2, 625);
        dotest(145, 2, 625);
        dotest(440, 2, 441);
        dotest(441, 2, 625);
        dotest(257, 2, 441);
    }
}
