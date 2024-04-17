fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    println!("Code : {}", code);
    println!("Iter : {}", iterations);
    println!("width : {}", width);
    println!("Height : {}", height);
    let mut c: Vec<u8> = vec![0; width * height];
    let mut s = String::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut ptr: Vec<usize> = vec![0; code.len()];
    let mut temp_ptr: Vec<usize> = vec![0; code.len()];
    let mut idx: u16 = 0;
    for (i, c) in code.chars().enumerate() {
        match c {
            '[' => temp_ptr.push(i),
            ']' => {
                let n = temp_ptr.pop().unwrap();
                ptr[n] = i + 1;
                ptr[i] = n + 1;
            }
            _ => {}
        }
    }
    let mut it_counter = 0;
    let mut pointer = 0;
    let vcode: Vec<char> = code.chars().collect();
    let vcode_len = vcode.len();
    while it_counter < iterations && pointer < vcode_len {
        match vcode[pointer] {
            'n' => {
                it_counter += 1;
                pointer += 1;
                y = if y == 0 { height - 1 } else { y - 1 };
            }
            's' => {
                it_counter += 1;
                pointer += 1;
                y = if y == height - 1 { 0 } else { y + 1 };
            }
            'w' => {
                it_counter += 1;
                pointer += 1;
                x = if x == 0 { width - 1 } else { x - 1 };
            }
            'e' => {
                it_counter += 1;
                pointer += 1;
                x = if x == width - 1 { 0 } else { x + 1 };
            }
            '[' => {
                ptr.push(pointer + 1);
                if c[x + width * y] == 0 {
                    pointer = ptr[pointer];
                } else {
                    pointer += 1;
                }
                it_counter += 1;
            }
            ']' => {
                if c[x + width * y] != 0 {
                    pointer = ptr[pointer];
                }
                it_counter += 1;
            }
            '*' => {
                c[x + width * y] = if c[x + width * y] == 1 { 0 } else { 1 };

                pointer += 1;
                it_counter += 1;
            }
            _ => {}
        }
    }

    //c.iter().for_each(|f|f.iter().)
    for i in 0..height {
        for j in 0..width {
            s += (format!("{}", c[j + width * i])).as_str();
        }
        s += format!("\r\n").as_str();
    }
    s.pop();
    s.pop();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cases() {
        assert_eq!(
            display_actual(&interpreter("*[s[e]*]", 9, 5, 5)),
            display_expected("10000\r\n10000\r\n10000\r\n00000\r\n00000"),
            "Your interpreter should adhere to the number of iterations specified"
        );
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9)), display_expected("111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should adhere to the number of iterations specified");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9)), display_expected("000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should initialize all cells in the datagrid to 0");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9)), display_expected("111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should traverse the 2D datagrid correctly");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
        assert_eq!(
            display_actual(&interpreter("*[s[e]*]", 5, 5, 5)),
            display_expected("10000\r\n10000\r\n00000\r\n00000\r\n00000"),
            "Your interpreter should initialize all cells in the datagrid to 0"
        );
    }

    /// Prints representation of datagrid - 0's are black and 1's are white.
    /// Note: it only works properly if your interpreter returns a representation
    /// of the datagrid in the correct format.
    fn pretty_print(datagrid: &str) -> &str {
        let rows = datagrid.split("\r\n");
        let mut output = String::new();
        output += "<pre>";
        for row in rows {
            for cell in row.chars() {
                output += "<span style=\"color:";
                output += if cell == '0' { "black" } else { "white" };
                output += ";background-color:";
                output += if cell == '0' { "black" } else { "white" };
                output += "\">xx</span>";
            }
            output += "<br />";
        }
        output += "</pre>";
        println!("{}", output);
        datagrid
    }

    /// Displays the grid the interpreter returns
    fn display_actual(actual: &str) -> &str {
        println!("You returned:");
        pretty_print(actual)
    }

    /// Displays the expected final state of datagrid
    fn display_expected(expected: &str) -> &str {
        println!("Expected final state of data grid:");
        pretty_print(expected)
    }
}
