//! T4-U5b (DEL-04-07 Design `u4b-u5b-implementation.md` §3, §7 items 4 and 6):
//! the integrated tie-rod producer.
//!
//! - Every §3.1 refusal through the public entry, by name, with
//!   `MODEL_INCOMPLETE` and no results, and §1's precedence (a listed
//!   connector refused by a hardware finding carries only that code).
//! - K-D5 closure at product level: the rod models close with the member bound
//!   on the ordinary and W2 routes in both solver modes, and each mutant of the
//!   supplied `TieRodReference` primitives (dropped, duplicated, extra, bound
//!   to the wrong rod or connector, or with wrong operands) fails closed with
//!   the named load-closure detail while the producer's manifest keeps every
//!   original summand.
//!
//! Requests are the frozen t4_u5b case descriptions, authored in 0.9.0 in
//! memory by the shared builder (`tests/common/t4_u5b.rs`).
use super::*;
use crate::certified_loads::supplied_mutant;
use open_pipe_stress_frame_kernel::structural::{LoadFormula, LoadPrimitive};
use open_pipe_stress_frame_kernel::tie_rod::{TieRod, TieRodInput, TIE_ROD_REFERENCE_FAMILY};
use serde_json::{json, Value};

#[path = "../tests/common/current_model.rs"]
mod current_model;
#[path = "../tests/common/t4_u5b.rs"]
mod t4_u5b;

const MODES: [LinearSolveMode; 2] = [LinearSolveMode::DenseScrutiny, LinearSolveMode::SparseInteractive];

fn request(name: &str) -> Value {
    let oracle = t4_u5b::oracle();
    t4_u5b::request(name, &oracle["cases"][name])
}

struct Case {
    built: BuiltModel,
    force: AssembledForce,
}

/// The first load case's ledger through the product's own producers
/// (`case_force_ledger`), with any armed supplied-primitive mutant.
fn case(request: &Value) -> Case {
    let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
    let mut model = request.model;
    let mut materials = if request.materials.is_empty() { model.materials.clone() } else { request.materials };
    let mut diagnostics = Vec::new();
    resolve_shared_sections(&mut model, &mut diagnostics);
    normalize_model_units(&mut model, &mut materials, &mut diagnostics);
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let built = build_model(&model, &materials, &mut diagnostics).expect("model builds");
    let load_case = model.load_cases[0].clone();
    let exact = pressure_runtime::build_pressure_case_with_members(&model, &built, &materials, &load_case, None, &mut diagnostics);
    let loads = build_load_case_primitive_loads(&model, &load_case, &mut diagnostics);
    let application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
    let material_map = materials.iter().map(|m| (m.id.as_str(), m)).collect::<HashMap<_, _>>();
    let pipe_map = model.pipe_segments.iter().enumerate().map(|(i, p)| (p.id.as_str(), i)).collect::<HashMap<_, _>>();
    let thermal = build_thermal_element_loads(&model, &load_case, &material_map, &pipe_map, &built, &mut diagnostics);
    let bends = built.curved_bend_elements.iter().map(|e| (e.pipe_index, e)).collect::<HashMap<_, _>>();
    let force = case_force_ledger(&model, &built, &application, &bends, &thermal, exact.as_ref(), &load_case.id, &mut diagnostics)
        .finish(built.nodes.len() * DOF_PER_NODE)
        .unwrap();
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    Case { built, force }
}

/// (route, quality, formation-check text, member bound present): the
/// ordinary and W2 (at 2^b) checked solves in both modes.
fn checked_solves(case: &Case, b: i32) -> Vec<(String, SolveQuality, String, bool)> {
    let built = &case.built;
    let n = built.nodes.len() * DOF_PER_NODE;
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let free = (0..n).filter(|d| !boundary.restrained_dofs.contains(d)).collect::<Vec<_>>();
    let prescribed = boundary.restrained_dofs.iter().map(|&d| (d, 0.0)).collect::<Vec<_>>();
    let BasisStiffness::Formed(formed) = form_basis_stiffness(built, &[], true).unwrap() else { panic!("formed") };
    let dense = formed.to_dense();
    let members = crate::frame_member_identities(built);
    let mut out = Vec::new();
    for mode in MODES {
        let assembly = AssemblyEvidence::new(built.nodes.len(), &built.frame_elements, &built.connectors, &[], &[])
            .unwrap()
            .with_frame_members(&members);
        let ordinary = assembly.solve_assembled_with_formation_check(&dense, &case.force, &free, &prescribed, mode, &[], true).unwrap();
        out.push((format!("ordinary {mode:?}"), ordinary.report.quality, format!("{:?}", ordinary.formation_check), ordinary.member_bound.is_some()));
        let scale = ForceScale::new(b).unwrap();
        let factor = 2f64.powi(b);
        let scaled = dense.iter().map(|row| row.iter().map(|v| v * factor).collect()).collect::<Vec<Vec<f64>>>();
        let assembly = AssemblyEvidence::new_force_scaled(built.nodes.len(), &built.frame_elements, &built.connectors, &[], &[], scale)
            .unwrap()
            .with_frame_members(&members);
        let w2 = assembly.solve_force_scaled_with_formation_check(&scaled, &case.force, &free, &prescribed, mode, &[], true).unwrap();
        out.push((format!("W2 2^{b} {mode:?}"), w2.solution.report.quality, format!("{:?}", w2.solution.formation_check), w2.solution.member_bound.is_some()));
    }
    out
}

fn is_rod_reference(primitive: &LoadPrimitive) -> bool {
    matches!(primitive.formula, LoadFormula::TieRodReference { .. })
}

/// K-D5 closure (§7 item 6): every rod model of t4_u5b closes with the member
/// bound on both routes and both modes. The rods with a reference extension
/// carry their `tie_rod_reference` summands (one per DOF, source
/// `{len}:{connector}:{len}:{rod}`); a rod with e_ref = 0 carries none ("one ε").
#[test]
fn u5b_rod_models_close_with_the_member_bound_on_the_ordinary_and_w2_routes() {
    let oracle = t4_u5b::oracle();
    for name in oracle["cases"].as_object().unwrap().keys() {
        let description = t4_u5b::description(&oracle["cases"][name]);
        let rods = description["physical_inputs"]["connector"]["hardware"]["rods"].as_array().unwrap();
        let control = case(&request(name));
        let summands = control.force.load_primitives().iter().filter(|p| is_rod_reference(p)).collect::<Vec<_>>();
        let pretensioned = rods.iter().filter(|r| t4_u5b::number(&r["law"]["reference_extension_m"]) != 0.0).count();
        assert_eq!(summands.len(), 12 * pretensioned, "{name}");
        for primitive in &summands {
            assert_eq!(primitive.identity.family, TIE_ROD_REFERENCE_FAMILY, "{name}");
            assert!(primitive.identity.source.starts_with("13:component:C-J:"), "{name}: {}", primitive.identity.source);
        }
        for (route, quality, check, bound) in checked_solves(&control, 4) {
            assert_eq!(quality, SolveQuality::Passed, "{name} {route}: {check}");
            assert!(bound, "{name} {route}: the closed system carries the member bound");
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mutant {
    Drop,
    Duplicate,
    Extra,
    WrongRod,
    WrongConnector,
    WrongExtension,
    WrongStiffness,
    WrongOffset,
}

fn arm(mutant: Mutant) {
    supplied_mutant::arm(Box::new(move |_, supplied: &mut Vec<LoadPrimitive>| {
        let Some(index) = supplied.iter().position(is_rod_reference) else {
            return false;
        };
        let rebuild = |rod: &TieRod, change: &dyn Fn(&mut TieRodInput)| {
            let (offset_i, offset_j) = rod.local_offsets();
            let mut input = TieRodInput { offset_i, offset_j, stiffness: rod.stiffness(), reference_extension: rod.reference_extension() };
            change(&mut input);
            // A connector on the rod's own nodes and axes re-forms the rod.
            let connector = open_pipe_stress_frame_kernel::connector::ObjectiveConnector::new(
                rod.node_i(), rod.node_j(),
                open_pipe_stress_frame_kernel::connector::ConnectorAttachment::global([0.0; 3]),
                open_pipe_stress_frame_kernel::connector::ConnectorAttachment::global([0.0; 3]),
                rod.axes(),
                open_pipe_stress_frame_kernel::connector::ScaledWorkMatrix { upper_triangle: [1.0, 0., 0., 0., 0., 0., 1.0, 0., 0., 0., 0., 1.0, 0., 0., 0., 1.0, 0., 0., 1.0, 0., 1.0], translation_scale: 1.0 },
                [0.0; 6],
            ).unwrap();
            TieRod::new(&connector, input).unwrap()
        };
        match mutant {
            Mutant::Drop => {
                supplied.remove(index);
            }
            Mutant::Duplicate => supplied.push(supplied[index].clone()),
            Mutant::Extra => {
                let mut extra = supplied[index].clone();
                extra.identity.ordinal += 100;
                supplied.push(extra);
            }
            Mutant::WrongRod => {
                let LoadFormula::TieRodReference { rod, .. } = &mut supplied[index].formula else { unreachable!() };
                *rod = 1 - *rod;
            }
            Mutant::WrongConnector => {
                let LoadFormula::TieRodReference { connector, .. } = &mut supplied[index].formula else { unreachable!() };
                *connector += 1;
            }
            Mutant::WrongExtension | Mutant::WrongStiffness | Mutant::WrongOffset => {
                let LoadFormula::TieRodReference { tie_rod, .. } = &mut supplied[index].formula else { unreachable!() };
                *tie_rod = rebuild(tie_rod, &|input: &mut TieRodInput| match mutant {
                    Mutant::WrongExtension => input.reference_extension *= 2.0,
                    Mutant::WrongStiffness => input.stiffness *= 2.0,
                    _ => input.offset_i[1] = -input.offset_i[1],
                });
            }
        }
        true
    }));
}

/// K-D5 closure mutants (§7 item 6) on ER-A (two pretensioned rods, anchored,
/// pressurized): each fails closed on the ordinary and W2 routes in both
/// modes with the named load-closure detail, and the manifest installed
/// from the producer's own enumeration is unchanged by the supplied vector.
#[test]
fn u5b_tie_rod_reference_mutants_fail_closed_on_ordinary_and_w2_routes() {
    let request = request("ER-A");
    let control = case(&request);
    let scaled = control.force.force_scaled(ForceScale::new(4).unwrap()).unwrap();
    assert!(scaled.load_primitives().iter().filter(|p| is_rod_reference(p)).all(|p| p.force_scale_exponent == 4));
    for mutant in [Mutant::Drop, Mutant::Duplicate, Mutant::Extra, Mutant::WrongRod, Mutant::WrongConnector,
        Mutant::WrongExtension, Mutant::WrongStiffness, Mutant::WrongOffset] {
        arm(mutant);
        let mutated = case(&request);
        let term = supplied_mutant::disarm().unwrap_or_else(|| panic!("{mutant:?}: a rod reference term exists"));
        assert_eq!(mutated.force.closure_records()[term].manifest, control.force.closure_records()[term].manifest,
            "{mutant:?}: the manifest is independent of the supplied vector");
        for (route, quality, check, bound) in checked_solves(&mutated, 4) {
            assert_ne!(quality, SolveQuality::Passed, "{mutant:?} {route}");
            assert!(check.contains("closure"), "{mutant:?} {route}: {check}");
            assert!(!bound, "{mutant:?} {route}");
        }
    }
}

// ------------------------------------------------------------------ refusals

/// The captured entry; a request whose numbers its checked-JSON decoder
/// refuses (integral floats beyond 2^53) goes through the typed entry.
fn run(request: &Value) -> Value {
    match run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::SparseInteractive) {
        Ok(envelope) => serde_json::to_value(envelope).unwrap(),
        Err(error) => {
            assert!(error.contains("CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"), "{error}");
            serde_json::to_value(run_linear_static_preview_with_mode(
                serde_json::from_value(request.clone()).unwrap(), PreviewSolverMode::SparseInteractive)).unwrap()
        }
    }
}

fn blocking(source: &Value) -> Vec<String> {
    let mut codes: Vec<String> = source["diagnostics"].as_array().unwrap().iter()
        .filter(|d| d["severity"] == "blocking").map(|d| d["code"].as_str().unwrap().to_string()).collect();
    codes.sort();
    codes.dedup();
    codes
}

fn messages(source: &Value, code: &str) -> String {
    source["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code)
        .map(|d| d["message"].as_str().unwrap().to_string()).collect::<Vec<_>>().join(" | ")
}

fn assert_refused(request: &Value, code: &str, naming: &str, label: &str) {
    let source = run(request);
    assert_eq!(source["status"]["mechanics"], "MODEL_INCOMPLETE", "{label}: {}", source["diagnostics"]);
    assert_eq!(blocking(&source), vec![code.to_string()], "{label}: {}", source["diagnostics"]);
    assert_eq!(source["results"], json!([]), "{label}");
    assert!(messages(&source, code).contains(naming), "{label}: {}", messages(&source, code));
}

fn with_hardware(base: &Value, hardware: Value) -> Value {
    let mut request = base.clone();
    request["model"]["components"][0]["objective_connector"]["hardware"] = hardware;
    request
}

fn with_rod(base: &Value, change: impl Fn(&mut Value)) -> Value {
    let mut request = base.clone();
    change(&mut request["model"]["components"][0]["objective_connector"]["hardware"]["rods"][0]);
    request
}

/// §3.1's named refusals, each alone on a listed pressurized connector (AS-F,
/// so §1's precedence holds: no JOINT_PRESSURE_INTERFACE_UNRESOLVED beside
/// the model finding), MODEL_INCOMPLETE and no results.
#[test]
fn u5b_every_field_refusal_is_named_on_the_connector_with_no_results() {
    let base = request("AS-F");
    let solved = run(&base);
    assert_eq!(solved["status"]["mechanics"], "MECHANICS_SOLVED", "{}", solved["diagnostics"]);
    let rod = base["model"]["components"][0]["objective_connector"]["hardware"]["rods"][0].clone();
    // JOINT_HARDWARE_LAW_UNSUPPORTED names the kind or the law (§3.1).
    for kind in ["collective_axial_constraint_v1", "tied"] {
        assert_refused(&with_hardware(&base, json!({"kind": kind})), "JOINT_HARDWARE_LAW_UNSUPPORTED", kind, kind);
    }
    for law in ["ideal_inextensible", "tension_only", "gapped", "stop"] {
        assert_refused(&with_rod(&base, |r| r["law"] = json!(law)), "JOINT_HARDWARE_LAW_UNSUPPORTED", law, law);
    }
    // JOINT_TIE_ROD_INPUT_INVALID: every closed-object defect.
    let invalid = "JOINT_TIE_ROD_INPUT_INVALID";
    assert_refused(&with_hardware(&base, json!({"kind": "explicit_axial_rods_v1", "rods": []})), invalid, "at least one rod", "empty rods");
    assert_refused(&with_hardware(&base, json!({"kind": "explicit_axial_rods_v1"})), invalid, "rods array", "no rods");
    assert_refused(&with_hardware(&base, json!({"kind": "untied", "rods": [rod.clone()]})), invalid, "contradictory", "untied with rods");
    assert_refused(&with_hardware(&base, json!({"kind": "untied", "nuts": true})), invalid, "unknown member nuts", "untied unknown member");
    assert_refused(&with_hardware(&base, json!({"kind": "explicit_axial_rods_v1", "rods": [rod.clone()], "stops": []})), invalid, "unknown member stops", "hardware unknown member");
    assert_refused(&with_hardware(&base, json!({"kind": "explicit_axial_rods_v1", "rods": [rod.clone(), rod.clone()]})), invalid, "duplicate id", "duplicate id");
    let mut encoded = rod.clone();
    encoded["id"] = json!("rod-yp");
    assert_refused(&with_hardware(&base, json!({"kind": "explicit_axial_rods_v1", "rods": [rod.clone(), encoded]})), invalid, "duplicate id", "duplicate encoded id");
    assert_refused(&with_hardware(&base, json!({"kind": "explicit_axial_rods_v1", "rods": [7]})), invalid, "not an object", "rod not an object");
    for member in ["id", "law", "offset_i", "offset_j", "stiffness", "reference_extension", "source_reference"] {
        assert_refused(&with_rod(&base, |r| { r.as_object_mut().unwrap().remove(member); }), invalid, member, &format!("missing {member}"));
    }
    assert_refused(&with_rod(&base, |r| r["preload"] = json!(1)), invalid, "unknown member preload", "rod unknown member");
    assert_refused(&with_rod(&base, |r| r["source_reference"] = json!(" ")), invalid, "source_reference", "empty source");
    assert_refused(&with_rod(&base, |r| r["id"] = json!("")), invalid, "id", "empty id");
    assert_refused(&with_rod(&base, |r| r["law"] = json!(3)), invalid, "law", "law not a string");
    assert_refused(&with_rod(&base, |r| { r["offset_i"].as_object_mut().unwrap().remove("z"); }), invalid, "offset_i", "offset missing z");
    assert_refused(&with_rod(&base, |r| r["offset_j"]["x"] = json!({"value": 0.1, "unit": "N"})), invalid, "offset_j.x", "offset wrong dimension");
    assert_refused(&with_rod(&base, |r| r["offset_j"]["x"] = json!({"value": 0.1, "unit": "m", "frame": "q"})), invalid, "offset_j.x", "offset not closed");
    for (value, unit, label) in [(0.0, "N/m", "zero stiffness"), (-1.0e7, "N/m", "negative stiffness"), (1.0e7, "m", "stiffness wrong dimension"), (1.0e308, "kN/mm", "stiffness overflows")] {
        assert_refused(&with_rod(&base, |r| r["stiffness"] = json!({"value": value, "unit": unit})), invalid, "stiffness", label);
    }
    assert_refused(&with_rod(&base, |r| r["reference_extension"] = json!({"value": 1.0, "unit": "N"})), invalid, "reference_extension", "extension wrong dimension");
    assert_refused(&with_rod(&base, |r| r["reference_extension"] = json!({"value": 1.0e308, "unit": "km"})), invalid, "reference_extension", "extension overflows");
    // Range (at construction, never a panic): an unrepresentable formed
    // stiffness, and |k·e_ref| below the exact-split floor.
    assert_refused(&with_rod(&base, |r| {
        r["offset_i"]["y"] = json!({"value": 1.0e3, "unit": "m"});
        r["offset_j"]["y"] = json!({"value": 1.0e3, "unit": "m"});
        r["stiffness"] = json!({"value": 1.0e305, "unit": "N/m"});
    }), invalid, "formed stiffness", "formed stiffness range");
    assert_refused(&with_rod(&base, |r| r["reference_extension"] = json!({"value": 1.0e-300, "unit": "m"})), invalid, "reference load", "reference load range");
    assert_refused(&with_rod(&base, |r| r["stiffness"] = json!({"value": 1.0e-310, "unit": "N/m"})), invalid, "stiffness", "subnormal stiffness");
    // JOINT_TIE_ROD_ZERO_LENGTH: the end-i attachment on node C itself.
    let zero = |r: &mut Value| {
        r["offset_i"] = json!({"x": {"value": 0.3, "unit": "m"}, "y": {"value": 0.1, "unit": "m"}, "z": {"value": 0.0, "unit": "m"}});
        r["offset_j"] = json!({"x": {"value": 0.0, "unit": "m"}, "y": {"value": 0.1, "unit": "m"}, "z": {"value": 0.0, "unit": "m"}});
    };
    assert_refused(&with_rod(&base, zero), "JOINT_TIE_ROD_ZERO_LENGTH", "rod:yp", "zero length");
    // Below the representation guard (2^-48 of the scale), not only at zero.
    let near = |r: &mut Value| {
        r["offset_i"] = json!({"x": {"value": 0.3, "unit": "m"}, "y": {"value": 0.1, "unit": "m"}, "z": {"value": 0.0, "unit": "m"}});
        r["offset_j"] = json!({"x": {"value": 1.0e-16, "unit": "m"}, "y": {"value": 0.1, "unit": "m"}, "z": {"value": 0.0, "unit": "m"}});
    };
    assert_refused(&with_rod(&base, near), "JOINT_TIE_ROD_ZERO_LENGTH", "rod:yp", "degenerate length");
}

/// §3.4: a rod nearly perpendicular to the joint axis is admitted (no
/// parallelism threshold); its axial share is published and S_joint's
/// projected term becomes small. Rods on an unpressurized connector are
/// admitted (ruling 2) and publish s_x with Pe = 0 and no pe_x.
#[test]
fn u5b_perpendicular_and_unpressurized_rods_are_admitted() {
    let mut request = request("AS-F");
    let rod = &mut request["model"]["components"][0]["objective_connector"]["hardware"]["rods"][0];
    // The represented difference (x_j − x_i) + (o_j − o_i) has x component
    // fl(2.3 − 2) + (0 − fl(2.3 − 2)) = 0 exactly: the rod runs along y,
    // exactly perpendicular to Q_x, as a transverse elastic element.
    let chord = 2.3f64 - 2.0;
    rod["offset_i"]["x"] = json!({"value": chord, "unit": "m"});
    rod["offset_j"]["x"] = json!({"value": 0.0, "unit": "m"});
    rod["offset_j"]["y"] = json!({"value": 0.4, "unit": "m"});
    let source = run(&request);
    assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED", "{}", source["diagnostics"]);
    let rod = &source["contract_evidence"]["connector"][0]["tie_rods"][0];
    assert_eq!(rod["axial_share"].as_f64(), Some(0.0));
    assert_eq!(rod["direction_global"], json!([0.0, 1.0, 0.0]));
    let unpressurized = run(&self::request("MO-F"));
    assert_eq!(unpressurized["status"]["mechanics"], "MECHANICS_SOLVED");
    let kinds: Vec<&str> = unpressurized["results"].as_array().unwrap().iter().map(|r| r["kind"].as_str().unwrap()).collect();
    assert!(kinds.contains(&"connector_effective_axial_force_v1") && !kinds.contains(&"connector_pressure_force_v1"));
}

/// §3.3: an untied unpressurized connector keeps T4-U3's publication: its 24
/// rows, no tie-rod row, pe_x or s_x, and a record with `hardware` = untied
/// and no `tie_rods` (the corpus test checks its bytes against T4-U3's).
#[test]
fn u5b_untied_unpressurized_connectors_publish_no_rod_rows_or_s_x() {
    let mut request = request("MO-F");
    request["model"]["components"][0]["objective_connector"]["hardware"] = json!({"kind": "untied"});
    let source = run(&request);
    assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED", "{}", source["diagnostics"]);
    let record = &source["contract_evidence"]["connector"][0];
    assert_eq!(record["hardware"], "untied");
    assert!(record.get("tie_rods").is_none());
    assert!(source["results"].as_array().unwrap().iter().all(|r|
        !matches!(r["kind"].as_str(), Some("connector_tie_rod_force_v1" | "connector_effective_axial_force_v1" | "connector_pressure_force_v1"))));
    assert_eq!(source["results"].as_array().unwrap().iter().filter(|r| r["kind"].as_str().unwrap().starts_with("connector_")).count(), 24);
}

/// T4-U5b round 3 (absorption per element, §3.4): a rank-1 rod whose diagonal
/// contribution vanishes against the pipes' along one global axis is not
/// refused, because its other candidates are represented. Each control solves
/// and certifies (NUMERICAL_INTEGRITY_CHECKS_PASSED; ordinary and W2 checked
/// solves Passed with the member bound) in both modes, and the two modes agree
/// on the rod's force and s_x within 1e-9 relative and on its evidence exactly.
#[test]
fn round3_near_perpendicular_and_skewed_rods_solve_and_certify_in_both_modes() {
    let chord = 2.3f64 - 2.0;
    let perpendicular = |dx: f64| move |rod: &mut Value| {
        rod["offset_i"]["x"] = json!({"value": chord, "unit": "m"});
        rod["offset_j"]["x"] = json!({"value": dx, "unit": "m"});
        rod["offset_j"]["y"] = json!({"value": 0.4, "unit": "m"});
    };
    let base = request("AS-F");
    let controls: Vec<(&str, Value, f64)> = vec![
        ("axial share 0", with_rod(&base, perpendicular(0.0)), 0.0),
        ("axial share 1e-16", with_rod(&base, perpendicular(3e-17)), 1e-16),
        ("axial share 1e-8", with_rod(&base, perpendicular(3e-9)), 1e-8),
        ("parallel, 2.5e-12 global-y skew", with_rod(&base, |rod: &mut Value| {
            let y = rod["offset_j"]["y"]["value"].as_f64().unwrap();
            rod["offset_j"]["y"] = json!({"value": y + 1e-12, "unit": "m"});
        }), 1.0),
    ];
    for (label, request, share) in &controls {
        let mut published = Vec::new();
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let source = serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
            assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}: {}", source["diagnostics"]);
            assert!(blocking(&source).is_empty(), "{label} {mode:?}");
            assert!(source["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "NUMERICAL_INTEGRITY_CHECKS_PASSED"), "{label} {mode:?}: {}", source["diagnostics"]);
            let rod = source["contract_evidence"]["connector"][0]["tie_rods"][0].clone();
            let axial = rod["axial_share"].as_f64().unwrap();
            // The represented chord carries its own roundoff (about 2e-16 of
            // the rod length): the noise-level control is checked by order.
            let expected = match *share {
                s if s == 0.0 => axial == 0.0,
                s if s < 1e-15 => axial > 0.0 && axial < 1e-15,
                s => (axial - s).abs() <= 1e-6 * s,
            };
            assert!(expected, "{label}: axial share {axial:e}");
            let row = |kind: &str| source["results"].as_array().unwrap().iter()
                .find(|r| r["kind"] == kind).unwrap_or_else(|| panic!("{label}: {kind}"))["value"].as_f64().unwrap();
            published.push((rod, row("connector_tie_rod_force_v1"), row("connector_effective_axial_force_v1")));
        }
        let ((sparse_rod, t_s, s_s), (dense_rod, t_d, s_d)) = (&published[0], &published[1]);
        assert_eq!(sparse_rod, dense_rod, "{label}: rod evidence parity");
        for (a, b, what) in [(t_s, t_d, "t_r"), (s_s, s_d, "s_x")] {
            assert!((a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0), "{label}: {what} {a:e} vs {b:e}");
        }
        for (route, quality, check, bound) in checked_solves(&case(request), 4) {
            assert_eq!(quality, SolveQuality::Passed, "{label} {route}: {check}");
            assert!(bound, "{label} {route}: member bound");
        }
    }
}

/// The request in millimetres: every node position times 1000 and
/// `project.units.length` = "mm"; the rods' offsets stay in metres.
fn in_millimetres(base: &Value) -> Value {
    let mut request = base.clone();
    request["model"]["project"]["units"]["length"] = json!("mm");
    for node in request["model"]["nodes"].as_array_mut().unwrap() {
        for axis in ["x", "y", "z"] {
            let v = node["position"][axis].as_f64().unwrap();
            node["position"][axis] = json!(v * 1000.0);
        }
    }
    request
}

/// Round 4 (units): before `normalize_model_units` (`validate_profile` on
/// the solve path, and authoring) the rod formation runs on SI positions.
/// - A rod 299.7 m long in SI, (2.3 - 2.0) + (-300 - 0), is admitted and
///   solves; mixing millimetre positions with metre offsets gives
///   300 + (-300) = 0 and refused it as degenerate.
/// - A rod whose SI chord cancels to roundoff, (2.3 - 2.0) + (0 - fl(2.3 -
///   2.0)), is refused by authoring with the degenerate-length code; mixed
///   units saw 299.7 and admitted it.
/// - AS-F authored in millimetres solves with the metre case's rod force.
#[test]
fn round4_rod_formation_uses_si_positions_in_a_millimetre_project() {
    let base = request("AS-F");
    let long = with_rod(&base, |rod| {
        rod["offset_i"]["x"] = json!({"value": 0.0, "unit": "m"});
        rod["offset_j"]["x"] = json!({"value": -300.0, "unit": "m"});
    });
    let long_mm = run(&in_millimetres(&long));
    assert_eq!(long_mm["status"]["mechanics"], "MECHANICS_SOLVED", "{}", long_mm["diagnostics"]);
    let length = long_mm["contract_evidence"]["connector"][0]["tie_rods"][0]["length_m"].as_f64().unwrap();
    assert!((length - 299.7).abs() < 1e-9, "{length}");
    let degenerate = with_rod(&base, |rod| {
        rod["offset_i"]["x"] = json!({"value": 2.3f64 - 2.0, "unit": "m"});
        rod["offset_j"]["x"] = json!({"value": 0.0, "unit": "m"});
    });
    let findings = crate::joint::authoring_findings(&in_millimetres(&degenerate)["model"]).unwrap();
    assert!(findings.iter().any(|d| d.code == "JOINT_TIE_ROD_ZERO_LENGTH" && d.message.contains("rod:yp")), "{findings:?}");
    assert_refused(&in_millimetres(&degenerate), "JOINT_TIE_ROD_ZERO_LENGTH", "rod:yp", "mm degenerate length");
    let force = |source: &Value| source["results"].as_array().unwrap().iter()
        .find(|r| r["kind"] == "connector_tie_rod_force_v1").unwrap()["value"].as_f64().unwrap();
    let metres = run(&base);
    let millimetres = run(&in_millimetres(&base));
    assert_eq!(millimetres["status"]["mechanics"], "MECHANICS_SOLVED", "{}", millimetres["diagnostics"]);
    let (a, b) = (force(&metres), force(&millimetres));
    assert!((a - b).abs() <= 1e-9 * a.abs().max(1.0), "rod force {a:e} m vs {b:e} mm");
}

/// Round 4: the reference-load summand source length-prefixes both ids, so
/// ids that contain ':' cannot collide (the unprefixed `{component}:{rod}`
/// gave both pairs below "component:C-J:rod:x").
#[test]
fn round4_tie_rod_reference_sources_cannot_collide() {
    let a = crate::tie_rod_reference_source("component:C-J:rod", "x");
    let b = crate::tie_rod_reference_source("component:C-J", "rod:x");
    assert_ne!(a, b);
    assert_eq!(a, "17:component:C-J:rod:1:x");
    assert_eq!(b, "13:component:C-J:5:rod:x");
}
