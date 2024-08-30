mod constants;

use std::{str::FromStr, time::Instant};

use constants::HIGHLY_COMPOSITE_NUMBERS;
use num::BigInt;

fn is_square_number(number: BigInt) -> bool {
    let sqrt: BigInt = number.sqrt();
    &sqrt * &sqrt == number
}

fn get_square_factors(input: BigInt) -> Vec<BigInt> {
    let sqrt: BigInt = input.sqrt();
    let mut number: BigInt = BigInt::from(2);
    let mut square_factors: Vec<BigInt> = vec![];

    while number < sqrt {
        if input.clone() % number.clone() == BigInt::ZERO && is_square_number(number.clone()) {
            square_factors.push(number.clone());
        }

        number += 1;
    }

    square_factors
}

fn main() {
    let start_time: Instant = Instant::now();

    // let number_with_most_square_factors: usize = 6350400;
    // let number_with_most_square_factors: usize = 76204800;

    let number: BigInt = BigInt::from_str(HIGHLY_COMPOSITE_NUMBERS[0]).unwrap();

    let square_factors_count: usize = get_square_factors(number.clone()).len();

    println!("{:?}", get_square_factors(number));

    println!("{}", square_factors_count);

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
