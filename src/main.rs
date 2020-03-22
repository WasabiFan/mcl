mod resample;
mod geometry;
mod localization;

use resample::resample;

fn main() {
    let values: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let weights: [f64; 5] = [0.1, 0.2, 0.4, 0.2, 0.1];
    let mut hist = [0usize; 5];

    for _ in 0..10000 {
        let result = resample(&values, &weights);
        for v in result {
            hist[(v - 1.) as usize] += 1;
        }
    }

    println!("{:?}", hist);
}