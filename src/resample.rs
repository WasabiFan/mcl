use std::vec::Vec;
use rand::random;

pub fn resample<T: Copy>(values: &[T] , weights: &[f64]) -> Vec<T> {
    // Implements Algorithm 1 from:
    // J. Carpenter, P. Clifford, and P. Fernhead, “An improved particle filter
    // for non-linear problems,” tech. rep., Department of Statistics,
    // University of Oxford, 1997

    assert_eq!(values.len(), weights.len());
    let mut cum_weights: Vec<f64> = vec![0.0]; // Q
    let mut cum_rand: Vec<f64> = vec![-random::<f64>().ln()]; // T

    for i in 0..values.len() {
        cum_weights.push(cum_weights.last().unwrap() + weights[i]);
        // the negative log of a standard uniform is an exponential distribution
        cum_rand.push(cum_rand.last().unwrap() + -random::<f64>().ln());
    }

    let mut i: usize = 0;
    let mut j: usize = 1;
    let mut result: Vec<T> = Vec::default();

    while i < values.len() {
        if cum_weights[j] * cum_rand[values.len()] > cum_rand[i] {
            i += 1;
            result.push(values[j - 1]);
        } else {
            j += 1;
        }
    }

    result
}