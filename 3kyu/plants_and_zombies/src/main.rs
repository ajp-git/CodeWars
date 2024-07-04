mod pnz {
    use std::isize;

    #[derive(Debug, Clone)]
    struct Zombie {
        round: usize,
        x: isize,
        y: isize,
        health: usize,
    }

    #[derive(Debug)]
    struct Shooter {
        x: isize,
        y: usize,
        num_shots: Option<usize>,
        is_s_shooter: bool,
    }
    pub fn plants_and_zombies(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> usize {
        /*
               New zombies appear at the specified move and start at the farthest right column of their row.
               Existing zombies move one space to the left each move.
               Numbered shooters fire a specified number of times per move, and S-shooters fire in three directions (straight, diagonally up, and diagonally down) once per move.
               Shooters fire in a specific order: numbered shooters first, then S-shooters from right to left and top to bottom.
               A zombie is eliminated when its health points reach 0.
               If a zombie reaches a shooter"s position, it destroys that shooter.
        */

        let xsize = lawn[0].len();
        print!("Lawn xsize: {}", xsize);
        let mut shooters = vec![];

        for (y, row) in lawn.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    ' ' => {}
                    'S' => shooters.push(Shooter {
                        x: x as isize,
                        y,
                        num_shots: None,
                        is_s_shooter: true,
                    }),
                    digit => shooters.push(Shooter {
                        x: x as isize,
                        y,
                        num_shots: Some(digit.to_digit(10).unwrap() as usize),
                        is_s_shooter: false,
                    }),
                }
            }
        }
        let mut zombies: Vec<Zombie> = zombies
            .iter()
            .map(|z| Zombie {
                round: z[0],
                x: xsize as isize,
                y: z[1] as isize,
                health: z[2],
            })
            .collect();

        let mut moves: usize = 0;
        draw_lawn(&shooters, &zombies, moves, xsize, lawn.len() as isize);

        loop {
            zombies
                .iter_mut()
                .filter(|z| z.round <= moves)
                .for_each(|z| z.x -= 1);

            if zombies.iter().find(|z| z.x < 0).is_some() {
                return moves;
            }

            zombies.iter().filter(|z| z.round <= moves).for_each(|z| {
                shooters.retain(|s| s.x != z.x || s.y != z.y as usize);
            });

            println!("Before shooting");
            draw_lawn(&shooters, &zombies, moves, xsize, lawn.len() as isize);
            // Fire !!!!!
            // Start with nshooters

            for s in shooters.iter().filter(|s| !s.is_s_shooter) {
                for _ in 0..s.num_shots.unwrap() {
                    for x in s.x..xsize as isize {
                        if let Some(z) = zombies.iter_mut().filter(|z| z.round <= moves).find(|z| {
                            z.round <= moves && z.x == x && z.y == s.y as isize && z.health > 0
                        }) {

                            z.health -= 1;
                            if z.health == 0 {
                                println!("\tZombie {}{}, health {} is destroyed by Nshooter {}", z.x, z.y, z.health, s.x);
                                break;
                            }
                            break;
                        }
                    }
                }
                zombies.retain(|z| z.health > 0);
            }

            // Then SShooters.

            for y in 0..lawn.len() {
                for x in (0..xsize).rev() {
                    if let Some(s) = shooters
                        .iter()
                        .filter(|s| s.is_s_shooter)
                        .find(|s| s.x == x as isize && s.y == y)
                    {
                        for sx in 1..=(xsize - x) {
                            let x:isize = x as isize;
                            let sx:isize = sx as isize;
                            if let Some(z) =
                                zombies.iter_mut().filter(|z| z.round <= moves).find(|z| {
                                    z.x == sx as isize + x as isize
                                        && z.y as isize == s.y as isize - sx as isize
                                        && z.health > 0
                                })
                            {
                                /*println!(
                                    "Zombie {},{}, health {} hit by Sshooter {},{}",
                                    z.x, z.y, z.health, s.x, s.y
                                );*/
                                z.health -= 1;
                                if z.health == 0 {
                                    println!("\tZombie {}{}, health {} is destroyed by Sshooter {}", z.x, z.y, z.health, s.x);
                                    break;
                                }    
                                break;
                            }
                            zombies.retain(|z| z.health > 0);
                            if zombies.len() == 0 {
                                return 0;
                            }
                        }

                        for sx in 1..=(xsize - x) {
                            let x:isize = x as isize;
                            let sx:isize = sx as isize;
                            if let Some(z) =
                                zombies.iter_mut().filter(|z| z.round <= moves).find(|z| {
                                    z.x == sx as isize + x as isize
                                        && z.y == s.y as isize
                                        && z.health > 0
                                })
                            {
                                /*println!(
                                    "Zombie {},{}, health {} hit by Sshooter {},{}",
                                    z.x, z.y, z.health, s.x, s.y
                                );*/
                                z.health -= 1;
                                if z.health == 0 {
                                    println!("\tZombie {}{}, health {} is destroyed by Sshooter {}", z.x, z.y, z.health, s.x);
                                    break;
                                }    
                                break;
                            }
                            zombies.retain(|z| z.health > 0);
                            if zombies.len() == 0 {
                                return 0;
                            }
                        }            
            
                        for sx in 1..=(xsize - x) {
                            let x:isize = x as isize;
                            let sx:isize = sx as isize;
                            if let Some(z) =
                                zombies.iter_mut().filter(|z| z.round <= moves).find(|z| {
                                    z.x == sx as isize + x as isize
                                        && z.y == s.y as isize + sx as isize
                                        && z.health > 0
                                })
                            {
                                /*println!(
                                    "Zombie {},{}, health {} hit by Sshooter {},{}",
                                    z.x, z.y, z.health, s.x, s.y
                                );*/
                                z.health -= 1;
                                if z.health == 0 {
                                    println!("\tZombie {}{}, health {} is destroyed by Sshooter {}", z.x, z.y, z.health, s.x);
                                    break;
                                }    
                                break;
                            }
                            zombies.retain(|z| z.health > 0);
                            if zombies.len() == 0 {
                                return 0;
                            }
                        }
                    }
                }
            }

            if zombies.len() == 0 {
                return 0;
            }
            draw_lawn(&shooters, &zombies, moves, xsize, lawn.len() as isize);
            moves += 1;
        }
    }
    fn draw_lawn(
        shooters: &Vec<Shooter>,
        zombies: &Vec<Zombie>,
        moves: usize,
        xsize: usize,
        ysize: isize,
    ) {
        println!(
            "\nMove: {}\tShooters: {}\tZombies: {}\n",
            moves,
            shooters.len(),
            zombies.len()
        );
        for y in 0..ysize {
            for x in 0..xsize {
                if let Some(s) = shooters
                    .iter()
                    .find(|s| s.x == x as isize && s.y == y as usize)
                {
                    match s.is_s_shooter {
                        true => {
                            print!("S   ",);
                        }
                        false => {
                            print!("N:{} ", s.num_shots.unwrap());
                        }
                    }
                } else if let Some(z) = zombies
                    .iter()
                    .find(|z| z.x == x as isize && z.y == y && z.round <= moves)
                {
                    print!("Z:{}", z.health);
                } else {
                    print!(".   ");
                }
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod example_tests {
    use super::*;

    /*fn random_tests(){
        
        lawn ["1S 1            ", "111SS           ", "2 1  S 1        ", "S14  S          ", "11 SS           ", "SS7             ", "4 SS            ", "421S            ", "51              ", "11 S            ", "4SS             ", "121  S 1        ", "14SS            ", "12  SS          ", "7 S S           ", " 24             ", "14  SS 1        "]
zombies [Zombie { round: 2, x: -1, y: 16, health: 5 }, Zombie { round: 7, x: 4, y: 11, health: 5 }, Zombie { round: 8, x: 5, y: 2, health: 12 }, Zombie { round: 8, x: 5, y: 6, health: 13 }, Zombie { round: 8, x: 5, y: 12, health: 23 }, Zombie { round: 9, x: 6, y: 16, health: 45 }, Zombie { round: 12, x: 9, y: 0, health: 3 }, Zombie { round: 12, x: 9, y: 1, health: 4 }, Zombie { round: 12, x: 9, y: 7, health: 13 }, Zombie { round: 12, x: 9, y: 13, health: 3 }, Zombie { round: 14, x: 11, y: 8, health: 22 }, Zombie { round: 14, x: 11, y: 10, health: 14 }, Zombie { round: 14, x: 11, y: 11, health: 36 }, Zombie { round: 14, x: 11, y: 15, health: 31 }, Zombie { round: 16, x: 13, y: 2, health: 31 }, Zombie { round: 16, x: 13, y: 5, health: 52 }, Zombie { round: 16, x: 13, y: 9, health: 13 }, Zombie { round: 16, x: 13, y: 12, health: 42 }, Zombie { round: 18, x: 15, y: 16, health: 51 }, Zombie { round: 19, x: 16, y: 3, health: 63 }, Zombie { round: 19, x: 16, y: 4, health: 34 }, Zombie { round: 19, x: 16, y: 14, health: 71 }, Zombie { round: 20, x: 16, y: 6, health: 50 }, Zombie { round: 21, x: 16, y: 7, health: 48 }, Zombie { round: 21, x: 16, y: 13, health: 30 }, Zombie { round: 22, x: 16, y: 1, health: 34 }, Zombie { round: 22, x: 16, y: 8, health: 37 }, Zombie { round: 22, x: 16, y: 11, health: 36 }, Zombie { round: 22, x: 16, y: 15, health: 36 }, Zombie { round: 23, x: 16, y: 0, health: 22 }, Zombie { round: 23, x: 16, y: 2, health: 30 }, Zombie { round: 23, x: 16, y: 9, health: 19 }, Zombie { round: 23, x: 16, y: 12, health: 42 }, Zombie { round: 24, x: 16, y: 3, health: 42 }, Zombie { round: 24, x: 16, y: 5, health: 62 }, Zombie { round: 24, x: 16, y: 16, health: 48 }, Zombie { round: 25, x: 16, y: 4, health: 26 }, Zombie { round: 25, x: 16, y: 14, health: 58 }, Zombie { round: 26, x: 16, y: 10, health: 55 }, Zombie { round: 27, x: 16, y: 6, health: 44 }, Zombie { round: 27, x: 16, y: 7, health: 48 }, Zombie { round: 27, x: 16, y: 13, health: 30 }, Zombie { round: 28, x: 16, y: 1, health: 31 }, Zombie { round: 28, x: 16, y: 8, health: 37 }, Zombie { round: 28, x: 16, y: 11, health: 36 }, Zombie { round: 28, x: 16, y: 15, health: 36 }, Zombie { round: 29, x: 16, y: 0, health: 19 }, Zombie { round: 29, x: 16, y: 2, health: 30 }, Zombie { round: 29, x: 16, y: 9, health: 18 }, Zombie { round: 29, x: 16, y: 12, health: 42 }, Zombie { round: 30, x: 16, y: 5, health: 56 }, Zombie { round: 30, x: 16, y: 16, health: 48 }, Zombie { round: 33, x: 16, y: 3, health: 48 }, Zombie { round: 33, x: 16, y: 4, health: 25 }, Zombie { round: 33, x: 16, y: 10, health: 37 }, Zombie { round: 34, x: 16, y: 14, health: 62 }, Zombie { round: 36, x: 16, y: 6, health: 38 }, Zombie { round: 36, x: 16, y: 7, health: 48 }, Zombie { round: 36, x: 16, y: 13, health: 30 }, Zombie { round: 37, x: 16, y: 11, health: 36 }, Zombie { round: 38, x: 16, y: 0, health: 19 }, Zombie { round: 38, x: 16, y: 1, health: 34 }, Zombie { round: 38, x: 16, y: 9, health: 18 }, Zombie { round: 38, x: 16, y: 15, health: 40 }, Zombie { round: 40, x: 16, y: 2, health: 34 }, Zombie { round: 40, x: 16, y: 8, health: 45 }, Zombie { round: 40, x: 16, y: 16, health: 48 }, Zombie { round: 41, x: 16, y: 3, health: 43 }, Zombie { round: 41, x: 16, y: 4, health: 24 }, Zombie { round: 41, x: 16, y: 5, health: 61 }, Zombie { round: 41, x: 16, y: 12, health: 53 }, Zombie { round: 43, x: 16, y: 10, health: 40 }, Zombie { round: 43, x: 16, y: 14, health: 56 }, Zombie { round: 46, x: 16, y: 7, health: 48 }, Zombie { round: 46, x: 16, y: 13, health: 30 }, Zombie { round: 47, x: 16, y: 6, health: 41 }, Zombie { round: 47, x: 16, y: 11, health: 36 }, Zombie { round: 48, x: 16, y: 0, health: 18 }, Zombie { round: 48, x: 16, y: 1, health: 31 }, Zombie { round: 48, x: 16, y: 9, health: 18 }, Zombie { round: 50, x: 16, y: 2, health: 31 }, Zombie { round: 50, x: 16, y: 8, health: 38 }, Zombie { round: 50, x: 16, y: 15, health: 42 }, Zombie { round: 51, x: 16, y: 4, health: 24 }, Zombie { round: 51, x: 16, y: 16, health: 54 }]
lawn [" 232               ", "31SSS              ", "13 2 S  1          ", "S4SS               ", "2S                 ", "6S  1              ", "5S   S             ", "4SS S              ", "2                  ", "5 S                ", "31SSS              ", "2 SS4 1S1          ", "1S11               ", "S2SS               ", "41S                ", "22                 ", "52 S               ", "1   31             ", "1 1S               "]
zombies [Zombie { round: 5, x: -1, y: 11, health: 6 }, Zombie { round: 6, x: 0, y: 17, health: 7 }, Zombie { round: 9, x: 3, y: 2, health: 35 }, Zombie { round: 10, x: 4, y: 5, health: 10 }, Zombie { round: 14, x: 8, y: 11, health: 71 }, Zombie { round: 14, x: 8, y: 16, health: 1 }, Zombie { round: 15, x: 9, y: 1, health: 12 }, Zombie { round: 15, x: 9, y: 6, health: 5 }, Zombie { round: 15, x: 9, y: 12, health: 20 }, Zombie { round: 16, x: 10, y: 9, health: 26 }, Zombie { round: 16, x: 10, y: 15, health: 7 }, Zombie { round: 16, x: 10, y: 17, health: 34 }, Zombie { round: 17, x: 11, y: 0, health: 26 }, Zombie { round: 17, x: 11, y: 2, health: 51 }, Zombie { round: 17, x: 11, y: 3, health: 25 }, Zombie { round: 17, x: 11, y: 4, health: 6 }, Zombie { round: 17, x: 11, y: 7, health: 40 }, Zombie { round: 17, x: 11, y: 13, health: 9 }, Zombie { round: 17, x: 11, y: 14, health: 25 }, Zombie { round: 17, x: 11, y: 18, health: 7 }, Zombie { round: 21, x: 15, y: 5, health: 70 }, Zombie { round: 23, x: 17, y: 10, health: 47 }, Zombie { round: 23, x: 17, y: 11, health: 77 }, Zombie { round: 24, x: 18, y: 1, health: 50 }, Zombie { round: 24, x: 18, y: 6, health: 51 }, Zombie { round: 24, x: 18, y: 8, health: 14 }, Zombie { round: 24, x: 18, y: 12, health: 28 }, Zombie { round: 24, x: 18, y: 16, health: 62 }, Zombie { round: 25, x: 19, y: 15, health: 28 }, Zombie { round: 25, x: 19, y: 17, health: 37 }, Zombie { round: 26, x: 19, y: 0, health: 52 }, Zombie { round: 26, x: 19, y: 2, health: 56 }, Zombie { round: 26, x: 19, y: 3, health: 49 }, Zombie { round: 26, x: 19, y: 4, health: 22 }, Zombie { round: 26, x: 19, y: 7, health: 50 }, Zombie { round: 26, x: 19, y: 9, health: 47 }, Zombie { round: 26, x: 19, y: 14, health: 44 }, Zombie { round: 26, x: 19, y: 18, health: 22 }, Zombie { round: 32, x: 19, y: 5, health: 66 }, Zombie { round: 32, x: 19, y: 10, health: 49 }, Zombie { round: 32, x: 19, y: 13, health: 52 }, Zombie { round: 34, x: 19, y: 1, health: 49 }, Zombie { round: 34, x: 19, y: 6, health: 49 }, Zombie { round: 34, x: 19, y: 8, health: 14 }, Zombie { round: 34, x: 19, y: 11, health: 85 }, Zombie { round: 36, x: 19, y: 12, health: 31 }, Zombie { round: 36, x: 19, y: 17, health: 35 }, Zombie { round: 37, x: 19, y: 0, health: 50 }, Zombie { round: 37, x: 19, y: 3, health: 49 }, Zombie { round: 37, x: 19, y: 4, health: 21 }, Zombie { round: 37, x: 19, y: 7, health: 49 }, Zombie { round: 37, x: 19, y: 14, health: 42 }, Zombie { round: 37, x: 19, y: 15, health: 31 }, Zombie { round: 37, x: 19, y: 16, health: 70 }, Zombie { round: 37, x: 19, y: 18, health: 22 }, Zombie { round: 38, x: 19, y: 2, health: 62 }, Zombie { round: 38, x: 19, y: 9, health: 48 }, Zombie { round: 41, x: 19, y: 5, health: 59 }, Zombie { round: 43, x: 19, y: 1, health: 49 }, Zombie { round: 43, x: 19, y: 11, health: 79 }, Zombie { round: 44, x: 19, y: 8, health: 16 }, Zombie { round: 44, x: 19, y: 10, health: 59 }, Zombie { round: 44, x: 19, y: 12, health: 29 }, Zombie { round: 44, x: 19, y: 13, health: 47 }, Zombie { round: 44, x: 19, y: 17, health: 35 }, Zombie { round: 45, x: 19, y: 0, health: 49 }, Zombie { round: 45, x: 19, y: 3, health: 49 }, Zombie { round: 45, x: 19, y: 4, health: 21 }, Zombie { round: 45, x: 19, y: 6, health: 59 }, Zombie { round: 45, x: 19, y: 7, health: 49 }, Zombie { round: 45, x: 19, y: 15, health: 29 }, Zombie { round: 45, x: 19, y: 16, health: 59 }, Zombie { round: 46, x: 19, y: 14, health: 47 }, Zombie { round: 47, x: 19, y: 2, health: 64 }, Zombie { round: 47, x: 19, y: 9, health: 48 }, Zombie { round: 47, x: 19, y: 18, health: 25 }]
assertion failed: `(left == right)`
  left: `34`,
 right: `0`
  
        let random_test1 = vec![
            "1S 1            ",
            "111SS           ",
            "2 1  S 1        ",
            "S14  S          ",
            "11 SS           ",
            "SS7             ",
            "4 SS            ",
            "421S            ",
            "51              ",
            "11 S            ",
            "4SS             ",
            "121  S 1        ",
            "14SS            ",
            "12  SS          ",
            "7 S S           ",
            " 24             ",
            "14  SS 1        "
        ], vec![],0);
    }*/
    #[test]
    fn submit_test1(){
        let lawn = vec![
            "311S S             ",
            "51SSS              ",
            "41S                ",
            "S5SS               ",
            "311SS S            ",
            "1SS5               ",
            "1S1S               ",
            "3 SS               ",
            "12S1               ",
            "11  S SS1          ",
            "  2SS              ",
            "S2 2S1             ",
            " 1 1 2             ",
            "13SS               ",
            "4                  ",
            "14 2SS S           ",
            "311 SSSS           ",
            "3 SS S             ",
            "S2SSS              ",
            "1S2 SS             ",
            "1 1 S SS1          ",
            "S2S S              ",
            "411 S  S1          ",
            "22SS S             ",
            "13SSSS             "];
        let zombies = vec![
            vec![1, 1, 126],
            vec![1, 5, 112],
            vec![1, 6, 56],
            vec![1, 7, 70], vec![1, 8, 70], vec![1, 9, 84], vec![1, 10, 56], vec![1, 11, 98], vec![1, 13, 84], vec![1, 14, 56], vec![1, 16, 126], vec![1, 18, 84], vec![1, 20, 84], vec![1, 21, 70], vec![1, 22, 126], vec![1, 23, 98], vec![2, 12, 59], vec![2, 15, 148], vec![2, 17, 88], vec![2, 19, 88], vec![2, 24, 118], vec![3, 0, 108], vec![3, 2, 93], vec![3, 3, 124], vec![4, 4, 130], vec![6, 5, 59], vec![6, 6, 29], vec![6, 7, 37], vec![6, 8, 37], vec![6, 9, 44], vec![6, 11, 51], vec![6, 13, 44], vec![6, 14, 29], vec![6, 16, 66], vec![6, 18, 44], vec![6, 20, 44], vec![6, 22, 66], vec![6, 23, 51], vec![7, 1, 73], vec![7, 10, 32], vec![7, 15, 75], vec![7, 17, 45], vec![7, 19, 45], vec![7, 21, 40], vec![7, 24, 60], vec![8, 0, 54], vec![8, 2, 46], vec![8, 3, 62], vec![8, 4, 57], vec![10, 12, 39], vec![14, 6, 28], vec![14, 7, 35], vec![14, 8, 35], vec![14, 9, 42], vec![14, 11, 50], vec![14, 14, 28], vec![14, 16, 64], vec![14, 18, 42], vec![14, 22, 64], vec![14, 23, 50], vec![15, 1, 65], vec![15, 10, 29], vec![15, 15, 71], vec![15, 17, 43], vec![15, 21, 37], vec![15, 24, 57], vec![17, 0, 51], vec![17, 2, 43], vec![17, 3, 57], vec![17, 5, 68], vec![17, 13, 51], vec![19, 12, 31], vec![19, 19, 56], vec![19, 20, 60], vec![20, 4, 74], vec![22, 6, 28], vec![22, 9, 42], vec![22, 11, 49], vec![22, 14, 28], vec![22, 18, 42], vec![22, 22, 63], vec![22, 23, 49], vec![25, 1, 64], vec![25, 7, 39], vec![25, 8, 39], vec![25, 15, 71], vec![25, 16, 70], vec![25, 17, 42], vec![25, 21, 35], vec![25, 24, 56], vec![27, 2, 42], vec![27, 3, 56], vec![27, 5, 59], vec![27, 10, 31], vec![27, 13, 45], vec![28, 0, 54], vec![28, 20, 42], vec![29, 12, 28], vec![29, 19, 46], vec![30, 4, 61], vec![33, 14, 28], vec![33, 23, 49], vec![34, 1, 63], vec![34, 7, 36], vec![34, 16, 65], vec![34, 17, 42], vec![34, 24, 56], vec![35, 2, 42], vec![35, 3, 56],
            vec![35, 6, 34],
            vec![35, 8, 39],
            vec![35, 10, 29],
            vec![35, 11, 59],
            vec![35, 13, 42],
            vec![35, 15, 77],
            vec![35, 21, 39],
            vec![35, 22, 76],
            vec![36, 0, 51],
            vec![36, 5, 63],
            vec![36, 18, 56],
            vec![36, 20, 42],
            vec![37, 9, 60],
            vec![37, 12, 28],
            vec![38, 4, 57],
            vec![38, 19, 48],
            vec![40, 14, 28],
            vec![40, 23, 49],
            vec![41, 7, 35],
            vec![41, 16, 63],
            vec![41, 17, 42],
            vec![43, 1, 70],
            vec![43, 2, 42], 
            vec![43, 3, 56],
            vec![43, 8, 37],
            vec![43, 10, 28],
            vec![43, 13, 42],
            vec![43, 15, 72],
            vec![43, 21, 36],
            vec![43, 22, 67],
            vec![46, 5, 58],
            vec![46, 6, 33],
            vec![46, 9, 42],
            vec![46, 11, 57],
            vec![46, 20, 42],
            vec![46, 24, 68],
            vec![47, 12, 28],
            vec![47, 18, 50],
            vec![48, 0, 59],
            vec![48, 4, 56],
            vec![48, 19, 43]];

        assert_eq!(pnz::plants_and_zombies(&lawn, &zombies), 0);
    }

    #[test]
    fn example_tests() {
        let example_tests: Vec<(Vec<&str>, Vec<Vec<usize>>, usize)> = vec![
            (
                vec!["2       ", "  S     ", "21  S   ", "13      ", "2 3     "],
                vec![
                    vec![0, 4, 28],
                    vec![1, 1, 6],
                    vec![2, 0, 10],
                    vec![2, 4, 15],
                    vec![3, 2, 16],
                    vec![3, 3, 13],
                ],
                10,
            ),
            (
                vec!["11      ", " 2S     ", "11S     ", "3       ", "13      "],
                vec![
                    vec![0, 3, 16],
                    vec![2, 2, 15],
                    vec![2, 1, 16],
                    vec![4, 4, 30],
                    vec![4, 2, 12],
                    vec![5, 0, 14],
                    vec![7, 3, 16],
                    vec![7, 0, 13],
                ],
                12,
            ),
            (
                vec![
                    "12        ",
                    "3S        ",
                    "2S        ",
                    "1S        ",
                    "2         ",
                    "3         ",
                ],
                vec![
                    vec![0, 0, 18],
                    vec![2, 3, 12],
                    vec![2, 5, 25],
                    vec![4, 2, 21],
                    vec![6, 1, 35],
                    vec![6, 4, 9],
                    vec![8, 0, 22],
                    vec![8, 1, 8],
                    vec![8, 2, 17],
                    vec![10, 3, 18],
                    vec![11, 0, 15],
                    vec![12, 4, 21],
                ],
                20,
            ),
            (
                vec!["12      ", "2S      ", "1S      ", "2S      ", "3       "],
                vec![
                    vec![0, 0, 15],
                    vec![1, 1, 18],
                    vec![2, 2, 14],
                    vec![3, 3, 15],
                    vec![4, 4, 13],
                    vec![5, 0, 12],
                    vec![6, 1, 19],
                    vec![7, 2, 11],
                    vec![8, 3, 17],
                    vec![9, 4, 18],
                    vec![10, 0, 15],
                    vec![11, 4, 14],
                ],
                19,
            ),
            (
                vec![
                    "1         ",
                    "SS        ",
                    "SSS       ",
                    "SSS       ",
                    "SS        ",
                    "1         ",
                ],
                vec![
                    vec![0, 2, 16],
                    vec![1, 3, 19],
                    vec![2, 0, 18],
                    vec![4, 2, 21],
                    vec![6, 3, 20],
                    vec![7, 5, 17],
                    vec![8, 1, 21],
                    vec![8, 2, 11],
                    vec![9, 0, 10],
                    vec![11, 4, 23],
                    vec![12, 1, 15],
                    vec![13, 3, 22],
                ],
                0,
            ),
        ];

        example_tests.into_iter().for_each(|(grid, zqueue, sol)| {
            assert_eq!(pnz::plants_and_zombies(&grid, &zqueue), sol)
        });
    }
    #[test]
    fn python_test_cases(){
        let example_tests: Vec<(Vec<&str>, Vec<Vec<usize>>, usize)> = vec![
            (
                vec![
                    "2       ",
                    "  S     ",
                    "21  S   ",
                    "13      ",
                    "2 3     "],
                vec![vec![0,4,28],vec![1,1,6],vec![2,0,10],vec![2,4,15],vec![3,2,16],vec![3,3,13]]
                ,10
            ),
            (
                vec![
                    "11      ",
                    " 2S     ",
                    "11S     ",
                    "3       ",
                    "13      "],
                vec![vec![0,3,16],vec![2,2,15],vec![2,1,16],vec![4,4,30],vec![4,2,12],vec![5,0,14],vec![7,3,16],vec![7,0,13]],
                12
            ),
            (
                vec![
                    "12        ",
                    "3S        ",
                    "2S        ",
                    "1S        ",
                    "2         ",
                    "3         "],
                vec![vec![0,0,18],vec![2,3,12],vec![2,5,25],vec![4,2,21],vec![6,1,35],vec![6,4,9],vec![8,0,22],vec![8,1,8],vec![8,2,17],vec![10,3,18],vec![11,0,15],vec![12,4,21]],
                20
            ),
            (
                vec![
                    "12      ",
                    "2S      ",
                    "1S      ",
                    "2S      ",
                    "3       "],
                vec![vec![0,0,15],vec![1,1,18],vec![2,2,14],vec![3,3,15],vec![4,4,13],vec![5,0,12],vec![6,1,19],vec![7,2,11],vec![8,3,17],vec![9,4,18],vec![10,0,15],vec![11,4,14]],
                19
            ),
            (
                vec![
                    "1         ",
                    "SS        ",
                    "SSS       ",
                    "SSS       ",
                    "SS        ",
                    "1         "],
                vec![vec![0,2,16],vec![1,3,19],vec![2,0,18],vec![4,2,21],vec![6,3,20],vec![7,5,17],vec![8,1,21],vec![8,2,11],vec![9,0,10],vec![11,4,23],vec![12,1,15],vec![13,3,22]],
                0
            ),
            (
                vec![
                    "121         ",
                    "22S         ",
                    "12S         ",
                    "3 S         ",
                    "12S         ",
                    "22          ",
                    "2SS         ",
                    "1S1         "],
                vec![vec![0,5,25],vec![2,4,26],vec![2,5,15],vec![3,0,41],vec![3,1,39],vec![5,2,27],vec![5,7,34],vec![7,0,23],vec![7,5,29],vec![8,2,25],vec![8,4,26],vec![11,5,22],vec![12,2,18],vec![12,6,20],vec![12,7,12],vec![13,3,26],vec![13,0,29],vec![16,5,14],vec![20,1,40],vec![20,2,28],vec![20,3,34],vec![21,7,16]],
                25
            ),
            (
                vec![
                    "42S1            ",
                    "6S              ",
                    "32 S  S         ",
                    "22 S S S        ",
                    "6               ",
                    "5 1 2           ",
                    "3 2 S  6        ",
                    "4 1 S           ",
                    "  8   S         ",
                    "8SS             "],
                vec![vec![0,8,48],vec![0,1,47],vec![0,2,55],vec![0,9,99],vec![0,6,58],vec![0,7,42],vec![0,0,92],vec![0,3,39],vec![0,5,66],vec![0,4,71],vec![2,3,36],vec![2,5,59],vec![2,8,36],vec![4,0,21],vec![4,7,21],vec![5,2,14],vec![6,1,48],vec![8,6,23],vec![11,1,41],vec![11,9,25],vec![11,3,53],vec![12,1,54],vec![12,0,75],vec![13,5,48],vec![13,9,113],vec![14,8,66],vec![15,7,82],vec![15,5,54],vec![16,0,76],vec![16,4,96],vec![16,9,42],vec![18,1,23],vec![18,8,91],vec![18,3,39],vec![19,0,16],vec![20,5,37]],
                34
            ),
            (
                vec![
                    "2121                ",
                    "6    S              ",
                    "3 2  S              ",
                    "22 S S              ",
                    "2 1 2S              ",
                    "311                 "],
                vec![vec![0,4,49],vec![0,0,88],vec![0,1,92],vec![0,2,75],vec![1,5,69],vec![1,3,78],vec![3,1,24],vec![4,2,18],vec![4,5,21],vec![6,0,51],vec![7,4,59],vec![7,1,29],vec![10,2,34],vec![11,5,37],vec![11,1,42],vec![13,0,44],vec![13,3,33],vec![13,2,59],vec![15,1,54],vec![16,0,24],vec![16,2,42],vec![17,5,36],vec![18,3,48],vec![18,4,39],vec![19,2,85]],
                35
            ),
            (
                vec![
                    "6   S             ",
                    " 55               ",
                    "3 S               ",
                    "31S               ",
                    "SSSS              ",
                    "1 S2              ",
                    "1 2 3             ",
                    " 4  S             "],
                vec![vec![0,0,70],vec![0,1,90],vec![0,2,40],vec![0,3,50],vec![0,4,40],vec![0,5,48],vec![0,6,50],vec![0,7,42],vec![3,0,50],vec![3,1,60],vec![3,2,42],vec![3,3,40],vec![3,4,36],vec![3,5,28],vec![3,6,50],vec![3,7,25],vec![7,0,25],vec![7,1,26],vec![7,2,35],vec![7,3,44],vec![7,4,26],vec![7,5,32],vec![7,6,42],vec![7,7,31],vec![12,0,33],vec![12,1,29],vec![12,2,34],vec![12,3,29],vec![12,4,35],vec![12,5,27],vec![12,6,22],vec![12,7,39]]
                ,25
             ),
             (
                vec![
                    "2 2 3  S                ",
                    " 1 3 1                  ",
                    "3 1  S                  ",
                    "11  4   S               ",
                    "22   S  S               ",
                    "3 1 2 S                 ",
                    "4    S                  ",
                    "1 1 1 1                 ",
                    "1 S 3                   ",
                    "1 S 2                   ",
                    "4      S                ",
                    "2 4 2   S               ",
                    "4       1               ",
                    "3 1S1                   ",
                    "4   S  2                ",
                    "11 3 S 2                "],
                vec![vec![0,0,96],vec![0,3,75],vec![0,7,82],vec![0,12,98],vec![0,14,104],vec![2,5,102],vec![2,6,51],vec![2,8,56],vec![3,1,74],vec![3,2,65],vec![3,3,58],vec![3,4,85],vec![3,9,44],vec![3,10,60],vec![4,14,63],vec![4,11,91],vec![5,13,120],vec![5,15,26],vec![7,1,43],vec![7,6,38],vec![7,9,61],vec![7,12,64],vec![9,0,69],vec![9,2,37],vec![9,14,51],vec![10,3,84],vec![10,7,68],vec![10,8,82],vec![10,11,77],vec![10,15,101],vec![12,4,100],vec![13,5,70],vec![13,6,76],vec![14,7,54]],
                31
        )];
        example_tests.into_iter().for_each(|(grid, zqueue, sol)| {
            assert_eq!(pnz::plants_and_zombies(&grid, &zqueue), sol)
        });    }
}
