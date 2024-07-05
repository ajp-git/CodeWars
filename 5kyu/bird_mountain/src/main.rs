use std::{collections::HashMap, usize};

fn main() {
    let mountain = [
        "^^^",
        "^^^",
        "^^^",
    ];
    peak_height(&mountain);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coord {
    x:isize,
    y:isize,
}

fn peak_height(mountain: &[&str]) -> u32 {

    let xsize=mountain.len();
    let ysize=mountain[0].len();

    let mut peaks:HashMap<Coord, usize>=HashMap::new();

    /*for (y, line) in mountain.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                peaks.insert(Level { x: x as isize, y: y as isize, height: 1});
            }
        }
    }*/


    for x in 0..xsize as isize{
        for y in 0..ysize as isize{
            if mountain[y as usize].chars().nth(x as usize).unwrap() == '^' {
                peaks.insert(Coord { x: x, y: y}, usize::MAX);
            }
        }
    }
    
    let mut curr_level = 1;
    let mut finished = false;

    while !finished {
        print_mountain(&peaks, xsize, ysize);
        finished=true;

        for x in 0..xsize as isize {
            for y in 0..ysize as isize {
                match neighbours(&peaks, x, y) {
                    (level,2) if level < 100 => {
                        finished=false; 
                        peaks.insert(Coord { x: x, y: y }, level as usize +1); 
                    },
                    _ => {},
                }
            }
        }
    }

 0
}

// returns the min level and the number of this level
fn neighbours(peaks: &HashMap<Coord, usize>, x: isize, y: isize) -> (isize, usize) {

    let mut min_level = 0;
    let mut values=vec![0,0,0,0];
    let left = peaks.get(&Coord{x:x-1,y:y}).unwrap_or(&0);
    let top = peaks.get(&Coord{x:x,y:y-1}).unwrap_or(&0);
    let right = peaks.get(&Coord{x:x+1,y:y}).unwrap_or(&0);
    let bottom = peaks.get(&Coord{x:x,y:y+1}).unwrap_or(&0);

    values[0]=*left;
    values[1]=*top;
    values[2]=*right;
    values[3]=*bottom;

    let mut max_value=0;
    let mut max_count=0;

    for i in 0..values.len() {
        if values.iter().filter(|&&v|v==values[i]).count() >max_count {
            max_value=values[i];
            max_count=values.iter().filter(|&&v|v==values[i]).count();
            min_level=i as isize;
        }
    }
    println!("Max value {} and count {}",max_value, max_count);
    (0,0)
}
fn print_mountain(peaks: &HashMap<Coord, usize>, xsize:usize, ysize:usize){
    for row in -1..=ysize as isize {
        for col in -1..=xsize as isize {
            match peaks.get(&Coord { x: col, y: row }) {
                None => print!(" . "),
                Some(&level) => {
                    match level {
                        usize::MAX => print!(" ^ "),
                        _ => print!("{:3}", level),
                    }
                }
            }
        }
        println!();
    }
    println!();
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::peak_height;

    #[test]
    fn example_simple() {
        let mountain = [
            "^^^",
            "^^^",
            "^^^",
        ];
        assert_eq!(peak_height(&mountain), 2, "\nYour result (left) did not match expected result (right)");
    }

    #[test]
    fn example() {
        let mountain = [
            "^^^^^^        ",
            " ^^^^^^^^     ",
            "  ^^^^^^^     ",
            "  ^^^^^       ",
            "  ^^^^^^^^^^^ ",
            "  ^^^^^^      ",
            "  ^^^^        "
        ];
        assert_eq!(peak_height(&mountain), 3, "\nYour result (left) did not match expected result (right)");
    }
}
