mod helpers;
mod math;

use clap::Parser;

#[derive(Parser)]
struct Args {

    /// Specify period obtained from oscilloscope
    #[arg(short, long, default_value_t = 1.00)]
    period: f32,

    /// Specify period units (s, ms, us)
    #[arg(short, long, default_value_t = 's'.to_string())]
    unit: String
}

fn main() {

    let args = Args::parse();

    if args.unit == "s" {
        math::primitives::compute_capacitance_from_period(&args.period);
    } else if args.unit == "ms" {
        let p = helpers::conversions::msec_to_sec(&args.period);
        math::primitives::compute_capacitance_from_period(&p);
    } else if args.unit == "us" {
        let p = helpers::conversions::usec_to_sec(&args.period);
        math::primitives::compute_capacitance_from_period(&p);
    } else {
        eprintln!("Invalid unit. Valid units are s, ms and us");
        std::process::exit(1);
    }
}
