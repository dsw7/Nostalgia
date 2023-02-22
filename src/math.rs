pub mod primitives {

    const R1: f32 = 99.5 * 1000.;
    const R2: f32 = 21.5 * 1000.;

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

    pub fn compute_capacitance_from_period(p: &f32) -> f32 {

        let f = compute_frequency_from_period(p);
        compute_capacitance_from_frequency(&f)
    }
}
