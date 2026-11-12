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

const EXCEPTION_BASIS: &str = "straight_region_interior_cap_pairs_representation_guard_departure_thrust";

fn assembly(output: &Value) -> &Value {
    &output["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]
}

/// How a solved case's cancellation screen was judged. `true`: the original
/// screen failed and the case was admitted by the exception, whose record
/// names its family and states a measured |F|/(|p|·A_hi) within the
/// consistency bound beside it. `false`: the original screen passed and no
/// exception record is published. Returns the record (or null).
fn assert_screen(output: &Value, by_exception: bool, label: &str) -> Value {
    let assembly = assembly(output);
    let screen = &assembly["cancellation_screen"];
    let record = assembly.get("cancellation_screen_exception");
    if by_exception {
        assert!(screen.is_null() || screen.as_f64().unwrap() > 1e-9, "{label}: the original screen {screen} must have failed");
        let record = record.unwrap_or_else(|| panic!("{label}: exception record"));
        assert_eq!(record["basis"], EXCEPTION_BASIS, "{label}");
        let ratio = record["thrust_to_bore_force_ratio"].as_f64().unwrap();
        let bound = record["consistency_bound_ratio"].as_f64().unwrap();
        assert!(ratio >= 0.0 && ratio <= bound, "{label}: ratio {ratio} bound {bound}");
        record.clone()
    } else {
        assert!((0.0..=1e-9).contains(&screen.as_f64().unwrap()), "{label}: screen {screen}");
        assert!(record.is_none(), "{label}: no exception record beside a passing screen");
        Value::Null
    }
}

/// The standing the product publishes for the (one) case: its solve quality,
/// and whether its integrity diagnostic names the formation of the loads as
/// the reason (`formation_check: reason=estimate`, K-D5's re-formation of
/// every pressure operand, or the S11-G load-row guard's formation-noise
/// sentence naming the region). An observation recorded per model (DEL-04-07);
/// the standing logic is DEL-04-01's.
fn standing(output: &Value) -> (String, bool) {
    let quality = output["numerical_quality"]["cases"][0]["solve_quality"].as_str().unwrap().to_owned();
    let message = output["diagnostics"].as_array().unwrap().iter()
        .find(|d| d["id"].as_str().is_some_and(|id| id.starts_with("diagnostic:numerical-integrity:")))
        .map_or("", |d| d["message"].as_str().unwrap());
    let formation = message.contains("formation_check: reason=estimate") || message.contains("S11-G formation-noise guard");
    (quality, formation)
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

/// An anchored closed straight run without pressure elongation. The formed
/// load at an interior node is the thrust of the bore and alignment
/// differences the region guards admit, and the record's bound only checks
/// that it is consistent with them. The response is observed, not bounded by
/// it: for these small models the reactions and the effective force S are
/// within 1e-9*p*Ai of the closed form 0 of the idealised reference with
/// equal bores and exact collinearity, and the wall force N_w within 1e-9*p*Ai
/// of its closed form p*Ai. `expected` is the standing the product publishes
/// (solve quality, and whether the integrity diagnostic names the loads'
/// formation as its reason), recorded for each model.
fn assert_closed_off(input: &Value, by_exception: bool, expected: (&str, bool), label: &str) -> Value {
    let pai = 2e6 * std::f64::consts::PI * 0.05 * 0.05;
    let mut last = Value::Null;
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}: {}", output["diagnostics"]);
        assert_eq!(assembly(&output)["pressure_elongation"], "not_included", "{label}");
        assert_screen(&output, by_exception, label);
        let (quality, formation) = standing(&output);
        assert_eq!((quality.as_str(), formation), expected, "{label} {mode:?}");
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
    // original screen passes; no exception is taken. Published established.
    assert_closed_off(&chain(&x, &[0.12, 0.12], &[0, 2]), false, ("checks_passed", false), "bit-equal");
    // Outside diameters 1 and 60 ulps apart: bores equal within 64 epsilon.
    // Each is published Sensitive because K-D5, re-forming every pressure
    // operand from the recorded geometry, finds the formation of the
    // interior thrust too uncertain relative to the response it drives.
    assert_closed_off(&chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]), true, ("sensitive", true), "1 ulp");
    assert_closed_off(&chain(&x, &[0.12, next_up(0.12, 60)], &[0, 2]), true, ("sensitive", true), "60 ulps");
    // Three members with mixed ulps: two interior nodes, each a cap pair.
    let three = [[0.0; 3], [2.0, 0.0, 0.0], [4.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    assert_closed_off(&chain(&three, &[0.12, next_up(0.12, 1), next_up(0.12, 3)], &[0, 3]), true, ("sensitive", true), "three members");
    // Directions equal within the collinearity guard, the interior node 1/1000
    // of the span from a terminal and 50 epsilon of the span off the chord.
    assert_closed_off(&chain(&skew(1e-3, 50.0 * f64::EPSILON * 3.0), &[0.12, 0.12], &[0, 2]), true, ("sensitive", true), "skew");
}

#[test]
fn a_cantilevered_closed_run_admitted_without_elongation_is_observed_not_to_move() {
    // Anchored at one end only, outside diameters one ulp apart. Observed for
    // this model: the forces as in `assert_closed_off`, and every nodal
    // translation within the axial stretch that a load of 1e-9*p*Ai would
    // give the whole run, 1e-9*p*Ai*L/(E*As). Published Sensitive, K-D5 naming
    // the loads' formation.
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let input = chain(&x, &[0.12, next_up(0.12, 1)], &[0]);
    assert_closed_off(&input, true, ("sensitive", true), "cantilever");
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
fn a_member_of_a_millionth_of_the_span_within_the_collinearity_guard_is_admitted_without_elongation() {
    // The interior node 1e-6 of the span from a terminal, 50 epsilon of the
    // span off the chord: admitted by the collinearity guard, which is
    // relative to the span and not to a member's length, and of the
    // exception's family. The members' directions differ by about 1e-8, so
    // the thrust at the interior node is about 1e-8 of the caps that form it.
    // The former constant 1e-9 refused it; it had no derivation from the
    // guards, and the bound derived from them admits what they admit (the
    // thrust is 0.33 of it here). Published Sensitive: the solve's condition
    // estimate is below the square root of epsilon, so K-D5 does not run and
    // the diagnostic names the conditioning, not the loads' formation.
    let input = chain(&skew(1e-6, 50.0 * f64::EPSILON * 3.0), &[0.12, 0.12], &[0, 2]);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", output["diagnostics"]);
        let record = assert_screen(&output, true, "short member");
        assert_eq!(record["node_ref"], "node:C1");
        assert_eq!(standing(&output), ("sensitive".to_owned(), false), "{mode:?}");
    }
}

/// DEL-04-07, the consistency bound at the guards' edges: the members at an
/// interior node differ in bore by twice the equal-bore guard (one at +64ε of
/// the reference radius, the other at −64ε), and the node lies at the
/// collinearity guard's edge with a short member, its neighbours on the other
/// side of the chord, at every scale of pressure and either sign. Each is
/// admitted, its measured ratio within the bound, and the record's ratio and
/// bound do not depend on p.
#[test]
fn the_consistency_bound_holds_at_the_guards_edges_for_every_pressure() {
    let reference = 0.12_f64;
    let edge = |direction: f64| {
        let radius = |od: f64| od * 0.5 - 0.01;
        let admits = |od: f64| (radius(reference) - radius(od)).abs() <= 64.0 * f64::EPSILON * radius(reference).max(radius(od));
        let mut od = reference;
        loop {
            let next = if direction > 0.0 { od.next_up() } else { od.next_down() };
            if !admits(next) { return od; }
            od = next;
        }
    };
    let (up, down) = (edge(1.0), edge(-1.0));
    let four = [[0.0; 3], [2.0, 0.0, 0.0], [4.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let output = serde_json::to_value(solve(chain(&four, &[reference, up, down], &[0, 3]), MODES[0])).unwrap();
    let record = assert_screen(&output, true, "pairwise bores");
    // The bore term alone: up to 2*64ε of the radius between the two members (each
    // against the reference, on opposite sides), so above one guard's 128ε of the area.
    let ratio = record["thrust_to_bore_force_ratio"].as_f64().unwrap();
    assert!(ratio > 128.0 * f64::EPSILON, "pairwise bores: ratio {ratio}");
    // A short member at the collinearity guard's edge, on the other side of the chord from its neighbour.
    let (u, n) = ([2.0 / 13f64.sqrt(), 3.0 / 13f64.sqrt()], [-3.0 / 13f64.sqrt(), 2.0 / 13f64.sqrt()]);
    let at = |t: f64, offset: f64| [LENGTH * t * u[0] + offset * n[0], LENGTH * t * u[1] + offset * n[1], 0.0];
    let off = 63.9 * f64::EPSILON * LENGTH;
    let points = [[0.0; 3], at(0.5, off), at(0.5 + 1e-3 / LENGTH, -off), at(1.0, 0.0)];
    let mut first: Option<Value> = None;
    for p in [2e6, -2e6, 1.5, 1e-100, 4503599627370495.5] {
        let mut input = chain(&points, &[reference, up, down], &[0, 3]);
        input["model"]["load_cases"][0]["pressure_regions"][0]["pressure"]["value"] = json!(p);
        let output = serde_json::to_value(solve(input, MODES[0])).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "p={p}: {}", output["diagnostics"]);
        let record = assert_screen(&output, true, &format!("guard edge p={p}"));
        let used = record["thrust_to_bore_force_ratio"].as_f64().unwrap() / record["consistency_bound_ratio"].as_f64().unwrap();
        assert!(used > 0.5 && used < 1.0, "p={p}: used {used}");
        assert_eq!(standing(&output).0, "sensitive", "p={p}");
        match &first {
            None => first = Some(record),
            Some(base) => {
                assert_eq!(record["consistency_bound_ratio"], base["consistency_bound_ratio"], "p={p}");
                let (a, b) = (record["thrust_to_bore_force_ratio"].as_f64().unwrap(), base["thrust_to_bore_force_ratio"].as_f64().unwrap());
                assert!((a - b).abs() <= 1e-5 * b, "p={p}: {a} against {b}");
            }
        }
    }
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
    let changed = |output: &Value, change: &dyn Fn(&mut Value)| {
        let mut changed = output.clone();
        change(changed.pointer_mut(RHS).unwrap());
        changed
    };
    let refused = |output: &Value, change: &dyn Fn(&mut Value)| validate_pressure_evidence(&changed(output, change)).unwrap_err();
    let set = |pointer: &str, value: Option<Value>| {
        let pointer = pointer.to_string();
        move |rhs: &mut Value| {
            let (parent, key) = pointer.rsplit_once('/').unwrap();
            let object = if parent.is_empty() { rhs.as_object_mut().unwrap() } else { rhs.pointer_mut(parent).unwrap().as_object_mut().unwrap() };
            match &value {
                Some(value) => object.insert(key.to_string(), value.clone()),
                None => object.remove(key),
            };
        }
    };
    // The interior groups of the record's node.
    let interior = |rhs: &Value| -> Vec<usize> {
        let node = rhs["cancellation_screen_exception"]["node_ref"].clone();
        rhs["groups"].as_array().unwrap().iter().enumerate().filter(|(_, g)| g["node_ref"] == node).map(|(i, _)| i).collect()
    };
    for (index, input) in admitted.iter().enumerate() {
        for mode in MODES {
            let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
            let record = assert_screen(&output, true, "admitted");
            validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{index} {mode:?}: {e}"));
            let model = &input["model"];
            let verdict = |document: &Value| open_pipe_stress_result_export::reader_verdict::validate("pressure", document, Some(model), None);
            assert_eq!(verdict(&output)["ok"], true, "{index} {mode:?}: {}", verdict(&output));
            let ratio = record["thrust_to_bore_force_ratio"].as_f64().unwrap();
            let bound = record["consistency_bound_ratio"].as_f64().unwrap();
            let screen = output.pointer(&format!("{RHS}/cancellation_screen")).unwrap().as_f64().unwrap();
            let rhs = output.pointer(RHS).unwrap();
            let groups = interior(rhs);
            // The record and the original screen.
            assert_eq!(refused(&output, &set("/cancellation_screen_exception", None)), "SOURCE_PHYSICS_RHS_SCREEN");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/basis", Some(json!("straight_region_interior_cap_pairs_without_pressure_elongation")))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/extra", Some(json!(0)))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/thrust_to_bore_force_ratio", Some(json!(next_up(ratio, 1))))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_RATIO");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/consistency_bound_ratio", Some(json!(next_up(bound, 1))))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BOUND");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/node_ref", Some(rhs["node_order"][0].clone()))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_NODE");
            let mut swapped = record["member_pipe_ids"].clone();
            swapped.as_array_mut().unwrap().reverse();
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/member_pipe_ids", Some(swapped))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_NODE");
            // A span a millionth of the published one gives a smaller bound than the record's.
            let span = record["region_span_m"].as_f64().unwrap();
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/region_span_m", Some(json!(span * 1e-6)))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BOUND");
            // A length one ulp off may leave the bound's bits unchanged: from the
            // envelope alone the lengths are the record's; given the model, they are refused.
            let mut lengths = record["member_lengths_m"].clone();
            lengths[0] = json!(next_up(lengths[0].as_f64().unwrap(), 1));
            let moved = changed(&output, &set("/cancellation_screen_exception/member_lengths_m", Some(lengths)));
            let expected = match validate_pressure_evidence(&moved) {
                Ok(()) => "SOURCE_PHYSICS_MODEL_EXCEPTION_GEOMETRY",
                Err(code) => {
                    assert_eq!(code, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BOUND", "{index} {mode:?}");
                    "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BOUND"
                }
            };
            assert_eq!(verdict(&moved)["error"]["code"], expected, "{index} {mode:?}");
            assert_eq!(refused(&output, &set("/cancellation_screen", Some(json!(1e-12)))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BESIDE_PASSING_SCREEN");
            assert_eq!(refused(&output, &set("/cancellation_screen", Some(json!(next_up(screen, 1))))), "SOURCE_PHYSICS_RHS_SCREEN_RECOMPUTED");
            assert_eq!(refused(&output, &set("/cancellation_screen", Some(Value::Null))), "SOURCE_PHYSICS_RHS_SCREEN_RECOMPUTED");
            assert_eq!(refused(&output, &set("/cancellation_screen", Some(json!("over")))), "SOURCE_PHYSICS_RHS_SCREEN");
            // Finding 2: the reviewer's two mutations and their neighbours.
            let negative = groups.iter().copied().find(|&g| rhs["groups"][g]["assembled_force_n"].as_f64().unwrap() < 0.0).unwrap();
            let same_sign = |rhs: &mut Value| {
                let group = &mut rhs["groups"][negative];
                group["assembled_force_n"] = json!(-group["assembled_force_n"].as_f64().unwrap());
                group["coefficient_sum"] = json!(-group["coefficient_sum"].as_f64().unwrap());
                for term in group["terms"].as_array_mut().unwrap() {
                    term["coefficient"] = json!(-term["coefficient"].as_f64().unwrap());
                }
            };
            assert_eq!(refused(&output, &same_sign), "SOURCE_PHYSICS_RHS_ASSEMBLED_SUM", "same-sign groups");
            let zero = |rhs: &mut Value| {
                let n = rhs["assembled_pressure_rhs_global"].as_array().unwrap().len();
                rhs["assembled_pressure_rhs_global"] = json!(vec![0.0; n]);
            };
            assert_eq!(refused(&output, &zero), "SOURCE_PHYSICS_RHS_ASSEMBLED_SUM", "zeroed vector");
            let one_group = |rhs: &mut Value| {
                let value = rhs["groups"][groups[0]]["assembled_force_n"].as_f64().unwrap();
                rhs["groups"][groups[0]]["assembled_force_n"] = json!(next_up(value, 1));
            };
            assert_eq!(refused(&output, &one_group), "SOURCE_PHYSICS_RHS_GROUP_VALUE", "one group's force");
            let node = rhs["node_order"].as_array().unwrap().iter().position(|n| *n == record["node_ref"]).unwrap();
            let vector_only = |rhs: &mut Value| {
                let value = rhs["assembled_pressure_rhs_global"][node * 6].as_f64().unwrap();
                rhs["assembled_pressure_rhs_global"][node * 6] = json!(if value == 0.0 { 1e-300 } else { next_up(value, 1) });
            };
            assert_eq!(refused(&output, &vector_only), "SOURCE_PHYSICS_RHS_ASSEMBLED_SUM", "vector alone");
            let flip = |rhs: &mut Value| {
                let term = &mut rhs["groups"][groups[0]]["terms"][0]["coefficient"];
                *term = json!(-term.as_f64().unwrap());
            };
            assert_eq!(refused(&output, &flip), "SOURCE_PHYSICS_RHS_GROUP_COEFFICIENT", "a coefficient sign flipped");
            // The same flip carried through the group's sum, its value and the
            // vector: only the binding to the member's direction and the
            // opposition at the node can see it.
            let consistent = |rhs: &mut Value| {
                let group = &mut rhs["groups"][groups[0]];
                for key in ["coefficient_sum", "assembled_force_n"] {
                    group[key] = json!(-group[key].as_f64().unwrap());
                }
                let term = &mut group["terms"][0]["coefficient"];
                *term = json!(-term.as_f64().unwrap());
                let (target, component) = (group["node_ref"].clone(), group["component"].clone());
                let axis = ["Fx", "Fy", "Fz"].iter().position(|c| component == *c).unwrap();
                let at = rhs["groups"].as_array().unwrap().iter().filter(|g| g["node_ref"] == target && g["component"] == component)
                    .map(|g| g["assembled_force_n"].as_f64().unwrap()).collect::<Vec<_>>();
                let sum = if let [a, b] = at[..] { a + b } else { at[0] };
                rhs["assembled_pressure_rhs_global"][node * 6 + axis] = json!(sum);
            };
            let code = refused(&output, &consistent);
            assert!(["SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_DIRECTION", "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_OPPOSITION"].contains(&code.as_str()), "{index} {mode:?}: {code}");
            // A member's published direction negated: from the envelope alone
            // that is the member reversed, with each end sign reversed too;
            // given the model, the end signs are bound to its from and to.
            let mut reversed = output.clone();
            let loads = reversed.pointer_mut("/contract_evidence/pressure/0/applied_loads").unwrap().as_array_mut().unwrap();
            loads[1]["local_x_global"] = json!(loads[1]["local_x_global"].as_array().unwrap().iter().map(|x| -x.as_f64().unwrap()).collect::<Vec<_>>());
            validate_pressure_evidence(&reversed).unwrap();
            assert_eq!(verdict(&reversed)["error"]["code"], "SOURCE_PHYSICS_MODEL_EXCEPTION_DIRECTION", "{index} {mode:?}");
        }
    }
    // A passing screen: no record may stand beside it, and a forged failing
    // screen with a record over a case outside the family is refused.
    for mode in MODES {
        let output = serde_json::to_value(solve(chain(&x, &[0.12, 0.12], &[0, 2]), mode)).unwrap();
        validate_pressure_evidence(&output).unwrap();
        let record = json!({"basis": EXCEPTION_BASIS, "node_ref": "node:C1", "member_pipe_ids": ["pipe:C0", "pipe:C1"], "member_lengths_m": [3.0, 3.0],
            "region_span_m": LENGTH, "thrust_to_bore_force_ratio": 0.0, "consistency_bound_ratio": 1e-13});
        assert_eq!(refused(&output, &set("/cancellation_screen_exception", Some(record.clone()))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BESIDE_PASSING_SCREEN");
        let mut open = chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]);
        open["model"]["load_cases"][0]["pressure_regions"][0]["terminals"][1]["closure_transfer"] = json!("separately_supported_or_compensated");
        let mut forged = serde_json::to_value(solve(open, mode)).unwrap();
        assert_screen(&forged, false, "a terminal that does not transfer");
        forged.pointer_mut(RHS).unwrap()["cancellation_screen"] = json!(1.0);
        assert_eq!(refused(&forged, &set("/cancellation_screen_exception", Some(record))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY");
    }
}

const WARNING: &str = "PRESSURE_REGION_DEPARTURE_THRUST_RETAINED";
const WARNING_MESSAGE: &str = "Small differences in bore or alignment between adjacent members of this pressure region, within the region's representation limits, produce pressure thrust at interior nodes in this model. The solver retains that thrust. The exception to the pressure cancellation screen that admitted this case does not establish that the thrust's effect is negligible; the case's numerical standing concerns the recorded geometry, not an idealised region of equal bores and exact collinearity.";

fn warnings(output: &Value) -> Vec<Value> {
    output["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == WARNING).cloned().collect()
}

fn exact_case<'a>(output: &'a Value, case: &str) -> &'a Value {
    output["contract_evidence"]["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == case).unwrap()
}

/// DEL-04-07: a case admitted under the cancellation screen's exception
/// carries the named warning once, at warning severity, with the fixed
/// message and the references [case, region, the record's node]; a case the
/// original screen admits carries none. The readers refuse a record without
/// its warning, a warning without a record, a changed message and a wrong node.
#[test]
fn a_case_admitted_under_the_exception_carries_its_warning_and_the_readers_bind_it() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let refused = |document: &Value| validate_pressure_evidence(document).unwrap_err();
    let without = |document: &Value| {
        let mut document = document.clone();
        document["diagnostics"].as_array_mut().unwrap().retain(|d| d["code"] != WARNING);
        document
    };
    for mode in MODES {
        let admitted = serde_json::to_value(solve(chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]), mode)).unwrap();
        let record = assert_screen(&admitted, true, "admitted");
        let found = warnings(&admitted);
        assert_eq!(found.len(), 1, "{mode:?}");
        assert_eq!(found[0]["severity"], "warning");
        assert_eq!(found[0]["message"], WARNING_MESSAGE);
        assert_eq!(found[0]["affected_refs"], json!([CASE, REGION, record["node_ref"]]));
        validate_pressure_evidence(&admitted).unwrap();
        let original = serde_json::to_value(solve(chain(&x, &[0.12, 0.12], &[0, 2]), mode)).unwrap();
        assert_screen(&original, false, "original screen");
        assert!(warnings(&original).is_empty(), "{mode:?}");
        // A record without its warning.
        assert_eq!(refused(&without(&admitted)), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING_MISSING");
        // A warning without a record.
        let mut unrecorded = original.clone();
        unrecorded["diagnostics"].as_array_mut().unwrap().push(found[0].clone());
        assert_eq!(refused(&unrecorded), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING_UNRECORDED");
        // A changed message.
        let mut changed = without(&admitted);
        let mut warning = found[0].clone();
        warning["message"] = json!(WARNING_MESSAGE.replace("not establish", "establish"));
        changed["diagnostics"].as_array_mut().unwrap().push(warning);
        assert_eq!(refused(&changed), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING");
        // A wrong node.
        let mut moved = without(&admitted);
        let mut warning = found[0].clone();
        warning["affected_refs"][2] = assembly(&admitted)["node_order"][0].clone();
        moved["diagnostics"].as_array_mut().unwrap().push(warning);
        assert_eq!(refused(&moved), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING");
    }
}

/// DEL-04-07, the warning's words against two models. One case with two
/// pressure regions, a straight run whose interior node falls in the family
/// and the arc corpus's L line (not straight-only): the eligibility rule
/// admits no case with a realized arc, so the case is refused by the original
/// screen and no warning is published. Two cases, one admitted and one passing
/// the original screen: the warning stands on the admitted case only, and its
/// references name that case's region and node.
#[test]
fn the_warning_names_the_admitted_case_s_region_and_node_only() {
    let corpus: Value = serde_json::from_str(&std::fs::read_to_string(
        current_model::project_root().join("fixtures/results/pressure_v3_arc_reader_corpus.json")).unwrap()).unwrap();
    let case = corpus["cases"].as_array().unwrap().iter().find(|c| c["label"] == "U2-L-ANCH-PTW-K2 0.3.0 sparse_interactive").unwrap();
    let mut document = pressure_elongation::without_elongation(&case["document"]);
    let model = &mut document["model"];
    let template = model["pipe_segments"][0].clone();
    let od = template["section"]["outside_diameter"]["value"].as_f64().unwrap();
    for (k, x) in [0.0, 3.0, LENGTH].iter().enumerate() {
        model["nodes"].as_array_mut().unwrap().push(node(&format!("node:P{k}"), [*x, 10.0, 0.0]));
    }
    for (k, outside) in [od, next_up(od, 1)].iter().enumerate() {
        let mut member = template.clone();
        member["id"] = json!(format!("pipe:P{k}"));
        member["from"] = json!(format!("node:P{k}"));
        member["to"] = json!(format!("node:P{}", k + 1));
        member["section"]["outside_diameter"]["value"] = json!(outside);
        model["pipe_segments"].as_array_mut().unwrap().push(member);
    }
    model["supports"].as_array_mut().unwrap().extend([anchor("support:P0", "node:P0"), anchor("support:P2", "node:P2")]);
    let mut straight = model["load_cases"][0]["pressure_regions"][0].clone();
    straight["id"] = json!("region:straight");
    straight["member_pipe_ids"] = json!(["pipe:P0", "pipe:P1"]);
    straight["terminals"] = json!([terminal("node:P0", true), terminal("node:P2", true)]);
    model["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().push(straight);
    for mode in MODES {
        let output = serde_json::to_value(solve(document.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        let blocking = output["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").map(|d| d["code"].clone()).collect::<Vec<_>>();
        assert_eq!(blocking, [json!("PRESSURE_ASSEMBLY_CANCELLATION_UNRESOLVED")], "{mode:?}");
        assert!(warnings(&output).is_empty(), "{mode:?}");
    }
    // Two cases over two disjoint runs: the first admitted, the second's bit-equal members pass the original screen.
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let mut input = chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]);
    let model = &mut input["model"];
    for (k, x) in [0.0, 3.0, LENGTH].iter().enumerate() {
        model["nodes"].as_array_mut().unwrap().push(node(&format!("node:D{k}"), [*x, 5.0, 0.0]));
    }
    for k in 0..2 {
        model["pipe_segments"].as_array_mut().unwrap().push(pipe(&format!("pipe:D{k}"), &format!("node:D{k}"), &format!("node:D{}", k + 1), 0.12, 0.01, "material:one"));
    }
    model["supports"].as_array_mut().unwrap().extend([anchor("support:D0", "node:D0"), anchor("support:D2", "node:D2")]);
    let mut second = model["load_cases"][0].clone();
    second["id"] = json!("case:second");
    second["pressure_regions"][0]["id"] = json!("region:second");
    second["pressure_regions"][0]["member_pipe_ids"] = json!(["pipe:D0", "pipe:D1"]);
    second["pressure_regions"][0]["terminals"] = json!([terminal("node:D0", true), terminal("node:D2", true)]);
    model["load_cases"].as_array_mut().unwrap().push(second);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", output["diagnostics"]);
        let record = exact_case(&output, CASE)["pressure_rhs_assembly"]["cancellation_screen_exception"].clone();
        assert_eq!(record["node_ref"], "node:C1", "{mode:?}");
        assert!(exact_case(&output, "case:second")["pressure_rhs_assembly"].get("cancellation_screen_exception").is_none());
        let found = warnings(&output);
        assert_eq!(found.len(), 1, "{mode:?}");
        assert_eq!(found[0]["affected_refs"], json!([CASE, REGION, "node:C1"]), "{mode:?}");
        open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence(&output).unwrap();
        let verdict = open_pipe_stress_result_export::reader_verdict::validate("pressure", &output, Some(&input["model"]), None);
        assert_eq!(verdict["ok"], true, "{mode:?}: {verdict}");
    }
}
