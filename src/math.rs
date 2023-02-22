pub fn compute_frequency_from_period(period: &f32) -> f32 {
    // abort if period is 0
    1. / period
}

pub fn compute_capacitance_from_frequency(freq: &f32) -> f32 {

    static R1: f32 = 99.5 * 1000.;
    static R2: f32 = 21.5 * 1000.;

    1.44 / (freq * (R1 + 2. * R2))
}

pub fn compute_capacitance_from_period(period: &f32) -> f32 {

    let freq = compute_frequency_from_period(&period);
    compute_capacitance_from_frequency(&freq)
}
