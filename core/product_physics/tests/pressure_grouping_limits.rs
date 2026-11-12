//! Manager public control for explicit unresolved source-factor cancellation.
//! This tests the declared admission guard; it does not add a physical-response oracle.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope,
    PreviewSolverMode,
};
use serde_json::{json, Value};

#[path = "common/current_model.rs"]
mod current_model;
#[path = "common/pressure_elongation.rs"]
mod pressure_elongation;

const CASE: &str = "case:pressure-oracle";
const REGION: &str = "region:pressure-oracle";
const PIPE: &str = "pipe:A-B";
const ROOT: &str = "node:A";
const TIP: &str = "node:B";
const ROOT_SUPPORT: &str = "support:A";
const TIP_SUPPORT: &str = "support:B";
const LENGTH: f64 = 6.0;
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn node(id: &str, position: [f64; 3]) -> Value {
    json!({"id":id,"position":{"x":position[0],"y":position[1],"z":position[2]},
        "provenance":"independent_pressure_runtime_oracle"})
}

fn anchor(id: &str, node_id: &str) -> Value {
    json!({"id":id,"node":node_id,"family":"anchor",
        "restraints":["UX","UY","UZ","RX","RY","RZ"],
        "provenance":"independent_pressure_runtime_oracle"})
}

fn terminal(node_id: &str, transfer: bool) -> Value {
    json!({"node_ref":node_id,"closure_transfer":if transfer {"transfers_to_wall"}
        else {"separately_supported_or_compensated"},
        "provenance":"explicit_independent_pressure_boundary"})
}

fn material(id: &str, e: f64, nu: f64) -> Value {
    json!({"id":id,"constitutive_basis":"homogeneous_isotropic_E_nu_v1",
        "elastic_modulus":{"value":e,"unit":"Pa"},
        "poisson_ratio":{"value":nu,"unit":"1"},
        "thermal_expansion_coefficient":{"value":0.000012,"unit":"1/degC"},
        "temperature_points":[],"provenance":"synthetic_isotropic_reference_no_catalog"})
}

fn pipe(id: &str, from: &str, to: &str, outside: f64, wall: f64, material_id: &str) -> Value {
    json!({"id":id,"from":from,"to":to,
        "section":{"outside_diameter":{"value":outside,"unit":"m"},
            "wall_thickness":{"value":wall,"unit":"m"},
            "corrosion_allowance":{"value":0.0,"unit":"m"}},
        "material":material_id,"y_reference":{"x":0.0,"y":0.0,"z":1.0},
        "provenance":"synthetic_straight_annulus"})
}

fn model(fixed: bool, transfer: bool, thermal: f64) -> Value {
    let mut supports = vec![anchor(ROOT_SUPPORT, ROOT)];
    if fixed {
        supports.push(anchor(TIP_SUPPORT, TIP));
    }
    let loads = if thermal == 0.0 {
        vec![]
    } else {
        vec![json!({"id":"load:thermal","category":"thermal",
            "target":{"type":"element","pipe":PIPE},"direction":"global_x",
            "magnitude":{"value":thermal/0.000012,"unit":"degC"},
            "dimension":"temperature_interval","provenance":"independent_thermal_strain"})]
    };
    json!({"model":{
        "schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"},
        "project":{"id":"project:pressure-oracle","units":{"length":"m","force":"N",
            "angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics",
            "rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":[node(ROOT,[0.0,0.0,0.0]),node(TIP,[LENGTH,0.0,0.0])],
        "pipe_segments":[pipe(PIPE,ROOT,TIP,0.12,0.01,"material:one")],
        "supports":supports,"components":[],
        "materials":[material("material:one",200e9,0.3)],
        "load_cases":[{"id":CASE,"primitive_loads":loads,"pressure_regions":[{
            "id":REGION,"member_pipe_ids":[PIPE],
            "pressure_basis":"internal_differential_zero_external_v1",
            "pressure":{"value":2e6,"unit":"Pa"},
            "terminals":[terminal(ROOT,transfer),terminal(TIP,transfer)],
            "provenance":"independent_pressure_region"}],
            "provenance":"synthetic_load_case"}],"combinations":[]
    },"materials":[]})
}

fn solve(input: Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let request: LinearStaticPreviewRequest = serde_json::from_value(input)
        .expect("the independently authored public DTO must deserialize");
    run_linear_static_preview_with_mode(request, mode)
}

/// The two-pipe geometry this control used before §8.30 correction 1: two
/// parallel pipes between A and B, each its own region, region 2 separately
/// supported at both terminals. Its terminals share A and B, so it is a
/// shared-terminal pressure junction with a dropped cap and is refused by name
/// before assembly (the expected code moved from
/// PRESSURE_ASSEMBLY_CANCELLATION_UNRESOLVED; Agent 0, 2026-10-28).
fn two_parallel_regions() -> Value {
    let mut input = model(false, true, 0.0);
    let second_pipe = pipe("pipe:parallel", ROOT, TIP, 0.16, 0.02, "material:one");
    input["model"]["pipe_segments"]
        .as_array_mut()
        .unwrap()
        .push(second_pipe);
    let mut second = input["model"]["load_cases"][0]["pressure_regions"][0].clone();
    second["id"] = json!("region:other-source");
    second["member_pipe_ids"] = json!(["pipe:parallel"]);
    second["terminals"] = json!([terminal(ROOT, false), terminal(TIP, false)]);
    let nu = 0.3;
    second["pressure"]["value"] =
        json!(2e6 * 25.0 / 36.0 * ((1.0 - 2.0 * nu) / (2.0 * nu)) * (1.0 + 1e-13));
    input["model"]["load_cases"][0]["pressure_regions"]
        .as_array_mut()
        .unwrap()
        .push(second);
    input
}

#[test]
fn strongly_cancelling_distinct_pressure_factors_are_explicitly_unqualified() {
    // One region over two collinear pipes A-M-B, transferring at both
    // terminals, so no terminal is shared. Pressure groups are keyed by
    // pressure and bore. The second pipe's outside diameter is one ulp above
    // the first's: its bore differs within the region's equal-bore guard (64
    // epsilon of the radius), so at M the two members' Poisson groups,
    // -2*nu*p*Ai1 and +2*nu*p*Ai2, are distinct and strongly cancel. With
    // nu = 0.5*(1 - 1e-13) each terminal's cap and Poisson terms share a group
    // whose load (1 - 2*nu)*p*Ai is about 1e-13*p*Ai, so the whole
    // translational RHS is tiny and the guard's ratio is about 1e13.
    let mut input = model(false, true, 0.0);
    let middle = "node:M";
    let document = &mut input["model"];
    document["nodes"].as_array_mut().unwrap().push(node(middle, [LENGTH / 2.0, 0.0, 0.0]));
    document["pipe_segments"] = json!([
        pipe(PIPE, ROOT, middle, 0.12, 0.01, "material:one"),
        pipe("pipe:M-B", middle, TIP, f64::from_bits(0.12_f64.to_bits() + 1), 0.01, "material:one"),
    ]);
    document["materials"] = json!([material("material:one", 200e9, 0.5 * (1.0 - 1e-13))]);
    document["load_cases"][0]["pressure_regions"][0]["member_pipe_ids"] = json!([PIPE, "pipe:M-B"]);
    // The coefficient 2*nu above is the Poisson pair of a case that includes
    // pressure elongation, so the case names itself in the setting. Without
    // it the pair is the cap pair alone (coefficient 1), every terminal group
    // cancels exactly and this would not be the strong cancellation it controls.
    document["model_settings"] = json!({"pressure_elongation": {"operating_load_cases": [CASE]}});
    for mode in MODES {
        let output = solve(input.clone(), mode);
        assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
        assert!(output.results.is_empty());
        assert!(
            output
                .diagnostics
                .iter()
                .any(|d| d.code == "PRESSURE_ASSEMBLY_CANCELLATION_UNRESOLVED"),
            "{:?}",
            output.diagnostics
        );
    }
}

#[test]
fn the_former_two_parallel_region_control_is_a_refused_shared_terminal_junction() {
    for mode in MODES {
        let output = solve(two_parallel_regions(), mode);
        assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE");
        assert!(output.results.is_empty());
        let codes = output.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| (d.code.as_str(), d.affected_refs.clone())).collect::<Vec<_>>();
        assert_eq!(
            codes,
            [
                ("EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED", vec![CASE.to_string(), ROOT.to_string(), "region:other-source".to_string()]),
                ("EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED", vec![CASE.to_string(), TIP.to_string(), "region:other-source".to_string()]),
            ],
            "{mode:?}"
        );
    }
}

/// One region over the chain `points` (pipe k from point k to k+1, outside
/// diameter `outside[k]`, wall 0.01 m), anchored at the listed points,
/// transferring at both terminals, elongation not included.
fn chain(points: &[[f64; 3]], outside: &[f64], anchored: &[usize]) -> Value {
    let mut input = model(false, true, 0.0);
    let names = (0..points.len()).map(|k| format!("node:C{k}")).collect::<Vec<_>>();
    let pipes = (0..outside.len()).map(|k| format!("pipe:C{k}")).collect::<Vec<_>>();
    let document = &mut input["model"];
    document["nodes"] = json!(points.iter().zip(&names).map(|(p, n)| node(n, *p)).collect::<Vec<_>>());
    document["pipe_segments"] = json!(pipes.iter().enumerate()
        .map(|(k, id)| pipe(id, &names[k], &names[k + 1], outside[k], 0.01, "material:one")).collect::<Vec<_>>());
    document["supports"] = json!(anchored.iter().map(|k| anchor(&format!("support:C{k}"), &names[*k])).collect::<Vec<_>>());
    let region = &mut document["load_cases"][0]["pressure_regions"][0];
    region["member_pipe_ids"] = json!(pipes);
    region["terminals"] = json!([terminal(&names[0], true), terminal(names.last().unwrap(), true)]);
    input
}

fn next_up(value: f64, ulps: u64) -> f64 {
    f64::from_bits(value.to_bits() + ulps)
}

/// A skew collinear run of length 3 m whose interior node sits at `t` of the
/// span, displaced normal to it by `offset` m.
fn skew(t: f64, offset: f64) -> Vec<[f64; 3]> {
    let (u, n) = ([2.0 / 13f64.sqrt(), 3.0 / 13f64.sqrt()], [-3.0 / 13f64.sqrt(), 2.0 / 13f64.sqrt()]);
    vec![[0.0; 3], [3.0 * t * u[0] + offset * n[0], 3.0 * t * u[1] + offset * n[1], 0.0], [3.0 * u[0], 3.0 * u[1], 0.0]]
}

fn values<'a>(output: &'a Value, kind: &str) -> Vec<f64> {
    output["results"].as_array().unwrap().iter().filter(|r| r["kind"] == kind).map(|r| r["value"].as_f64().unwrap()).collect()
}

const EXCEPTION_BASIS: &str = "straight_region_interior_cap_pairs_without_pressure_elongation";

fn assembly(output: &Value) -> &Value {
    &output["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]
}

/// How a solved case's cancellation screen was judged. `true`: the original
/// screen failed and the case was admitted by the exception, whose record
/// states its family and a measured ratio within its limit. `false`: the
/// original screen passed and no exception record is published.
fn assert_screen(output: &Value, by_exception: bool, label: &str) {
    let assembly = assembly(output);
    let screen = &assembly["cancellation_screen"];
    let record = assembly.get("cancellation_screen_exception");
    if by_exception {
        assert!(screen.is_null() || screen.as_f64().unwrap() > 1e-9, "{label}: the original screen {screen} must have failed");
        let record = record.unwrap_or_else(|| panic!("{label}: exception record"));
        assert_eq!(record["basis"], EXCEPTION_BASIS, "{label}");
        assert_eq!(record["residual_to_source_ratio_limit"], 1e-9, "{label}");
        let ratio = record["residual_to_source_ratio"].as_f64().unwrap();
        assert!((0.0..=1e-9).contains(&ratio), "{label}: ratio {ratio}");
    } else {
        assert!((0.0..=1e-9).contains(&screen.as_f64().unwrap()), "{label}: screen {screen}");
        assert!(record.is_none(), "{label}: no exception record beside a passing screen");
    }
}

fn assert_refused(input: &Value, label: &str) {
    for mode in MODES {
        let output = solve(input.clone(), mode);
        assert_eq!(output.status.mechanics, "MODEL_INCOMPLETE", "{label} {mode:?}");
        assert!(output.results.is_empty(), "{label}");
        let blocking = output.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect::<Vec<_>>();
        assert_eq!(blocking, ["PRESSURE_ASSEMBLY_CANCELLATION_UNRESOLVED"], "{label} {mode:?}");
        // A refused case publishes no assembly evidence, so no exception record either.
        assert!(output.contract_evidence.as_ref().is_none_or(|e| e["exact_cases"].as_array().is_none_or(|c| c.is_empty())), "{label}");
    }
}

/// An anchored closed straight run without pressure elongation. What the
/// exception bounds is the assembled load: at most 1e-9 of the source terms
/// that cancelled to form it, wherever it is formed. The response is
/// observed, not bounded by it: for these small, well-conditioned models the
/// reactions and the effective force S are within 1e-9*p*Ai of the closed form
/// 0, and the wall force N_w within 1e-9*p*Ai of the closed form p*Ai.
fn assert_closed_off(input: &Value, by_exception: bool, label: &str) -> Value {
    let pai = 2e6 * std::f64::consts::PI * 0.05 * 0.05;
    let mut last = Value::Null;
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}: {}", output["diagnostics"]);
        assert_eq!(assembly(&output)["pressure_elongation"], "not_included", "{label}");
        assert_screen(&output, by_exception, label);
        for value in values(&output, "support_reaction_force_magnitude_v2") {
            assert!(value.abs() <= 1e-9 * pai, "{label} {mode:?}: reaction {value}");
        }
        for value in values(&output, "pipe_effective_axial_force_v2") {
            assert!(value.abs() <= 1e-9 * pai, "{label} {mode:?}: S {value}");
        }
        let walls = values(&output, "pipe_wall_axial_force_v2");
        assert!(!walls.is_empty());
        for value in walls {
            // Ai on each member's own bore, equal to 0.05 m within the region's guard.
            assert!((value - pai).abs() <= 1e-9 * pai, "{label} {mode:?}: Nw {value}");
        }
        last = output;
    }
    last
}

#[test]
fn a_closed_straight_run_whose_members_are_equal_within_the_region_guards_is_admitted_without_elongation() {
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    // Bit-equal members: every pressure group cancels exactly and the
    // original screen passes; no exception is taken.
    assert_closed_off(&chain(&x, &[0.12, 0.12], &[0, 2]), false, "bit-equal");
    // Outside diameters 1 and 60 ulps apart: bores equal within 64 epsilon.
    assert_closed_off(&chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]), true, "1 ulp");
    assert_closed_off(&chain(&x, &[0.12, next_up(0.12, 60)], &[0, 2]), true, "60 ulps");
    // Three members with mixed ulps: two interior nodes, each a cap pair.
    let three = [[0.0; 3], [2.0, 0.0, 0.0], [4.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    assert_closed_off(&chain(&three, &[0.12, next_up(0.12, 1), next_up(0.12, 3)], &[0, 3]), true, "three members");
    // Directions equal within the collinearity guard, the interior node 1/1000
    // of the span from a terminal and 50 epsilon of the span off the chord.
    assert_closed_off(&chain(&skew(1e-3, 50.0 * f64::EPSILON * 3.0), &[0.12, 0.12], &[0, 2]), true, "skew");
}

#[test]
fn a_cantilevered_closed_run_admitted_without_elongation_is_observed_not_to_move() {
    // Anchored at one end only, outside diameters one ulp apart. Observed for
    // this model: the forces as in `assert_closed_off`, and every nodal
    // translation within the axial stretch that a load of 1e-9*p*Ai would
    // give the whole run, 1e-9*p*Ai*L/(E*As).
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let input = chain(&x, &[0.12, next_up(0.12, 1)], &[0]);
    assert_closed_off(&input, true, "cantilever");
    let pai = 2e6 * std::f64::consts::PI * 0.05 * 0.05;
    let area = std::f64::consts::PI * (0.06 * 0.06 - 0.05 * 0.05);
    let stretch = 1e-9 * pai * LENGTH / (200e9 * area);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        let moves = output["results"].as_array().unwrap().iter()
            .filter(|r| ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"].contains(&r["kind"].as_str().unwrap()))
            .map(|r| r["value"].as_f64().unwrap())
            .collect::<Vec<_>>();
        assert!(!moves.is_empty(), "{mode:?}");
        for value in moves {
            assert!(value.abs() <= stretch, "{mode:?}: translation {value} beyond {stretch}");
        }
    }
}

#[test]
fn a_residual_above_one_billionth_of_its_source_terms_stays_unqualified_without_elongation() {
    // The interior node 1e-6 of the span from a terminal, 50 epsilon of the
    // span off the chord: admitted by the collinearity guard and of the
    // exception's family, but the members' directions differ by about 1e-8,
    // so the cap residual there is above 1e-9 of the caps that formed it.
    assert_refused(&chain(&skew(1e-6, 50.0 * f64::EPSILON * 3.0), &[0.12, 0.12], &[0, 2]), "short member");
}

/// Two or three collinear-or-branching regions meeting at node J, every
/// terminal transferring: region R1 over A-M-J with outside diameters one ulp
/// apart at M, region R2 over J-K with outside diameter `second`, and, when
/// `tee`, region R3 over J-T normal to the run. Every cap pair at J cancels
/// in its own group, so the only formed load is the residual at M: the
/// original screen fails and the ratio alone would pass, but J is a junction
/// of regions and the case is not of the exception's family.
fn junction(second: f64, tee: bool) -> Value {
    let mut input = model(false, true, 0.0);
    let document = &mut input["model"];
    let mut nodes = vec![node("node:A", [0.0; 3]), node("node:M", [3.0, 0.0, 0.0]), node("node:J", [LENGTH, 0.0, 0.0]), node("node:K", [9.0, 0.0, 0.0])];
    let mut pipes = vec![
        pipe("pipe:A-M", "node:A", "node:M", 0.12, 0.01, "material:one"),
        pipe("pipe:M-J", "node:M", "node:J", next_up(0.12, 1), 0.01, "material:one"),
        pipe("pipe:J-K", "node:J", "node:K", second, 0.01, "material:one"),
    ];
    let mut supports = vec![anchor("support:A", "node:A"), anchor("support:K", "node:K")];
    let region = |id: &str, members: &[&str], from: &str, to: &str| json!({"id":id,"member_pipe_ids":members,
        "pressure_basis":"internal_differential_zero_external_v1","pressure":{"value":2e6,"unit":"Pa"},
        "terminals":[terminal(from,true),terminal(to,true)],"provenance":"independent_pressure_region"});
    let mut regions = vec![region("region:R1", &["pipe:A-M", "pipe:M-J"], "node:A", "node:J"), region("region:R2", &["pipe:J-K"], "node:J", "node:K")];
    if tee {
        nodes.push(node("node:T", [LENGTH, 3.0, 0.0]));
        pipes.push(pipe("pipe:J-T", "node:J", "node:T", 0.12, 0.01, "material:one"));
        supports.push(anchor("support:T", "node:T"));
        regions.push(region("region:R3", &["pipe:J-T"], "node:J", "node:T"));
    }
    document["nodes"] = json!(nodes);
    document["pipe_segments"] = json!(pipes);
    document["supports"] = json!(supports);
    document["load_cases"][0]["pressure_regions"] = json!(regions);
    input
}

#[test]
fn a_junction_of_regions_keeps_the_original_screen_alone() {
    // A second region's term at the node, a reducer step between two regions,
    // and a tee junction.
    assert_refused(&junction(next_up(0.12, 1), false), "second region at J");
    assert_refused(&junction(0.14, false), "reducer step at J");
    assert_refused(&junction(0.12, true), "tee at J");
}

#[test]
fn a_region_with_a_realized_bend_keeps_the_original_screen_alone() {
    // The arc reader corpus's L line (S1, a realized bend, S2), not including
    // pressure elongation, with S2 split at its midpoint into two members one
    // ulp apart in outside diameter. Every bend term and kink remainder
    // cancels exactly in its own group, so the only formed load is the
    // residual at the split: the original screen fails and the ratio alone
    // would pass, but the region has a bend and kink remainders.
    let corpus: Value = serde_json::from_str(&std::fs::read_to_string(
        current_model::project_root().join("fixtures/results/pressure_v3_arc_reader_corpus.json")).unwrap()).unwrap();
    let case = corpus["cases"].as_array().unwrap().iter().find(|c| c["label"] == "U2-L-ANCH-PTW-K2 0.3.0 sparse_interactive").unwrap();
    let mut document = pressure_elongation::without_elongation(&case["document"]);
    let model = &mut document["model"];
    let mut split = model["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == "pipe:S2").unwrap().clone();
    assert_eq!((split["from"].as_str(), split["to"].as_str()), (Some("node:C"), Some("node:D")));
    let mut middle = model["nodes"].as_array().unwrap().iter().find(|n| n["id"] == "node:D").unwrap().clone();
    middle["id"] = json!("node:E");
    middle["position"]["y"] = json!(2.25);
    model["nodes"].as_array_mut().unwrap().push(middle);
    let mut second = split.clone();
    split["id"] = json!("pipe:S2a");
    split["to"] = json!("node:E");
    second["id"] = json!("pipe:S2b");
    second["from"] = json!("node:E");
    let od = second["section"]["outside_diameter"]["value"].as_f64().unwrap();
    second["section"]["outside_diameter"]["value"] = json!(next_up(od, 1));
    let pipes = model["pipe_segments"].as_array_mut().unwrap();
    pipes.retain(|p| p["id"] != "pipe:S2");
    pipes.push(split);
    pipes.push(second);
    let case = &mut model["load_cases"][0];
    case["primitive_loads"] = json!([]);
    case["pressure_regions"][0]["member_pipe_ids"] = json!(["pipe:S1", "pipe:BEND", "pipe:S2a", "pipe:S2b"]);
    assert_refused(&document, "bend");
}

#[test]
fn the_readers_recompute_the_exception_from_the_published_groups() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let three = [[0.0; 3], [2.0, 0.0, 0.0], [4.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let admitted = [
        chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]),
        chain(&three, &[0.12, next_up(0.12, 1), next_up(0.12, 3)], &[0, 3]),
        chain(&skew(1e-3, 50.0 * f64::EPSILON * 3.0), &[0.12, 0.12], &[0, 2]),
    ];
    const RHS: &str = "/contract_evidence/exact_cases/0/pressure_rhs_assembly";
    let refused = |output: &Value, pointer: &str, value: Option<Value>| {
        let mut changed = output.clone();
        let (parent, key) = pointer.rsplit_once('/').unwrap();
        let object = changed.pointer_mut(parent).unwrap().as_object_mut().unwrap();
        match value {
            Some(value) => object.insert(key.to_string(), value),
            None => object.remove(key),
        };
        validate_pressure_evidence(&changed).unwrap_err()
    };
    for (index, input) in admitted.iter().enumerate() {
        for mode in MODES {
            let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
            assert_screen(&output, true, "admitted");
            validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{index} {mode:?}: {e}"));
            let ratio = output.pointer(&format!("{RHS}/cancellation_screen_exception/residual_to_source_ratio")).unwrap().as_f64().unwrap();
            let screen = output.pointer(&format!("{RHS}/cancellation_screen")).unwrap().as_f64().unwrap();
            let exception = format!("{RHS}/cancellation_screen_exception");
            assert_eq!(refused(&output, &exception, None), "SOURCE_PHYSICS_RHS_SCREEN");
            assert_eq!(refused(&output, &format!("{exception}/basis"), Some(json!("source_group_magnitudes"))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION");
            assert_eq!(refused(&output, &format!("{exception}/residual_to_source_ratio_limit"), Some(json!(1e-8))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION");
            assert_eq!(refused(&output, &format!("{exception}/extra"), Some(json!(0))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION");
            assert_eq!(refused(&output, &format!("{exception}/residual_to_source_ratio"), Some(json!(next_up(ratio, 1)))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_RATIO");
            assert_eq!(refused(&output, &format!("{exception}/residual_to_source_ratio"), Some(json!(2e-9))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_RATIO");
            assert_eq!(refused(&output, &format!("{RHS}/cancellation_screen"), Some(json!(1e-12))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BESIDE_PASSING_SCREEN");
            assert_eq!(refused(&output, &format!("{RHS}/cancellation_screen"), Some(json!(next_up(screen, 1)))), "SOURCE_PHYSICS_RHS_SCREEN_RECOMPUTED");
            assert_eq!(refused(&output, &format!("{RHS}/cancellation_screen"), Some(Value::Null)), "SOURCE_PHYSICS_RHS_SCREEN_RECOMPUTED");
            assert_eq!(refused(&output, &format!("{RHS}/cancellation_screen"), Some(json!("over"))), "SOURCE_PHYSICS_RHS_SCREEN");
        }
    }
    // A passing screen: no record may stand beside it, and a forged failing
    // screen with a record over a case outside the family is refused.
    for mode in MODES {
        let output = serde_json::to_value(solve(chain(&x, &[0.12, 0.12], &[0, 2]), mode)).unwrap();
        validate_pressure_evidence(&output).unwrap();
        let record = json!({"basis": EXCEPTION_BASIS, "residual_to_source_ratio": 0.0, "residual_to_source_ratio_limit": 1e-9});
        assert_eq!(refused(&output, &format!("{RHS}/cancellation_screen_exception"), Some(record.clone())), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BESIDE_PASSING_SCREEN");
        let mut open = chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]);
        open["model"]["load_cases"][0]["pressure_regions"][0]["terminals"][1]["closure_transfer"] = json!("separately_supported_or_compensated");
        let mut forged = serde_json::to_value(solve(open, mode)).unwrap();
        assert_screen(&forged, false, "a terminal that does not transfer");
        forged.pointer_mut(RHS).unwrap()["cancellation_screen"] = json!(1.0);
        assert_eq!(refused(&forged, &format!("{RHS}/cancellation_screen_exception"), Some(record)), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY");
    }
}
