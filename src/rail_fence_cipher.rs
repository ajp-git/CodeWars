#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Descending,
    Ascending,
}
struct RailPos {
    pos: usize,
    dir: Direction,
    num_rails: usize,
}

fn get_next_pos(rail_pos: &mut RailPos) -> &RailPos {
    match rail_pos.dir {
        Direction::Ascending => {
            if rail_pos.pos == rail_pos.num_rails - 1 {
                rail_pos.pos = rail_pos.pos - 1;
                rail_pos.dir = Direction::Descending;
            } else {
                rail_pos.pos += 1;
            }
        }
        Direction::Descending => {
            if rail_pos.pos == 0 {
                rail_pos.pos = 1;
                rail_pos.dir = Direction::Ascending
            } else {
                rail_pos.pos -= 1;
            }
        }
    }
    rail_pos
}

fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut out: Vec<Vec<char>> = vec![Vec::new(); num_rails];
    let mut out_string: String = String::new();
    let mut rail_pos = RailPos {
        pos: 0,
        dir: Direction::Ascending,
        num_rails,
    };

    for c in text.to_string().chars() {
        out[rail_pos.pos].push(c);
        get_next_pos(&mut rail_pos);
    }
    for i in out.iter() {
        for &j in i.iter() {
            out_string.push(j);
        }
    }
    out_string
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let zero: char = std::char::from_u32(0).unwrap();
    let mut out_string: String = String::new();
    let mut rails: Vec<Vec<char>> = vec![vec![zero; num_rails]; text.len()];

    let mut rail_pos = RailPos {
        pos: 0,
        dir: Direction::Ascending,
        num_rails,
    };
    let mut curr_index = 0;

    for _ in text.to_string().chars() {
        rails[curr_index][rail_pos.pos] = 'X';
        get_next_pos(&mut rail_pos);
        curr_index += 1;
    }
    curr_index = 0;
    for j in 0..num_rails {
        for i in 0..text.len() {
            match rails[i][j] {
                'X' => {
                    rails[i][j] = text.chars().nth(curr_index).unwrap();
                    curr_index += 1;
                }
                _ => {}
            }
        }
    }
    for i in 0..text.len() {
        for j in 0..num_rails {
            if rails[i][j] != zero {
                out_string.push(rails[i][j]);
            }
        }
    }
    out_string
}
/*
fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut out_string: String = String::new();
    let stext = text.to_string();

    let base = (num_rails + 1) * 2;
    for i in 0..text.len() {
        for y in 0..num_rails {
            if (i % base) == y || (i % base) == (base - y) {
                out_string.push(stext.chars().nth(i).unwrap());
            }
        }
    }

    out_string
}
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3),
            "WECRLTEERDSOEEFEAOCAIVDEN"
        );
        assert_eq!(
            decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3),
            "WEAREDISCOVEREDFLEEATONCE"
        );
        assert_eq!(
            encode_rail_fence_cipher("Hello, World!", 3),
            "Hoo!el,Wrdl l"
        );
        assert_eq!(
            decode_rail_fence_cipher("Hoo!el,Wrdl l", 3),
            "Hello, World!"
        );
    }
}
