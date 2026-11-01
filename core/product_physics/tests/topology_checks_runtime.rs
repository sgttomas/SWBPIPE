//! W-8 (DEL-04-06 `Design/model-topology-checks.md`): the pre-solve topology
//! warnings for FR-KNOW-002 and PRD §12.8, through the captured value entry the
//! desktop uses, in both solver modes. Every finding is a warning; the solve's
//! numbers are those of the same parts solved alone.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const TOPOLOGY_CODES: [&str; 4] =
    ["MODEL_TOPOLOGY_DISCONNECTED", "NODE_COINCIDENT", "NODE_NEAR_DUPLICATE", "BRANCH_TOPOLOGY_INVALID"];

fn base(length_unit: &str, nodes: Value, pipes: Value, supports: Value, components: Value) -> Value {
    json!({"model":{"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:w8","units":{"length":length_unit,"force":"N","angle":"rad","pressure":"Pa","stress":"Pa","temperature":"degC"}},
        "analysis_status":{"mechanics":"ready_for_preview_diagnostics","rule_check":"not_performed_user_rule_inputs_missing","professional_acceptance":"not_provided"},
        "nodes":nodes,"pipe_segments":pipes,"supports":supports,"components":components,
        "materials":[{"id":"material:m","elastic_modulus":{"value":200e9,"unit":"Pa"},"shear_modulus":{"value":76.923076923e9,"unit":"Pa"},
            "thermal_expansion_coefficient":{"value":1.2e-5,"unit":"1/degC"},"provenance":"w8_invented_test_values"}],
        "load_cases":[{"id":"case:f","primitive_loads":[
            {"id":"load:f","category":"concentrated_force","target":{"type":"node","node":"node:b"},"direction":"global_y","magnitude":{"value":1000.0,"unit":"N"},"dimension":"force","provenance":"w8"}],
            "provenance":"w8"}],
        "combinations":[]},"materials":[]})
}
fn node(id: &str, x: f64, y: f64, z: f64) -> Value {
    json!({"id":id,"position":{"x":x,"y":y,"z":z},"provenance":"w8"})
}
fn pipe(id: &str, from: &str, to: &str, yref: [f64; 3]) -> Value {
    json!({"id":id,"from":from,"to":to,"section":{"outside_diameter":{"value":0.12,"unit":"m"},"wall_thickness":{"value":0.01,"unit":"m"}},"material":"material:m","y_reference":{"x":yref[0],"y":yref[1],"z":yref[2]},"provenance":"w8"})
}
fn anchor(id: &str, node: &str) -> Value {
    json!({"id":id,"node":node,"family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":"w8"})
}
fn tee(id: &str, node: &str, header: &str, branch: &str) -> Value {
    json!({"id":id,"kind":"tee","node":node,"geometry":{"branch_header_pipe_ref":header,"branch_branch_pipe_ref":branch},"provenance":"w8"})
}

fn envelope(wire: &Value, mode: PreviewSolverMode) -> Value {
    let envelope = run_linear_static_preview_value_with_mode(wire.clone(), mode).expect("captured entry");
    serde_json::to_value(envelope).unwrap()
}
fn diagnostics(v: &Value) -> Vec<Value> {
    v["diagnostics"].as_array().cloned().unwrap_or_default()
}
fn with_code(v: &Value, code: &str) -> Vec<Value> {
    diagnostics(v).into_iter().filter(|d| d["code"] == code).collect()
}
fn topology_codes(v: &Value) -> Vec<String> {
    diagnostics(v)
        .iter()
        .filter_map(|d| d["code"].as_str())
        .filter(|c| TOPOLOGY_CODES.contains(c))
        .map(str::to_string)
        .collect()
}
fn blocking(v: &Value) -> Vec<Value> {
    diagnostics(v).into_iter().filter(|d| d["severity"] == "blocking").collect()
}
fn uy_row(v: &Value, node: &str) -> f64 {
    let rows = v["results"].as_array().unwrap();
    rows.iter()
        .find(|r| r["entity_ref"] == node && r["kind"] == "global_nodal_displacement_y")
        .unwrap_or_else(|| panic!("no UY row for {node} among {} rows", rows.len()))["value"]
        .as_f64()
        .unwrap()
}

/// A 1 m cantilever along +x (a anchored, loaded tip b), alone or with a
/// second cantilever c-d along +z at `offset`, anchored at c when `anchored`.
fn cantilevers(second: Option<([f64; 3], bool)>, length_unit: &str, scale: f64) -> Value {
    let mut nodes = vec![node("node:a", 0.0, 0.0, 0.0), node("node:b", scale, 0.0, 0.0)];
    let mut pipes = vec![pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0])];
    let mut supports = vec![anchor("support:a", "node:a")];
    if let Some((c, anchored)) = second {
        nodes.push(node("node:c", c[0], c[1], c[2]));
        nodes.push(node("node:d", c[0], c[1], c[2] + scale));
        pipes.push(pipe("pipe:c-d", "node:c", "node:d", [0.0, 1.0, 0.0]));
        if anchored {
            supports.push(anchor("support:c", "node:c"));
        }
    }
    base(length_unit, json!(nodes), json!(pipes), json!(supports), json!([]))
}

#[test]
fn a_connected_model_has_no_topology_finding() {
    for mode in MODES {
        let v = envelope(&cantilevers(None, "m", 1.0), mode);
        assert!(blocking(&v).is_empty(), "{:?}", blocking(&v));
        assert!(topology_codes(&v).is_empty(), "{:?}", topology_codes(&v));
    }
}

#[test]
fn two_anchored_parts_warn_and_solve_as_each_part_alone() {
    let alone = envelope(&cantilevers(None, "m", 1.0), MODES[0]);
    let tip_alone = uy_row(&alone, "node:b");
    for mode in MODES {
        let v = envelope(&cantilevers(Some(([5.0, 0.0, 0.0], true)), "m", 1.0), mode);
        assert!(blocking(&v).is_empty(), "{:?}", blocking(&v));
        let found = with_code(&v, "MODEL_TOPOLOGY_DISCONNECTED");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        let d = &found[0];
        assert_eq!(d["severity"], "warning");
        assert_eq!(d["id"], "diagnostic:topology:disconnected");
        let message = d["message"].as_str().unwrap();
        assert!(message.contains("form 2 disconnected parts"), "{message}");
        assert!(message.contains("part 1 (from node:a; 2 nodes; supported)"), "{message}");
        assert!(message.contains("part 2 (from node:c; 2 nodes; supported)"), "{message}");
        assert_eq!(d["affected_refs"], json!(["node:a", "node:c"]));
        assert_eq!(uy_row(&v, "node:b"), tip_alone, "the warning changes no number");
        assert!(topology_codes(&v).iter().all(|c| c == "MODEL_TOPOLOGY_DISCONNECTED"));
    }
}

#[test]
fn an_unsupported_part_is_named_and_still_refused_by_the_solve() {
    for mode in MODES {
        let v = envelope(&cantilevers(Some(([5.0, 0.0, 0.0], false)), "m", 1.0), mode);
        let found = with_code(&v, "MODEL_TOPOLOGY_DISCONNECTED");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        let message = found[0]["message"].as_str().unwrap();
        assert!(message.contains("part 2 (from node:c; 2 nodes; no support)"), "{message}");
        let refused = diagnostics(&v)
            .into_iter()
            .any(|d| d["severity"] == "blocking" || d["code"] == "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM");
        assert!(refused, "the unsupported part is still refused: {:?}", diagnostics(&v));
    }
}

#[test]
fn an_isolated_anchored_node_is_not_a_part() {
    let mut wire = cantilevers(None, "m", 1.0);
    wire["model"]["nodes"].as_array_mut().unwrap().push(node("node:z", 3.0, 0.0, 0.0));
    wire["model"]["supports"].as_array_mut().unwrap().push(anchor("support:z", "node:z"));
    for mode in MODES {
        let v = envelope(&wire, mode);
        assert!(blocking(&v).is_empty(), "{:?}", blocking(&v));
        assert!(topology_codes(&v).is_empty(), "{:?}", topology_codes(&v));
    }
}

#[test]
fn coincident_and_near_duplicate_nodes_warn_in_any_length_unit() {
    // The default node-proximity tolerance is 5 mm, inclusive, whatever the
    // model's length unit or extent.
    for (unit, scale) in [("m", 1.0), ("mm", 1000.0)] {
        for mode in MODES {
            // c coincides with b.
            let v = envelope(&cantilevers(Some(([scale, 0.0, 0.0], true)), unit, scale), mode);
            let found = with_code(&v, "NODE_COINCIDENT");
            assert_eq!(found.len(), 1, "{unit}: {:?}", diagnostics(&v));
            assert_eq!(found[0]["severity"], "warning");
            assert_eq!(found[0]["affected_refs"], json!(["node:b", "node:c"]));
            assert!(found[0]["message"].as_str().unwrap().contains("1 pair of distinct nodes share a position"));
            assert!(with_code(&v, "NODE_NEAR_DUPLICATE").is_empty());
            assert_eq!(with_code(&v, "MODEL_TOPOLOGY_DISCONNECTED").len(), 1);

            // c lies 4.9 mm and exactly 5 mm from b: near, not coincident.
            for gap in [0.0049 * scale, 0.005 * scale] {
                let v = envelope(&cantilevers(Some(([scale, gap, 0.0], true)), unit, scale), mode);
                let found = with_code(&v, "NODE_NEAR_DUPLICATE");
                assert_eq!(found.len(), 1, "{unit} {gap}: {:?}", diagnostics(&v));
                assert_eq!(found[0]["severity"], "warning");
                assert_eq!(found[0]["affected_refs"], json!(["node:b", "node:c"]));
                let message = found[0]["message"].as_str().unwrap();
                assert!(message.contains("node-proximity tolerance (5e-3 m)"), "{message}");
                assert!(with_code(&v, "NODE_COINCIDENT").is_empty());
            }

            // c lies just beyond 5 mm from b: not near.
            let beyond = if unit == "m" { f64::from_bits(0.005_f64.to_bits() + 1) } else { 5.000001 };
            let v = envelope(&cantilevers(Some(([scale, beyond, 0.0], true)), unit, scale), mode);
            assert!(with_code(&v, "NODE_NEAR_DUPLICATE").is_empty(), "{unit}: {:?}", diagnostics(&v));
            assert!(with_code(&v, "NODE_COINCIDENT").is_empty());
        }
    }
}

/// The tolerance does not scale with the model: in a 2 km model two unjoined
/// nodes 4 mm apart warn, and 6 mm apart do not.
#[test]
fn the_tolerance_is_absolute_in_a_large_model() {
    for (gap, warns) in [(0.004, true), (0.006, false)] {
        let v = envelope(&cantilevers(Some(([2000.0, gap, 0.0], true)), "m", 2000.0), PreviewSolverMode::SparseInteractive);
        assert_eq!(with_code(&v, "NODE_NEAR_DUPLICATE").len(), usize::from(warns), "{gap}: {:?}", diagnostics(&v));
    }
}

/// REQ-04-06-012: the model sets the tolerance in
/// `model_settings.node_proximity_tolerance` (any catalog length unit); the
/// check and its message use that value, inclusive, instead of 5 mm.
#[test]
fn a_user_set_tolerance_replaces_the_default() {
    let with_tolerance = |mut wire: Value, value: f64, unit: &str| {
        wire["model"]["model_settings"] = json!({"node_proximity_tolerance": {"value": value, "unit": unit}});
        wire
    };
    for mode in MODES {
        // 8 mm apart: beyond the 5 mm default, within a user-set 10 mm.
        let wire = cantilevers(Some(([1.0, 0.008, 0.0], true)), "m", 1.0);
        assert!(with_code(&envelope(&wire, mode), "NODE_NEAR_DUPLICATE").is_empty());
        for (value, unit) in [(10.0, "mm"), (0.01, "m")] {
            let v = envelope(&with_tolerance(wire.clone(), value, unit), mode);
            let found = with_code(&v, "NODE_NEAR_DUPLICATE");
            assert_eq!(found.len(), 1, "{value} {unit}: {:?}", diagnostics(&v));
            let message = found[0]["message"].as_str().unwrap();
            assert!(message.contains("node-proximity tolerance (1e-2 m)"), "{message}");
            assert!(blocking(&v).is_empty(), "{:?}", blocking(&v));
        }
        // Exactly the user-set value is near; a smaller value than the gap is not.
        let v = envelope(&with_tolerance(wire.clone(), 8.0, "mm"), mode);
        assert_eq!(with_code(&v, "NODE_NEAR_DUPLICATE").len(), 1, "{:?}", diagnostics(&v));
        let gap_4mm = cantilevers(Some(([1.0, 0.004, 0.0], true)), "m", 1.0);
        let v = envelope(&with_tolerance(gap_4mm, 2.0, "mm"), mode);
        assert!(with_code(&v, "NODE_NEAR_DUPLICATE").is_empty(), "{:?}", diagnostics(&v));
    }
}

/// REQ-04-06-012: a non-finite, non-positive or non-length tolerance is refused
/// by the solve with the setting reader's named code, as the document validator
/// refuses it; nothing is solved.
#[test]
fn an_invalid_tolerance_is_refused_by_name() {
    let wire = cantilevers(Some(([1.0, 0.004, 0.0], true)), "m", 1.0);
    for mode in MODES {
        for (value, unit, code) in [
            (json!(0.0), "mm", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE"),
            (json!(-5.0), "mm", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE"),
            (json!(5.0), "N", "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH"),
            (json!(5.0), "furlong_invented", "MODEL-SETTING-TOLERANCE-UNIT-NOT-LENGTH"),
            (json!(1e-322), "mm", "MODEL-SETTING-TOLERANCE-NOT-POSITIVE"),
        ] {
            let mut bad = wire.clone();
            bad["model"]["model_settings"] = json!({"node_proximity_tolerance": {"value": value, "unit": unit}});
            let v = envelope(&bad, mode);
            let refused = blocking(&v);
            assert_eq!(refused.len(), 1, "{value} {unit}: {:?}", diagnostics(&v));
            assert_eq!(refused[0]["code"], code, "{value} {unit}");
            assert_eq!(refused[0]["affected_refs"], json!(["model_settings.node_proximity_tolerance"]));
            assert!(v["results"].as_array().is_none_or(|rows| rows.is_empty()), "{value} {unit}: nothing is solved");
            assert!(topology_codes(&v).is_empty(), "{value} {unit}: the topology check does not run");
        }
    }
}

/// a-b-c a straight run with a branch b-d along +y, or without it.
fn run_with_tee(branch: bool, header: &str, branch_ref: &str) -> Value {
    let mut nodes = vec![node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0), node("node:c", 2.0, 0.0, 0.0)];
    let mut pipes = vec![pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]), pipe("pipe:b-c", "node:b", "node:c", [0.0, 1.0, 0.0])];
    let mut supports = vec![anchor("support:a", "node:a"), anchor("support:c", "node:c")];
    if branch {
        nodes.push(node("node:d", 1.0, 1.0, 0.0));
        pipes.push(pipe("pipe:b-d", "node:b", "node:d", [1.0, 0.0, 0.0]));
        supports.push(anchor("support:d", "node:d"));
    }
    base("m", json!(nodes), json!(pipes), json!(supports), json!([tee("component:tee", "node:b", header, branch_ref)]))
}

#[test]
fn a_tee_must_join_three_pipes_with_distinct_header_and_branch() {
    for mode in MODES {
        let v = envelope(&run_with_tee(true, "pipe:a-b", "pipe:b-d"), mode);
        assert!(with_code(&v, "BRANCH_TOPOLOGY_INVALID").is_empty(), "{:?}", diagnostics(&v));

        let v = envelope(&run_with_tee(false, "pipe:a-b", "pipe:b-c"), mode);
        let found = with_code(&v, "BRANCH_TOPOLOGY_INVALID");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        assert_eq!(found[0]["severity"], "warning");
        assert_eq!(found[0]["affected_refs"], json!(["component:tee", "node:b"]));
        assert!(found[0]["message"].as_str().unwrap().ends_with("node node:b joins 2"));

        let v = envelope(&run_with_tee(true, "pipe:b-d", "pipe:b-d"), mode);
        let found = with_code(&v, "BRANCH_TOPOLOGY_INVALID");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        assert!(found[0]["message"].as_str().unwrap().ends_with("node node:b joins 3"));
    }
}

/// Owner ruling (2026-10-23): topology findings warn, not refuse, and the warning
/// says plainly that the tee's branch stress factors are still applied to the
/// pipes it names; without entered factors the warning claims none.
#[test]
fn an_invalid_tee_says_which_branch_stress_factors_still_apply() {
    let with_factors = |mut model: Value| {
        model["model"]["components"][0]["modifiers"] = json!({
            "branch_header_sif_user_value": {"value": 1.4, "unit": "none"},
            "branch_branch_sif_user_value": {"value": 1.9, "unit": "none"},
            "flexibility_factor_user_value": {"value": 1.0, "unit": "none"},
            "source_reference": "w8_invented_factors"
        });
        model
    };
    let intensified = |v: &Value, pipe: &str| {
        v["results"].as_array().unwrap().iter().filter(|r| {
            let id = r["id"].as_str().unwrap_or("");
            id.contains("intensified-bending:component-tee") && id.contains(&pipe.replace(':', "-"))
        }).count()
    };
    for mode in MODES {
        let v = envelope(&with_factors(run_with_tee(false, "pipe:a-b", "pipe:b-c")), mode);
        let found = with_code(&v, "BRANCH_TOPOLOGY_INVALID");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        assert!(found[0]["message"].as_str().unwrap().ends_with(
            "node node:b joins 2; its branch stress factors are still applied (header SIF to pipe:a-b, branch SIF to pipe:b-c)"
        ), "{}", found[0]["message"]);
        assert!(intensified(&v, "pipe:a-b") > 0 && intensified(&v, "pipe:b-c") > 0, "the named factors are applied");

        let v = envelope(&with_factors(run_with_tee(true, "pipe:b-d", "pipe:b-d")), mode);
        let found = with_code(&v, "BRANCH_TOPOLOGY_INVALID");
        assert!(found[0]["message"].as_str().unwrap().ends_with(
            "node node:b joins 3; its branch stress factors are still applied (header SIF to pipe:b-d)"
        ), "{}", found[0]["message"]);
        assert!(intensified(&v, "pipe:b-d") > 0);
        assert_eq!(intensified(&v, "pipe:a-b") + intensified(&v, "pipe:b-c"), 0);

        let v = envelope(&run_with_tee(false, "pipe:a-b", "pipe:b-c"), mode);
        assert_eq!(intensified(&v, "pipe:a-b") + intensified(&v, "pipe:b-c"), 0, "no factors entered");
    }
}

/// The sides an envelope publishes intensified rows for, read from its
/// evidence records ("header SIF to pipe:…" or "branch SIF to pipe:…"); each
/// record's row is checked to be a published intensified row, and every such
/// row of the tee to have its record.
fn published_sides(v: &Value) -> Vec<String> {
    let rows = v["results"].as_array().cloned().unwrap_or_default();
    let mut sides = Vec::new();
    let mut records = 0;
    for case in v["contract_evidence"]["preview_cases"].as_array().into_iter().flatten() {
        for record in case["intensified_measures"].as_array().into_iter().flatten() {
            if record["component_id"] != "component:tee" {
                continue;
            }
            records += 1;
            let id = record["result_id"].as_str().unwrap();
            assert!(rows.iter().any(|r| r["id"] == id && r["kind"] == "component_equal_factor_intensified_bending_stress_v1"
                && r["entity_ref"] == "component:tee"), "the record's row {id} is published");
            let side = match record["factor_role"].as_str() {
                Some("branch_header") => "header",
                Some("branch_branch") => "branch",
                other => panic!("unexpected factor role {other:?}"),
            };
            let named = format!("{side} SIF to {}", record["pipe_id"].as_str().unwrap());
            if !sides.contains(&named) {
                sides.push(named);
            }
        }
    }
    let tee_rows = rows.iter().filter(|r| r["kind"] == "component_equal_factor_intensified_bending_stress_v1"
        && r["entity_ref"] == "component:tee").count();
    assert_eq!(tee_rows, records, "every published intensified row of the tee has its record");
    sides.sort_by_key(|side| !side.starts_with("header"));
    sides
}

/// REQ-04-06-012 and the owner's ruling (2026-10-23): the branch-topology warning
/// names exactly the sides whose intensified rows the envelope publishes: never
/// a side it does not publish, never omitting one it does, in both solver modes.
/// Every combination of header and branch SIF (absent, zero, negative, positive,
/// positive in a non-dimensionless unit) and flexibility factor (absent, present,
/// zero, present in a non-dimensionless unit) on: a node joining two pipes; a header
/// pipe that is missing or does not end at the node; a header that is the branch
/// (on three pipes); a missing branch pipe; another solver consumption; a branch
/// pipe realized as a curved-bend arc; and a valid tee (no warning). Also a tee
/// without modifiers, and a model refused for missing supports (the warning
/// stays; nothing is claimed).
#[test]
fn the_branch_warning_names_exactly_the_published_sides() {
    let factors = [
        None,
        Some(json!({"value": 0.0, "unit": "none"})),
        Some(json!({"value": -1.0, "unit": "none"})),
        Some(json!({"value": 1.4, "unit": "none"})),
        Some(json!({"value": 1.9, "unit": "N"})),
    ];
    let flexibilities = [
        None,
        Some(json!({"value": 1.0, "unit": "none"})),
        Some(json!({"value": 0.0, "unit": "none"})),
        Some(json!({"value": 1.0, "unit": "N"})),
    ];
    let topology = |degree: usize| {
        format!("a branch component's node must join exactly three pipes, and its header and branch pipes must differ; node node:b joins {degree}")
    };
    // (scenario, the model, the node's pipe count; None for a valid tee).
    let arc = || {
        // a-b straight; b-c a quarter arc realized by the bend at c; c-d straight.
        base("m",
            json!([node("node:a", 0.0, 0.0, 0.0), node("node:b", 1.0, 0.0, 0.0), node("node:c", 1.2, 0.2, 0.0), node("node:d", 1.2, 1.2, 0.0)]),
            json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]),
                pipe("pipe:b-c", "node:b", "node:c", [std::f64::consts::FRAC_1_SQRT_2, -std::f64::consts::FRAC_1_SQRT_2, 0.0]),
                pipe("pipe:c-d", "node:c", "node:d", [1.0, 0.0, 0.0])]),
            json!([anchor("support:a", "node:a"), anchor("support:d", "node:d")]),
            json!([tee("component:tee", "node:b", "pipe:a-b", "pipe:b-c"),
                {"id":"component:bend","kind":"bend","node":"node:c",
                 "geometry":{"bend_pipe_ref":"pipe:b-c","bend_radius":{"value":0.2,"unit":"m"},"bend_angle":{"value":std::f64::consts::FRAC_PI_2,"unit":"rad"},
                    "bend_plane_orientation":"global_xy_preview","bend_geometry_source_reference":"w8"},
                 "modifiers":{"flexibility_factor_user_value":{"value":2.0,"unit":"none"},"source_reference":"w8_invented"},
                 "mechanics_interface":{"solver_consumption":"curved_bend_macro_element"},"provenance":"w8"}]))
    };
    let remote = || {
        let mut w = run_with_tee(false, "pipe:remote", "pipe:b-c");
        w["model"]["nodes"].as_array_mut().unwrap().extend([node("node:r", 5.0, 0.0, 0.0), node("node:s", 6.0, 0.0, 0.0)]);
        w["model"]["pipe_segments"].as_array_mut().unwrap().push(pipe("pipe:remote", "node:r", "node:s", [0.0, 1.0, 0.0]));
        w["model"]["supports"].as_array_mut().unwrap().push(anchor("support:r", "node:r"));
        w
    };
    let other_consumption = || {
        let mut w = run_with_tee(false, "pipe:a-b", "pipe:b-c");
        w["model"]["components"][0]["mechanics_interface"] = json!({"solver_consumption": "not_solver_consumed"});
        w
    };
    let scenarios: Vec<(&str, Value, Option<usize>)> = vec![
        ("two pipes", run_with_tee(false, "pipe:a-b", "pipe:b-c"), Some(2)),
        ("missing header pipe", run_with_tee(false, "pipe:absent", "pipe:b-c"), Some(2)),
        ("header pipe not at the node", remote(), Some(2)),
        ("header is the branch", run_with_tee(true, "pipe:a-b", "pipe:a-b"), Some(3)),
        ("missing branch pipe", run_with_tee(false, "pipe:a-b", "pipe:absent"), Some(2)),
        ("other solver consumption", other_consumption(), Some(2)),
        ("arc branch pipe", arc(), Some(2)),
        ("valid tee", run_with_tee(true, "pipe:a-b", "pipe:b-d"), None),
    ];
    let (mut cases, mut claimed_without_flexibility, mut refused_with_valid_sif, mut arc_header_only) = (0, 0, 0, 0);
    for mode in MODES {
        for (name, model, degree) in &scenarios {
            for header in &factors {
                for branch in &factors {
                    for flexibility in &flexibilities {
                        let mut w = model.clone();
                        let modifiers = &mut w["model"]["components"][0]["modifiers"];
                        *modifiers = json!({"source_reference": "w8_invented_factors"});
                        for (key, value) in [("branch_header_sif_user_value", header), ("branch_branch_sif_user_value", branch),
                            ("flexibility_factor_user_value", flexibility)] {
                            if let Some(value) = value {
                                modifiers[key] = value.clone();
                            }
                        }
                        let v = envelope(&w, mode);
                        let published = published_sides(&v);
                        let found = with_code(&v, "BRANCH_TOPOLOGY_INVALID");
                        let context = format!("{name} {mode:?} header={header:?} branch={branch:?} flexibility={flexibility:?}");
                        let Some(degree) = degree else {
                            assert!(found.is_empty(), "{context}: {:?}", diagnostics(&v));
                            cases += 1;
                            continue;
                        };
                        assert_eq!(found.len(), 1, "{context}: {:?}", diagnostics(&v));
                        let expected = if published.is_empty() {
                            topology(*degree)
                        } else {
                            format!("{}; its branch stress factors are still applied ({})", topology(*degree), published.join(", "))
                        };
                        assert_eq!(found[0]["message"], expected.as_str(), "{context}");
                        if v["status"]["mechanics"] != "MECHANICS_SOLVED" {
                            assert!(published.is_empty(), "{context}: a refused envelope publishes nothing");
                            if header.as_ref().is_some_and(|q| q["unit"] == "none" && q["value"] == 1.4) {
                                refused_with_valid_sif += 1;
                            }
                        }
                        if flexibility.is_none() && !published.is_empty() {
                            claimed_without_flexibility += 1;
                        }
                        if *name == "arc branch pipe" {
                            // The arc publishes no intensified row (its member is not straight).
                            assert!(published.iter().all(|side| side.starts_with("header")), "{context}: {published:?}");
                            if v["status"]["mechanics"] == "MECHANICS_SOLVED" {
                                assert_eq!(v["summary"]["curved_bend_macro_element_count"], 1, "{context}");
                            }
                            if published == ["header SIF to pipe:a-b"] && branch.as_ref().is_some_and(|q| q["unit"] == "none" && q["value"] == 1.4) {
                                arc_header_only += 1;
                            }
                        }
                        cases += 1;
                    }
                }
            }
        }
        // A tee with no modifiers claims nothing.
        let v = envelope(&run_with_tee(false, "pipe:a-b", "pipe:b-c"), mode);
        assert!(published_sides(&v).is_empty());
        assert_eq!(with_code(&v, "BRANCH_TOPOLOGY_INVALID")[0]["message"], topology(2).as_str());
        // Refused after the topology check (no support): the warning stays, without the statement.
        let mut w = run_with_tee(false, "pipe:a-b", "pipe:b-c");
        w["model"]["components"][0]["modifiers"] = json!({
            "branch_header_sif_user_value": {"value": 1.4, "unit": "none"},
            "branch_branch_sif_user_value": {"value": 1.9, "unit": "none"},
            "flexibility_factor_user_value": {"value": 1.0, "unit": "none"},
            "source_reference": "w8_invented_factors"
        });
        w["model"]["supports"] = json!([]);
        let v = envelope(&w, mode);
        assert!(blocking(&v).iter().any(|d| d["code"] == "SUPPORT_INPUT_MISSING"), "{:?}", diagnostics(&v));
        assert!(published_sides(&v).is_empty());
        assert_eq!(with_code(&v, "BRANCH_TOPOLOGY_INVALID")[0]["message"], topology(2).as_str());
        cases += 2;
    }
    // The combinations reach the cases the review found: a side published with no
    // flexibility factor is named; a refused envelope with a valid SIF claims none;
    // an arc branch pipe publishes (and the warning names) the header only.
    assert!(claimed_without_flexibility > 0 && refused_with_valid_sif > 0 && arc_header_only > 0,
        "{claimed_without_flexibility} {refused_with_valid_sif} {arc_header_only}");
    assert_eq!(cases, 2 * (8 * 5 * 5 * 4 + 2));
}

/// Two unjoined nodes on a part each, 4 mm apart across a cell boundary
/// (cells are 8 x 5 mm = 0.04 m wide; the pair straddles x = 0.04).
#[test]
fn a_near_pair_across_a_cell_boundary_is_found() {
    let (a, b) = (0.038_f64, 0.042_f64);
    let nodes = json!([node("node:a", -1.0, 0.0, 0.0), node("node:b", a, 0.0, 0.0),
        node("node:c", b, 0.0, 0.0), node("node:d", 1.0, 0.0, 0.0)]);
    let pipes = json!([pipe("pipe:a-b", "node:a", "node:b", [0.0, 1.0, 0.0]), pipe("pipe:c-d", "node:c", "node:d", [0.0, 1.0, 0.0])]);
    let supports = json!([anchor("support:a", "node:a"), anchor("support:d", "node:d")]);
    for mode in MODES {
        let v = envelope(&base("m", nodes.clone(), pipes.clone(), supports.clone(), json!([])), mode);
        let found = with_code(&v, "NODE_NEAR_DUPLICATE");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        assert_eq!(found[0]["affected_refs"], json!(["node:b", "node:c"]));
    }
}

/// Distances do not underflow: a gap of 1e-200 m is near, not coincident.
/// (Squares cannot overflow through this entry: the checked-JSON gate refuses
/// floats at or above 2^53 as unsafe integral floats; the norm is scaled anyway.)
#[test]
fn a_tiny_gap_is_near_not_coincident() {
    let tiny = cantilevers(Some(([1.0, 1e-200, 0.0], true)), "m", 1.0);
    for mode in MODES {
        let v = envelope(&tiny, mode);
        assert!(with_code(&v, "NODE_COINCIDENT").is_empty(), "{:?}", diagnostics(&v));
        let found = with_code(&v, "NODE_NEAR_DUPLICATE");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        assert!(found[0]["message"].as_str().unwrap().contains("node:b and node:c (1e-200 m apart)"));
    }
}

/// Eight anchored parts: six are named, the rest counted.
#[test]
fn more_than_six_parts_are_counted_and_six_named() {
    let mut nodes = Vec::new();
    let mut pipes = Vec::new();
    let mut supports = Vec::new();
    for k in 0..8 {
        let (i, j) = (format!("node:{k}i"), format!("node:{k}j"));
        nodes.push(node(&i, 0.0, 0.0, 2.0 * k as f64));
        nodes.push(node(&j, 1.0, 0.0, 2.0 * k as f64));
        pipes.push(pipe(&format!("pipe:{k}"), &i, &j, [0.0, 1.0, 0.0]));
        supports.push(anchor(&format!("support:{k}"), &i));
    }
    // The load case loads node:b, which does not exist here: load the first part's tip.
    let mut wire = base("m", json!(nodes), json!(pipes), json!(supports), json!([]));
    wire["model"]["load_cases"][0]["primitive_loads"][0]["target"]["node"] = json!("node:0j");
    for mode in MODES {
        let v = envelope(&wire, mode);
        let found = with_code(&v, "MODEL_TOPOLOGY_DISCONNECTED");
        assert_eq!(found.len(), 1, "{:?}", diagnostics(&v));
        let message = found[0]["message"].as_str().unwrap();
        assert!(message.contains("form 8 disconnected parts"), "{message}");
        assert!(message.ends_with("; and 2 more"), "{message}");
        assert_eq!(found[0]["affected_refs"].as_array().unwrap().len(), 6);
    }
}
