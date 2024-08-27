use std::time::Instant;

use rayon::prelude::*;

#[inline(always)]
fn is_square_number(number: usize) -> bool {
    let sqrt: usize = (number as f64).sqrt() as usize;
    sqrt * sqrt == number
}

#[inline(always)]
fn get_square_factors(input: usize) -> Vec<usize> {
    (2..(input as f64).sqrt() as usize)
        .filter(|number: &usize| input % number == 0)
        .filter(|number: &usize| is_square_number(*number))
        .collect()
}

fn get_number_with_most_square_factors(limit: usize) -> usize {
    (2..limit)
        .par_bridge()
        .map(|number: usize| (number, get_square_factors(number).len()))
        .max_by_key(|&(_, count)| count)
        .map(|(number, _)| number)
        .unwrap()
}

fn main() {
    let start_time: Instant = Instant::now();

    // let number_with_most_square_factors: usize = 6350400;
    // let number_with_most_square_factors: usize = 76204800;

    let limit: usize = 100000000;

    let number_with_most_square_factors: usize = get_number_with_most_square_factors(limit);
    println!(
        "Below {}, the number with most square factors is {}",
        limit, number_with_most_square_factors
    );

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
