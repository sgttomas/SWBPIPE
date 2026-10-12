//! J7 RF census (test-only diagnostic; `#[ignore]`d, never in the ordinary suite).
//!
//! Every frozen T3 R1 case that lies in D1's source family (and every authorable
//! must-refuse mechanism) runs, in both solver modes, as a genuine 0.3.0 request
//! under the v3 contract (`3.0.0/exact_pressure_v3`) and as its v2 twin
//! (`2.0.0/exact_straight_pressure_v2`, the only change). For each run it records:
//! the public producer's admission/identity/quality, the public source-method
//! outcome (selected, or the typed decline the producer emitted), the Rust raw
//! reader result for every selected publication, the published quantities judged by
//! the unchanged predicate `|obs - exp| <= 1e-9 * max(|exp|, scale)` (selected and
//! ordinary counted separately), and the private method-only outcome of
//! `solve_ordinary` on the actual normalized/built/boundary/load-owned input
//! (the committed J7 pattern of `j7_t4_consequences.rs`). Nothing here changes a
//! reference, expected value, scale, criterion, exclusion, method, limit or product
//! behaviour; it reads, runs and reports.
//!
//! Inputs come from `j7_rf_census_generate.py` (path in `J7_RF_CENSUS_INPUT`);
//! records are written as JSON lines to `J7_RF_CENSUS_OUT`.
use super::*;
use crate::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::Write as _;
#[allow(dead_code)]
#[path = "../../../../validation/benchmarks/numerical_robustness/src/exact.rs"]
mod r1_exact;
use r1_exact::Exact;

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn frozen() -> Value {
    let directory = root().join("validation/references/t3_r1/REFERENCES");
    let json = std::fs::read(directory.join("references.json")).unwrap();
    assert_eq!(sha(&json), "7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9");
    assert_eq!(
        sha(&std::fs::read(directory.join("references.py")).unwrap()),
        "80d473a7351e92a2a903d233b9e633aac794aeff07f3ac41e40d25a0d94fc8a8"
    );
    serde_json::from_slice(&json).unwrap()
}

/// The actual normalized/built/boundary/load-owned method input (same sequence as
/// the committed `j7_t4_consequences.rs` control).
struct Owned {
    model: PreviewModel,
    built: BuiltModel,
    stiffness: Vec<Vec<f64>>,
    force: AssembledForce,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    springs: Vec<SpringEntry>,
    loads: open_pipe_stress_primitive_loads::LoadApplication,
}
impl Owned {
    fn actual(raw: &Value, mode: PreviewSolverMode) -> Result<Self, String> {
        let (request, _captured) = source_receipt::CapturedInvocation::parse(raw.clone(), mode)
            .map_err(|e| format!("captured parsing: {e:?}"))?;
        let mut model = request.model;
        let mut materials = model.materials.clone();
        let mut diagnostics = Vec::new();
        pressure_runtime::validate_profile(&model, &mut diagnostics);
        validate_model_inputs(&model, &materials, &mut diagnostics);
        validate_support_family_tokens(&model, &mut diagnostics);
        resolve_shared_sections(&mut model, &mut diagnostics);
        normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
        if has_blocking(&diagnostics) {
            return Err(format!("normalization/admission: {:?}", blocking_codes(&diagnostics)));
        }
        let built = build_model_for_members(&model, &materials, None, &mut diagnostics)
            .ok_or_else(|| format!("model build: {:?}", blocking_codes(&diagnostics)))?;
        if has_blocking(&diagnostics) {
            return Err(format!("model build: {:?}", blocking_codes(&diagnostics)));
        }
        model.materials = materials;
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        if !boundary.findings.is_empty() {
            return Err(format!("actual boundary: {:?}", boundary.findings));
        }
        let mut stiffness =
            assemble_global_stiffness_with_connectors(built.nodes.len(), &built.frame_elements, &built.connectors)
                .map_err(|e| format!("actual formation: {e:?}"))?;
        let springs = boundary.springs;
        for spring in &springs {
            let d = spring.node_dof.global_index();
            stiffness[d][d] += spring.stiffness.value;
        }
        let primitives = build_load_case_primitive_loads(&model, &model.load_cases[0], &mut diagnostics);
        let loads = prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
        if has_blocking(&diagnostics) || loads.is_blocked() {
            return Err(format!("actual loads: {:?}", blocking_codes(&diagnostics)));
        }
        let force = nodal_and_eigen_case_force(&loads, &[], &built).map_err(|e| format!("actual ledger: {e:?}"))?;
        let prescribed: Vec<_> = boundary.restrained_dofs.into_iter().map(|d| (d, 0.0)).collect();
        let free = (0..force.len()).filter(|d| !prescribed.iter().any(|(p, _)| p == d)).collect();
        Ok(Self { model, built, stiffness, force, free, prescribed, springs, loads })
    }
    fn input(&self) -> Input<'_> {
        Input {
            model: &self.model,
            built: &self.built,
            stiffness: &self.stiffness,
            force: &self.force,
            free: &self.free,
            prescribed: &self.prescribed,
            spring_entries: &self.springs,
            load_case: &self.model.load_cases[0],
            load_application: &self.loads,
            thermal_loads: &[],
            load_state: None,
        }
    }
    /// Member axial (EA/L) or torsional (GJ/L) coefficient of the actual frame.
    fn coefficient(&self, member: &str, torsion: bool) -> Result<f64, String> {
        let i = self.model.pipe_segments.iter().position(|m| m.id == member).ok_or("member absent")?;
        let local = self.built.frame_elements[i].local_stiffness().map_err(|e| format!("member coefficient: {e:?}"))?;
        let value = if torsion { local[3][3] } else { local[0][0] };
        if !(value.is_finite() && value > 0.0) {
            return Err("coefficient range".into());
        }
        Ok(value)
    }
    /// Sizes of the free connected components of the binary64 assembled pattern
    /// (diagnostic; the method's own blocks use exact expansions).
    fn components(&self) -> Vec<usize> {
        let n = self.stiffness.len();
        let mut free = vec![false; n];
        for &i in &self.free {
            free[i] = true;
        }
        let mut seen = vec![false; n];
        let mut sizes = Vec::new();
        for &start in &self.free {
            if seen[start] {
                continue;
            }
            seen[start] = true;
            let mut stack = vec![start];
            let mut size = 0;
            while let Some(r) = stack.pop() {
                size += 1;
                for c in 0..n {
                    if free[c] && !seen[c] && (self.stiffness[r][c] != 0.0 || self.stiffness[c][r] != 0.0) {
                        seen[c] = true;
                        stack.push(c);
                    }
                }
            }
            sizes.push(size);
        }
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes
    }
}
fn blocking_codes(diagnostics: &[Diagnostic]) -> Vec<String> {
    diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect()
}

/// Decimal exponent of a unit symbol relative to the SI base used by R1.
fn unit_exp(unit: &str) -> Result<i64, String> {
    match unit {
        "m" | "N" | "N*m" | "rad" | "1" => Ok(0),
        "mm" | "N*mm" => Ok(-3),
        other => Err(format!("unit {other}")),
    }
}
#[derive(Debug, Clone, Copy)]
enum Obs {
    /// A published binary64 value in `unit`.
    Scalar(f64, &'static str),
    /// A bending magnitude from published local My/Mz in N*m.
    Magnitude(f64, f64),
    /// A relative motion derived as published action / actual member coefficient
    /// (twist T/k_t in rad, extension N/k_a in m); not itself a published quantity.
    Derived(f64, f64, &'static str),
}
enum Unavailable {
    NotPublished(String),
    Other(String),
}
fn comp(dof: &str) -> Option<(&'static str, usize)> {
    match dof {
        "UX" | "FX" => Some(("Fx", 0)),
        "UY" | "FY" => Some(("Fy", 1)),
        "UZ" | "FZ" => Some(("Fz", 2)),
        "RX" | "MX" => Some(("Mx", 3)),
        "RY" | "MY" => Some(("My", 4)),
        "RZ" | "MZ" => Some(("Mz", 5)),
        _ => None,
    }
}
fn leak(s: &str) -> &'static str {
    match s {
        "m" => "m",
        "mm" => "mm",
        "N" => "N",
        "N*m" => "N*m",
        "rad" => "rad",
        _ => "?",
    }
}
/// One published row (by entity/kind/location/component), with its unit.
fn row<'a>(rows: &'a [&'a Value], entity: &str, kind: &str, location: Option<&str>, component: Option<&str>) -> Result<(f64, &'static str), Unavailable> {
    let found: Vec<_> = rows
        .iter()
        .filter(|r| {
            r["entity_ref"] == entity
                && r["kind"] == kind
                && location.is_none_or(|s| r["metadata"]["location"] == s)
                && component.is_none_or(|s| r["metadata"]["component"] == s)
        })
        .collect();
    match found.len() {
        0 => Err(Unavailable::NotPublished(format!("{entity}/{kind}/{location:?}/{component:?}"))),
        1 => {
            let v = found[0]["value"].as_f64().ok_or_else(|| Unavailable::Other("non-numeric value".into()))?;
            Ok((v, leak(found[0]["unit"].as_str().unwrap_or("?"))))
        }
        k => Err(Unavailable::Other(format!("{k} rows for {entity}/{kind}"))),
    }
}
enum Source<'a> {
    Public(Vec<&'a Value>),
    Method(&'a SelectedSourceRecovery),
}
fn observe(key: &str, source: &Source<'_>, owned: Option<&Owned>) -> Result<Obs, Unavailable> {
    let parts: Vec<_> = key.split('.').collect();
    let other = |s: &str| Unavailable::Other(s.to_string());
    match (parts.as_slice(), source) {
        ([kind, node, dof], Source::Public(rows)) if matches!(*kind, "u" | "th") => {
            let (_, axis) = comp(dof).ok_or_else(|| other("dof"))?;
            let name = if *kind == "u" {
                ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"][axis]
            } else {
                ["global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"][axis - 3]
            };
            let (v, u) = row(rows, node, name, None, None)?;
            Ok(Obs::Scalar(v, u))
        }
        ([kind, node, dof], Source::Method(m)) if matches!(*kind, "u" | "th") => {
            let (_, axis) = comp(dof).ok_or_else(|| other("dof"))?;
            let owned = owned.ok_or_else(|| other("owned"))?;
            let i = owned.model.nodes.iter().position(|n| n.id == *node).ok_or_else(|| other("node"))?;
            Ok(Obs::Scalar(m.displacements()[i * 6 + axis], if *kind == "u" { "m" } else { "rad" }))
        }
        ([kind, node, rest @ ..], _) if matches!(*kind, "R" | "S") => {
            let (id, c) = match (*kind, rest) {
                ("R", [dof]) => (format!("rigid:{node}"), *dof),
                ("S", [ordinal, c]) => (format!("spring:{node}:{ordinal}"), *c),
                _ => return Err(other("support key")),
            };
            let (name, axis) = comp(c).ok_or_else(|| other("support component"))?;
            match source {
                Source::Public(rows) => {
                    let (v, u) = row(rows, &id, "support_reaction_component_v2", None, Some(name))?;
                    Ok(Obs::Scalar(v, u))
                }
                Source::Method(m) => {
                    let s = m
                        .support_actions()
                        .iter()
                        .find(|s| s.support_id == id)
                        .ok_or_else(|| Unavailable::NotPublished(format!("method support {id}")))?;
                    Ok(Obs::Scalar(s.values[axis], if axis < 3 { "N" } else { "N*m" }))
                }
            }
        }
        ([kind, member], _) if matches!(*kind, "N" | "T" | "tw" | "ext") => {
            let torsion = matches!(*kind, "T" | "tw");
            let action = match source {
                Source::Public(rows) => {
                    let name = if torsion { "element_local_torsional_moment" } else { "element_local_axial_force" };
                    let (v, u) = row(rows, member, name, Some("end_j"), None)?;
                    if u != if torsion { "N*m" } else { "N" } {
                        return Err(other("action unit"));
                    }
                    v
                }
                Source::Method(m) => {
                    m.members()
                        .iter()
                        .find(|x| x.member_id == *member)
                        .ok_or_else(|| Unavailable::NotPublished(format!("method member {member}")))?
                        .end_forces[if torsion { 9 } else { 6 }]
                }
            };
            if matches!(*kind, "tw" | "ext") {
                let k = owned
                    .ok_or_else(|| other("owned"))?
                    .coefficient(member, torsion)
                    .map_err(|e| Unavailable::Other(e))?;
                Ok(Obs::Derived(action, k, if torsion { "rad" } else { "m" }))
            } else {
                Ok(Obs::Scalar(action, if torsion { "N*m" } else { "N" }))
            }
        }
        (["Mb", member, site], _) => {
            let loc = match *site {
                "i" => "end_i",
                "j" => "end_j",
                "mid" => "midspan",
                _ => return Err(other("bending site")),
            };
            let (y, z) = match source {
                Source::Public(rows) => {
                    let (y, uy) = row(rows, member, "element_local_bending_moment_y", Some(loc), None)?;
                    let (z, uz) = row(rows, member, "element_local_bending_moment_z", Some(loc), None)?;
                    if uy != "N*m" || uz != "N*m" {
                        return Err(other("bending unit"));
                    }
                    (y, z)
                }
                Source::Method(m) => {
                    let x = m
                        .members()
                        .iter()
                        .find(|x| x.member_id == *member)
                        .ok_or_else(|| Unavailable::NotPublished(format!("method member {member}")))?;
                    match *site {
                        "i" => (x.end_forces[4], x.end_forces[5]),
                        "j" => (x.end_forces[10], x.end_forces[11]),
                        _ => (x.sections[2][4], x.sections[2][5]),
                    }
                }
            };
            Ok(Obs::Magnitude(y, z))
        }
        _ => Err(other("unmapped reference key")),
    }
}
/// The unchanged predicate on one observation, decided exactly.
fn holds(obs: Obs, expected: &Exact, scale: &Exact, ref_exp: i64) -> Result<bool, String> {
    match obs {
        Obs::Scalar(v, unit) => {
            if !v.is_finite() {
                return Ok(false);
            }
            // Expected and scale re-expressed exactly in the published unit.
            let shift = ref_exp - unit_exp(unit)?;
            Ok(r1_exact::predicate(&Exact::from_f64(v), &expected.scaled(0, shift), &scale.scaled(0, shift)))
        }
        Obs::Magnitude(y, z) => {
            let shift = ref_exp; // published N*m
            Ok(y.is_finite() && z.is_finite() && r1_exact::magnitude_predicate(y, z, &expected.scaled(0, shift), &scale.scaled(0, shift)))
        }
        Obs::Derived(action, k, unit) => {
            if !(action.is_finite() && k.is_finite() && k > 0.0) {
                return Ok(false);
            }
            let shift = ref_exp - unit_exp(unit)?;
            let (e, s) = (expected.scaled(0, shift), scale.scaled(0, shift));
            // |a/k - e| <= t  <=>  |a - e k| <= t k  (k > 0), exactly.
            let kk = Exact::from_f64(k);
            let t = r1_exact::tolerance(&e, &s).mul(&kk);
            Ok(Exact::from_f64(action).sub(&e.mul(&kk)).abs().cmp(&t) != std::cmp::Ordering::Greater)
        }
    }
}
fn ref_unit_exp(refcase: &Value, class: &str) -> i64 {
    let kind = class.split('@').next().unwrap();
    let units = &refcase["units"];
    match kind {
        "translation" | "extension" if units["length"] == "mm" => -3,
        "moment" if units["moment"] == "N*mm" => -3,
        _ => 0,
    }
}
/// Every reference row of the case's basis, judged against `source`.
fn compare(refcase: &Value, source: &Source<'_>, owned: Option<&Owned>) -> Value {
    let rows = if refcase["basis"] == "represented" { &refcase["expected_represented"] } else { &refcase["expected"] };
    let (mut pass, mut fail, mut derived_pass, mut derived_fail) = (0usize, Vec::new(), 0usize, Vec::new());
    let (mut not_published, mut unavailable) = (Vec::new(), Vec::new());
    let cancel = refcase["family"] == "RF-CANCEL";
    for r in rows.as_array().unwrap() {
        let key = r[0].as_str().unwrap();
        let class = r[2].as_str().unwrap();
        let expected = Exact::parse_decimal(r[1].as_str().unwrap()).unwrap();
        let scale_text = if cancel { r[3].as_str().unwrap() } else { refcase["scales"][class]["value"].as_str().unwrap() };
        let scale = Exact::parse_decimal(scale_text).unwrap();
        match observe(key, source, owned) {
            Ok(obs) => {
                let derived = matches!(obs, Obs::Derived(..));
                match holds(obs, &expected, &scale, ref_unit_exp(refcase, class)) {
                    Ok(true) if derived => derived_pass += 1,
                    Ok(true) => pass += 1,
                    Ok(false) => {
                        let item = json!({"key": key, "observed": format!("{obs:?}"), "expected": r[1], "scale": scale_text});
                        if derived { derived_fail.push(item) } else { fail.push(item) }
                    }
                    Err(e) => unavailable.push(json!({"key": key, "reason": e})),
                }
            }
            Err(Unavailable::NotPublished(why)) => not_published.push(json!({"key": key, "reason": why})),
            Err(Unavailable::Other(why)) => unavailable.push(json!({"key": key, "reason": why})),
        }
    }
    json!({
        "reference_rows": rows.as_array().unwrap().len(),
        "published_pass": pass, "published_fail": fail.len(), "published_failures": fail,
        "derived_pass": derived_pass, "derived_fail": derived_fail.len(), "derived_failures": derived_fail,
        "not_published": not_published.len(), "not_published_keys": not_published,
        "unavailable": unavailable.len(), "unavailable_keys": unavailable,
    })
}
fn work_json(w: &exact::WorkReport) -> Value {
    json!({"charged": w.charged, "rejected": w.rejected, "limit": w.limit})
}
/// Bits digest of every value of a method response (for the v3/v2 twin).
fn response_digest(m: &SelectedSourceRecovery) -> String {
    let mut h = Sha256::new();
    let mut put = |v: f64| h.update(v.to_bits().to_le_bytes());
    m.displacements().iter().copied().for_each(&mut put);
    m.reactions().iter().copied().for_each(&mut put);
    m.published_nodal_components().iter().copied().for_each(&mut put);
    for x in m.members() {
        x.end_forces.iter().copied().for_each(&mut put);
        x.sections.iter().flatten().copied().for_each(&mut put);
    }
    for s in m.support_actions() {
        s.values.iter().copied().for_each(&mut put);
    }
    format!("{:x}", h.finalize())
}
fn public_rows_digest(rows: &[&Value]) -> String {
    let mut h = Sha256::new();
    for r in rows {
        h.update(serde_json::to_string(&json!([r["id"], r["kind"], r["unit"], r["entity_ref"], r["metadata"], r["value"].as_f64().map(f64::to_bits)])).unwrap().as_bytes());
    }
    format!("{:x}", h.finalize())
}

/// Every differing leaf (JSON pointer) of two public envelopes, for the twin.
fn leaf_diff(a: &Value, b: &Value, path: String, out: &mut Vec<Value>) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let keys: std::collections::BTreeSet<_> = x.keys().chain(y.keys()).collect();
            for k in keys {
                leaf_diff(x.get(k).unwrap_or(&Value::Null), y.get(k).unwrap_or(&Value::Null), format!("{path}/{k}"), out);
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            for (i, (u, v)) in x.iter().zip(y).enumerate() {
                leaf_diff(u, v, format!("{path}/{i}"), out);
            }
        }
        (Value::Number(x), Value::Number(y)) if x.as_f64().map(f64::to_bits) == y.as_f64().map(f64::to_bits) && x.is_f64() == y.is_f64() => {}
        _ if a == b => {}
        _ => out.push(json!({"path": path, "v3": a, "v2": b})),
    }
}

fn run_one(entry: &Value, refcase: &Value, raw: &Value, contract: &str, mode: PreviewSolverMode) -> Value {
    let started = std::time::Instant::now();
    let source_contract = if contract == "v3" {
        result_export::physics_source::SourceContract::PressureV3
    } else {
        result_export::physics_source::SourceContract::PhysicsV2
    };
    let owned = Owned::actual(raw, mode);
    let public = run_linear_static_preview_value_with_mode(raw.clone(), mode).map(|e| serde_json::to_value(e).unwrap());
    if let Ok(env) = &public {
        ENVELOPE.with(|slot| *slot.borrow_mut() = Some(env.clone()));
    } else {
        ENVELOPE.with(|slot| *slot.borrow_mut() = None);
    }
    let mut record = json!({
        "id": entry["id"], "family": entry["family"], "role": entry["role"], "basis": entry["basis"],
        "contract": contract, "mode": mode.as_str(),
        "request_sha256": sha(serde_json::to_string(raw).unwrap().as_bytes()),
    });
    match &public {
        Err(e) => {
            record["public"] = json!({"outcome": "refused_before_envelope", "error": e});
        }
        Ok(env) => {
            let identity = env["producer"]["semantic_contract_id"].clone();
            let diags = env["diagnostics"].as_array().cloned().unwrap_or_default();
            let blocking: Vec<_> = diags.iter().filter(|d| d["severity"] == "blocking").map(|d| json!({"code": d["code"], "message": d["message"]})).collect();
            let source_unavailable: Vec<_> = diags.iter().filter(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE").map(|d| d["message"].clone()).collect();
            let codes: Vec<_> = diags.iter().map(|d| d["code"].clone()).collect();
            let rows: Vec<&Value> = env["results"].as_array().map(|a| a.iter().collect()).unwrap_or_default();
            let selected = env.get("source_block_recovery").is_some();
            let mut p = json!({
                "outcome": if !blocking.is_empty() && rows.is_empty() { "refused_blocking" } else if selected { "selected_source_published" } else if rows.is_empty() { "no_rows" } else { "ordinary_published" },
                "identity": identity,
                "profile": env["formulation_basis"]["profile_id"],
                "status": env["status"],
                "numerical_quality": env["numerical_quality"]["status"],
                "case_quality": env["numerical_quality"]["cases"],
                "blocking": blocking,
                "diagnostic_codes": codes,
                "source_attempt_declines": source_unavailable,
                "row_count": rows.len(),
                "rows_digest": public_rows_digest(&rows),
            });
            if selected {
                let body = &env["source_block_recovery"]["body"];
                p["receipt"] = json!({
                    "policy": body["policy"], "status": body["status"], "invocation_work": body["invocation_work"],
                    "cases": body["cases"].as_array().map(|c| c.iter().map(|c| json!({"outcome": c["outcome"], "selected_method": c["selected_method"], "work": c["work"], "failure": c["failure"]})).collect::<Vec<_>>()),
                });
                let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
                let reader = result_export::physics_source::validate_in(env, Some(&invocation), source_contract);
                p["raw_reader"] = json!(format!("{reader:?}"));
                p["selected_comparison"] = compare(refcase, &Source::Public(rows.clone()), owned.as_ref().ok());
            } else if !rows.is_empty() && refcase.get("expected").is_some() {
                p["ordinary_comparison"] = compare(refcase, &Source::Public(rows.clone()), owned.as_ref().ok());
            }
            record["public"] = p;
        }
    }
    match &owned {
        Err(e) => record["method_only"] = json!({"outcome": "input_not_ready", "error": e}),
        Ok(owned) => {
            let components = owned.components();
            let mut m = json!({
                "global_dofs": owned.force.len(), "free_dofs": owned.free.len(),
                "binary64_pattern_free_components": components,
            });
            let result = solve_ordinary(owned.input(), exact::Limits { operations: PHYSICS_SOURCE_WORK_LIMIT, ..Default::default() }, ForceScale::UNSCALED);
            match &result {
                Ok(response) => {
                    let s = response.summary();
                    m["outcome"] = json!("selected");
                    m["work"] = work_json(&s.work);
                    m["summary"] = json!({"blocks": s.blocks, "largest_block": s.largest_block, "functional_count": s.functional_count, "dof_projection_count": s.dof_projection_count, "max_relative_projection_error": s.max_relative_projection_error});
                    m["response_digest"] = json!(response_digest(response));
                    if refcase.get("expected").is_some() {
                        m["comparison"] = compare(refcase, &Source::Method(response), Some(owned));
                    }
                }
                Err(f) => {
                    m["outcome"] = json!("declined");
                    m["stage"] = json!(f.stage);
                    m["helper_stage"] = json!(format!("{:?}", f.helper_stage));
                    m["error"] = json!(format!("{:?}", f.error));
                    m["work"] = work_json(&f.work);
                }
            }
            record["method_only"] = m;
        }
    }
    record["elapsed_ms"] = json!(started.elapsed().as_millis() as u64);
    record
}

thread_local! {
    static ENVELOPE: std::cell::RefCell<Option<Value>> = const { std::cell::RefCell::new(None) };
}

#[test]
#[ignore ="J7 RF census diagnostic: long; run explicitly with J7_RF_CENSUS_INPUT/J7_RF_CENSUS_OUT"]
fn j7_rf_census_all_source_family_cases() {
    let references = frozen();
    let input_path = std::env::var("J7_RF_CENSUS_INPUT").expect("J7_RF_CENSUS_INPUT");
    let out_path = std::env::var("J7_RF_CENSUS_OUT").expect("J7_RF_CENSUS_OUT");
    let only = std::env::var("J7_RF_CENSUS_ONLY").ok();
    let bytes = std::fs::read(&input_path).unwrap();
    eprintln!("J7_RF_CENSUS_INPUT {} sha256 {}", input_path, sha(&bytes));
    let inputs: Value = serde_json::from_slice(&bytes).unwrap();
    let mut out = std::fs::OpenOptions::new().create(true).append(true).open(&out_path).unwrap();
    for entry in inputs["cases"].as_array().unwrap() {
        let id = entry["id"].as_str().unwrap();
        if let Some(only) = &only {
            if !only.split(',').any(|x| x == id) {
                continue;
            }
        }
        let refcase = &references["cases"][id];
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let mut v3_envelope = None;
            for contract in ["v3", "v2"] {
                let mut raw = entry["request"].clone();
                if contract == "v2" {
                    raw["model"]["pressure_contract"] = json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"});
                }
                let mut record = run_one(entry, refcase, &raw, contract, mode);
                let envelope = ENVELOPE.with(|slot| slot.borrow_mut().take());
                if contract == "v3" {
                    v3_envelope = envelope;
                } else {
                    // Complete public-envelope twin: every differing leaf, v3 against v2.
                    let mut diffs = Vec::new();
                    match (&v3_envelope, &envelope) {
                        (Some(a), Some(b)) => leaf_diff(a, b, String::new(), &mut diffs),
                        (None, None) => {}
                        _ => diffs.push(json!({"path": "", "v3_envelope": v3_envelope.is_some(), "v2_envelope": envelope.is_some()})),
                    }
                    record["twin_envelope_leaf_differences"] = json!(diffs);
                }
                let line = serde_json::to_string(&record).unwrap();
                eprintln!("J7_RF_CENSUS {} {} {} {}ms", id, mode.as_str(), contract, record["elapsed_ms"]);
                writeln!(out, "{line}").unwrap();
                out.flush().unwrap();
            }
        }
    }
}
