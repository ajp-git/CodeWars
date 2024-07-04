use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Debug)]
enum MazeCell {
    Free,
    Wall,
    //    Exit,
}
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
}

fn path_finder(maze: &str) -> Option<u32> {
    let mut grid: Vec<Vec<MazeCell>> = Vec::new();
    for line in maze.lines() {
        let mut maze_line: Vec<MazeCell> = Vec::new();
        for c in line.chars() {
            match c {
                '.' => maze_line.push(MazeCell::Free),
                'W' => maze_line.push(MazeCell::Wall),
                _ => panic!("Invalid value in maze cell: {}", c),
            }
        }
        grid.push(maze_line);
    }
    let start: Coordinates = Coordinates { x: 0, y: 0 };

    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut queue: VecDeque<(Coordinates, Vec<Coordinates>)> = VecDeque::new();
    queue.push_back((start.clone(), vec![start]));

    while let Some((curr_pos, path)) = queue.pop_front() {
        if curr_pos
            == (Coordinates {
                x: grid.len() - 1,
                y: grid.len() - 1,
            })
        {
            println!("Path {:?}", path);
            return Some(path.len() as u32 - 1);
        }

        if !visited.insert(curr_pos.clone()) {
            continue;
        }
        for dir in get_direction(&grid, &curr_pos) {
            if !visited.contains(&dir) {
                let mut new_path = path.clone();
                new_path.push(dir.clone());
                queue.push_back((dir, new_path));
            }
        }
        draw_maze(&grid, &curr_pos);
    }
    None
}
fn draw_maze(grid: &Vec<Vec<MazeCell>>, cursor: &Coordinates) {
    for (y, v) in grid.iter().enumerate() {
        for (x, cell) in v.iter().enumerate() {
            if x == cursor.x && y == cursor.y {
                print!("X")
            } else {
                match cell {
                    MazeCell::Free => print!("."),
                    MazeCell::Wall => print!("W"),
                }
            }
        }
        println!();
    }

    println!();
}

fn get_direction(maze: &Vec<Vec<MazeCell>>, pos: &Coordinates) -> Vec<Coordinates> {
    let mut directions: Vec<Coordinates> = Vec::new();

    // North
    if pos.y != 0 {
        if maze[pos.y - 1][pos.x] == MazeCell::Free {
            directions.push(Coordinates {
                x: pos.x,
                y: pos.y - 1,
            });
        }
    }
    // South
    if pos.y < maze.len() - 1 {
        if maze[pos.y + 1][pos.x] == MazeCell::Free {
            directions.push(Coordinates {
                x: pos.x,
                y: pos.y + 1,
            });
        }
    }
    //East
    if pos.x < maze[0].len() - 1 {
        if maze[pos.y][pos.x + 1] == MazeCell::Free {
            directions.push(Coordinates {
                x: pos.x + 1,
                y: pos.y,
            })
        }
    }
    // West
    if pos.x > 0 {
        if maze[pos.y][pos.x - 1] == MazeCell::Free {
            directions.push(Coordinates {
                x: pos.x - 1,
                y: pos.y,
            })
        }
    }
    directions
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn fixed_tests() {
        assert_eq!(
            path_finder(".W.\n.W.\n..."),
            Some(4),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder(".W.\n.W.\nW.."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("......\n......\n......\n......\n......\n......"),
            Some(10),
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            path_finder("......\n......\n......\n......\n.....W\n....W."),
            None,
            "\nYour answer (left) is not the expected answer (right)."
        );
    }
}
