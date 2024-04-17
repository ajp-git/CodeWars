enum Direction {
    East,
    West,
    South,
    North,
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let l = matrix.len();
    let mut v: Vec<i32> = Vec::new();

    if l == 0 {
        return v;
    }
    let mut dir = Direction::East;
    let mut left = 0;
    let mut right = l - 1;
    let mut top = 0;
    let mut bottom = l - 1;

    let total_count: usize = matrix.iter().map(|f| f.len()).sum();

    while v.len() != total_count {
        match dir {
            Direction::East => {
                if left == right {
                    v.push(matrix[top][left]);
                    break;
                }
                for i in left..right {
                    v.push(matrix[top][i]);
                }
                dir = Direction::South;
            }
            Direction::West => {
                for i in ((left + 1)..=right).rev() {
                    v.push(matrix[bottom][i]);
                }
                dir = Direction::North;
            }
            Direction::South => {
                for i in top..bottom {
                    v.push(matrix[i][right]);
                }
                dir = Direction::West;
            }
            Direction::North => {
                for i in ((top + 1)..=bottom).rev() {
                    v.push(matrix[i][left]);
                }
                bottom -= 1;
                top += 1;
                left += 1;
                right -= 1;
                dir = Direction::East;
            }
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
    #[test]
    fn sample_test5() {
        let square: &[Vec<i32>; 5] = &[
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        let expected = vec![
            1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12,
            13,
        ];
        assert_eq!(snail(square), expected);
    }
}
