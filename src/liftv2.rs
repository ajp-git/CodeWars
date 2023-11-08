use std::cmp::{max, min};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Direction {
    Ascending,
    Descending,
}
fn find_next_stop(
    waiting: &Vec<Option<Vec<u32>>>,
    inside: &Vec<u32>,
    dir: Direction,
    curr_level: u32,
    capacity: u32,
) -> (Option<u32>, Direction) {
    println!("\tfind_next_stop : in");

    if waiting.len() == 0 && inside.len() == 0 {
        return (None, Direction::Ascending);
    }

    let mut next_level: Option<u32> = None;
    let mut next_dir = dir;

    while next_level == None {
        if next_dir == Direction::Ascending {
            let mut o: Option<u32> = None; // up to go up
            let mut p: Option<u32> = None; // up to go dow
            let mut q: Option<u32> = None; // here to go down

            // 4 cases
            // 1 - First exit needed
            let i = inside.clone().into_iter().filter(|&x| x > curr_level).min();

            // 2 - someone up wanting to go up
            for (x, stair) in waiting.iter().enumerate().skip(curr_level as usize + 1) {
                if stair.is_some() {
                    if stair.as_ref().unwrap().iter().any(|&y| y > x as u32)
                        && inside.len() < capacity as usize
                    {
                        o = Some(x as u32);
                        break;
                    }
                }
            }
            // 3 - Noone up wanting to go up but someone up wanting to go down : go to the highest wanted to go down
            if o == None && i == None {
                for (x, stair) in waiting
                    .iter()
                    .enumerate()
                    .skip(curr_level as usize + 1)
                    .rev()
                {
                    if let Some(stair_values) = stair {
                        for &y in stair_values.iter() {
                            if y < x as u32 {
                                p = Some(max(p.unwrap_or(0), x as u32));
                            }
                            break; // We found the maximum value, so we can break the loop
                        }
                    }
                }
            }
            // 4 someone just here that want to go down
            if let Some(stair) = waiting.get(curr_level as usize).unwrap() {
                q = stair.iter().filter(|&&y| y < curr_level).max().copied();
            }

            match (i, o, p, q) {
                (Some(x), Some(y), _, _) => next_level = Some(min(x, y)),
                (Some(x), None, _, _) => next_level = Some(x),
                (None, Some(y), _, _) => next_level = Some(y),
                (None, None, None, Some(r)) => {
                    next_level = Some(r);
                    next_dir = Direction::Descending;
                }
                (None, None, Some(z), _) => {
                    next_level = Some(z);
                    next_dir = Direction::Descending;
                }
                (None, None, None, None) => {
                    next_level = None;
                    next_dir = Direction::Descending;
                }
            }
            println!(
                "\t\tAscending i:{:?} o:{:?} p{:?} q{:?} inside{:?} curr : {}",
                i, o, p, q, inside, curr_level
            );
        } else if next_dir == Direction::Descending {
            let mut o: Option<u32> = None;
            let mut p: Option<u32> = None;
            let mut q: Option<u32> = None;

            let i = inside.clone().into_iter().filter(|&x| x < curr_level).max();

            // 2 - someone down wanting to go down
            for (x, stair) in waiting.iter().enumerate().take(curr_level as usize).rev() {
                if stair.is_some() {
                    if stair.as_ref().unwrap().iter().any(|&y| y < x as u32) {
                        o = Some(x as u32);
                        break;
                    }
                }
            }
            // 3 - Noone up wanting to go up but someone up wanting to go down : go to the highest wanted to go down
            if o == None && i == None {
                for (x, stair) in waiting.iter().enumerate().take(curr_level as usize) {
                    if let Some(stair_values) = stair {
                        for &y in stair_values.iter() {
                            if y > x as u32 {
                                p = Some(min(p.unwrap_or(0), x as u32));
                            }
                            break; // We found the maximum value, so we can break the loop
                        }
                    }
                }
            }

            // 4 someone just here that want to go up
            if let Some(stair) = waiting.get(curr_level as usize).unwrap() {
                q = stair.iter().filter(|&&y| y > curr_level).min().copied();
            }

            match (i, o, p, q) {
                (Some(x), Some(y), _, _) => next_level = Some(max(x, y)),
                (Some(x), None, _, _) => next_level = Some(x),
                (None, Some(y), _, _) => next_level = Some(y),
                (None, None, None, Some(r)) => {
                    next_level = Some(r);
                    next_dir = Direction::Ascending;
                }
                (None, None, Some(z), _) => {
                    next_level = Some(z);
                    next_dir = Direction::Ascending;
                }
                (None, None, None, None) => {
                    next_level = None;
                    next_dir = Direction::Ascending;
                }
            }
            println!(
                "\t\tDescending i:{:?} o:{:?} p{:?} q{:?} inside{:?} curr : {}",
                i, o, p, q, inside, curr_level
            );
        }
    }
    println!("   Next stop should be {:?} and {:?}", next_level, next_dir);
    println!("\tfind_next_stop : out");

    (next_level, next_dir)
}

fn lift_needs_moving(waiting: &Vec<Option<Vec<u32>>>, inside: &Vec<u32>) -> bool {
    if inside.len() > 0 {
        return true;
    }
    waiting.iter().any(|stair| !stair.is_none())
}

fn let_people_outside(inside: &mut Vec<u32>, curr_level: u32) -> bool {
    let b = inside.len();
    inside.retain(|&x| x != curr_level);
    let c = inside.len();

    if b - c > 0 {
        println!("   {} leaving the lift", curr_level);
        println!("   {} people left the lift", b - c);
        return true;
    }
    return false;
}
/*
fn is_someone_above(    waiting: &mut Vec<Option<Vec<u32>>>,
    curr_level: u32,
    dir: Direction,
) -> Option<u32> {
    for (x, stair) in waiting.iter().enumerate().skip(curr_level as usize + 1) {
        if stair.is_some() {
            if stair.as_ref().unwrap().iter().any(|&y| y > x as u32)
            {
                return Some(x as u32);
            }
        }
    }
    None
}
 */
/*
fn change_direction(
    &inside: &Vec<u32>,
    waiting: &Vec<Option<Vec<u32>>>,
    dir: Direction,
    current_level: u32,
) -> bool {
    // if going up and there is waiting or inside up => still go up
    // else go down
    if dir == Direction::Ascending
        && (inside.iter().filter(|&&x| x > current_level).sum() > 0
            || waiting.iter().filter(|&&x| x.is_some()).iter().count() > 0)
    {
        return true;
    }
    if dir == Direction::Descending
        && (inside.iter().filter(|&&x| x < current_level)
            || waiting.iter().iter().filter(|x| x < current_level))
    {
        return true;
    }
}
 */
fn let_people_inside(
    inside: &mut Vec<u32>,
    waiting: &mut Vec<Option<Vec<u32>>>,
    curr_level: u32,
    capacity: u32,
    dir: Direction,
) -> bool {
    println!("\tlet_people_inside : in");
    if inside.len() < capacity as usize {
        if waiting.get(curr_level as usize).is_some() {
            if let Some(waiting_vec_option) = waiting.get_mut(curr_level as usize) {
                if let Some(waiting_vec_old) = waiting_vec_option.take() {
                    let mut waiting_vec_new = Vec::new();
                    for &x in waiting_vec_old.iter() {
                        let mut added = false;
                        if dir == Direction::Ascending && inside.len() < capacity as usize {
                            if x > curr_level {
                                inside.push(x);
                                added = true;
                                println!("    {} new inside, so inside len is {}", x, inside.len());
                            }
                        } else if dir == Direction::Descending {
                            if x < curr_level && inside.len() < capacity as usize {
                                inside.push(x);
                                added = true;
                                println!("    {} new inside, so inside len is {}", x, inside.len());
                            }
                        }
                        if !added {
                            waiting_vec_new.push(x);
                        }
                    }
                    if waiting_vec_new.len() > 0 {
                        waiting_vec_option.replace(waiting_vec_new);
                    }
                }
            }
        }
    }
    println!("\tlet_people_inside : out");

    true
}

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    let mut dir: Direction = Direction::Ascending;
    let mut inside: Vec<u32> = Vec::new();
    let mut curr_level: u32 = 0;
    let mut waiting: Vec<Option<Vec<u32>>> = queues
        .iter()
        .map(|q| {
            if !q.is_empty() {
                Some(q.to_vec())
            } else {
                None
            }
        })
        .collect();
    let mut visited: Vec<u32> = vec![0];

    while lift_needs_moving(&waiting, &inside) {
        println!("Lift is at level {}", curr_level);

        let_people_outside(&mut inside, curr_level);

        let (next_level, next_dir) = find_next_stop(&waiting, &inside, dir, curr_level, capacity);
        dir = next_dir;
        let_people_inside(&mut inside, &mut waiting, curr_level, capacity, dir);
        println!("   Lift is going at level {:?}", next_level);
        if curr_level != next_level.unwrap() {
            curr_level = next_level.unwrap();
            visited.push(curr_level);
        }
    }
    if visited.last() != Some(&0) {
        visited.push(0);
    }
    visited
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
    fn test_fire_drill() {
        do_test(
            &[
                vec![],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            5,
            &[
                0, 6, 5, 4, 3, 2, 1, 0, 5, 4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 3, 2, 1, 0, 1, 0,
            ],
        );
    }
}
