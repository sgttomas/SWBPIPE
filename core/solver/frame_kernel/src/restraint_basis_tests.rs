//! S-K1 step 1 tests (DEL-04-03 Work `skew-restraints.md` §15 step 1), against
//! the frozen references `validation/references/skew_restraints/references.json`.
//! The references are read, never regenerated. Binary64 fields are hex floats
//! and are compared bit for bit where the design binds bits (policy, Q, T, w,
//! n_w, §7.2's attribution bits). Values the design binds within 1e-12
//! relative are compared against the references' nearest binary64 of the
//! exact rational (a proxy within 2^-53 relative of the exact value); exact
//! rationals that fit in i128 are also parsed and checked against that field.
use super::*;
use crate::load_ledger::LoadLedger;
use crate::structural::{SparsePattern, SparseStiffness, StiffnessContribution};

const REFERENCES: &str =
    include_str!("../../../../validation/references/skew_restraints/references.json");


/// M-1: the adopted §2.2 strings, test-local; the product's one source is PP
/// `document_members::skew_code` (the product references test asserts them by name,
/// against the references).
fn code_text(code: SkewCode) -> &'static str {
    match code {
        SkewCode::DirectionInvalid => "SUPPORT_DIRECTION_INVALID",
        SkewCode::DirectionZero => "SUPPORT_DIRECTION_ZERO",
        SkewCode::ExactUnsupported => "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED",
        SkewCode::NonlinearModelUnsupported => "SUPPORT_DIRECTION_SKEW_NONLINEAR_MODEL_UNSUPPORTED",
        SkewCode::RangeUnsupported => "SUPPORT_DIRECTION_RANGE_UNSUPPORTED",
        SkewCode::ImposedDisplacementSkewSupport => "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED",
        SkewCode::NodeUnconnected => "SUPPORT_DIRECTION_NODE_UNCONNECTED",
        SkewCode::StopsPerNode => "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED",
        SkewCode::ConstraintRedundant => "SUPPORT_CONSTRAINT_REDUNDANT",
        SkewCode::NodePrescribedMotion => "SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED",
        SkewCode::ForceScaledUnsupported => "SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED",
        SkewCode::ReactionRange => "SUPPORT_SKEW_REACTION_RANGE_UNSUPPORTED",
    }
}

// ------------------------------------------------------------------ JSON

#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Number(String),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    fn at(&self, key: &str) -> &Json {
        self.get(key).unwrap_or_else(|| panic!("missing key {key}"))
    }
    fn text(&self) -> &str {
        match self {
            Json::Str(s) => s,
            other => panic!("not a string: {other:?}"),
        }
    }
    fn items(&self) -> &[Json] {
        match self {
            Json::Array(items) => items,
            other => panic!("not an array: {other:?}"),
        }
    }
    fn entries(&self) -> &[(String, Json)] {
        match self {
            Json::Object(entries) => entries,
            other => panic!("not an object: {other:?}"),
        }
    }
    fn int(&self) -> i64 {
        match self {
            Json::Number(n) => n.parse().expect("integer"),
            other => panic!("not a number: {other:?}"),
        }
    }
    fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }
    fn boolean(&self) -> bool {
        match self {
            Json::Bool(b) => *b,
            other => panic!("not a bool: {other:?}"),
        }
    }
    /// A hex-float string, or the `binary64` member of a value record.
    fn b64(&self) -> f64 {
        match self {
            Json::Str(s) => hex(s),
            Json::Object(_) => self.at("binary64").b64(),
            other => panic!("not a binary64: {other:?}"),
        }
    }
}

fn parse(text: &str) -> Json {
    let mut p = Parser {
        bytes: text.as_bytes(),
        at: 0,
    };
    let value = p.value();
    p.space();
    assert_eq!(p.at, p.bytes.len(), "trailing data");
    value
}

struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn space(&mut self) {
        while self.at < self.bytes.len() && self.bytes[self.at].is_ascii_whitespace() {
            self.at += 1;
        }
    }
    fn next(&mut self) -> u8 {
        self.space();
        let b = self.bytes[self.at];
        self.at += 1;
        b
    }
    fn value(&mut self) -> Json {
        self.space();
        match self.bytes[self.at] {
            b'{' => {
                self.at += 1;
                let mut entries = Vec::new();
                self.space();
                if self.bytes[self.at] == b'}' {
                    self.at += 1;
                    return Json::Object(entries);
                }
                loop {
                    self.space();
                    let key = self.string();
                    assert_eq!(self.next(), b':');
                    entries.push((key, self.value()));
                    match self.next() {
                        b'}' => return Json::Object(entries),
                        b',' => {}
                        other => panic!("object separator {other}"),
                    }
                }
            }
            b'[' => {
                self.at += 1;
                let mut items = Vec::new();
                self.space();
                if self.bytes[self.at] == b']' {
                    self.at += 1;
                    return Json::Array(items);
                }
                loop {
                    items.push(self.value());
                    match self.next() {
                        b']' => return Json::Array(items),
                        b',' => {}
                        other => panic!("array separator {other}"),
                    }
                }
            }
            b'"' => Json::Str(self.string()),
            b't' => {
                self.at += 4;
                Json::Bool(true)
            }
            b'f' => {
                self.at += 5;
                Json::Bool(false)
            }
            b'n' => {
                self.at += 4;
                Json::Null
            }
            _ => {
                let start = self.at;
                while self.at < self.bytes.len()
                    && matches!(self.bytes[self.at], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
                {
                    self.at += 1;
                }
                Json::Number(String::from_utf8(self.bytes[start..self.at].to_vec()).unwrap())
            }
        }
    }
    fn string(&mut self) -> String {
        assert_eq!(self.bytes[self.at], b'"');
        self.at += 1;
        let mut out = Vec::new();
        loop {
            let b = self.bytes[self.at];
            self.at += 1;
            match b {
                b'"' => return String::from_utf8(out).unwrap(),
                b'\\' => {
                    let e = self.bytes[self.at];
                    self.at += 1;
                    match e {
                        b'u' => {
                            let code = u32::from_str_radix(
                                std::str::from_utf8(&self.bytes[self.at..self.at + 4]).unwrap(),
                                16,
                            )
                            .unwrap();
                            self.at += 4;
                            let mut buf = [0; 4];
                            out.extend_from_slice(
                                char::from_u32(code).unwrap().encode_utf8(&mut buf).as_bytes(),
                            );
                        }
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        other => out.push(other),
                    }
                }
                other => out.push(other),
            }
        }
    }
}

/// A hex float as Python's `float.hex` writes it, exactly.
fn hex(text: &str) -> f64 {
    let (negative, body) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let body = body.strip_prefix("0x").unwrap_or_else(|| panic!("hex float {text}"));
    let (mantissa, exponent) = body.split_once('p').expect("hex exponent");
    let exponent: i32 = exponent.parse().expect("exponent");
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut m = u64::from_str_radix(int, 16).expect("hex digits");
    for ch in frac.chars() {
        m = m * 16 + u64::from(ch.to_digit(16).expect("hex digit"));
    }
    assert!(m < 1u64 << 53);
    let value = scale_pow2(m as f64, exponent - 4 * frac.len() as i32);
    if negative {
        -value
    } else {
        value
    }
}

/// An exact "n/d" that fits in i128, if it does.
fn rational(text: &str) -> Option<(i128, i128)> {
    let (n, d) = text.split_once('/')?;
    Some((n.parse().ok()?, d.parse().ok()?))
}

fn references() -> Json {
    parse(REFERENCES)
}

fn bits(x: f64) -> u64 {
    x.to_bits()
}

/// The binary64 of a `{exact, binary64}` record, after checking that the
/// exact rational (when it fits in i128) is within half an ulp of it.
fn expected(record: &Json) -> f64 {
    let b = record.at("binary64").b64();
    if let Some(exact) = record.get("exact") {
        if let Some((n, d)) = rational(exact.text()) {
            let x = n as f64 / d as f64;
            assert!(
                (x - b).abs() <= 4.0 * b.abs() * f64::EPSILON || x == b,
                "exact {n}/{d} against {b:e}"
            );
        }
    }
    b
}

fn close(actual: f64, expect: f64, scale: f64, what: &str) {
    let tolerance = 1e-12 * if expect == 0.0 { scale } else { expect.abs() };
    assert!(
        (actual - expect).abs() <= tolerance,
        "{what}: {actual:e} against {expect:e}"
    );
}

fn axis_index(letter: &str) -> usize {
    match letter {
        "x" => 0,
        "y" => 1,
        "z" => 2,
        other => panic!("axis {other}"),
    }
}

fn policy(name: &str) -> [f64; 3] {
    let refs = references();
    let entry = refs
        .at("direction_policy")
        .items()
        .iter()
        .find(|e| e.at("name").text() == name)
        .unwrap_or_else(|| panic!("policy {name}"));
    let a = entry.at("authored").items();
    [a[0].b64(), a[1].b64(), a[2].b64()]
}

// ------------------------------------------------------------------ single-node kernel

/// One kernel case's inputs, as the generator builds them
/// (`generators/skew_reference.py`, `kernel_cases`).
struct Input {
    name: &'static str,
    held: Vec<(&'static str, usize)>,
    stops: Vec<(&'static str, [f64; 3])>,
    springs: Vec<(&'static str, [f64; 3], f64)>,
}

fn s_k1_inputs() -> Vec<Input> {
    let input = |name, held, stops, springs| Input {
        name,
        held,
        stops,
        springs,
    };
    vec![
        input("S1_spring_isotropic", vec![], vec![], vec![("s", [3.0, 4.0, 0.0], 1.0)]),
        input(
            "S2_spring_pair",
            vec![],
            vec![],
            vec![("s1", [3.0, 4.0, 0.0], 1.0), ("s2", [-4.0, 3.0, 0.0], 1.0)],
        ),
        input("R1_stop_isotropic", vec![], vec![("stop", [3.0, 4.0, 0.0])], vec![]),
        input(
            "R1_stop_scaled_2p600",
            vec![],
            vec![("stop", [3.0 * pow2(600), 4.0 * pow2(600), 0.0])],
            vec![],
        ),
        input("R2_stop_anisotropic", vec![], vec![("stop", [3.0, 4.0, 0.0])], vec![]),
        input("R3_stop_negative", vec![], vec![("stop", [-3.0, -4.0, 0.0])], vec![]),
        input("R3_stop_plane_13", vec![], vec![("stop", [1.0, 3.0, 0.0])], vec![]),
        input("R4_guide_plus_stop", vec![("guide_uz", 2)], vec![("stop", [3.0, 4.0, 5.0])], vec![]),
        input(
            "R4_full_fix_one_free_axis",
            vec![("ux", 0), ("uz", 2)],
            vec![("stop", [3.0, 4.0, 0.0])],
            vec![],
        ),
        input(
            "R4_near_dependent_axis",
            vec![("ux", 0)],
            vec![("stop", [1.0, pow2(-500), 0.0])],
            vec![],
        ),
        input(
            "R4_tiny_restricted_direction",
            vec![("ux", 0), ("uz", 2)],
            vec![("stop", [1.0, pow2(-600), 0.0])],
            vec![],
        ),
        input(
            "RS_stop_and_spring",
            vec![],
            vec![("stop", [3.0, 4.0, 0.0])],
            vec![("spring", [0.0, 3.0, 4.0], 1.0)],
        ),
    ]
}

fn kernel_case(name: &str) -> Json {
    references()
        .at("kernel")
        .at("cases")
        .items()
        .iter()
        .find(|c| c.at("name").text() == name)
        .unwrap_or_else(|| panic!("kernel case {name}"))
        .clone()
}

fn triple_of(record: &Json) -> [f64; 3] {
    let items = record.items();
    [expected(&items[0]), expected(&items[1]), expected(&items[2])]
}

/// A dense matrix on the full pattern of its dimension (every entry stored).
fn full(dense: &[Vec<f64>]) -> SparseStiffness {
    let n = dense.len();
    let pattern = SparsePattern::from_positions(n, (0..n).flat_map(|r| (0..n).map(move |c| (r, c)))).unwrap();
    let contributions: Vec<StiffnessContribution> = (0..n)
        .flat_map(|r| (0..n).map(move |c| (r, c)))
        .map(|(row, col)| StiffnessContribution {
            row,
            col,
            value: dense[row][col],
        })
        .collect();
    SparseStiffness::from_pattern_and_contributions(pattern, &contributions).unwrap()
}

/// Test-local binary64 Cholesky (no pivoting, sums in index order): odd in a
/// diagonal sign flip, so it meets P-SYM.
fn cholesky_solve(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = b.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut s = a[i][j];
            for k in 0..j {
                s -= l[i][k] * l[j][k];
            }
            if i == j {
                assert!(s > 0.0, "positive pivot");
                l[i][i] = s.sqrt();
            } else {
                l[i][j] = s / l[j][j];
            }
        }
    }
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut s = b[i];
        for k in 0..i {
            s -= l[i][k] * y[k];
        }
        y[i] = s / l[i][i];
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = y[i];
        for k in i + 1..n {
            s -= l[k][i] * x[k];
        }
        x[i] = s / l[i][i];
    }
    x
}

struct Run {
    basis: RestraintBasis,
    k: SparseStiffness,
    k_prime: SparseStiffness,
    v: Vec<f64>,
    u: Vec<f64>,
    /// r = K u − f on the node's translations, each one exact sum rounded once.
    r: [f64; 3],
}

/// §15 step 1's single-node model: a 6×6 node, translations diag(tip) plus
/// the skewed spring blocks (after any axis spring), rotations 1 and
/// restrained; the coordinate solve by Cholesky; the lift; the residual rows.
fn run(
    tip: [f64; 3],
    load: [f64; 3],
    held: &[usize],
    stops: &[(&str, [f64; 3])],
    springs: &[(&str, [f64; 3], f64)],
) -> Result<Run, Vec<BasisFinding>> {
    let mut axis: Vec<usize> = held.iter().copied().chain([3, 4, 5]).collect();
    axis.sort_unstable();
    let stops = stops
        .iter()
        .map(|&(id, d)| SkewStop {
            support_id: id.to_string(),
            node: 0,
            direction: SkewDirection::new(d).expect("skew stop direction"),
        })
        .collect();
    let springs = springs
        .iter()
        .map(|&(id, d, k)| SkewSpring {
            support_id: id.to_string(),
            node: 0,
            direction: SkewDirection::new(d).expect("skew spring direction"),
            stiffness: k,
        })
        .collect();
    let basis = RestraintBasis::build(1, &axis, stops, springs)?;
    let mut dense = vec![vec![0.0; 6]; 6];
    for i in 0..3 {
        dense[i][i] = tip[i];
        dense[i + 3][i + 3] = 1.0;
    }
    for s in 0..basis.springs().len() {
        let block = basis.spring_block(s);
        for i in 0..3 {
            for j in 0..3 {
                dense[i][j] += block[i][j];
            }
        }
    }
    let k = full(&dense);
    let mut ledger = LoadLedger::new();
    for (dof, &f) in load.iter().enumerate() {
        if f != 0.0 {
            ledger.push("load", dof, f);
        }
    }
    let force = ledger.finish(6).unwrap();
    let k_prime = basis.transform_stiffness(&k).expect("K'");
    let reduction = basis.reduce(&k_prime, &force, None).expect("reduce");
    let free = &reduction.reduced.free_dofs;
    let a: Vec<Vec<f64>> = free
        .iter()
        .map(|&r| free.iter().map(|&c| k_prime.get(r, c)).collect())
        .collect();
    let x = cholesky_solve(&a, reduction.reduced.force.values());
    let mut v = vec![0.0; 6];
    for (p, &dof) in free.iter().enumerate() {
        v[dof] = x[p];
    }
    let u = basis.lift(&v).expect("lift");
    let mut r = [0.0; 3];
    for (i, slot) in r.iter_mut().enumerate() {
        let mut sum = ExactAccumulator::new();
        for (j, &uj) in u.iter().enumerate() {
            sum.add_product(k.get(i, j), uj).unwrap();
        }
        force.accumulate_dof(i, &mut sum, true).unwrap();
        *slot = sum.round().unwrap();
    }
    Ok(Run {
        basis,
        k,
        k_prime,
        v,
        u,
        r,
    })
}

fn tip_and_load(case: &Json) -> ([f64; 3], [f64; 3]) {
    (
        triple_of(case.at("tip_stiffness_diagonal")),
        triple_of(case.at("load")),
    )
}

fn ulp(x: f64) -> f64 {
    let biased = (x.abs().to_bits() >> 52) as u32;
    if biased <= 52 {
        f64::from_bits(if biased == 0 { 1 } else { 1u64 << (biased - 1) })
    } else {
        f64::from_bits(u64::from(biased - 52) << 52)
    }
}

/// The scale of a case's reactions and displacements (for exact zeros).
fn case_scale(expect: &Json) -> f64 {
    let mut scale: f64 = 0.0;
    for u in expect.at("u").items() {
        scale = scale.max(expected(u).abs());
    }
    for (_, record) in expect.at("records").entries() {
        for key in ["global_force", "components"] {
            if let Some(values) = record.get(key) {
                for value in values.items() {
                    scale = scale.max(expected(value).abs());
                }
            }
        }
    }
    scale
}

// ------------------------------------------------------------------ direction policy

#[test]
fn direction_policy_matches_references_bit_for_bit() {
    let refs = references();
    let entries = refs.at("direction_policy").items();
    assert_eq!(entries.len(), 30);
    let mut checked = 0;
    for entry in entries {
        let name = entry.at("name").text();
        let expect = entry.at("expect");
        let Some(authored) = entry.get("authored") else {
            // Text-level inputs are the document validator's (PP). The one
            // that parses to a binary64 (1e999 = +inf) is refused here too.
            if name == "non_finite_overflow_text" {
                let d = [f64::INFINITY, 0.0, 0.0];
                assert_eq!(SkewDirection::new(d), Err(DirectionError::NonFinite));
                let facts = SkewRecordFacts { authored: d, pressure_contract: false, nonlinear_model: false };
                assert_eq!(judge(&facts).unwrap_err().code, SkewCode::DirectionInvalid);
                assert_eq!(code_text(SkewCode::DirectionInvalid), expect.at("code").text());
            }
            continue;
        };
        let a = authored.items();
        let d = [a[0].b64(), a[1].b64(), a[2].b64()];
        let facts = SkewRecordFacts { authored: d, pressure_contract: false, nonlinear_model: false };
        match expect.at("class").text() {
            "skew" => {
                let dir = SkewDirection::new(d).unwrap_or_else(|e| panic!("{name}: {e:?}"));
                assert_eq!(dir.authored().map(bits), d.map(bits), "{name}: stored as authored");
                assert_eq!(i64::from(dir.scale_exponent()), expect.at("scale_exponent").int(), "{name}");
                let scaled = expect.at("scaled").items();
                for i in 0..3 {
                    assert_eq!(bits(dir.scaled()[i]), bits(scaled[i].b64()), "{name}: scaled[{i}]");
                }
                assert_eq!(dir.pivot(), axis_index(expect.at("pivot").text()), "{name}: pivot");
                assert_eq!(
                    bits(dir.scaled_norm_squared()),
                    bits(expect.at("norm_squared_of_scaled").b64()),
                    "{name}: n"
                );
                assert_eq!(judge(&facts), Ok(Some(dir)), "{name}: judge");
                // The node matrix Q with every axis free, and the free columns T.
                let basis = RestraintBasis::build(
                    1,
                    &[3, 4, 5],
                    vec![SkewStop { support_id: "s".into(), node: 0, direction: dir }],
                    vec![],
                )
                .unwrap();
                let node = basis.node_basis(0).unwrap();
                assert_eq!(node.restricted_scale_exponent, 0, "{name}: A = ∅ gives ŝ = 0");
                assert_eq!(node.w.map(bits), dir.scaled().map(bits), "{name}: w = d̃");
                let q_rows = expect.at("Q_rows").items();
                for i in 0..3 {
                    for c in 0..3 {
                        assert_eq!(bits(node.q[i][c]), bits(q_rows[i].items()[c].b64()), "{name}: Q[{i}][{c}]");
                    }
                }
                let t = expect.at("T_columns_all_axes_free").items();
                let columns: Vec<usize> = node.free_slots().collect();
                assert_eq!(columns.len(), t.len(), "{name}: T columns");
                for (column, &c) in t.iter().zip(&columns) {
                    for i in 0..3 {
                        assert_eq!(bits(node.q[i][c]), bits(column.items()[i].b64()), "{name}: T[{i}][{c}]");
                    }
                }
                // No division: every Q entry is ±w_i, 1.0 or +0.0.
                for row in node.q {
                    for x in row {
                        assert!(
                            x.to_bits() == 0 || x == 1.0 || node.w.iter().any(|&w| w != 0.0 && x.abs() == w.abs()),
                            "{name}: Q entry {x:e}"
                        );
                    }
                }
            }
            "axis" => {
                let axis = axis_index(&expect.at("axis").text().to_lowercase());
                assert_eq!(SkewDirection::new(d), Err(DirectionError::AxisAligned { axis }), "{name}");
                assert_eq!(judge(&facts), Ok(None), "{name}: no basis, today's path");
            }
            "refused" => {
                let code = expect.at("code").text();
                let refusal = judge(&facts).unwrap_err();
                assert_eq!(code_text(refusal.code), code, "{name}");
                match code {
                    "SUPPORT_DIRECTION_ZERO" => assert_eq!(SkewDirection::new(d), Err(DirectionError::Zero)),
                    "SUPPORT_DIRECTION_RANGE_UNSUPPORTED" => {
                        let mut components = [false; 3];
                        for c in expect.at("subnormal_components").items() {
                            components[axis_index(c.text())] = true;
                        }
                        let scale_exponent = expect.at("scale_exponent").int() as i32;
                        assert_eq!(
                            SkewDirection::new(d),
                            Err(DirectionError::RangeUnsupported { components, scale_exponent }),
                            "{name}"
                        );
                        assert_eq!(refusal.direction, Some(DirectionError::RangeUnsupported { components, scale_exponent }));
                    }
                    other => panic!("{name}: unexpected code {other}"),
                }
            }
            other => panic!("{name}: class {other}"),
        }
        checked += 1;
    }
    assert_eq!(checked, 26);
}

#[test]
fn judge_gate_order_and_named_codes() {
    let skew = [3.0, 4.0, 0.0];
    let range = policy("range_tiny_third");
    let facts = |authored, pressure_contract, nonlinear_model| SkewRecordFacts {
        authored,
        pressure_contract,
        nonlinear_model,
    };
    let code = |f: SkewRecordFacts| judge(&f).unwrap_err().code;
    // S-K5 (EX) and S-K4 (nonlinear) inputs, named.
    assert_eq!(code(facts(skew, true, false)), SkewCode::ExactUnsupported);
    assert_eq!(code(facts(skew, false, true)), SkewCode::NonlinearModelUnsupported);
    // Gate order with two failing conditions: 2 before 3 before 4.
    assert_eq!(code(facts(skew, true, true)), SkewCode::ExactUnsupported);
    assert_eq!(code(facts(range, true, false)), SkewCode::ExactUnsupported);
    assert_eq!(code(facts(range, false, true)), SkewCode::NonlinearModelUnsupported);
    assert_eq!(code(facts(range, false, false)), SkewCode::RangeUnsupported);
    // An axis-aligned direction is never a skewed record: no code at all.
    assert_eq!(judge(&facts([0.0, -2.0, 0.0], true, true)), Ok(None));
    // Every S-K1 code string is the references' (`negatives`).
    let refs = references();
    let codes: Vec<&str> = refs.at("negatives").items().iter().map(|n| n.at("code").text()).collect();
    for c in [
        SkewCode::DirectionInvalid,
        SkewCode::DirectionZero,
        SkewCode::ExactUnsupported,
        SkewCode::NonlinearModelUnsupported,
        SkewCode::RangeUnsupported,
        SkewCode::ImposedDisplacementSkewSupport,
        SkewCode::NodeUnconnected,
        SkewCode::StopsPerNode,
        SkewCode::ConstraintRedundant,
        SkewCode::NodePrescribedMotion,
        SkewCode::ForceScaledUnsupported,
        SkewCode::ReactionRange,
    ] {
        assert!(codes.contains(&code_text(c)), "{}", code_text(c));
    }
}

// ------------------------------------------------------------------ S-K1 kernel cases

#[test]
fn s_k1_kernel_cases_match_references() {
    let mut checked = 0;
    for input in s_k1_inputs() {
        let name = input.name;
        let case = kernel_case(name);
        assert_eq!(case.at("slice").text(), "S-K1", "{name}");
        let expect = case.at("expect");
        let (tip, load) = tip_and_load(&case);
        let held: Vec<usize> = input.held.iter().map(|&(_, a)| a).collect();
        let run = run(tip, load, &held, &input.stops, &input.springs).unwrap_or_else(|f| panic!("{name}: {f:?}"));
        let scale = case_scale(expect);
        // K′ is symmetric bit for bit and every entry is K's where untouched.
        for r in 0..6 {
            for c in 0..6 {
                assert_eq!(bits(run.k_prime.get(r, c)), bits(run.k_prime.get(c, r)), "{name}: K' symmetry");
                let terms = run.basis.congruence_terms(r, c);
                assert!(terms.len() <= 4, "{name}: S-K1 congruence terms");
                if terms.len() == 1 && terms.as_slice()[0].left == 1.0 && terms.as_slice()[0].right == 1.0 {
                    assert_eq!(bits(run.k_prime.get(r, c)), bits(run.k.get(r, c)), "{name}: untouched entry");
                }
            }
        }
        // u within 1e-12 relative.
        let u = expect.at("u").items();
        for i in 0..3 {
            close(run.u[i], expected(&u[i]), scale, &format!("{name}: u[{i}]"));
        }
        // Rank.
        let rank = expect.at("rank").int() as usize;
        match run.basis.node_basis(0) {
            Some(node) => assert_eq!(node.rank(), rank, "{name}: rank"),
            None => assert_eq!(rank, 0, "{name}: no stop"),
        }
        // Records.
        let attribution = run.basis.node_basis(0).map(|_| run.basis.attribute(0, run.r).expect("attribute"));
        for (record, value) in expect.at("records").entries() {
            match value.at("kind").text() {
                "skew_stop" => {
                    let a = attribution.as_ref().expect("stop node");
                    let g = triple_of(value.at("global_force"));
                    for i in 0..3 {
                        close(a.stop.global[i], g[i], scale, &format!("{name}: {record} R[{i}]"));
                    }
                    close(a.stop.along, value.at("along_direction").b64(), scale, &format!("{name}: {record} along"));
                    assert_eq!(value.at("d_dot_u_exact").text(), "0/1");
                }
                "rigid_axis_rows" => {
                    let a = attribution.as_ref().expect("stop node");
                    let slot = input.held.iter().find(|&&(id, _)| id == record.as_str()).expect("axis record").1;
                    let component = a.axis.iter().find(|&&(dof, _)| dof == slot).expect("axis component").1;
                    close(component, expected(&value.at("components").items()[0]), scale, &format!("{name}: {record}"));
                }
                "skew_spring" => {
                    let s = input.springs.iter().position(|&(id, _, _)| id == record.as_str()).expect("spring");
                    let reaction = run.basis.spring_reaction(s, &run.u).expect("spring reaction");
                    let g = triple_of(value.at("global_force"));
                    for i in 0..3 {
                        close(reaction.global[i], g[i], scale, &format!("{name}: {record} R[{i}]"));
                    }
                    close(reaction.along, value.at("along_direction").b64(), scale, &format!("{name}: {record} along"));
                }
                other => panic!("{name}: kind {other}"),
            }
        }
        if let Some(node) = run.basis.node_basis(0) {
            // The exact constraint on the unrounded lift.
            assert!(run.basis.constraint_residual(0, &run.v).unwrap().is_zero(), "{name}: d̃ᵀ(Qv) = 0 exactly");
            // The published lift's residual: 2|d̃ᵀu_p| ≤ Σ|d̃_i|·ulp(u_p,i).
            let d = run.basis.stops()[node.stop].direction.scaled();
            let mut residual = ExactAccumulator::new();
            for i in 0..3 {
                residual.add_product(d[i], run.u[i]).unwrap();
            }
            let sign = f64::from(residual.signum());
            let mut margin = ExactAccumulator::new();
            for i in 0..3 {
                margin.add_product(d[i].abs(), ulp(run.u[i])).unwrap();
                margin.add_product(-2.0 * sign * d[i], run.u[i]).unwrap();
            }
            assert!(margin.signum() >= 0, "{name}: published-lift bound");
            check_attribution_block(name, &run, node, expect.at("attribution"), scale);
        }
        checked += 1;
    }
    assert_eq!(checked, 12);
}

fn check_attribution_block(name: &str, run: &Run, node: &NodeBasis, block: &Json, scale: f64) {
    let free: Vec<usize> = block.at("free_axes").items().iter().map(|a| axis_index(a.text())).collect();
    for i in 0..3 {
        assert_eq!(node.free_axes[i], free.contains(&i), "{name}: F");
    }
    assert_eq!(node.pivot, axis_index(block.at("pivot_on_F").text()), "{name}: pivot on F");
    assert_eq!(i64::from(node.restricted_scale_exponent), block.at("restricted_scale_exponent").int(), "{name}: ŝ");
    let w = block.at("w").items();
    for i in 0..3 {
        assert_eq!(bits(node.w[i]), bits(w[i].b64()), "{name}: w[{i}]");
    }
    let (_, trace) = run.basis.attribute_traced(0, run.r).unwrap();
    assert_eq!(bits(trace.n_w), bits(block.at("n_w").b64()), "{name}: n_w");
    close(trace.mu, expected(block.at("mu_per_unit_w")), scale, &format!("{name}: μ"));
    // n_F formed directly would round to zero in the tiny case; n_w does not.
    let mut n_f = ExactAccumulator::new();
    let d = run.basis.stops()[node.stop].direction.scaled();
    for &i in &free {
        n_f.add_product(d[i], d[i]).unwrap();
    }
    assert_eq!(n_f.round().unwrap() == 0.0, block.at("n_F_rounds_to_zero").boolean(), "{name}: n_F");
    assert!(trace.n_w >= 1.0 && trace.n_w < 12.0);
    // Q on F from w, bit for bit.
    let columns: Vec<usize> = node.free_slots().collect();
    let expect_columns = block.at("Q_on_F_columns_from_w").items();
    assert_eq!(columns.len(), expect_columns.len(), "{name}: Q columns on F");
    for (column, &c) in expect_columns.iter().zip(&columns) {
        for i in 0..3 {
            assert_eq!(bits(node.q[i][c]), bits(column.items()[i].b64()), "{name}: Q[{i}][{c}]");
        }
    }
    // each physical translation's free coordinates, or i32::MIN.
    let n = 6;
    let exponents: Vec<i32> = (0..n)
        .map(|c| if run.basis.restrained_coordinates().contains(&c) { i32::MIN } else { 0 })
        .collect();
    let lifted = run
        .basis
        .lift_member_bound(
            &run.v,
            &MemberBoundInputs { w: vec![0.0; n], w2: vec![0.0; n], exponents, lambda_sigma: 1.0, lift_radius: Vec::new() },
        )
        .unwrap();
    for (axis, coords) in block.at("lifted_remainder_exponent").entries() {
        let i = axis_index(axis);
        let e = lifted.bound.exponents[i];
        match coords {
            Json::Str(s) => {
                assert_eq!(s, "i32::MIN");
                assert_eq!(e, i32::MIN, "{name}: E_{axis}");
                assert_eq!(lifted.bound.lift_radius[i], 0.0);
                assert_eq!(lifted.u[i], 0.0, "{name}: u_{axis} exactly prescribed");
            }
            _ => {
                let list: Vec<usize> = coords.items().iter().map(|c| axis_index(c.text())).collect();
                let mapped: Vec<usize> = (0..3)
                    .filter(|&c| node.q[i][c] != 0.0 && !run.basis.restrained_coordinates().contains(&c))
                    .collect();
                assert_eq!(mapped, list, "{name}: free coordinates of {axis}");
                // E = ⌈log2 Σ_j |q_ij|⌉ with every exp_j = 0.
                let sum: f64 = mapped.iter().map(|&c| node.q[i][c].abs()).sum();
                assert_eq!(f64::from(e), sum.log2().ceil(), "{name}: E_{axis}");
            }
        }
    }
}

#[test]
fn s_k1_spring_pair_matches_its_axis_twin() {
    let pair = kernel_case("S2_spring_pair");
    let twin = kernel_case("S2_axis_twin");
    let (tip, load) = tip_and_load(&pair);
    let run = run(tip, load, &[], &[], &[("s1", [3.0, 4.0, 0.0], 1.0), ("s2", [-4.0, 3.0, 0.0], 1.0)]).unwrap();
    assert!(!run.basis.has_constraint_transform());
    let twin_u = twin.at("expect").at("u").items();
    for i in 0..3 {
        close(run.u[i], expected(&twin_u[i]), 0.5, "S-2 equivalence");
    }
}

#[test]
fn s_k1_refuses_stops_per_node_and_redundant_sets() {
    let refs = references();
    let mut k2 = 0;
    let stops_of = |name: &str| -> (Vec<usize>, Vec<(&'static str, [f64; 3])>) {
        match name {
            "K2_two_stops_3d" => (vec![], vec![("stop1", [3.0, 4.0, 0.0]), ("stop2", [0.0, 3.0, 4.0])]),
            "K2_orthogonal_pair" => (vec![], vec![("stop1", [3.0, 4.0, 0.0]), ("stop2", [-4.0, 3.0, 0.0])]),
            "K2_oblique_pair" => (vec![], vec![("stop1", [3.0, 4.0, 0.0]), ("stop2", [4.0, 3.0, 0.0])]),
            "K2_near_parallel" => (vec![], vec![("stop1", [3.0, 4.0, 0.0]), ("stop2", [3.0, 4.0 + pow2(-48), 0.0])]),
            "K2_parallel" => (vec![], vec![("stop1", [3.0, 4.0, 0.0]), ("stop2", [6.0, 8.0, 0.0])]),
            "K2_antiparallel_scaled" => {
                (vec![], vec![("stop1", [3.0, 4.0, 0.0]), ("stop2", [-3.0 / 1024.0, -4.0 / 1024.0, 0.0])])
            }
            "K2_three_stops_fixed" => (
                vec![],
                vec![("stop1", [3.0, 4.0, 0.0]), ("stop2", [0.0, 3.0, 4.0]), ("stop3", [1.0, 1.0, 1.0])],
            ),
            "K2_axis_plus_two_stops_redundant" => {
                (vec![2], vec![("stop1", [3.0, 4.0, 0.0]), ("stop2", [3.0, 4.0, 7.0])])
            }
            other => panic!("K2 case {other}"),
        }
    };
    for case in refs.at("kernel").at("cases").items() {
        let name = case.at("name").text();
        if !name.starts_with("K2_") {
            continue;
        }
        assert_eq!(case.at("s_k1_refusal").text(), code_text(SkewCode::StopsPerNode), "{name}");
        let (held, stops) = stops_of(name);
        let (tip, load) = tip_and_load(case);
        let findings = run(tip, load, &held, &stops, &[]).err().unwrap_or_else(|| panic!("{name} admitted"));
        assert_eq!(findings.len(), 1, "{name}");
        match &findings[0] {
            BasisFinding::StopsPerNode { node: 0, supports } => {
                let ids: Vec<&str> = stops.iter().map(|&(id, _)| id).collect();
                assert_eq!(supports, &ids, "{name}: every stop named");
            }
            other => panic!("{name}: {other:?}"),
        }
        assert_eq!(findings[0].code(), Some(SkewCode::StopsPerNode));
        k2 += 1;
    }
    assert_eq!(k2, 8);
    // R4_redundant_axes: d̃_F = 0 on F = {z}.
    let case = kernel_case("R4_redundant_axes");
    assert_eq!(case.at("expect").at("code").text(), code_text(SkewCode::ConstraintRedundant));
    let (tip, load) = tip_and_load(&case);
    let findings = run(tip, load, &[0, 1], &[("stop", [3.0, 4.0, 0.0])], &[]).err().unwrap();
    assert_eq!(findings, vec![BasisFinding::Redundant { node: 0, supports: vec!["stop".into()] }]);
    // Every node is reported, ascending: a stops-per-node node and a redundant node.
    let d = |v| SkewDirection::new(v).unwrap();
    let stop = |id: &str, node, v| SkewStop { support_id: id.into(), node, direction: d(v) };
    let findings = RestraintBasis::build(
        3,
        &[12, 13],
        vec![
            stop("c", 2, [3.0, 4.0, 0.0]),
            stop("a", 0, [3.0, 4.0, 0.0]),
            stop("b", 0, [4.0, 3.0, 0.0]),
            stop("ok", 1, [1.0, 1.0, 0.0]),
        ],
        vec![],
    )
    .unwrap_err();
    assert_eq!(
        findings,
        vec![
            BasisFinding::StopsPerNode { node: 0, supports: vec!["a".into(), "b".into()] },
            BasisFinding::Redundant { node: 2, supports: vec!["c".into()] },
        ]
    );
}

#[test]
fn s_k3_motion_at_a_stop_node_is_refused_by_name() {
    let case = kernel_case("K3_moving_guide_plus_stop");
    assert_eq!(case.at("s_k1_refusal").text(), code_text(SkewCode::NodePrescribedMotion));
    let d = SkewDirection::new([3.0, 4.0, 5.0]).unwrap();
    let basis = RestraintBasis::build(
        1,
        &[2, 3, 4, 5],
        vec![SkewStop { support_id: "stop".into(), node: 0, direction: d }],
        vec![],
    )
    .unwrap();
    let h = 1.0 / 1024.0;
    // §2.2 item 9 is judged before reduction; rotations may move.
    assert_eq!(basis.judge_prescribed(&[(2, h), (3, 0.5)]), vec![2]);
    assert!(basis.judge_prescribed(&[(2, 0.0), (2, -0.0), (4, 1.0)]).is_empty());
    // Reduction refuses it too, never forms a reduced system with it.
    let k = full(&(0..6).map(|i| (0..6).map(|j| if i == j { 1.0 } else { 0.0 }).collect()).collect::<Vec<_>>());
    let force = LoadLedger::new().finish(6).unwrap();
    let k_prime = basis.transform_stiffness(&k).unwrap();
    assert_eq!(
        basis.reduce(&k_prime, &force, Some(&[h, 0.0, 0.0, 0.0][..])).unwrap_err(),
        BasisError::PrescribedMotion { dof: 2 }
    );
    assert!(basis.reduce(&k_prime, &force, Some(&[0.0, 0.5, 0.0, 0.0][..])).is_ok());
    // Ground motion on the skewed record itself (item 5) has no kernel
    // representation; the product's `imposed_motion::validate` names it.
    for name in ["K3_stop_ground_meaning_A", "K3_A_ground_x_with_ux_held", "K3_B_plane_13"] {
        assert_eq!(
            kernel_case(name).at("s_k1_refusal").text(),
            code_text(SkewCode::ImposedDisplacementSkewSupport)
        );
    }
}

// ------------------------------------------------------------------ invariance

fn outputs(d: [f64; 3]) -> Vec<f64> {
    let load = [0.0, -1.0, 0.0];
    let stop = run([1.0; 3], load, &[], &[("stop", d)], &[]).unwrap();
    let a = stop.basis.attribute(0, stop.r).unwrap();
    let spring = run([1.0; 3], load, &[], &[], &[("s", d, 1.0)]).unwrap();
    let s = spring.basis.spring_reaction(0, &spring.u).unwrap();
    let mut out = stop.u.clone();
    out.extend(a.stop.global);
    out.push(a.stop.along);
    out.extend(spring.u.iter().copied());
    out.extend(s.global);
    out.push(s.along);
    out
}

/// The positions of the along-direction values in `outputs`.
const ALONG: [usize; 2] = [9, 19];

#[test]
fn invariance_groups() {
    let refs = references();
    let inv = refs.at("invariance");
    for group in inv.at("bitwise_identical_outputs").items() {
        let names: Vec<&str> = group.items().iter().map(|n| n.text()).collect();
        let base = outputs(policy(names[0]));
        let base_dir = SkewDirection::new(policy(names[0])).unwrap();
        for &name in &names[1..] {
            let dir = SkewDirection::new(policy(name)).unwrap();
            assert_eq!(dir.scaled().map(bits), base_dir.scaled().map(bits), "{name}: d̃");
            assert_eq!(dir.pivot(), base_dir.pivot(), "{name}: pivot");
            assert_eq!(outputs(policy(name)).iter().map(|&x| bits(x)).collect::<Vec<_>>(),
                       base.iter().map(|&x| bits(x)).collect::<Vec<_>>(), "{name}: I1 bit for bit");
        }
    }
    for group in inv.at("exact_system_identical_positive_factor").items() {
        let names: Vec<&str> = group.items().iter().map(|n| n.text()).collect();
        let base = outputs(policy(names[0]));
        let scale = base.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
        for &name in &names[1..] {
            for (k, (a, b)) in outputs(policy(name)).iter().zip(&base).enumerate() {
                close(*a, *b, scale, &format!("{name}: I2 output {k}"));
            }
        }
    }
    // I3 under P-SYM: u and R bit for bit (up to the sign of a zero), the
    // along-direction values exactly negated. A failure is reported, not
    // silenced.
    for pair in inv.at("negation").items() {
        let names: Vec<&str> = pair.items().iter().map(|n| n.text()).collect();
        let base = outputs(policy(names[0]));
        let negated = outputs(policy(names[1]));
        for (k, (a, b)) in negated.iter().zip(&base).enumerate() {
            if ALONG.contains(&k) {
                assert_eq!(bits(*a), bits(-*b), "{}: along negated exactly", names[1]);
            } else {
                assert!(*a == *b, "{}: P-SYM output {k}: {a:e} against {b:e}", names[1]);
            }
        }
    }
}

// ------------------------------------------------------------------ attribution range

#[test]
fn attribution_range_bits() {
    let refs = references();
    let mut refused = 0;
    for case in refs.at("attribution_range").items() {
        let name = case.at("name").text();
        let a = case.at("authored").items();
        let d = [a[0].b64(), a[1].b64(), a[2].b64()];
        let mut axis: Vec<usize> = case.at("held_axes").items().iter().map(|h| axis_index(h.text())).collect();
        axis.sort_unstable();
        let basis = RestraintBasis::build(
            1,
            &axis,
            vec![SkewStop { support_id: "stop".into(), node: 0, direction: SkewDirection::new(d).unwrap() }],
            vec![],
        )
        .unwrap();
        let node = basis.node_basis(0).unwrap();
        assert_eq!(i64::from(node.restricted_scale_exponent), case.at("restricted_scale_exponent").int(), "{name}");
        let w = case.at("w").items();
        for i in 0..3 {
            assert_eq!(bits(node.w[i]), bits(w[i].b64()), "{name}: w");
        }
        let rows = case.at("rows_r").items();
        let r = [rows[0].b64(), rows[1].b64(), rows[2].b64()];
        let mut numerator = ExactAccumulator::new();
        for i in (0..3).filter(|&i| node.free_axes[i]) {
            numerator.add_product(node.w[i], r[i]).unwrap();
        }
        assert_eq!(numerator.round().is_ok(), case.at("N_w_representable").boolean(), "{name}: N_w");
        let expect = case.at("expect");
        let b = expect.at("binary64_per_design_7_2");
        let result = basis.attribute_traced(0, r);
        if expect.at("code").is_null() {
            let (attribution, trace) = result.unwrap_or_else(|e| panic!("{name}: {e:?}"));
            assert_eq!(i64::from(trace.k), b.at("k").int(), "{name}: k");
            assert_eq!(bits(trace.n_prime), bits(b.at("N_prime").b64()), "{name}: N′");
            assert_eq!(bits(trace.mu_prime), bits(b.at("mu_prime").b64()), "{name}: μ′");
            assert_eq!(bits(trace.mu), bits(b.at("mu").b64()), "{name}: μ");
            assert_eq!(bits(trace.n_w), bits(case.at("n_w").b64()), "{name}: n_w");
            let components = b.at("components").items();
            for i in 0..3 {
                assert_eq!(bits(attribution.stop.global[i]), bits(components[i].b64()), "{name}: R[{i}]");
            }
            assert_eq!(bits(attribution.stop.along), bits(b.at("along_direction").b64()), "{name}: s");
            let expect_axis = b.at("axis").entries();
            assert_eq!(attribution.axis.len(), expect_axis.len(), "{name}: axis records");
            for (letter, value) in expect_axis {
                let dof = axis_index(letter);
                let got = attribution.axis.iter().find(|&&(d, _)| d == dof).unwrap().1;
                assert_eq!(bits(got), bits(value.b64()), "{name}: axis {letter}");
            }
        } else {
            assert_eq!(expect.at("code").text(), code_text(SkewCode::ReactionRange));
            assert_eq!(b.at("along_direction").text(), "outside binary64");
            let representable = expect.at("published_representable");
            assert!(!representable.at("along_direction").boolean());
            assert!(representable.at("components").items().iter().all(|c| c.boolean()));
            assert_eq!(
                result.unwrap_err(),
                BasisError::ReactionRange { node: 0, value: PublishedValue::AlongDirection },
                "{name}: refused on the along-direction value only"
            );
            refused += 1;
        }
    }
    assert_eq!(refused, 1);
}

// ------------------------------------------------------------------ mutant

#[test]
fn rounded_division_mutant_is_killed_by_the_exact_constraint_check() {
    let refs = references();
    let mutant = refs.at("mutant");
    let third = 1.0_f64 / 3.0;
    let column = mutant.at("mutant_column").items();
    assert_eq!(bits(expected(&column[0])), bits(1.0));
    assert_eq!(bits(expected(&column[1])), bits(-third));
    // The references' residual, in the authored frame: 1 − 3·fl(1/3) = 2^-54.
    let mut authored = ExactAccumulator::new();
    authored.add_product(1.0, 1.0).unwrap();
    authored.add_product(3.0, -third).unwrap();
    let per_unit = expected(mutant.at("exact_constraint_residual_per_unit_coordinate"));
    assert_eq!(bits(authored.round().unwrap()), bits(per_unit));
    assert_eq!(per_unit, pow2(-54));
    // The canonical basis: the exact check is zero for v_x = 1.
    let basis = RestraintBasis::build(
        1,
        &[3, 4, 5],
        vec![SkewStop { support_id: "stop".into(), node: 0, direction: SkewDirection::new([1.0, 3.0, 0.0]).unwrap() }],
        vec![],
    )
    .unwrap();
    let mut v = vec![0.0; 6];
    v[0] = 1.0;
    assert!(basis.constraint_residual(0, &v).unwrap().is_zero());
    // The rounded-division basis c = (1, −fl(1/3)) in place of column x: the
    // check sees 2^-55 in the w frame (w = d/2), i.e. 2^-54 per unit authored d.
    let mut mutated = basis.clone();
    let node = &mut mutated.nodes[0];
    assert_eq!(node.pivot, 1);
    node.q[0][0] = 1.0;
    node.q[1][0] = -third;
    let residual = mutated.constraint_residual(0, &v).unwrap();
    assert!(!residual.is_zero(), "the mutant is detected");
    assert_eq!(residual.round().unwrap(), pow2(-55));
}

// ------------------------------------------------------------------ transform, reduce, lift

/// Two nodes, a full symmetric integer K (every product below is exact in
/// binary64), stops at both nodes (node 1 with UZ held).
fn two_node() -> (RestraintBasis, Vec<Vec<f64>>) {
    let n = 12;
    let mut k = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            k[i][j] = if i == j { 40.0 + i as f64 } else { ((i * 7 + j * 7 + i * j) % 11) as f64 - 5.0 };
        }
    }
    let stop = |id: &str, node, v| SkewStop { support_id: id.into(), node, direction: SkewDirection::new(v).unwrap() };
    let basis = RestraintBasis::build(
        2,
        &[3, 4, 5, 8, 9, 10, 11],
        vec![stop("b", 1, [1.0, 1.0, 1.0]), stop("a", 0, [3.0, 4.0, 5.0])],
        vec![],
    )
    .unwrap();
    (basis, k)
}

fn dense_q(basis: &RestraintBasis) -> Vec<Vec<f64>> {
    let n = basis.dimension();
    let mut q: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect()).collect();
    for node in basis.nodes() {
        let b = node.node * DOF_PER_NODE;
        for i in 0..3 {
            for c in 0..3 {
                q[b + i][b + c] = node.q[i][c];
            }
        }
    }
    q
}

#[test]
fn transform_matches_the_dense_congruence_bit_for_bit() {
    let (basis, k) = two_node();
    assert!(basis.has_constraint_transform());
    assert_eq!(basis.restrained_coordinates(), &[2, 3, 4, 5, 6, 8, 9, 10, 11]);
    let q = dense_q(&basis);
    let n = 12;
    let k_prime = basis.transform_stiffness(&full(&k)).unwrap();
    let mut max_terms = 0;
    for r in 0..n {
        for c in 0..n {
            // Every product and partial sum here is a small dyadic: exact.
            let mut exact = 0.0;
            for i in 0..n {
                for j in 0..n {
                    exact += q[i][r] * k[i][j] * q[j][c];
                }
            }
            let got = k_prime.get(r, c);
            assert_eq!(got, exact, "K'[{r}][{c}]");
            assert_ne!(got.to_bits(), (-0.0_f64).to_bits(), "never −0.0");
            let terms = basis.congruence_terms(r, c);
            max_terms = max_terms.max(terms.len());
            let mut enumerated = 0.0;
            for t in terms.as_slice() {
                assert_eq!(t.left, q[t.i][r]);
                assert_eq!(t.right, q[t.j][c]);
                enumerated += t.left * k[t.i][t.j] * t.right;
            }
            assert_eq!(enumerated, exact, "one enumeration");
        }
    }
    assert_eq!(max_terms, 4, "S-K1: at most 2 entries per Q column, ≤ 4 (≤ 9) terms");
    // A node block missing from the pattern (§2.2 item 6) is refused.
    let diag: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect()).collect();
    assert_eq!(
        basis.transform_stiffness(&SparseStiffness::from_dense(&diag).unwrap()).unwrap_err(),
        BasisError::Shape("skew node block outside the stiffness pattern")
    );
}

#[test]
fn reduce_forms_the_transformed_ledger_exactly() {
    let (basis, k) = two_node();
    let k_prime = basis.transform_stiffness(&full(&k)).unwrap();
    let mut ledger = LoadLedger::new();
    ledger.push("a", 0, 3.0);
    ledger.push_product("b", 1, 0.5, 6.0);
    ledger.push("c", 7, -2.0);
    ledger.push_product("d", 6, 3.0, 0.25);
    ledger.push("e", 2, 9.0);
    let force = ledger.finish(12).unwrap();
    // Values aligned with axis_restrained [3, 4, 5, 8, 9, 10, 11]: node 1's
    // UZ (8) held at zero, its RX (9) prescribed 0.5 (admitted at a stop's node).
    let values = [0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0];
    let reduction = basis.reduce(&k_prime, &force, Some(&values[..])).unwrap();
    let q = dense_q(&basis);
    let f: Vec<f64> = (0..12).map(|i| force.get(i).unwrap()).collect();
    let mut g = vec![0.0; 12];
    g[9] = 0.5;
    assert_eq!(reduction.reduced.prescribed.iter().find(|p| p.0 == 9), Some(&(9, 0.5)));
    for pivot in [2, 6] {
        assert!(reduction.reduced.prescribed.contains(&(pivot, 0.0)), "pivot {pivot} at 0");
        assert!(reduction.reduced.free_position(pivot).is_none());
    }
    for (row, &r) in reduction.reduced.free_dofs.iter().enumerate() {
        let mut exact: f64 = (0..12).map(|i| q[i][r] * f[i]).sum();
        for (c, &gc) in g.iter().enumerate() {
            exact -= k_prime.get(r, c) * gc;
        }
        assert_eq!(reduction.reduced.force.values()[row], exact, "row {r}");
    }
    // Load terms stay physical in kind: a product enters as products.
    assert!(reduction.force.terms_for(0).all(|t| matches!(t.kind, ForceTermKind::Product(..))));
    // A product whose split has a nonzero low part: coordinate row 0 takes
    // q = 1.25 = 5/4 of physical DOF 0's a·b, exactly: 4·Σ terms − 5·a·b = 0.
    let (a, b) = (1.0_f64 / 3.0, 1.0_f64 / 7.0);
    assert_ne!(1.25_f64.mul_add(a, -(1.25 * a)), 0.0);
    let mut ledger = LoadLedger::new();
    ledger.push_product("p", 0, a, b);
    let transformed = basis.transform_force(&ledger.finish(12).unwrap()).unwrap();
    let mut check = ExactAccumulator::new();
    for term in transformed.terms_for(0) {
        for _ in 0..4 {
            term.accumulate(&mut check, false).unwrap();
        }
    }
    for _ in 0..5 {
        check.add_product(-a, b).unwrap();
    }
    assert!(check.is_zero(), "f′_0 = (5/4)·a·b exactly");
}

/// σ·(exact(Q(v + w + w₂))_dof − u_p − w_p − w₂,p), exactly.
fn lift_residual(q: &[Vec<f64>], vectors: [&[f64]; 3], lifted: &LiftedMemberBound, dof: usize, sigma: f64) -> ExactAccumulator {
    let mut sum = ExactAccumulator::new();
    for x in vectors {
        for c in 0..q.len() {
            sum.add_product(sigma * q[dof][c], x[c]).unwrap();
        }
    }
    for p in [lifted.u[dof], lifted.bound.w[dof], lifted.bound.w2[dof]] {
        sum.add(-sigma * p).unwrap();
    }
    sum
}

#[test]
fn lift_and_lifted_member_bound_enclose_the_exact_lift() {
    let (basis, _) = two_node();
    // Node 0: d̃ = (0.75, 1, 1.25), pivot z (DOF 2, held 0); node 1: d̃ =
    // (1, 1, 1) with UZ held, pivot x (DOF 6, held 0), free coordinate y.
    assert_eq!(basis.node_basis(0).unwrap().pivot, 2);
    assert_eq!(basis.node_basis(1).unwrap().pivot, 0);
    let mut v = vec![0.0; 12];
    for (dof, value) in [(0, 0.1), (1, -0.3), (7, 1.0 / 3.0)] {
        v[dof] = value;
    }
    let u = basis.lift(&v).unwrap();
    let q = dense_q(&basis);
    // Corrections live on free coordinates only (K-D5's w and w₂ are zero on
    // every restrained coordinate).
    let restrained = |c: usize| basis.restrained_coordinates().contains(&c);
    let w: Vec<f64> = (0..12).map(|i| if restrained(i) { 0.0 } else { 1e-17 * (i as f64 + 1.0) }).collect();
    let w2: Vec<f64> = (0..12).map(|i| if restrained(i) { 0.0 } else { -3e-34 * (i as f64 + 2.0) }).collect();
    let exponents: Vec<i32> = (0..12).map(|c| if restrained(c) { i32::MIN } else { -3 }).collect();
    let lifted = basis
        .lift_member_bound(&v, &MemberBoundInputs { w: w.clone(), w2: w2.clone(), exponents: exponents.clone(), lambda_sigma: 0.25, lift_radius: Vec::new() })
        .unwrap();
    // i32::MIN requires exactly-zero lift values: a correction on a held
    // coordinate at a stop's node is refused, never given a MIN exponent.
    let mut held_w = w.clone();
    held_w[8] = 1e-17;
    assert_eq!(
        basis
            .lift_member_bound(&v, &MemberBoundInputs { w: held_w, w2: w2.clone(), exponents, lambda_sigma: 0.25, lift_radius: Vec::new() })
            .unwrap_err(),
        BasisError::Shape("prescribed DOF with a nonzero lift")
    );
    assert_eq!(lifted.u, u);
    assert_eq!(lifted.bound.lambda_sigma, 0.25);
    assert_eq!(lifted.bound.lift_radius.len(), 12);
    for dof in 0..12 {
        // u_p is the lift rounded once.
        let mut exact = ExactAccumulator::new();
        basis.lift_exact(&v, dof, &mut exact).unwrap();
        assert_eq!(bits(exact.round().unwrap()), bits(u[dof]), "u_p at {dof}");
        // |exact − u_p − w_p − w₂,p| ≤ lift_radius.
        let residual = lift_residual(&q, [&v[..], &w[..], &w2[..]], &lifted, dof, 1.0);
        let sigma = if residual.signum() < 0 { -1.0 } else { 1.0 };
        let mut margin = lift_residual(&q, [&v[..], &w[..], &w2[..]], &lifted, dof, -sigma);
        margin.add(lifted.bound.lift_radius[dof]).unwrap();
        assert!(margin.signum() >= 0, "lift radius at {dof}");
        assert!(lifted.bound.lift_radius[dof] >= 0.0);
        if basis.translation(dof).is_none() {
            assert_eq!((lifted.bound.w[dof], lifted.bound.w2[dof], lifted.bound.lift_radius[dof]), (w[dof], w2[dof], 0.0));
            assert_eq!(lifted.bound.exponents[dof], if basis.restrained_coordinates().contains(&dof) { i32::MIN } else { -3 });
        }
    }
    // Node 1's physical x gets coordinate y only (|q| = 1): E = −3; its UZ: MIN.
    assert_eq!(lifted.bound.exponents[6], -3);
    assert_eq!(lifted.bound.exponents[8], i32::MIN, "UZ held");
    assert_eq!((lifted.u[8], lifted.bound.w[8], lifted.bound.lift_radius[8]), (0.0, 0.0, 0.0));
    // Node 0's physical z gets x and y (|q| = 0.75 and 1): 1.75·2^-3 → E = −2.
    assert_eq!(lifted.bound.exponents[2], -2);
    // A nonzero restrained coordinate at a stop's node is refused.
    let mut bad = v.clone();
    bad[6] = 1.0;
    assert!(matches!(basis.lift(&bad), Err(BasisError::Shape(_))));
}

#[test]
fn ceil_log2_of_dyadic_sums_is_exact() {
    assert_eq!(ceil_log2_dyadic(&[]), None);
    assert_eq!(ceil_log2_dyadic(&[(1, 0)]), Some(0));
    assert_eq!(ceil_log2_dyadic(&[(3, -2)]), Some(0)); // 0.75
    assert_eq!(ceil_log2_dyadic(&[(1, 0), (1, 0)]), Some(1)); // 2
    assert_eq!(ceil_log2_dyadic(&[(1, 0), (1, -2000)]), Some(1)); // 1 + tiny
    assert_eq!(ceil_log2_dyadic(&[(5, 3), (3, -1)]), Some(6)); // 41.5
    assert_eq!(ceil_log2_dyadic(&[(1, 1000), (1, 1000)]), Some(1001));
}

#[test]
fn split_range_guard_refuses_inexact_triples() {
    let a = (1.0 + f64::EPSILON) * pow2(-500);
    let mut sum = ExactAccumulator::new();
    // Every pairwise product is below 2^-969: no exact split exists.
    assert!(!add_triple(&mut sum, a, a, a).unwrap());
    assert!(sum.is_zero(), "nothing is rounded into the sum");
    // A pair in range is split; the sum is the exact triple 0.9375·tiny.
    let mut sum = ExactAccumulator::new();
    let tiny = (1.0 + f64::EPSILON) * pow2(-990);
    assert!(add_triple(&mut sum, 0.75, tiny, 1.25).unwrap());
    sum.add_product(-0.9375, tiny).unwrap();
    assert!(sum.is_zero(), "exact");
    // A split with a nonzero low part: (1 + 2^-30)²·3 = 3 + 3·2^-29 + 3·2^-60.
    let x = 1.0 + pow2(-30);
    let mut sum = ExactAccumulator::new();
    assert!(add_triple(&mut sum, x, x, 3.0).unwrap());
    for part in [3.0, 3.0 * pow2(-29), 3.0 * pow2(-60)] {
        sum.add(-part).unwrap();
    }
    assert!(sum.is_zero(), "the TwoProduct low part is kept");
    // The transformed ledger refuses a product it cannot split, by row.
    let (basis, _) = two_node();
    // Row 0's weights are 1.25 and −0.75; q·t is below 2^-969 for both factors.
    let mut ledger = LoadLedger::new();
    ledger.push_product("p", 0, tiny, tiny);
    let force = ledger.finish(12).unwrap();
    assert_eq!(basis.transform_force(&force).unwrap_err(), BasisError::SplitRange { dof: 0 });
    // In range, the same product transforms exactly.
    let mut ledger = LoadLedger::new();
    ledger.push_product("p", 0, a, a);
    assert!(basis.transform_force(&ledger.finish(12).unwrap()).is_ok());
}

// ------------------------------------------------------------------ ground rows and capacity

#[test]
fn ground_rows_and_identity_capacity() {
    let d = |v| SkewDirection::new(v).unwrap();
    let basis = RestraintBasis::build(
        2,
        &[3, 4, 5],
        vec![SkewStop { support_id: "stop".into(), node: 1, direction: d([3.0, 4.0, 0.0]) }],
        vec![
            SkewSpring { support_id: "s0".into(), node: 0, direction: d([0.0, 3.0, 4.0]), stiffness: 2.0 },
            SkewSpring { support_id: "z".into(), node: 0, direction: d([1.0, 1.0, 0.0]), stiffness: 0.0 },
        ],
    )
    .unwrap();
    assert_eq!(
        basis.ground_rows(),
        vec![
            GroundRow::Axis(3),
            GroundRow::Axis(4),
            GroundRow::Axis(5),
            GroundRow::Direction { node: 1, d: [0.75, 1.0, 0.0] },
            GroundRow::Direction { node: 0, d: [0.0, 0.75, 1.0] },
        ]
    );
    // Springs only: no constraint transform and no node storage at all.
    let springs_only = RestraintBasis::build(
        1,
        &[3, 4, 5],
        vec![],
        vec![SkewSpring { support_id: "s".into(), node: 0, direction: d([3.0, 4.0, 0.0]), stiffness: 1.0 }],
    )
    .unwrap();
    assert!(!springs_only.has_constraint_transform());
    assert_eq!(springs_only.nodes.capacity(), 0);
    assert_eq!(springs_only.restrained_coordinates(), &[3, 4, 5]);
    // The spring block is symmetric bit for bit, from d̃ and n.
    let block = springs_only.spring_block(0);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(bits(block[i][j]), bits(block[j][i]));
        }
    }
    assert_eq!(block[0][0], 1.0 * ((0.75 * 0.75) / 1.5625));
    // Invalid inputs are named, never admitted.
    assert_eq!(
        RestraintBasis::build(1, &[4, 3], vec![], vec![]).unwrap_err(),
        vec![BasisFinding::InvalidInput("axis restrained DOFs")]
    );
}

// ------------------------------------------------------------------ §10.5 certified attribution

/// The kernel node as K-D5's certified source: the tip's axis springs (the
/// translations' diag(tip)); the skewed spring blocks come from the basis.
fn certified_inputs(tip: [f64; 3], load: [f64; 3]) -> (crate::structural::FormationSource<'static>, AssembledForce) {
    let source = crate::structural::FormationSource {
        node_count: 1,
        springs: (0..3).filter(|&i| tip[i] != 0.0).map(|i| (i, tip[i])).collect(),
        ..Default::default()
    };
    let mut ledger = LoadLedger::new();
    for (dof, &f) in load.iter().enumerate() {
        if f != 0.0 {
            ledger.push("load", dof, f);
        }
    }
    (source, ledger.finish(6).unwrap())
}

/// The certified displacement as K-D5 hands it over: the coordinate solution v
/// with an empty correction and a valid remainder, lifted by §10.4
/// (`lift_member_bound`: u_p = fl(Qv) and the lift radius enclosing the exact
/// lift). The kernel nodes are well conditioned (K = diag(tip) + blocks, tip
/// entries 1 or 4), so |v* − v| ≤ 2^-40 per free coordinate is a generous valid
/// remainder (λσ = 1); the attribution's consistency holds only within a valid
/// enclosure of the intended solution, never for the rounded v alone.
fn lifted(run: &Run) -> (Vec<f64>, crate::structural::MemberBoundInputs) {
    let held = run.basis.restrained_coordinates();
    let coordinate = crate::structural::MemberBoundInputs {
        w: vec![0.0; 6],
        w2: vec![0.0; 6],
        exponents: (0..6).map(|c| if held.contains(&c) { i32::MIN } else { -40 }).collect(),
        lambda_sigma: 1.0,
        lift_radius: Vec::new(),
    };
    let lifted = run.basis.lift_member_bound(&run.v, &coordinate).unwrap();
    assert_eq!(lifted.u, run.u, "u_p = fl(Qv)");
    (lifted.u, lifted.bound)
}

/// §10.5 on every S-K1 kernel case: the certified stop, axis and spring rows
/// match the references (within §13's 1e-12 proxy) and enclose the binary64
/// attribution's value at the same u within one rounding.
#[test]
fn certified_attribution_matches_the_kernel_cases() {
    use crate::structural::{certify_node_attribution, certify_skew_spring_reaction};
    let mut checked = 0;
    for input in s_k1_inputs() {
        let name = input.name;
        let case = kernel_case(name);
        let expect = case.at("expect");
        let (tip, load) = tip_and_load(&case);
        let held: Vec<usize> = input.held.iter().map(|&(_, a)| a).collect();
        let run = run(tip, load, &held, &input.stops, &input.springs).unwrap();
        let scale = case_scale(expect);
        let (source, force) = certified_inputs(tip, load);
        let (_, bound) = lifted(&run);
        let attribution = run.basis.node_basis(0).map(|_| {
            certify_node_attribution(&source, &force, &run.basis, 0, &run.u, &bound).unwrap_or_else(|e| panic!("{name}: {e:?}"))
        });
        let binary64 = run.basis.node_basis(0).map(|_| run.basis.attribute(0, run.r).unwrap());
        let within = |certified: &crate::structural::CertifiedRow, b64: f64, what: &str| {
            let gap = (certified.published - b64).abs();
            assert!(gap <= certified.bound() + 4.0 * ulp(b64.abs().max(scale)), "{name}: {what}: {} against {b64}", certified.published);
        };
        for (record, value) in expect.at("records").entries() {
            match value.at("kind").text() {
                "skew_stop" => {
                    let (a, b) = (attribution.as_ref().unwrap(), binary64.as_ref().unwrap());
                    let g = triple_of(value.at("global_force"));
                    for i in 0..3 {
                        close(a.stop[i].published, g[i], scale, &format!("{name}: {record} R[{i}]"));
                        within(&a.stop[i], b.stop.global[i], &format!("{record} R[{i}]"));
                    }
                    close(a.along.published, value.at("along_direction").b64(), scale, &format!("{name}: {record} along"));
                    within(&a.along, b.stop.along, "along");
                }
                "rigid_axis_rows" => {
                    let a = attribution.as_ref().unwrap();
                    let slot = input.held.iter().find(|&&(id, _)| id == record.as_str()).unwrap().1;
                    let row = &a.axis.iter().find(|(dof, _)| *dof == slot).unwrap().1;
                    close(row.published, expected(&value.at("components").items()[0]), scale, &format!("{name}: {record}"));
                }
                "skew_spring" => {
                    let s = input.springs.iter().position(|&(id, _, _)| id == record.as_str()).unwrap();
                    let certified = certify_skew_spring_reaction(&run.basis.springs()[s], &run.u, &bound).unwrap();
                    let b = run.basis.spring_reaction(s, &run.u).unwrap();
                    let g = triple_of(value.at("global_force"));
                    for i in 0..3 {
                        close(certified.global[i].published, g[i], scale, &format!("{name}: {record} R[{i}]"));
                        within(&certified.global[i], b.global[i], &format!("{record} R[{i}]"));
                    }
                    close(certified.along.published, value.at("along_direction").b64(), scale, &format!("{name}: {record} along"));
                    within(&certified.along, b.along, "spring along");
                }
                other => panic!("{name}: kind {other}"),
            }
        }
        checked += 1;
    }
    assert_eq!(checked, 12);
}

/// Review P1-2 (`regressions.reaction_rows_include_skew_springs`): the physical rows
/// at RS_stop_and_spring's node include the skewed spring block. Omitting it
/// leaves r_z = 54/553 P against a stop with no z component, and the attribution's
/// consistency check refuses the record by name.
#[test]
fn certified_rows_include_skew_spring_blocks() {
    use crate::structural::{certify_node_attribution, certify_support_reaction, certify_support_reaction_with_basis};
    let case = kernel_case("RS_stop_and_spring");
    let (tip, load) = tip_and_load(&case);
    let input = s_k1_inputs().into_iter().find(|i| i.name == "RS_stop_and_spring").unwrap();
    let run = run(tip, load, &[], &input.stops, &input.springs).unwrap();
    let regression = references().at("regressions").at("reaction_rows_include_skew_springs").clone();
    assert_eq!(regression.at("case").text(), "RS_stop_and_spring");
    let (source, force) = certified_inputs(tip, load);
    let (_, bound) = lifted(&run);
    let with = triple_of(regression.at("rows_with_block"));
    let omitted = triple_of(regression.at("rows_block_omitted"));
    for i in 0..3 {
        let row = certify_support_reaction_with_basis(&source, &force, Some(&run.basis), i, &run.u, &bound).unwrap();
        close(row.published, with[i], 1.0, &format!("with the block r[{i}]"));
        // With no basis the row is today's (no skewed spring block).
        let plain = certify_support_reaction(&source, &force, i, &run.u, &bound).unwrap();
        close(plain.published, omitted[i], 1.0, &format!("block omitted r[{i}]"));
        assert_eq!(
            plain.published.to_bits(),
            certify_support_reaction_with_basis(&source, &force, None, i, &run.u, &bound).unwrap().published.to_bits()
        );
    }
    // The block-omission mutant: the same u, a basis without the spring.
    let stop_only = RestraintBasis::build(
        1,
        &[3, 4, 5],
        vec![SkewStop { support_id: "stop".into(), node: 0, direction: SkewDirection::new([3.0, 4.0, 0.0]).unwrap() }],
        vec![],
    )
    .unwrap();
    let refused = certify_node_attribution(&source, &force, &stop_only, 0, &run.u, &bound).unwrap_err();
    assert!(format!("{refused:?}").contains("restraint_basis: reaction not along the constraint"), "{refused:?}");
    assert!(certify_node_attribution(&source, &force, &run.basis, 0, &run.u, &bound).is_ok());
}

/// Review P1-3 (`regressions.spring_along_direction`): both outputs branch from κ;
/// S1_spring_isotropic's along-direction is +2/5 P, not the chained 32/125 P.
#[test]
fn certified_skew_spring_branches_from_kappa() {
    use crate::structural::certify_skew_spring_reaction;
    for record in references().at("regressions").at("spring_along_direction").items() {
        let name = record.at("case").text();
        let input = s_k1_inputs().into_iter().find(|i| i.name == name).unwrap();
        let (tip, load) = tip_and_load(&kernel_case(name));
        let run = run(tip, load, &[], &input.stops, &input.springs).unwrap();
        let (_, bound) = lifted(&run);
        let s = input.springs.iter().position(|&(id, _, _)| id == record.at("spring").text()).unwrap();
        let certified = certify_skew_spring_reaction(&run.basis.springs()[s], &run.u, &bound).unwrap();
        let along = record.at("along_minus_kt_over_norm").b64();
        close(certified.along.published, along, 1.0, &format!("{name}: along"));
        let rejected = record.at("rejected_chain_minus_kt_over_n_over_norm").b64();
        assert!((certified.along.published - rejected).abs() > 1e-3, "{name}: not the chained division");
        let components = triple_of(record.at("components_minus_kt_d_over_n"));
        for i in 0..3 {
            close(certified.global[i].published, components[i], 1.0, &format!("{name}: R[{i}]"));
        }
    }
}

// ------------------------------------------------------------------ Astra B1

/// Astra B1: an intermediate of the spring recovery (t = d̃ᵀu, κ = k·t) may leave binary64
/// while every published value fits; the reaction is published, and only a published value
/// outside binary64 is refused. d = (1, 1, 0) is canonical (n = 2, ν = RN(√2)).
#[test]
fn spring_recovery_publishes_finite_values_past_an_overflowing_intermediate() {
    let spring = |k: f64| {
        RestraintBasis::build(1, &[3, 4, 5], Vec::new(), vec![SkewSpring {
            support_id: "s".into(),
            node: 0,
            direction: SkewDirection::new([1.0, 1.0, 0.0]).unwrap(),
            stiffness: k,
        }])
        .unwrap()
    };
    let max = f64::MAX;
    // κ = 2^1023·2 overflows; the components −κ/2 = −2^1023 and −κ/ν fit.
    let basis = spring(2f64.powi(1023));
    let nu = basis.springs()[0].direction.scaled_norm();
    let r = basis.spring_reaction(0, &[1.0, 1.0, 0.0, 0.0, 0.0, 0.0]).expect("finite published values");
    assert_eq!(r.global.map(f64::to_bits), [-(2f64.powi(1023)), -(2f64.powi(1023)), -0.0].map(f64::to_bits));
    // fl(2^1024/ν) = 2·fl(2^1023/ν) (a power of two moves only the exponent).
    assert_eq!(r.along.to_bits(), (-(2f64.powi(1023) / nu) * 2.0).to_bits());
    // t = 2·MAX overflows; with k = 2^-4, κ = MAX/8 and every published value fits.
    let basis = spring(2f64.powi(-4));
    let r = basis.spring_reaction(0, &[max, max, 0.0, 0.0, 0.0, 0.0]).expect("finite published values");
    assert_eq!(r.global.map(f64::to_bits), [-(max / 16.0), -(max / 16.0), -0.0].map(f64::to_bits));
    assert_eq!(r.along.to_bits(), (-((max / 8.0) / nu)).to_bits());
    // A published component outside binary64 is still refused, by name.
    let basis = spring(2.0);
    assert!(matches!(
        basis.spring_reaction(0, &[max, max, 0.0, 0.0, 0.0, 0.0]),
        Err(BasisError::ReactionRange { node: 0, value: PublishedValue::SpringComponent { axis: 0 } })
    ));
    // The ordinary path is unchanged: a finite κ gives today's bits.
    let basis = spring(3.0);
    let r = basis.spring_reaction(0, &[0.5, 0.25, 0.0, 0.0, 0.0, 0.0]).unwrap();
    let kappa = 3.0 * 0.75;
    assert_eq!(r.global.map(f64::to_bits), [-(kappa / 2.0), -(kappa / 2.0), -((kappa / 2.0) * 0.0)].map(f64::to_bits));
    assert_eq!(r.along.to_bits(), (-(kappa / nu)).to_bits());
}

/// Astra B1′: the wide path's published component is the exact final product rounded once,
/// subnormals included, never rounded at 53 bits and then again onto the subnormal grid.
/// The reviewer's admitted counterexample: d = (1, 1, 0x1.2000000000001p-1022),
/// u = (MAX, MAX, 0), k = 2^-1074. t = 2·MAX overflows, κ = 2^-1074·2·MAX and κ/n are exact
/// (n = fl(2 + d_z²) = 2), and Rz = −RN(κ/n · d_z): the exact product is 4.5·2^-1074 plus
/// about 2^-53 of it, so its single rounding is −5·2^-1074 (the double rounding gave −4).
#[test]
fn the_wide_spring_path_rounds_a_subnormal_component_once() {
    use crate::exact_sum::ExactAccumulator;
    let max = f64::MAX;
    let dz = f64::from_bits(0x0012_0000_0000_0001); // 0x1.2000000000001p-1022
    assert_eq!(dz, (1.125 + 2f64.powi(-52)) * 2f64.powi(-1022));
    let tiny = f64::from_bits(1);
    let basis = RestraintBasis::build(1, &[3, 4, 5], Vec::new(), vec![SkewSpring {
        support_id: "s".into(),
        node: 0,
        direction: SkewDirection::new([1.0, 1.0, dz]).unwrap(),
        stiffness: tiny,
    }])
    .unwrap();
    assert_eq!(basis.springs()[0].direction.scaled_norm_squared(), 2.0);
    let r = basis.spring_reaction(0, &[max, max, 0.0, 0.0, 0.0, 0.0]).expect("finite published values");
    // The exact reference: κ/n = MAX·2^-1074 (exact: a power-of-two scaling of MAX), and
    // Rz = −RN(κ/n · d_z), one exact product rounded once.
    let per_n = max * 2f64.powi(-1000) * 2f64.powi(-74);
    let mut exact = ExactAccumulator::new();
    exact.add_product(per_n, dz).unwrap();
    let expect_z = -exact.round().unwrap();
    assert_eq!(expect_z.to_bits(), (-5.0 * tiny).to_bits());
    assert_eq!(r.global[2].to_bits(), expect_z.to_bits(), "Rz {:e} against {:e}", r.global[2], expect_z);
    assert_eq!(r.global[0].to_bits(), (-per_n).to_bits());
    assert_eq!(r.global[1].to_bits(), (-per_n).to_bits());
    // The along value −κ/ν (normal here) is one rounding of the exact quotient.
    let kappa = 2.0 * per_n;
    assert_eq!(r.along.to_bits(), (-(kappa / basis.springs()[0].direction.scaled_norm())).to_bits());
}

/// Astra B1′: the once-rounded quotient onto the subnormal grid (`publish_quotient`), checked
/// against exact residuals: for κ/ν landing in the subnormal range the published value is the
/// grid point nearest the exact quotient (ties to even), never a 53-bit rounding then a second.
#[test]
fn the_wide_quotient_rounds_once_onto_the_subnormal_grid() {
    let nu = std::f64::consts::SQRT_2;
    for (m, e) in [(1.5, -1070), (1.0 + 2f64.powi(-52), -1073), (1.9, -1060), (1.25, -1075)] {
        let w = WideBinary64 { m, e };
        let got = w.publish_quotient(nu).unwrap();
        // Exact check: |m·2^e − got·ν| ≤ |m·2^e − c·ν| for the grid neighbours c of got.
        let a = scale_pow2(m, e + 1074);
        let n = scale_pow2(got, 1074);
        let dist = |c: f64| {
            let mut r = crate::exact_sum::ExactAccumulator::new();
            r.add(a).unwrap();
            r.add_product(-c, nu).unwrap();
            r
        };
        for c in [n - 1.0, n + 1.0] {
            let (rc, rn) = (dist(c), dist(n));
            let mut diff = crate::exact_sum::ExactAccumulator::new();
            diff.add(f64::from(rn.signum()) * a).unwrap();
            diff.add_product(-f64::from(rn.signum()) * n, nu).unwrap();
            diff.add(-f64::from(rc.signum()) * a).unwrap();
            diff.add_product(f64::from(rc.signum()) * c, nu).unwrap();
            assert!(diff.signum() <= 0, "({m}, {e}): {got:e} is not the nearest grid point");
        }
    }
}
