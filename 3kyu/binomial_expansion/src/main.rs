use regex::Regex;


fn expand_binomial(expr: &str) -> String {

    let re_binomial=Regex::new(r"\((-?\d*)(\w)([\-\+])(\d*)\)\^(\d+)").unwrap();

    let parts=re_binomial.captures(expr).unwrap();


    let first_coef=parts[1].parse::<i32>().unwrap_or(1);
    let variable=parts[2].to_string();
    let operation=parts[3].to_string();

    println!("{} {} {}", first_coef, variable, operation);


    "".to_string()
}

fn main() {
    println!("Hello, world!");

    expand_binomial("(x+1)^0");
    expand_binomial("(x+1)^1");
    expand_binomial("(x+1)^2");
    expand_binomial("(x-1)^0");
    expand_binomial("(x-1)^1");
    expand_binomial("(x-1)^2");
    expand_binomial("(5m+3)^4");
    expand_binomial("(2x-3)^3");
    expand_binomial("(7x-7)^0");
    expand_binomial("(-5m+3)^4");
    expand_binomial("(-2k-3)^3");
    expand_binomial("(-7x-7)^0");
}

#[cfg(test)]
mod tests {
    use super::expand_binomial;
        
    fn dotest(expr: &str, expected: &str) {
        let actual = expand_binomial(expr);
        assert!(actual == expected, 
            "With expr = \"{expr}\"\nExpected \"{expected}\" but got \"{actual}\"")
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
