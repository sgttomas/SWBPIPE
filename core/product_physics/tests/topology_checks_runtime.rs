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
    json!({"model":{"schema_version":"0.6.0","document_kind":"openpipestress.product_preview.model",
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
