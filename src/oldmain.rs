use std::cmp::max;

fn factorial(x: u32) -> Option<String> {
    //println!("Calling factorial with {}", x);
    if x <= 0 {
        return None;
    }
    if x == 1 {
        return Some("1".to_string());
    }
    Some(big_multiply(x.to_string(), factorial(x - 1).unwrap()))
}

fn big_multiply(a: String, b: String) -> String {
    //println!("Multiply {} with {}", a, b);

    //    let mut matrix: Vec<Vec<u8>> = Vec::new();

    let va = string_to_vector(a);
    let vb = string_to_vector(b);
    let mut vtotal: Vec<u8> = Vec::new();

    let mut total: String = String::new();

    /* 123 *456
       123
       456
       ---
        738
       6150
      49200
    */
    let mut vtotal: Vec<u8> = Vec::new();

    for i in (0..vb.len()).rev() {
        let mut carry: u8 = 0;
        let mut vstotal: Vec<u8> = Vec::new();
        println!("va.len() = {}\tvb.len()={}", va.len(), vb.len());
        vstotal.extend(vec![0; vb.len() - i - 1]);
        for j in (0..va.len()).rev() {
            //      println!("vb[{}]={}", j, vb[j]);
            vstotal.insert(0, (vb[i] * va[j] + carry) % 10);
            carry = (vb[i] * va[j] + carry) / 10;
        }
        if carry > 0 {
            vstotal.insert(0, carry);
        }
        for t in 0..(vstotal.len() - vtotal.len()) {
            vtotal.insert(0, 0);
        }
        //println!("vstotal: {:?}", vstotal);
        let mut carry: u8 = 0;
        for z in (0..vstotal.len()).rev() {
            let t_tot = vstotal[z] + vtotal[z] + carry;
            vtotal[z] = t_tot % 10;
            carry = t_tot / 10;
        }
        if carry > 0 {
            vtotal.insert(0, carry);
        }
    }
    vector_to_string(vtotal)
}

fn string_to_vector(s: String) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for c in s.chars() {
        v.push((c as u8) - 48)
    }
    //println!("string_to_vector s: {} v: {:?}", s, v);
    v
}

fn vector_to_string(v: Vec<u8>) -> String {
    let mut s = String::new();
    s.extend(v.iter().map(|&f| (f + 48) as char));
    println!("vector_to_string s: {}", s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_numbers() {
        //assert_eq!(factorial(1), Some(String::from("1")));
        assert_eq!(factorial(5), Some(String::from("120")));
        assert_eq!(factorial(9), Some(String::from("362880")));
        assert_eq!(factorial(15), Some(String::from("1307674368000")));
    }
}
