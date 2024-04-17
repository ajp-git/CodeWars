fn coprimes(n: u32) -> Vec<u32> {
    let pg = pgc(n);
    (1..=n)
        .into_iter()
        .filter_map(|f| {
            if !common_pg(pgc(f), &pg) || f == 1 {
                Some(f)
            } else {
                None
            }
        })
        .collect()
}
fn pgc(n: u32) -> Vec<u32> {
    let pg: Vec<u32> = (1..=n)
        .into_iter()
        .filter_map(|f| if n % f == 0 { Some(f) } else { None })
        .collect();
    pg
}

fn common_pg(v: Vec<u32>, w: &Vec<u32>) -> bool {
    for i in v.into_iter() {
        if w.contains(&i) && i != 1 {
            return true;
        }
    }
    false
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
fn main() {
    println!("pgc {} : {:?}", 10, pgc(10));
    println!("pgc {} : {:?}", 20, pgc(20));
    println!("pgc {} : {:?}", 30, pgc(30));
    println!("pgc {} : {:?}", 45, pgc(45));
}
#[cfg(test)]
mod tests {
    use super::coprimes;

    fn dotest(n: u32, expected: &[u32]) {
        let actual = coprimes(n);
        assert!(
            actual == expected,
            "Test failed with n = {n}\nExpected {expected:?} but got {actual:?}"
        )
    }
    #[test]
    fn fixed_tests() {
        dotest(2, &[1]);
        dotest(3, &[1, 2]);
        dotest(6, &[1, 5]);
        dotest(10, &[1, 3, 7, 9]);
        dotest(20, &[1, 3, 7, 9, 11, 13, 17, 19]);
        dotest(
            25,
            &[
                1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 23, 24,
            ],
        );
        dotest(30, &[1, 7, 11, 13, 17, 19, 23, 29]);
    }
}
