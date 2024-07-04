fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {

    let mut matrix= matrix.to_vec();
    let mut result:Vec<i32>=Vec::new();

    while !matrix.is_empty() {
        result.extend(matrix.remove(0));


        for line in matrix.iter_mut() {
            if let Some(val)=line.pop() {
                result.push(val);
            }
        }
        
        if let Some(mut last_row) = matrix.pop() {
            while ! last_row.is_empty() {
                if let Some(val) = last_row.pop() {
                    result.push(val);
                }
            }
    
        }

        for i in (0..matrix.len()).rev() {
            result.push(matrix[i].remove(0));
        }

    }
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];
        let expected = vec![1,2,3,6,9,8,7,4,5];
        assert_eq!(snail(square), expected);
    }
    
    #[test]
    fn sample_test2() {
        let square = &[
            vec![1,2,3],
            vec![8,9,4],
            vec![7,6,5],
        ];
        let expected = vec![1,2,3,4,5,6,7,8,9];
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
}
