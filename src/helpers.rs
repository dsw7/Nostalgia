pub fn exit_with_error(error: &String) {
    eprintln!("An error occurred: {}", error);
    std::process::exit(1);
}

pub fn usec_to_sec(period: &f32) -> f32 {
    period / 1000.
}
