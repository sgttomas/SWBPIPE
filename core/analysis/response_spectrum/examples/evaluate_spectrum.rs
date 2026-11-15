//! Evaluates a user-entered spectrum table at the user's query frequencies.
//!
//! Usage: `cargo run --example evaluate_spectrum -- request.json` (or `-` to read stdin).
//! The request is one spectrum record and its queries (see the crate README); every value is
//! the user's and nothing is defaulted. It prints the output document as JSON: each query's
//! ordinate or named refusal, or the table's refusals.
use std::io::Read;

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(path), None) = (args.next(), args.next()) else {
        eprintln!("usage: evaluate_spectrum <request.json | ->");
        std::process::exit(2);
    };
    let text = if path == "-" {
        let mut text = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut text) {
            eprintln!("cannot read stdin: {e}");
            std::process::exit(2);
        }
        text
    } else {
        match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("cannot read {path}: {e}");
                std::process::exit(2);
            }
        }
    };
    match open_pipe_stress_response_spectrum::evaluate_request_json(&text) {
        Ok(document) => println!("{}", serde_json::to_string_pretty(&document).expect("a JSON value serializes")),
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    }
}
