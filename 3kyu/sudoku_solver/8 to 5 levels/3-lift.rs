use std::{collections::HashMap, cmp::{min, max}};

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    if queues.iter().all(|queue| queue.is_empty()) {
        let v=vec![0];
        return v;
    }
    let mut l=Lift::new(queues, capacity);
    l.run()
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
}

struct Lift{
    capacity: u32,
    queues:HashMap<u32,Vec<u32>>,
    direction: Direction,
    inside:Vec<u32>,
    level: u32,
    floors_visited:Vec<u32>,
}

impl Lift {
    fn new (queues: &[Vec<u32>], capacity: u32) -> Self {
        let mut t_queues=HashMap::new();

        for (i,v) in queues.iter().enumerate(){
            t_queues.insert(i as u32, v.clone());
        }
    
        Lift { capacity, queues: t_queues, direction: Direction::Up, inside: Vec::new(), level: 0, floors_visited: Vec::new() }
    }

    fn open_door(&mut self){
        self.floors_visited.push(self.level);
        self.let_out();
        self.let_in();
    }

    fn change_direction(&mut self) {
        if self.direction==Direction::Up {
            self.direction=Direction::Down;
        } else {
            self.direction=Direction::Up;
        }
    }

        // Find the nearest floor above the current level where someone wants to go up
        fn find_first_up(&self) -> Option<u32> {
            self.queues.iter()
                .filter(|(&floor, wanted_floors)| floor > self.level && wanted_floors.iter().any(|&wanted| wanted > floor))
                .min_by_key(|(&floor, _)| floor)
                .map(|(&floor, _)| floor)
        }
    
        // Find the nearest floor below the current level where someone wants to go down
        fn find_first_down(&self) -> Option<u32> {
            self.queues.iter()
                .filter(|(&floor, wanted_floors)| floor < self.level && wanted_floors.iter().any(|&wanted| wanted < floor))
                .max_by_key(|(&floor, _)| floor)
                .map(|(&floor, _)| floor)
        }
    
        // Find the lowest floor below the current level where someone wants to go up
        fn find_lowest_up(&self) -> Option<u32> {
            self.queues.iter()
                .filter(|(&floor, wanted_floors)| floor < self.level && wanted_floors.iter().any(|&wanted| wanted > floor))
                .min_by_key(|(&floor, _)| floor)
                .map(|(&floor, _)| floor)
        }
    
        // Find the highest floor above the current level where someone wants to go down
        fn find_highest_down(&self) -> Option<u32> {
            self.queues.iter()
                .filter(|(&floor, wanted_floors)| floor > self.level && wanted_floors.iter().any(|&wanted| wanted < floor))
                .max_by_key(|(&floor, _)| floor)
                .map(|(&floor, _)| floor)
        }
    
    fn someone_above (&self) -> bool {
        if self.level==self.queues.len() as u32 -1{return false};
        let mut someone:bool=false;
        
        for i in self.level+1..(self.queues.len() as u32) {
            if let Some(q) = self.queues.get(&i) {
                if !q.is_empty(){
                    someone=true;
                    break;
                }
            }
        }
        someone
    }

    fn someone_at_this_level_wants_up (&self) -> bool{
        if let Some(q)=self.queues.get(&self.level) {
            return q.iter().any(|&f| f>self.level);
        }
        false
    }

    fn someone_at_this_level_wants_down (&self) -> bool{
        if let Some(q)=self.queues.get(&self.level) {
            return q.iter().any(|&f| f<self.level);
        }
        false
    }

    fn someone_under (&self) -> bool {
        if self.level==0{return false};
        let mut someone:bool=false;
        
        for i in 0..=self.level-1 {
            if let Some(q) = self.queues.get(&i) {
                if !q.is_empty(){
                    someone=true;
                    break;
                }
            }
        }
        someone
    }

    fn someone_inside_wants_down(&self) -> bool{
        self.inside.iter().any(|&f|f<self.level)
    }
    
    fn someone_inside_wants_up(&self) -> bool{
        self.inside.iter().any(|&f|f>self.level)

    }
    
    fn get_next_level(&mut self)->Option<u32>{

        // inside first above
        let inside_first_up:Option<u32>=self.inside.iter().filter(|&&f| f>self.level).min().copied();
        let inside_first_down:Option<u32>=self.inside.iter().filter(|&&f| f<self.level).max().copied();

        if self.level==0{
            self.direction=Direction::Up;
        }
        if self.level==(self.queues.len()) as u32{
            self.direction=Direction::Down;
        }

        let x=self.level;

        let first_up_floor=self.find_first_up();
        let first_down_floor=self.find_first_down();
        let lowest_up_floor=self.find_lowest_up();
        let highest_down_floor=self.find_highest_down();

        let mut result:Option<u32>=None;
        match self.direction {
            Direction::Up =>{
                result=match (inside_first_up, first_up_floor, highest_down_floor,lowest_up_floor) {
                    (Some(a), Some(b),_,_) => Some(min(a,b)),
                    (Some(a),None,_,_) => Some(a),
                    (None,Some(a),_,_) => Some(a),
                    (None,None,Some(a),_) => {self.change_direction(); Some(a)},
                    (None,None,None,Some(a)) => Some(a),
                    _=>None,
                }
            },
            Direction::Down => {
                result=match (inside_first_down, first_down_floor, lowest_up_floor, highest_down_floor ) {
                    (Some(a),Some(b),_,_) => Some(max(a,b)),
                    (Some(a),None,_,_) => Some(a),
                    (None,Some(a),_,_) => Some(a),
                    (None,None,Some(a),_) => {self.change_direction(); Some(a)},
                    (None,None,None,Some(a)) => Some(a),
                    _ => None,
                }
            }
        }
        result
    }

    fn go_next_level(&mut self){
        if !self.is_someone_waiting(){
            if self.level!=0{
                self.level=0;
                self.floors_visited.push(0);
            }
            return;
        }
        match self.direction {
            Direction::Up => {
                if let Some(l)=self.get_next_level(){
                    self.level=l;
                }        
            },
            Direction::Down => {
                if let Some(l)=self.get_next_level(){
                    self.level=l;
                }       
            },
        }
    }

    fn let_in(&mut self) {
        if !self.someone_above() && !self.someone_at_this_level_wants_up() && !self.someone_inside_wants_up() && self.direction==Direction::Up {
            self.direction=Direction::Down;
        } else if !self.someone_under()&&!self.someone_at_this_level_wants_down() && !self.someone_inside_wants_down() && self.direction==Direction::Down{
            self.direction=Direction::Up;
        }
            
        let mut next_level_queue: Vec<usize> = Vec::new(); // Use usize for indexing

        if let Some(current_level_queue) = self.queues.get_mut(&self.level) {
            let mut i = 0;
            while i < current_level_queue.len() {
                // Calculate remaining slots directly here
                let remaining_slots = self.capacity as usize - self.inside.len();
                if remaining_slots == 0 {
                    break; 
                }
                let l = current_level_queue[i];
                match self.direction {
                    Direction::Up => {
                        if l > self.level {
                            self.inside.push(l);
                            next_level_queue.push(i); // Mark for removal
                        }
                    },
                    Direction::Down => {
                        if l < self.level {
                            self.inside.push(l);
                            next_level_queue.push(i); // Mark for removal
                        }
                    },
                }
                i += 1; // Increment index
            }

            // Remove elements in reverse order to avoid shifting the remaining elements
            for &index in next_level_queue.iter().rev() {
                current_level_queue.remove(index);
            }
        }
       
    }
       
    fn let_out(&mut self){

        let out = self.inside.iter().filter(|&&f|f==self.level).count();
        self.inside.retain(|&f| f!=self.level);

    }

    fn run(&mut self) -> Vec<u32>{
        while self.is_someone_waiting() {
            self.open_door();
            self.go_next_level();
        }
        self.floors_visited.dedup();
        self.floors_visited.clone()
    }

    fn is_someone_waiting(&self)->bool {
        
        let mut q_wait:u32=0;
        for (l,q) in &self.queues{
            q_wait+=q.len() as u32;
        }
        self.inside.len()!=0 || q_wait !=0
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
    fn test_fire() {
        do_test(&[vec![],vec![],vec![],vec![],vec![],vec![],vec![0,0,0,0,0,0,0,0,0]], 5, &[0,6,0,6,0]);
    }
    #[test]
    fn test_empty_buiding() {
        do_test(&[vec![],vec![],vec![],vec![],vec![],vec![],vec![]], 5, &[0]);
    }
    #[test]
    fn test_down_and_down() {
        do_test(&[vec![],vec![0],vec![],vec![],vec![2],vec![3],vec![]], 5, &[0, 5, 4, 3, 2, 1, 0]);
    }
    #[test]
    fn test_yoyo() {
        do_test(&[vec![],vec![],vec![4,4,4,4],vec![],vec![2,2,2,2],vec![],vec![]], 2, &[0, 2, 4, 2, 4, 2, 0]);
    }
    #[test]
    fn test_lift_full_up_and_down() {
        do_test(&[vec![3, 3, 3, 3, 3, 3],vec![],vec![],vec![],vec![],vec![4, 4, 4, 4, 4, 4],vec![]], 5, &[0, 3, 5, 4, 0, 3, 5, 4, 0]);
    }
    #[test]
    fn test_highlander() {
        do_test(&[vec![],vec![2],vec![3,3,3],vec![1],vec![],vec![],vec![]], 1, &[0, 1, 2, 3, 1, 2, 3, 2, 3, 0]);
    }
    #[test]
    fn test_random() {
        do_test(&[vec![8],vec![],vec![],vec![7,6,1],vec![7,3,9],vec![7,9,7,0],vec![2,3,9,2,10],vec![9],vec![2,4],vec![7],vec![1]], 5, &[0, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 5, 6, 7, 9, 10, 6, 5, 2, 0]);
    }
  
    #[test]
    fn test_random4() {
        do_test(
            &[vec![9, 3, 2, 8, 10],
            vec![7, 9, 10],
            vec![],
            vec![4, 12, 12, 8, 11],
            vec![1, 0, 12, 6, 3],
            vec![10],
            vec![4,1,12,4],
            vec![11,2],
            vec![],
            vec![2,11,12,2],
            vec![9,7,12,9,12],
            vec![5],
            vec![3]], 4, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 11, 10, 9, 7, 6, 5, 4, 3, 2, 1, 0, 1, 3, 4, 5, 6, 7, 9, 10, 11, 12, 10, 9, 6, 4, 3, 2, 1, 0, 3, 4, 5, 6, 8, 11, 12, 5, 10, 0]);
    }
}