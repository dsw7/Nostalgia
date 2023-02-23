mod compute;
mod helpers;

use clap::Parser;

#[derive(Parser)]
struct Args {

    /// Specify period obtained from oscilloscope
    #[arg(short, long)]
    period: f32,

    /// Specify period units (s, ms, us)
    #[arg(short, long, default_value_t = 's'.to_string())]
    unit: String,

    /// Specify whether to export results to text file
    #[arg(short, long, default_value_t = false)]
    export: bool,
}

fn main() {

    let args = Args::parse();

    if args.unit == "s" {
        compute::compute_main(&args.period, &args.export);
    } else if args.unit == "ms" {
        let p = helpers::msec_to_sec(&args.period);
        compute::compute_main(&p, &args.export);
    } else if args.unit == "us" {
        let p = helpers::usec_to_sec(&args.period);
        compute::compute_main(&p, &args.export);
    } else {
        eprintln!("Invalid unit. Valid units are s, ms and us");
        std::process::exit(1);
    }
}
