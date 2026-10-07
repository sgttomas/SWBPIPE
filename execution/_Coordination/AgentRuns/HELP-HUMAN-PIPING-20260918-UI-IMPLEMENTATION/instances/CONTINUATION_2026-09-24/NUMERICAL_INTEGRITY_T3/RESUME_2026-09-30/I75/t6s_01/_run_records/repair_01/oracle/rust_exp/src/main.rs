//! I75 REPAIR_01 oracle (scratch only): for each input file of 16-hex-digit binary64 words,
//! writes `<file>.rust.txt` with Rust's `{:e}` of each word, one per line, as
//! `result_export::derivative::class_disclosure` prints the bound.
use std::fs;
fn main() {
    for file in std::env::args().skip(1) {
        let text = fs::read_to_string(&file).unwrap();
        let mut out = String::new();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let bits = u64::from_str_radix(line.trim(), 16).unwrap();
            out.push_str(&format!("{:e}\n", f64::from_bits(bits)));
        }
        fs::write(format!("{file}.rust.txt"), out).unwrap();
        eprintln!("{file}: {} words", text.lines().count());
    }
}
