//! RV101 ADDENDUM_01 oracle: for every word list named on the command line (one 16-hex-digit binary64
//! word per line), writes `<list>.rust.txt` with Rust's `{:e}` of each word, the formatting that
//! `derivative::class_disclosure` applies to the receipt's bound b. No dependencies.
use std::fs;
fn main() {
    for path in std::env::args().skip(1) {
        let text = fs::read_to_string(&path).expect("read");
        let mut out = String::new();
        let mut n = 0usize;
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let bits = u64::from_str_radix(line.trim(), 16).expect("hex word");
            out.push_str(&format!("{:e}\n", f64::from_bits(bits)));
            n += 1;
        }
        fs::write(format!("{path}.rust.txt"), out).expect("write");
        eprintln!("{path}: {n} words");
    }
}
