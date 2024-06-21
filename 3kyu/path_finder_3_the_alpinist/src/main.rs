fn main() {
    println!("Hello, world!");
}

fn path_finder(area: &[Vec<u32>]) -> u32 {
    // code here
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_equal(input: &[Vec<u32>], actual: u32, expected: u32) {
        assert_eq!(
            actual, expected,
            "\nFor the input: {:?}\nYour result (left) did not match the expected output (right)",
            input
        );
    }

    #[test]
    fn test_basic() {
        let area: Vec<Vec<u32>> = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        test_equal(&area, path_finder(&area), 0);

        let area: Vec<Vec<u32>> = vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        test_equal(&area, path_finder(&area), 2);

        let area: Vec<Vec<u32>> = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]];
        test_equal(&area, path_finder(&area), 4);

        let area: Vec<Vec<u32>> = vec![
            vec![0, 7, 0, 7],
            vec![7, 0, 7, 0],
            vec![0, 7, 0, 7],
            vec![7, 0, 7, 0],
        ];
        test_equal(&area, path_finder(&area), 42);

        let area: Vec<Vec<u32>> = vec![
            vec![7, 0, 0, 0, 0, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 0, 0, 0, 0, 7],
        ];
        test_equal(&area, path_finder(&area), 14);

        let area: Vec<Vec<u32>> = vec![
            vec![7, 7, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 7, 7, 7],
        ];
        test_equal(&area, path_finder(&area), 0);

        let area: Vec<Vec<u32>> = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 1, 0, 9],
            vec![0, 0, 1, 0, 1, 0],
        ];
        test_equal(&area, path_finder(&area), 4);
    }
}
