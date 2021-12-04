use rand::Rng;
use rayon::prelude::*;

const NUMBERS_PER_ITERATION: usize = 100_000;

fn main() {
    let mut start = std::time::Instant::now();
    let mut count = 0;
    loop {
        let mut numbers = Vec::with_capacity(NUMBERS_PER_ITERATION);
        let mut rng = rand::thread_rng();
        for _ in 0..NUMBERS_PER_ITERATION {
            numbers.push(rng.gen::<u128>());
        }

        let total_steps = numbers
            .par_iter()
            .map(|number| {
                let mut steps = 0usize;
                let mut number = *number;
                while number > 1 {
                    if number & 1 == 0 {
                        number >>= 1;
                    } else {
                        number = 3 * number + 1;
                    }

                    steps += 1;
                }

                steps
            })
            .sum::<usize>();

        count += NUMBERS_PER_ITERATION;

        if start.elapsed().as_millis() >= 1_000 {
            println!(
                "~{} numbers/second, {} steps on average",
                count,
                total_steps as f32 / NUMBERS_PER_ITERATION as f32
            );
            start = std::time::Instant::now();
            count = 0;
        }
    }
}
