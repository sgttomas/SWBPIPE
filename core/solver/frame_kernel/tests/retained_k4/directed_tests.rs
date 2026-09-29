//! K4 tests of `retained/directed.rs` (D1 revision 5a.3, R7 §4.1.6.3 item 7;
//! SD-G5's "the directed roundings"): every operation against the generator's
//! Fraction oracle (`directed.txt`: nearest, then one step when on the wrong
//! side), at P = 3 to 1024, both directions, and `binary64_up`.
use super::super::wide::multi::{SupportedWidth, WideContext};
use super::super::wide::Wide;
use super::super::wide_sum::ExactWideSum;
use super::*;

#[allow(dead_code)]
#[path = "support.rs"]
mod support;
use support::{parse, tok};

const DIRECTED: &str = include_str!("directed.txt");

fn run<const L: usize>(op: &str, p: u32, toward: Toward, a: &str, b: &str) -> String
where
    Wide<L>: SupportedWidth,
{
    let mut ctx = WideContext::<L>::new(p).unwrap();
    let mut sum = ExactWideSum::new();
    let (a, b) = (parse::<L>(a), parse::<L>(b));
    let out = match op {
        "add" => add_toward(&mut ctx, &mut sum, &a, &b, toward),
        "sub" => sub_toward(&mut ctx, &mut sum, &a, &b, toward),
        "mul" => mul_toward(&mut ctx, &mut sum, &a, &b, toward),
        "div" => div_toward(&mut ctx, &mut sum, &a, &b, toward),
        other => panic!("{other}"),
    }
    .unwrap();
    tok(&out)
}

fn up64<const L: usize>(x: &str) -> f64
where
    Wide<L>: SupportedWidth,
{
    binary64_up(&parse::<L>(x)).unwrap()
}

#[test]
fn directed_operations_equal_the_fraction_oracle_in_both_directions() {
    let (mut ops, mut conversions) = (0, 0);
    for line in DIRECTED.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f[0] {
            "dir" => {
                let p: u32 = f[2].parse().unwrap();
                let toward = if f[4] == "up" {
                    Toward::Up
                } else {
                    Toward::Down
                };
                let got = match f[3] {
                    "4" => run::<4>(f[1], p, toward, f[5], f[6]),
                    "8" => run::<8>(f[1], p, toward, f[5], f[6]),
                    _ => run::<16>(f[1], p, toward, f[5], f[6]),
                };
                assert_eq!(got, f[7], "{line}");
                ops += 1;
            }
            "up64" => {
                let got = match f[2] {
                    "4" => up64::<4>(f[3]),
                    "8" => up64::<8>(f[3]),
                    _ => up64::<16>(f[3]),
                };
                assert_eq!(format!("{:016x}", got.to_bits()), f[4], "{line}");
                conversions += 1;
            }
            _ => panic!("{line}"),
        }
    }
    assert!(ops > 1500 && conversions > 60, "{ops} {conversions}");
}

#[test]
fn a_directed_result_is_never_on_the_wrong_side_and_exact_values_do_not_move() {
    // 1/3 at 10 bits: up is above, down below, and they are adjacent.
    let mut ctx = WideContext::<4>::new(10).unwrap();
    let mut sum = ExactWideSum::new();
    let one = Wide::<4>::ONE;
    let three = Wide::<4>::from_f64(3.0).unwrap();
    let up = div_toward(&mut ctx, &mut sum, &one, &three, Toward::Up).unwrap();
    let down = div_toward(&mut ctx, &mut sum, &one, &three, Toward::Down).unwrap();
    for (v, sign) in [(&up, 1i8), (&down, -1)] {
        sum.clear();
        sum.add_product(&mut ctx, v, &three, false).unwrap();
        sum.add_wide(&one, true).unwrap();
        assert_eq!(sum.signum(), sign);
    }
    // An exact product and an exact sum stay put in both directions.
    let two = Wide::<4>::from_f64(2.0).unwrap();
    for toward in [Toward::Up, Toward::Down] {
        assert_eq!(
            tok(&mul_toward(&mut ctx, &mut sum, &two, &three, toward).unwrap()),
            "+cp2"
        );
        assert_eq!(
            tok(&add_toward(&mut ctx, &mut sum, &two, &three, toward).unwrap()),
            "+ap2"
        );
    }
    // Below a power of two the downward step is half an ulp above it.
    let tiny = Wide::<4>::from_f64(2f64.powi(-20)).unwrap();
    let below = sub_toward(&mut ctx, &mut sum, &one, &tiny, Toward::Down).unwrap();
    assert_eq!(tok(&below), "+ffcp-1");
}
