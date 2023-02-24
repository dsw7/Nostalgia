use std::fs;
use std::env;
use std::path::Path;
use crate::consts::Results;

pub fn export_data_to_file(results: &Results) {

    let mut outgoing = String::new();

    outgoing.push_str("{");
    outgoing.push_str("\"description\":\"See https://github.com/dsw7/Nostalgia for more information\",");
    outgoing.push_str(&format!("\"frequency_in_hz\": {},", results.frequency));
    outgoing.push_str(&format!("\"period_in_secs\": {},", results.period));
    outgoing.push_str(&format!("\"capacitance_in_f\": {},", results.cap_f));
    outgoing.push_str(&format!("\"capacitance_in_uf\": {},", results.cap_uf));
    outgoing.push_str(&format!("\"capacitance_in_nf\": {}", results.cap_nf));
    outgoing.push_str("}");

    let tmpdir = env::temp_dir();
    let export_path = Path::new(&tmpdir).join("nos.json");

    match fs::write(export_path.as_os_str(), outgoing) {
        Ok(()) => println!("Exported results to {}", export_path.display()),
        Err(error) => panic!("Could not export to {}. The error was {}", export_path.display(), error),
    };
}

pub fn export_data_to_stdout(results: &Results) {

    println!("Parsed frequency (Hz): {}", results.frequency);
    println!("Parsed period (s):     {}", results.period);
    println!("Capacitance (F):       {}", results.cap_f);
    println!("Capacitance (uF):      {}", results.cap_uf);
    println!("Capacitance (nF):      {}", results.cap_nf);

}
