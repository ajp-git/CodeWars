fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {


    let instructions=code.chars().collect::<Vec<char>>();
    let mut ptr:usize=0;
    let mut input_ptr:usize=0;
    let mut data_ptr=0;
    let mut data:Vec<u8>=vec![0;256];
    let mut out:String=String::new();

    loop{
        match instructions[ptr] {
            '>' => { data_ptr+=1; if data_ptr>=data.len(){data.push(0)}},
            '<' => { data_ptr-=1;},
            '+' => { data[data_ptr]=data[data_ptr].wrapping_add(1)},
            '-' => { data[data_ptr]=if data[data_ptr]==0 { 255} else{data[data_ptr]-1}},
            '.' => {out.push(data[data_ptr] as char)},
            ',' => { data[data_ptr]=input[input_ptr];input_ptr+=1;},
            '[' => {
                if data[data_ptr]==0 { 
                    for i in ptr..instructions.len(){
                        if instructions[i]==']'{
                            ptr=i;
                            break;
                        }
                    }
                }
            },
            ']' => if data[data_ptr]!=0 {
                for i in (0..ptr).rev() {
                    if instructions[i]=='[' {
                        ptr=i;
                        break;
                    }
                }
            },
            _ => panic!("unknown instruction {}", instructions[ptr]),
        }
        ptr+=1;
        if ptr==instructions.len(){break;}
    }
    out.chars().map(|c|c as u8).collect::<Vec<u8>>()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_tests() {
        // Echo until byte 0 encountered
        assert_eq!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
        // Echo until byte 255 encountered
        assert_eq!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
        // Multiply two numbers
        assert_eq!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
    }
    
    // Takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
    // Without it, character-based tests are a pain   
    fn ez_vec(s: &str, i: u8) -> Vec<u8> {
      let mut v = s.to_string().into_bytes();
      v.push(i);
      v
    }   
}