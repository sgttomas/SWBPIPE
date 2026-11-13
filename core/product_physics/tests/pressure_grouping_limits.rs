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

const EXCEPTION_BASIS: &str = "straight_region_interior_cap_pairs_cap_remainder_retained";

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
        let ratio = record["remainder_to_bore_force_ratio"].as_f64().unwrap();
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
/// load at an interior node is the remainder of the two members' cap loads,
/// from the bore and alignment differences the region guards admit, from the
/// rounding of each member's direction and bore area, or both, and the
/// record's bound only checks that it is consistent with what they admit. The response is observed, not bounded by
/// it: for these small models the reactions and the effective force S are
/// within 1e-9*p*Ai of the closed form 0 of an idealised region in which the
/// cap loads cancel, and the wall force N_w within 1e-9*p*Ai
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
    // interior remainder too uncertain relative to the response it drives.
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
    // the remainder at the interior node is about 1e-8 of the caps that form it.
    // The former constant 1e-9 refused it; it had no derivation from the
    // guards, and the bound derived from them admits what they admit (the
    // remainder is 0.33 of it here). Published Sensitive: the solve's condition
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
    let ratio = record["remainder_to_bore_force_ratio"].as_f64().unwrap();
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
        let used = record["remainder_to_bore_force_ratio"].as_f64().unwrap() / record["consistency_bound_ratio"].as_f64().unwrap();
        assert!(used > 0.5 && used < 1.0, "p={p}: used {used}");
        assert_eq!(standing(&output).0, "sensitive", "p={p}");
        match &first {
            None => first = Some(record),
            Some(base) => {
                assert_eq!(record["consistency_bound_ratio"], base["consistency_bound_ratio"], "p={p}");
                let (a, b) = (record["remainder_to_bore_force_ratio"].as_f64().unwrap(), base["remainder_to_bore_force_ratio"].as_f64().unwrap());
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
            let ratio = record["remainder_to_bore_force_ratio"].as_f64().unwrap();
            let bound = record["consistency_bound_ratio"].as_f64().unwrap();
            let screen = output.pointer(&format!("{RHS}/cancellation_screen")).unwrap().as_f64().unwrap();
            let rhs = output.pointer(RHS).unwrap();
            let groups = interior(rhs);
            // The record and the original screen.
            assert_eq!(refused(&output, &set("/cancellation_screen_exception", None)), "SOURCE_PHYSICS_RHS_SCREEN");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/basis", Some(json!("straight_region_interior_cap_pairs_without_pressure_elongation")))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/extra", Some(json!(0)))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/remainder_to_bore_force_ratio", Some(json!(next_up(ratio, 1))))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_RATIO");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/consistency_bound_ratio", Some(json!(next_up(bound, 1))))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BOUND");
            // The record's force: |F| at its node, bit for bit, and present.
            let force = record["remainder_force_n"].as_f64().unwrap();
            assert!(force > 0.0);
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/remainder_force_n", Some(json!(next_up(force, 1))))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FORCE");
            assert_eq!(refused(&output, &set("/cancellation_screen_exception/remainder_force_n", None)), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION");
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
            "region_span_m": LENGTH, "remainder_to_bore_force_ratio": 0.0, "remainder_force_n": 0.0, "consistency_bound_ratio": 1e-13});
        assert_eq!(refused(&output, &set("/cancellation_screen_exception", Some(record.clone()))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BESIDE_PASSING_SCREEN");
        let mut open = chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]);
        open["model"]["load_cases"][0]["pressure_regions"][0]["terminals"][1]["closure_transfer"] = json!("separately_supported_or_compensated");
        let mut forged = serde_json::to_value(solve(open, mode)).unwrap();
        assert_screen(&forged, false, "a terminal that does not transfer");
        forged.pointer_mut(RHS).unwrap()["cancellation_screen"] = json!(1.0);
        assert_eq!(refused(&forged, &set("/cancellation_screen_exception", Some(record))), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY");
    }
}

const WARNING: &str = "PRESSURE_REGION_CAP_REMAINDER_RETAINED";
/// The warning's fixed opening; the whole message ends with one sentence
/// formed from the case's exception record ([`warning_message`]).
const WARNING_MESSAGE: &str = "At interior nodes of this case's pressure regions the cap loads of adjacent members are formed separately and need not cancel exactly in this model. The remainder comes from small differences in bore or alignment within each region's representation limits, from the rounding of each member's direction and bore area, or both. The solver retains it as a pressure load. The exception to the pressure cancellation screen that admitted this case does not establish that its effect is negligible; the case's numerical standing concerns the recorded geometry as computed, not an idealised region in which those loads cancel.";

/// The whole message for the case `case` of a solved output, written out
/// independently of the producer's and the readers' functions: the opening,
/// then the case, the record's force by `{:e}` in N, the record's node and the
/// not-a-bound clause.
fn warning_message(output: &Value, case: &str) -> String {
    let record = &exact_case(output, case)["pressure_rhs_assembly"]["cancellation_screen_exception"];
    let force = record["remainder_force_n"].as_f64().unwrap();
    format!("{WARNING_MESSAGE} In case {case}, the largest retained remainder relative to the pressure force on the bore is {force:e} N, at node {}; it is not a bound on the resulting displacement.", record["node_ref"].as_str().unwrap())
}

fn warnings(output: &Value) -> Vec<Value> {
    output["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == WARNING).cloned().collect()
}

fn exact_case<'a>(output: &'a Value, case: &str) -> &'a Value {
    output["contract_evidence"]["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == case).unwrap()
}

/// DEL-04-07: a case admitted under the cancellation screen's exception
/// carries the named warning once, at warning severity, with the message
/// formed from its record and the references [case, region, the record's node]; a case the
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
        assert_eq!(found[0]["message"], warning_message(&admitted, CASE));
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
        warning["message"] = json!(warning_message(&admitted, CASE).replace("not establish", "establish"));
        changed["diagnostics"].as_array_mut().unwrap().push(warning);
        assert_eq!(refused(&changed), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING");
        // The fixed opening alone; another force in the sentence; the not-a-bound clause dropped.
        let message = warning_message(&admitted, CASE);
        let force = record["remainder_force_n"].as_f64().unwrap();
        for text in [WARNING_MESSAGE.to_owned(), message.replace(&format!("{force:e} N"), &format!("{:e} N", next_up(force, 1))),
            message.replace("; it is not a bound on the resulting displacement.", ".")] {
            let mut changed = without(&admitted);
            let mut warning = found[0].clone();
            warning["message"] = json!(text);
            changed["diagnostics"].as_array_mut().unwrap().push(warning);
            assert_eq!(refused(&changed), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING");
        }
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

// DEL-04-07, the widened family (one member's term alone on an axis where the
// other member's formed direction component is exactly zero), its witnesses,
// and the readers' two kinds of check: consistency of the published evidence
// with itself (no model) and binding to the model's geometry (the verdict
// given the model).

const EPS: f64 = f64::EPSILON;

/// The second review's chain: A-M along x (its y component a source-known
/// zero), M-N off the chord by 63ε of the span in y, N-B back to the chord.
fn reviewer_chain() -> Value {
    let up = next_up(0.12, 1);
    chain(&[[0.0; 3], [2.0, 0.0, 0.0], [3.0, 63.0 * EPS * LENGTH, 0.0], [LENGTH, 0.0, 0.0]], &[0.12, up, up], &[0, 3])
}

/// A run along y (the members' y reference is z) whose second interior node
/// is off the axis in x only, by 40ε of the span; outside diameters one ulp
/// apart member to member (with bit-equal bores the first node's two y terms
/// share one group, cancel exactly and the original screen passes).
fn axis_run_offset_in_x() -> Value {
    chain(&[[0.0; 3], [0.0, 2.0, 0.0], [40.0 * EPS * LENGTH, 4.0, 0.0], [0.0, LENGTH, 0.0]], &[0.12, next_up(0.12, 1), next_up(0.12, 2)], &[0, 3])
}

/// A run in the x-z plane along (3, 0, 4)/5, its interior node off the chord
/// in that plane by 40ε of the span: every y component is a source-known
/// zero, so the interior node's y axis holds no term of either member.
fn plane_run() -> Value {
    let (u, n) = ([0.6, 0.0, 0.8], [-0.8, 0.0, 0.6]);
    let at = |t: f64, off: f64| [LENGTH * t * u[0] + off * n[0], 0.0, LENGTH * t * u[2] + off * n[2]];
    chain(&[[0.0; 3], at(0.5, 40.0 * EPS * LENGTH), at(1.0, 0.0)], &[0.12, next_up(0.12, 2)], &[0, 2])
}

/// The number of (interior node, axis) pairs of a solved case whose published
/// groups hold exactly one term.
fn lone_terms(output: &Value) -> usize {
    let rhs = assembly(output);
    let terminals = output["contract_evidence"]["pressure"].as_array().unwrap().iter()
        .flat_map(|r| r["terminals"].as_array().unwrap().iter().map(|t| t["node_ref"].clone())).collect::<Vec<_>>();
    let mut counts: std::collections::BTreeMap<(String, String), usize> = Default::default();
    for group in rhs["groups"].as_array().unwrap() {
        if terminals.contains(&group["node_ref"]) {
            continue;
        }
        *counts.entry((group["node_ref"].to_string(), group["component"].to_string())).or_default() += group["terms"].as_array().unwrap().len();
    }
    counts.values().filter(|&&n| n == 1).count()
}

fn verdict(output: &Value, model: &Value) -> Value {
    open_pipe_stress_result_export::reader_verdict::validate("pressure", output, Some(model), None)
}

#[test]
fn the_widened_family_admits_its_zero_component_witnesses_in_both_solver_modes() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    for (label, input, lone) in [("reviewer's chain", reviewer_chain(), true), ("axis run offset in x", axis_run_offset_in_x(), true), ("x-z plane run", plane_run(), false)] {
        for mode in MODES {
            let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
            assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{label} {mode:?}: {}", output["diagnostics"]);
            assert_screen(&output, true, label);
            assert_eq!(warnings(&output).len(), 1, "{label} {mode:?}");
            assert_eq!(lone_terms(&output) > 0, lone, "{label} {mode:?}");
            validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{label} {mode:?}: {e}"));
            assert_eq!(verdict(&output, &input["model"])["ok"], true, "{label} {mode:?}: {}", verdict(&output, &input["model"]));
        }
    }
}

/// Not admitted under the exception: either refused, or judged by the
/// original screen alone (no record and no warning).
fn assert_not_excepted(input: &Value, label: &str) -> Vec<String> {
    let mut outcomes = Vec::new();
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        let excepted = output["contract_evidence"]["exact_cases"].as_array().is_some_and(|cases| cases.iter().any(|c| c["pressure_rhs_assembly"].get("cancellation_screen_exception").is_some()));
        assert!(!excepted, "{label} {mode:?}: admitted under the exception");
        assert!(warnings(&output).is_empty(), "{label} {mode:?}");
        let blocking = output["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").map(|d| d["code"].as_str().unwrap().to_owned()).collect::<Vec<_>>();
        outcomes.push(format!("{} {}", output["status"]["mechanics"].as_str().unwrap(), blocking.join(",")));
    }
    outcomes
}

/// The reviewer's chain with region R1 ending at J = the chain's last node,
/// and further regions at J: `second` the outside diameter of region R2 over
/// J-K along x, `tee` adding region R3 over J-T normal to the run.
fn junction_on_the_reviewer_chain(second: f64, tee: bool) -> Value {
    let mut input = reviewer_chain();
    let document = &mut input["model"];
    document["nodes"].as_array_mut().unwrap().push(node("node:K", [9.0, 0.0, 0.0]));
    document["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:J-K", "node:C3", "node:K", second, 0.01, "material:one"));
    document["supports"] = json!([anchor("support:C0", "node:C0"), anchor("support:K", "node:K")]);
    let region = |id: &str, members: &[&str], from: &str, to: &str| json!({"id":id,"member_pipe_ids":members,
        "pressure_basis":"internal_differential_zero_external_v1","pressure":{"value":2e6,"unit":"Pa"},
        "terminals":[terminal(from,true),terminal(to,true)],"provenance":"independent_pressure_region"});
    let regions = document["load_cases"][0]["pressure_regions"].as_array_mut().unwrap();
    regions.push(region("region:R2", &["pipe:J-K"], "node:C3", "node:K"));
    if tee {
        document["nodes"].as_array_mut().unwrap().push(node("node:T", [LENGTH, 3.0, 0.0]));
        document["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:J-T", "node:C3", "node:T", 0.12, 0.01, "material:one"));
        document["supports"].as_array_mut().unwrap().push(anchor("support:T", "node:T"));
        document["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().push(region("region:R3", &["pipe:J-T"], "node:C3", "node:T"));
    }
    input
}

#[test]
fn no_foreign_pressure_structure_passes_through_the_widened_family() {
    // An uncancelled cap at a terminal that does not transfer.
    let mut open = reviewer_chain();
    open["model"]["load_cases"][0]["pressure_regions"][0]["terminals"][1]["closure_transfer"] = json!("separately_supported_or_compensated");
    // Pressure elongation included for the case.
    let elongated = pressure_elongation::with_elongation(&reviewer_chain());
    // A second region crossing the chain's first interior node along y, that node interior to both.
    let mut crossing = reviewer_chain();
    {
        let document = &mut crossing["model"];
        document["nodes"].as_array_mut().unwrap().extend([node("node:Z0", [2.0, -3.0, 0.0]), node("node:Z1", [2.0, 3.0, 0.0])]);
        document["pipe_segments"].as_array_mut().unwrap().extend([
            pipe("pipe:Z0", "node:Z0", "node:C1", 0.12, 0.01, "material:one"), pipe("pipe:Z1", "node:C1", "node:Z1", next_up(0.12, 1), 0.01, "material:one")]);
        document["supports"].as_array_mut().unwrap().extend([anchor("support:Z0", "node:Z0"), anchor("support:Z1", "node:Z1")]);
        let mut second = document["load_cases"][0]["pressure_regions"][0].clone();
        second["id"] = json!("region:Z");
        second["member_pipe_ids"] = json!(["pipe:Z0", "pipe:Z1"]);
        second["terminals"] = json!([terminal("node:Z0", true), terminal("node:Z1", true)]);
        document["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().push(second);
    }
    for (label, input) in [
        ("uncancelled cap", open),
        ("elongation included", elongated),
        ("second region's term at J", junction_on_the_reviewer_chain(next_up(0.12, 1), false)),
        ("reducer step at J", junction_on_the_reviewer_chain(0.14, false)),
        ("tee junction closure at J", junction_on_the_reviewer_chain(0.12, true)),
        ("a second region crossing an interior node", crossing),
    ] {
        let outcomes = assert_not_excepted(&input, label);
        eprintln!("foreign structure {label}: {outcomes:?}");
    }
}

/// The admitted output's published regions, mutable.
fn loads_of(output: &mut Value) -> Vec<&mut Value> {
    output["contract_evidence"]["pressure"].as_array_mut().unwrap().iter_mut()
        .flat_map(|r| r["applied_loads"].as_array_mut().unwrap().iter_mut()).collect()
}

/// Every published direction scaled by `factor`, and, with `loads`, every
/// group's magnitude and value, the assembled vector and the record's
/// measured ratio scaled with it (the second review's forgery for 1/2).
fn scaled(output: &Value, factor: f64, loads: bool) -> Value {
    let mut forged = output.clone();
    for load in loads_of(&mut forged) {
        load["local_x_global"] = json!(load["local_x_global"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap() * factor).collect::<Vec<_>>());
    }
    if loads {
        let rhs = forged.pointer_mut("/contract_evidence/exact_cases/0/pressure_rhs_assembly").unwrap();
        for group in rhs["groups"].as_array_mut().unwrap() {
            for key in ["direction_component_magnitude", "assembled_force_n"] {
                group[key] = json!(group[key].as_f64().unwrap() * factor);
            }
        }
        rhs["assembled_pressure_rhs_global"] = json!(rhs["assembled_pressure_rhs_global"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap() * factor).collect::<Vec<_>>());
        let ratio = rhs["cancellation_screen_exception"]["remainder_to_bore_force_ratio"].as_f64().unwrap();
        rhs["cancellation_screen_exception"]["remainder_to_bore_force_ratio"] = json!(ratio * factor);
        let force = rhs["cancellation_screen_exception"]["remainder_force_n"].as_f64().unwrap();
        rhs["cancellation_screen_exception"]["remainder_force_n"] = json!(force * factor.abs());
    }
    forged
}

/// Every published direction negated, and, with `loads`, every coefficient,
/// coefficient sum, group value and the assembled vector negated with it
/// (the second review's reversal).
fn reversed(output: &Value, loads: bool) -> Value {
    let mut forged = scaled(output, -1.0, false);
    if loads {
        let rhs = forged.pointer_mut("/contract_evidence/exact_cases/0/pressure_rhs_assembly").unwrap();
        for group in rhs["groups"].as_array_mut().unwrap() {
            for key in ["coefficient_sum", "assembled_force_n"] {
                group[key] = json!(-group[key].as_f64().unwrap());
            }
            for term in group["terms"].as_array_mut().unwrap() {
                term["coefficient"] = json!(-term["coefficient"].as_f64().unwrap());
            }
        }
        rhs["assembled_pressure_rhs_global"] = json!(rhs["assembled_pressure_rhs_global"].as_array().unwrap().iter().map(|x| -x.as_f64().unwrap()).collect::<Vec<_>>());
    }
    forged
}

#[test]
fn forged_directions_are_refused_by_consistency_where_it_can_see_them_and_by_the_model_binding_otherwise() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let consistency = |document: &Value| validate_pressure_evidence(document).err();
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    for (label, input) in [("one ulp", chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2])), ("reviewer's chain", reviewer_chain()),
        ("skew", chain(&skew(1e-3, 50.0 * EPS * 3.0), &[0.12, 0.12], &[0, 2]))] {
        let model = &input["model"];
        for mode in MODES {
            let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
            assert_screen(&output, true, label);
            let both = |document: &Value| (consistency(document), verdict(document, model)["error"]["code"].as_str().map(str::to_owned));
            let not_unit = Some("SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_DIRECTION_NOT_UNIT".to_owned());
            // Scaled, with the loads carried through and without them. A
            // magnitude carried above 1 is refused first by the group check
            // that a straight member's component is at most 1.
            for factor in [0.5, 2.0, 1.0 + 16.0 * EPS, 1.0 - 16.0 * EPS, 0.999] {
                for loads in [true, false] {
                    let (plain, bound) = both(&scaled(&output, factor, loads));
                    // An inexactly scaled value no longer sums to the vector and is refused there first.
                    let expected = [not_unit.as_deref(), Some("SOURCE_PHYSICS_RHS_DIRECTION"), Some("SOURCE_PHYSICS_RHS_ASSEMBLED_SUM"), Some("SOURCE_PHYSICS_RHS_GROUP_VALUE")];
                    assert!(expected.contains(&plain.as_deref()), "{label} {mode:?} x{factor} loads={loads}: {plain:?}");
                    if factor == 0.5 || !loads && factor < 1.0 { assert_eq!(plain, not_unit, "{label} {mode:?} x{factor} loads={loads}"); }
                    assert_eq!(bound, plain, "{label} {mode:?} x{factor} loads={loads}");
                }
            }
            // One member only, halved.
            let mut one = output.clone();
            let load = &mut loads_of(&mut one)[1];
            load["local_x_global"] = json!(load["local_x_global"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap() * 0.5).collect::<Vec<_>>());
            assert_eq!(both(&one), (not_unit.clone(), not_unit.clone()), "{label} {mode:?} one member");
            // One component altered: far from unit, and by one ulp (unit, but not the group's magnitude).
            let mut far = output.clone();
            loads_of(&mut far)[0]["local_x_global"][2] = json!(0.25);
            assert_eq!(both(&far).0, not_unit, "{label} {mode:?} a component altered");
            let mut ulp = output.clone();
            let component = loads_of(&mut ulp)[0]["local_x_global"][0].as_f64().unwrap();
            loads_of(&mut ulp)[0]["local_x_global"][0] = json!(next_up(component, 1));
            assert_eq!(both(&ulp).0.as_deref(), Some("SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_DIRECTION"), "{label} {mode:?} one ulp");
            // Reversed, with and without the loads: self-consistent, so only the model binding sees it.
            for loads in [true, false] {
                let (plain, bound) = both(&reversed(&output, loads));
                assert_eq!(plain, None, "{label} {mode:?} reversed loads={loads}");
                assert_eq!(bound.as_deref(), Some("SOURCE_PHYSICS_MODEL_EXCEPTION_DIRECTION"), "{label} {mode:?} reversed loads={loads}");
            }
        }
    }
}

#[test]
fn a_published_zero_component_the_model_coordinates_contradict_is_refused_by_the_model_binding() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let input = reviewer_chain();
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_screen(&output, true, "reviewer's chain");
        assert!(lone_terms(&output) > 0);
        // The document as published (A-M's y component zero and its y term
        // absent at M), against a model in which M is off the x axis in y.
        let mut moved = input["model"].clone();
        moved["nodes"][1]["position"]["y"] = json!(1e-15);
        validate_pressure_evidence(&output).unwrap();
        assert_eq!(verdict(&output, &moved)["error"]["code"], "SOURCE_PHYSICS_MODEL_EXCEPTION_ZERO_COMPONENT", "{mode:?}");
        // An omitted nonzero term: M-N's y group at M removed, the vector there
        // made its sum; its published y component is not zero.
        let mut omitted = output.clone();
        let rhs = omitted.pointer_mut("/contract_evidence/exact_cases/0/pressure_rhs_assembly").unwrap();
        let order = rhs["node_order"].clone();
        let m = order.as_array().unwrap().iter().position(|n| n == "node:C1").unwrap();
        rhs["groups"].as_array_mut().unwrap().retain(|g| !(g["node_ref"] == "node:C1" && g["component"] == "Fy"));
        rhs["assembled_pressure_rhs_global"][m * 6 + 1] = json!(0.0);
        assert_eq!(validate_pressure_evidence(&omitted).unwrap_err(), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY", "{mode:?}");
        // A published zero where the member's own term is present: A-M's x
        // published as zero is not unit; M-N's y published as zero leaves its
        // term without a direction component.
        let mut zeroed = output.clone();
        let loads = loads_of(&mut zeroed);
        let load = loads.into_iter().find(|l| l["pipe_id"] == "pipe:C1").unwrap();
        load["local_x_global"][1] = json!(0.0);
        assert_eq!(validate_pressure_evidence(&zeroed).unwrap_err(), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_DIRECTION", "{mode:?}");
    }
}

#[test]
fn the_model_binding_refuses_a_model_that_contradicts_the_exception_s_evidence() {
    let input = reviewer_chain();
    let output = serde_json::to_value(solve(input.clone(), MODES[0])).unwrap();
    assert_screen(&output, true, "reviewer's chain");
    assert_eq!(verdict(&output, &input["model"])["ok"], true);
    let changed = |change: &dyn Fn(&mut Value)| {
        let mut model = input["model"].clone();
        change(&mut model);
        verdict(&output, &model)["error"]["code"].as_str().unwrap_or("ACCEPTED").to_owned()
    };
    fn region(model: &mut Value) -> &mut Value {
        model.pointer_mut("/load_cases/0/pressure_regions/0").unwrap()
    }
    let cases: Vec<(&str, Box<dyn Fn(&mut Value)>, &str)> = vec![
        ("pressure one ulp off", Box::new(|m| { region(m)["pressure"]["value"] = json!(next_up(2e6, 1)) }), "SOURCE_PHYSICS_MODEL_EXCEPTION_PRESSURE"),
        ("a terminal that does not transfer", Box::new(|m| { region(m)["terminals"][1]["closure_transfer"] = json!("separately_supported_or_compensated") }), "SOURCE_PHYSICS_MODEL_EXCEPTION_REGION"),
        ("terminals swapped", Box::new(|m| { let t = region(m)["terminals"].as_array_mut().unwrap(); t.swap(0, 1) }), "SOURCE_PHYSICS_MODEL_EXCEPTION_REGION"),
        ("a member dropped", Box::new(|m| { region(m)["member_pipe_ids"].as_array_mut().unwrap().pop(); }), "SOURCE_PHYSICS_MODEL_EXCEPTION_REGION"),
        ("another region in the case", Box::new(move |m| { let r = m["load_cases"][0]["pressure_regions"][0].clone(); m["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().push(r); }), "SOURCE_PHYSICS_MODEL_EXCEPTION_REGION"),
        ("a connector in the region", Box::new(|m| { region(m)["member_connector_ids"] = json!(["component:joint"]) }), "SOURCE_PHYSICS_MODEL_EXCEPTION_FAMILY"),
        ("elongation included", Box::new(|m| m["model_settings"] = json!({"pressure_elongation": {"operating_load_cases": [CASE]}})), "SOURCE_PHYSICS_MODEL_EXCEPTION_FAMILY"),
        ("a bend on a member", Box::new(|m| m["components"] = json!([{"id": "component:bend", "kind": "bend", "geometry": {"bend_pipe_ref": "pipe:C1"}}])), "SOURCE_PHYSICS_MODEL_EXCEPTION_FAMILY"),
        ("another node order", Box::new(|m| m["nodes"].as_array_mut().unwrap().swap(0, 3)), "SOURCE_PHYSICS_MODEL_EXCEPTION_GEOMETRY"),
        ("a member reversed", Box::new(|m| { let p = &mut m["pipe_segments"][1]; let (f, t) = (p["from"].clone(), p["to"].clone()); p["from"] = t; p["to"] = f; }), "SOURCE_PHYSICS_MODEL_EXCEPTION_DIRECTION"),
        ("an interior node moved along the run", Box::new(|m| m["nodes"][2]["position"]["x"] = json!(3.5)), "SOURCE_PHYSICS_MODEL_EXCEPTION_DIRECTION"),
        ("a terminal moved along the run", Box::new(|m| m["nodes"][3]["position"]["x"] = json!(7.0)), "SOURCE_PHYSICS_MODEL_EXCEPTION_DIRECTION"),
        ("a section one ulp off", Box::new(|m| m["pipe_segments"][0]["section"]["outside_diameter"]["value"] = json!(next_up(0.12, 1))), "SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY"),
    ];
    for (label, change, code) in cases {
        assert_eq!(changed(&*change), code, "{label}");
    }
}

/// The second review's collinear model: equal bores, members joining
/// (0,0,0), (1,1,1) and (4,4,4), exactly collinear in the recorded
/// coordinates (chords (1,1,1) and (3,3,3)). Admitted, with the warning, and
/// each sentence of the warning is true of it: the cap loads at the interior
/// node are formed separately and here do not cancel exactly; the remainder comes from the rounding of the
/// separately formed directions (the bores' bits are equal and the chords are
/// exactly proportional, so no bore or alignment difference is recorded); it
/// is retained as a pressure load; and the exception's record states no
/// effect, only a measured ratio within a consistency bound.
#[test]
fn exactly_collinear_equal_bore_members_can_leave_a_rounding_remainder_and_carry_the_warning() {
    let input = chain(&[[0.0; 3], [1.0, 1.0, 1.0], [4.0, 4.0, 4.0]], &[0.12, 0.12], &[0, 2]);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", output["diagnostics"]);
        let record = assert_screen(&output, true, "collinear");
        let found = warnings(&output);
        assert_eq!(found.len(), 1, "{mode:?}");
        assert_eq!(found[0]["message"], warning_message(&output, CASE));
        let rhs = assembly(&output);
        let m = rhs["node_order"].as_array().unwrap().iter().position(|n| *n == record["node_ref"]).unwrap();
        let load = (0..3).map(|k| rhs["assembled_pressure_rhs_global"][m * 6 + k].as_f64().unwrap()).collect::<Vec<_>>();
        // Sentence 1 and 3: a nonzero remainder, retained in the solved load vector.
        assert!(load.iter().any(|v| *v != 0.0), "{mode:?}: {load:?}");
        // Sentence 2: equal bore bits, exactly proportional chords, directions formed with different roundings.
        let groups = rhs["groups"].as_array().unwrap();
        let bores = groups.iter().map(|g| g["source_inner_radius_hi_bits"].as_str().unwrap()).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(bores.len(), 1, "{mode:?}");
        let directions = output["contract_evidence"]["pressure"][0]["applied_loads"].as_array().unwrap().iter().map(|l| l["local_x_global"].clone()).collect::<Vec<_>>();
        assert_ne!(directions[0], directions[1], "{mode:?}: the two formed directions differ by rounding");
        // Sentence 4: the record states a ratio within a consistency bound, nothing about the effect.
        assert!(record.as_object().unwrap().keys().all(|k| ["basis", "node_ref", "member_pipe_ids", "member_lengths_m", "region_span_m", "remainder_to_bore_force_ratio", "remainder_force_n", "consistency_bound_ratio"].contains(&k.as_str())));
        open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence(&output).unwrap();
        assert_eq!(verdict(&output, &input["model"])["ok"], true, "{mode:?}");
    }
}

/// A region whose authored member list is not in chain order: the record
/// names its two members in the order the region's published evidence lists
/// them (its traversal), the order the readers re-derive, and is read.
#[test]
fn a_region_listing_its_members_out_of_chain_order_is_read() {
    let mut input = reviewer_chain();
    input["model"]["load_cases"][0]["pressure_regions"][0]["member_pipe_ids"] = json!(["pipe:C2", "pipe:C1", "pipe:C0"]);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        let record = assert_screen(&output, true, "out of order");
        let published = output["contract_evidence"]["pressure"][0]["member_pipe_ids"].as_array().unwrap().clone();
        let place = |id: &Value| published.iter().position(|m| m == id).unwrap();
        assert!(place(&record["member_pipe_ids"][0]) < place(&record["member_pipe_ids"][1]), "{mode:?}");
        open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_eq!(verdict(&output, &input["model"])["ok"], true, "{mode:?}: {}", verdict(&output, &input["model"]));
    }
}

/// DEL-04-07: one case with two closed straight regions, both of the
/// exception's family. The case is admitted once and carries exactly one
/// warning, whose references name the region and node of the largest
/// remainder (the record's node). Each sentence of the message is true of both
/// regions: at an interior node of each, the two members' cap loads leave a
/// nonzero remainder (their bores differ within the equal-bore guard, which
/// each region passed, so the case was not refused), and the remainder is
/// retained as a pressure load: the published groups there carry it and sum
/// to the assembled load.
#[test]
fn a_case_with_two_straight_regions_of_the_family_carries_one_warning_true_of_both() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let mut input = chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]);
    let model = &mut input["model"];
    for (k, x) in [0.0, 3.0, LENGTH].iter().enumerate() {
        model["nodes"].as_array_mut().unwrap().push(node(&format!("node:D{k}"), [*x, 5.0, 0.0]));
    }
    // The second region's members five ulps apart: its remainder is the larger.
    for (k, outside) in [0.12, next_up(0.12, 5)].iter().enumerate() {
        model["pipe_segments"].as_array_mut().unwrap().push(pipe(&format!("pipe:D{k}"), &format!("node:D{k}"), &format!("node:D{}", k + 1), *outside, 0.01, "material:one"));
    }
    model["supports"].as_array_mut().unwrap().extend([anchor("support:D0", "node:D0"), anchor("support:D2", "node:D2")]);
    let mut second = model["load_cases"][0]["pressure_regions"][0].clone();
    second["id"] = json!("region:second");
    second["member_pipe_ids"] = json!(["pipe:D0", "pipe:D1"]);
    second["terminals"] = json!([terminal("node:D0", true), terminal("node:D2", true)]);
    model["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().push(second);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", output["diagnostics"]);
        let record = assert_screen(&output, true, "two regions");
        assert_eq!(record["node_ref"], "node:D1", "{mode:?}: the largest remainder");
        let found = warnings(&output);
        assert_eq!(found.len(), 1, "{mode:?}: one warning for the case");
        assert_eq!(found[0]["message"], warning_message(&output, CASE));
        assert_eq!(found[0]["affected_refs"], json!([CASE, "region:second", "node:D1"]), "{mode:?}");
        let rhs = assembly(&output);
        let order = rhs["node_order"].as_array().unwrap();
        for (region, interior) in [(REGION, "node:C1"), ("region:second", "node:D1")] {
            let at = order.iter().position(|n| n == interior).unwrap();
            let load = (0..3).map(|axis| rhs["assembled_pressure_rhs_global"][at * 6 + axis].as_f64().unwrap()).collect::<Vec<_>>();
            assert!(load.iter().any(|value| *value != 0.0), "{mode:?} {region}: a nonzero remainder at {interior}");
            for (axis, component) in ["Fx", "Fy", "Fz"].iter().enumerate() {
                let groups = rhs["groups"].as_array().unwrap().iter().filter(|g| g["node_ref"] == interior && g["component"] == *component).collect::<Vec<_>>();
                assert!(groups.iter().all(|g| g["terms"].as_array().unwrap().iter().all(|t| t["region_id"] == region)), "{mode:?} {region}");
                let sum = groups.iter().map(|g| g["assembled_force_n"].as_f64().unwrap()).sum::<f64>();
                if load[axis] != 0.0 {
                    assert!(!groups.is_empty() && (sum - load[axis]).abs() <= f64::EPSILON * sum.abs().max(load[axis].abs()), "{mode:?} {region} {component}: retained, {sum} against {}", load[axis]);
                }
            }
        }
        validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        let verdict = open_pipe_stress_result_export::reader_verdict::validate("pressure", &output, Some(&input["model"]), None);
        assert_eq!(verdict["ok"], true, "{mode:?}: {verdict}");
    }
}

// DEL-04-07, the third review's finding 2: an interior node can be omitted
// whole. The producer enumerates each region's interior nodes from its chain,
// the reader without the model from the published traversal, and the verdict
// with the model from the model's region membership and coordinates, so a
// node with no groups is examined by each.

/// The third review's witness: a 1000 m run along x, its second member a
/// millionth of the span and off the axis in y, outside diameters one ulp
/// apart at the middle member, anchored at its ends and its second interior node.
fn reviewer_witness() -> Value {
    chain(&[[0.0; 3], [500.0, 0.0, 0.0], [500.000001, 1.3988810110276972e-11, 0.0], [1000.0, 0.0, 0.0]], &[0.12, next_up(0.12, 1), 0.12], &[0, 2, 3])
}

/// The original screen as the producer forms it from a document's groups and
/// vector (at most two groups per degree of freedom here, so one rounded
/// addition is the exact sum correctly rounded); null where not finite.
fn restamp_screen(rhs: &mut Value) {
    let order = rhs["node_order"].as_array().unwrap().clone();
    let assembled = rhs["assembled_pressure_rhs_global"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect::<Vec<_>>();
    let max_rhs = assembled.iter().enumerate().filter(|(i, _)| i % 6 < 3).fold(0.0_f64, |m, (_, v)| m.max(v.abs()));
    let mut by_dof: std::collections::BTreeMap<usize, Vec<f64>> = Default::default();
    for group in rhs["groups"].as_array().unwrap() {
        let node = order.iter().position(|n| *n == group["node_ref"]).unwrap();
        let axis = ["Fx", "Fy", "Fz"].iter().position(|c| group["component"] == *c).unwrap();
        by_dof.entry(node * 6 + axis).or_default().push(group["assembled_force_n"].as_f64().unwrap());
    }
    let mut maximum = 0.0_f64;
    for values in by_dof.values() {
        assert!(values.len() <= 2);
        if max_rhs == 0.0 {
            if values.iter().any(|v| *v != 0.0) { maximum = f64::INFINITY; }
        } else {
            maximum = maximum.max(values.iter().map(|v| v.abs() / max_rhs).sum::<f64>());
        }
    }
    let screen = 32.0 * f64::EPSILON * maximum;
    rhs["cancellation_screen"] = if screen.is_finite() { json!(screen) } else { Value::Null };
}

/// The third review's forgery: every group at `node` removed, the vector
/// there zeroed and the original screen re-stamped.
fn omit_node(output: &Value, case: usize, node: &str) -> Value {
    let mut forged = output.clone();
    let rhs = forged.pointer_mut(&format!("/contract_evidence/exact_cases/{case}/pressure_rhs_assembly")).unwrap();
    rhs["groups"].as_array_mut().unwrap().retain(|g| g["node_ref"] != node);
    let at = rhs["node_order"].as_array().unwrap().iter().position(|n| n == node).unwrap() * 6;
    for axis in 0..3 {
        rhs["assembled_pressure_rhs_global"][at + axis] = json!(0.0);
    }
    restamp_screen(rhs);
    forged
}

#[test]
fn an_interior_node_omitted_whole_is_refused_by_name_by_both_reader_stages() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    const MISSING: &str = "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_INTERIOR_NODE_MISSING";
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let three = [[0.0; 3], [2.0, 0.0, 0.0], [4.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    // Omitting a run's only interior node leaves every published load zero, so
    // the re-stamped original screen passes and the record beside it is refused.
    const BESIDE: &str = "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BESIDE_PASSING_SCREEN";
    // (label, model, the node omitted, the refusal)
    let cases = [
        ("the third review's whole-node omission", reviewer_witness(), "node:C1", MISSING),
        ("the record's own node, the only interior node", chain(&x, &[0.12, next_up(0.12, 1)], &[0, 2]), "node:C1", BESIDE),
        ("one node omitted of several", chain(&three, &[0.12, next_up(0.12, 1), next_up(0.12, 3)], &[0, 3]), "node:C2", MISSING),
        ("the record's node, one of several", chain(&three, &[0.12, next_up(0.12, 1), next_up(0.12, 3)], &[0, 3]), "record", MISSING),
        // Its other interior node omitted instead, the loads left pass the original screen.
        ("one node of the second review's chain", reviewer_chain(), "node:C2", MISSING),
        ("the other node of the second review's chain", reviewer_chain(), "node:C1", BESIDE),
    ];
    for (label, input, node, code) in cases {
        for mode in MODES {
            let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
            let record = assert_screen(&output, true, label);
            let node = if node == "record" { record["node_ref"].as_str().unwrap().to_owned() } else { node.to_owned() };
            let node = node.as_str();
            validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{label} {mode:?}: {e}"));
            assert_eq!(verdict(&output, &input["model"])["ok"], true, "{label} {mode:?}: {}", verdict(&output, &input["model"]));
            let at = assembly(&output)["node_order"].as_array().unwrap().iter().position(|n| n == node).unwrap() * 6;
            let removed = (0..3).map(|k| assembly(&output)["assembled_pressure_rhs_global"][at + k].as_f64().unwrap()).collect::<Vec<_>>();
            eprintln!("{label} {mode:?}: the model requires {removed:?} N at {node}");
            let forged = omit_node(&output, 0, node);
            assert_eq!(validate_pressure_evidence(&forged).unwrap_err(), code, "{label} {mode:?}");
            assert_eq!(verdict(&forged, &input["model"])["error"]["code"], code, "{label} {mode:?}");
        }
    }
    // Both terms of one axis omitted at one node, the vector there made zero.
    for mode in MODES {
        let output = serde_json::to_value(solve(reviewer_chain(), mode)).unwrap();
        let mut forged = output.clone();
        let rhs = forged.pointer_mut("/contract_evidence/exact_cases/0/pressure_rhs_assembly").unwrap();
        rhs["groups"].as_array_mut().unwrap().retain(|g| !(g["node_ref"] == "node:C2" && g["component"] == "Fx"));
        let at = rhs["node_order"].as_array().unwrap().iter().position(|n| n == "node:C2").unwrap() * 6;
        rhs["assembled_pressure_rhs_global"][at] = json!(0.0);
        restamp_screen(rhs);
        assert_eq!(validate_pressure_evidence(&forged).unwrap_err(), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY", "{mode:?}");
    }
}

#[test]
fn a_whole_node_omitted_in_the_second_of_two_family_regions_is_refused_by_name() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let x = [[0.0; 3], [3.0, 0.0, 0.0], [LENGTH, 0.0, 0.0]];
    let mut input = chain(&x, &[0.12, next_up(0.12, 5)], &[0, 2]);
    let model = &mut input["model"];
    for (k, x) in [0.0, 3.0, LENGTH].iter().enumerate() {
        model["nodes"].as_array_mut().unwrap().push(node(&format!("node:D{k}"), [*x, 5.0, 0.0]));
    }
    for (k, outside) in [0.12, next_up(0.12, 1)].iter().enumerate() {
        model["pipe_segments"].as_array_mut().unwrap().push(pipe(&format!("pipe:D{k}"), &format!("node:D{k}"), &format!("node:D{}", k + 1), *outside, 0.01, "material:one"));
    }
    model["supports"].as_array_mut().unwrap().extend([anchor("support:D0", "node:D0"), anchor("support:D2", "node:D2")]);
    let mut second = model["load_cases"][0]["pressure_regions"][0].clone();
    second["id"] = json!("region:second");
    second["member_pipe_ids"] = json!(["pipe:D0", "pipe:D1"]);
    second["terminals"] = json!([terminal("node:D0", true), terminal("node:D2", true)]);
    model["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().push(second);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        let record = assert_screen(&output, true, "two regions");
        assert_eq!(record["node_ref"], "node:C1", "{mode:?}: the first region's remainder is the larger");
        validate_pressure_evidence(&output).unwrap();
        let forged = omit_node(&output, 0, "node:D1");
        assert_eq!(validate_pressure_evidence(&forged).unwrap_err(), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_INTERIOR_NODE_MISSING", "{mode:?}");
        assert_eq!(verdict(&forged, &input["model"])["error"]["code"], "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_INTERIOR_NODE_MISSING", "{mode:?}");
    }
}

/// Self-consistent evidence whose interior nodes are not the model's: the
/// published groups (and vector entries) of the second review's two interior
/// nodes exchanged, with the record's node and the warning's reference moved
/// with them. The published evidence agrees with itself; only the model's own
/// enumeration of its interior nodes sees it.
#[test]
fn interior_nodes_the_model_does_not_have_are_refused_by_the_model_binding() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let input = reviewer_chain();
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_screen(&output, true, "reviewer's chain");
        let mut forged = output.clone();
        let swap = |name: &Value| if name == "node:C1" { json!("node:C2") } else if name == "node:C2" { json!("node:C1") } else { name.clone() };
        let rhs = forged.pointer_mut("/contract_evidence/exact_cases/0/pressure_rhs_assembly").unwrap();
        for group in rhs["groups"].as_array_mut().unwrap() {
            group["node_ref"] = swap(&group["node_ref"]);
        }
        let order = rhs["node_order"].as_array().unwrap().clone();
        let (c1, c2) = (order.iter().position(|n| n == "node:C1").unwrap() * 6, order.iter().position(|n| n == "node:C2").unwrap() * 6);
        for axis in 0..3 {
            let (a, b) = (rhs["assembled_pressure_rhs_global"][c1 + axis].clone(), rhs["assembled_pressure_rhs_global"][c2 + axis].clone());
            rhs["assembled_pressure_rhs_global"][c1 + axis] = b;
            rhs["assembled_pressure_rhs_global"][c2 + axis] = a;
        }
        let node = swap(&rhs["cancellation_screen_exception"]["node_ref"]);
        rhs["cancellation_screen_exception"]["node_ref"] = node.clone();
        for warning in forged["diagnostics"].as_array_mut().unwrap().iter_mut().filter(|d| d["code"] == WARNING) {
            warning["affected_refs"][2] = node.clone();
            reform_warning_message(warning, &node);
        }
        validate_pressure_evidence(&forged).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_eq!(verdict(&forged, &input["model"])["error"]["code"], "SOURCE_PHYSICS_MODEL_EXCEPTION_INTERIOR_NODE", "{mode:?}");
    }
}

/// The warning's message re-formed for a moved node (the message names the
/// record's node; the record's force is unchanged by the exchange).
fn reform_warning_message(warning: &mut Value, node: &Value) {
    let old = warning["message"].as_str().unwrap().to_owned();
    let (head, _) = old.rsplit_once(", at node ").unwrap();
    warning["message"] = json!(format!("{head}, at node {}; it is not a bound on the resulting displacement.", node.as_str().unwrap()));
}

/// A run along x whose interior node is off the axis in y by the smallest
/// subnormal, 2^-1074 m, on members of 3 m: each member's y chord component
/// is nonzero but its formed direction component underflows to exactly zero
/// (2^-1074 times a reciprocal length below one half). Neither member's y is
/// a source zero, so the producer keeps both y terms as groups of value zero;
/// the published groups omit them (zero binary64 direction) and publish both
/// y components as zero. Admitted (the outside diameters are one ulp apart),
/// read by both stages, and within the bound: the underflowed component's
/// lost term is inside the formation allowance (pressure_runtime
/// BORE_DEPARTURE_RATIO, "A formed zero that is not a source zero").
#[test]
fn a_formed_zero_from_an_underflowed_offset_at_the_smallest_subnormal_is_admitted_and_read() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let tiny = f64::from_bits(1);
    let input = chain(&[[0.0; 3], [3.0, tiny, 0.0], [LENGTH, 0.0, 0.0]], &[0.12, next_up(0.12, 1)], &[0, 2]);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", output["diagnostics"]);
        let record = assert_screen(&output, true, "underflowed offset");
        let loads = output["contract_evidence"]["pressure"][0]["applied_loads"].as_array().unwrap();
        for load in loads {
            assert_eq!(load["local_x_global"][1].as_f64().unwrap(), 0.0, "{mode:?}: the formed y component underflows to zero");
        }
        assert_ne!(input["model"]["nodes"][1]["position"]["y"].as_f64().unwrap(), 0.0, "the source coordinates differ in y");
        assert!(assembly(&output)["groups"].as_array().unwrap().iter().all(|g| g["component"] != "Fy"));
        let ratio = record["remainder_to_bore_force_ratio"].as_f64().unwrap();
        assert!(ratio <= record["consistency_bound_ratio"].as_f64().unwrap(), "{mode:?}");
        validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_eq!(verdict(&output, &input["model"])["ok"], true, "{mode:?}: {}", verdict(&output, &input["model"]));
    }
}

/// The third review's subnormal model: the collinear equal-bore run of
/// `exactly_collinear_...` at 1e-310 Pa. The original screen fails (every
/// assembled load is zero while a group value is not) and the exception is
/// taken; the two members' separately grouped subnormal cap loads at the
/// interior node cancel exactly, so the published assembled vector is all
/// zero. The warning stands, and what each sentence states holds of the case:
/// (1) the cap loads at the interior node are formed separately (one group per
/// member, distinct direction bits) and need not cancel, here they do; (2) the
/// separation comes from the rounding of the members' directions (equal bore
/// bits, exactly proportional chords, unequal formed directions); (3) the
/// formed groups are the published load (they sum to it, zero, and the readers
/// re-form it); (4) the record states a ratio within a consistency bound and
/// nothing of the effect.
#[test]
fn a_subnormal_pressure_s_exactly_cancelling_remainder_carries_a_true_warning() {
    use open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence;
    let mut input = chain(&[[0.0; 3], [1.0, 1.0, 1.0], [4.0, 4.0, 4.0]], &[0.12, 0.12], &[0, 2]);
    input["model"]["load_cases"][0]["pressure_regions"][0]["pressure"]["value"] = json!(1e-310);
    for mode in MODES {
        let output = serde_json::to_value(solve(input.clone(), mode)).unwrap();
        assert_eq!(output["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}: {}", output["diagnostics"]);
        let record = assert_screen(&output, true, "subnormal");
        assert!(assembly(&output)["cancellation_screen"].is_null(), "{mode:?}: the screen is not finite");
        let rhs = assembly(&output);
        assert!(rhs["assembled_pressure_rhs_global"].as_array().unwrap().iter().all(|v| v.as_f64().unwrap() == 0.0), "{mode:?}: all zero");
        assert_eq!(record["node_ref"], "node:C1", "{mode:?}");
        assert_eq!(record["remainder_to_bore_force_ratio"].as_f64().unwrap(), 0.0, "{mode:?}");
        let found = warnings(&output);
        assert_eq!(found.len(), 1, "{mode:?}");
        assert_eq!(found[0]["message"], warning_message(&output, CASE), "{mode:?}");
        // (5) the last sentence: the case, a remainder of zero newtons at the record's node, and no bound on the displacement.
        assert_eq!(record["remainder_force_n"].as_f64().unwrap(), 0.0, "{mode:?}");
        assert!(found[0]["message"].as_str().unwrap().ends_with(&format!(" In case {CASE}, the largest retained remainder relative to the pressure force on the bore is 0e0 N, at node node:C1; it is not a bound on the resulting displacement.")), "{mode:?}");
        assert!(found[0]["message"].as_str().unwrap().starts_with("At interior nodes of this case's pressure regions the cap loads of adjacent members are formed separately and need not cancel exactly in this model."));
        // (1) one group per member at each axis of the interior node, each nonzero, summing to zero.
        let at_node = rhs["groups"].as_array().unwrap().iter().filter(|g| g["node_ref"] == "node:C1").collect::<Vec<_>>();
        for component in ["Fx", "Fy", "Fz"] {
            let groups = at_node.iter().filter(|g| g["component"] == component).collect::<Vec<_>>();
            assert_eq!(groups.len(), 2, "{mode:?} {component}: formed separately");
            assert!(groups.iter().all(|g| g["terms"].as_array().unwrap().len() == 1 && g["assembled_force_n"].as_f64().unwrap() != 0.0));
            assert_ne!(groups[0]["direction_component_magnitude"], groups[1]["direction_component_magnitude"], "{mode:?}");
            assert_eq!(groups[0]["assembled_force_n"].as_f64().unwrap() + groups[1]["assembled_force_n"].as_f64().unwrap(), 0.0, "{mode:?}: here they cancel");
        }
        // (2) equal bore bits, unequal formed directions of exactly proportional chords.
        let bores = rhs["groups"].as_array().unwrap().iter().map(|g| g["source_inner_radius_hi_bits"].as_str().unwrap()).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(bores.len(), 1, "{mode:?}");
        let directions = output["contract_evidence"]["pressure"][0]["applied_loads"].as_array().unwrap().iter().map(|l| l["local_x_global"].clone()).collect::<Vec<_>>();
        assert_ne!(directions[0], directions[1], "{mode:?}");
        // (3) and (4): the readers re-form the load from the groups and read the record.
        validate_pressure_evidence(&output).unwrap_or_else(|e| panic!("{mode:?}: {e}"));
        assert_eq!(verdict(&output, &input["model"])["ok"], true, "{mode:?}: {}", verdict(&output, &input["model"]));
    }
}
