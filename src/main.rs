use rand::Rng;
use rayon::prelude::*;

const NUMBERS_PER_ITERATION: usize = 1000;

fn main() {
    let mut start = std::time::Instant::now();
    let mut count = 0;
    loop {
        let mut numbers = Vec::with_capacity(NUMBERS_PER_ITERATION);
        let mut rng = rand::thread_rng();
        for _ in 0..NUMBERS_PER_ITERATION {
            numbers.push(rng.gen::<u128>());
        }

        let result = numbers.par_iter().map(|number| {
            let mut number = *number;
            while number > 1 {
                if number & 1 == 0 {
                    number >>= 1;
                } else {
                    number = 3 * number + 1;
                }
            }

            number
        }).sum::<u128>();

        if result != NUMBERS_PER_ITERATION as u128 {
            unreachable!()
        }

        count += NUMBERS_PER_ITERATION;

        if start.elapsed().as_millis() >= 1_000 {
            println!("~{} numbers/second", count);
            start = std::time::Instant::now();
            count = 0;
        }
    }
}
