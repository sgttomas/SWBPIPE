//! The committed spectrum result documents (DEL-04-10 REQ-006, AC-004) and the `spectrum-1`
//! semantic table, under `fixtures/results/`.
//!
//! Each document here is constructed from a frozen reference's published signed contributions,
//! not produced by a spectrum solve, and says so in its `origin`, listing what was constructed
//! rather than taken from the reference. Its combined magnitudes are formed from its contributions
//! by the product's combinations; this writer copies no reference combined value into a document. Each
//! document is committed beside the model document stand-in and the spectrum request it binds.
//!
//! `write_spectrum_documents` (ignored) writes them; `documents_are_the_projection_of_their_inputs`
//! requires the committed bytes. After a deliberate change:
//!
//! ```sh
//! <cargo wrapper> test --manifest-path core/analysis/response_spectrum/Cargo.toml --test spectrum_documents write_spectrum_documents -- --ignored
//! ```
mod support;

use open_pipe_stress_response_spectrum::document::{
    spectrum_doc_sha256, spectrum_result_document, spectrum_semantic_table, ArcMember, Census, CompliantSupport,
    Connector, DocumentInputs, Extraction, FormationRecord, ModalResultBinding, ModalStatement, Origin, RigidConstraint,
    SignedContributions, StraightMember, TieRod, Triad,
};
use open_pipe_stress_response_spectrum::{
    combine, ClusterIndex, CombinationRequest, ConnectorComponent, Direction, DirectionalRule, End, ForceComponent, Frame,
    MemberFamily, MissingMass, ModalContribution, ModalMode, ModalRule, ModeIndex, Quantity, ReactionComponent,
    RigidContribution, RigidRule,
};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use support::{expected, TOLERANCE};

const C3: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c3_cantilever_3d.json");
const C2: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c2_ground_spring.json");
const C7: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c7_repeated_pair.json");
const C8: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c8_pipe_x.json");
const C9: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c9_pipe_diagonal.json");
const W2_INPUTS: &str = include_str!("../../../../validation/references/f2_spectrum/recovery_w2/w2_inputs.json");
const W2_OBLIQUE: &str = include_str!("../../../../validation/references/f2_spectrum/recovery_w2/w2_v_arc_oblique.json");
const W2_JOINT_R: &str = include_str!("../../../../validation/references/f2_spectrum/recovery_w2/w2_v_joint_r.json");

const DISCRETE: &str = "validation/references/f2_spectrum/discrete";
/// δ_c = 2^-20, as both packages state it (discrete README, "Modal basis"; recovery_w2
/// DERIVATION.md).
const CLUSTER_TOLERANCE: f64 = 1.0 / 1_048_576.0;
const RECOVERY_W2: &str = "validation/references/f2_spectrum/recovery_w2";

const SEMANTIC_TABLE: &str = include_str!("../../../../fixtures/results/semantic_contract_spectrum_1.json");
/// (name, document, model stand-in, request), read at compile time.
const COMMITTED: [(&str, &str, &str, &str); 10] = [
    ("c9_y_two_frames", include_str!("../../../../fixtures/results/spectrum_c9_y_two_frames.json"),
        include_str!("../../../../fixtures/results/spectrum_c9_y_two_frames.model.json"),
        include_str!("../../../../fixtures/results/spectrum_c9_y_two_frames.request.json")),
    ("c7_cancelling_cluster", include_str!("../../../../fixtures/results/spectrum_c7_cancelling_cluster.json"),
        include_str!("../../../../fixtures/results/spectrum_c7_cancelling_cluster.model.json"),
        include_str!("../../../../fixtures/results/spectrum_c7_cancelling_cluster.request.json")),
    ("w2_arc_oblique", include_str!("../../../../fixtures/results/spectrum_w2_arc_oblique.json"),
        include_str!("../../../../fixtures/results/spectrum_w2_arc_oblique.model.json"),
        include_str!("../../../../fixtures/results/spectrum_w2_arc_oblique.request.json")),
    ("w2_joint_rods", include_str!("../../../../fixtures/results/spectrum_w2_joint_rods.json"),
        include_str!("../../../../fixtures/results/spectrum_w2_joint_rods.model.json"),
        include_str!("../../../../fixtures/results/spectrum_w2_joint_rods.request.json")),
    ("c8_shared_constraint", include_str!("../../../../fixtures/results/spectrum_c8_shared_constraint.json"),
        include_str!("../../../../fixtures/results/spectrum_c8_shared_constraint.model.json"),
        include_str!("../../../../fixtures/results/spectrum_c8_shared_constraint.request.json")),
    ("c2_separator_identities", include_str!("../../../../fixtures/results/spectrum_c2_separator_identities.json"),
        include_str!("../../../../fixtures/results/spectrum_c2_separator_identities.model.json"),
        include_str!("../../../../fixtures/results/spectrum_c2_separator_identities.request.json")),
    ("refused_arc_member_axes", include_str!("../../../../fixtures/results/spectrum_refused_arc_member_axes.json"),
        include_str!("../../../../fixtures/results/spectrum_refused_arc_member_axes.model.json"),
        include_str!("../../../../fixtures/results/spectrum_refused_arc_member_axes.request.json")),
    ("c3_grouping_100_40_40", include_str!("../../../../fixtures/results/spectrum_c3_grouping_100_40_40.json"),
        include_str!("../../../../fixtures/results/spectrum_c3_grouping_100_40_40.model.json"),
        include_str!("../../../../fixtures/results/spectrum_c3_grouping_100_40_40.request.json")),
    ("c9_chain_ten_percent", include_str!("../../../../fixtures/results/spectrum_c9_chain_ten_percent.json"),
        include_str!("../../../../fixtures/results/spectrum_c9_chain_ten_percent.model.json"),
        include_str!("../../../../fixtures/results/spectrum_c9_chain_ten_percent.request.json")),
    ("c8_absolute_sums", include_str!("../../../../fixtures/results/spectrum_c8_absolute_sums.json"),
        include_str!("../../../../fixtures/results/spectrum_c8_absolute_sums.model.json"),
        include_str!("../../../../fixtures/results/spectrum_c8_absolute_sums.request.json")),
];

fn fixture_text(value: &Value) -> String {
    let mut text = serde_json::to_string_pretty(value).expect("a document serializes");
    text.push('\n');
    text
}

// ---------------------------------------------------------------------------------------------
// Exact decimal arithmetic, for the one constructed rotation (V-ARC-OBL's oblique straights).

/// A signed decimal m·10^e with m in base 10^9, little-endian.
#[derive(Clone, Debug)]
struct Dec {
    negative: bool,
    limbs: Vec<u64>,
    exponent: i64,
}

const BASE: u64 = 1_000_000_000;

impl Dec {
    /// Parses a reference decimal (`"0"`, `"-1.25"`, `"3.1e-7"`) exactly.
    fn parse(text: &str) -> Dec {
        let (negative, rest) = match text.strip_prefix('-') {
            Some(r) => (true, r),
            None => (false, text.strip_prefix('+').unwrap_or(text)),
        };
        let (mantissa, exp) = match rest.find(['e', 'E']) {
            Some(i) => (&rest[..i], rest[i + 1..].parse::<i64>().expect("an exponent")),
            None => (rest, 0),
        };
        let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let digits: String = format!("{int}{frac}");
        assert!(digits.bytes().all(|b| b.is_ascii_digit()), "a decimal: {text}");
        let mut d = Dec { negative, limbs: vec![0], exponent: exp - frac.len() as i64 };
        for b in digits.bytes() {
            d.mul_small(10);
            d.add_small(u64::from(b - b'0'));
        }
        d
    }

    fn mul_small(&mut self, k: u64) {
        let mut carry = 0u64;
        for l in &mut self.limbs {
            let v = *l * k + carry;
            *l = v % BASE;
            carry = v / BASE;
        }
        while carry > 0 {
            self.limbs.push(carry % BASE);
            carry /= BASE;
        }
    }

    fn add_small(&mut self, k: u64) {
        let mut carry = k;
        for l in &mut self.limbs {
            if carry == 0 {
                break;
            }
            let v = *l + carry;
            *l = v % BASE;
            carry = v / BASE;
        }
        if carry > 0 {
            self.limbs.push(carry);
        }
    }

    fn scale_to(&mut self, exponent: i64) {
        assert!(exponent <= self.exponent);
        for _ in exponent..self.exponent {
            self.mul_small(10);
        }
        self.exponent = exponent;
    }

    fn trim(&mut self) {
        while self.limbs.len() > 1 && *self.limbs.last().unwrap() == 0 {
            self.limbs.pop();
        }
    }

    fn cmp_magnitude(a: &[u64], b: &[u64]) -> std::cmp::Ordering {
        a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }

    /// The exact sum of signed decimals.
    fn sum(terms: Vec<Dec>) -> Dec {
        let e = terms.iter().map(|t| t.exponent).min().unwrap_or(0);
        let mut acc = Dec { negative: false, limbs: vec![0], exponent: e };
        for mut t in terms {
            t.scale_to(e);
            t.trim();
            acc.trim();
            if t.negative == acc.negative {
                let n = acc.limbs.len().max(t.limbs.len());
                acc.limbs.resize(n, 0);
                let mut carry = 0;
                for i in 0..n {
                    let v = acc.limbs[i] + t.limbs.get(i).copied().unwrap_or(0) + carry;
                    acc.limbs[i] = v % BASE;
                    carry = v / BASE;
                }
                if carry > 0 {
                    acc.limbs.push(carry);
                }
            } else {
                let (big, small, negative) = if Dec::cmp_magnitude(&acc.limbs, &t.limbs) != std::cmp::Ordering::Less {
                    (acc.limbs.clone(), t.limbs.clone(), acc.negative)
                } else {
                    (t.limbs.clone(), acc.limbs.clone(), t.negative)
                };
                let mut out = big;
                let mut borrow = 0i64;
                for i in 0..out.len() {
                    let mut v = out[i] as i64 - small.get(i).copied().unwrap_or(0) as i64 - borrow;
                    borrow = 0;
                    if v < 0 {
                        v += BASE as i64;
                        borrow = 1;
                    }
                    out[i] = v as u64;
                }
                acc.limbs = out;
                acc.negative = negative;
            }
        }
        acc.trim();
        acc
    }

    /// The exact product of two decimals.
    fn mul(&self, other: &Dec) -> Dec {
        let mut out = vec![0u64; self.limbs.len() + other.limbs.len()];
        for (i, a) in self.limbs.iter().enumerate() {
            let mut carry = 0u64;
            for (j, b) in other.limbs.iter().enumerate() {
                let v = out[i + j] + a * b + carry;
                out[i + j] = v % BASE;
                carry = v / BASE;
            }
            let mut k = i + other.limbs.len();
            while carry > 0 {
                let v = out[k] + carry;
                out[k] = v % BASE;
                carry = v / BASE;
                k += 1;
            }
        }
        let mut d = Dec { negative: self.negative != other.negative, limbs: out, exponent: self.exponent + other.exponent };
        d.trim();
        d
    }

    /// The nearest binary64 (Rust's decimal parsing is correctly rounded).
    fn to_f64(&self) -> f64 {
        let mut digits = String::new();
        for (i, l) in self.limbs.iter().rev().enumerate() {
            if i == 0 {
                digits.push_str(&l.to_string());
            } else {
                digits.push_str(&format!("{l:09}"));
            }
        }
        let v: f64 = format!("{}{digits}e{}", if self.negative { "-" } else { "" }, self.exponent).parse().unwrap();
        if v == 0.0 { 0.0 } else { v }
    }
}

/// Σ (p_k / 25)·d_k exactly, rounded once to the nearest binary64: one row of an exact rational
/// frame applied to three published decimals.
fn rotate_row(row: [i64; 3], components: [&str; 3]) -> f64 {
    let terms = row
        .iter()
        .zip(components)
        .filter(|(p, _)| **p != 0)
        .map(|(p, text)| {
            let mut d = Dec::parse(text);
            // p/25 = 4p/100.
            d.mul_small(4 * p.unsigned_abs());
            d.exponent -= 2;
            if *p < 0 {
                d.negative = !d.negative;
            }
            d
        })
        .collect();
    Dec::sum(terms).to_f64()
}

// ---------------------------------------------------------------------------------------------
// Reading the references.

fn cases(text: &str) -> Vec<Value> {
    serde_json::from_str::<Value>(text).unwrap()["cases"].as_array().unwrap().clone()
}

/// A published real: a pair `[decimal, binary64]`, or the bare string `0` (a value below 1e-50 of
/// its block's scale, carried as zero), or a plain number.
fn real(v: &Value) -> f64 {
    match v {
        Value::String(s) if s == "0" => 0.0,
        Value::Array(pair) => pair[1].as_f64().unwrap_or_else(|| panic!("binary64 of {v}")),
        Value::Number(n) => n.as_f64().unwrap(),
        other => panic!("not a published real: {other}"),
    }
}

/// The decimal of a published real (`0` for the bare zero).
fn decimal(v: &Value) -> &str {
    match v {
        Value::String(s) if s == "0" => "0",
        Value::Array(pair) => pair[0].as_str().unwrap(),
        other => panic!("not a published real: {other}"),
    }
}

fn decimal_f64(s: &str) -> f64 {
    s.parse().unwrap()
}

const FORCES: [ForceComponent; 6] =
    [ForceComponent::Fx, ForceComponent::Fy, ForceComponent::Fz, ForceComponent::Mx, ForceComponent::My, ForceComponent::Mz];
const REACTION_KEYS: [(ReactionComponent, &str); 6] = [
    (ReactionComponent::Ux, "Fx"),
    (ReactionComponent::Uy, "Fy"),
    (ReactionComponent::Uz, "Fz"),
    (ReactionComponent::Rx, "Mx"),
    (ReactionComponent::Ry, "My"),
    (ReactionComponent::Rz, "Mz"),
];
const DOF_COMPONENTS: [(&str, ReactionComponent); 6] = [
    ("ux", ReactionComponent::Ux),
    ("uy", ReactionComponent::Uy),
    ("uz", ReactionComponent::Uz),
    ("rx", ReactionComponent::Rx),
    ("ry", ReactionComponent::Ry),
    ("rz", ReactionComponent::Rz),
];

fn reaction_key(c: ReactionComponent) -> &'static str {
    REACTION_KEYS.iter().find(|(k, _)| *k == c).unwrap().1
}

fn dof_component(dof: &str) -> ReactionComponent {
    DOF_COMPONENTS.iter().find(|(d, _)| *d == dof).unwrap_or_else(|| panic!("dof {dof}")).1
}

/// The force or moment triple of an end action component (0 for F, 3 for M) and its index.
fn triple(c: ForceComponent) -> (usize, usize) {
    let i = FORCES.iter().position(|f| *f == c).unwrap();
    (i / 3 * 3, i % 3)
}

/// The triad of an axis-aligned straight member from end i to end j with reference vector `yref`
/// (REQ-006: x from i to j, y the part of yref normal to x, normalized, z = x × y), exact here
/// because every operand is a signed unit or zero.
fn axis_aligned_triad(from: [f64; 3], to: [f64; 3], yref: [f64; 3]) -> Triad {
    let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    let x = [d[0] / len, d[1] / len, d[2] / len];
    let dot = yref[0] * x[0] + yref[1] * x[1] + yref[2] * x[2];
    let yr = [yref[0] - dot * x[0], yref[1] - dot * x[1], yref[2] - dot * x[2]];
    let ny = (yr[0] * yr[0] + yr[1] * yr[1] + yr[2] * yr[2]).sqrt();
    let y = [yr[0] / ny, yr[1] / ny, yr[2] / ny];
    let z = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]];
    let t = [x, y, z];
    for row in &t {
        assert_eq!(row.iter().filter(|v| **v != 0.0).count(), 1, "an axis-aligned member's frame is a signed permutation");
        assert!(row.iter().all(|v| *v == 0.0 || v.abs() == 1.0));
    }
    t
}

/// A member-axes component by the signed permutation `t` of an axis-aligned frame: exact.
fn permuted(t: &Triad, global: [f64; 3], axis: usize) -> f64 {
    let row = t[axis];
    let k = row.iter().position(|v| *v != 0.0).unwrap();
    row[k] * global[k]
}

/// A model-document spectrum record from a discrete case's spectrum.
fn discrete_spectrum(s: &Value) -> Value {
    let period = s["abscissa"].as_str().unwrap().starts_with("period");
    let scale = |v: &Value| match v.as_str().unwrap() {
        "linear" => "linear",
        "log" | "logarithmic" => "log",
        other => panic!("interpolation {other}"),
    };
    json!({
        "id": s["id"],
        "abscissa": if period { "period" } else { "frequency" },
        "ordinate": "acceleration",
        "units": {"abscissa": if period { "s" } else { "Hz" }, "ordinate": "m/s^2"},
        "damping_ratio": s["damping_ratio"],
        "interpolation": {"abscissa": scale(&s["interpolation"]["abscissa"]), "ordinate": scale(&s["interpolation"]["ordinate"])},
        "points": s["points"],
    })
}

/// A model-document spectrum record from W2's inputs.
fn w2_spectrum(id: &str, s: &Value, zeta: f64) -> Value {
    let period = s["abscissa"].as_str().unwrap().starts_with("period");
    let points: Vec<Value> = s["points"].as_array().unwrap().iter()
        .map(|p| json!([decimal_f64(p[0].as_str().unwrap()), decimal_f64(p[1].as_str().unwrap())])).collect();
    json!({
        "id": id,
        "abscissa": if period { "period" } else { "frequency" },
        "ordinate": "acceleration",
        "units": {"abscissa": if period { "s" } else { "Hz" }, "ordinate": "m/s^2"},
        "damping_ratio": zeta,
        "interpolation": {"abscissa": s["x_interp"], "ordinate": s["y_interp"]},
        "points": points,
    })
}

fn request_value(directions: &[(Direction, &str, f64)], modal: ModalRule, directional: DirectionalRule, missing: MissingMass) -> Value {
    let missing_mass = match missing {
        MissingMass::NotApplied => json!({"applied": false}),
        MissingMass::Applied { rigid_frequency_hz, rule } => json!({"applied": true,
            "rigid_frequency": {"value": rigid_frequency_hz, "unit": "Hz"}, "rigid_combination": rule.as_str()}),
    };
    json!({
        "document_kind": "openpipestress.spectrum_request",
        "schema_version": "0.1.0",
        "directions": directions.iter().map(|(d, s, f)| json!({"direction": d.as_str(), "spectrum": s, "scale_factor": f})).collect::<Vec<_>>(),
        "modal_combination": modal.as_str(),
        "directional_combination": directional.as_str(),
        "missing_mass": missing_mass,
    })
}

/// A model document stand-in: the reference's entities in the model document's shape as far as
/// this unit needs them (nodes, pipe segments, components, supports) and the spectra, exactly. It
/// is a byte source for the model binding and the spectra; no product code reads it as a model.
fn model_value(project: &str, provenance: &str, nodes: &[(String, [f64; 3])], segments: Vec<Value>, components: Vec<Value>,
    supports: Vec<Value>, spectra: Vec<Value>) -> Value {
    json!({
        "document_kind": "openpipestress.product_preview.model",
        "schema_version": "0.9.0",
        "project": {"id": project, "units": {"angle": "rad", "force": "N", "length": "m", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "materials": [{"id": "steel", "elastic_modulus": {"unit": "Pa", "value": 200000000000.0}, "shear_modulus": {"unit": "Pa", "value": 78125000000.0}, "provenance": provenance}],
        "nodes": nodes.iter().map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": provenance})).collect::<Vec<_>>(),
        "pipe_segments": segments,
        "components": components,
        "supports": supports,
        "load_cases": [],
        "combinations": [],
        "spectra": spectra,
    })
}

fn segment(id: &str, from: &str, to: &str, od: f64, wall: f64, yref: [f64; 3], provenance: &str) -> Value {
    json!({"id": id, "from": from, "to": to, "material": "steel",
        "section": {"outside_diameter": {"unit": "m", "value": od}, "wall_thickness": {"unit": "m", "value": wall},
            "material_density": {"unit": "kg/m^3", "value": 7850.0}},
        "y_reference": {"x": yref[0], "y": yref[1], "z": yref[2]}, "provenance": provenance})
}

fn support_value(id: &str, node: &str, family: &str, restraints: &[&str], provenance: &str) -> Value {
    json!({"id": id, "node": node, "family": family, "restraints": restraints, "provenance": provenance})
}

/// One committed document's inputs.
struct Fixture {
    name: &'static str,
    origin: Origin,
    model: Value,
    request: Value,
    modal: ModalStatement,
    census: Census,
    combination: CombinationRequest,
    /// Each direction's formation record, made where the reference's contributions are taken,
    /// from the reference's own request and spectrum.
    formation: Vec<FormationRecord>,
}

impl Fixture {
    fn document(&self) -> Value {
        let binding = ModalResultBinding::NotApplicable {
            reason: "constructed from a frozen reference's contributions: no modal result exists, and the modal basis is \
the reference's, stated in modal_basis and attributed to it"
                .into(),
        };
        self.document_with(&self.origin, &binding)
    }

    fn document_with(&self, origin: &Origin, binding: &ModalResultBinding) -> Value {
        let contributions = SignedContributions { combination: self.combination.clone(), formation: self.formation.clone() };
        spectrum_result_document(&DocumentInputs {
            origin,
            model: &self.model,
            request: &self.request,
            modal_result: binding,
            modal: &self.modal,
            census: &self.census,
            contributions: &contributions,
        })
    }
}

/// The formation records of a reference's request: for each of its directions (direction,
/// spectrum, scale factor), the spectrum's record as this writer states it in the model stand-in.
fn formation_records(directions: &[(Direction, String, f64)], record: impl Fn(&str) -> Value) -> Vec<FormationRecord> {
    directions.iter().map(|(d, id, scale)| FormationRecord {
        direction: *d, scale_factor: *scale, spectrum_id: id.clone(),
        spectrum_record_sha256: spectrum_doc_sha256(&record(id)).expect("a spectrum record in the profile"),
    }).collect()
}

/// The signed contributions of every periodic mode and direction and the rigid responses, one
/// value per roster row, by `value(row, source)` from a per-mode (or rigid) source.
fn contributions<'a, S>(rows: &[Quantity], directions: &[Direction], periodic_modes: &[u32], mode_source: impl Fn(Direction, u32) -> S,
    rigid_source: Option<&dyn Fn(Direction) -> S>, value: impl Fn(&Quantity, &S) -> f64) -> (Vec<ModalContribution>, Vec<Direction>, Vec<Vec<f64>>)
where
    S: 'a,
{
    let mut periodic = Vec::new();
    for &d in directions {
        for &m in periodic_modes {
            let s = mode_source(d, m);
            periodic.push(ModalContribution { mode: ModeIndex(m), direction: d, values: rows.iter().map(|q| value(q, &s)).collect() });
        }
    }
    let mut rigid_values = Vec::new();
    if let Some(r) = rigid_source {
        for &d in directions {
            let s = r(d);
            rigid_values.push(rows.iter().map(|q| value(q, &s)).collect());
        }
    }
    (periodic, directions.to_vec(), rigid_values)
}

fn rigid_entries(directions: &[Direction], values: Vec<Vec<f64>>, left_out: &[ClusterIndex]) -> Vec<RigidContribution> {
    directions.iter().zip(values).map(|(d, v)| RigidContribution { direction: *d, periodic_clusters_left_out: left_out.to_vec(), values: v }).collect()
}

// ---------------------------------------------------------------------------------------------
// Discrete cases.

struct Discrete {
    case: Value,
    analysis: Value,
}

impl Discrete {
    fn new(text: &str, case: usize, request_id: &str) -> Discrete {
        let case = cases(text)[case].clone();
        let analysis = case["analyses"].as_array().unwrap().iter().find(|a| a["request"]["id"] == request_id).unwrap().clone();
        Discrete { case, analysis }
    }

    fn directions(&self) -> Vec<(Direction, String, f64)> {
        let mut v: Vec<(Direction, String, f64)> = self.analysis["request"]["directions"].as_array().unwrap().iter()
            .map(|d| (Direction::parse(d["direction"].as_str().unwrap()).unwrap(), d["spectrum"].as_str().unwrap().to_string(),
                d["scale_factor"].as_f64().unwrap()))
            .collect();
        v.sort_by_key(|(d, _, _)| *d);
        v
    }

    fn damping(&self) -> f64 {
        let id = &self.directions()[0].1;
        self.case["spectra"].as_array().unwrap().iter().find(|s| s["id"] == id.as_str()).unwrap()["damping_ratio"].as_f64().unwrap()
    }

    fn cluster_of(&self, mode: u64) -> u32 {
        self.case["modal_basis"]["scope_clusters"].as_array().unwrap().iter()
            .position(|c| c["modes"].as_array().unwrap().iter().any(|m| m.as_u64() == Some(mode))).unwrap() as u32 + 1
    }

    fn modes(&self) -> Vec<ModalMode> {
        self.analysis["mode_status"].as_array().unwrap().iter().map(|m| {
            let mode = m["mode"].as_u64().unwrap();
            ModalMode { mode: ModeIndex(mode as u32), cluster: ClusterIndex(self.cluster_of(mode)), frequency_hz: real(&m["frequency_Hz"]) }
        }).collect()
    }

    fn periodic_modes(&self, applied: bool) -> Vec<u32> {
        self.analysis["mode_status"].as_array().unwrap().iter()
            .filter(|m| !applied || m["status"] == "periodic").map(|m| m["mode"].as_u64().unwrap() as u32).collect()
    }

    fn missing_mass(&self) -> MissingMass {
        let mm = &self.analysis["request"]["missing_mass"];
        if mm["applied"] == true {
            MissingMass::Applied { rigid_frequency_hz: mm["rigid_frequency_Hz"].as_f64().unwrap(),
                rule: RigidRule::parse(mm["rigid_periodic_combination"].as_str().unwrap()).unwrap() }
        } else {
            MissingMass::NotApplied
        }
    }

    fn rules(&self) -> (ModalRule, DirectionalRule) {
        let r = &self.analysis["request"];
        (ModalRule::parse(r["modal_combination"].as_str().unwrap()).unwrap(),
            DirectionalRule::parse(r["directional_combination"].as_str().unwrap()).unwrap())
    }

    /// The values of one mode's contribution in one direction (the global block, or `block`).
    fn mode_values(&self, block: Option<&str>, d: Direction, mode: u32) -> &Map<String, Value> {
        let source = block.map_or(&self.analysis, |b| &self.analysis[b]);
        source["contributions_signed"][d.as_str()].as_array().unwrap().iter()
            .find(|e| e["mode"].as_u64() == Some(u64::from(mode))).unwrap()["values"].as_object().unwrap()
    }

    fn rigid_values(&self, d: Direction) -> &Map<String, Value> {
        self.analysis["rigid_response_signed"][d.as_str()].as_object().unwrap()
    }

    fn all_modes(&self) -> Vec<&Value> {
        self.case["modal_basis"]["clusters"].as_array().unwrap().iter().collect()
    }

    /// The modal basis as the reference states it.
    fn modal(&self, modes: &[ModalMode], effective: Vec<[f64; 3]>) -> ModalStatement {
        let extracted = modes.len();
        let all = self.all_modes();
        let finite: usize = all.iter().map(|c| c["modes"].as_array().unwrap().len()).sum();
        let next = all.iter().find(|c| c["modes"].as_array().unwrap().iter().any(|m| m.as_u64() == Some(extracted as u64 + 1)));
        let total = &self.case["modal_basis"]["total_mass_by_direction_kg"];
        // Each mode's ω is its exact-repeat group's published circular frequency.
        let omega = |mode: u32| -> f64 {
            let g = all.iter().find(|c| c["modes"].as_array().unwrap().iter().any(|m| m.as_u64() == Some(u64::from(mode)))).unwrap();
            real(&g["circular_frequency_rad_per_s"])
        };
        ModalStatement {
            attributed_to: format!("frozen_reference:{DISCRETE}:{}", self.case["id"].as_str().unwrap()),
            standings: None,
            mode_standings: vec![None; extracted],
            state: None,
            extraction: Extraction {
                endpoint: if next.is_some() { "below_computed_cluster".into() } else { "whole_finite_spectrum".into() },
                reported_mode_count: extracted,
                established_finite_mode_count: Some(finite),
                next_computed_frequency_hz: next.map(|c| real(&c["frequency_Hz"])),
                next_computed_circular_frequency_rad_s: next.map(|c| real(&c["circular_frequency_rad_per_s"])),
                evidence: format!("the reference lists every one of the model's {finite} finite modes (modal_basis.clusters, with \
Sturm counts agreeing at every gap in modal_cross_checks); the next computed mode above the {extracted} extracted is stated \
with its frequency"),
            },
            total_mass_kg: [real(&total["X"]), real(&total["Y"]), real(&total["Z"])],
            effective_mass_kg: effective,
            circular_frequency_rad_s: modes.iter().map(|m| omega(m.mode.0)).collect(),
            cluster_tolerance: CLUSTER_TOLERANCE,
        }
    }

    /// Each extracted mode's effective mass, from its single-mode cluster.
    fn single_mode_effective_masses(&self, modes: &[ModalMode]) -> Vec<[f64; 3]> {
        modes.iter().map(|m| {
            let c = self.all_modes().into_iter().find(|c| c["modes"].as_array().unwrap() == &vec![json!(m.mode.0)]).expect("a single-mode cluster");
            let e = &c["effective_mass_kg"];
            [real(&e["X"]), real(&e["Y"]), real(&e["Z"])]
        }).collect()
    }

    fn nodes(&self) -> Vec<(String, [f64; 3])> {
        self.case["model"]["nodes"].as_array().unwrap().iter().map(|n| {
            let p = n["xyz"].as_array().unwrap();
            (n["id"].as_str().unwrap().to_string(), [p[0].as_f64().unwrap(), p[1].as_f64().unwrap(), p[2].as_f64().unwrap()])
        }).collect()
    }

    /// The formation records of the analysis's own request and the case's spectra.
    fn formation(&self) -> Vec<FormationRecord> {
        formation_records(&self.directions(), |id| {
            discrete_spectrum(self.case["spectra"].as_array().unwrap().iter().find(|s| s["id"] == id).unwrap())
        })
    }

    fn spectra_used(&self) -> Vec<Value> {
        let used: Vec<String> = self.directions().into_iter().map(|(_, s, _)| s).collect();
        self.case["spectra"].as_array().unwrap().iter().filter(|s| used.iter().any(|u| s["id"] == u.as_str())).map(discrete_spectrum).collect()
    }

    fn section(&self) -> (f64, f64) {
        let s = self.case["model"]["sections"].as_object().unwrap().values().next().unwrap();
        (s["outside_diameter_m"].as_f64().unwrap(), s["wall_thickness_m"].as_f64().unwrap())
    }

    fn members(&self) -> Vec<(String, String, String, [f64; 3])> {
        self.case["model"]["members"].as_array().unwrap().iter().map(|m| {
            let y = m["y_reference"].as_array().unwrap();
            (m["id"].as_str().unwrap().to_string(), m["i"].as_str().unwrap().to_string(), m["j"].as_str().unwrap().to_string(),
                [y[0].as_f64().unwrap(), y[1].as_f64().unwrap(), y[2].as_f64().unwrap()])
        }).collect()
    }

    /// The rigid constraints the reference's restrained DOFs give, grouped by node and component.
    fn rigid_constraints(&self) -> Vec<RigidConstraint> {
        self.case["derived_model"]["restrained_dofs"].as_array().unwrap().iter().map(|r| RigidConstraint {
            node: r[0].as_str().unwrap().to_string(),
            component: dof_component(r[1].as_str().unwrap()),
            holders: vec![r[2].as_str().unwrap().to_string()],
        }).collect()
    }
}

/// The straight members of a discrete case whose frames are signed permutations, with their
/// triads.
fn axis_aligned_members(d: &Discrete) -> Vec<StraightMember> {
    let nodes: BTreeMap<String, [f64; 3]> = d.nodes().into_iter().collect();
    d.members().into_iter().map(|(id, i, j, yref)| StraightMember {
        axes: axis_aligned_triad(nodes[&i], nodes[&j], yref), id, end_i: i, end_j: j, reference_vector: yref,
    }).collect()
}

/// A discrete case's value of a row from a per-mode or rigid key map, with axis-aligned member
/// axes formed by signed permutation and reaction keys named by `reaction`.
fn discrete_value(q: &Quantity, m: &Map<String, Value>, frames: &BTreeMap<String, Triad>, reaction: &dyn Fn(&Quantity) -> String,
    names: &dyn Fn(&str) -> String) -> f64 {
    match q {
        Quantity::NodeDisplacement { node, component } => real(&m[&format!("{}.{}", names(node), component.as_str())]),
        Quantity::ElementEndAction { member, end, component, frame: Frame::GlobalAxes, .. } => {
            real(&m[&format!("{}.{}.{}", names(member), end.as_str(), component.as_str())])
        }
        Quantity::ElementEndAction { member, end, component, frame: Frame::MemberAxes, .. } => {
            let (base, axis) = triple(*component);
            let g: Vec<f64> = (0..3).map(|k| real(&m[&format!("{}.{}.{}", names(member), end.as_str(), FORCES[base + k].as_str())])).collect();
            permuted(&frames[member], [g[0], g[1], g[2]], axis)
        }
        Quantity::ConstraintReaction { .. } | Quantity::CompliantSupportForce { .. } => real(&m[&reaction(q)]),
        other => panic!("no discrete row {other:?}"),
    }
}

fn origin(package: &str, case: &Value, analysis: &str, constructed: Vec<String>) -> Origin {
    Origin::ConstructedFromReference { package: package.into(), case: case["id"].as_str().unwrap().into(), analysis: analysis.into(), constructed }
}

const STAND_IN: &str = "Except where stated above, each entity identity and the spectra are the reference's. The model \
document is a stand-in in the model document's shape, not a schema-checked model; its materials, sections and support \
families are placeholders, not checked against the reference. The spectrum request is this unit's stand-in shape. The \
extraction endpoint, the reported and established mode counts and the completion evidence sentence in modal_basis are \
derived by this writer from the reference's mode list and its Sturm evidence; they are not fields of the reference. Each \
direction's formation record (its spectrum, the digest of that spectrum's record in the model stand-in, and its scale \
factor) is this writer's statement of the reference's request.";

/// The census axes of axis-aligned straight members, which the discrete references do not state.
const AXES_FROM_GEOMETRY: &str = "The member axes stated in the census are constructed by this writer from the reference's \
node coordinates and each member's y_reference, by REQ-006's definition (x from end i to end j, y the part of the reference \
vector normal to x, normalized, z = x cross y); the reference does not state them as vectors. Each is a signed permutation \
of the global axes.";

/// 1. C9-Y: the straight members on the space diagonal, in global and member axes.
fn c9_y_two_frames() -> Fixture {
    let d = Discrete::new(C9, 0, "C9-Y");
    let modes = d.modes();
    let (modal_rule, directional_rule) = d.rules();
    let missing = d.missing_mass();
    let dirs = d.directions();
    let directions: Vec<Direction> = dirs.iter().map(|x| x.0).collect();
    // The member frames, from the reference's 40-digit rows (each parsed to its nearest binary64).
    let frames = &d.case["derived_model"]["member_frames"];
    let members: Vec<StraightMember> = d.members().into_iter().map(|(id, i, j, yref)| {
        let rows = &frames[&id]["rows_x_y_z_in_global"];
        let row = |k: usize| -> [f64; 3] { [0, 1, 2].map(|c| decimal_f64(rows[k][c].as_str().unwrap())) };
        StraightMember { axes: [row(0), row(1), row(2)], id, end_i: i, end_j: j, reference_vector: yref }
    }).collect();
    let spring = &d.case["model"]["springs"][0];
    let census = Census {
        nodes: d.nodes().into_iter().map(|(n, _)| n).collect(),
        straight_members: members,
        rigid_constraints: d.rigid_constraints(),
        compliant_supports: vec![CompliantSupport { support: spring["id"].as_str().unwrap().into(), node: spring["node"].as_str().unwrap().into(),
            components: vec![dof_component(spring["dof"].as_str().unwrap())] }],
        ..Census::default()
    };
    let rows = census.roster();
    let periodic = d.periodic_modes(false);
    let reaction = |q: &Quantity| -> String {
        match q {
            Quantity::ConstraintReaction { component, holders, .. } => format!("{}.{}", holders[0], reaction_key(*component)),
            Quantity::CompliantSupportForce { support, component } => format!("{support}.{}", reaction_key(*component)),
            _ => unreachable!(),
        }
    };
    let value = |q: &Quantity, (g, m): &(&Map<String, Value>, &Map<String, Value>)| -> f64 {
        match q {
            Quantity::ElementEndAction { member, end, component, frame: Frame::MemberAxes, .. } => {
                real(&m[&format!("{member}.{}.{}_m", end.as_str(), component.as_str())])
            }
            _ => discrete_value(q, g, &BTreeMap::new(), &reaction, &|s| s.to_string()),
        }
    };
    let (contributions, _, _) = contributions(&rows, &directions, &periodic,
        |dir, mode| (d.mode_values(None, dir, mode), d.mode_values(Some("element_end_forces_member_axes"), dir, mode)), None, value);
    let effective = d.single_mode_effective_masses(&modes);
    let (od, wall) = d.section();
    let prov = "stand-in: discrete C9-PIPE-DIAGONAL";
    let nodes = d.nodes();
    let model = model_value("project:spectrum-c9", prov, &nodes,
        d.members().iter().map(|(id, i, j, y)| segment(id, i, j, od, wall, *y, prov)).collect(), vec![],
        vec![support_value("SA", "A", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"], prov),
            json!({"id": "S1", "node": "B", "family": "spring", "restraints": ["UX"], "stiffness": {"dof": "UX", "value": {"unit": "N/m", "value": 8.0}}, "provenance": prov})],
        d.spectra_used());
    let request = request_value(&dirs.iter().map(|(a, b, c)| (*a, b.as_str(), *c)).collect::<Vec<_>>(), modal_rule, directional_rule, missing);
    Fixture {
        name: "c9_y_two_frames",
        origin: origin(DISCRETE, &d.case, "C9-Y", vec![
            "The member-axes contributions are the reference's element_end_forces_member_axes block; the global contributions are its global block; nothing is rotated here.".into(),
            "The member axes stated in the census are the reference's derived_model.member_frames rows, each 40-digit decimal parsed to its nearest binary64.".into(),
            STAND_IN.into(),
        ]),
        model,
        request,
        modal: d.modal(&modes, effective),
        census,
        combination: CombinationRequest { damping_ratio: d.damping(), modal_rule, directional_rule, missing_mass: missing, directions,
            modes, rows, periodic: contributions, rigid: vec![] },
        formation: d.formation(),
    }
}

/// 2. C7-PAIRS basis 1: an exactly repeated pair whose contributions to a row cancel exactly.
fn c7_cancelling_cluster() -> Fixture {
    let d = Discrete::new(C7, 0, "C7-PAIRS");
    let modes = d.modes();
    let (modal_rule, directional_rule) = d.rules();
    let missing = d.missing_mass();
    let dirs = d.directions();
    let directions: Vec<Direction> = dirs.iter().map(|x| x.0).collect();
    let basis = &d.analysis["basis_witness"]["bases"]["basis_2"];
    let vectors = basis["vectors"].as_array().unwrap();
    // The reduced roster: the quantities the basis witness publishes per vector.
    let anchor = d.rigid_constraints();
    let census = Census { nodes: vec!["N1".into(), "N3".into()], rigid_constraints: anchor.clone(), ..Census::default() };
    let rows = census.roster();
    let pair_two = |dir: Direction| -> &Map<String, Value> {
        d.analysis["contributions_signed"][dir.as_str()].as_array().unwrap().iter()
            .find(|e| e["modes"] == json!([3, 4])).unwrap()["values_signed_sum_over_repeated_cluster"].as_object().unwrap()
    };
    let key = |q: &Quantity| -> String {
        match q {
            Quantity::NodeDisplacement { node, component } => format!("{node}.{}", component.as_str()),
            Quantity::ConstraintReaction { component, holders, .. } => format!("{}.{}", holders[0], reaction_key(*component)),
            _ => unreachable!(),
        }
    };
    // N1's six components are rigidly restrained by the anchor: each row is zero (REQ-006, Nodes).
    let value = |q: &Quantity, m: &Map<String, Value>| -> f64 {
        match q {
            Quantity::NodeDisplacement { node, .. } if node == "N1" => 0.0,
            _ => real(&m[&key(q)]),
        }
    };
    let mut periodic = Vec::new();
    for &dir in &directions {
        for (k, v) in vectors.iter().enumerate() {
            let m = v["contributions_signed"][dir.as_str()].as_object().unwrap();
            periodic.push(ModalContribution { mode: ModeIndex(k as u32 + 1), direction: dir, values: rows.iter().map(|q| value(q, m)).collect() });
        }
        let s = pair_two(dir);
        periodic.push(ModalContribution { mode: ModeIndex(3), direction: dir, values: rows.iter().map(|q| value(q, s)).collect() });
        periodic.push(ModalContribution { mode: ModeIndex(4), direction: dir, values: vec![0.0; rows.len()] });
    }
    // Effective masses per mode: each cluster 1 vector's Γ_d² · m_gen, from its published decimals
    // exactly and rounded once; cluster 2's published effective mass on mode 3 and none on mode 4.
    let vector_mass = |v: &Value| -> [f64; 3] {
        let m = Dec::parse(decimal(&v["generalized_mass_kg"]));
        ["X", "Y", "Z"].map(|dir| {
            let g = Dec::parse(decimal(&v["participation_factor"][dir]));
            g.mul(&g).mul(&m).to_f64()
        })
    };
    let all = d.all_modes();
    let cluster_mass = |i: usize| -> [f64; 3] { let e = &all[i]["effective_mass_kg"]; [real(&e["X"]), real(&e["Y"]), real(&e["Z"])] };
    let (c1, c2) = (cluster_mass(0), cluster_mass(1));
    let (v1, v2) = (vector_mass(&vectors[0]), vector_mass(&vectors[1]));
    for k in 0..3 {
        assert!((v1[k] + v2[k] - c1[k]).abs() <= 1e-14 * c1[k], "the vectors' effective masses sum to the cluster's: {k}");
    }
    let effective = vec![v1, v2, c2, [0.0; 3]];
    let (od, wall) = d.section();
    let prov = "stand-in: discrete C7-REPEATED-PAIR, reduced to the quantities its basis witness publishes per vector";
    let nodes: Vec<(String, [f64; 3])> = d.nodes();
    let model = model_value("project:spectrum-c7", prov, &nodes,
        d.members().iter().map(|(id, i, j, y)| segment(id, i, j, od, wall, *y, prov)).collect(), vec![],
        vec![support_value("A1", "N1", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"], prov)], d.spectra_used());
    let request = request_value(&dirs.iter().map(|(a, b, c)| (*a, b.as_str(), *c)).collect::<Vec<_>>(), modal_rule, directional_rule, missing);
    Fixture {
        name: "c7_cancelling_cluster",
        origin: origin(DISCRETE, &d.case, "C7-PAIRS", vec![
            "The roster and census are a subset of the reference model: the tip node N3 and the anchor A1's six constraint reactions at N1, the quantities the reference's basis_witness publishes per vector, and the node N1. Node N2 and members P1 and P2 are in the model and have no rows here.".into(),
            "N1's six displacement and rotation rows are constructed: each contribution is zero because all six components at N1 are rigidly restrained by the anchor A1 (REQ-006, Nodes: a component restrained on a global axis is zero), not because the reference publishes them.".into(),
            "Cluster 1 (modes 1 and 2) is the basis_witness basis_2 vectors 1 and 2 (the M-orthonormal rotation of basis_1 by cos 3/5, sin 4/5, each normalized to unit maximum translation), each value as published; under Y their contributions to N3.uz, N3.ry, the anchor's UZ and RY reactions cancel exactly, and under Z those to N3.uy, N3.rz, UY and RZ.".into(),
            "Cluster 2's per-mode contributions are constructed: mode 3 carries the cluster's published signed sum (values_signed_sum_over_repeated_cluster) and mode 4 carries zero, in each direction.".into(),
            "The per-mode effective masses are constructed: each cluster 1 vector's is its published participation factor squared times its published generalized mass, formed exactly from the decimals and rounded once (their sum agrees with the cluster's published effective mass to 1e-14 relative); mode 3 carries cluster 2's published effective mass and mode 4 none.".into(),
            STAND_IN.into(),
        ]),
        model,
        request,
        modal: d.modal(&modes, effective),
        census,
        combination: CombinationRequest { damping_ratio: d.damping(), modal_rule, directional_rule, missing_mass: missing, directions,
            modes, rows, periodic, rigid: vec![] },
        formation: d.formation(),
    }
}

// ---------------------------------------------------------------------------------------------
// W2 cases.

struct W2 {
    doc: Value,
    inputs: Value,
}

impl W2 {
    fn new(text: &str) -> W2 {
        W2 { doc: serde_json::from_str(text).unwrap(), inputs: serde_json::from_str(W2_INPUTS).unwrap() }
    }

    fn analysis(&self) -> &Value {
        &self.doc["analysis"]
    }

    fn zeta(&self) -> f64 {
        decimal_f64(self.analysis()["request"]["zeta"].as_str().unwrap())
    }

    fn dirs(&self) -> Vec<(Direction, String, f64)> {
        self.analysis()["request"]["directions"].as_array().unwrap().iter()
            .map(|d| (Direction::parse(d[0].as_str().unwrap()).unwrap(), d[1].as_str().unwrap().to_string(), decimal_f64(d[2].as_str().unwrap())))
            .collect()
    }

    fn rigid_frequency(&self) -> f64 {
        decimal_f64(self.analysis()["request"]["rigid_frequency_Hz"].as_str().unwrap())
    }

    fn modes(&self) -> Vec<ModalMode> {
        let clusters = self.analysis()["extracted_clusters"].as_array().unwrap();
        let basis = self.doc["modal_basis"]["modes"].as_array().unwrap();
        self.analysis()["extracted_modes"].as_array().unwrap().iter().map(|m| {
            let mode = m.as_u64().unwrap();
            let cluster = clusters.iter().position(|c| c.as_array().unwrap().iter().any(|x| x.as_u64() == Some(mode))).unwrap() as u32 + 1;
            let f = basis.iter().find(|b| b["index"].as_u64() == Some(mode)).unwrap();
            ModalMode { mode: ModeIndex(mode as u32), cluster: ClusterIndex(cluster), frequency_hz: real(&f["frequency_Hz"]) }
        }).collect()
    }

    fn periodic_modes(&self) -> Vec<u32> {
        self.analysis()["periodic_modes"].as_array().unwrap().iter().map(|m| m.as_u64().unwrap() as u32).collect()
    }

    fn mode_values(&self, d: Direction, mode: u32) -> &Map<String, Value> {
        self.analysis()["contributions_signed"][d.as_str()][mode.to_string()].as_object().unwrap()
    }

    fn rigid_values(&self, d: Direction) -> &Map<String, Value> {
        self.analysis()["rigid_response_signed"][d.as_str()].as_object().unwrap()
    }

    fn modal(&self, modes: &[ModalMode], case: &str) -> ModalStatement {
        let all = self.doc["modal_basis"]["modes"].as_array().unwrap();
        let extracted = modes.len();
        let next = all.iter().find(|m| m["index"].as_u64() == Some(extracted as u64 + 1));
        let total = &self.doc["modal_basis"]["T_k_kg"];
        let effective = modes.iter().map(|m| {
            let e = &all.iter().find(|b| b["index"].as_u64() == Some(u64::from(m.mode.0))).unwrap()["effective_mass_kg"];
            [real(&e["X"]), real(&e["Y"]), real(&e["Z"])]
        }).collect();
        ModalStatement {
            attributed_to: format!("frozen_reference:{RECOVERY_W2}:{case}"),
            standings: None,
            mode_standings: vec![None; extracted],
            state: None,
            extraction: Extraction {
                endpoint: if next.is_some() { "below_computed_cluster".into() } else { "whole_finite_spectrum".into() },
                reported_mode_count: extracted,
                established_finite_mode_count: Some(all.len()),
                next_computed_frequency_hz: next.map(|m| real(&m["frequency_Hz"])),
                next_computed_circular_frequency_rad_s: next.map(|m| real(&m["omega_rad_per_s"])),
                evidence: format!("the reference lists every one of the model's {} finite modes (modal_basis.modes, with Sturm counts \
agreeing at every gap); the next computed mode above the {extracted} extracted below the cut-off is stated with its frequency", all.len()),
            },
            total_mass_kg: [real(&total["X"]), real(&total["Y"]), real(&total["Z"])],
            effective_mass_kg: effective,
            circular_frequency_rad_s: modes.iter()
                .map(|m| real(&all.iter().find(|b| b["index"].as_u64() == Some(u64::from(m.mode.0))).unwrap()["omega_rad_per_s"]))
                .collect(),
            cluster_tolerance: CLUSTER_TOLERANCE,
        }
    }

    fn spectra(&self) -> Vec<Value> {
        let zeta = self.zeta();
        let s = self.inputs["spectra"].as_object().unwrap();
        self.dirs().iter().map(|(_, id, _)| w2_spectrum(id, &s[id], zeta)).collect()
    }

    /// The formation records of the analysis's own request and W2's spectra.
    fn formation(&self) -> Vec<FormationRecord> {
        let zeta = self.zeta();
        formation_records(&self.dirs(), |id| w2_spectrum(id, &self.inputs["spectra"][id], zeta))
    }

    fn constituent(&self, id: &str) -> &Value {
        self.doc["derived_model"]["stiffness_constituents"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap()
    }

    fn tangent_frame(&self, end: &str) -> Triad {
        let rows = &self.constituent("BEND1")[format!("tangent_frame_rows_end_{end}")];
        [0, 1, 2].map(|r| [0, 1, 2].map(|c| real(&rows[r][c])))
    }
}

fn w2_request(w: &W2) -> Value {
    request_value(&w.dirs().iter().map(|(a, b, c)| (*a, b.as_str(), *c)).collect::<Vec<_>>(), ModalRule::Cqc, DirectionalRule::Srss,
        MissingMass::Applied { rigid_frequency_hz: w.rigid_frequency(), rule: RigidRule::Srss })
}

/// The rows of an anchor named by the reference by its node: the constructed support identity
/// `<node>-anchor`.
fn anchors(nodes: &[&str]) -> Vec<RigidConstraint> {
    nodes.iter().flat_map(|n| REACTION_KEYS.iter().map(move |(c, _)| RigidConstraint { node: (*n).into(), component: *c, holders: vec![format!("{n}-anchor")] }))
        .collect()
}

/// W2's value of a row: the published key, with arc tangent rows from `_t`, connector and rod rows
/// from `J1.g.*`, `J1.<end>.*` and `R*.T`, anchor reactions by node, and member axes by `member`.
fn w2_value(q: &Quantity, m: &Map<String, Value>, member: &dyn Fn(&str, End, ForceComponent, &Map<String, Value>) -> f64) -> f64 {
    let g = |key: String| real(&m[&key]);
    match q {
        Quantity::NodeDisplacement { node, component } => g(format!("{node}.{}", component.as_str())),
        Quantity::ElementEndAction { member: id, end, component, frame, .. } => match frame {
            Frame::GlobalAxes => g(format!("{id}.{}.{}", end.as_str(), component.as_str())),
            Frame::ArcTangentFrame => g(format!("{id}.{}.{}_t", end.as_str(), component.as_str())),
            Frame::MemberAxes => member(id, *end, *component, m),
            other => panic!("frame {other:?}"),
        },
        Quantity::ConnectorGeneralizedForce { connector, component } => {
            let name = match component {
                ConnectorComponent::Axial => "N",
                ConnectorComponent::LateralY => "Vy",
                ConnectorComponent::LateralZ => "Vz",
                ConnectorComponent::Torsion => "T",
                ConnectorComponent::BendingY => "My",
                ConnectorComponent::BendingZ => "Mz",
            };
            g(format!("{connector}.g.{name}"))
        }
        Quantity::ConnectorEndAction { connector, end, component } => g(format!("{connector}.{}.{}", end.as_str(), component.as_str())),
        Quantity::TieRodForce { rod, .. } => g(format!("{rod}.T")),
        Quantity::ConstraintReaction { node, component, .. } => g(format!("{node}.{}", reaction_key(*component))),
        other => panic!("no W2 row {other:?}"),
    }
}

/// The oblique straights' exact frames as numerators over 25 (rows x, y, z; REQ-006's definition
/// with reference vector (0, 0, 1)).
const OBLIQUE_FRAMES: [(&str, [[i64; 3]; 3]); 2] = [
    ("P1", [[15, 20, 0], [0, 0, 25], [20, -15, 0]]),
    ("P2", [[-12, 9, 20], [16, -12, 15], [15, 20, 0]]),
];

fn oblique_frame(id: &str) -> [[i64; 3]; 3] {
    OBLIQUE_FRAMES.iter().find(|(m, _)| *m == id).unwrap().1
}

fn w2_arc_census(w: &W2) -> Census {
    let triad = |id: &str| oblique_frame(id).map(|r| r.map(|p| p as f64 / 25.0));
    Census {
        nodes: ["A", "B", "C", "D"].map(String::from).to_vec(),
        straight_members: vec![
            StraightMember { id: "P1".into(), end_i: "A".into(), end_j: "B".into(), reference_vector: [0.0, 0.0, 1.0], axes: triad("P1") },
            StraightMember { id: "P2".into(), end_i: "C".into(), end_j: "D".into(), reference_vector: [0.0, 0.0, 1.0], axes: triad("P2") },
        ],
        arcs: vec![ArcMember { id: "BEND1".into(), end_i: "B".into(), end_j: "C".into(), tangent_frame_i: w.tangent_frame("i"), tangent_frame_j: w.tangent_frame("j") }],
        rigid_constraints: anchors(&["A", "D"]),
        ..Census::default()
    }
}

fn w2_arc_fixture(name: &'static str, arc_member_axes: bool) -> Fixture {
    let w = W2::new(W2_OBLIQUE);
    let modes = w.modes();
    let census = w2_arc_census(&w);
    let mut rows = census.roster();
    let dirs = w.dirs();
    let directions: Vec<Direction> = dirs.iter().map(|x| x.0).collect();
    // P1's and P2's member-axes rows: the published global decimals rotated exactly.
    let member = |id: &str, end: End, c: ForceComponent, m: &Map<String, Value>| -> f64 {
        let (base, axis) = triple(c);
        let comps: [&str; 3] = [0, 1, 2].map(|k| decimal(&m[&format!("{id}.{}.{}", end.as_str(), FORCES[base + k].as_str())]));
        rotate_row(oblique_frame(id)[axis], comps)
    };
    let value = |q: &Quantity, m: &&Map<String, Value>| w2_value(q, m, &member);
    let periodic_modes = w.periodic_modes();
    let rigid_source = |dir: Direction| w.rigid_values(dir);
    let (periodic, _, rigid) = contributions(&rows, &directions, &periodic_modes, |dir, mode| w.mode_values(dir, mode), Some(&rigid_source), value);
    let mut left_out: Vec<ClusterIndex> = modes.iter().filter(|m| periodic_modes.contains(&m.mode.0)).map(|m| m.cluster).collect();
    left_out.dedup();
    let mut constructed = vec![
        "P1's and P2's member-axes rows are constructed, not published by the reference: each signed contribution and rigid response is the exact rotation, by the member's rows x, y and z (x along the member, y = (0, 0, 1) normal to x, z = x cross y; exact rationals), of the reference's published global decimals, rounded once to the nearest binary64.".into(),
        "P1's and P2's member axes stated in the census are constructed: each component is the exact rational over 25 that REQ-006's definition gives from the stand-in's node coordinates with reference vector (0, 0, 1), rounded once to binary64; the reference does not state them.".into(),
        "The anchors at A and D have no identity in the reference, which names each reaction by its node; their support identities A-anchor and D-anchor are constructed.".into(),
        "The arc's rows are its total end action as published (BEND1.<end>.* and BEND1.<end>.*_t); the reference's elastic and lump_load parts are not rows.".into(),
        STAND_IN.into(),
    ];
    if arc_member_axes {
        for q in &mut rows {
            if let Quantity::ElementEndAction { family: MemberFamily::Arc, frame, .. } = q {
                if *frame == Frame::ArcTangentFrame {
                    *frame = Frame::MemberAxes;
                }
            }
        }
        constructed.insert(0, "Mutation: the arc's twelve arc-tangent-frame rows are stated in member axes, with their values unchanged; member axes are never an arc's frame, so the request is refused.".into());
    }
    let nodes: Vec<(String, [f64; 3])> = [("A", [0.0, 0.0, 0.0]), ("B", [0.75, 1.0, 0.0]), ("C", [0.84375, 1.90625, 0.625]), ("D", [0.09375, 2.46875, 1.875])]
        .iter().map(|(n, p)| (n.to_string(), *p)).collect();
    let prov = "stand-in: recovery_w2 V-ARC-OBL";
    let (od, wall) = (0.171875, 0.0078125);
    let model = model_value("project:spectrum-w2-arc-oblique", prov, &nodes,
        vec![segment("P1", "A", "B", od, wall, [0.0, 0.0, 1.0], prov), segment("BEND1", "B", "C", od, wall, [27.0, 11.0, -20.0], prov),
            segment("P2", "C", "D", od, wall, [0.0, 0.0, 1.0], prov)],
        vec![json!({"id": "BEND1 bend", "kind": "bend", "node": "B", "geometry": {"bend_pipe_ref": "BEND1", "bend_radius": {"unit": "m", "value": 0.78125}},
            "modifiers": {"flexibility_factor_user_value": {"unit": "none", "value": 4.25}}, "provenance": prov})],
        vec![support_value("A-anchor", "A", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"], prov),
            support_value("D-anchor", "D", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"], prov)],
        w.spectra());
    Fixture {
        name,
        origin: Origin::ConstructedFromReference { package: RECOVERY_W2.into(), case: "V-ARC-OBL".into(), analysis: "V-ARC-OBL base request".into(), constructed },
        model,
        request: w2_request(&w),
        modal: w.modal(&modes, "V-ARC-OBL"),
        census,
        combination: CombinationRequest { damping_ratio: w.zeta(), modal_rule: ModalRule::Cqc, directional_rule: DirectionalRule::Srss,
            missing_mass: MissingMass::Applied { rigid_frequency_hz: w.rigid_frequency(), rule: RigidRule::Srss }, directions: directions.clone(),
            modes, rows, periodic, rigid: rigid_entries(&directions, rigid, &left_out) },
        formation: w.formation(),
    }
}

/// 4. V-JOINT-R: the connector J1 with its tie rods R1 and R2 between the face pipes.
fn w2_joint_rods() -> Fixture {
    let w = W2::new(W2_JOINT_R);
    let modes = w.modes();
    let q = &w.inputs["connector"]["derived_exact"]["Q_columns_are_axes_row_major"];
    let rational = |v: &Value| -> f64 {
        let s = v.as_str().unwrap();
        match s.split_once('/') {
            Some((a, b)) => a.parse::<f64>().unwrap() / b.parse::<f64>().unwrap(),
            None => s.parse().unwrap(),
        }
    };
    // Q's columns are the connector axes.
    let connector_axes: Triad = [0, 1, 2].map(|col| [0, 1, 2].map(|row| rational(&q[row][col])));
    let node = |id: &str| -> [f64; 3] { [0, 1, 2].map(|k| decimal_f64(w.inputs["nodes"][id][k].as_str().unwrap())) };
    let face = |id: &str, i: &str, j: &str| StraightMember {
        id: id.into(), end_i: i.into(), end_j: j.into(), reference_vector: [0.0, 0.0, 1.0], axes: axis_aligned_triad(node(i), node(j), [0.0, 0.0, 1.0]),
    };
    let rod_axis = |id: &str| -> [f64; 3] { [0, 1, 2].map(|k| rational(&w.inputs["rods"]["derived_exact"][id]["unit_direction_n"][k])) };
    let census = Census {
        nodes: ["C", "D", "E", "F"].map(String::from).to_vec(),
        straight_members: vec![face("P2", "C", "D"), face("P3", "E", "F")],
        connectors: vec![Connector { id: "J1".into(), end_i: "D".into(), end_j: "E".into(), axes: connector_axes }],
        tie_rods: vec![TieRod { connector: "J1".into(), rod: "R1".into(), axis: rod_axis("R1") },
            TieRod { connector: "J1".into(), rod: "R2".into(), axis: rod_axis("R2") }],
        rigid_constraints: anchors(&["C", "F"]),
        ..Census::default()
    };
    let frames: BTreeMap<String, Triad> = census.straight_members.iter().map(|m| (m.id.clone(), m.axes)).collect();
    let rows = census.roster();
    let dirs = w.dirs();
    let directions: Vec<Direction> = dirs.iter().map(|x| x.0).collect();
    let member = |id: &str, end: End, c: ForceComponent, m: &Map<String, Value>| -> f64 {
        let (base, axis) = triple(c);
        let g = [0, 1, 2].map(|k| real(&m[&format!("{id}.{}.{}", end.as_str(), FORCES[base + k].as_str())]));
        permuted(&frames[id], g, axis)
    };
    let value = |q: &Quantity, m: &&Map<String, Value>| w2_value(q, m, &member);
    let periodic_modes = w.periodic_modes();
    let rigid_source = |dir: Direction| w.rigid_values(dir);
    let (periodic, _, rigid) = contributions(&rows, &directions, &periodic_modes, |dir, mode| w.mode_values(dir, mode), Some(&rigid_source), value);
    let mut left_out: Vec<ClusterIndex> = modes.iter().filter(|m| periodic_modes.contains(&m.mode.0)).map(|m| m.cluster).collect();
    left_out.dedup();
    let nodes: Vec<(String, [f64; 3])> = ["C", "D", "E", "F"].iter().map(|n| (n.to_string(), node(n))).collect();
    let prov = "stand-in: recovery_w2 V-JOINT-R";
    let (od, wall) = (0.171875, 0.0078125);
    let model = model_value("project:spectrum-w2-joint-rods", prov, &nodes,
        vec![segment("P2", "C", "D", od, wall, [0.0, 0.0, 1.0], prov), segment("span D-E", "D", "E", od, wall, [0.0, 0.0, 1.0], prov),
            segment("P3", "E", "F", od, wall, [0.0, 0.0, 1.0], prov)],
        vec![json!({"id": "J1", "kind": "expansion_joint", "node": "D", "provenance": prov,
            "notes": "stand-in for the objective connector J1 that replaces the span D-E, with its tie rods R1 and R2 (recovery_w2 w2_inputs.json connector and rods); not an objective_connector record"})],
        vec![support_value("C-anchor", "C", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"], prov),
            support_value("F-anchor", "F", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"], prov)],
        w.spectra());
    Fixture {
        name: "w2_joint_rods",
        origin: Origin::ConstructedFromReference { package: RECOVERY_W2.into(), case: "V-JOINT-R".into(), analysis: "V-JOINT-R base request".into(), constructed: vec![
            "P2's and P3's member-axes contributions are the reference's global contributions mapped by the signed permutation of their axis-aligned frames (x = +Y, y = +Z, z = +X), as its convention 6 states; each combined magnitude is formed from them.".into(),
            "The span D-E that the connector replaces has no rows; the rods' end actions (R1.<end>.*, R2.<end>.*) are not REQ-006 rows and are left out; each rod has its one axial force.".into(),
            "The anchors at C and F have no identity in the reference, which names each reaction by its node; their support identities C-anchor and F-anchor are constructed.".into(),
            "The connector axes are the columns of the reference's exact Q; the rod axes are its exact unit directions n.".into(),
            "P2's and P3's member axes stated in the census are constructed by this writer from the reference's node coordinates and the reference vector (0, 0, 1), by REQ-006's definition; the reference states the mapping in its convention 6, not the axes as vectors.".into(),
            STAND_IN.into(),
        ]},
        model,
        request: w2_request(&w),
        modal: w.modal(&modes, "V-JOINT-R"),
        census,
        combination: CombinationRequest { damping_ratio: w.zeta(), modal_rule: ModalRule::Cqc, directional_rule: DirectionalRule::Srss,
            missing_mass: MissingMass::Applied { rigid_frequency_hz: w.rigid_frequency(), rule: RigidRule::Srss }, directions: directions.clone(),
            modes, rows, periodic, rigid: rigid_entries(&directions, rigid, &left_out) },
        formation: w.formation(),
    }
}

/// The rule choice of one entry of a discrete analysis's published `mutations` grid: the modal
/// rule, the rigid rule (`None` for the correction off) and the directional rule.
type Mutation = (ModalRule, Option<RigidRule>, DirectionalRule);

/// A discrete case's fixture with axis-aligned members, a constraint map and an identity
/// renaming (`names` maps a document identity back to the reference's). With `mutation`, the
/// rules are that grid entry's rather than the analysis's base request's; `zero_nodes` are nodes
/// whose six components a rigid anchor restrains, each row zero.
#[allow(clippy::too_many_arguments)]
fn discrete_fixture<'d>(name: &'static str, d: &'d Discrete, analysis: &str, census: Census, constructed: Vec<String>, model: Value,
    reaction: &dyn Fn(&Quantity) -> String, names: &dyn Fn(&str) -> String, mutation: Option<Mutation>, zero_nodes: &[&str]) -> Fixture {
    let modes = d.modes();
    let (mut modal_rule, mut directional_rule) = d.rules();
    let mut missing = d.missing_mass();
    if let Some((modal, rigid, directional)) = mutation {
        modal_rule = modal;
        directional_rule = directional;
        missing = match (rigid, missing) {
            (None, _) => MissingMass::NotApplied,
            (Some(rule), MissingMass::Applied { rigid_frequency_hz, .. }) => MissingMass::Applied { rigid_frequency_hz, rule },
            (Some(_), MissingMass::NotApplied) => panic!("{name}: the base request states no rigid frequency"),
        };
    }
    let applied = matches!(missing, MissingMass::Applied { .. });
    let dirs = d.directions();
    let directions: Vec<Direction> = dirs.iter().map(|x| x.0).collect();
    let rows = census.roster();
    let frames: BTreeMap<String, Triad> = census.straight_members.iter().map(|m| (m.id.clone(), m.axes)).collect();
    let periodic_modes = d.periodic_modes(applied);
    let value = |q: &Quantity, m: &&Map<String, Value>| match q {
        Quantity::NodeDisplacement { node, .. } if zero_nodes.contains(&node.as_str()) => 0.0,
        _ => discrete_value(q, m, &frames, reaction, names),
    };
    let rigid_source = |dir: Direction| d.rigid_values(dir);
    let rigid_fn: Option<&dyn Fn(Direction) -> &'d Map<String, Value>> = if applied { Some(&rigid_source) } else { None };
    let (periodic, _, rigid) = contributions(&rows, &directions, &periodic_modes, |dir, mode| d.mode_values(None, dir, mode), rigid_fn, value);
    let left_out: Vec<ClusterIndex> = {
        let mut c: Vec<ClusterIndex> = modes.iter().filter(|m| periodic_modes.contains(&m.mode.0)).map(|m| m.cluster).collect();
        c.dedup();
        c
    };
    let effective = d.single_mode_effective_masses(&modes);
    let request = request_value(&dirs.iter().map(|(a, b, c)| (*a, b.as_str(), *c)).collect::<Vec<_>>(), modal_rule, directional_rule, missing);
    Fixture {
        name,
        origin: origin(DISCRETE, &d.case, analysis, constructed),
        model,
        request,
        modal: d.modal(&modes, effective),
        census,
        combination: CombinationRequest { damping_ratio: d.damping(), modal_rule, directional_rule, missing_mass: missing, directions: directions.clone(),
            modes, rows, periodic, rigid: rigid_entries(&directions, rigid, &left_out) },
        formation: d.formation(),
    }
}

/// 5. C8-III: a rigid constraint at B, UZ, held by the reference's restraint SB and a constructed
/// rigid snubber SN-B together.
fn c8_shared_constraint() -> Fixture {
    let d = Discrete::new(C8, 0, "C8-III-TRUNCATION");
    let mut constraints = d.rigid_constraints();
    for c in &mut constraints {
        if c.node == "B" && c.component == ReactionComponent::Uz {
            c.holders.push("SN-B".into());
        }
    }
    let census = Census {
        nodes: d.nodes().into_iter().map(|(n, _)| n).collect(),
        straight_members: axis_aligned_members(&d),
        rigid_constraints: constraints,
        ..Census::default()
    };
    let (od, wall) = d.section();
    let prov = "stand-in: discrete C8-PIPE-X, with a constructed rigid snubber SN-B at B on UZ";
    let model = model_value("project:spectrum-c8", prov, &d.nodes(),
        d.members().iter().map(|(id, i, j, y)| segment(id, i, j, od, wall, *y, prov)).collect(), vec![],
        vec![support_value("SA", "A", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"], prov),
            support_value("SB", "B", "line_stop", &["UZ"], prov), support_value("SN-B", "B", "snubber", &["UZ"], prov)],
        d.spectra_used());
    let reaction = |q: &Quantity| -> String {
        match q {
            // The reference's reaction of the restraint SB is the constraint's.
            Quantity::ConstraintReaction { component, holders, .. } => format!("{}.{}", holders[0], reaction_key(*component)),
            _ => unreachable!(),
        }
    };
    discrete_fixture("c8_shared_constraint", &d, "C8-III-TRUNCATION", census, vec![
        AXES_FROM_GEOMETRY.into(),
        "The holder set of the rigid constraint at node B, component UZ, is constructed: the reference's directional restraint SB holds it, and a rigid snubber SN-B, which the reference does not have, is named as a second holder. The constraint's reaction is the reference's SB.Fz, as published; one row names both holders and is not divided between them.".into(),
        "The member-axes contributions are the reference's global contributions mapped by the signed permutation of each member's axis-aligned frame (x = +X, y = +Z, z = -Y); the reference's convention 2 states that such a frame maps component to component. Each combined magnitude is formed from them.".into(),
        STAND_IN.into(),
    ], model, &reaction, &|s| s.to_string(), None, &[])
}

/// 6. C2-SPRING with every entity renamed to contain the characters a composed key would separate
/// its fields with.
const RENAMES: [(&str, &str); 7] = [
    ("N1", "N1.ux: anchor node"),
    ("N2", "N2 .i.Fx_m"),
    ("N3", "node:N3.uy_m tip"),
    ("P1", "P1.i.Fx_m"),
    ("P2", "pipe: P2.j.Mz_m"),
    ("A1", "A1.Fx: anchor_m"),
    ("S1", "S1.UY spring: 1"),
];

fn renamed(id: &str) -> String {
    RENAMES.iter().find(|(a, _)| *a == id).map_or_else(|| panic!("no rename of {id}"), |(_, b)| (*b).to_string())
}

fn original(id: &str) -> String {
    RENAMES.iter().find(|(_, b)| *b == id).map_or_else(|| panic!("no original of {id}"), |(a, _)| (*a).to_string())
}

fn c2_separator_identities() -> Fixture {
    let d = Discrete::new(C2, 0, "C2-SPRING");
    let mut members = axis_aligned_members(&d);
    for m in &mut members {
        m.id = renamed(&m.id);
        m.end_i = renamed(&m.end_i);
        m.end_j = renamed(&m.end_j);
    }
    let spring = &d.case["model"]["springs"][0];
    let census = Census {
        nodes: d.nodes().into_iter().map(|(n, _)| renamed(&n)).collect(),
        straight_members: members,
        rigid_constraints: d.rigid_constraints().into_iter().map(|c| RigidConstraint { node: renamed(&c.node), component: c.component,
            holders: c.holders.iter().map(|h| renamed(h)).collect() }).collect(),
        compliant_supports: vec![CompliantSupport { support: renamed(spring["id"].as_str().unwrap()), node: renamed(spring["node"].as_str().unwrap()),
            components: vec![dof_component(spring["dof"].as_str().unwrap())] }],
        ..Census::default()
    };
    let (od, wall) = d.section();
    let prov = "stand-in: discrete C2-GROUND-SPRING, every entity renamed";
    let nodes: Vec<(String, [f64; 3])> = d.nodes().into_iter().map(|(n, p)| (renamed(&n), p)).collect();
    let model = model_value("project:spectrum-c2-separators", prov, &nodes,
        d.members().iter().map(|(id, i, j, y)| segment(&renamed(id), &renamed(i), &renamed(j), od, wall, *y, prov)).collect(), vec![],
        vec![support_value(&renamed("A1"), &renamed("N1"), "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"], prov),
            json!({"id": renamed("S1"), "node": renamed("N3"), "family": "spring", "restraints": ["UY"],
                "stiffness": {"dof": "UY", "value": {"unit": "N/m", "value": spring["stiffness"]}}, "provenance": prov})],
        d.spectra_used());
    let reaction = |q: &Quantity| -> String {
        match q {
            Quantity::ConstraintReaction { component, holders, .. } => format!("{}.{}", original(&holders[0]), reaction_key(*component)),
            Quantity::CompliantSupportForce { support, component } => format!("{}.{}", original(support), reaction_key(*component)),
            _ => unreachable!(),
        }
    };
    discrete_fixture("c2_separator_identities", &d, "C2-SPRING", census, vec![
        format!("Every entity identity is constructed by renaming the reference's: {}. Each renamed identity contains a dot, a space, a colon or the member-axes suffix _m, the characters a composed key would separate its fields with; each global-axes value is the reference's value for the original entity.",
            RENAMES.iter().map(|(a, b)| format!("{a} as \"{b}\"")).collect::<Vec<_>>().join(", ")),
        AXES_FROM_GEOMETRY.into(),
        "The member-axes contributions are the reference's global contributions mapped by the signed permutation of each member's axis-aligned frame (x = +X, y = +Z, z = -Y); the reference's convention 2 states that such a frame maps component to component. Each combined magnitude is formed from them.".into(),
        STAND_IN.into(),
    ], model, &reaction, &original, None, &[])
}

/// The note of a document built on a grid entry; the entry has rigid responses only where the
/// correction is applied.
fn mutation_note(applied: bool) -> String {
    let own = if applied {
        "The contributions, rigid responses and modal basis are the analysis's own."
    } else {
        "The contributions and modal basis are the analysis's own; with the correction off the entry has no rigid response."
    };
    format!("The rules are those of one entry of the reference analysis's published mutations grid, not its base request's; \
the request stand-in states them. {own}")
}

fn global_reaction(q: &Quantity) -> String {
    match q {
        Quantity::ConstraintReaction { component, holders, .. } => format!("{}.{}", holders[0], reaction_key(*component)),
        Quantity::CompliantSupportForce { support, component } => format!("{support}.{}", reaction_key(*component)),
        other => unreachable!("{other:?}"),
    }
}

/// A discrete case's full census: every node, its axis-aligned straight members, its rigid
/// constraints and its springs.
fn full_census(d: &Discrete) -> Census {
    Census {
        nodes: d.nodes().into_iter().map(|(n, _)| n).collect(),
        straight_members: axis_aligned_members(d),
        rigid_constraints: d.rigid_constraints(),
        compliant_supports: d.case["model"]["springs"].as_array().unwrap().iter().map(|s| CompliantSupport {
            support: s["id"].as_str().unwrap().into(), node: s["node"].as_str().unwrap().into(),
            components: vec![dof_component(s["dof"].as_str().unwrap())] }).collect(),
        ..Census::default()
    }
}

/// The model stand-in of a discrete case: its nodes, members, anchors and springs.
fn discrete_model(d: &Discrete, project: &str, prov: &str) -> Value {
    let (od, wall) = d.section();
    let mut supports: Vec<Value> = Vec::new();
    let mut held: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    for r in d.case["derived_model"]["restrained_dofs"].as_array().unwrap() {
        let e = held.entry(r[2].as_str().unwrap().to_string()).or_insert((r[0].as_str().unwrap().to_string(), Vec::new()));
        e.1.push(dof_component(r[1].as_str().unwrap()).as_str().to_string());
    }
    for (id, (node, dofs)) in &held {
        let family = if dofs.len() == 6 { "anchor" } else { "guide" };
        supports.push(support_value(id, node, family, &dofs.iter().map(String::as_str).collect::<Vec<_>>(), prov));
    }
    for s in d.case["model"]["springs"].as_array().unwrap() {
        let c = dof_component(s["dof"].as_str().unwrap()).as_str();
        supports.push(json!({"id": s["id"], "node": s["node"], "family": "spring", "restraints": [c],
            "stiffness": {"dof": c, "value": {"unit": "N/m", "value": s["stiffness"]}}, "provenance": prov}));
    }
    model_value(project, prov, &d.nodes(), d.members().iter().map(|(id, i, j, y)| segment(id, i, j, od, wall, *y, prov)).collect(), vec![],
        supports, d.spectra_used())
}

/// 8. C3-BASE under the grid's GROUPING, rigid ABSOLUTE_SUM, 100-40-40 entry: a group of two
/// clusters, and a governing direction over three requested directions with the correction
/// applied.
fn c3_grouping_100_40_40() -> Fixture {
    let d = Discrete::new(C3, 0, "C3-BASE");
    let census = full_census(&d);
    let model = discrete_model(&d, "project:spectrum-c3", "stand-in: discrete C3-CANTILEVER-3D");
    discrete_fixture("c3_grouping_100_40_40", &d, "C3-BASE", census, vec![
        mutation_note(true),
        AXES_FROM_GEOMETRY.into(),
        "The member-axes contributions are the reference's global contributions mapped by the signed permutation of each member's axis-aligned frame; each combined magnitude is formed from them.".into(),
        STAND_IN.into(),
    ], model, &global_reaction, &|s| s.to_string(),
        Some((ModalRule::Grouping, Some(RigidRule::AbsoluteSum), DirectionalRule::HundredFortyForty)), &[])
}

/// 9. C9-CHAIN-Y under the ten percent method: the chain of modes 21, 22 and 23, where the ten
/// percent method couples 22 with 23 and the grouping method does not.
fn c9_chain_ten_percent() -> Fixture {
    let d = Discrete::new(C9, 1, "C9-CHAIN-Y");
    let spring = &d.case["model"]["springs"][0];
    let census = Census {
        nodes: vec!["A".into(), "B".into()],
        rigid_constraints: d.rigid_constraints(),
        compliant_supports: vec![CompliantSupport { support: spring["id"].as_str().unwrap().into(), node: spring["node"].as_str().unwrap().into(),
            components: vec![dof_component(spring["dof"].as_str().unwrap())] }],
        ..Census::default()
    };
    assert!(census.rigid_constraints.iter().all(|c| c.node == "A") && census.rigid_constraints.len() == 6);
    let model = discrete_model(&d, "project:spectrum-c9-chain", "stand-in: discrete C9-CHAIN, reduced to the quantities it publishes per mode");
    discrete_fixture("c9_chain_ten_percent", &d, "C9-CHAIN-Y", census, vec![
        mutation_note(false),
        "The roster and census are a subset of the reference model: the tip node B, the anchor SA's six constraint reactions at A and the spring S1's force, the quantities the reference publishes per mode, and the node A. The nodes M1 to M7 and the members P1 to P8 are in the model and have no rows here.".into(),
        "A's six displacement and rotation rows are constructed: each contribution is zero because all six components at A are rigidly restrained by the anchor SA (REQ-006, Nodes: a component restrained on a global axis is zero), not because the reference publishes them.".into(),
        STAND_IN.into(),
    ], model, &global_reaction, &|s| s.to_string(), Some((ModalRule::TenPercent, None, DirectionalRule::Srss)), &["A"])
}

/// 10. C8-III-TRUNCATION under the grid's all-absolute-sum entry: the absolute-sum modal, rigid
/// and directional rules over Y and Z.
fn c8_absolute_sums() -> Fixture {
    let d = Discrete::new(C8, 0, "C8-III-TRUNCATION");
    let census = full_census(&d);
    let model = discrete_model(&d, "project:spectrum-c8-absolute-sums", "stand-in: discrete C8-PIPE-X");
    discrete_fixture("c8_absolute_sums", &d, "C8-III-TRUNCATION", census, vec![
        mutation_note(true),
        AXES_FROM_GEOMETRY.into(),
        "The member-axes contributions are the reference's global contributions mapped by the signed permutation of each member's axis-aligned frame (x = +X, y = +Z, z = -Y); the reference's convention 2 states that such a frame maps component to component. Each combined magnitude is formed from them.".into(),
        STAND_IN.into(),
    ], model, &global_reaction, &|s| s.to_string(),
        Some((ModalRule::AbsoluteSum, Some(RigidRule::AbsoluteSum), DirectionalRule::AbsoluteSum)), &[])
}

fn fixtures() -> Vec<Fixture> {
    vec![
        c9_y_two_frames(),
        c7_cancelling_cluster(),
        w2_arc_fixture("w2_arc_oblique", false),
        w2_joint_rods(),
        c8_shared_constraint(),
        c2_separator_identities(),
        w2_arc_fixture("refused_arc_member_axes", true),
        c3_grouping_100_40_40(),
        c9_chain_ten_percent(),
        c8_absolute_sums(),
    ]
}

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results")
}

/// The deterministic generation of the committed documents, their inputs and the semantic table
/// (run explicitly; `--ignored`).
#[test]
#[ignore = "writes the committed spectrum documents; run explicitly to regenerate them"]
fn write_spectrum_documents() {
    let root = root();
    std::fs::write(root.join("semantic_contract_spectrum_1.json"), fixture_text(&spectrum_semantic_table())).unwrap();
    for f in fixtures() {
        std::fs::write(root.join(format!("spectrum_{}.json", f.name)), fixture_text(&f.document())).unwrap();
        std::fs::write(root.join(format!("spectrum_{}.model.json", f.name)), fixture_text(&f.model)).unwrap();
        std::fs::write(root.join(format!("spectrum_{}.request.json", f.name)), fixture_text(&f.request)).unwrap();
    }
}

fn committed(name: &str) -> (&'static str, &'static str, &'static str) {
    COMMITTED.iter().find(|c| c.0 == name).map(|c| (c.1, c.2, c.3)).unwrap_or_else(|| panic!("no committed {name}"))
}

fn document(name: &str) -> Value {
    serde_json::from_str(committed(name).0).unwrap()
}

#[test]
fn the_semantic_table_is_the_committed_one() {
    let committed_table: Value = serde_json::from_str(SEMANTIC_TABLE).unwrap();
    assert_eq!(spectrum_semantic_table(), committed_table);
    assert_eq!(fixture_text(&spectrum_semantic_table()), SEMANTIC_TABLE);
    for (name, ..) in COMMITTED {
        assert_eq!(document(name)["semantic_contract"]["sha256"].as_str(), spectrum_doc_sha256(&committed_table).as_deref(), "{name}");
    }
}

#[test]
fn documents_are_the_projection_of_their_inputs() {
    let all = fixtures();
    assert_eq!(all.len(), COMMITTED.len());
    for f in &all {
        let (doc, model, request) = committed(f.name);
        assert_eq!(fixture_text(&f.model), model, "{}: the committed model stand-in", f.name);
        assert_eq!(fixture_text(&f.request), request, "{}: the committed request", f.name);
        let d = f.document();
        assert!(fixture_text(&d) == doc, "{}: the committed document is the projection of its inputs (regenerate with write_spectrum_documents)", f.name);
        assert_eq!(f.document(), d, "{}: deterministic", f.name);
        // Each document binds the committed model and request bytes' values.
        let m: Value = serde_json::from_str(model).unwrap();
        let r: Value = serde_json::from_str(request).unwrap();
        assert_eq!(d["bindings"]["model"]["sha256"].as_str(), spectrum_doc_sha256(&m).as_deref());
        assert_eq!(d["bindings"]["request"]["sha256"].as_str(), spectrum_doc_sha256(&r).as_deref());
        assert_eq!(d["origin"]["kind"], "constructed_from_reference");
        assert_eq!(d["origin"]["produced_by_spectrum_solve"], false);
        assert_eq!(d["modal_result"]["binding"], "not_applicable");
        assert_eq!(d["admission"], json!({"status": "experimental", "public_engineering_result": false}));
    }
}

#[test]
fn every_document_states_its_outcome() {
    for (name, ..) in COMMITTED {
        let d = document(name);
        if name.starts_with("refused") {
            assert_eq!(d["status"], "refused");
            let rows = d["refusals"]["rows"].as_array().unwrap();
            assert_eq!(rows.len(), 12, "every arc end row stated in member axes");
            for r in rows {
                assert_eq!(r["code"], "SPECTRUM_ROW_INVALID");
                assert_eq!(r["field"], "frame");
                assert!(r["detail"].as_str().unwrap().contains("member_axes"));
                assert!(r["object"].as_str().unwrap().contains("[BEND1]"));
            }
        } else {
            assert_eq!(d["status"], "solved", "{name}");
            let signed = d["rows"]["signed"]["rows"].as_array().unwrap();
            let combined = d["rows"]["combined"]["rows"].as_array().unwrap();
            assert_eq!(signed.len(), combined.len());
            assert!(signed.iter().all(|r| r["sign_convention"].is_string() && r["sign_status"] == "signed_contribution"));
            assert!(combined.iter().all(|r| r.get("sign_convention").is_none() && r["sign_status"] == "unsigned_combined_magnitude"));
        }
    }
}

/// The row of `document` in table `table` whose identity fields match.
fn row<'a>(document: &'a Value, table: &str, fields: &[(&str, Value)]) -> &'a Value {
    let rows = document["rows"][table]["rows"].as_array().unwrap();
    let found: Vec<&Value> = rows.iter().filter(|r| fields.iter().all(|(k, v)| &r[*k] == v)).collect();
    assert_eq!(found.len(), 1, "{fields:?}");
    found[0]
}

fn column(document: &Value, mode: u32) -> usize {
    document["rows"]["contribution_columns"]["rows"].as_array().unwrap().iter()
        .position(|c| c["mode_ref"] == format!("mode:{mode}")).unwrap()
}

/// `a` rounds to the six significant figures quoted as `b`.
fn close(a: f64, b: f64) -> bool {
    format!("{a:.5e}") == format!("{b:.5e}")
}

/// REQ-006's worked rows, quoted to six figures, as the documents carry them.
#[test]
fn the_documents_carry_req_006_worked_rows() {
    let c9 = document("c9_y_two_frames");
    let member = |frame: &str| row(&c9, "signed", &[("member", json!("P1")), ("end", json!("i")), ("component", json!("Fx")), ("frame", json!(frame))]).clone();
    let combined = |frame: &str| row(&c9, "combined", &[("member", json!("P1")), ("end", json!("i")), ("component", json!("Fx")), ("frame", json!(frame))]).clone();
    let m2 = column(&c9, 2);
    let v = |r: &Value| r["periodic"]["Y"][m2].as_f64().unwrap();
    assert!(close(v(&member("member_axes")), -2.19139), "{}", v(&member("member_axes")));
    assert!(close(combined("member_axes")["combined"].as_f64().unwrap(), 2.19160));
    assert!(close(v(&member("global_axes")), 37.9929));
    assert!(close(combined("global_axes")["combined"].as_f64().unwrap(), 40.3434));

    let arc = document("w2_arc_oblique");
    let m1 = column(&arc, 1);
    for (frame, contribution, rigid, magnitude) in [("arc_tangent_frame", 0.0, -6.87748, 85.9727), ("global_axes", 110.057, -50.3015, 289.879)] {
        let fields = [("member", json!("BEND1")), ("end", json!("i")), ("component", json!("Fy")), ("frame", json!(frame))];
        let s = row(&arc, "signed", &fields);
        let c = row(&arc, "combined", &fields);
        let x = s["periodic"]["X"][m1].as_f64().unwrap();
        if contribution == 0.0 {
            assert_eq!(x, 0.0, "exactly 0");
        } else {
            assert!(close(x, contribution), "{x}");
        }
        assert!(close(s["rigid"]["X"].as_f64().unwrap(), rigid));
        assert!(close(c["combined"].as_f64().unwrap(), magnitude));
        assert_eq!(s["member_family"], "arc");
    }
}

/// The C7 document's cancellation: cluster 1's two contributions to N3.uz under Y are equal and
/// opposite, its stated sum is exactly zero, and with cluster 2's exact zero sum the Y magnitude is
/// an exact zero.
#[test]
fn the_cancelling_cluster_combines_to_an_exact_zero() {
    let d = document("c7_cancelling_cluster");
    let fields = [("node", json!("N3")), ("component", json!("uz"))];
    let s = row(&d, "signed", &fields);
    let (a, b) = (s["periodic"]["Y"][column(&d, 1)].as_f64().unwrap(), s["periodic"]["Y"][column(&d, 2)].as_f64().unwrap());
    assert!(a != 0.0 && a == -b, "{a} {b}");
    let sums = d["rows"]["cluster_sums"]["rows"].as_array().unwrap().iter().find(|r| r["quantity_ref"] == s["row_id"]).unwrap();
    assert_eq!(sums["sums"]["Y"], json!([0.0, 0.0]));
    let c = row(&d, "combined", &fields);
    let y = c["per_direction"].as_array().unwrap().iter().find(|p| p["direction"] == "Y").unwrap();
    assert_eq!(y["combined"].as_f64(), Some(0.0));
    assert!(c["combined"].as_f64().unwrap() > 0.0, "Z excites it");
}

/// In the documents built on a reference's base request, each combined magnitude for which the
/// reference publishes a combined value (and, where it publishes them, each per-direction value)
/// agrees with it at REQ-008's verification tolerance (1e-9 relative, an expected zero exactly).
/// Member-axes rows other than C9's have no reference value and are not compared here.
#[test]
fn constructed_documents_agree_with_their_references() {
    let check = |d: &Value, lookup: &dyn Fn(&Value) -> Option<(f64, Vec<(String, f64)>)>| -> usize {
        let mut n = 0;
        for r in d["rows"]["combined"]["rows"].as_array().unwrap() {
            let Some((combined, per)) = lookup(r) else { continue };
            let ok = |a: f64, e: f64| if e == 0.0 { a == 0.0 } else { (a - e).abs() <= TOLERANCE * e.abs() };
            assert!(ok(r["combined"].as_f64().unwrap(), combined), "{r}");
            for (dir, e) in per {
                let p = r["per_direction"].as_array().unwrap().iter().find(|p| p["direction"] == dir.as_str()).unwrap();
                assert!(ok(p["combined"].as_f64().unwrap(), e), "{dir} {r}");
            }
            n += 1;
        }
        n
    };
    // W2: arc global and tangent rows, nodes, anchors, connector and rods.
    for (name, text) in [("w2_arc_oblique", W2_OBLIQUE), ("w2_joint_rods", W2_JOINT_R)] {
        let w = W2::new(text);
        let a = w.analysis();
        let d = document(name);
        let key = |r: &Value| -> Option<String> {
            let s = |k: &str| r[k].as_str().unwrap().to_string();
            Some(match r["quantity_kind"].as_str().unwrap() {
                "node_displacement" => format!("{}.{}", s("node"), s("component")),
                "element_end_action" => match r["frame"].as_str().unwrap() {
                    "global_axes" => format!("{}.{}.{}", s("member"), s("end"), s("component")),
                    "arc_tangent_frame" => format!("{}.{}.{}_t", s("member"), s("end"), s("component")),
                    _ => return None,
                },
                "connector_generalized_force" => format!("{}.g.{}", s("connector"), ["N", "Vy", "Vz", "T", "My", "Mz"][
                    ["axial", "lateral_y", "lateral_z", "torsion", "bending_y", "bending_z"].iter().position(|c| *c == s("component")).unwrap()]),
                "connector_end_action" => format!("{}.{}.{}", s("connector"), s("end"), s("component")),
                "tie_rod_force" => format!("{}.T", s("rod")),
                "constraint_reaction" => format!("{}.{}", s("node"), reaction_key(ReactionComponent::parse(&s("component")).unwrap())),
                _ => return None,
            })
        };
        let n = check(&d, &|r| {
            let k = key(r)?;
            Some((real(&a["combined"][&k]), ["X", "Y", "Z"].iter().map(|dir| (dir.to_string(), real(&a["per_direction_combined"][*dir][&k]))).collect()))
        });
        assert_eq!(n, non_member_axes(&d), "{name}: every row but the straights' member-axes rows has a reference value");
        assert_eq!(n, expected_compared(name), "{name}");
    }
    // Discrete: C9-Y in both frames; C8-III, C2 (renamed) and C7 (reduced) in global axes.
    let c9 = Discrete::new(C9, 0, "C9-Y");
    let d = document("c9_y_two_frames");
    let n = check(&d, &|r| {
        let s = |k: &str| r[k].as_str().unwrap().to_string();
        let (block, key) = match r["quantity_kind"].as_str().unwrap() {
            "node_displacement" => (&c9.analysis, format!("{}.{}", s("node"), s("component"))),
            "element_end_action" if r["frame"] == "member_axes" => (&c9.analysis["element_end_forces_member_axes"], format!("{}.{}.{}_m", s("member"), s("end"), s("component"))),
            "element_end_action" => (&c9.analysis, format!("{}.{}.{}", s("member"), s("end"), s("component"))),
            "constraint_reaction" => (&c9.analysis, format!("{}.{}", r["holders"][0].as_str().unwrap(), reaction_key(ReactionComponent::parse(&s("component")).unwrap()))),
            "compliant_support_force" => (&c9.analysis, format!("{}.{}", s("support"), reaction_key(ReactionComponent::parse(&s("component")).unwrap()))),
            _ => return None,
        };
        Some((expected(&block["combined"][&key]), vec![("Y".into(), expected(&block["per_direction_combined"]["Y"][&key]))]))
    });
    assert_eq!(n, d["rows"]["combined"]["row_count"].as_u64().unwrap() as usize);
    assert_eq!(n, expected_compared("c9_y_two_frames"));
    for (name, text, analysis, names) in [("c8_shared_constraint", C8, "C8-III-TRUNCATION", false), ("c2_separator_identities", C2, "C2-SPRING", true),
        ("c7_cancelling_cluster", C7, "C7-PAIRS", false)] {
        let c = Discrete::new(text, 0, analysis);
        let d = document(name);
        let id = |s: &str| if names { original(s) } else { s.to_string() };
        let n = check(&d, &|r| {
            let s = |k: &str| r[k].as_str().unwrap().to_string();
            let key = match r["quantity_kind"].as_str().unwrap() {
                "node_displacement" => format!("{}.{}", id(&s("node")), s("component")),
                "element_end_action" if r["frame"] == "global_axes" => format!("{}.{}.{}", id(&s("member")), s("end"), s("component")),
                "constraint_reaction" => format!("{}.{}", id(r["holders"][0].as_str().unwrap()), reaction_key(ReactionComponent::parse(&s("component")).unwrap())),
                "compliant_support_force" => format!("{}.{}", id(&s("support")), reaction_key(ReactionComponent::parse(&s("component")).unwrap())),
                _ => return None,
            };
            let dirs: Vec<(String, f64)> = c.directions().iter().map(|(dir, _, _)| (dir.as_str().to_string(), expected(&c.analysis["per_direction_combined"][dir.as_str()][&key]))).collect();
            Some((expected(&c.analysis["combined"][&key]), dirs))
        });
        assert_eq!(n, non_member_axes(&d), "{name}: every row but the member-axes rows has a reference value");
        assert_eq!(n, expected_compared(name), "{name}");
    }
}

/// The number of combined rows each document compares with its reference, stated here so that a
/// dropped row cannot pass: the roster's rows less the straights' member-axes rows (all rows for
/// C9-Y, whose reference publishes both frames).
fn expected_compared(name: &str) -> usize {
    match name {
        "w2_arc_oblique" => 84,
        "w2_joint_rods" => 80,
        "c9_y_two_frames" => 253,
        "c8_shared_constraint" => 157,
        "c2_separator_identities" => 49,
        "c7_cancelling_cluster" => 18,
        other => panic!("no stated count for {other}"),
    }
}

fn non_member_axes(d: &Value) -> usize {
    d["rows"]["combined"]["rows"].as_array().unwrap().iter().filter(|r| r["frame"] != "member_axes").count()
}

/// Where a straight member's frame is a signed permutation of the global axes, each member-axes
/// magnitude is exactly the global magnitude of the component its axis maps to: one sign flips
/// every contribution of a row together, which no combination sees.
#[test]
fn axis_aligned_member_axes_rows_are_the_permuted_global_rows() {
    for name in ["w2_joint_rods", "c8_shared_constraint", "c2_separator_identities", "c3_grouping_100_40_40", "c8_absolute_sums"] {
        let d = document(name);
        let members = d["census"]["straight_members"]["rows"].as_array().unwrap();
        let combined = d["rows"]["combined"]["rows"].as_array().unwrap();
        let mut n = 0;
        for r in combined.iter().filter(|r| r["frame"] == "member_axes") {
            let m = members.iter().find(|m| m["member_id"] == r["member"]).unwrap();
            let c = ForceComponent::parse(r["component"].as_str().unwrap()).unwrap();
            let (base, axis) = triple(c);
            let row: Vec<f64> = m["member_axes"][["x", "y", "z"][axis]].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            let k = row.iter().position(|v| *v != 0.0).unwrap();
            assert_eq!(row[k].abs(), 1.0);
            let global = combined.iter().find(|g| g["frame"] == "global_axes" && g["member"] == r["member"] && g["end"] == r["end"]
                && g["component"] == FORCES[base + k].as_str()).unwrap();
            assert_eq!(r["combined"], global["combined"], "{name} {r}");
            assert_eq!(r["per_direction"], global["per_direction"], "{name} {r}");
            n += 1;
        }
        assert_eq!(n, members.len() * 12, "{name}");
    }
}

fn refusal_details(d: &Value) -> Vec<(String, String)> {
    d["refusals"]["rows"].as_array().map_or_else(Vec::new, |rows| {
        rows.iter().map(|r| (r["code"].as_str().unwrap().to_string(), r["detail"].as_str().unwrap().to_string())).collect()
    })
}

/// C7's cluster 1 is an exactly repeated pair (equal circular frequencies). Split into two
/// clusters, with the contributions unchanged, its stated clusters contradict the cluster rule on
/// the stated circular frequencies, and the assembler refuses the document naming both modes.
#[test]
fn a_split_of_an_exactly_repeated_cluster_is_refused() {
    let mut f = c7_cancelling_cluster();
    assert_eq!(f.modal.circular_frequency_rad_s[0], f.modal.circular_frequency_rad_s[1]);
    for m in &mut f.combination.modes {
        if m.mode.0 >= 2 {
            m.cluster = ClusterIndex(m.cluster.0 + 1);
        }
    }
    let d = f.document();
    assert_eq!(d["status"], "refused");
    let refusals = refusal_details(&d);
    assert!(refusals.iter().any(|(code, detail)| code == "SPECTRUM_DOCUMENT_INPUT_INVALID"
        && detail == "mode:1 and mode:2 are split within the cluster tolerance"), "{refusals:?}");
    assert_eq!(refusals.iter().filter(|(_, detail)| detail.contains("cluster tolerance")).count(), 1, "{refusals:?}");
}

/// A pair whose gap lies within the margin of the threshold is neither refused shared nor refused
/// split, and with its frequency set consistently with its circular frequency the document is
/// solved; a pair clearly above the threshold is refused when shared.
#[test]
fn a_pair_inside_the_margin_is_not_refused() {
    let wb = c7_cancelling_cluster().modal.circular_frequency_rad_s[1];
    let delta = CLUSTER_TOLERANCE;
    let two_pi = 2.0 * std::f64::consts::PI;
    // ω_a with ω_b² − ω_a² = δ_c·ω_b², rounded: its gap is within a few u·ω_b² of the threshold.
    let wa = wb * (1.0 - delta).sqrt();
    let (gap, threshold) = (wb * wb - wa * wa, delta * (wb * wb));
    let margin = 16.0 * f64::EPSILON / 2.0 * (1.0 + delta) * (wb * wb);
    assert!((gap - threshold).abs() < margin, "inside the margin");
    let with_mode_1 = |w: f64, split: bool| {
        let mut f = c7_cancelling_cluster();
        f.modal.circular_frequency_rad_s[0] = w;
        f.combination.modes[0].frequency_hz = w / two_pi;
        if split {
            for m in &mut f.combination.modes {
                if m.mode.0 >= 2 {
                    m.cluster = ClusterIndex(m.cluster.0 + 1);
                }
            }
        }
        f.document()
    };
    for split in [false, true] {
        let d = with_mode_1(wa, split);
        assert_eq!(d["status"], "solved", "split {split}: {:?}", refusal_details(&d));
    }
    let refusals = refusal_details(&with_mode_1(wb * (1.0 - 2.0 * delta).sqrt(), false));
    assert_eq!(refusals, vec![("SPECTRUM_DOCUMENT_INPUT_INVALID".to_string(),
        "mode:1 and mode:2 share a cluster across a relative gap above the cluster tolerance".to_string())], "{refusals:?}");
}

/// The documents built on a grid entry of the reference's published `mutations`: (name, source
/// text, case index, analysis, modal, rigid (`OFF` for the correction off), directional).
const MUTATION_DOCUMENTS: [(&str, &str, usize, &str, &str, &str, &str); 3] = [
    ("c3_grouping_100_40_40", C3, 0, "C3-BASE", "GROUPING", "ABSOLUTE_SUM", "100-40-40"),
    ("c9_chain_ten_percent", C9, 1, "C9-CHAIN-Y", "TEN_PERCENT", "OFF", "SRSS"),
    ("c8_absolute_sums", C8, 0, "C8-III-TRUNCATION", "ABSOLUTE_SUM", "ABSOLUTE_SUM", "ABSOLUTE_SUM"),
];

/// Each combined magnitude over the directions that the grid entry publishes agrees with it at
/// 1e-9 relative (an expected zero exactly). The grid publishes no per-direction value and no
/// member-axes value; the constructed zero rows of an anchored node have none either.
#[test]
fn mutation_documents_agree_with_their_grid_entries() {
    for (name, text, case, analysis, modal, rigid, directional) in MUTATION_DOCUMENTS {
        let d = Discrete::new(text, case, analysis);
        let entry = d.analysis["mutations"].as_array().unwrap().iter()
            .find(|m| m["modal_combination"] == modal && m["missing_mass"] == rigid && m["directional_combination"] == directional)
            .unwrap_or_else(|| panic!("{name}: the grid entry"));
        let published = entry["combined"].as_object().unwrap();
        let doc = document(name);
        let mut compared = 0;
        for r in doc["rows"]["combined"]["rows"].as_array().unwrap() {
            let s = |k: &str| r[k].as_str().unwrap().to_string();
            let key = match r["quantity_kind"].as_str().unwrap() {
                "node_displacement" => format!("{}.{}", s("node"), s("component")),
                "element_end_action" if r["frame"] == "global_axes" => format!("{}.{}.{}", s("member"), s("end"), s("component")),
                "element_end_action" => continue,
                "constraint_reaction" => format!("{}.{}", r["holders"][0].as_str().unwrap(), reaction_key(ReactionComponent::parse(&s("component")).unwrap())),
                "compliant_support_force" => format!("{}.{}", s("support"), reaction_key(ReactionComponent::parse(&s("component")).unwrap())),
                other => panic!("{other}"),
            };
            let Some(e) = published.get(&key) else {
                assert_eq!((name, s("node").as_str()), ("c9_chain_ten_percent", "A"), "{name}: {key} has no published value");
                continue;
            };
            let (a, e) = (r["combined"].as_f64().unwrap(), expected(e));
            assert!(if e == 0.0 { a == 0.0 } else { (a - e).abs() <= TOLERANCE * e.abs() }, "{name} {key}: {a:e} against {e:e}");
            compared += 1;
        }
        assert_eq!(compared, published.len(), "{name}: every published magnitude is compared");
    }
}

fn cluster_refs(v: &Value, key: &str) -> Vec<String> {
    v[key].as_array().unwrap().iter().map(|c| c.as_str().unwrap().to_string()).collect()
}

/// The grouping method's groups are the reference's (`grouping_groups.periodic`, each mode its own
/// cluster), one of them of two clusters; and with 100-40-40 over three directions every row with
/// a nonzero magnitude names its governing direction.
#[test]
fn the_grouping_document_states_the_reference_groups_and_governing_directions() {
    let doc = document("c3_grouping_100_40_40");
    let d = Discrete::new(C3, 0, "C3-BASE");
    let reference: Vec<Vec<String>> = d.analysis["grouping_groups"]["periodic"].as_array().unwrap().iter()
        .map(|g| g.as_array().unwrap().iter().map(|c| format!("cluster:{}", d.cluster_of(c[0].as_u64().unwrap()))).collect()).collect();
    let groups: Vec<Vec<String>> = doc["rule_statements"]["groups"]["rows"].as_array().unwrap().iter().map(|g| cluster_refs(g, "cluster_refs")).collect();
    assert_eq!(groups, reference);
    assert!(groups.iter().any(|g| g.len() > 1));
    assert!(doc["rule_statements"]["coupled_pairs"].is_null() && doc["rule_statements"]["correlation"].is_null());
    assert_eq!(doc["rows"]["directions"], json!(["X", "Y", "Z"]));
    assert_eq!(doc["request"]["missing_mass"]["rigid_combination"], "ABSOLUTE_SUM");
    // Each row's governing direction is the direction of its largest published directional
    // magnitude, the first in X, Y, Z order on a tie, zero rows included.
    let combined = doc["rows"]["combined"]["rows"].as_array().unwrap();
    let mut zero_rows = 0;
    for r in combined {
        let parts = r["per_direction"].as_array().unwrap();
        let mut expected = 0;
        for (i, p) in parts.iter().enumerate() {
            if p["combined"].as_f64().unwrap() > parts[expected]["combined"].as_f64().unwrap() {
                expected = i;
            }
        }
        assert_eq!(r["governing_direction"], parts[expected]["direction"], "{r}");
        if parts.iter().all(|p| p["combined"].as_f64() == Some(0.0)) {
            assert_eq!(r["governing_direction"], "X", "an all-zero row: the first direction {r}");
            zero_rows += 1;
        }
    }
    assert!(zero_rows > 0, "the document has rows that are zero in every direction");
    let named: BTreeSet<&str> = combined.iter().filter_map(|r| r["governing_direction"].as_str()).collect();
    assert!(named.len() > 1, "more than one direction governs: {named:?}");
}

/// The ten percent method couples the chain's 21 with 22 and 22 with 23, while the reference's
/// grouping method groups 21 with 22 and starts a new group at 23: coupled pairs are not groups.
#[test]
fn the_ten_percent_document_couples_a_chain_the_groups_split() {
    let doc = document("c9_chain_ten_percent");
    let d = Discrete::new(C9, 1, "C9-CHAIN-Y");
    let pairs: Vec<(String, String)> = doc["rule_statements"]["coupled_pairs"]["rows"].as_array().unwrap().iter()
        .map(|p| (p["first_cluster_ref"].as_str().unwrap().to_string(), p["second_cluster_ref"].as_str().unwrap().to_string())).collect();
    let c = |mode: u64| format!("cluster:{}", d.cluster_of(mode));
    assert!(pairs.contains(&(c(21), c(22))) && pairs.contains(&(c(22), c(23))), "{pairs:?}");
    assert!(!pairs.contains(&(c(21), c(23))), "{pairs:?}");
    let groups = &d.analysis["grouping_groups"]["periodic"];
    assert!(groups.as_array().unwrap().contains(&json!([[21], [22]])) && groups.as_array().unwrap().contains(&json!([[23]])));
    assert!(doc["rule_statements"]["groups"].is_null());
    assert_eq!(doc["census"]["nodes"]["rows"].as_array().unwrap().iter().map(|r| r["node_id"].clone()).collect::<Vec<_>>(), [json!("A"), json!("B")]);
}

fn has_refusal(d: &Value, detail: &str) -> bool {
    refusal_details(d).iter().any(|(code, x)| code == "SPECTRUM_DOCUMENT_INPUT_INVALID" && x.contains(detail))
}

/// A census frame that is not orthonormal and right-handed to 2^-40, a rod axis that is not a unit
/// vector to 2^-40, an entity that occurs twice and a holder named twice are each refused.
#[test]
fn census_frames_and_entries_are_checked() {
    let mut f = c9_y_two_frames();
    f.census.straight_members[0].axes[0][0] += 2f64.powi(-30);
    assert!(has_refusal(&f.document(), "straight member P1 member axes: |x.x - 1|"));
    let mut f = w2_arc_fixture("w2_arc_oblique", false);
    f.census.arcs[0].tangent_frame_j[2] = f.census.arcs[0].tangent_frame_j[2].map(|v| -v);
    assert!(has_refusal(&f.document(), "arc BEND1 tangent frame at end j: |(x cross y - z)_"));
    let mut f = w2_joint_rods();
    f.census.tie_rods[1].axis = [0.0, 1.0 + 2f64.powi(-30), 0.0];
    assert!(has_refusal(&f.document(), "tie rod R2 of connector J1: |a.a - 1|"));
    let mut f = c8_shared_constraint();
    let first = f.census.nodes[0].clone();
    f.census.nodes.push(first.clone());
    let b_uz = f.census.rigid_constraints.iter_mut().find(|c| c.node == "B" && c.component == ReactionComponent::Uz).unwrap();
    b_uz.holders.push("SB".into());
    let d = f.document();
    assert!(has_refusal(&d, &format!("node {first} occurs more than once in the census nodes table")));
    assert!(has_refusal(&d, "rigid constraint at B UZ names holder SB twice"));
}

/// A circular frequency that is not 2π times its frequency within 8u is refused; the committed
/// documents' published pairs pass.
#[test]
fn frequency_and_circular_frequency_are_checked() {
    let mut f = c7_cancelling_cluster();
    f.modal.circular_frequency_rad_s[2] *= 1.0 + 1e-12;
    assert!(has_refusal(&f.document(), "mode:3: its circular frequency"));
    let mut f = w2_arc_fixture("w2_arc_oblique", false);
    let next = f.modal.extraction.next_computed_circular_frequency_rad_s.unwrap();
    f.modal.extraction.next_computed_circular_frequency_rad_s = Some(next * (1.0 + 1e-12));
    assert!(has_refusal(&f.document(), "the next computed mode: its circular frequency"));
}

/// A bound modal result as the modal family states one: its run id formed from its request and
/// integrity-parameter digests.
fn bound_binding() -> ModalResultBinding {
    let (request, parameters) = ("a".repeat(64), "b".repeat(64));
    let run = spectrum_doc_sha256(&json!({"request_sha256": request, "integrity_parameters_sha256": parameters})).unwrap();
    ModalResultBinding::Bound {
        run_id: format!("modal-run:{}", &run[..16]), request_sha256: request, model_sha256: None, materials_sha256: "c".repeat(64),
        integrity_parameters_sha256: parameters, state_case: None,
    }
}

fn refusal_fields(d: &Value) -> Vec<(String, String, Option<String>)> {
    d["refusals"]["rows"].as_array().map_or_else(Vec::new, |rows| rows.iter().map(|r| (r["code"].as_str().unwrap().to_string(),
        r["object"].as_str().unwrap().to_string(), r["field"].as_str().map(str::to_string))).collect())
}

/// A spectrum solve's document binds its modal result, and a constructed document binds none.
#[test]
fn origin_and_modal_binding_are_checked() {
    let f = c7_cancelling_cluster();
    let not_applicable = ModalResultBinding::NotApplicable { reason: "none".into() };
    for (origin, binding, detail) in [
        (&f.origin, &bound_binding(), "a document constructed from a reference binds a modal result"),
        (&Origin::SpectrumSolve, &not_applicable, "a document produced by a spectrum solve states its modal result binding as not applicable"),
    ] {
        let d = f.document_with(origin, binding);
        assert_eq!(d["status"], "refused");
        assert_eq!(refusal_details(&d), vec![("SPECTRUM_DOCUMENT_INPUT_INVALID".to_string(), detail.to_string())]);
    }
}

/// A bound modal result's digests, run id and state case, and an unbound one's reason, are
/// checked: each failure is refused naming the field.
#[test]
fn modal_result_binding_fields_are_checked() {
    let f = c7_cancelling_cluster();
    let solve = Origin::SpectrumSolve;
    let ModalResultBinding::Bound { run_id, request_sha256, materials_sha256, integrity_parameters_sha256, .. } = bound_binding() else { unreachable!() };
    let bound = |run_id: &str, request: &str, model: Option<&str>, state: Option<(&str, usize, &str)>| ModalResultBinding::Bound {
        run_id: run_id.into(), request_sha256: request.into(), model_sha256: model.map(str::to_string), materials_sha256: materials_sha256.clone(),
        integrity_parameters_sha256: integrity_parameters_sha256.clone(), state_case: state.map(|(a, b, c)| (a.into(), b, c.into())),
    };
    let fields = |b: &ModalResultBinding| -> Vec<(String, String, Option<String>)> { refusal_fields(&f.document_with(&solve, b)) };
    let invalid = |field: &str| ("SPECTRUM_DOCUMENT_INPUT_INVALID".to_string(), "modal_result".to_string(), Some(field.to_string()));
    let sha = "d".repeat(64);
    // The valid binding passes these checks; the document then publishes.
    assert!(fields(&bound(&run_id, &request_sha256, Some(sha.as_str()), Some(("case:1", 1, sha.as_str())))).is_empty());
    assert_eq!(fields(&bound("modal-run:0000000000000000", &request_sha256, None, None)), vec![invalid("run_id")]);
    assert_eq!(fields(&bound(" ", &request_sha256, None, None)), vec![invalid("run_id")]);
    assert_eq!(fields(&bound(&run_id, &"A".repeat(64), None, None)), vec![invalid("request_sha256")]);
    assert_eq!(fields(&bound(&run_id, &request_sha256, Some("short"), None)), vec![invalid("model_sha256")]);
    assert_eq!(fields(&bound(&run_id, &request_sha256, None, Some(("", 0, "x")))), vec![invalid("state_case"), invalid("state_case")]);
    assert_eq!(fields(&bound(&run_id, &request_sha256, None, Some(("case:1", 1 << 53, sha.as_str())))),
        vec![("SPECTRUM_DOCUMENT_OUTSIDE_CANONICAL_PROFILE".to_string(), "modal_result".to_string(), Some("state_case".to_string()))]);
    let blank = ModalResultBinding::NotApplicable { reason: "  ".into() };
    assert_eq!(refusal_fields(&f.document_with(&f.origin, &blank)), vec![invalid("reason")]);
}

/// A census that names a node or a connector it does not hold is refused, naming the entry and
/// what is missing.
#[test]
fn a_census_not_closed_on_itself_is_refused() {
    let mut f = c7_cancelling_cluster();
    f.census.nodes.retain(|n| n != "N1");
    let refusals = refusal_details(&f.document());
    for c in ["UX", "UY", "UZ", "RX", "RY", "RZ"] {
        let expected = format!("rigid constraint at N1 {c} names node N1, which is not a census node");
        assert!(refusals.iter().any(|(code, d)| code == "SPECTRUM_DOCUMENT_INPUT_INVALID" && *d == expected), "{expected}: {refusals:?}");
    }
    let mut f = w2_joint_rods();
    f.census.connectors.clear();
    let refusals = refusal_details(&f.document());
    for rod in ["R1", "R2"] {
        let expected = format!("tie rod {rod} names connector J1, which is not a census connector");
        assert!(refusals.iter().any(|(_, d)| *d == expected), "{expected}: {refusals:?}");
    }
}

/// The C8 document's shared constraint: one row, naming both holders.
#[test]
fn the_shared_constraint_is_one_row_naming_both_holders() {
    let d = document("c8_shared_constraint");
    let r = row(&d, "combined", &[("node", json!("B")), ("component", json!("UZ"))]);
    assert_eq!(r["holders"], json!(["SB", "SN-B"]));
    assert_eq!(r["quantity_kind"], "constraint_reaction");
    let census = d["census"]["rigid_constraints"]["rows"].as_array().unwrap();
    assert_eq!(census.iter().filter(|c| c["node_id"] == "B").count(), 1);
}

/// The renamed identities are carried byte for byte, and a document built from them combines to
/// the same magnitudes as the reference's names do.
#[test]
fn separator_identities_are_carried_byte_for_byte() {
    let d = document("c2_separator_identities");
    let nodes: Vec<&str> = d["census"]["nodes"]["rows"].as_array().unwrap().iter().map(|r| r["node_id"].as_str().unwrap()).collect();
    assert_eq!(nodes, ["N1.ux: anchor node", "N2 .i.Fx_m", "node:N3.uy_m tip"]);
    let f = c2_separator_identities();
    let renamed_result = combine(&f.combination).unwrap();
    let mut plain = f.combination.clone();
    for q in &mut plain.rows {
        *q = match q.clone() {
            Quantity::NodeDisplacement { node, component } => Quantity::NodeDisplacement { node: original(&node), component },
            Quantity::ElementEndAction { member, family, end, component, frame } => Quantity::ElementEndAction { member: original(&member), family, end, component, frame },
            Quantity::ConstraintReaction { node, component, holders } => Quantity::ConstraintReaction { node: original(&node), component, holders: holders.iter().map(|h| original(h)).collect() },
            Quantity::CompliantSupportForce { support, component } => Quantity::CompliantSupportForce { support: original(&support), component },
            other => other,
        };
    }
    let plain_result = combine(&plain).unwrap();
    assert_eq!(renamed_result.rows.len(), f.combination.rows.len());
    assert_eq!(plain_result.rows.len(), renamed_result.rows.len());
    for (a, b) in renamed_result.rows.iter().zip(&plain_result.rows) {
        assert_eq!(a.combined, b.combined);
        assert_eq!(a.per_direction, b.per_direction);
    }
    // Two tie rods whose composed labels are equal and whose identities differ are two rows: the
    // combinations compare identities, not labels.
    let a = Quantity::TieRodForce { connector: "x] [y".into(), rod: "z".into() };
    let b = Quantity::TieRodForce { connector: "x".into(), rod: "y] [z".into() };
    assert_eq!(a.label(), b.label());
    assert_ne!(a.identity(), b.identity());
    let mut colliding = f.combination.clone();
    colliding.rows.extend([a, b]);
    for p in &mut colliding.periodic {
        p.values.extend([0.0, 0.0]);
    }
    for r in &mut colliding.rigid {
        r.values.extend([0.0, 0.0]);
    }
    let result = combine(&colliding).unwrap();
    assert_eq!(result.rows.len(), f.combination.rows.len() + 2);
}

fn mass_row<'a>(d: &'a Value, direction: &str) -> &'a Value {
    d["disclosures"]["missing_mass"]["directions"]["rows"].as_array().unwrap().iter().find(|r| r["direction"] == direction).unwrap()
}

fn value_of(v: &Value) -> f64 {
    v["value"].as_f64().unwrap()
}

/// The published U is RN(T − E) with E the exact sum, not T minus the published S: the committed
/// C3 Y row, where the two differ by one unit in the last place; the fractions are the binary64
/// quotients of the published S and U by T.
#[test]
fn the_unrepresented_mass_is_formed_from_the_exact_sum() {
    let d = document("c3_grouping_100_40_40");
    assert_eq!(d["disclosures"]["missing_mass"]["arithmetic"], "E is the exact sum of the stated binary64 effective masses of the \
periodic modes. S is E rounded once to the nearest binary64 (ties to even). U is the exact difference T \u{2212} E, rounded once the \
same way. U is not T minus the published S, and the two can differ: where E rounds to T, T minus the published S is zero while the \
mass the periodic modes leave out is not. The cumulative fraction is the binary64 quotient of the published S by T and the \
unrepresented fraction the binary64 quotient of the published U by T, each rounded to nearest; they state what fraction of the \
published total each published mass is, and are recomputable from the published values. T is the stated total mass of the \
direction, which includes the mass that moves with the supports. Where T is zero both fractions are undefined.");
    let y = mass_row(&d, "Y");
    let (t, s, u) = (value_of(&y["total_mass"]), value_of(&y["periodic_effective_mass_sum"]), value_of(&y["unrepresented_effective_mass"]));
    assert_eq!(u, 58.872593076971675);
    assert_eq!(t - s, 58.87259307697167);
    assert_ne!(u, t - s);
    assert_eq!(value_of(&y["periodic_cumulative_fraction"]), s / t);
    assert_eq!(value_of(&y["unrepresented_fraction"]), u / t);
}

/// Effective masses whose exact sum E is not T but rounds to T: the published S equals T, T − S
/// is zero, and the published U is the nonzero RN(T − E).
#[test]
fn a_sum_that_rounds_to_the_total_leaves_a_published_unrepresented_mass() {
    let mut f = c7_cancelling_cluster();
    let k = 1; // Y
    f.modal.total_mass_kg[k] = 1.0;
    let masses = [1.0 - 2f64.powi(-53), 2f64.powi(-53) - 2f64.powi(-60), 0.0, 0.0];
    for (e, m) in f.modal.effective_mass_kg.iter_mut().zip(masses) {
        e[k] = m;
    }
    let d = f.document();
    assert_eq!(d["status"], "solved", "{:?}", refusal_details(&d));
    let y = mass_row(&d, "Y");
    let (t, s, u) = (value_of(&y["total_mass"]), value_of(&y["periodic_effective_mass_sum"]), value_of(&y["unrepresented_effective_mass"]));
    assert_eq!((t, s), (1.0, 1.0));
    assert_eq!(t - s, 0.0);
    assert_eq!(u, 2f64.powi(-60));
    assert_eq!(value_of(&y["periodic_cumulative_fraction"]), 1.0);
    assert_eq!(value_of(&y["unrepresented_fraction"]), 2f64.powi(-60));
}

/// The mass operands are checked before any arithmetic on them: a nonfinite total or effective
/// mass is outside the canonical profile, a negative one is invalid, and a zero total with a
/// nonzero effective mass is invalid; none panics.
#[test]
fn mass_operands_are_checked_before_the_arithmetic() {
    let outside = ("SPECTRUM_DOCUMENT_OUTSIDE_CANONICAL_PROFILE".to_string(), "missing_mass/Y".to_string(), Some("mass".to_string()));
    let invalid = ("SPECTRUM_DOCUMENT_INPUT_INVALID".to_string(), "missing_mass/Y".to_string(), Some("mass".to_string()));
    for v in [f64::NAN, f64::INFINITY] {
        let mut f = c7_cancelling_cluster();
        f.modal.total_mass_kg[1] = v;
        assert_eq!(refusal_fields(&f.document()), vec![outside.clone()]);
        let mut f = c7_cancelling_cluster();
        f.modal.effective_mass_kg[2][1] = v;
        assert_eq!(refusal_fields(&f.document()), vec![outside.clone()]);
    }
    let mut f = c7_cancelling_cluster();
    f.modal.effective_mass_kg[0][1] = -1.0;
    assert_eq!(refusal_fields(&f.document()), vec![invalid.clone()]);
    let mut f = c7_cancelling_cluster();
    f.modal.total_mass_kg[1] = 0.0;
    assert_eq!(refusal_fields(&f.document()), vec![invalid]);
}

/// An array nested `depth` deep, built without recursion.
fn nested(depth: usize) -> Value {
    let mut v = json!(0);
    for _ in 0..depth {
        v = Value::Array(vec![v]);
    }
    v
}

/// The model, the request and the modal state are checked for depth before anything serializes
/// them: too deep for the checked parse is refused, a state that passes on its own but not at
/// /modal_basis/state is refused, and a very deep model does not exhaust the stack.
#[test]
fn caller_values_are_checked_for_depth_first() {
    let outside = |object: &str, field: Option<&str>| ("SPECTRUM_DOCUMENT_OUTSIDE_CANONICAL_PROFILE".to_string(), object.to_string(), field.map(str::to_string));
    let mut f = c7_cancelling_cluster();
    f.model["deep"] = nested(127);
    assert_eq!(refusal_fields(&f.document()), vec![outside("model_document", None)]);
    let mut f = c7_cancelling_cluster();
    f.request["deep"] = nested(127);
    assert_eq!(refusal_fields(&f.document()), vec![outside("spectrum_request", None)]);
    // 126 nested arrays pass the checked parse on their own (levels 1 to 126) and not at level 3.
    let state = nested(126);
    assert!(spectrum_doc_sha256(&state).is_some());
    let mut f = c7_cancelling_cluster();
    f.modal.state = Some(state);
    assert_eq!(refusal_fields(&f.document()), vec![outside("modal_basis", Some("state"))]);
    let mut f = c7_cancelling_cluster();
    f.modal.state = Some(nested(125));
    assert_eq!(f.document()["status"], "solved");
    let mut f = c7_cancelling_cluster();
    f.model["deep"] = nested(200_000);
    assert_eq!(refusal_fields(&f.document()), vec![outside("model_document", None)]);
    // Dropping a value this deep recurses; the test leaks it instead.
    std::mem::forget(f);
}

/// The extraction statement's joint invariants, each refused as an invalid input naming the
/// extraction.
#[test]
fn the_extraction_statement_is_checked() {
    let extraction = |change: &dyn Fn(&mut Extraction)| -> Vec<String> {
        let mut f = c7_cancelling_cluster();
        change(&mut f.modal.extraction);
        let d = f.document();
        assert_eq!(d["status"], "refused");
        refusal_details(&d).into_iter().map(|(code, detail)| {
            assert_eq!(code, "SPECTRUM_DOCUMENT_INPUT_INVALID");
            detail
        }).collect()
    };
    assert_eq!(c7_cancelling_cluster().modal.extraction.endpoint, "below_computed_cluster");
    assert_eq!(extraction(&|e| e.endpoint = "below_the_cut".into()),
        ["`below_the_cut` is not an extraction endpoint (one of whole_finite_spectrum, below_computed_cluster, below_lowest_mode)"]);
    assert_eq!(extraction(&|e| { e.next_computed_frequency_hz = None; e.next_computed_circular_frequency_rad_s = None; }),
        ["the extraction is stated below a computed cluster without both the next computed mode's frequency and circular frequency"]);
    assert_eq!(extraction(&|e| e.next_computed_circular_frequency_rad_s = None),
        ["the extraction is stated below a computed cluster without both the next computed mode's frequency and circular frequency"]);
    assert_eq!(extraction(&|e| e.established_finite_mode_count = Some(3)), ["the established finite mode count 3 is below the 4 reported"]);
    assert_eq!(extraction(&|e| e.established_finite_mode_count = Some(4)),
        ["a next computed mode is stated and the established finite mode count 4 does not exceed the 4 reported"]);
    assert_eq!(extraction(&|e| e.endpoint = "whole_finite_spectrum".into()), [
        "the whole finite spectrum is stated with an established finite mode count of Some(12), not the 4 reported",
        "the whole finite spectrum is stated with a next computed mode"]);
    assert_eq!(extraction(&|e| e.evidence = " ".into()), ["the completion evidence is blank"]);
    assert_eq!(extraction(&|e| { e.endpoint = "below_lowest_mode".into(); }), ["the extraction is stated below the lowest mode with 4 reported modes"]);
}

/// A request whose spectrum or scale factor is not the one a direction's contributions were
/// formed with, a model whose record of that spectrum is not the one they were formed from, and a
/// direction with no formation record are each refused, naming the direction.
#[test]
fn formation_records_are_checked_against_the_request_and_the_model() {
    let refused_at = |f: &Fixture| -> Vec<(String, String, Option<String>)> { refusal_fields(&f.document()) };
    let at_y = vec![("SPECTRUM_DOCUMENT_INPUT_INVALID".to_string(), "Y".to_string(), Some("formation".to_string()))];
    let f = c2_separator_identities();
    assert_eq!(f.request["directions"].as_array().unwrap().len(), 1);
    assert_eq!(f.request["directions"][0]["direction"], "Y");
    // The scale doubled in the request, the contributions unchanged.
    let mut g = c2_separator_identities();
    let scale = g.request["directions"][0]["scale_factor"].as_f64().unwrap();
    g.request["directions"][0]["scale_factor"] = json!(2.0 * scale);
    assert_eq!(refused_at(&g), at_y);
    // The request's spectrum swapped for another in-range spectrum of the same damping.
    let mut g = c2_separator_identities();
    let mut other = g.model["spectra"][0].clone();
    other["id"] = json!("another spectrum");
    for p in other["points"].as_array_mut().unwrap() {
        p[1] = json!(p[1].as_f64().unwrap() * 2.0);
    }
    g.model["spectra"].as_array_mut().unwrap().push(other);
    g.request["directions"][0]["spectrum"] = json!("another spectrum");
    assert_eq!(refused_at(&g), at_y);
    // The same spectrum identity with one ordinate changed in the model.
    let mut g = c2_separator_identities();
    let p = &mut g.model["spectra"][0]["points"][1][1];
    *p = json!(p.as_f64().unwrap() * 1.5);
    assert_eq!(refused_at(&g), at_y);
    // No formation record for a direction with contributions; a record for a direction without.
    let mut g = c2_separator_identities();
    g.formation.clear();
    assert_eq!(refused_at(&g), at_y);
    let mut g = c2_separator_identities();
    let mut extra = g.formation[0].clone();
    extra.direction = Direction::Z;
    g.formation.push(extra);
    assert_eq!(refused_at(&g), vec![("SPECTRUM_DOCUMENT_INPUT_INVALID".to_string(), "Z".to_string(), Some("formation".to_string()))]);
}

/// A model that holds two spectra of the identity a requested direction names is refused.
#[test]
fn a_requested_spectrum_identity_held_twice_is_refused() {
    let mut f = c2_separator_identities();
    let copy = f.model["spectra"][0].clone();
    f.model["spectra"].as_array_mut().unwrap().push(copy);
    let d = f.document();
    assert_eq!(refusal_fields(&d), vec![("SPECTRUM_DOCUMENT_INPUT_INVALID".to_string(), "Y".to_string(), Some("spectrum".to_string()))]);
    assert!(refusal_details(&d)[0].1.contains("holds 2 spectra of identity"));
}

/// A node displacement row whose component a census rigid constraint restrains at that node has
/// a nonzero contribution: refused.
#[test]
fn a_restrained_component_with_a_nonzero_contribution_is_refused() {
    let mut f = c7_cancelling_cluster();
    let r = f.combination.rows.iter().position(|q| matches!(q, Quantity::NodeDisplacement { node, .. } if node == "N1")).unwrap();
    f.combination.periodic[0].values[r] = 1e-3;
    let d = f.document();
    assert_eq!(refusal_fields(&d).len(), 1);
    assert_eq!(refusal_fields(&d)[0].2.as_deref(), Some("contribution"));
    assert!(refusal_details(&d)[0].1.contains("restrains this component at this node"), "{:?}", refusal_details(&d));
}

/// A constraint's holders are compared with the census's as a set and published in the census's
/// order; a row supplied twice is refused against the roster, not as an invalid row.
#[test]
fn rows_are_held_to_the_roster() {
    let reference = c8_shared_constraint().document();
    let mut f = c8_shared_constraint();
    for q in &mut f.combination.rows {
        if let Quantity::ConstraintReaction { holders, .. } = q {
            holders.reverse();
        }
    }
    let d = f.document();
    assert_eq!(d, reference, "a reordered holder list is not a refusal and is published in the census's order");
    let mut f = c8_shared_constraint();
    let first = f.combination.rows[0].clone();
    f.combination.rows.push(first);
    for p in &mut f.combination.periodic {
        let v = p.values[0];
        p.values.push(v);
    }
    let fields = refusal_fields(&f.document());
    assert_eq!(fields.len(), 1, "{fields:?}");
    assert_eq!((fields[0].0.as_str(), fields[0].2.as_deref()), ("SPECTRUM_DOCUMENT_INPUT_INVALID", Some("roster")));
}

/// A rigid constraint held by one support is `single`; the C8 constraint held by SB and SN-B is
/// `jointly_undivided`, in its census row, its signed row and its combined row.
#[test]
fn constraint_rows_state_how_they_are_held() {
    let d = document("c8_shared_constraint");
    let fields = [("node", json!("B")), ("component", json!("UZ"))];
    assert_eq!(row(&d, "signed", &fields)["held"], "jointly_undivided");
    assert_eq!(row(&d, "combined", &fields)["held"], "jointly_undivided");
    let census = d["census"]["rigid_constraints"]["rows"].as_array().unwrap();
    for c in census {
        let expected = if c["node_id"] == "B" && c["component"] == "UZ" { "jointly_undivided" } else { "single" };
        assert_eq!(c["held"], expected, "{c}");
    }
    let table: Value = serde_json::from_str(SEMANTIC_TABLE).unwrap();
    assert_eq!(table["held"][1]["statement"], "The row states that it is the reaction of the constraint, that the supports it names \
hold it together, and that it is not divided between them or to be added once per support.");
}

/// The grouping and ten percent documents state their closeness rule; the others state none.
#[test]
fn the_closeness_rule_is_stated_with_its_method() {
    for (name, ..) in COMMITTED {
        let d = document(name);
        if d["status"] != "solved" {
            continue;
        }
        let rule = &d["rule_statements"]["closeness_rule"];
        match d["request"]["modal_combination"].as_str().unwrap() {
            "GROUPING" => assert!(rule.as_str().unwrap().starts_with("Periodic clusters in ascending order form groups."), "{name}"),
            "TEN_PERCENT" => assert!(rule.as_str().unwrap().starts_with("Two periodic clusters i < j are coupled"), "{name}"),
            _ => assert!(rule.is_null(), "{name}"),
        }
    }
}

/// With no extracted mode (the endpoint below the lowest mode), as the code stands: with the
/// correction off the document publishes, with empty mode, cluster and column tables and zero
/// magnitudes; with it applied the modes' check refuses it. Neither panics.
#[test]
fn an_empty_extracted_set_as_the_code_stands() {
    let empty = |f: &mut Fixture| {
        let (fr, w) = (f.combination.modes[0].frequency_hz, f.modal.circular_frequency_rad_s[0]);
        f.combination.modes.clear();
        f.combination.periodic.clear();
        f.modal.mode_standings.clear();
        f.modal.effective_mass_kg.clear();
        f.modal.circular_frequency_rad_s.clear();
        let e = &mut f.modal.extraction;
        e.endpoint = "below_lowest_mode".into();
        e.reported_mode_count = 0;
        e.next_computed_frequency_hz = Some(fr);
        e.next_computed_circular_frequency_rad_s = Some(w);
        for r in &mut f.combination.rigid {
            r.periodic_clusters_left_out.clear();
        }
    };
    let mut f = c7_cancelling_cluster();
    empty(&mut f);
    let d = f.document();
    assert_eq!(d["status"], "solved", "{:?}", refusal_details(&d));
    assert_eq!(d["modal_basis"]["modes"]["row_count"], 0);
    assert_eq!(d["modal_basis"]["clusters"]["row_count"], 0);
    assert!(d["rows"]["combined"]["rows"].as_array().unwrap().iter().all(|r| r["combined"].as_f64() == Some(0.0)));
    let mut f = w2_arc_fixture("w2_arc_oblique", false);
    empty(&mut f);
    let fields = refusal_fields(&f.document());
    assert!(fields.iter().all(|(code, ..)| code == "SPECTRUM_MODAL_METADATA_INVALID"), "{fields:?}");
    assert!(!fields.is_empty());
}

// ---------------------------------------------------------------------------------------------
// The document's shape, as the committed semantic table states it.

/// Checks a document against the `document_shape` of a semantic table.
struct ShapeCheck<'a> {
    shapes: BTreeMap<&'a str, &'a Value>,
    units: BTreeMap<&'a str, &'a str>,
    root: &'a Value,
    errors: Vec<String>,
}

impl<'a> ShapeCheck<'a> {
    fn new(table: &'a Value, root: &'a Value) -> Self {
        let shapes = table["document_shape"]["shapes"].as_array().unwrap().iter().map(|s| (s["shape"].as_str().unwrap(), s)).collect();
        let units = table["value_kinds"].as_array().unwrap().iter().map(|k| (k["kind"].as_str().unwrap(), k["unit"].as_str().unwrap())).collect();
        Self { shapes, units, root, errors: Vec::new() }
    }

    fn fail(&mut self, path: &str, what: String) {
        self.errors.push(format!("{path}: {what}"));
    }

    /// Whether `null` (is the member null) is what its rule permits.
    fn null_ok(&self, rule: &str, parent: &Map<String, Value>, member: &str, null: bool) -> Option<bool> {
        let root = self.root;
        let modal = root["request"]["modal_combination"].as_str();
        let exactly = |condition: bool| null == condition;
        Some(match rule {
            "never" => !null,
            "always" => null,
            "permitted" => true,
            "refused_only" => !null || root["status"] == "refused",
            "correction_not_applied" => exactly(root["request"]["missing_mass"]["applied"] != true),
            "modal_rule_not_cqc" => exactly(modal != Some("CQC")),
            "modal_rule_not_grouping" => exactly(modal != Some("GROUPING")),
            "modal_rule_not_ten_percent" => exactly(modal != Some("TEN_PERCENT")),
            "modal_rule_not_grouping_or_ten_percent" => exactly(!matches!(modal, Some("GROUPING" | "TEN_PERCENT"))),
            "directional_rule_not_100_40_40" => exactly(root["request"]["directional_combination"] != "100-40-40"),
            "binding_not_applicable" => exactly(root["modal_result"]["binding"] == "not_applicable"),
            "binding_bound" => exactly(root["modal_result"]["binding"] == "bound"),
            "origin_spectrum_solve" => exactly(root["origin"]["kind"] == "spectrum_solve"),
            "endpoint_whole_finite_spectrum" => exactly(root["modal_basis"]["extraction"]["endpoint"] == "whole_finite_spectrum"),
            "no_cluster" => exactly(root["modal_basis"]["clusters"]["row_count"] == 0),
            "undefined_reason_stated" => {
                let other = if member == "value" { "undefined_reason" } else { "value" };
                exactly(!parent.get(other).map_or(true, Value::is_null))
            }
            _ => return None,
        })
    }

    fn check(&mut self, v: &Value, ty: &Value, path: &str) {
        let ok = match ty["type"].as_str().unwrap() {
            "string" => v.is_string(),
            "token" => v.as_str().is_some_and(|s| ty["tokens"].as_array().unwrap().iter().any(|t| t == s)),
            "boolean" => v.is_boolean(),
            "integer" => v.as_u64().is_some_and(|n| n < 1 << 53),
            "number" => v.as_f64().is_some_and(|x| x.is_finite() && x.abs() < 9_007_199_254_740_992.0),
            "opaque_json" => true,
            "value_object" => {
                self.object(v, "value_object", path);
                let kind = v["kind"].as_str().unwrap_or("");
                ty["kinds"].as_array().unwrap().iter().any(|k| k == kind) && self.units.get(kind).is_some_and(|u| v["unit"] == *u)
            }
            "row_ref" => {
                let table = self.root.pointer(ty["target"].as_str().unwrap());
                table.and_then(|t| t["rows"].as_array()).is_some_and(|rows| rows.iter().any(|r| r["row_id"] == *v))
            }
            "array" => match v.as_array() {
                Some(items) => {
                    for (i, item) in items.iter().enumerate() {
                        self.check(item, &ty["items"], &format!("{path}/{i}"));
                    }
                    ty["length"].as_u64().map_or(true, |n| n as usize == items.len())
                }
                None => false,
            },
            "object" => {
                self.object(v, ty["shape"].as_str().unwrap(), path);
                true
            }
            "direction_map" => match v.as_object() {
                Some(map) => {
                    let keys: Vec<&str> = map.keys().map(String::as_str).collect();
                    let mut expected: Vec<&str> = self.root["rows"]["directions"].as_array().unwrap().iter().map(|d| d.as_str().unwrap()).collect();
                    expected.sort_unstable();
                    for (k, item) in map {
                        self.check(item, &ty["values"], &format!("{path}/{k}"));
                    }
                    keys == expected
                }
                None => false,
            },
            "table" => self.table(v, ty["row"].as_str().unwrap(), path),
            other => {
                self.fail(path, format!("unknown type {other}"));
                true
            }
        };
        if !ok {
            self.fail(path, format!("{v} is not of type {ty}"));
        }
    }

    fn table(&mut self, v: &Value, row: &str, path: &str) -> bool {
        let Some(map) = v.as_object() else { return false };
        let keys: Vec<&str> = map.keys().map(String::as_str).collect();
        if keys != ["digest", "row_count", "rows", "status"] || v["status"] != "published" {
            return false;
        }
        let Some(rows) = v["rows"].as_array() else { return false };
        let checksums: Vec<Value> = rows.iter().map(|r| r["checksum"].clone()).collect();
        let mut ok = v["row_count"].as_u64() == Some(rows.len() as u64)
            && v["digest"].as_str() == spectrum_doc_sha256(&Value::Array(checksums)).as_deref();
        let mut ids = BTreeSet::new();
        for (i, r) in rows.iter().enumerate() {
            let at = format!("{path}/rows/{i}");
            let mut unsealed = r.as_object().cloned().unwrap_or_default();
            let checksum = unsealed.remove("checksum");
            ok &= r["ordinal"].as_u64() == Some(i as u64) && r["row_id"].as_str().is_some_and(|id| ids.insert(id.to_string()))
                && checksum.as_ref().and_then(Value::as_str) == spectrum_doc_sha256(&Value::Object(unsealed.clone())).as_deref();
            unsealed.remove("row_id");
            unsealed.remove("ordinal");
            self.object(&Value::Object(unsealed), row, &at);
        }
        ok
    }

    fn object(&mut self, v: &Value, shape_name: &str, path: &str) {
        let Some(map) = v.as_object() else { return self.fail(path, format!("not an object of shape {shape_name}")) };
        let Some(shape) = self.shapes.get(shape_name).copied() else { return self.fail(path, format!("no shape {shape_name}")) };
        let mut members: Vec<&Value> = shape["members"].as_array().unwrap().iter().collect();
        if let Some(variants) = shape["variants"].as_object() {
            let by = variants["by"].as_str().unwrap();
            let token = map.get(by).and_then(Value::as_str).unwrap_or("");
            match variants["alternatives"].as_array().unwrap().iter().find(|a| a["when"] == token) {
                Some(a) => members.extend(a["members"].as_array().unwrap()),
                None => return self.fail(path, format!("{by} = {token:?} selects no alternative of {shape_name}")),
            }
        }
        let declared: BTreeSet<&str> = members.iter().map(|m| m["member"].as_str().unwrap()).collect();
        for k in map.keys() {
            if !declared.contains(k.as_str()) {
                self.fail(path, format!("undeclared member {k} in {shape_name}"));
            }
        }
        for m in members {
            let name = m["member"].as_str().unwrap();
            let at = format!("{path}/{name}");
            let Some(value) = map.get(name) else {
                self.fail(&at, format!("missing member of {shape_name}"));
                continue;
            };
            match self.null_ok(m["null"].as_str().unwrap(), map, name, value.is_null()) {
                None => self.fail(&at, format!("unknown null rule {}", m["null"])),
                Some(false) => self.fail(&at, format!("null is not as rule {} states", m["null"])),
                Some(true) if value.is_null() => {}
                Some(true) => self.check(value, &m["type"], &at),
            }
        }
    }
}

fn shape_errors(table: &Value, document: &Value) -> Vec<String> {
    let mut check = ShapeCheck::new(table, document);
    check.object(document, "document", "");
    check.errors
}

/// Every committed document has exactly the members, types and nulls the committed semantic
/// table's `document_shape` declares, and its tables are sealed as the shape states.
#[test]
fn every_committed_document_conforms_to_the_document_shape() {
    let table: Value = serde_json::from_str(SEMANTIC_TABLE).unwrap();
    for (name, text, ..) in COMMITTED {
        let d: Value = serde_json::from_str(text).unwrap();
        let errors = shape_errors(&table, &d);
        assert!(errors.is_empty(), "{name}: {errors:#?}");
    }
    // The check sees an undeclared member, a missing member and a wrong type.
    let base = document("c2_separator_identities");
    let mut d = base.clone();
    d["rows"]["combined"]["rows"][0]["extra"] = json!(1);
    assert!(shape_errors(&table, &d).iter().any(|e| e.contains("undeclared member extra")));
    let mut d = base.clone();
    d["request"].as_object_mut().unwrap().remove("directions");
    assert!(shape_errors(&table, &d).iter().any(|e| e.contains("/request/directions: missing member")));
    let mut d = base;
    d["modal_basis"]["cluster_tolerance"]["value"] = json!("small");
    assert!(!shape_errors(&table, &d).is_empty());
}

/// The member names no document of this family or its semantic table uses (`token` only in a
/// standing object), and no published string is the literal "TBD".
#[test]
fn no_document_member_is_a_reserved_name() {
    const RESERVED: [&str; 26] = ["payload", "content", "contents", "secret", "password", "credential", "path", "uri", "query",
        "table", "sql", "classification", "privacy_class", "privacy_classification", "redistribution_status", "review_status",
        "lifecycle_status", "provenance", "professional_claim", "field_class", "field_id", "record_id", "export_policy",
        "user_intent", "local_private_intent", "explicit_local_private_intent"];
    fn walk(v: &Value, parent: &str, at: &str, bad: &mut Vec<String>) {
        match v {
            Value::Object(map) => {
                for (k, x) in map {
                    let token_outside = k == "token" && !(parent.ends_with("standing") || parent == "standing");
                    if RESERVED.contains(&k.as_str()) || token_outside {
                        bad.push(format!("{at}/{k}"));
                    }
                    walk(x, k, &format!("{at}/{k}"), bad);
                }
            }
            Value::Array(items) => items.iter().for_each(|x| walk(x, parent, at, bad)),
            Value::String(s) if s == "TBD" => bad.push(format!("{at}: TBD")),
            _ => {}
        }
    }
    let mut texts: Vec<(&str, &str)> = COMMITTED.iter().map(|c| (c.0, c.1)).collect();
    texts.push(("semantic table", SEMANTIC_TABLE));
    for (name, text) in texts {
        let mut bad = Vec::new();
        walk(&serde_json::from_str(text).unwrap(), "", "", &mut bad);
        assert!(bad.is_empty(), "{name}: {bad:?}");
    }
}
