fn main() {
    println!("Hello, world!");
}

struct CaesarCipher {
    shift: u32,
}

impl CaesarCipher {
    fn new(shift: u32) -> CaesarCipher {
        CaesarCipher { shift }
    }

    fn encode(&self, message: &str) -> String {
        message.to_uppercase().chars().map(|c|{
            match c {
                ('A'..='Z') => {
                    let c = c as u8;
                    let c=c-b'A';
                    let new_c = ((c as u32 + self.shift) % 26) as u8 + b'A';
                    new_c as char
                },
                _ => c,
            }
        }).collect()
    }

    fn decode(&self, message: &str) -> String {
        message.chars().map(|c|{
            match c {
                ('A'..='Z') => {
                    let c = c as u8;
                    let c=c-b'A';
                    let new_c = ((c as i32 - self.shift as i32 +26) % 26) as u8 + b'A';
                    new_c as char        
                },
                 _ => c,
            }
        }).collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    fn dotest(shift: u32, message: &str, expected_encoded: &str, expected_decoded: &str) {
        let cipher = CaesarCipher::new(shift);
        assert_eq!(
            cipher.encode(&message),
            expected_encoded,
            "Encoded message does not match expected message for input: \"{message}\""
        );
        assert_eq!(
            cipher.decode(&expected_encoded),
            expected_decoded,
            "Decoded message does not match expected message for input: \"{expected_encoded}\""
        );
    }

    fn dotest_encode(shift: u32, message: &str, expected_encoded: &str, expected_decoded: &str) {
        let cipher = CaesarCipher::new(shift);
        assert_eq!(
            cipher.encode(&message),
            expected_encoded,
            "Encoded message does not match expected message for input: \"{message}\""
        );
    }

    #[test]
    fn shift_1() {
        dotest_encode(5, "Codewars", "HTIJBFWX", "CODEWARS");
    }
    #[test]
    fn shift_of_5() {
        dotest(5, "Codewars", "HTIJBFWX", "CODEWARS");
        dotest(5, "WAFFLES", "BFKKQJX", "WAFFLES");
        dotest(
            5,
            "IT'S A SHIFT CIPHER!!",
            "NY'X F XMNKY HNUMJW!!",
            "IT'S A SHIFT CIPHER!!",
        );
        dotest(
            5,
            "IT\'S A SHIFT CIPHER!!",
            "NY\'X F XMNKY HNUMJW!!",
            "IT\'S A SHIFT CIPHER!!",
        );
    }

    #[test]
    fn shift_of_13() {
        dotest(13, "CNAPNXRF", "PANCAKES", "CNAPNXRF");
        dotest(13, "JAVASCRIPT", "WNINFPEVCG", "JAVASCRIPT");
    }

    #[test]
    fn simple_test() {
        dotest(0, "Codewars", "CODEWARS", "CODEWARS");
        dotest(1, "", "", "");
        dotest(2, " ", " ", " ");
    }
}