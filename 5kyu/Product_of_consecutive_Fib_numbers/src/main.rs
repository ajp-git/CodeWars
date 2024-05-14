use core::prelude::v1;

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

fn product_fib(prod: u64) -> (u64, u64, bool) {

    let mut min_above=u64::MAX;
    let mut min_a=0;
    let mut min_b=0;

    let mut v_fib: Vec<u64> = vec![1, 1];
    while *v_fib.last().unwrap() < prod {
        let next_fib = v_fib[v_fib.len() - 1] + v_fib[v_fib.len() - 2];
        v_fib.push(next_fib);
    }

    let v1: Vec<usize> = (0..v_fib.len()).collect();
    let v2 = v1.clone();

    let result = v1.iter()
        .cartesian_product(v2.iter())
        .find(|(&a, &b)| v_fib[a] * v_fib[b] == prod);

    match result {
        Some((a, b)) => {
            return (v_fib[*a] as u64,v_fib[*b] as u64,true);
        },
        None => {},
    }
    
    for x in v1.iter().take(v1.len()-2) {
        let p = v_fib[*x]*v_fib[*x+1];
        if p>prod && p < min_above{
            min_above=p;
            min_a=v_fib[*x];
            min_b=v_fib[*x+1];
        }
    }

    (min_a as u64,min_b as u64,false)
}

fn fib(n:u64)->u64{
    if n==0 || n==1 {return 1};
    fib(n-1)+fib(n-2)
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}