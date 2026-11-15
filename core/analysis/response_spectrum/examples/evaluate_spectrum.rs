//! Evaluates a user-entered spectrum table at the user's query frequencies.
//!
//! Usage: `cargo run --example evaluate_spectrum < request.json`.
//! The request on standard input is one spectrum record and its queries (see the crate README);
//! every value is the user's and nothing is defaulted. It prints the output document as JSON:
//! each query's ordinate or named refusal, or the table's refusals. A request that does not
//! decode is reported on standard error with exit status 1.
use std::io::Read;

fn main() {
    if std::env::args().len() > 1 {
        eprintln!("usage: evaluate_spectrum < request.json");
        std::process::exit(2);
    }
    let mut text = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut text) {
        eprintln!("cannot read standard input: {e}");
        std::process::exit(2);
    }
    match open_pipe_stress_response_spectrum::evaluate_request_json(&text) {
        Ok(document) => println!("{}", serde_json::to_string_pretty(&document).expect("a JSON value serializes")),
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    }
}
