use std::cmp::{max, min};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}
#[derive(Clone)]
struct Lift<'a> {
    dir: Direction,
    inside: Vec<u32>,
    capacity: u32,
    curr_level: u32,
    levels: u32,
    waiting: &'a [Vec<u32>], // Use the named lifetime parameter 'a
    visited: Vec<u32>,
}

impl Lift<'a> {
    fn new(capacity: u32, queues: &[Vec<u32>]) -> Self {
        let mut levels = 0;
        queues
            .iter()
            .map(|f| f.iter().map(|g| levels = max(*g, levels)));

        // levels should be the  max between size of the queues table and the number requested by people waiting
        levels = max(queues.len() as u32, levels);
        Lift {
            dir: Direction::Ascending,
            inside: Vec::new(),
            capacity: capacity,
            curr_level: 0,
            levels: levels,
            waiting: queues.clone(),
            visited: Vec::new(),
        }
    }
    fn get_current_direction(&self) -> Direction {
        self.dir
    }
    fn get_capacity(&self) -> u32 {
        self.capacity
    }
    fn add_people(&mut self, people: u32) -> bool {
        if self.inside.len() < self.capacity as usize {
            self.inside.push(people);
            return true;
        }
        false
    }
    /*
       fn people_waiting(&mut self, outside: Vec<u32>, level: u32) -> bool {
           self.outside[level].contains(&level)
       }
    */

    fn open_door(&mut self) {
        // when opening the door
        // update visited
        self.visited.push(self.curr_level);
        // inside queue pops requested people
        self.inside.retain(|&x| x == self.curr_level);
        // outside queue to inside queue :
        while self.people_waiting_above() || self.people_waiting_below() || self.inside.len() > 0 {}
        // - going the direction of the lift, by order until lift is full.
    }

    fn get_next_stop_above(&mut self) -> Option<u32> {
        let inside_next_stop = self.inside.iter().filter(|&&x| x > self.curr_level).min();
        let outside_next_stop = self
            .waiting
            .iter()
            .flat_map(|queue| queue.iter().filter(|&&x| x > self.curr_level).min());

        match (inside_next_stop, outside_next_stop) {
            (Some(&i), Some(&o)) => Some(cmp::min(i, o)),
            (None, Some(&o)) => Some(o),
            (Some(&i), None) => Some(i),
            _ => None,
        }
    }

    fn people_waiting_above(&mut self) -> bool {
        self.waiting[self.curr_level]
            .iter()
            .filter(|x| x > self.curr_level)
            .count()
            > 0
    }
    fn people_waiting_below(&mut self) -> bool {
        self.waiting[self.curr_level]
            .iter()
            .filter(|x| x < self.curr_level)
            .count()
            > 0
    }

    /*
        if dir == Direction::Ascending {
            if some_above {
                return true;
            }
            if some_below {
                self.dir = Direction::Descending;
                return true;
            }
            false
        } else {
            if some_below{
                return true;
            }
            if some_above
            {
                return true;
            }
            false
        }
    }
     */

    fn go_next_stop(&mut self) {
        // le next est le min interne et externe en montant
        // ou le max en descendant
        // if ascending :
        // next stop is min (inside>currlevel, waiting>currlevel)
        // if none -> Descending
        if self.dir == Direction::Ascending {
            self.curr_level = min(i, o);
        }
        // if descending :
        // next stop is max (inside<currlevel, waiting<currlevel)
        // if none -> Ascending
        if self.dir == Direction::Descending {
            let i = self.inside.iter().filter(|x| x < self.curr_level).max();
            let o = self.waiting.iter().filter(|x| x < self.curr_level).max();
            self.curr_level = max(i, o);
        }
    }

    fn run(&mut self) -> Vec<u32> {
        while (self.inside.len() + self.waiting.len()) > 0 {
            self.go_next_stop();
            self.open_door();
        }
        self.visited
    }
}

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    let mut stairs: Vec<Vec<u32>> = vec![Vec::new(); queues.len()];
    let mut lift: Lift = Lift::new(capacity, queues);

    lift.run()
}
// direction : Montant, descendat
// nombre de personnes dans le lift
// Next stop
// queue interne du lift
// bouton poussé dans le sens actuel

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
    fn test_down_and_down() {
        do_test(
            &[vec![], vec![0], vec![], vec![], vec![2], vec![3], vec![]],
            5,
            &[0, 5, 4, 3, 2, 1, 0],
        );
    }
}
