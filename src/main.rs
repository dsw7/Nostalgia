mod helpers;
mod math;

fn get_period_from_cli() -> f32 {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Not enough arguments provided!");
        std::process::exit(1);
    }

    let p_usec = &args[1];
    p_usec.parse::<f32>().unwrap()
}

fn main() {

    let p_usec = get_period_from_cli();
    let p = helpers::usec_to_sec(&p_usec);

    let c = math::primitives::compute_capacitance_from_period(&p);
    println!("{}", c);
}
