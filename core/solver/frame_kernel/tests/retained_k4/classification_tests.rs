//! K4 tests of S\* and the classification (the brief's J), bit for bit against
//! the generator's binary64 reimplementation (`classification.txt`).
//!
//! S\*-dependent assertions (every test here; to be updated mechanically if
//! D1 revision 5a.3 changes S\*): `SD-J1` the seeded and targeted row sets'
//! classes, bounds and body scales; `SD-J2` S8-W's far-node rows.
use super::super::recover::QuantityId;
use super::super::source::Dof;
use super::*;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;

const VECTORS: &str = include_str!("classification.txt");

fn hex(h: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(h, 16).unwrap())
}

fn outcome(token: &str) -> Binary64Outcome {
    match token {
        "U+" => Binary64Outcome::Underflow { negative: false },
        "U-" => Binary64Outcome::Underflow { negative: true },
        "O+" => Binary64Outcome::Overflow { negative: false },
        "O-" => Binary64Outcome::Overflow { negative: true },
        _ => {
            let (kind, bits) = token.split_once(':').unwrap();
            let value = hex(bits);
            if kind == "N" {
                Binary64Outcome::Normal(value)
            } else {
                Binary64Outcome::Subnormal {
                    value,
                    relative_precision: 0.0,
                }
            }
        }
    }
}

fn class(token: &str) -> RowClass {
    match token {
        "I" => RowClass::InputDerived,
        "R" => RowClass::RelativeVerified,
        "U" => RowClass::Unpublishable,
        _ => RowClass::AbsoluteVerified {
            bound_bits: u64::from_str_radix(&token[2..], 16).unwrap(),
        },
    }
}

#[test]
fn the_classification_matches_the_binary64_reimplementation_bit_for_bit() {
    // SD-J1.
    let mut sets = 0;
    let mut counts = [0usize; 4];
    let mut layout = Vec::new();
    let mut values = Vec::new();
    let mut expected = Vec::new();
    let mut scales: Vec<(u32, Kind, u64)> = Vec::new();
    let mut extents = Vec::new();
    for line in VECTORS.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f[0] {
            "set" => {
                layout.clear();
                values.clear();
                expected.clear();
                scales.clear();
                extents = f[3..].iter().map(|h| hex(h)).collect();
            }
            "row" => {
                layout.push(QuantityMeta {
                    id: QuantityId::Displacement(Dof::from_global(layout.len())),
                    kind: Kind::ALL[f[1].parse::<usize>().unwrap()],
                    body: f[2].parse().unwrap(),
                    input_derived: f[3] == "1",
                });
                values.push(outcome(f[4]));
                expected.push(class(f[5]));
            }
            "scales" => {
                let b: u32 = f[1].parse().unwrap();
                for (k, h) in f[2..].iter().enumerate() {
                    scales.push((b, Kind::ALL[k], u64::from_str_radix(h, 16).unwrap()));
                }
            }
            "end" => {
                let publication = classify_rows(&layout, &values, &extents);
                let got: Vec<RowClass> = publication.rows.iter().map(|r| r.class).collect();
                assert_eq!(got, expected, "set {sets}");
                assert_eq!(publication.body_scales, scales, "set {sets}");
                for c in &got {
                    counts[match c {
                        RowClass::RelativeVerified => 0,
                        RowClass::AbsoluteVerified { .. } => 1,
                        RowClass::InputDerived => 2,
                        RowClass::Unpublishable => 3,
                    }] += 1;
                }
                sets += 1;
            }
            _ => {}
        }
    }
    assert_eq!(sets, 405);
    assert!(counts.iter().all(|&c| c > 50), "{counts:?}");
}

#[test]
fn the_classifications_boundaries_hold_exactly() {
    // One ulp either side of t = fl(R·S*) (§7.3-20), with S* = 1.
    let t = threshold(1.0);
    assert_eq!(t, f64::from_bits(FLOOR_RATIO_BITS));
    let below = f64::from_bits(t.to_bits() - 1);
    assert_eq!(classify(t, 1.0), RowClass::RelativeVerified);
    assert!(matches!(
        classify(below, 1.0),
        RowClass::AbsoluteVerified { .. }
    ));
    assert_eq!(
        classify(f64::from_bits(t.to_bits() + 1), 1.0),
        RowClass::RelativeVerified
    );
    // S* < 2^-988: every row is absolute, whatever its value.
    let small = f64::from_bits(0x022F_FFFF_FFFF_FFFF);
    assert!(matches!(
        classify(small, small),
        RowClass::AbsoluteVerified { .. }
    ));
    // b = fl↑(2^-64·S*): exact when representable, at least 2^-1074 for any
    // S* > 0 (including S* < 2^-1011), and 0 only at S* = 0.
    assert_eq!(absolute_bound(1.0), 2f64.powi(-64));
    assert_eq!(absolute_bound(f64::from_bits(1)), f64::from_bits(1));
    let s = 2f64.powi(-1015);
    assert_eq!(absolute_bound(s), f64::from_bits(1));
    assert!(absolute_bound(3.0 * 2f64.powi(-1000)) >= 3.0 * 2f64.powi(-1064));
    assert_eq!(absolute_bound(0.0), 0.0);
}

#[test]
fn stress_scales_and_intensified_factors_match_bit_for_bit() {
    assert_eq!(K_SQRT2_BITS, 0x3FF6_A09E_667F_3BCD);
    assert_eq!(K_TWO_SQRT2_BITS, 0x4006_A09E_667F_3BCD);
    let (mut ki, mut stresses) = (0, 0);
    for line in VECTORS.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f[0] {
            "kint" => {
                assert_eq!(
                    intensified_k(hex(f[1])).to_bits(),
                    hex(f[2]).to_bits(),
                    "{line}"
                );
                ki += 1;
            }
            "stress" => {
                let v: Vec<f64> = f[1..].iter().map(|h| hex(h)).collect();
                assert_eq!(
                    stress_scale(v[0], v[1], v[2], v[3], v[4]).to_bits(),
                    v[5].to_bits()
                );
                stresses += 1;
            }
            _ => {}
        }
    }
    assert_eq!((ki, stresses), (300, 1500));
}

#[test]
fn input_derived_rows_and_s8_ws_far_node_rows_are_classified_as_the_design_says() {
    // Restrained and prescribed DOFs are input-derived (rule 2a).
    let m = models::model("PRESCRIBED");
    let (limit, mut meter) = (CaseLimit::new(u64::MAX), InvocationMeter::new(u64::MAX));
    let CaseOutcome::Selected(solve) = solve_case(m.source(), limit, &mut meter) else {
        panic!()
    };
    for r in &solve.publish().rows {
        if let QuantityId::Displacement(d) = r.id {
            let derived = solve.source().constraint(d.global()).is_some();
            assert_eq!(r.class == RowClass::InputDerived, derived, "{:?}", r.id);
        }
    }
    // SD-J2: S8-W's far-node displacements are below fl(2^-34·S*): withheld as
    // absolute_verified with their bound; the near ones are relative.
    for name in ["S8-W-1e-10", "S8-W-1e-14"] {
        let m = models::model(name);
        let (limit, mut meter) = (CaseLimit::new(u64::MAX), InvocationMeter::new(u64::MAX));
        let CaseOutcome::Selected(solve) = solve_case(m.source(), limit, &mut meter) else {
            panic!()
        };
        let publication = solve.publish();
        let (mut absolute, mut relative) = (0, 0);
        for r in &publication.rows {
            let s = publication
                .body_scales
                .iter()
                .find(|x| x.0 == r.body && x.1 == r.kind)
                .map(|x| f64::from_bits(x.2))
                .unwrap();
            match (r.class, r.value.value()) {
                (RowClass::AbsoluteVerified { bound_bits }, Some(v)) => {
                    assert!(v.abs() < threshold(s));
                    assert_eq!(bound_bits, absolute_bound(s).to_bits());
                    absolute += 1;
                }
                (RowClass::RelativeVerified, Some(v)) => {
                    assert!(v.abs() >= threshold(s));
                    relative += 1;
                }
                _ => {}
            }
        }
        println!("{name}: {absolute} absolute_verified, {relative} relative_verified");
        assert!(absolute > 0 && relative > 0, "{name}");
    }
}
