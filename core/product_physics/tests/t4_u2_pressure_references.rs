//! T4-U2 phase 1: pressure through realized bends (`3.0.0/exact_pressure_v3`)
//! against the frozen independent references, through PP's public entries in
//! both solver modes and on both document versions (0.3.0 and 0.4.0).
//!
//! - `validation/references/t4_i7/` (T4-I7, bend references): every valued
//!   case at |observed − expected| ≤ 1e-9·max(|expected|, zero_scale(group)),
//!   published Passed; the mitre refusal control; each negative control's
//!   listed wrong values are not what the product publishes.
//! - `validation/references/t4_i8/` (T4-I8, rebuilt straight cases): every
//!   row on the v3 straight route at the row's own criterion, and every v3
//!   twin bit-equal to its v2 original apart from SP-1's closed exclusions.
//!
//! SP-3: a mismatch here is the product's to fix, never the reference's.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode,
    PreviewSolverMode,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};

const I7: &str = include_str!("../../../validation/references/t4_i7/u2_reference_cases.json");
const I7_DOCUMENTS: &str =
    include_str!("../../../validation/references/t4_i7/u2_document_sketches.json");
const I8: &str = include_str!("../../../validation/references/t4_i8/rebuilt_reference_cases.json");
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
const VERSIONS: [&str; 2] = ["0.3.0", "0.4.0"];
const PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
const LOCATIONS: [&str; 5] = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];

fn number(value: &Value) -> f64 {
    match value {
        Value::String(text) => text.parse().unwrap_or_else(|_| panic!("decimal {text}")),
        Value::Number(number) => number.as_f64().unwrap(),
        other => panic!("not a number: {other}"),
    }
}

fn captured(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(
        run_linear_static_preview_value_with_mode(document.clone(), mode)
            .expect("the captured entry accepts the document"),
    )
    .unwrap()
}

fn typed(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_with_mode(
        serde_json::from_value(document.clone()).expect("the typed DTO deserializes"),
        mode,
    ))
    .unwrap()
}

fn codes(envelope: &Value) -> Vec<String> {
    envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| format!("{}:{}", d["severity"].as_str().unwrap(), d["code"].as_str().unwrap()))
        .collect()
}

/// The published rows by (kind, entity, location, component).
struct Rows(HashMap<(String, String, String, String), Vec<f64>>);

impl Rows {
    fn of(envelope: &Value) -> Self {
        let mut rows: HashMap<_, Vec<f64>> = HashMap::new();
        for row in envelope["results"].as_array().unwrap() {
            let metadata = &row["metadata"];
            let text = |value: &Value| value.as_str().unwrap_or("").to_string();
            rows.entry((
                text(&row["kind"]),
                text(&row["entity_ref"]),
                text(&metadata["location"]),
                text(&metadata["component"]),
            ))
            .or_default()
            .push(number(&row["value"]));
        }
        Self(rows)
    }
    /// The one row matching; `component` empty matches any.
    fn one(&self, kind: &str, entity: &str, location: &str, component: &str) -> Result<f64, String> {
        let found = self
            .0
            .iter()
            .filter(|((k, e, l, c), _)| {
                k == kind && e == entity && (location.is_empty() || l == location) && (component.is_empty() || c == component)
            })
            .flat_map(|(_, values)| values.iter().copied())
            .collect::<Vec<_>>();
        match found.as_slice() {
            [value] => Ok(*value),
            other => Err(format!("{kind} {entity} {location} {component}: {} rows", other.len())),
        }
    }
    fn any(&self, kind: &str, entity: &str) -> bool {
        self.0.keys().any(|(k, e, _, _)| k == kind && e == entity)
    }
}

// ------------------------------------------------------------------ T4-I7

/// One T4-I7 assertion: the reference pointer, the expected value in the
/// published unit, the group's zero scale in that unit and the observed value.
struct Check {
    pointer: String,
    expected: f64,
    zero_scale: f64,
    observed: Result<f64, String>,
}

impl Check {
    fn tolerance(&self) -> f64 {
        1e-9 * self.expected.abs().max(self.zero_scale)
    }
    fn distance(&self) -> Option<f64> {
        self.observed.as_ref().ok().map(|observed| (observed - self.expected).abs() / self.tolerance())
    }
}

const DOFS: [(&str, &str, f64, &str); 6] = [
    ("ux", "global_nodal_displacement_x", 1000.0, "displacement"),
    ("uy", "global_nodal_displacement_y", 1000.0, "displacement"),
    ("uz", "global_nodal_displacement_z", 1000.0, "displacement"),
    ("rx", "global_nodal_rotation_x", 1.0, "rotation"),
    ("ry", "global_nodal_rotation_y", 1.0, "rotation"),
    ("rz", "global_nodal_rotation_z", 1.0, "rotation"),
];
/// Station quantity -> (kind, published unit factor, group, end-row kind).
const STATION: [(&str, &str, f64, &str, bool); 11] = [
    ("N_w", "pipe_wall_axial_force_v2", 1.0, "wall_axial_force", false),
    ("S", "pipe_effective_axial_force_v2", 1.0, "effective_axial_force", false),
    ("sigma_m", "pipe_axial_membrane_stress_v2", 1.0, "membrane_stress", false),
    ("V_y", "element_local_shear_force_y", 1.0, "shear_force", true),
    ("V_z", "element_local_shear_force_z", 1.0, "shear_force", true),
    ("T", "element_local_torsional_moment", 1.0, "section_moment", true),
    ("M_y", "element_local_bending_moment_y", 1.0, "section_moment", true),
    ("M_z", "element_local_bending_moment_z", 1.0, "section_moment", true),
    ("sigma_b_y", "element_local_bending_normal_stress_y", 1e-6, "bending_torsion_stress", false),
    ("sigma_b_z", "element_local_bending_normal_stress_z", 1e-6, "bending_torsion_stress", false),
    ("tau_t", "element_local_torsional_shear_stress", 1e-6, "bending_torsion_stress", false),
];
const END_ROWS: [(&str, &str); 5] = [
    ("F_y", "element_local_shear_force_y"),
    ("F_z", "element_local_shear_force_z"),
    ("M_x", "element_local_torsional_moment"),
    ("M_y", "element_local_bending_moment_y"),
    ("M_z", "element_local_bending_moment_z"),
];
const LAME: [(&str, &str, &str); 4] = [
    ("inner_radial", "pipe_lame_radial_stress_v2", "lame_inner_radial_stress"),
    ("outer_radial", "pipe_lame_radial_stress_v2", "lame_outer_radial_stress"),
    ("inner_hoop", "pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress"),
    ("outer_hoop", "pipe_lame_hoop_stress_v2", "lame_outer_hoop_stress"),
];

/// Every T4-I7 assertion of one valued case against one envelope, keyed by
/// the reference's own JSON pointer.
fn i7_checks(case: &Value, envelope: &Value) -> (Vec<Check>, Vec<String>) {
    let rows = Rows::of(envelope);
    let zero = |group: &str| number(&case["zero_scale"][group]["zero_scale"]);
    let mut checks = Vec::new();
    let mut structural = Vec::new();
    let mut push = |pointer: String, expected: &Value, scale: f64, group: &str, observed: Result<f64, String>| {
        checks.push(Check {
            pointer,
            expected: number(expected) * scale,
            zero_scale: zero(group) * scale,
            observed,
        });
    };
    let expected = &case["expected"];
    for (node, values) in expected["nodes"].as_object().unwrap() {
        for (key, kind, scale, group) in DOFS {
            push(format!("/expected/nodes/{node}/{key}"), &values[key], scale, group, rows.one(kind, node, "", ""));
        }
    }
    for (support, values) in expected["supports"].as_object().unwrap() {
        for (key, value) in values.as_object().unwrap() {
            let group = format!("support_{}@{support}", if key.starts_with('F') { "force" } else { "moment" });
            push(format!("/expected/supports/{support}/{key}"), value, 1.0, &group,
                rows.one("support_reaction_component_v2", support, "", key));
        }
    }
    for (pipe, member) in expected["members"].as_object().unwrap() {
        let arc = member["kind"] == "arc";
        for location in LOCATIONS {
            let station = &member["stations"][location];
            for (key, kind, scale, group, end_row) in STATION {
                let observed = if end_row && location.starts_with("end") {
                    // A straight's end station is the j-side action: -(end_i
                    // row), +(end_j row). An arc publishes its end rows in the
                    // chord frame only (checked below).
                    if arc {
                        continue;
                    }
                    let sign = if location == "end_i" { -1.0 } else { 1.0 };
                    rows.one(kind, pipe, location, "").map(|value| sign * value)
                } else {
                    rows.one(kind, pipe, location, "")
                };
                push(format!("/expected/members/{pipe}/stations/{location}/{key}"), &station[key], scale, group, observed);
            }
        }
        for location in ["end_i", "end_j"] {
            let end = &member["end_rows"][location];
            let block = if arc { "chord_frame_elastic" } else { "element_local" };
            for (key, kind) in END_ROWS {
                let group = match (key.starts_with('F'), arc) {
                    (true, true) => "elastic_end_force_chord_frame",
                    (true, false) => "shear_force",
                    (false, _) => "section_moment",
                };
                push(format!("/expected/members/{pipe}/end_rows/{location}/{block}/{key}"), &end[block][key], 1.0, group,
                    rows.one(kind, pipe, location, ""));
            }
            let wall_block = if arc { "tangent_frame" } else { "element_local" };
            push(format!("/expected/members/{pipe}/end_rows/{location}/{wall_block}/wall_axial_end_action"),
                &end[wall_block]["wall_axial_end_action"], 1.0, "wall_axial_force",
                rows.one("pipe_wall_endpoint_action_v2", pipe, location, "wall_axial_end_action"));
        }
        if arc {
            // Plan section 4.3 item 1 and U0's policy: withheld on arcs; the
            // elastic axial rows are replaced by the pressure family.
            for kind in [
                "pipe_lame_radial_stress_v2",
                "pipe_lame_hoop_stress_v2",
                "pipe_elastic_normal_stress_maximum_v2",
                "element_local_axial_force",
                "element_local_axial_normal_stress",
            ] {
                if rows.any(kind, pipe) {
                    structural.push(format!("{pipe}: {kind} published on an arc"));
                }
            }
        } else {
            for location in LOCATIONS {
                for (key, kind, component) in LAME {
                    push(format!("/expected/members/{pipe}/lame_surface/{key}@{location}"), &member["lame_surface"][key], 1.0,
                        "lame_stress", rows.one(kind, pipe, location, component));
                }
            }
        }
    }
    let regions = envelope["contract_evidence"]["pressure"].as_array().unwrap();
    for (node, terminal) in expected["terminals"].as_object().unwrap() {
        let evidence = regions
            .iter()
            .flat_map(|region| region["terminals"].as_array().unwrap())
            .find(|t| t["node_ref"] == node.as_str())
            .unwrap_or_else(|| panic!("terminal {node} evidence"));
        for (key, field) in [
            ("closure_pressure_load_global", "closure_pressure_load_global_n"),
            ("pipe_cap_transfer_global", "pipe_cap_transfer_global_n"),
            ("remote_closure_support_reaction_global", "remote_closure_support_reaction_global_n"),
        ] {
            if terminal[key].is_null() {
                if !evidence[field].is_null() {
                    structural.push(format!("{node}: {field} is not null"));
                }
                continue;
            }
            for (axis, component) in ["Fx", "Fy", "Fz"].into_iter().enumerate() {
                push(format!("/expected/terminals/{node}/{key}/{component}"), &terminal[key][component], 1.0,
                    "remote_closure_force", Ok(number(&evidence[field][axis])));
            }
        }
    }
    if !codes(envelope).contains(&format!("info:{PASSED}")) {
        structural.push(format!("not published Passed: {:?}", codes(envelope)));
    }
    (checks, structural)
}

fn i7() -> (Value, Value) {
    (serde_json::from_str(I7).unwrap(), serde_json::from_str(I7_DOCUMENTS).unwrap())
}

/// Every valued T4-I7 case, on 0.3.0 and 0.4.0, in both modes (captured
/// entry), within 1e-9 of the reference with its zero-scale floors, and
/// published Passed (C2, at ordinary and UTM coordinates alike).
#[test]
fn i7_valued_cases_match_in_both_modes_and_document_versions() {
    let (references, documents) = i7();
    let mut by_family: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut failures = Vec::new();
    let mut checked = 0usize;
    let mut worst = 0.0f64;
    for (id, case) in references["cases"].as_object().unwrap() {
        if case["expected"].is_null() {
            continue;
        }
        for version in VERSIONS {
            let document = &documents["sketches"][id][version]["document"];
            for mode in MODES {
                let envelope = captured(document, mode);
                let (checks, mut problems) = i7_checks(case, &envelope);
                for check in &checks {
                    checked += 1;
                    match check.distance() {
                        Some(distance) if distance <= 1.0 => worst = worst.max(distance),
                        Some(distance) => problems.push(format!(
                            "{}: observed {:?}, expected {:e}, {distance:.3e} tolerances",
                            check.pointer, check.observed, check.expected
                        )),
                        None => problems.push(format!("{}: {:?}", check.pointer, check.observed)),
                    }
                }
                let family = case["family"].as_str().unwrap().to_string();
                let tally = by_family.entry(family).or_default();
                if problems.is_empty() {
                    tally.0 += 1;
                } else {
                    tally.1 += 1;
                    failures.push(format!("{id} {version} {mode:?}: {:#?}", &problems[..problems.len().min(6)]));
                }
            }
        }
    }
    eprintln!("T4-I7: {checked} values checked; worst {worst:.3e} tolerances; [pass, fail] per family {by_family:?}");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert_eq!(by_family.values().map(|t| t.0).sum::<usize>(), 79 * 4);
}

/// The typed entry agrees with the captured entry bit for bit on the
/// headline cases (plan section 2's PTW case, the U-loop and the rebuilt
/// CBPT), at ordinary and UTM coordinates.
#[test]
fn i7_headline_cases_agree_across_entries() {
    let (_, documents) = i7();
    for id in [
        "U2-L-ANCH-PTW-K2",
        "U2-L-ANCH-PTW-K2-SKEW-X7P3E6",
        "U2-U-ANCH-ALL-K2",
        "MECH-CURVED-BEND-EXACT-PRESSURE-ARC-K2",
    ] {
        for version in VERSIONS {
            let document = &documents["sketches"][id][version]["document"];
            for mode in MODES {
                assert_eq!(captured(document, mode)["results"], typed(document, mode)["results"], "{id} {version} {mode:?}");
            }
        }
    }
}

/// The refusal control (about 2·α_tan at node C): refused by name with the
/// reference's refs, and nothing is published.
#[test]
fn i7_mitre_control_is_refused_and_publishes_nothing() {
    let (references, documents) = i7();
    let case = &references["cases"]["U2-L-MITRE-REFUSED-P-K2"];
    assert!(case["expected"].is_null());
    let refusal = &case["expected_refusal"];
    for version in VERSIONS {
        let document = &documents["sketches"]["U2-L-MITRE-REFUSED-P-K2"][version]["document"];
        for mode in MODES {
            for envelope in [captured(document, mode), typed(document, mode)] {
                assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
                assert_eq!(envelope["results"], json!([]));
                let found = envelope["diagnostics"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|d| d["code"] == refusal["code"])
                    .collect::<Vec<_>>();
                assert_eq!(found.len(), 1, "{:#?}", envelope["diagnostics"]);
                assert_eq!(found[0]["severity"], refusal["severity"]);
                // The reference's refs [region, node, pipe, pipe], after the case.
                let mut expected_refs = vec![json!("case:u2")];
                expected_refs.extend(refusal["refs"].as_array().unwrap().iter().cloned());
                assert_eq!(found[0]["affected_refs"], Value::Array(expected_refs));
            }
        }
    }
}

/// Each T4-I7 negative control: at every listed row the product publishes the
/// reference, not the control's wrong value (each listed row is at least 1e3
/// tolerances from the reference). The controls themselves were run as
/// mutants of the product (T4-I32's record).
#[test]
fn i7_negative_controls_are_not_what_the_product_publishes() {
    let (references, documents) = i7();
    let mut evaluated = BTreeMap::new();
    for (id, case) in references["cases"].as_object().unwrap() {
        let Some(controls) = case["wrong_result_discriminators"].as_array() else {
            continue;
        };
        for version in VERSIONS {
            for mode in MODES {
                let envelope = captured(&documents["sketches"][id][version]["document"], mode);
                let (checks, _) = i7_checks(case, &envelope);
                let by_pointer = checks.iter().map(|c| (c.pointer.as_str(), c)).collect::<HashMap<_, _>>();
                for control in controls {
                    let name = control["id"].as_str().unwrap().to_string();
                    for row in control["discriminating_values"].as_array().unwrap() {
                        let pointer = row["pointer"].as_str().unwrap();
                        // Arc end-station shear and moments are published only
                        // in the chord frame; those rows are not mapped.
                        let Some(check) = by_pointer.get(pointer) else {
                            continue;
                        };
                        let observed = *check.observed.as_ref().unwrap();
                        let scale = if pointer.contains("/nodes/") && !pointer.ends_with("/rx")
                            && !pointer.ends_with("/ry") && !pointer.ends_with("/rz")
                        {
                            1000.0
                        } else {
                            1.0
                        };
                        let wrong = number(&row["wrong_value"]) * scale;
                        assert!((observed - check.expected).abs() <= check.tolerance(), "{id} {name} {pointer}");
                        assert!(
                            (observed - wrong).abs() > 1e3 * check.tolerance(),
                            "{id} {version} {mode:?} {name} {pointer}: {observed} is the control's {wrong}"
                        );
                        *evaluated.entry(name.clone()).or_insert(0usize) += 1;
                    }
                }
            }
        }
    }
    eprintln!("T4-I7 negative-control rows evaluated: {evaluated:?}");
    assert_eq!(evaluated.len(), 7, "{evaluated:?}");
}

/// The admitted kinks (θ ≈ α_tan/2 at node C) are carried, not snapped: the
/// junction is recorded with its θ and the remainder's source terms are in
/// the ledger evidence.
#[test]
fn i7_admitted_kinks_are_recorded_and_carried_by_the_remainder() {
    let (references, documents) = i7();
    for id in ["U2-L-KINK-FREE-P-K2", "U2-L-KINK-ANCH-P-K2"] {
        let reference_theta = number(&references["cases"][id]["derived"]["junction_angles_rad"]["node:C"]);
        for version in VERSIONS {
            let envelope = captured(&documents["sketches"][id][version]["document"], PreviewSolverMode::DenseScrutiny);
            let region = &envelope["contract_evidence"]["pressure"][0];
            assert_eq!(region["tangency_tolerance_rad"], json!(1e-3));
            let junction = region["bend_adjacent_junctions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|j| j["node_ref"] == "node:C")
                .unwrap();
            let theta = number(&junction["theta_rad"]);
            assert!((theta - reference_theta).abs() <= 1e-9 * reference_theta, "{theta} vs {reference_theta}");
            let groups = envelope["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"]
                .as_array()
                .unwrap();
            let remainder = groups.iter().any(|group| {
                group["node_ref"] == "node:C"
                    && group["terms"].as_array().unwrap().iter().any(|term| term["kind"] == "kink_remainder")
                    && number(&group["assembled_force_n"]) != 0.0
            });
            assert!(remainder, "{id}: no nonzero remainder group at node:C");
        }
    }
}

// ------------------------------------------------------------------ T4-I8

/// Each T4-I8 document with its v3 twin (the reference's `v3_patch_for_both`).
fn i8_documents(references: &Value) -> Vec<(String, Value, Value, Vec<Value>)> {
    let mut found = Vec::new();
    for (case_id, case) in references["cases"].as_object().unwrap() {
        let mut owners = Vec::new();
        if let Some(variants) = case["variants"].as_object() {
            for (variant, body) in variants {
                owners.push((format!("{case_id}/{variant}"), body));
            }
        } else {
            owners.push((case_id.clone(), case));
        }
        for (name, owner) in owners {
            let patch = &owner["documents"]["v3_patch_for_both"];
            assert_eq!(patch["op"], "replace");
            assert_eq!(patch["path"], "/model/pressure_contract");
            let rows: Vec<Value> = owner["load_cases"]
                .as_object()
                .map(|cases| cases.values().flat_map(|c| c["rows"].as_array().unwrap().clone()).collect())
                .unwrap_or_default();
            for version in VERSIONS {
                let v2 = owner["documents"][format!("v2_model_{version}")].clone();
                let mut v3 = v2.clone();
                v3["model"]["pressure_contract"] = patch["value"].clone();
                found.push((format!("{name} {version}"), v2, v3, rows.clone()));
            }
        }
    }
    found
}

fn unit_factor(reference_unit: &str, row_unit: &str) -> f64 {
    match (reference_unit, row_unit) {
        ("m", "mm") => 1000.0,
        ("Pa", "MPa") => 1e-6,
        (a, b) if a == b => 1.0,
        other => panic!("unit pair {other:?}"),
    }
}

/// Every T4-I8 row on the v3 straight route, in both modes and on both
/// document versions, at the row's own criterion.
#[test]
fn i8_rows_match_on_the_v3_straight_route() {
    let references: Value = serde_json::from_str(I8).unwrap();
    let mut checked = 0usize;
    let mut failures = Vec::new();
    for (name, _, v3, rows) in i8_documents(&references) {
        for mode in MODES {
            let envelope = captured(&v3, mode);
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{name}: {:?}", codes(&envelope));
            assert_eq!(envelope["formulation_basis"]["profile_id"], "exact_pressure_v3");
            let results = envelope["results"].as_array().unwrap();
            for row in &rows {
                let origin = &row["reference_origin"];
                let expected = number(&references.pointer(origin["pointer"].as_str().unwrap()).unwrap()["value"])
                    * unit_factor(origin["reference_unit"].as_str().unwrap(), row["unit"].as_str().unwrap());
                let matching = results
                    .iter()
                    .filter(|r| {
                        r["kind"] == row["kind"]
                            && r["entity_ref"] == row["entity_ref"]
                            && r["metadata"]["component"] == row["component"]
                            && r["metadata"]["location"] == row["location"]
                            && r["basis_ref"]["ref_id"] == row["basis_ref"]["ref_id"]
                    })
                    .collect::<Vec<_>>();
                let criterion = &row["criterion"];
                let tolerance = number(&criterion["relative_tolerance"]) * expected.abs()
                    + number(&criterion["absolute_tolerance"]);
                checked += 1;
                match matching.as_slice() {
                    [found] if (number(&found["value"]) - expected).abs() <= tolerance => {}
                    other => failures.push(format!(
                        "{name} {mode:?} {} {} {} {}: {:?} vs {expected:e}",
                        row["kind"], row["entity_ref"], row["location"], row["basis_ref"]["ref_id"],
                        other.iter().map(|r| number(&r["value"])).collect::<Vec<_>>()
                    )),
                }
            }
        }
    }
    eprintln!("T4-I8: {checked} rows checked on v3");
    assert!(failures.is_empty(), "{}", failures[..failures.len().min(20)].join("\n"));
    assert!(checked > 0);
}

/// SP-1's twin clause on every T4-I8 document: every numeric leaf of the v3
/// envelope is bit-equal to the v2 one, apart from the closed exclusion list
/// (identity, semantics, limitations and the evidence strings naming the
/// contract), on both entries and in both modes.
#[test]
fn i8_v3_twins_are_bit_equal_to_v2() {
    let references: Value = serde_json::from_str(I8).unwrap();
    for (name, v2, v3, _) in i8_documents(&references) {
        for mode in MODES {
            for run in [captured, typed] {
                let (original, twin) = (run(&v2, mode), run(&v3, mode));
                assert_eq!(original["status"]["mechanics"], "MECHANICS_SOLVED", "{name}");
                let mut differences = Vec::new();
                compare("", &original, &twin, &mut differences);
                assert!(differences.is_empty(), "{name} {mode:?}: {differences:#?}");
            }
        }
    }
}

/// The closed exclusion list (T4-I8 /sp1_pair_scope E1-E4), as T4-U2a's
/// seam test applies it.
fn excluded(path: &str) -> bool {
    path == "/producer/semantic_contract_id"
        || path == "/formulation_basis/profile_id"
        || path.starts_with("/formulation_basis/limitations")
        || (path.starts_with("/contract_evidence/exact_cases/") && path.ends_with("/profile_mode"))
        || (path.starts_with("/contract_evidence/pressure/")
            && (path.ends_with("/profile_mode") || path.ends_with("/profile_version")))
}

fn compare(path: &str, a: &Value, b: &Value, differences: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            for key in x.keys().chain(y.keys().filter(|k| !x.contains_key(*k))) {
                let child = format!("{path}/{key}");
                match (x.get(key), y.get(key)) {
                    (Some(p), Some(q)) => compare(&child, p, q, differences),
                    _ if excluded(&child) => {}
                    _ => differences.push(format!("{child}: key present on one side")),
                }
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() || excluded(path) => {
            if !excluded(path) {
                for (index, (p, q)) in x.iter().zip(y).enumerate() {
                    compare(&format!("{path}/{index}"), p, q, differences);
                }
            }
        }
        (Value::Number(x), Value::Number(y)) => {
            let (p, q) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            if p.to_bits() != q.to_bits() || x.to_string() != y.to_string() {
                differences.push(format!("{path}: {x} vs {y}"));
            }
        }
        _ if a == b || excluded(path) => {}
        _ => differences.push(format!("{path}: {a} vs {b}")),
    }
}

// --------------------------------------------------------- admission (D-2)

/// D-2: a geometry-only (chord) bend on the v3 route is refused with the
/// "realize this bend" text; the same pipe realized as an arc solves.
#[test]
fn a_chord_bend_is_refused_with_the_realize_message_and_its_realization_solves() {
    let (_, documents) = i7();
    let realized = documents["sketches"]["U2-L-ANCH-P-K2"]["0.3.0"]["document"].clone();
    let mut chord = realized.clone();
    chord["model"]["components"][0]["mechanics_interface"]["solver_consumption"] = json!("mechanics_geometry_only");
    for mode in MODES {
        let envelope = captured(&chord, mode);
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE");
        let refusal = envelope["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["code"] == "EXACT_PRESSURE_FAMILY_NOT_ADMITTED")
            .expect("the seam refuses the chord bend");
        assert_eq!(refusal["affected_refs"], json!(["component:bend-1"]));
        let message = refusal["message"].as_str().unwrap();
        assert!(message.contains("realize this bend") && message.contains("(D-2)"), "{message}");
        assert_eq!(captured(&realized, mode)["status"]["mechanics"], "MECHANICS_SOLVED");
    }
}

/// D-E: in a region with a realized bend a straight-straight kink stays
/// `PRESSURE_REGION_NONCOLLINEAR` (each straight run keeps the guard).
#[test]
fn a_straight_straight_kink_beside_a_bend_stays_noncollinear() {
    let (_, documents) = i7();
    let mut document = documents["sketches"]["U2-L-ANCH-P-K2"]["0.3.0"]["document"].clone();
    // Split S2 at (3.25, 2.25) and kink its second half by 1e-4 rad.
    let model = &mut document["model"];
    model["nodes"].as_array_mut().unwrap().push(json!({"id":"node:E","position":{"x":3.25,"y":2.25,"z":0.0},"provenance":"T4-I32 control"}));
    let mut s3 = model["pipe_segments"][2].clone();
    model["pipe_segments"][2]["to"] = json!("node:E");
    s3["id"] = json!("pipe:S3");
    s3["from"] = json!("node:E");
    model["pipe_segments"].as_array_mut().unwrap().push(s3);
    model["nodes"][3]["position"]["x"] = json!(3.25 + 2.0e-4);
    let region = &mut model["load_cases"][0]["pressure_regions"][0];
    region["member_pipe_ids"].as_array_mut().unwrap().push(json!("pipe:S3"));
    for mode in MODES {
        let envelope = captured(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{:?}", codes(&envelope));
        assert!(codes(&envelope).contains(&"blocking:PRESSURE_REGION_NONCOLLINEAR".to_string()), "{:?}", codes(&envelope));
        assert!(!codes(&envelope).iter().any(|code| code.ends_with("PRESSURE_REGION_MITRE_UNSUPPORTED")));
    }
}
