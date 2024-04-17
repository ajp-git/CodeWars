use num::bigint::BigInt;
use num::traits::{One, Zero};

fn matrix_multiply(a: &[BigInt], b: &[BigInt]) -> Vec<BigInt> {
    vec![
        &a[0] * &b[0] + &a[1] * &b[2],
        &a[0] * &b[1] + &a[1] * &b[3],
        &a[2] * &b[0] + &a[3] * &b[2],
        &a[2] * &b[1] + &a[3] * &b[3],
    ]
}

fn matrix_power(matrix: &[BigInt], n: i32) -> Vec<BigInt> {
    if n == 0 {
        return vec![BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one()];
    } else if n < 0 {
        let neg_power = matrix_power(matrix, -n);
        vec![
            neg_power[3].clone(),
            neg_power[2].clone(),
            neg_power[1].clone(),
            neg_power[0].clone(),
        ]
    } else if n == 1 {
        return matrix.to_vec();
    } else {
        let half_power = matrix_power(matrix, n / 2);
        let result = matrix_multiply(&half_power, &half_power);
        if n % 2 == 0 {
            result.to_vec()
        } else {
            matrix_multiply(&result, matrix)
        }
    }
}

fn fib(n: i32) -> BigInt {
    if n == 0 {
        return BigInt::zero();
    }
    if n == 1 {
        return BigInt::one();
    }
    let matrix = vec![BigInt::one(), BigInt::one(), BigInt::one(), BigInt::zero()];
    let powered_matrix = matrix_power(&matrix, n - 1);
    let fib_n = powered_matrix[0].clone();
    if n < 0 && n % 2 == 0 {
        -fib_n
    } else {
        fib_n
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::fib;
    use num::bigint::BigInt;
    use num::traits::{One, Zero};
    use std::str::FromStr;

    fn dotest(n: i32, expected: BigInt) {
        let actual = fib(n);
        assert!(
            actual == expected,
            "Test failed with n = {n}\nExpected \"{expected:?}\"\nBut got \"{actual:?}\""
        )
    }

    #[test]
    fn small_positive_numbers() {
        dotest(0, BigInt::zero());
        dotest(1, BigInt::one());
        dotest(2, BigInt::one());
        dotest(3, BigInt::from(2));
        dotest(4, BigInt::from(3));
        dotest(5, BigInt::from(5));
    }

    #[test]
    fn small_negative_numbers() {
        dotest(-1, BigInt::from(1));
        dotest(-6, BigInt::from(-8));
        dotest(-96, BigInt::from_str("-51680708854858323072").unwrap());
    }

    #[test]
    fn large_numbers() {
        dotest(
            -500,
            BigInt::from_str("-139423224561697880139724382870407283950070256587697307264108962948325571622863290691557658876222521294125")
            .unwrap()
        );

        dotest(
            1000,
            BigInt::from_str("43466557686937456435688527675040625802564660517371780402481729089536555417949051890403879840079255169295922593080322634775209689623239873322471161642996440906533187938298969649928516003704476137795166849228875")
            .unwrap()
        );
    }
}
