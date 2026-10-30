//! T4-U5b references (`validation/references/t4_u5b/`) through the actual
//! typed, captured and headless consumers, in both solver modes, with export
//! (DEL-04-07 Design `u4b-u5b-implementation.md` §7 item 2). Each of the 38
//! requests is authored in model 0.8.0 in memory from its SHA-bound case
//! description by the shared builder (`product_physics/tests/common/t4_u5b.rs`):
//! its t4_u5a base is re-expressed by the current-model rule and the rods are
//! added in §3.1's shape, offsets in the connector axes. Every frozen
//! observable the product publishes is compared at t4_u5b's criterion
//! (|obs − exp| ≤ 1e-9·max(|exp|, F)); the discriminators' wrong values must
//! lie outside it. Oracle values never move. These controls grant no
//! retained permit or qualification.
use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_mode, PrivacyContext, ProfessionalBoundary, Provenance,
    RedistributionStatus, Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode, PreviewSolverMode,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[path = "../../../product_physics/tests/common/current_model.rs"]
mod current_model;
#[path = "../../../product_physics/tests/common/t4_u5b.rs"]
mod t4_u5b;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
const RELATIVE: f64 = 1e-9;

fn runner_request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "u5b-current-reference".into(), operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "ops.solve.job"),
        project_ref: Reference::new("project", model["project"]["id"].as_str().unwrap()),
        model_ref: Reference::new("model", model["project"]["id"].as_str().unwrap()),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: model["load_cases"].as_array().unwrap().iter()
            .map(|c| Reference::new("load_case", c["id"].as_str().unwrap())).collect(),
        input_manifest_ref: Reference::new("audit_manifest", "invented-u5b-current-reference"),
        requested_outputs: vec!["result_envelope".into(), "audit_manifest".into(), "diagnostics".into()],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented independent U5b controls".into(),
            source_location: "validation/references/t4_u5b".into(),
            source_license: "project invented".into(), contributor: "OpenPipeStress".into(),
            contributor_certification: "invented non-engineering controls".into(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "independently refuted reference".into(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}

/// The captured, typed and headless envelopes (identical results and
/// evidence) and the headless export document.
fn run(request: &Value, mode: PreviewSolverMode) -> (Vec<Value>, Option<Value>) {
    let captured = serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
    let typed = serde_json::to_value(run_linear_static_preview_with_mode(
        serde_json::from_value(request.clone()).unwrap(), mode)).unwrap();
    let headless = run_preview_model_value_with_mode(runner_request(&request["model"]), request.clone(), mode).unwrap();
    let raw = serde_json::to_value(headless.mechanics_envelope.unwrap()).unwrap();
    for other in [&typed, &raw] {
        assert_eq!(captured["status"], other["status"]);
        assert_eq!(captured["results"], other["results"]);
        assert_eq!(captured["contract_evidence"], other["contract_evidence"]);
    }
    (vec![captured, typed, raw], headless.result_envelope_document)
}

fn rows<'a>(source: &'a Value, kind: &str, entity: &str, component: &str, location: Option<&str>) -> Vec<&'a Value> {
    source["results"].as_array().unwrap().iter().filter(|row|
        row["kind"] == kind && row["entity_ref"] == entity && row["metadata"]["component"] == component
        && location.is_none_or(|s| row["metadata"]["location"] == s)).collect()
}
fn si(row: &Value) -> f64 {
    let value = row["value"].as_f64().unwrap();
    match row["unit"].as_str().unwrap() {
        "mm" => value / 1000.0, "m" | "rad" | "N" | "N*m" | "Pa" => value,
        unit => panic!("unexpected published unit {unit}"),
    }
}
fn row(source: &Value, kind: &str, entity: &str, component: &str, location: Option<&str>) -> f64 {
    let found = rows(source, kind, entity, component, location);
    assert_eq!(found.len(), 1, "{kind} {entity} {component} {location:?}");
    si(found[0])
}
fn by_id<'a>(source: &'a Value, id: &str) -> Option<&'a Value> {
    source["results"].as_array().unwrap().iter().find(|row| row["id"] == id)
}
fn stable_suffix(id: &str) -> String { id.replace(':', "-") }
fn rod_row_id(component: &str, rod: &str) -> String {
    format!("result:connector:{}:tie-rod-force:tie_rod:{}:t_r", stable_suffix(component), stable_suffix(rod))
}

/// The criterion family of an observable key (t4_u5b README "Criterion").
fn floor(key: &str) -> (&'static str, f64) {
    let last = key.rsplit('.').collect::<Vec<_>>();
    if key.starts_with("q.translation") || key.contains(".translation.") || key.ends_with(".extension") || key.ends_with("elongation") || key.ends_with("separation") || key.ends_with("axial_displacement") {
        ("length_m", 1e-5)
    } else if key.starts_with("q.rotation") || key.contains(".rotation.") {
        ("rotation_rad", 1e-5)
    } else if key.starts_with("g.moment") || key.contains(".moment.") || key.contains(".moment_local.") {
        ("moment_nm", 1.0)
    } else if key.starts_with("direction") || key.contains("direction_global") || key.ends_with("axial_share") {
        ("unitless", 1.0)
    } else {
        let _ = last;
        ("force_n", 1.0)
    }
}

struct Case<'a> {
    name: &'a str,
    oracle: &'a Value,
    request: &'a Value,
}

impl Case<'_> {
    fn connector(&self) -> &str { self.request["model"]["components"][0]["id"].as_str().unwrap() }
    fn pipe(&self, id: &str) -> &Value {
        self.request["model"]["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == id).unwrap()
    }
    fn position(&self, id: &str) -> [f64; 3] {
        let node = self.request["model"]["nodes"].as_array().unwrap().iter().find(|n| n["id"] == id).unwrap();
        ["x", "y", "z"].map(|a| node["position"][a].as_f64().unwrap())
    }
    fn pressurized(&self, pipe: &str) -> bool {
        self.request["model"]["load_cases"][0].get("pressure_regions").and_then(Value::as_array).into_iter().flatten()
            .any(|r| r["member_pipe_ids"].as_array().unwrap().iter().any(|id| id == pipe))
    }
    /// The member frame (x along the chord, y = y_reference, z = x × y).
    fn frame(&self, pipe: &str) -> [[f64; 3]; 3] {
        let p = self.pipe(pipe);
        let (a, b) = (self.position(p["from"].as_str().unwrap()), self.position(p["to"].as_str().unwrap()));
        let d: [f64; 3] = std::array::from_fn(|i| b[i] - a[i]);
        let l = d.iter().map(|v| v * v).sum::<f64>().sqrt();
        let x = d.map(|v| v / l);
        let y = ["x", "y", "z"].map(|k| p["y_reference"][k].as_f64().unwrap());
        let z = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]];
        [x, y, z]
    }
    fn expected(&self, key: &str) -> Option<f64> {
        let c = self.oracle;
        let n = t4_u5b::number;
        let axis = |s: &str| ["x", "y", "z"].iter().position(|a| *a == s).unwrap();
        let parts: Vec<&str> = key.split('.').collect();
        Some(match parts.as_slice() {
            ["S_joint"] => n(&c["S_joint_n"]),
            ["Tb"] => n(&c["Tb_gx_n"]),
            ["Pe"] => n(&c["Pe_n"]),
            ["sum_rod_tension"] => n(&c["sum_rod_tension_n"]),
            ["q", "translation", a] => n(&c["generalized"]["q_translation_m"][axis(a)]),
            ["q", "rotation", a] => n(&c["generalized"]["q_rotation_rad"][axis(a)]),
            ["g", "force", a] => n(&c["generalized"]["g_force_n"][axis(a)]),
            ["g", "moment", a] => n(&c["generalized"]["g_moment_nm"][axis(a)]),
            ["connector_endpoint", end, kind, a] => n(&c["connector_endpoint_elastic_node_on_element"][*end][if *kind == "force" { "force_global_n" } else { "moment_global_nm" }][axis(a)]),
            ["node", node, "translation", a] => n(&c["nodes"][*node]["translation_global_m"][axis(a)]),
            ["node", node, "rotation", a] => n(&c["nodes"][*node]["rotation_global_rad"][axis(a)]),
            ["node", node, "axial_displacement"] => n(&c["nodes"][*node]["axial_displacement_m"]),
            ["pipe", pipe, "Nw"] => n(&c["pipes"][*pipe]["Nw_n"]),
            ["pipe", pipe, "S"] => n(&c["pipes"][*pipe]["S_n"]),
            ["pipe", pipe, "elongation"] => n(&c["pipes"][*pipe]["elongation_m"]),
            ["pipe", pipe, end, kind, a] => {
                let field = match *kind { "force" => "force_global_n", "moment" => "moment_global_nm", "force_local" => "force_local_n", _ => "moment_local_nm" };
                n(&c["pipes"][*pipe]["end_actions_node_on_element"][*end][field][axis(a)])
            }
            ["reaction", support, kind, a] => n(&c["reactions_support_on_model"][*support][if *kind == "force" { "force_global_n" } else { "moment_global_nm" }][axis(a)]),
            ["rod", rod, "tension"] => n(&c["rods"][*rod]["tension_n"]),
            ["rod", rod, "axial_share"] => n(&c["rods"][*rod]["direction_dot_connector_x"]),
            ["rod", rod, "direction_global", a] => n(&c["rods"][*rod]["direction_global"][axis(a)]),
            ["identity", face, "S_pipe_minus_S_joint"] => n(&c["interface_identity"][*face]["S_pipe_n"]) - n(&c["interface_identity"][*face]["S_joint_n"]),
            ["identity", face, "S_pipe"] => n(&c["interface_identity"][*face]["S_pipe_n"]),
            ["identity", face, "S_joint"] => n(&c["interface_identity"][*face]["S_joint_n"]),
            ["separation"] => n(c.get("outer_end_separation_change_m")?),
            ["rhs", node, a] => n(&c["pressure_rhs_global_n"][*node][axis(a)]),
            _ => return None,
        })
    }
    /// The published value of an observable, or `None` when the product does
    /// not publish it (rod extensions and rod nodal actions: §3.3 publishes
    /// one force row per rod).
    fn observed(&self, source: &Value, key: &str) -> Option<f64> {
        let connector = self.connector();
        let axis = |s: &str| ["x", "y", "z"].iter().position(|a| *a == s).unwrap();
        let local = Some("connector_local");
        let parts: Vec<&str> = key.split('.').collect();
        let wall = |pipe: &str| if self.pressurized(pipe) {
            row(source, "pipe_wall_axial_force_v2", pipe, "wall_axial_force", Some("midspan"))
        } else {
            row(source, "element_local_axial_force", pipe, "axial_force", Some("end_j"))
        };
        let effective = |pipe: &str| if self.pressurized(pipe) {
            row(source, "pipe_effective_axial_force_v2", pipe, "effective_axial_force", Some("midspan"))
        } else {
            wall(pipe)
        };
        let s_joint = || row(source, "connector_effective_axial_force_v1", connector, "s_x", local);
        Some(match parts.as_slice() {
            ["S_joint"] => s_joint(),
            ["Tb"] | ["g", "force", "x"] => row(source, "connector_generalized_force_v1", connector, "gt_x", local),
            ["Pe"] => {
                let found = rows(source, "connector_pressure_force_v1", connector, "pe_x", local);
                if found.is_empty() { 0.0 } else { assert_eq!(found.len(), 1); si(found[0]) }
            }
            ["sum_rod_tension"] => self.oracle["rods"].as_object().unwrap().keys()
                .map(|rod| si(by_id(source, &rod_row_id(connector, rod)).expect("rod row"))).sum(),
            ["q", "translation", a] => row(source, "connector_generalized_translation_v1", connector, &format!("qt_{a}"), local),
            ["q", "rotation", a] => row(source, "connector_generalized_rotation_v1", connector, &format!("qr_{a}"), local),
            ["g", "force", a] => row(source, "connector_generalized_force_v1", connector, &format!("gt_{a}"), local),
            ["g", "moment", a] => row(source, "connector_generalized_moment_v1", connector, &format!("gr_{a}"), local),
            ["connector_endpoint", end, kind, a] => row(source, &format!("connector_endpoint_{kind}_v1"), connector,
                &format!("{}{}", if *kind == "force" { "F" } else { "M" }, a), Some(end)),
            ["node", node, "translation", a] => row(source, &format!("global_nodal_displacement_{a}"), node, &format!("nodal_displacement_{a}"), None),
            ["node", node, "rotation", a] => row(source, &format!("global_nodal_rotation_{a}"), node, &format!("nodal_rotation_{a}"), None),
            ["node", node, "axial_displacement"] => {
                let q = &self.oracle["chain_axis_global"];
                (0..3).map(|i| self.observed(source, &format!("node.{node}.translation.{}", ["x", "y", "z"][i])).unwrap() * t4_u5b::number(&q[i])).sum()
            }
            ["pipe", pipe, "Nw"] => wall(pipe),
            ["pipe", pipe, "S"] => effective(pipe),
            ["pipe", pipe, "elongation"] => {
                let p = self.pipe(pipe);
                let (from, to) = (p["from"].as_str().unwrap(), p["to"].as_str().unwrap());
                let x = self.frame(pipe)[0];
                (0..3).map(|i| {
                    let a = ["x", "y", "z"][i];
                    (self.observed(source, &format!("node.{to}.translation.{a}")).unwrap()
                        - self.observed(source, &format!("node.{from}.translation.{a}")).unwrap()) * x[i]
                }).sum()
            }
            ["pipe", pipe, end, kind, a] => {
                let local_force = |k: usize| match k {
                    0 if self.pressurized(pipe) => row(source, "pipe_wall_endpoint_action_v2", pipe, "wall_axial_end_action", Some(end)),
                    0 => row(source, "element_local_axial_force", pipe, "axial_force", Some(end)),
                    1 => row(source, "element_local_shear_force_y", pipe, "shear_force_y", Some(end)),
                    _ => row(source, "element_local_shear_force_z", pipe, "shear_force_z", Some(end)),
                };
                let local_moment = |k: usize| match k {
                    0 => row(source, "element_local_torsional_moment", pipe, "torsional_moment", Some(end)),
                    1 => row(source, "element_local_bending_moment_y", pipe, "bending_moment_y", Some(end)),
                    _ => row(source, "element_local_bending_moment_z", pipe, "bending_moment_z", Some(end)),
                };
                match *kind {
                    "force_local" => local_force(axis(a)),
                    "moment_local" => local_moment(axis(a)),
                    _ => {
                        let f = self.frame(pipe);
                        let k = axis(a);
                        (0..3).map(|b| f[b][k] * if *kind == "force" { local_force(b) } else { local_moment(b) }).sum()
                    }
                }
            }
            ["reaction", support, kind, a] => row(source, "support_reaction_component_v2", support,
                &format!("{}{}", if *kind == "force" { "F" } else { "M" }, a), None),
            ["rod", rod, "tension"] => si(by_id(source, &rod_row_id(connector, rod)).expect("rod row")),
            ["rod", rod, "axial_share"] => self.rod_evidence(source, rod)["axial_share"].as_f64().unwrap(),
            ["rod", rod, "direction_global", a] => self.rod_evidence(source, rod)["direction_global"][axis(a)].as_f64().unwrap(),
            ["rod", _, "extension"] | ["rod", _, _, _, _] => return None,
            ["identity", face, "S_pipe"] => effective(self.oracle["interface_identity"][*face]["pipe"].as_str().unwrap()),
            ["identity", _, "S_joint"] => s_joint(),
            ["identity", face, "S_pipe_minus_S_joint"] => effective(self.oracle["interface_identity"][*face]["pipe"].as_str().unwrap()) - s_joint(),
            ["separation"] => {
                let nodes = self.request["model"]["nodes"].as_array().unwrap();
                let (first, last) = (nodes.first().unwrap()["id"].as_str().unwrap(), nodes.last().unwrap()["id"].as_str().unwrap());
                let q = &self.oracle["chain_axis_global"];
                (0..3).map(|i| {
                    let a = ["x", "y", "z"][i];
                    (self.observed(source, &format!("node.{last}.translation.{a}")).unwrap()
                        - self.observed(source, &format!("node.{first}.translation.{a}")).unwrap()) * t4_u5b::number(&q[i])
                }).sum()
            }
            ["rhs", node, a] => {
                let assembly = &source["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"];
                let order = assembly["node_order"].as_array()?;
                let index = order.iter().position(|n| n == node)?;
                assembly["assembled_pressure_rhs_global"][6 * index + axis(a)].as_f64().unwrap()
            }
            _ => panic!("{}: unmapped observable {key}", self.name),
        })
    }
    fn rod_evidence<'s>(&self, source: &'s Value, rod: &str) -> &'s Value {
        let record = source["contract_evidence"]["connector"].as_array().unwrap().iter()
            .find(|r| r["component_id"] == self.connector()).unwrap();
        record["tie_rods"].as_array().unwrap().iter().find(|r| r["id"] == rod).unwrap()
    }
    /// Every frozen observable the product publishes, by key.
    fn keys(&self) -> Vec<String> {
        let c = self.oracle;
        let xyz = ["x", "y", "z"];
        let mut keys: Vec<String> = ["S_joint", "Tb", "Pe", "sum_rod_tension"].map(String::from).to_vec();
        for kind in ["q.translation", "q.rotation", "g.force", "g.moment"] {
            keys.extend(xyz.map(|a| format!("{kind}.{a}")));
        }
        for end in ["end_i", "end_j"] {
            for kind in ["force", "moment"] {
                keys.extend(xyz.map(|a| format!("connector_endpoint.{end}.{kind}.{a}")));
            }
        }
        for node in c["nodes"].as_object().unwrap().keys() {
            for kind in ["translation", "rotation"] {
                keys.extend(xyz.map(|a| format!("node.{node}.{kind}.{a}")));
            }
            keys.push(format!("node.{node}.axial_displacement"));
        }
        for pipe in c["pipes"].as_object().unwrap().keys() {
            keys.extend(["Nw", "S", "elongation"].map(|k| format!("pipe.{pipe}.{k}")));
            for end in ["end_i", "end_j"] {
                for kind in ["force", "moment", "force_local", "moment_local"] {
                    keys.extend(xyz.map(|a| format!("pipe.{pipe}.{end}.{kind}.{a}")));
                }
            }
        }
        for support in c["reactions_support_on_model"].as_object().unwrap().keys() {
            for kind in ["force", "moment"] {
                keys.extend(xyz.map(|a| format!("reaction.{support}.{kind}.{a}")));
            }
        }
        for rod in c["rods"].as_object().unwrap().keys() {
            keys.push(format!("rod.{rod}.tension"));
            keys.push(format!("rod.{rod}.axial_share"));
            keys.extend(xyz.map(|a| format!("rod.{rod}.direction_global.{a}")));
        }
        for face in ["face_i", "face_j"] {
            keys.extend(["S_pipe", "S_joint", "S_pipe_minus_S_joint"].map(|k| format!("identity.{face}.{k}")));
        }
        if c.get("outer_end_separation_change_m").is_some_and(|v| !v.is_null()) {
            keys.push("separation".into());
        }
        for node in c["pressure_rhs_global_n"].as_object().unwrap().keys() {
            keys.extend(xyz.map(|a| format!("rhs.{node}.{a}")));
        }
        keys
    }
}

thread_local! { static FAILURES: std::cell::RefCell<Vec<(String, String, f64)>> = const { std::cell::RefCell::new(Vec::new()) }; }

/// The worst |obs − exp| / (1e-9·max(|exp|, F)) per criterion family.
type Worst = BTreeMap<&'static str, (f64, String)>;

fn assert_case(source: &Value, case: &Case, label: &str, worst: &mut Worst) {
    assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {}", source["diagnostics"]);
    assert_eq!(source["producer"]["semantic_contract_id"], PRESSURE_ID, "{label}");
    assert!(source["diagnostics"].as_array().unwrap().iter().all(|d| d["severity"] != "blocking"), "{label}");
    let mut compared = 0;
    for key in case.keys() {
        let expected = case.expected(&key).unwrap_or_else(|| panic!("{label}: no oracle value for {key}"));
        let observed = case.observed(source, &key).unwrap_or_else(|| panic!("{label}: {key} not published"));
        let (family, floor) = floor(&key);
        let fraction = (observed - expected).abs() / (RELATIVE * expected.abs().max(floor));
        if !(observed.is_finite() && fraction <= 1.0) {
            FAILURES.with(|f| f.borrow_mut().push((case.name.to_string(), key.clone(),
                if observed.is_finite() { fraction } else { f64::INFINITY })));
            println!("t4_u5b over the criterion: {label}: {key}: observed={observed:.17e}, expected={expected:.17e} ({fraction:.3e} of the criterion)");
        }
        // The worst fraction per family.
        let entry = worst.entry(family).or_insert((0.0, String::new()));
        if fraction > entry.0 {
            *entry = (fraction, format!("{label} {key}"));
        }
        compared += 1;
    }
    assert!(compared > 50, "{label}: {compared} observables");
    // The rods' rows and evidence: exactly one row per authored rod, bound by row ID.
    let connector = case.connector();
    let rods = case.oracle["rods"].as_object().unwrap();
    let published: Vec<&Value> = source["results"].as_array().unwrap().iter().filter(|r| r["kind"] == "connector_tie_rod_force_v1").collect();
    assert_eq!(published.len(), rods.len(), "{label}: one force row per rod");
    for row in &published {
        assert_eq!(row["entity_ref"], connector);
        assert_eq!(row["unit"], "N");
        assert_eq!(row["metadata"]["component"], "t_r");
        assert_eq!(row["metadata"]["location"], "tie_rod");
        assert_eq!(row["metadata"]["coordinate_system"], "element_local");
        assert_eq!(row["metadata"]["basis"], "recovered_from_tie_rod_extension_v1");
    }
    let record = source["contract_evidence"]["connector"].as_array().unwrap().iter().find(|r| r["component_id"] == connector).unwrap();
    assert_eq!(record["hardware"], "explicit_axial_rods_v1", "{label}");
    assert_eq!(record["tie_rods"].as_array().unwrap().len(), rods.len(), "{label}");
    // pe_x only on a constant-effective-area connector; s_x with rods always.
    let constant = case.oracle["connector_pressure_model"] == "constant_effective_area_reference_v1";
    assert_eq!(rows(source, "connector_pressure_force_v1", connector, "pe_x", None).len(), usize::from(constant), "{label}: pe_x");
    assert_eq!(rows(source, "connector_effective_axial_force_v1", connector, "s_x", None).len(), 1, "{label}: s_x");
    // The replaced span publishes nothing.
    let replaced = case.request["model"]["components"][0]["objective_connector"]["topology"]["span_ref"].as_str().unwrap();
    assert!(source["results"].as_array().unwrap().iter().all(|r| r["entity_ref"] != replaced), "{label}");
}

fn cases() -> Vec<(String, Value, Value)> {
    let oracle = t4_u5b::oracle();
    let cases = oracle["cases"].as_object().unwrap();
    assert_eq!(cases.len(), 38);
    cases.iter().map(|(name, case)| (name.clone(), case.clone(), t4_u5b::request(name, case))).collect()
}

#[test]
fn u5b_thirty_eight_oracles_match_typed_captured_and_headless_consumers_with_export_in_both_modes() {
    let mut worst = Worst::new();
    for (name, oracle, request) in cases() {
        let case = Case { name: &name, oracle: &oracle, request: &request };
        for mode in MODES {
            let (sources, document) = run(&request, mode);
            for source in &sources {
                assert_case(source, &case, &format!("{name} {mode:?}"), &mut worst);
            }
            let document = document.unwrap_or_else(|| panic!("{name} {mode:?}: current pressure export missing"));
            // The export carries every rod row with the envelope's value.
            let exported = document.to_string();
            for rod in oracle["rods"].as_object().unwrap().keys() {
                assert!(exported.contains(&rod_row_id(case.connector(), rod)), "{name} {mode:?}: exported rod row {rod}");
            }
        }
    }
    for (family, (fraction, at)) in &worst {
        println!("t4_u5b worst fraction of criterion: {family} = {fraction:.3e} ({at})");
    }
    // No exception: every observable of all 38 cases, both modes, within the
    // unchanged criterion (HELP_HUMAN's ruling (A) removed the AS-F-R carve-out).
    let failures = FAILURES.with(|f| f.borrow().clone());
    assert!(failures.is_empty(), "over the criterion: {failures:?}");
}

/// Each discriminator's wrong value of a published observable lies outside
/// the criterion of the product's value (so the mutant would fail the
/// comparison above). Observables the product does not publish (rod
/// extensions and rod nodal actions) are counted, not scored; every mutant is
/// separated on at least one case.
#[test]
fn u5b_discriminator_wrong_values_are_separated_from_the_product_in_both_modes() {
    let oracle = t4_u5b::oracle();
    let built: BTreeMap<String, (Value, Value)> = cases().into_iter().map(|(n, c, r)| (n, (c, r))).collect();
    let mut runs: BTreeMap<(String, usize), Value> = BTreeMap::new();
    for (name, (_, request)) in &built {
        for (index, mode) in MODES.iter().enumerate() {
            let source = serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), *mode).unwrap()).unwrap();
            runs.insert((name.clone(), index), source);
        }
    }
    for (mutant, spec) in oracle["discriminators"].as_object().unwrap() {
        let mut separated_cases = BTreeSet::new();
        let mut smallest = f64::INFINITY;
        let mut unpublished = 0;
        for (name, entry) in spec["cases"].as_object().unwrap() {
            let (case_oracle, request) = &built[name];
            let case = Case { name, oracle: case_oracle, request };
            for index in 0..MODES.len() {
                let source = &runs[&(name.clone(), index)];
                for (key, moved) in entry["moved"].as_object().unwrap() {
                    let Some(observed) = case.observed(source, key) else {
                        unpublished += 1;
                        continue;
                    };
                    let expected = case.expected(key).unwrap();
                    let wrong = t4_u5b::number(&moved["wrong"]);
                    let (_, floor) = floor(key);
                    let distance = (observed - wrong).abs() / (RELATIVE * expected.abs().max(floor));
                    assert!(distance > 1.0, "{mutant} {name} {key}: product {observed:e} within the criterion of the wrong value {wrong:e}");
                    smallest = smallest.min(distance);
                    separated_cases.insert(name.clone());
                }
            }
        }
        assert!(!separated_cases.is_empty(), "{mutant}: separated on no case by published rows");
        println!("t4_u5b discriminator {mutant}: separated on {} case(s), smallest {smallest:.2e} tolerances, {unpublished} unpublished observable comparisons", separated_cases.len());
    }
}

/// The committed builder outputs (README "Binding to model 0.8.0"), kept beside
/// this test, outside the frozen reference tree (DEL-04-07 Design §1.4).
const COMMITTED: [(&str, &str); 39] = [
    ("AS-A.request.json", include_str!("t4_u5b_requests/AS-A.request.json")),
    ("AS-A-ER.request.json", include_str!("t4_u5b_requests/AS-A-ER.request.json")),
    ("AS-A-R.request.json", include_str!("t4_u5b_requests/AS-A-R.request.json")),
    ("AS-A-V.request.json", include_str!("t4_u5b_requests/AS-A-V.request.json")),
    ("AS-F.request.json", include_str!("t4_u5b_requests/AS-F.request.json")),
    ("AS-F-ER.request.json", include_str!("t4_u5b_requests/AS-F-ER.request.json")),
    ("AS-F-R.request.json", include_str!("t4_u5b_requests/AS-F-R.request.json")),
    ("AS-F-V.request.json", include_str!("t4_u5b_requests/AS-F-V.request.json")),
    ("ER-A.request.json", include_str!("t4_u5b_requests/ER-A.request.json")),
    ("ER-F.request.json", include_str!("t4_u5b_requests/ER-F.request.json")),
    ("L2-CL-2E11.request.json", include_str!("t4_u5b_requests/L2-CL-2E11.request.json")),
    ("L2-CL-2E3.request.json", include_str!("t4_u5b_requests/L2-CL-2E3.request.json")),
    ("L2-CL-2E5.request.json", include_str!("t4_u5b_requests/L2-CL-2E5.request.json")),
    ("L2-CL-2E7.request.json", include_str!("t4_u5b_requests/L2-CL-2E7.request.json")),
    ("L2-CL-2E9.request.json", include_str!("t4_u5b_requests/L2-CL-2E9.request.json")),
    ("L2-PR-2E11.request.json", include_str!("t4_u5b_requests/L2-PR-2E11.request.json")),
    ("L2-PR-2E3.request.json", include_str!("t4_u5b_requests/L2-PR-2E3.request.json")),
    ("L2-PR-2E5.request.json", include_str!("t4_u5b_requests/L2-PR-2E5.request.json")),
    ("L2-PR-2E7.request.json", include_str!("t4_u5b_requests/L2-PR-2E7.request.json")),
    ("L2-PR-2E9.request.json", include_str!("t4_u5b_requests/L2-PR-2E9.request.json")),
    ("L4-CL-2E11.request.json", include_str!("t4_u5b_requests/L4-CL-2E11.request.json")),
    ("L4-CL-2E3.request.json", include_str!("t4_u5b_requests/L4-CL-2E3.request.json")),
    ("L4-CL-2E5.request.json", include_str!("t4_u5b_requests/L4-CL-2E5.request.json")),
    ("L4-CL-2E7.request.json", include_str!("t4_u5b_requests/L4-CL-2E7.request.json")),
    ("L4-CL-2E9.request.json", include_str!("t4_u5b_requests/L4-CL-2E9.request.json")),
    ("L4-PR-2E11.request.json", include_str!("t4_u5b_requests/L4-PR-2E11.request.json")),
    ("L4-PR-2E3.request.json", include_str!("t4_u5b_requests/L4-PR-2E3.request.json")),
    ("L4-PR-2E5.request.json", include_str!("t4_u5b_requests/L4-PR-2E5.request.json")),
    ("L4-PR-2E7.request.json", include_str!("t4_u5b_requests/L4-PR-2E7.request.json")),
    ("L4-PR-2E9.request.json", include_str!("t4_u5b_requests/L4-PR-2E9.request.json")),
    ("MO-F.request.json", include_str!("t4_u5b_requests/MO-F.request.json")),
    ("MO-T.request.json", include_str!("t4_u5b_requests/MO-T.request.json")),
    ("OP-4.request.json", include_str!("t4_u5b_requests/OP-4.request.json")),
    ("OP-Y.request.json", include_str!("t4_u5b_requests/OP-Y.request.json")),
    ("OP-Z.request.json", include_str!("t4_u5b_requests/OP-Z.request.json")),
    ("RR-P.request.json", include_str!("t4_u5b_requests/RR-P.request.json")),
    ("RR-U.request.json", include_str!("t4_u5b_requests/RR-U.request.json")),
    ("SK-F.request.json", include_str!("t4_u5b_requests/SK-F.request.json")),
    ("bindings.json", include_str!("t4_u5b_requests/bindings.json")),
];
const CASE_NAMES: [&str; 38] = ["AS-A", "AS-A-ER", "AS-A-R", "AS-A-V", "AS-F", "AS-F-ER", "AS-F-R", "AS-F-V", "ER-A", "ER-F", "L2-CL-2E11", "L2-CL-2E3", "L2-CL-2E5", "L2-CL-2E7", "L2-CL-2E9", "L2-PR-2E11", "L2-PR-2E3", "L2-PR-2E5", "L2-PR-2E7", "L2-PR-2E9", "L4-CL-2E11", "L4-CL-2E3", "L4-CL-2E5", "L4-CL-2E7", "L4-CL-2E9", "L4-PR-2E11", "L4-PR-2E3", "L4-PR-2E5", "L4-PR-2E7", "L4-PR-2E9", "MO-F", "MO-T", "OP-4", "OP-Y", "OP-Z", "RR-P", "RR-U", "SK-F"];

/// The builder's request files and `bindings.json` (each request's SHA-256
/// beside its case's `case_sha256`), under `tests/t4_u5b_requests/`. The
/// frozen reference tree is not touched.
fn request_files() -> BTreeMap<String, Vec<u8>> {
    use sha2::{Digest, Sha256};
    let oracle = t4_u5b::oracle();
    let mut files = BTreeMap::new();
    let mut bindings = serde_json::Map::new();
    for name in CASE_NAMES {
        let case = &oracle["cases"][name];
        let path = format!("{name}.request.json");
        let bytes = t4_u5b::request_bytes(&t4_u5b::request(name, case));
        bindings.insert(name.to_string(), serde_json::json!({
            "case_description": case["case_description"], "case_sha256": case["case_sha256"],
            "request": path, "request_sha256": format!("{:x}", Sha256::digest(&bytes)),
        }));
        files.insert(path, bytes);
    }
    let record = serde_json::json!({
        "binding": "validation/references/t4_u5b README \"Binding to model 0.8.0\": one model 0.8.0 request per case, authored by the shared builder (core/product_physics/tests/common/t4_u5b.rs) from the SHA-bound case description and its t4_u5a base, kept beside the headless t4_u5b test outside the frozen reference tree; only request bytes and hashes are added, and no oracle value, case description or reference file moves",
        "cases": bindings,
    });
    files.insert("bindings.json".into(), t4_u5b::request_bytes(&record));
    files
}

#[test]
fn u5b_committed_requests_are_the_builders_bytes() {
    let files = request_files();
    assert_eq!(files.len(), COMMITTED.len());
    if std::env::var("T4_U5B_WRITE_REQUESTS").as_deref() == Ok("1") {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/t4_u5b_requests");
        for (path, bytes) in &files {
            std::fs::write(root.join(path), bytes).unwrap();
        }
        return;
    }
    for (path, committed) in COMMITTED {
        assert!(files[path] == committed.as_bytes(), "{path}: committed bytes differ from the builder's (T4_U5B_WRITE_REQUESTS=1)");
    }
}

/// C99 hexadecimal form of a binary64 (`float.fromhex` reads it exactly).
fn hex_float(v: f64) -> String {
    let bits = v.to_bits();
    let sign = if bits >> 63 == 1 { "-" } else { "" };
    let exponent = ((bits >> 52) & 0x7ff) as i64;
    let fraction = bits & ((1u64 << 52) - 1);
    match (exponent, fraction) {
        (0, 0) => format!("{sign}0x0p+0"),
        (0, f) => format!("{sign}0x0.{f:013x}p-1022"),
        (e, f) => format!("{sign}0x1.{f:013x}p{:+}", e - 1023),
    }
}

/// Evidence for an independent reproduction (not a check): with
/// `T4_U5B_DUMP_DIR` set, writes AS-F-R's and AS-F's request bytes and every
/// scored observation of both modes, with the expected value and criterion.
#[test]
fn u5b_writes_case_observations_when_asked() {
    use sha2::{Digest, Sha256};
    let Some(dir) = std::env::var_os("T4_U5B_DUMP_DIR") else { return };
    let dir = std::path::PathBuf::from(dir);
    let oracle = t4_u5b::oracle();
    for (name, file) in [("AS-F-R", "as_f_r_observations.json"), ("AS-F", "as_f_observations.json")] {
        let case_oracle = oracle["cases"][name].clone();
        let request = t4_u5b::request(name, &case_oracle);
        let bytes = t4_u5b::request_bytes(&request);
        let request_file = format!("{name}.request.json");
        std::fs::write(dir.join(&request_file), &bytes).unwrap();
        let case = Case { name, oracle: &case_oracle, request: &request };
        let mut modes = serde_json::Map::new();
        for mode in MODES {
            let (sources, _) = run(&request, mode);
            let source = &sources[0];
            let observables: Vec<Value> = case.keys().into_iter().map(|key| {
                let expected = case.expected(&key).unwrap();
                let observed = case.observed(source, &key).unwrap();
                let (family, floor) = floor(&key);
                let tolerance = RELATIVE * expected.abs().max(floor);
                serde_json::json!({
                    "key": key, "family": family,
                    "observed_hex": hex_float(observed), "observed_bits": format!("{:#018x}", observed.to_bits()),
                    "observed_decimal": format!("{observed:e}"),
                    "expected_hex": hex_float(expected), "expected_decimal": format!("{expected:e}"),
                    "criterion": {"relative": RELATIVE, "floor": floor, "tolerance": tolerance},
                    "fraction_of_criterion": (observed - expected).abs() / tolerance,
                })
            }).collect();
            modes.insert(format!("{mode:?}"), serde_json::json!({
                "mechanics": source["status"]["mechanics"], "semantic_contract_id": source["producer"]["semantic_contract_id"],
                "observables": observables,
            }));
        }
        let record = serde_json::json!({
            "case": name, "case_sha256": case_oracle["case_sha256"],
            "request_file": request_file, "request_sha256": format!("{:x}", Sha256::digest(&bytes)),
            "criterion": "|observed - expected| <= relative * max(|expected|, floor) (t4_u5b README Criterion)",
            "source": "captured envelope (typed and headless results asserted identical)",
            "modes": modes,
        });
        std::fs::write(dir.join(file), serde_json::to_string_pretty(&record).unwrap() + "\n").unwrap();
    }
}
