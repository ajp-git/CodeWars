fn main() {
    println!("Hello, world!");
}

fn peak_height(mountain: &[&str]) -> u32 {
    todo!();
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::peak_height;

    #[test]
    fn example() {
        let mountain = [
            "^^^^^^        ",
            " ^^^^^^^^     ",
            "  ^^^^^^^     ",
            "  ^^^^^       ",
            "  ^^^^^^^^^^^ ",
            "  ^^^^^^      ",
            "  ^^^^        "
        ];
        assert_eq!(peak_height(&mountain), 3, "\nYour result (left) did not match expected result (right)");
    }
}
