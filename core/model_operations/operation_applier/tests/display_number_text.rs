//! The engine's number display, exported to the desktop (`display_number_text`, native and
//! wasm) so delete before values take the engine's own text instead of a mirror of it. Pinned
//! by bit pattern on the cases where a mirror diverges or might:
//! - an exact tie between two shortest round-trip strings (1e15 + 0.25: Rust prints `.3`,
//!   JavaScript `.2`);
//! - the integral range edge (1e15, just below it, 20);
//! - values JavaScript prints in exponent form (1e21, 1e-7, 10^18 + 128);
//! - negative zero (`0`), and ordinary fractions.
use open_pipe_stress_operation_applier::display_number_text;

const PINS: &[(u64, &str)] = &[
    (0x430c6bf526340002, "1000000000000000.3"),
    (0x430c6bf526340000, "1000000000000000"),
    (0x430c6bf52633fff8, "999999999999999"),
    (0x4034000000000000, "20"),
    (0x444b1ae4d6e2ef50, "1000000000000000000000"),
    (0x3e7ad7f29abcaf48, "0.0000001"),
    (0x43abc16d674ec801, "1000000000000000100"),
    (0x8000000000000000, "0"),
    (0x3fb999999999999a, "0.1"),
    (0x4004000000000000, "2.5"),
    (0xbff8000000000000, "-1.5"),
    (0x419d6f3454800000, "123456789.125"),
];

#[test]
fn the_exported_display_is_pinned_on_the_divergent_cases() {
    for &(bits, expected) in PINS {
        let value = f64::from_bits(bits);
        assert_eq!(display_number_text(value), expected, "{bits:#018x} ({value:?})");
    }
    // The tie this export exists for: both one-digit candidates round-trip.
    let tie = f64::from_bits(0x430c6bf526340002);
    assert_eq!(tie, 1_000_000_000_000_000.25);
    assert_eq!("1000000000000000.2".parse::<f64>().unwrap(), tie);
    assert_eq!("1000000000000000.3".parse::<f64>().unwrap(), tie);
}
