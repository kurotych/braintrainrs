use num::{bigint::ToBigUint, rational::BigRational, traits::ToPrimitive, BigUint};

fn factorial(n: BigUint) -> BigUint {
    let big_1 = 1u32.into();
    let big_2 = 2u32.into();
    if n < big_2 {
        big_1
    } else {
        let prev_factorial = factorial(n.clone() - big_1);
        n * prev_factorial
    }
}

fn n_choose_k(n: BigUint, k: BigUint) -> BigUint {
    factorial(n.clone()) / (factorial(k.clone()) * factorial(n - k))
}

fn calc_binominal_sequence(n: BigUint) -> Vec<BigUint> {
    if n == 0u32.into() {
        return vec![0u32.into()];
    }
    let mut res = Vec::new();
    for k in 0..=n.to_u64().unwrap() {
        res.push(n_choose_k(n.clone(), k.into()));
    }
    res
}

fn main() {
    let n: BigUint = 1000u32.into();
    let denominator = ToBigUint::to_biguint(&2).unwrap().pow(n.to_u32().unwrap());

    let binom_seq = calc_binominal_sequence(n);
    let mut sum: BigUint = 0u32.into();
    for i in 400..=600 {
        sum += &binom_seq[i];
    }
    println!(
        "Answer is {:?} percent",
        BigRational::new_raw(sum.into(), denominator.into())
            .to_f64()
            .unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_n_choose_k() {
        assert_eq!(n_choose_k(5u32.into(), 2u32.into()), 10u32.into());

        println!("{}", n_choose_k(100u32.into(), 30u32.into()));
    }
    #[test]
    fn test_calc_binominal_sequence() {
        assert_eq!(
            calc_binominal_sequence(5u32.into()),
            vec![
                1u32.into(),
                5u32.into(),
                10u32.into(),
                10u32.into(),
                5u32.into(),
                1u32.into()
            ]
        );
        assert_eq!(calc_binominal_sequence(100u32.into()).len(), 101);
    }
}
