//! RF-INVARIANCE's recorded observations (plan Q7; ROOT: recorded, not
//! asserted). The gate compares each variant with its own R1 expectations.
//! Beside it, each variant's published values are set against its BASE's,
//! mapped by the variant's transform:
//! - OFF (offsets 1e3 and 1e6): every quantity unchanged;
//! - ROT (Q3, Q9): the vectors u, θ, R and S rotate by Q; N, T, Mb, tw and
//!   ext are unchanged;
//! - RELABEL: nodes and members renamed, member directions reversed, so
//!   Mb.i ↔ Mb.j; everything else unchanged;
//! - UNITS-mm: lengths, moments and extensions ×1,000; forces, rotations and
//!   twists unchanged.
//!
//! Each observation is the largest |variant − mapped BASE| in units of the
//! variant's predicate allowance 1e-9·max(|exp|, scale), formed in binary64
//! for display (never a decision), and the number of rows whose bits are
//! identical where the map is the identity (OFF, RELABEL). Nodes are matched
//! by coordinates, members by their end nodes.
use crate::cases::{Case, Model, Target};
use crate::compare::Observed;
use crate::lane::{observe, CaseRun};
use serde_json::{json, Value};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Transform {
    Offset([f64; 3]),
    Rotation([[f64; 3]; 3]),
    Relabel,
    Millimetres,
}

fn transform_of(id: &str) -> Option<Transform> {
    let q3 = [[1.0, 2.0, 2.0], [2.0, 1.0, -2.0], [-2.0, 2.0, -1.0]].map(|r| r.map(|x| x / 3.0));
    let q9 = [[1.0, 8.0, 4.0], [8.0, 1.0, -4.0], [-4.0, 4.0, -7.0]].map(|r| r.map(|x| x / 9.0));
    Some(if id.ends_with("-OFF-1e3") {
        Transform::Offset([1e3, -1e3, 1e3])
    } else if id.ends_with("-OFF-1e6") {
        Transform::Offset([1e6, -1e6, 1e6])
    } else if id.ends_with("-ROT-Q3") {
        Transform::Rotation(q3)
    } else if id.ends_with("-ROT-Q9") {
        Transform::Rotation(q9)
    } else if id.ends_with("-RELABEL") {
        Transform::Relabel
    } else if id.ends_with("-UNITS-mm") {
        Transform::Millimetres
    } else {
        return None;
    })
}

/// The variant's coordinates back in the BASE's frame.
fn back(t: Transform, x: [f64; 3]) -> [f64; 3] {
    match t {
        Transform::Offset(o) => [x[0] - o[0], x[1] - o[1], x[2] - o[2]],
        Transform::Rotation(q) => {
            // Qᵀx (Q orthogonal).
            [0, 1, 2].map(|c| q[0][c] * x[0] + q[1][c] * x[1] + q[2][c] * x[2])
        }
        Transform::Relabel => x,
        Transform::Millimetres => x.map(|v| v / 1e3),
    }
}

fn node_map(base: &Model, var: &Model, t: Transform) -> Vec<Option<usize>> {
    var.nodes
        .iter()
        .map(|&x| {
            let b = back(t, x);
            base.nodes
                .iter()
                .position(|p| (0..3).all(|k| (p[k] - b[k]).abs() <= 1e-6 * (1.0 + p[k].abs())))
        })
        .collect()
}

fn scalar(o: &Observed) -> Option<f64> {
    match o {
        Observed::Value(v) => Some(*v),
        // Display only: the magnitude in binary64.
        Observed::Magnitude(y, z) => Some((y * y + z * z).sqrt()),
        Observed::StructuralZero => Some(0.0),
        Observed::Unavailable(_) => None,
    }
}

/// The value the variant should carry for `key`, mapped from the BASE.
fn mapped(
    key: &str,
    t: Transform,
    base: &Case,
    base_run: &CaseRun,
    nodes: &[Option<usize>],
    var: &Model,
) -> Option<f64> {
    let bm = base.model.as_ref().unwrap();
    let value =
        |k: &str| -> Option<f64> { scalar(&observe(bm.resolve(k).ok()?, &base_run.published, bm)) };
    let parts: Vec<&str> = key.split('.').collect();
    let base_node = |name: &str| -> Option<&str> {
        let i = var.node_index(name)? as usize;
        Some(bm.node_names[nodes[i]?].as_str())
    };
    let base_member = |name: &str| -> Option<(String, bool)> {
        let m = var.member(name)?;
        let (i, j) = (nodes[m.node_i as usize]?, nodes[m.node_j as usize]?);
        bm.members.iter().find_map(|b| {
            let (bi, bj) = (b.node_i as usize, b.node_j as usize);
            if (bi, bj) == (i, j) {
                Some((b.name.clone(), false))
            } else if (bi, bj) == (j, i) {
                Some((b.name.clone(), true))
            } else {
                None
            }
        })
    };
    // The base key with the node or member renamed.
    let (base_key, is_vector, class_factor) = match parts.as_slice() {
        ["u", n, c] => (format!("u.{}.{c}", base_node(n)?), true, 1e3),
        ["th", n, c] => (format!("th.{}.{c}", base_node(n)?), true, 1.0),
        ["R", n, c] => {
            let f = if c.starts_with('U') { 1.0 } else { 1e3 };
            (format!("R.{}.{c}", base_node(n)?), true, f)
        }
        ["S", n, i, c] => {
            let f = if c.starts_with('F') { 1.0 } else { 1e3 };
            (format!("S.{}.{i}.{c}", base_node(n)?), true, f)
        }
        ["N", m] => (format!("N.{}", base_member(m)?.0), false, 1.0),
        ["T", m] => (format!("T.{}", base_member(m)?.0), false, 1e3),
        ["tw", m] => (format!("tw.{}", base_member(m)?.0), false, 1.0),
        ["ext", m] => (format!("ext.{}", base_member(m)?.0), false, 1e3),
        ["Mb", m, end] => {
            let (b, reversed) = base_member(m)?;
            let end = match (*end, reversed) {
                ("i", true) => "j",
                ("j", true) => "i",
                (e, _) => e,
            };
            (format!("Mb.{b}.{end}"), false, 1e3)
        }
        _ => return None,
    };
    match t {
        Transform::Offset(_) | Transform::Relabel => value(&base_key),
        Transform::Millimetres => Some(value(&base_key)? * class_factor),
        Transform::Rotation(q) if is_vector => {
            // The same quantity's three components at the BASE's node.
            let (stem, last) = base_key.rsplit_once('.').unwrap();
            let (letter, axis) = last.split_at(1);
            let row = ["X", "Y", "Z"].iter().position(|a| *a == axis)?;
            let mut sum = 0.0;
            for (k, a) in ["X", "Y", "Z"].iter().enumerate() {
                sum += q[row][k] * value(&format!("{stem}.{letter}{a}"))?;
            }
            Some(sum)
        }
        Transform::Rotation(_) => value(&base_key),
    }
}

/// One variant against its BASE.
pub fn observe_variant(base: &Case, base_run: &CaseRun, var: &Case, var_run: &CaseRun) -> Value {
    let t = transform_of(&var.id).expect("an RF-INVARIANCE variant");
    let (bm, vm) = (base.model.as_ref().unwrap(), var.model.as_ref().unwrap());
    let nodes = node_map(bm, vm, t);
    let (mut compared, mut identical, mut unmatched) = (0usize, 0usize, 0usize);
    let (mut worst, mut worst_key) = (0.0f64, String::new());
    for row in &var.rows {
        let target = vm.resolve(&row.key).unwrap();
        let (Some(got), Some(want)) = (
            scalar(&observe(target, &var_run.published, vm)),
            mapped(&row.key, t, base, base_run, &nodes, vm),
        ) else {
            unmatched += 1;
            continue;
        };
        compared += 1;
        if matches!(t, Transform::Offset(_) | Transform::Relabel) && got.to_bits() == want.to_bits()
        {
            identical += 1;
        }
        let exp: f64 = row.expected.parse().unwrap();
        let scale: f64 = var.scale_of(row).parse().unwrap();
        let allowance = 1e-9 * exp.abs().max(scale);
        let ratio = if target == Target::StructuralZero || allowance == 0.0 {
            0.0
        } else {
            (got - want).abs() / allowance
        };
        if ratio > worst {
            worst = ratio;
            worst_key = row.key.clone();
        }
    }
    json!({
        "variant": var.id, "base": base.id,
        "rows_compared": compared, "rows_unmatched": unmatched,
        "bit_identical": if matches!(t, Transform::Offset(_) | Transform::Relabel) {
            Value::from(identical)
        } else {
            Value::Null
        },
        "largest_difference_in_allowances": worst,
        "at": worst_key,
    })
}

/// Every RF-INVARIANCE variant against its BASE, from runs keyed by case id.
pub fn observations(cases: &[Case], runs: &[CaseRun]) -> Vec<Value> {
    let run = |id: &str| runs.iter().find(|r| r.id == id).unwrap();
    let mut out = Vec::new();
    for var in cases.iter().filter(|c| transform_of(&c.id).is_some()) {
        let stem = [
            "-OFF-1e3",
            "-OFF-1e6",
            "-ROT-Q3",
            "-ROT-Q9",
            "-RELABEL",
            "-UNITS-mm",
        ]
        .iter()
        .find_map(|s| var.id.strip_suffix(s))
        .unwrap();
        let base_id = format!("{stem}-BASE");
        let base = cases.iter().find(|c| c.id == base_id).unwrap();
        out.push(observe_variant(base, run(&base.id), var, run(&var.id)));
    }
    out
}
