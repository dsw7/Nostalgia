use crate::consts::{Results, R1, R2};
use crate::helpers::{farad_to_uf, farad_to_nf};

fn compute_frequency_from_period(p: &f32) -> f32 {

    if p < &0.0001 {
        eprintln!("Period is too small! Minimum period is 0.0001 seconds");
        std::process::exit(1);
    }

    1. / p
}

fn compute_capacitance_from_frequency(f: &f32) -> f32 {

    if f < &0.0001 {
        eprintln!("Frequency is too small! Minimum frequency is 0.0001 Hz");
        std::process::exit(1);
    }

    1.44 / (f * (R1 + 2. * R2))
}

pub fn compute_main(p: &f32) -> Results {

    let f = compute_frequency_from_period(p);
    let c = compute_capacitance_from_frequency(&f);

    Results {
        period: *p,
        frequency: f,
        cap_f: c,
        cap_uf: farad_to_uf(&c),
        cap_nf: farad_to_nf(&c),
    }
}
