fn multiply(a: &str, b: &str) -> String {
    let va = str_to_vector(a);
    let vb = str_to_vector(b);
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
        for _ in 0..(vstotal.len() - vtotal.len()) {
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

fn str_to_vector(s: &str) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for c in s.to_string().chars() {
        v.push((c as u8) - 48)
    }
    //println!("string_to_vector s: {} v: {:?}", s, v);
    v
}

fn vector_to_string(v: Vec<u8>) -> String {
    let mut s = String::new();
    s.extend(v.iter().map(|&f| (f + 48) as char));
    let ret_s = s.trim_start_matches('0');
    println!("vector_to_string s: {}", ret_s);
    if ret_s.len() == 0 {
        return "0".to_string();
    }
    ret_s.to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::multiply;

    fn do_test(a: &str, b: &str, expected: &str) {
        let actual = multiply(&a, &b);
        assert_eq!(actual, expected,
               "\n\nMultiplying a*b with\na = {a}\nb = {b}\nshould return: {expected}\ninstead got: {actual}");
    }

    #[test]
    fn simple_cases() {
        //        input       expected
        do_test("2", "3", "6");
        do_test("30", "69", "2070");
        do_test("11", "85", "935");
    }

    #[test]
    fn edge_cases() {
        do_test("2", "0", "0");
        do_test("0", "30", "0");
        do_test("0000001", "3", "3");
        do_test("1009", "03", "3027");
    }

    #[test]
    fn big_numbers() {
        do_test("98765", "56894", "5619135910");
        do_test(
            "9007199254740991",
            "9007199254740991",
            "81129638414606663681390495662081",
        );
        do_test(
            "1020303004875647366210",
            "2774537626200857473632627613",
            "2830869077153280552556547081187254342445169156730",
        );
        do_test(
            "58608473622772837728372827",
            "7586374672263726736374",
            "444625839871840560024489175424316205566214109298",
        );
    }
}
