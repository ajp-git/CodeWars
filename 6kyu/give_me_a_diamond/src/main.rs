fn main() {
    println!("Hello, world!");
}

fn print(n: i32) -> Option<String> {
    if n < 1 || n % 2 == 0 {
        return None;
    }
    let mut result = String::new();

    let steps = (1..=n).step_by(2).chain((1..n).step_by(2).rev());

    for step in steps {
        result.push_str(&(" ".to_string().repeat(((n - step) / 2) as usize)));
        result.push_str(&("*".repeat(step as usize)));
        result.push('\n');
    }
    Some(result)
}

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}
