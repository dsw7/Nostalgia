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

struct Results {
    period: f32,
    frequency: f32,
    cap_f: f32,
    cap_uf: f32,
    cap_nf: f32,
}

fn compute_capacitance_from_period(p: &f32) -> Results {

    let f = compute_frequency_from_period(p);
    let c = compute_capacitance_from_frequency(&f);

    Results {
        period: *p,
        frequency: f,
        cap_f: c,
        cap_uf: crate::helpers::farad_to_uf(&c),
        cap_nf: crate::helpers::farad_to_nf(&c),
    }
}

pub fn compute_main(period: &f32, export: &bool) {

    let results = compute_capacitance_from_period(period);

    if !export {
        println!("Parsed frequency (Hz): {}", results.frequency);
        println!("Parsed period (s):     {}", results.period);
        println!("Capacitance (F):       {}", results.cap_f);
        println!("Capacitance (uF):      {}", results.cap_uf);
        println!("Capacitance (nF):      {}", results.cap_nf);
        return;
    }
}
