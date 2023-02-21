const R1: f32 = 99.5 * 1000.;
const R2: f32 = 21.5 * 1000.;

fn compute_frequency(period: &f32) -> f32 {
    1. / period
}

fn compute_capacitance(freq: &f32) -> f32 {
    1.44 / (freq * (R1 + 2. * R2))
}

fn main() {
    let p = 5.00 / 1000.;

    let f = compute_frequency(&p);
    let c = compute_capacitance(&f);

    println!("{}", c);
}
