mod helpers;
mod math;

fn get_period_from_cli() -> f32 {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        let error_msg = "Not enough arguments provided!".to_string();
        helpers::exit_with_error(&error_msg);
    }

    let p_usec = &args[1];
    p_usec.parse::<f32>().unwrap()
}

fn main() {

    let p_usec = get_period_from_cli();
    let p = helpers::usec_to_sec(&p_usec);

    let c = math::compute_capacitance_from_period(&p);
    println!("{}", c);
}
