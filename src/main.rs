fn exit_with_error(error: &String) {
    eprintln!("An error occurred: {}", error);
    std::process::exit(1);
}

fn get_period_from_cli() -> f32 {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        let error_msg = "Not enough arguments provided!".to_string();
        exit_with_error(&error_msg);
    }

    let p_usec = &args[1];
    p_usec.parse::<f32>().unwrap()
}

fn usec_to_sec(period: &f32) -> f32 {
    period / 1000.
}

fn compute_frequency(period: &f32) -> f32 {
    // abort if period is 0
    1. / period
}

fn compute_capacitance(freq: &f32) -> f32 {
    static R1: f32 = 99.5 * 1000.;
    static R2: f32 = 21.5 * 1000.;

    1.44 / (freq * (R1 + 2. * R2))
}

fn main() {

    let p_usec = get_period_from_cli();
    let p = usec_to_sec(&p_usec);

    let f = compute_frequency(&p);
    let c = compute_capacitance(&f);

    println!("{}", c);
}
