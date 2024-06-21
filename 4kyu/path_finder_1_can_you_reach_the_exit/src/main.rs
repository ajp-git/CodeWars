use std::collections::HashSet;

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

fn path_finder(maze: &str) -> bool {
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

    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut stack: Vec<Coordinates> = vec![Coordinates { x: 0, y: 0 }];
    while let Some(curr_pos) = stack.pop() {
        visited.insert(curr_pos.clone());
        for dir in get_direction(&grid, &curr_pos) {
            if !visited.contains(&dir) {
                stack.push(dir);
            }
        }
        //draw_maze(&grid, &curr_pos);

        if curr_pos
            == (Coordinates {
                x: grid.len() - 1,
                y: grid.len() - 1,
            })
        {
            return true;
        }
    }
    false
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

#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn basic() {
        test_maze(
            "\
            .W.\n\
            .W.\n\
            ...\
            ",
            true,
        );

        test_maze(
            "\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\
            ",
            true,
        );

        test_maze(
            "\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            .....W\n\
            ....W.\
            ",
            false,
        );
    }

    fn test_maze(maze: &str, expect: bool) {
        let actual = path_finder(maze);

        assert!(
            actual == expect,
            "Test failed!\n\
             Got:      {}\n\
             Expected: {}\n\
             Maze was: \n\
             {}",
            actual,
            expect,
            maze
        );
    }
}
