//! I109: `correct_norm` against the exact-rational oracle.
//!
//! `correct_norm_vectors` checks committed vectors: adversarial cases (exact
//! and near midpoints, sticky third components, binade, subnormal and overflow
//! boundaries) and random ones, whose expected bits were computed with exact
//! integer square roots (I109's oracle, `R/I109/platform_norm_01/`).
//!
//! `correct_norm_oracle_dump` is the oracle's Rust side for large runs. It is
//! ignored; it reads `CORRECT_NORM_ORACLE_IN` (little-endian binary64 triples
//! a, b, c) and writes `CORRECT_NORM_ORACLE_OUT` (the bits of norm3(a, b, c),
//! then of norm2(a, b), per triple), for the verifier to check exactly.

use open_pipe_stress_frame_kernel::correct_norm::{norm2, norm3};
use std::io::{BufReader, BufWriter, Read, Write};

const VECTORS: &str = include_str!("correct_norm_vectors.txt");

fn hex(word: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(word, 16).expect("hex binary64"))
}

#[test]
fn correct_norm_vectors() {
    let mut count = 0;
    for line in VECTORS
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
    {
        let w: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(w.len(), 5, "{line}");
        let (a, b, c) = (hex(w[0]), hex(w[1]), hex(w[2]));
        let (n3, n2) = (norm3(a, b, c), norm2(a, b));
        assert_eq!(format!("{:016x}", n3.to_bits()), w[3], "norm3 {line}");
        assert_eq!(format!("{:016x}", n2.to_bits()), w[4], "norm2 {line}");
        count += 1;
    }
    assert!(count >= 1000, "{count} vectors");
}

#[test]
#[ignore = "I109 oracle dump: set CORRECT_NORM_ORACLE_IN and CORRECT_NORM_ORACLE_OUT"]
fn correct_norm_oracle_dump() {
    let input = std::env::var("CORRECT_NORM_ORACLE_IN").expect("CORRECT_NORM_ORACLE_IN");
    let output = std::env::var("CORRECT_NORM_ORACLE_OUT").expect("CORRECT_NORM_ORACLE_OUT");
    let mut reader = BufReader::new(std::fs::File::open(input).expect("input"));
    let mut writer = BufWriter::new(std::fs::File::create(output).expect("output"));
    let mut record = [0u8; 24];
    let mut count = 0u64;
    loop {
        match reader.read_exact(&mut record) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => panic!("read: {e}"),
        }
        let x = |i: usize| f64::from_le_bytes(record[8 * i..8 * i + 8].try_into().unwrap());
        let (a, b, c) = (x(0), x(1), x(2));
        writer
            .write_all(&norm3(a, b, c).to_bits().to_le_bytes())
            .unwrap();
        writer
            .write_all(&norm2(a, b).to_bits().to_le_bytes())
            .unwrap();
        count += 1;
    }
    writer.flush().unwrap();
    eprintln!("correct_norm_oracle_dump: {count} triples");
}
