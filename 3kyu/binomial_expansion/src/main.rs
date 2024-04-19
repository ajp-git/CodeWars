use regex::Regex;


fn expand(expr: &str) -> String {

    let re_binomial=Regex::new(r"\((-?\d*)(\w)([\-\+]?\d*)\)\^(\d+)").unwrap();

    let parts=re_binomial.captures(expr).unwrap();

    //println!("Parts : {:?}",parts);
    let first_coef=
        if parts[1].len()==1 && parts[1].starts_with('-') {
            -1
        } else {
            parts[1].parse::<i32>().unwrap_or(1)
        };

    let variable=parts[2].to_string();
    //let operation=parts[3].to_string();
    let second_coef =parts[3].parse::<i32>().unwrap_or(0);
    let n=parts[4].parse::<i32>().unwrap_or(1);

    let mut out:String=String::new();

    println!("({} {} {}) ^{}", first_coef, variable,second_coef,n);

    if n == 0 {
        return "1".to_string();
    }
    if n==1 {
        let mut sign="";
        if second_coef>0{
            sign="+";
        }
        let first_coef_text=
            match first_coef {
                -1 => "-".to_string(),
                1 => "".to_string(),
                _ => format!("{}",first_coef).to_string(),
            };
        return format!("{}{}{sign}{}",
        first_coef_text, variable,second_coef);
    }

    for k in 0..=n {

        // 
        let mut bi = binom(n, k);
//        print!("bi : {bi}");
        let coef = first_coef.pow((n-k)as u32);
//        print!("\tcoef^ : {}",coef);
//        print!("\tk : {}\n",k);
        bi*=coef;
        bi*=second_coef.pow(k as u32);
        let mut sign="";
        if bi>=0 && k>0 {
            sign="+";
        }
         out.push_str(format!("{}{}", 
            match bi {
                0 => "".to_string(),
                _ if n==k => format!("{sign}{bi}"),
                -1 => format!("-{variable}"),
                1 => variable.to_string(),
                _ => format!("{sign}{bi}{variable}"),
            }, 
            if n-k>1 {
                format!("^{}",n-k)
            } else {
                "".to_string()
            }).as_str());
    }
    out
}
// Computes a!-b!
fn binom(n:i32,k:i32) -> i32{
    // k : 0..=n
    //{n!}{k!(n-k)!} 
    let mut up=1;
    let mut down=1;
    for i in (k+1)..=n { up*=i; }
    for i in 1..=(n-k) { down*=i; }
    up/down
}

fn main() {
    println!(" = {}", expand("(-t+19)^1"));
    println!(" = {}", expand("(-s+7)^3"));
    
    /*
    println!(" = {}", expand_binomial("(x+1)^0"));
    println!(" = {}", expand_binomial("(x+1)^0"));
    println!(" = {}", expand_binomial("(x+1)^1"));
    println!(" = {}", expand_binomial("(x+1)^2"));
    println!(" = {}", expand_binomial("(x-1)^0"));
    println!(" = {}", expand_binomial("(-12t+43)^2"));
    println!(" = {}", expand_binomial("(x-1)^2"));
    println!(" = {}", expand_binomial("(2x-1)^3"));
    println!(" = {}", expand_binomial("(5m+3)^4"));
    println!(" = {}", expand_binomial("(2x-3)^3"));
    println!(" = {}", expand_binomial("(7x-7)^0"));
    println!(" = {}", expand_binomial("(-5m+3)^4"));
    println!(" = {}", expand_binomial("(-2k-3)^3"));
    println!(" = {}", expand_binomial("(-7x-7)^0"));
 */
}

#[cfg(test)]
mod tests {
    use super::expand;
    use super::binom;
        
    fn dotest(expr: &str, expected: &str) {
        let actual = expand(expr);
        assert!(actual == expected, 
            "With expr = \"{expr}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn binom_tests() {
        assert_eq!(binom(5,3),10);
        assert_eq!(binom(10,3),120);
        assert_eq!(binom(20,16),4845);
    }

    #[test]
    fn fixed_tests() {
        dotest("(x+1)^0", "1");
        dotest("(x+1)^1", "x+1");
        dotest("(x+1)^2", "x^2+2x+1");
        dotest("(x-1)^0", "1");
        dotest("(x-1)^1", "x-1");
        dotest("(x-1)^2", "x^2-2x+1");
        dotest("(5m+3)^4", "625m^4+1500m^3+1350m^2+540m+81");
        dotest("(2x-3)^3", "8x^3-36x^2+54x-27");
        dotest("(7x-7)^0", "1");
        dotest("(-5m+3)^4", "625m^4-1500m^3+1350m^2-540m+81");
        dotest("(-2k-3)^3", "-8k^3-36k^2-54k-27");
        dotest("(-7x-7)^0", "1");
    }
}
