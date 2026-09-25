#![cfg(feature = "checked-cli")]
use open_pipe_stress_canonical_json::binary64_cli::{
    canonicalize_binary64_v1_request, MAX_REQUEST_BYTES,
};
use std::io::{self, Read};
fn run() -> Result<(), String> {
    let mut bytes = Vec::new();
    io::stdin()
        .lock()
        .take(MAX_REQUEST_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("BINARY64-CLI-STDIN: {e}"))?;
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err("BINARY64-CLI-BYTE-LIMIT".into());
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| "BINARY64-CLI-UTF8".to_owned())?;
    let response = canonicalize_binary64_v1_request(text)?;
    print!("{response}");
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
