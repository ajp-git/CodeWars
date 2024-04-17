use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    if queues.iter().all(|queue| queue.is_empty()) {
        let v = vec![0];
        return v;
    }
    let mut l = Lift::new(queues, capacity);
    l.run()
}
#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
}
struct Lift {
    capacity: u32,
    queues: HashMap<u32, Vec<u32>>,
    direction: Direction,
    inside: Vec<u32>,
    level: u32,
    floors_visited: Vec<u32>,
}
impl Lift {
    fn new(queues: &[Vec<u32>], capacity: u32) -> Self {
        let mut t_queues = HashMap::new();

        for (i, v) in queues.iter().enumerate() {
            t_queues.insert(i as u32, v.clone());
        }

        Lift {
            capacity,
            queues: t_queues,
            direction: Direction::Up,
            inside: Vec::new(),
            level: 0,
            floors_visited: Vec::new(),
        }
    }
    fn run(&mut self) -> Vec<u32> {
        let mut levels_done: Vec<u32> = Vec::new();
        loop {
            if self.queues.is_empty() {
                return levels_done;
            }
            let out = self.queues.get(&self.level).unwrap();
            println!("out {:?}", out);
        }
    }
}
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
    /*
       #[test]
       fn test_get_first_up() {
           // Create a new Lift instance with test data
           let mut lift = Lift {
               capacity: 5,
               queues: HashMap::new(),
               direction: Direction::Up,
               inside: HashMap::new(),
               level: 2,
               floors_visited: Vec::new(),
                           // ... (rest of the Lift struct initialization if needed)
           };

           // Populate the queues with test data
           lift.queues.insert(1, vec![3, 4]);
           lift.queues.insert(3, vec![5]);
           lift.queues.insert(4, vec![5, 6, 7]);

           // Call the get_first_up method
           let result = lift.get_first_up();

           // Define the expected result
           let expected = Some(3); // Assuming this is the expected 'up' value based on the test data

           // Assert that the result matches the expected value
           assert_eq!(result, expected, "The get_first_up method did not return the expected result.");
       }
    */
    #[test]
    fn test_up() {
        do_test(
            &[
                vec![],
                vec![],
                vec![5, 5, 5],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            5,
            &[0, 2, 5, 0],
        );
    }
    #[test]
    fn test_down() {
        do_test(
            &[vec![], vec![], vec![1], vec![], vec![], vec![], vec![]],
            5,
            &[0, 2, 1, 0],
        );
    }
    #[test]
    fn test_up_and_up() {
        do_test(
            &[vec![], vec![3], vec![4], vec![], vec![5], vec![], vec![]],
            5,
            &[0, 1, 2, 3, 4, 5, 0],
        );
    }
    #[test]
    fn test_fire() {
        do_test(
            &[
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            5,
            &[0, 6, 0, 6, 0],
        );
    }
    #[test]
    fn test_empty_buiding() {
        do_test(
            &[vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
            5,
            &[0],
        );
    }
    #[test]
    fn test_down_and_down() {
        do_test(
            &[vec![], vec![0], vec![], vec![], vec![2], vec![3], vec![]],
            5,
            &[0, 5, 4, 3, 2, 1, 0],
        );
    }
    #[test]
    fn test_yoyo() {
        do_test(
            &[
                vec![],
                vec![],
                vec![4, 4, 4, 4],
                vec![],
                vec![2, 2, 2, 2],
                vec![],
                vec![],
            ],
            2,
            &[0, 2, 4, 2, 4, 2, 0],
        );
    }
    #[test]
    fn test_lift_full_up_and_down() {
        do_test(
            &[
                vec![3, 3, 3, 3, 3, 3],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![4, 4, 4, 4, 4, 4],
                vec![],
            ],
            5,
            &[0, 3, 5, 4, 0, 3, 5, 4, 0],
        );
    }
    #[test]
    fn test_highlander() {
        do_test(
            &[
                vec![],
                vec![2],
                vec![3, 3, 3],
                vec![1],
                vec![],
                vec![],
                vec![],
            ],
            1,
            &[0, 1, 2, 3, 1, 2, 3, 2, 3, 0],
        );
    }
    #[test]
    fn test_random() {
        do_test(
            &[
                vec![8],
                vec![],
                vec![],
                vec![7, 6, 1],
                vec![7, 3, 9],
                vec![7, 9, 7, 0],
                vec![2, 3, 9, 2, 10],
                vec![9],
                vec![2, 4],
                vec![7],
                vec![1],
            ],
            5,
            &[
                0, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 5, 6, 7, 9, 10, 6, 5, 2, 0,
            ],
        );
    }

    #[test]
    fn test_random4() {
        do_test(
            &[
                vec![9, 3, 2, 8, 10],
                vec![7, 9, 10],
                vec![],
                vec![4, 12, 12, 8, 11],
                vec![1, 0, 12, 6, 3],
                vec![10],
                vec![4, 1, 12, 4],
                vec![11, 2],
                vec![],
                vec![2, 11, 12, 2],
                vec![9, 7, 12, 9, 12],
                vec![5],
                vec![3],
            ],
            4,
            &[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 11, 10, 9, 7, 6, 5, 4, 3, 2, 1, 0, 1, 3,
                4, 5, 6, 7, 9, 10, 11, 12, 10, 9, 6, 4, 3, 2, 1, 0, 3, 4, 5, 6, 8, 11, 12, 5, 10,
                0,
            ],
        );
    }
}
/*
  left: `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 11, 10, 9, 7, 6, 5, 4, 3, 2, 1, 3, 4, 5, 6, 7, 9, 10, 11, 12, 10, 9, 6, 4, 3, 2, 1, 0, 3, 4, 5, 6, 8, 10, 11, 12, 5, 10, 0]`,
 right: `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 11, 10, 9, 7, 6, 5, 4, 3, 2, 1, 0, 1, 3, 4, 5, 6, 7, 9, 10, 11, 12, 10, 9, 6, 4, 3, 2, 1, 0, 3, 4, 5, 6, 8, 11, 12, 5, 10, 0]`:
Your result (left) did not match expected output (right) for the given queues:

Lift capacity = 4

 Floor    Queue
  12 .... [3]
  11 .... [5]
  10 .... [9, 7, 12, 9, 12]
   9 .... [2, 11, 12, 2]
   8 .... []
   7 .... [11, 2]
   6 .... [4, 1, 12, 4]
   5 .... [10]
   4 .... [1, 0, 12, 6, 3]
   3 .... [4, 12, 12, 8, 11]
   2 .... []
   1 .... [7, 9, 10]
   0 .... [9, 3, 2, 8, 10] */
