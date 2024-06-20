use std::{collections::HashSet, io::Write};

// Solve the given puzzle in place, no need to return a copy
fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    solve_sudoku(puzzle);
}

fn solve_sudoku(puzzle: &mut [[u8; 9]; 9]) -> bool {
    for y in 0..9 {
        for x in 0..9 {
            if puzzle[y][x] == 0 {
                let possible_values = get_possible_values(puzzle, x, y);
                for val in possible_values {
                    puzzle[y][x] = val;
                    if solve_sudoku(puzzle) {
                        return true;
                    }
                    puzzle[y][x] = 0;
                }
                return false;
            }
        }
    }
    true
}

fn get_possible_values(grid: &[[u8; 9]; 9], x: usize, y: usize) -> Vec<u8> {
    let rect = get_missing_digits_in_rectangle(grid, x / 3, y / 3);
    let line = get_missing_digits_in_line(grid, y);
    let column = get_missing_digits_in_column(grid, x);

    let set1: HashSet<u8> = rect.into_iter().collect();
    let set2: HashSet<u8> = line.into_iter().collect();
    let set3: HashSet<u8> = column.into_iter().collect();

    set1.intersection(&set2)
        .cloned()
        .collect::<HashSet<u8>>()
        .intersection(&set3)
        .cloned()
        .collect::<Vec<u8>>()
}

fn get_missing_digits_in_line(grid: &[[u8; 9]; 9], line: usize) -> Vec<u8> {
    let mut found: HashSet<u8> = HashSet::new();

    for &digit in grid[line].iter() {
        if digit != 0 {
            found.insert(digit);
        }
    }

    (1..=9).filter(|v| !found.contains(v)).collect()
}

fn get_missing_digits_in_column(grid: &[[u8; 9]; 9], column: usize) -> Vec<u8> {
    let mut found: HashSet<u8> = HashSet::new();

    for y in 0..9 {
        if grid[y][column] != 0 {
            found.insert(grid[y][column]);
        }
    }

    (1..=9).filter(|v| !found.contains(v)).collect()
}

fn get_missing_digits_in_rectangle(grid: &[[u8; 9]; 9], x: usize, y: usize) -> Vec<u8> {
    let mut found: HashSet<u8> = HashSet::new();

    for curr_y in y * 3..y * 3 + 3 {
        for curr_x in x * 3..x * 3 + 3 {
            if grid[curr_y][curr_x] != 0 {
                found.insert(grid[curr_y][curr_x]);
            }
        }
    }

    (1..=9).filter(|v| !found.contains(v)).collect()
}

fn print_sudoku(puzzle: &[[u8; 9]; 9]) {
    clear_screen();
    for y in 0..9 {
        for x in 0..9 {
            print!("{} ", puzzle[y][x]);
        }
        println!();
    }
    println!();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

fn main() {
    let mut puzzle = [
        [6, 0, 5, 7, 2, 0, 0, 3, 9],
        [4, 0, 0, 0, 0, 5, 1, 0, 0],
        [0, 2, 0, 1, 0, 0, 0, 0, 4],
        [0, 9, 0, 0, 3, 0, 7, 0, 6],
        [1, 0, 0, 8, 0, 9, 0, 0, 5],
        [2, 0, 4, 0, 5, 0, 0, 8, 0],
        [8, 0, 0, 0, 0, 3, 0, 2, 0],
        [0, 0, 2, 9, 0, 0, 0, 0, 1],
        [3, 5, 0, 0, 6, 7, 4, 0, 8],
    ];

    if solve_sudoku(&mut puzzle) {
        println!("Solved Sudoku:");
        print_sudoku(&puzzle);
    } else {
        println!("No solution found.");
    }
}

#[cfg(test)]
mod sample_tests {
    use crate::{get_missing_digits_in_line, get_missing_digits_in_rectangle};

    use super::sudoku;

    #[test]
    fn check_missing_digit_in_line() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let result = get_missing_digits_in_line(&puzzle, 0);
        assert_eq!(result, vec![1, 4, 8]);
    }
    #[test]
    fn check_missing_digit_in_rectangle() {
        let puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];

        let result = get_missing_digits_in_rectangle(&puzzle, 0, 0);
        assert_eq!(result, vec![1, 3, 7, 8, 9]);
    }

    #[test]
    fn puzzle_1() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let solution = [
            [6, 1, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }

    #[test]
    fn puzzle_2() {
        let mut puzzle = [
            [0, 0, 8, 0, 3, 0, 5, 4, 0],
            [3, 0, 0, 4, 0, 7, 9, 0, 0],
            [4, 1, 0, 0, 0, 8, 0, 0, 2],
            [0, 4, 3, 5, 0, 2, 0, 6, 0],
            [5, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 6, 0, 3, 0, 9, 4, 1, 0],
            [1, 0, 0, 8, 0, 0, 0, 2, 7],
            [0, 0, 5, 6, 0, 3, 0, 0, 4],
            [0, 2, 9, 0, 7, 0, 8, 0, 0],
        ];
        let solution = [
            [9, 7, 8, 2, 3, 1, 5, 4, 6],
            [3, 5, 2, 4, 6, 7, 9, 8, 1],
            [4, 1, 6, 9, 5, 8, 3, 7, 2],
            [8, 4, 3, 5, 1, 2, 7, 6, 9],
            [5, 9, 1, 7, 4, 6, 2, 3, 8],
            [2, 6, 7, 3, 8, 9, 4, 1, 5],
            [1, 3, 4, 8, 9, 5, 6, 2, 7],
            [7, 8, 5, 6, 2, 3, 1, 9, 4],
            [6, 2, 9, 1, 7, 4, 8, 5, 3],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }
}
