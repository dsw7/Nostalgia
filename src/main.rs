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

    let period_secs = match args.unit.as_str() {
        "s" => args.period,
        "ms" => helpers::msec_to_sec(&args.period),
        "us" => helpers::usec_to_sec(&args.period),
        _ => {
          eprintln!("Invalid unit. Valid units are s, ms and us");
          std::process::exit(1);
        },
    };

    compute::compute_main(&period_secs, &args.export);
}
