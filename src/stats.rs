pub fn normal_pdf(mean: f64, stddev: f64, x: f64) -> f64 {
    // (1 / sqrt(2σ^2 * π)) * e^(-(x - μ)^2 / (2σ^2))
    (2. * stddev.powf(2.) * std::f64::consts::PI).sqrt().recip() * std::f64::consts::E.powf(-(x - mean).powf(2.) /( 2. * mean.powf(2.)))
}