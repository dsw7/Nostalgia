mod compute;
mod consts;
mod helpers;
mod routers;

use clap::Parser;

#[derive(Parser)]
struct Args {

    /// Specify period obtained from oscilloscope
    #[arg(short, long)]
    period: f32,

    /// Specify period units (s, ms, us)
    #[arg(short, long, default_value_t = 's'.to_string())]
    unit: String,

    /// Specify whether to export results to JSON file. File will be placed on host's temporary
    /// directory
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

    let results = compute::compute_main(&period_secs);

    if args.export {
        routers::export_data_to_file(&results);
    }
    else {
        routers::export_data_to_stdout(&results);
    }
}
