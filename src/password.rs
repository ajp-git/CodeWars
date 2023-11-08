fn play_pass(s: &str, n: u32) -> String {
    let mut buf: Vec<char> = s.chars().collect();
    //  1  shift each letter by a given number but the transformed letter must be a letter (circular shift),
    //  2  replace each digit by its complement to 9,
    //  3  keep such as non alphabetic and non digit characters,
    //  4  downcase each letter in odd position, upcase each letter in even position (the first character is in position 0),

    println!("Before 1 : {:?}", buf);
    for i in 0..buf.len() {
        if buf[i].is_alphabetic() {
            buf[i] = (((buf[i] as u32 + n - 65) % (90 - 65 + 1) + 65) % 256) as u8 as char;
            if i % 2 == 1 && buf[i].is_uppercase() {
                buf[i] = (buf[i] as u8 + 32) as char;
            }
            if i % 2 == 0 && buf[i].is_lowercase() {
                buf[i] = (buf[i] as u8 - 32) as char;
            }
        }
        if buf[i].is_numeric() {
            buf[i] = ('9' as u8 - (buf[i] as u8) + 48) as char;
        }
    }
    buf.reverse();
    println!("After : {:?}", buf);
    //    reverse the whole result.
    buf.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, n: u32, exp: &str) -> () {
        println!(" s: {:?};", s);
        println!("n: {:?};", n);
        let ans = play_pass(s, n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest("I LOVE YOU!!!", 1, "!!!vPz fWpM J");
        dotest("I LOVE YOU!!!", 0, "!!!uOy eVoL I");
        dotest("AAABBCCY", 1, "zDdCcBbB");
    }
}
