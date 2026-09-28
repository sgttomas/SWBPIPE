//! K4 tests of `retained/ledger.rs` and the `exact_sum.rs` accessor (the
//! brief's C).
use super::super::adaptive::StageGuard;
use super::super::assemble::{assemble, form_members, reduced_rhs, Structure};
use super::super::source::{
    Component, Constraint, Dof, NodalLoad, PrimitiveSource, SourceParts, StraightMember,
};
use super::super::wide::multi::{Binary64Outcome, SupportedWidth};
use super::super::wide::Wide;
use super::super::wide_sum::ExactWideSum;
use super::*;

#[allow(dead_code)]
#[path = "support.rs"]
mod support;
use support::*;
#[allow(dead_code)]
#[path = "models.rs"]
mod models;

const LEDGER: &str = include_str!("ledger.txt");

struct Row {
    values: Vec<f64>,
    net: (bool, Vec<u64>, i64),
    rounded: Vec<(u32, String)>,
    round: Option<u64>,
}

fn rows() -> Vec<Row> {
    LEDGER
        .lines()
        .map(|line| {
            let f: Vec<&str> = line.split_whitespace().collect();
            let n: usize = f[1].parse().unwrap();
            let values = f[2..2 + n].iter().map(|h| f64_bits(h)).collect();
            let mut k = 2 + n;
            assert_eq!(f[k], "net");
            let net_tok = f[k + 1];
            let negative = net_tok.starts_with('-');
            let (hex, e) = net_tok[1..].split_once('@').unwrap();
            let mut mag = hex_limbs(hex);
            while mag.len() > 1 && mag.last() == Some(&0) {
                mag.pop();
            }
            if mag == [0] {
                mag.clear();
            }
            k += 2;
            let mut rounded = Vec::new();
            while f[k] != "round" {
                rounded.push((f[k][1..].parse().unwrap(), f[k + 1].to_string()));
                k += 2;
            }
            let round = match f[k + 1] {
                "R" => None,
                h => Some(u64::from_str_radix(h, 16).unwrap()),
            };
            Row {
                values,
                net: (negative, mag, e.parse().unwrap()),
                rounded,
                round,
            }
        })
        .collect()
}

fn accumulate(values: &[f64]) -> ExactAccumulator {
    let mut acc = ExactAccumulator::new();
    for &v in values {
        acc.add(v).unwrap();
    }
    acc
}

/// The accessor's (negative, magnitude at 2^-2148) in canonical odd form.
fn canonical(negative: bool, magnitude: &[u64], exponent: i64) -> (bool, Vec<u64>, i64) {
    let Some(low) = magnitude.iter().position(|&l| l != 0) else {
        return (false, Vec::new(), 0);
    };
    let shift = low * 64 + magnitude[low].trailing_zeros() as usize;
    let mut out = shr(magnitude, shift);
    while out.last() == Some(&0) {
        out.pop();
    }
    (negative, out, exponent + shift as i64)
}

fn rounded_at<const L: usize>(p: u32, negative: bool, magnitude: &[u64], exponent: i64) -> String
where
    Wide<L>: SupportedWidth,
{
    tok(&ctx::<L>(p)
        .from_integer(negative, magnitude, exponent)
        .unwrap())
}

#[test]
fn the_accessor_nets_once_with_the_quantum_and_a_zero_is_plus_zero() {
    let rows = rows();
    assert!(rows.len() > 400);
    let mut cancelled = 0;
    for row in &rows {
        let acc = accumulate(&row.values);
        let (negative, magnitude, exponent) = acc.net_parts();
        assert_eq!(exponent, -2148, "the 2^-2148 quantum");
        assert_eq!(magnitude.len(), 68);
        let got = canonical(negative, &magnitude, exponent);
        assert_eq!(got, row.net, "{:?}", row.values);
        if magnitude.iter().all(|&l| l == 0) {
            assert!(!negative, "a zero is (+, 0)");
            cancelled += 1;
        }
        // Netting agrees with the accumulator's own sign.
        assert_eq!(
            acc.signum(),
            if magnitude.iter().all(|&l| l == 0) {
                0
            } else if negative {
                -1
            } else {
                1
            }
        );
    }
    assert!(cancelled >= 3, "{cancelled}");
}

#[test]
fn the_projection_matches_the_fraction_oracle_at_every_precision() {
    for row in rows() {
        let (negative, magnitude, exponent) = accumulate(&row.values).net_parts();
        for (p, expect) in &row.rounded {
            let got = match *p {
                53 | 128 | 192 | 256 => rounded_at::<4>(*p, negative, &magnitude, exponent),
                320 | 512 => rounded_at::<8>(*p, negative, &magnitude, exponent),
                _ => rounded_at::<16>(*p, negative, &magnitude, exponent),
            };
            assert_eq!(&got, expect, "p {p} {:?}", row.values);
        }
    }
}

#[test]
fn the_projection_at_p53_gives_round_on_normal_results_and_the_exact_projection_everywhere() {
    let mut normal = 0;
    let mut exact_checked = 0;
    for row in rows() {
        let acc = accumulate(&row.values);
        let (negative, magnitude, exponent) = acc.net_parts();
        let rounded = acc.round();
        // p = 53: wherever round() is a normal binary64 (or zero), the bits agree.
        let at53 = ctx::<4>(53)
            .from_integer(negative, &magnitude, exponent)
            .unwrap();
        if let Ok(r) = rounded {
            if r == 0.0 || r.is_normal() {
                let v = at53.to_binary64();
                match v {
                    Binary64Outcome::Normal(x) => {
                        assert_eq!(x.to_bits(), r.to_bits(), "{:?}", row.values);
                        normal += 1;
                    }
                    other => panic!("{other:?} {:?}", row.values),
                }
            }
        }
        // The exact form: where the net spans at most 1024 bits, its projection
        // at p = 1024 is exact and its conversion equals round(), subnormal
        // results included; overflow and underflow map to `NonRepresentable`
        // and +0.0.
        let span = match (
            magnitude.iter().position(|&l| l != 0),
            bit_length(&magnitude),
        ) {
            (Some(low), top) => top - (low * 64 + magnitude[low].trailing_zeros() as usize),
            (None, _) => 0,
        };
        if span > 1024 {
            continue;
        }
        exact_checked += 1;
        let exact = ctx::<16>(1024)
            .from_integer(negative, &magnitude, exponent)
            .unwrap();
        match (exact.to_binary64(), rounded, row.round) {
            (Binary64Outcome::Normal(x), Ok(r), Some(bits)) => {
                assert_eq!(x.to_bits(), r.to_bits());
                assert_eq!(r.to_bits(), bits);
            }
            (Binary64Outcome::Subnormal { value, .. }, Ok(r), Some(bits)) => {
                assert_eq!(value.to_bits(), r.to_bits());
                assert_eq!(r.to_bits(), bits);
            }
            (Binary64Outcome::Underflow { .. }, Ok(r), Some(0)) => assert_eq!(r.to_bits(), 0),
            (Binary64Outcome::Overflow { .. }, Err(_), None) => {}
            other => panic!("{other:?} {:?}", row.values),
        }
    }
    assert!(normal > 300, "{normal}");
    assert!(exact_checked > 200, "{exact_checked}");
}

#[test]
fn cancelling_contributions_are_exact_per_dof() {
    // RF-CANCEL's authored orders and V1's check L: the net is exact in every
    // order (the ledger's granularity is one term per contribution).
    for g in [1e5, 1e6, 1e7, 1e8] {
        for order in [[g, 0.3, -g], [g, -g, 0.3], [0.3, g, -g]] {
            let net = accumulate(&order).round().unwrap();
            assert_eq!(net, 0.3, "{order:?}");
        }
    }
    for order in [
        [1e80, 1e-8, -1e80],
        [1e80, -1e80, 1e-8],
        [1e-8, 1e80, -1e80],
    ] {
        assert_eq!(accumulate(&order).round().unwrap(), 1e-8, "{order:?}");
    }
}

fn dof(node: u32, c: usize) -> Dof {
    Dof {
        node,
        component: Component::from_index(c),
    }
}

#[test]
fn the_ledger_enters_a_prescribed_coupled_rhs_exactly_even_when_its_tail_decides_a_tie() {
    // A member along x, L = 1, E·A = 2^20 (so EA/L = 2^20 exactly at any p);
    // u_j prescribed at 2^-20, so −K_ij·u_j = 1 exactly; the ledger at u_i is
    // 2^-128 + 2^-400. The exact rhs 1 + 2^-128 + 2^-400 rounds up at p = 128 to
    // 1 + 2^-127; the ledger rounded to p first (2^-128: its tail is 272 bits
    // below) would make a tie that rounds to even, 1 (the mutant K4-M8).
    let member = StraightMember {
        id: 1,
        node_i: 0,
        node_j: 1,
        elastic_modulus: 1024.0,
        shear_modulus: 1.0,
        area: 1024.0,
        second_moment_y: 1.0,
        second_moment_z: 1.0,
        torsion_constant: 1.0,
        y_reference: [0.0, 0.0, 1.0],
    };
    let mut parts = SourceParts {
        nodes: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        members: vec![member],
        ..Default::default()
    };
    for c in 1..6 {
        parts.constraints.push(Constraint {
            dof: dof(0, c),
            value: 0.0,
        });
    }
    for c in 0..6 {
        parts.constraints.push(Constraint {
            dof: dof(1, c),
            value: if c == 0 { 2f64.powi(-20) } else { 0.0 },
        });
    }
    parts.loads.push(NodalLoad {
        dof: dof(0, 0),
        value: 2f64.powi(-128),
        source_id: "a".into(),
    });
    parts.loads.push(NodalLoad {
        dof: dof(0, 0),
        value: 2f64.powi(-400),
        source_id: "b".into(),
    });
    let source = PrimitiveSource::new(parts).unwrap();
    let ledger = RetainedLedger::from_source(&source).unwrap();
    let structure = Structure::new(&source).unwrap();
    let mut c = ctx::<4>(128);
    let mut sum = ExactWideSum::new();
    let guard = StageGuard::unlimited();
    let members = form_members(&mut c, &mut sum, &guard, &source).unwrap();
    let k = assemble(&mut c, &mut sum, &guard, &source, &structure, &members, &[]).unwrap();
    let kij = k[structure.pattern.find(0, 6).unwrap()];
    assert_eq!(tok(&kij), tok(&lift::<4>(-(2f64.powi(20)))));
    let u: Vec<Wide<4>> = (0..source.dof_count())
        .map(|g| lift::<4>(source.constraint(g).unwrap_or(0.0)))
        .collect();
    let rhs = reduced_rhs(&mut c, &mut sum, &source, &structure, &k, &ledger, &[0], &u).unwrap();
    let expect = w_value::<4>(false, 0, &[1, 1 << 63]); // 1 + 2^-127
    assert_eq!(tok(&rhs[0]), tok(&expect));
    // The ledger's projection alone loses the tail (so a pre-rounded ledger
    // would give the tie's even neighbour, 1).
    let projected = ledger.project(0, &mut c).unwrap();
    assert_eq!(tok(&projected), tok(&lift::<4>(2f64.powi(-128))));
}

#[test]
fn the_ledger_encoding_is_canonical_and_order_independent() {
    let make = |loads: Vec<(u32, usize, f64, &str)>| {
        let parts = SourceParts {
            nodes: vec![[0.0; 3], [1.0, 0.0, 0.0]],
            loads: loads
                .into_iter()
                .map(|(n, c, v, s)| NodalLoad {
                    dof: dof(n, c),
                    value: v,
                    source_id: s.into(),
                })
                .collect(),
            ..Default::default()
        };
        RetainedLedger::from_source(&PrimitiveSource::new(parts).unwrap()).unwrap()
    };
    let a = make(vec![
        (1, 1, 1e80, "g"),
        (1, 1, 0.3, "n"),
        (1, 1, -1e80, "h"),
        (0, 3, 2.0, "m"),
    ]);
    let b = make(vec![
        (0, 3, 2.0, "m"),
        (1, 1, -1e80, "h"),
        (1, 1, 1e80, "g"),
        (1, 1, 0.3, "n"),
    ]);
    assert_eq!(a.encoding(), b.encoding());
    assert_eq!(a.net(7).unwrap().magnitude.len(), 1);
    let cancelled = make(vec![(1, 1, 5.0, "x"), (1, 1, -5.0, "y")]);
    assert!(cancelled.net(7).unwrap().is_zero());
}

#[test]
fn canonical_encodings_equal_the_generators_independent_bytes() {
    // Q10: GEN builds K4SRC, K4STF and K4LED (and combination ledgers) from
    // the models' own lists, independently of source.rs and ledger.rs, and
    // records their sha256 with hashlib.
    let hex = |bytes: &[u8]| bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    let mut kinds = std::collections::BTreeMap::new();
    for line in include_str!("encodings.txt").lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        let (kind, name, sha, expected) = (f[1], f[2], f[3], f[4]);
        let bytes = match kind {
            "src" => models::model(name).source().encoding(),
            "stf" => models::model(name).source().stiffness_encoding(),
            "led" => RetainedLedger::from_source(&models::model(name).source())
                .unwrap()
                .encoding(),
            "cled" => {
                let combo = models::combos()
                    .into_iter()
                    .find(|c| c.name == name)
                    .unwrap();
                let sources: Vec<PrimitiveSource> = combo
                    .operands
                    .iter()
                    .map(|(_, n)| models::model(n).source())
                    .collect();
                let operands: Vec<(f64, &PrimitiveSource)> = combo
                    .operands
                    .iter()
                    .zip(&sources)
                    .map(|((factor, _), s)| (*factor, s))
                    .collect();
                RetainedLedger::combined(&operands).unwrap().encoding()
            }
            other => panic!("{other}"),
        };
        assert_eq!(hex(&bytes), expected, "{kind} {name}");
        assert_eq!(sha256_hex(&bytes), sha, "{kind} {name}");
        *kinds.entry(kind.to_string()).or_insert(0) += 1;
    }
    assert_eq!(kinds.values().sum::<usize>(), 27, "{kinds:?}");
}
