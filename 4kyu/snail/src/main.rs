fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    
    let x_len=matrix[0].len();
    let mut out:Vec<i32>=Vec::new();

    let mut pos_x=0;
    let mut pos_y=0;

    let mut current_x=0;
    let mut current_y=0;

    let push=|| out.push(matrix[current_y][current_x]);

    let rules=[(0,1),(1,0),(0,-1),(-1,0)];


    let m_len=matrix[0].len()/2+1;
    for i in 1..m_len {
        for n in 0..x_len-i {
            println!("{},{} : {}",current_x,current_y, matrix[current_y][current_x]);
            current_x+=1;
        }
        for n in 0..x_len-i {
            println!("{},{} : {}",current_x,current_y, matrix[current_y][current_x]);
            current_y+=1;
        }
        for n in 0..x_len-i {
            current_x-=1;
            println!("{},{} : {}",current_x,current_y, matrix[current_y][current_x]);
        }
        for n in 0..x_len-i {
            current_y-=1;
            println!("{},{} : {}",current_x,current_y, matrix[current_y][current_x]);
        }
    }

 out
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
