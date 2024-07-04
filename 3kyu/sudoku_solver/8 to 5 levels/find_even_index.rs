fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    todo!()
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::the_lift;
    
    fn print_queues(queues: &[Vec<u32>], capacity: u32) -> String {
        let mut result = format!("\nLift capacity = {capacity}\n\n Floor    Queue");
        for (i, q) in queues.iter().enumerate().rev() {
            result.push_str(&format!("\n{i:>4} .... {q:?}"));
        }
        result
    }

    fn do_test(queues: &[Vec<u32>], capacity: u32, expected: &[u32]) {
        let actual = the_lift(queues, capacity);
        assert_eq!(actual, expected,
            "\nYour result (left) did not match expected output (right) for the given queues:\n{}\n",
            print_queues(queues, capacity));
    }

    #[test]
    fn test_up() {
        do_test(&[vec![], vec![], vec![5,5,5],vec![],vec![],vec![],vec![]], 5, &[0, 2, 5, 0]);
    }
    #[test]
    fn test_down() {
        do_test(&[vec![],vec![],vec![1],vec![],vec![],vec![],vec![]], 5, &[0, 2, 1, 0]);
    }
    #[test]
    fn test_up_and_up() {
        do_test(&[vec![],vec![3],vec![4],vec![],vec![5],vec![],vec![]], 5, &[0, 1, 2, 3, 4, 5, 0]);
    }
    #[test]
    fn test_down_and_down() {
        do_test(&[vec![],vec![0],vec![],vec![],vec![2],vec![3],vec![]], 5, &[0, 5, 4, 3, 2, 1, 0]);
    }
}
