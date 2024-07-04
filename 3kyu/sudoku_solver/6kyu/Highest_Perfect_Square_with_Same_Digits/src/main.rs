use std::collections::HashSet;
use itertools::Itertools;

fn next_perfectsq_perm(lower_limit: u32, k: u32) -> u32 {

    let mut index:u32=(lower_limit as f32).sqrt() as u32 +1;

    loop {
        let square=index*index;
        if square >  lower_limit {
            let sq:u32 = (square as f32).sqrt() as u32;

            if sq*sq == square {
                let square_txt=format!("{}",square);
                if ! square_txt.contains('0') {
                    let mut squares:HashSet<u32>=HashSet::new();
        
                    square_txt
                        .chars()
                        .permutations(square_txt.len())
                        .for_each(
                            |t| {
                                let val = t
                                    .iter()
                                    .collect::<String>()
                                    .parse::<u32>()
                                    .unwrap();

                                    let sq=(val as f32).sqrt() as u32;
                                    if sq*sq==val {                                         
                                        squares.insert(val);
                                    }
                                }
                        );

                    if squares.len()==k as usize{
                        let max_square=squares.iter().max().unwrap();
                        return *max_square;    
                    }
                }    
            }
    
        }
        index+=1;
    }
}
fn main(){
    next_perfectsq_perm(200, 2);
    next_perfectsq_perm(100, 2);
    next_perfectsq_perm(100, 3);
    next_perfectsq_perm(100, 4);
    next_perfectsq_perm(3550000, 5);
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
