//! Combines supplied signed contributions (DEL-04-10 unit S4): reads a combination input
//! document (`openpipestress.spectrum_combination.input/v2`) on standard input and writes the
//! output document on standard output. A document that does not decode is reported on standard
//! error with exit status 2; a refused combination is an output document with its refusals.
use open_pipe_stress_response_spectrum::combination_json::combine_json;
use std::io::Read;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut text = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut text) {
        eprintln!("cannot read standard input: {e}");
        return ExitCode::from(2);
    }
    match combine_json(&text) {
        Ok(output) => {
            println!("{}", serde_json::to_string_pretty(&output).expect("a JSON value serializes"));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(2)
        }
    }
}
