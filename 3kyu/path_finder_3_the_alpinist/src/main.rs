use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
    z: u32,
}
impl PartialEq for Coordinates {
    fn eq(&self, other: &Coordinates) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coordinates {}
impl Hash for Coordinates {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ClimbState {
    cost: u32,
    position: Coordinates,
}

impl Ord for ClimbState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Note: we want the smallest cost, so we reverse the order
    }
}

impl PartialOrd for ClimbState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn path_finder(area: &Vec<Vec<u32>>) -> u32 {
    
    let lenght=area.len()-1;
    let target=Coordinates { x: lenght, y: lenght, z: area[lenght][lenght] };
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    let mut heap = BinaryHeap::new();
    let mut distances = vec![vec![u32::MAX; area[0].len()]; area.len()];
    
    distances[0][0] = 0;

    heap.push(ClimbState { cost: 0, position: Coordinates { x: 0, y: 0, z: 0 } });

    while let Some(ClimbState { cost, position }) = heap.pop() {
        if position == target {
            return cost;
        }

        if cost > distances[position.x][position.y] {
            continue;
        }

        for &(dx, dy) in &directions {
            let next_x = position.x as isize + dx;
            let next_y = position.y as isize + dy;

            if next_x >= 0 && next_x < area.len() as isize && next_y >= 0 && next_y < area[0].len() as isize {
                let next_x = next_x as usize;
                let next_y = next_y as usize;
                let next_cost:u32 = cost + (area[position.x][position.y] as i32- area[next_x][next_y] as i32).unsigned_abs();

                if next_cost < distances[next_x][next_y] {
                    distances[next_x][next_y] = next_cost;
                    heap.push(ClimbState { cost: next_cost, position: Coordinates { x: next_x, y: next_y, z: area[next_x][next_y] } });
                }
            }
        }
    }

    u32::MAX 
}


fn main() {
    let area: Vec<Vec<u32>> = vec![
        vec![7, 0, 0, 0, 0, 0],
        vec![0, 7, 7, 7, 7, 0],
        vec![0, 7, 7, 7, 7, 0],
        vec![0, 7, 7, 7, 7, 0],
        vec![0, 7, 7, 7, 7, 0],
        vec![0, 0, 0, 0, 0, 7],
    ];
    path_finder(&area);
}



/*fn get_direction(maze: &Vec<Vec<MazeCell>>, pos: &Coordinates) -> Vec<Coordinates> {
    let mut directions: Vec<Coordinates> = Vec::new();

    // North
    if pos.y != 0 {
        directions.push(Coordinates {
            x: pos.x,
            y: pos.y - 1,
            z: (maze[pos.y][pos.x].height - maze[pos.y - 1][pos.x].height).abs(),
        });
    }
    // South
    if pos.y < maze.len() - 1 {
        directions.push(Coordinates {
            x: pos.x,
            y: pos.y + 1,
            z: (maze[pos.y][pos.x].height - maze[pos.y + 1][pos.x].height).abs(),
        });
    }
    //East
    if pos.x < maze[0].len() - 1 {
        directions.push(Coordinates {
            x: pos.x + 1,
            y: pos.y,
            z: (maze[pos.y][pos.x].height - maze[pos.y][pos.x + 1].height).abs(),
        })
    }
    // West
    if pos.x > 0 {
        directions.push(Coordinates {
            x: pos.x - 1,
            y: pos.y,
            z: (maze[pos.y][pos.x].height - maze[pos.y][pos.x - 1].height).abs(),
        })
    }
    directions
}
*/



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
    }
    #[test]
    fn test_basic2() {
        let area: Vec<Vec<u32>> = vec![
            vec![7, 0, 0, 0, 0, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 7, 7, 7, 7, 0],
            vec![0, 0, 0, 0, 0, 7],
        ];
        test_equal(&area, path_finder(&area), 14);
    }
    #[test]
    fn test_basic3() {
        let area: Vec<Vec<u32>> = vec![
            vec![7, 7, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 0, 0, 0],
            vec![0, 0, 7, 7, 7, 7],
        ];
        test_equal(&area, path_finder(&area), 0);
    }
    #[test]
    fn test_basic4() {
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
