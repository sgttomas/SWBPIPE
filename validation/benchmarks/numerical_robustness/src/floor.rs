//! The zero-scale floor check (T3 D1 §4.10, V1-S8, F2; plan §7).
//!
//! S\* per kind is formed from the case's reference values by D1 §4.1.6.1
//! items 4–6, in binary64 and in the stated order: S(kind) is the largest
//! |fl(exp)| of the kind (R1's class without `@region`), L_b is the body's
//! extent from the adapted node coordinates, and the four kinds are coupled.
//! Twist and extension are variant F: S\*_tw(m) = fl(mo/k_t) and
//! S\*_ext(m) = fl(fo/k_a), with k_t and k_a as §4.10 forms them (C1 below).
//! A comparison is covered iff max(|exp|, scale) ≥ R·S\*, R = 2^-34, decided
//! exactly (`exact::covered`).
use crate::cases::{Case, Model, Row};
use std::collections::BTreeMap;

/// R = 2^-34 (D1 §4.1.6, bits `0x3DD0000000000000`).
pub const R_BITS: u64 = 0x3DD0_0000_0000_0000;

pub fn r() -> f64 {
    f64::from_bits(R_BITS)
}

fn is_normal(x: f64) -> bool {
    x.is_normal()
}

/// FK's element length: `norm(subtract(xj, xi))` = `dot(v, v).sqrt()`, the
/// products and sums left to right (`FK/lib.rs` `norm`, `dot`).
pub fn member_length(xi: [f64; 3], xj: [f64; 3]) -> f64 {
    let v = [xj[0] - xi[0], xj[1] - xi[1], xj[2] - xi[2]];
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// §4.10's k = fl(fl(a·b)/L), where both intermediates are normal binary64
/// numbers; None where the design's formula is not defined.
pub fn design_coefficient(a: f64, b: f64, length: f64) -> Option<f64> {
    let p = a * b;
    let q = p / length;
    (is_normal(p) && is_normal(q)).then_some(q)
}

/// 2^k for |k| ≤ 1022 (a normal power of two), from its bits.
fn pow2(k: i32) -> f64 {
    assert!((-1022..=1023).contains(&k));
    f64::from_bits(((1023 + k) as u64) << 52)
}

/// x·2^k exactly for a normal result, in steps that keep every factor normal.
fn scale2(mut x: f64, mut k: i32) -> f64 {
    while k != 0 {
        let s = k.clamp(-1000, 1000);
        x *= pow2(s);
        k -= s;
    }
    x
}

/// The binary exponent e with |x| in [2^e, 2^(e+1)) for a finite nonzero x.
fn exponent(x: f64) -> i32 {
    let bits = x.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i32;
    if biased == 0 {
        let fraction = bits & ((1u64 << 52) - 1);
        -1075 + (64 - fraction.leading_zeros() as i32)
    } else {
        biased - 1023
    }
}

/// k = fl(fl(a·b)/L), ROOT's C1 ruling: where the design's formula is defined
/// it is used as written; elsewhere (RF-RANGE LEF-small, where fl(G·J) rounds
/// to 0, and LEF-large, where it overflows) the same two operations run on b
/// pre-scaled by an exact power of two s, chosen so that a·b·2^s lies in
/// [1, 4), and the quotient is unscaled exactly. For normal results a
/// rounding commutes with an exact power-of-two scaling, so the two agree
/// bit for bit wherever the design's intermediates are normal (tested on
/// every member of every case).
pub fn coefficient(a: f64, b: f64, length: f64) -> f64 {
    if let Some(k) = design_coefficient(a, b, length) {
        return k;
    }
    prescaled_coefficient(a, b, length)
}

/// The pre-scaled form alone (exposed for C1's equality test).
pub fn prescaled_coefficient(a: f64, b: f64, length: f64) -> f64 {
    let s = -(exponent(a) + exponent(b));
    let q = (a * scale2(b, s)) / length;
    scale2(q, -s)
}

/// S\* of one case: the four coupled kinds and, per member (by R1 name), the
/// variant-F twist and extension scales.
#[derive(Clone, Debug, PartialEq)]
pub struct Scales {
    /// translation, rotation, force, moment.
    pub kinds: [f64; 4],
    pub members: BTreeMap<String, (f64, f64)>,
}

fn kind_index(kind: &str) -> Option<usize> {
    ["translation", "rotation", "force", "moment"]
        .iter()
        .position(|k| *k == kind)
}

/// D1 §4.1.6.1 item 5: L_b from the body's node coordinates.
pub fn body_extent(nodes: &[[f64; 3]]) -> f64 {
    let mut d = [0.0f64; 3];
    for (a, slot) in d.iter_mut().enumerate() {
        let hi = nodes.iter().map(|p| p[a]).fold(f64::NEG_INFINITY, f64::max);
        let lo = nodes.iter().map(|p| p[a]).fold(f64::INFINITY, f64::min);
        *slot = hi - lo;
    }
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

/// D1 §4.1.6.1 item 6, in its order.
pub fn couple(s: [f64; 4], extent: f64) -> [f64; 4] {
    let [s_tr, s_ro, s_fo, s_mo] = s;
    if extent == 0.0 {
        return s;
    }
    [
        s_tr.max(extent * s_ro),
        s_ro.max(s_tr / extent),
        s_fo.max(s_mo / extent),
        s_mo.max(extent * s_fo),
    ]
}

/// S(kind) = the largest |fl(exp)| of each kind over `values`.
pub fn maxima<'a>(values: impl Iterator<Item = (&'a str, &'a str)>) -> [f64; 4] {
    let mut s = [0.0f64; 4];
    for (kind, expected) in values {
        if let Some(k) = kind_index(kind) {
            let v: f64 = expected.parse().expect("decimal");
            s[k] = s[k].max(v.abs());
        }
    }
    s
}

/// S\* from given maxima and the model (one body: asserted by the caller).
pub fn scales_from(maxima: [f64; 4], model: &Model) -> Scales {
    let kinds = couple(maxima, body_extent(&model.nodes));
    let (fo, mo) = (kinds[2], kinds[3]);
    let members = model
        .members
        .iter()
        .map(|m| {
            let l = member_length(
                model.nodes[m.node_i as usize],
                model.nodes[m.node_j as usize],
            );
            let kt = coefficient(m.shear_modulus, m.torsion_constant, l);
            let ka = coefficient(m.elastic_modulus, m.area, l);
            (m.name.clone(), (mo / kt, fo / ka))
        })
        .collect();
    Scales { kinds, members }
}

/// S\* of a case from its reference rows.
pub fn scales(case: &Case, model: &Model) -> Scales {
    scales_from(
        maxima(case.rows.iter().map(|r| (r.kind(), r.expected.as_str()))),
        model,
    )
}

/// The S\* a row is floored against.
pub fn row_scale(row: &Row, scales: &Scales) -> f64 {
    match kind_index(row.kind()) {
        Some(k) => scales.kinds[k],
        None => {
            let member = row.key.split_once('.').expect("tw.<m>/ext.<m>").1;
            let (tw, ext) = scales.members[member];
            match row.kind() {
                "twist" => tw,
                "extension" => ext,
                other => panic!("unknown class {other}"),
            }
        }
    }
}

/// The keys of a case's rows below the floor, in row order.
pub fn not_covered(case: &Case, model: &Model, scales: &Scales) -> Vec<String> {
    let _ = model;
    case.rows
        .iter()
        .filter(|row| {
            let exp = crate::exact::Exact::parse_decimal(&row.expected).unwrap();
            let scale = crate::exact::Exact::parse_decimal(case.scale_of(row)).unwrap();
            !crate::exact::covered(&exp, &scale, row_scale(row, scales))
        })
        .map(|row| row.key.clone())
        .collect()
}
