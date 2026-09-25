//! Independent public-route acceptance extension for the 0.4.0
//! load/reference-state route (review N-4, checkpoint 3).
//!
//! Authored by the CP2_RUNTIME_TESTS TASK, which is not the implementer, from
//! `CP2_WIRE.md`, `CP2_WIRE_ADDENDUM_1.md`, `CP2_WIRE_ADDENDUM_2.md` and the
//! reviewed design at Git `9e8a55d`. Every request goes through the public
//! `run_linear_static_preview_value_with_mode` in both solver modes. No
//! production helper is used as an oracle, and no observed output is used as
//! an expectation.
//!
//! Expected values come from one of two sources:
//! - `tests/fixtures/load_reference_states/reference_cases.json`, read at test
//!   time (the `value` projection);
//! - an elementary derivation stated beside the assertion, using fixture
//!   quantities or the declared inputs of the producer witness.
//!
//! All other numbers are invented analytical inputs and are marked as such.
//! None of them is library, component or code-rule data.
//!
//! Criterion: the protected relative `|actual - expected| <= 1e-9 * |expected|`.
//! For an exact-zero reference, the floor is `1e-9 * zero_scale`, where
//! `zero_scale` is the case magnitude named at each call site. No looser
//! tolerance is used.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode, ResultItem,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
/// Protected relative criterion; never loosened.
const RELATIVE: f64 = 1.0e-9;
const SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/load-reference-1";
const PROFILE: &str = "resolved_straight_load_state_v1";
const SOURCE_SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/load-reference-source-1";
const SOURCE_PROFILE: &str = "resolved_straight_load_state_source_v1";
const SOURCE_POLICY: &str = "LOAD-REFERENCE-SOURCE-1";
const SOURCE_METHOD: &str = "retained_source_blocks_exact_v1";
const STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
const REFERENCE: &str = "reference:installed";
const NOT_JOINED: &str = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED";
const CASE: &str = "case:reference";
const ALL: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const PROV: &str = "invented analytical test input; not library, component or code data";
const INVENTED_NU: f64 = 0.3;
/// Invented tip force for the temperature-identity cantilever.
const INVENTED_TIP_FORCE: f64 = 1000.0;
/// Invented internal differential pressure for the closed-region checks.
const INVENTED_PRESSURE_PA: f64 = 2.0e6;
/// Invented unused base pair for the shared material (E Pa, nu).
const INVENTED_BASE: (f64, f64) = (170.0e9, 0.28);
/// Exact unit definition (degC to K); used only to label fixture K values.
const KELVIN_OFFSET: f64 = 273.15;
const EIGEN_MOTION_REQUEST: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/product_preview/load_reference_source/eigen_motion.request.json"
);
const EIGEN_MOTION_SHA256: &str =
    "58ce5b95d185d56964a30962b355c14595cdf831505603c6e5d455425c4ea8e9";

// ---------------------------------------------------------------------------
// Maintained analytical reference access
// ---------------------------------------------------------------------------

fn fixture() -> &'static Value {
    static FIXTURE: OnceLock<Value> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/load_reference_states/reference_cases.json"
        );
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
        serde_json::from_str(&text).expect("reference_cases.json parses")
    })
}

fn at(path: &str) -> &'static Value {
    let mut node = fixture();
    for key in path.split('/') {
        node = match node {
            Value::Array(items) => {
                let index: usize = key
                    .parse()
                    .unwrap_or_else(|_| panic!("fixture path {path}: {key} is not an index"));
                items
                    .get(index)
                    .unwrap_or_else(|| panic!("fixture path {path}: index {key} missing"))
            }
            Value::Object(map) => map
                .get(key)
                .unwrap_or_else(|| panic!("fixture path {path}: key {key} missing")),
            _ => panic!("fixture path {path}: cannot descend into {key}"),
        };
    }
    node
}

fn qval(quantity: &Value, unit: &str, context: &str) -> f64 {
    assert_eq!(
        quantity["unit"].as_str(),
        Some(unit),
        "unit at {context}: {quantity}"
    );
    quantity["value"]
        .as_f64()
        .unwrap_or_else(|| panic!("value at {context}: {quantity}"))
}

fn q(path: &str, unit: &str) -> f64 {
    qval(at(path), unit, path)
}

fn text(path: &str) -> &'static str {
    at(path)
        .as_str()
        .unwrap_or_else(|| panic!("fixture text at {path}"))
}

fn ann(case: &str, rest: &str) -> String {
    format!("cases/{case}/variants/annular_companion/{rest}")
}

/// An authored fixture temperature as `(value, unit)`.
fn authored(quantity: &Value) -> (f64, String) {
    (
        quantity["value"].as_f64().expect("authored value"),
        quantity["unit"]
            .as_str()
            .expect("authored unit")
            .to_string(),
    )
}

struct Annulus {
    od: f64,
    wall: f64,
    area: f64,
    bore: f64,
    polar: f64,
}

fn annulus() -> Annulus {
    Annulus {
        od: q("geometry/authored/outside_diameter", "m"),
        wall: q("geometry/authored/wall_thickness", "m"),
        area: q("geometry/derived/As", "m^2"),
        bore: q("geometry/derived/Ai", "m^2"),
        polar: q("geometry/derived/J", "m^4"),
    }
}

// Shared-material pairs from the maintained serial companion (inputs only).
const SERIAL: &str = "shared_material_serial_companion";

fn pair_cold() -> (f64, f64) {
    (
        q(&ann(SERIAL, "inputs/E1"), "Pa"),
        q(&ann(SERIAL, "inputs/nu1"), "1"),
    )
}

fn pair_hot() -> (f64, f64) {
    (
        q(&ann(SERIAL, "inputs/E2"), "Pa"),
        q(&ann(SERIAL, "inputs/nu2"), "1"),
    )
}

// ---------------------------------------------------------------------------
// Numerical checks
// ---------------------------------------------------------------------------

fn close(actual: f64, expected: f64, zero_scale: f64, context: &str) {
    assert!(actual.is_finite(), "{context}: nonfinite {actual}");
    let tolerance = RELATIVE
        * if expected == 0.0 {
            zero_scale.abs()
        } else {
            expected.abs()
        };
    assert!(
        (actual - expected).abs() <= tolerance,
        "{context}: actual={actual:.17e}; expected={expected:.17e}; tolerance={tolerance:.17e}"
    );
}

fn differs(actual: f64, expected: f64, wrong: f64, zero_scale: f64, context: &str) {
    let threshold = RELATIVE
        * if wrong == 0.0 {
            zero_scale.abs()
        } else {
            wrong.abs()
        };
    assert!(
        (expected - wrong).abs() > threshold,
        "{context}: discriminator {wrong:.17e} is not distinct from the reference {expected:.17e}"
    );
    assert!(
        (actual - wrong).abs() > threshold,
        "{context}: observation {actual:.17e} reproduces the wrong result {wrong:.17e}"
    );
}

fn shear_modulus(e: f64, nu: f64) -> f64 {
    e / (2.0 * (1.0 + nu))
}

// ---------------------------------------------------------------------------
// 0.4.0 request builder (CP2_WIRE + addenda)
// ---------------------------------------------------------------------------

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

fn basis_temperature(value: f64, unit: &str) -> Value {
    json!({"kind": "temperature_reference", "installation_temperature": qv(value, unit)})
}

fn basis_direct() -> Value {
    json!({"kind": "direct_strain_reference"})
}

fn fit_none() -> Value {
    json!({"kind": "none"})
}

fn sel_base(material: &str) -> Value {
    json!({"kind": "explicit_base_properties", "material_ref": material,
        "applicability_reference": "invented analytical basis declared applicable to this member"})
}

fn sel_point(material: &str, point: &str) -> Value {
    json!({"kind": "exact_point", "material_ref": material, "point_ref": point})
}

fn sel_interp(material: &str, value: f64, unit: &str) -> Value {
    json!({"kind": "temperature_interpolation", "material_ref": material,
        "temperature": qv(value, unit), "interpolation": "piecewise_linear",
        "extrapolation": "forbidden"})
}

fn th_unchanged() -> Value {
    json!({"kind": "unchanged_reference", "provenance": PROV})
}

fn th_interval(strain: f64) -> Value {
    json!({"kind": "explicit_interval_strain", "strain": qv(strain, "1"),
        "interval_reference": "invented reference-to-state interval", "provenance": PROV})
}

fn th_free(law: &str) -> Value {
    json!({"kind": "free_length_state", "expansion_law_ref": law})
}

fn material(id: &str, e: f64, nu: f64) -> Value {
    json!({"id": id, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": qv(e, "Pa"), "poisson_ratio": qv(nu, "1"), "provenance": PROV})
}

fn point(id: &str, temperature: (f64, &str), pair: (f64, f64)) -> Value {
    json!({"id": id, "temperature": qv(temperature.0, temperature.1),
        "elastic_modulus": qv(pair.0, "Pa"), "poisson_ratio": qv(pair.1, "1"),
        "provenance": PROV})
}

fn nodal_force(id: &str, node: &str, value: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node},
        "direction": "global_x", "magnitude": qv(value, "N"), "dimension": "force",
        "provenance": PROV})
}

fn closed_region(id: &str, pipe: &str, from: &str, to: &str, pressure_pa: f64) -> Value {
    json!({"id": id, "member_pipe_ids": [pipe],
        "pressure_basis": "internal_differential_zero_external_v1",
        "pressure": qv(pressure_pa, "Pa"),
        "terminals": [
            {"node_ref": from, "closure_transfer": "transfers_to_wall", "provenance": PROV},
            {"node_ref": to, "closure_transfer": "transfers_to_wall", "provenance": PROV}],
        "provenance": PROV})
}

struct Model {
    id: String,
    nodes: Vec<Value>,
    pipes: Vec<Value>,
    members: Vec<Value>,
    materials: Vec<Value>,
    supports: Vec<Value>,
    cases: Vec<Value>,
}

impl Model {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            nodes: Vec::new(),
            pipes: Vec::new(),
            members: Vec::new(),
            materials: Vec::new(),
            supports: Vec::new(),
            cases: Vec::new(),
        }
    }

    fn node(mut self, id: &str, x: f64) -> Self {
        self.nodes
            .push(json!({"id": id, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV}));
        self
    }

    fn pipe(
        mut self,
        id: &str,
        from: &str,
        to: &str,
        material: &str,
        basis: Value,
        fit: Value,
    ) -> Self {
        let a = annulus();
        self.pipes.push(json!({"id": id, "from": from, "to": to,
            "section": {"outside_diameter": qv(a.od, "m"), "wall_thickness": qv(a.wall, "m")},
            "material": material, "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}));
        self.members
            .push(json!({"pipe_ref": id, "basis": basis, "fit": fit, "provenance": PROV}));
        self
    }

    fn material(mut self, material: Value) -> Self {
        self.materials.push(material);
        self
    }

    fn support(mut self, id: &str, node: &str, restraints: &[&str]) -> Self {
        self.supports
            .push(json!({"id": id, "node": node, "family": "anchor",
            "restraints": restraints, "provenance": PROV}));
        self
    }

    fn case(mut self, case: Case) -> Self {
        self.cases.push(case.to_json());
        self
    }

    fn request(&self) -> Value {
        json!({"model": {
            "schema_version": "0.4.0",
            "document_kind": "openpipestress.product_preview.model",
            "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
            "project": {"id": self.id, "units": {"length": "m", "force": "N", "angle": "rad",
                "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                "rule_check": "not_performed_user_rule_inputs_missing",
                "professional_acceptance": "not_provided"},
            "nodes": self.nodes,
            "pipe_segments": self.pipes,
            "supports": self.supports,
            "components": [],
            "materials": self.materials,
            "reference_configurations": [{
                "id": REFERENCE, "label": "Installed reference",
                "geometry_ref": {"kind": "authored_model_geometry"},
                "member_references": self.members, "provenance": PROV}],
            "load_cases": self.cases,
            "combinations": []},
        "materials": []})
    }
}

struct Case {
    id: String,
    elements: Vec<Value>,
    supports: Vec<Value>,
    primitives: Vec<Value>,
    sources: Vec<Value>,
    regions: Vec<Value>,
}

impl Case {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            elements: Vec::new(),
            supports: Vec::new(),
            primitives: Vec::new(),
            sources: Vec::new(),
            regions: Vec::new(),
        }
    }

    fn element(
        mut self,
        pipe: &str,
        operating: Option<(f64, &str)>,
        selection: Value,
        thermal: Value,
    ) -> Self {
        let mut state =
            json!({"pipe_ref": pipe, "material_selection": selection, "thermal_state": thermal});
        if let Some((value, unit)) = operating {
            state["operating_temperature"] = qv(value, unit);
        }
        self.elements.push(state);
        self
    }

    fn support(mut self, support: &str) -> Self {
        self.supports.push(
            json!({"support_ref": support, "participation": {"kind": "active_model_device"}}),
        );
        self
    }

    fn load(mut self, primitive: Value) -> Self {
        let id = primitive["id"].as_str().expect("primitive id").to_string();
        self.primitives.push(primitive);
        self.sources.push(json!({"source_ref": id, "factor": 1.0}));
        self
    }

    fn region(mut self, region: Value) -> Self {
        self.regions.push(region);
        self
    }

    fn to_json(&self) -> Value {
        json!({"id": self.id, "pressure_regions": self.regions,
            "primitive_loads": self.primitives,
            "analysis_state": {
                "contract": STATE_CONTRACT,
                "reference_configuration_ref": REFERENCE,
                "element_states": self.elements,
                "support_states": self.supports,
                "load_sources": self.sources,
                "history": {"kind": "independent_equilibrium"},
                "provenance": PROV},
            "provenance": PROV})
    }
}

// ---------------------------------------------------------------------------
// Public-boundary execution and envelope contracts
// ---------------------------------------------------------------------------

fn run(request: &Value, mode: PreviewSolverMode) -> Result<MechanicsEnvelope, String> {
    run_linear_static_preview_value_with_mode(request.clone(), mode)
}

fn case_ids(request: &Value) -> Vec<String> {
    request["model"]["load_cases"]
        .as_array()
        .expect("load_cases")
        .iter()
        .map(|c| c["id"].as_str().expect("case id").to_string())
        .collect()
}

fn blocking_codes(out: &MechanicsEnvelope) -> Vec<String> {
    out.diagnostics
        .iter()
        .filter(|d| d.severity == "blocking" || d.severity == "failure")
        .map(|d| d.code.clone())
        .collect()
}

fn blocking_details(out: &MechanicsEnvelope) -> Vec<String> {
    out.diagnostics
        .iter()
        .filter(|d| d.severity == "blocking" || d.severity == "failure")
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect()
}

fn contract<'a>(out: &'a MechanicsEnvelope, key: &str) -> &'a Vec<Value> {
    out.contract_evidence
        .as_ref()
        .expect("contract_evidence is published")[key]
        .as_array()
        .unwrap_or_else(|| panic!("contract_evidence.{key}[] is published"))
}

fn by_case<'a>(out: &'a MechanicsEnvelope, key: &str, case: &str) -> &'a Value {
    let matching: Vec<_> = contract(out, key)
        .iter()
        .filter(|r| r["load_case_id"] == case)
        .collect();
    assert_eq!(matching.len(), 1, "exactly one {key} record for {case}");
    matching[0]
}

fn evidence<'a>(out: &'a MechanicsEnvelope, case: &str) -> &'a Value {
    by_case(out, "load_reference_states", case)
}

fn by_pipe<'a>(records: &'a Value, pipe: &str, context: &str) -> &'a Value {
    let matching: Vec<_> = records
        .as_array()
        .unwrap_or_else(|| panic!("{context}: array expected, got {records}"))
        .iter()
        .filter(|m| m["pipe_id"] == pipe)
        .collect();
    assert_eq!(
        matching.len(),
        1,
        "{context}: exactly one record for {pipe}"
    );
    matching[0]
}

fn num(value: &Value, context: &str) -> f64 {
    value
        .as_f64()
        .or_else(|| value["value"].as_f64())
        .unwrap_or_else(|| panic!("{context}: expected a number, got {value}"))
}

fn indices(value: &Value, context: &str) -> Vec<u64> {
    let mut out: Vec<u64> = value
        .as_array()
        .unwrap_or_else(|| panic!("{context}: index array expected, got {value}"))
        .iter()
        .map(|v| v.as_u64().unwrap_or_else(|| panic!("{context}: index {v}")))
        .collect();
    out.sort_unstable();
    out
}

fn common_envelope(out: &MechanicsEnvelope, label: &str) {
    assert_eq!(
        out.status.mechanics, "MECHANICS_SOLVED",
        "[{label}] supported reference did not solve; diagnostics={:#?}",
        out.diagnostics
    );
    assert!(
        blocking_codes(out).is_empty(),
        "[{label}] solved reference carries blocking diagnostics: {:#?}",
        out.diagnostics
    );
    assert!(!out.accepted_model_state_mutated);
    assert!(out.results.iter().all(|r| r.value.is_finite()));
}

/// Solve an ordinary-route (load-reference-1) reference.
fn solve(request: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let label = mode.as_str();
    let out = run(request, mode).unwrap_or_else(|error| {
        panic!("[{label}] valid 0.4.0 request rejected at the typed boundary: {error}")
    });
    common_envelope(&out, label);
    assert_eq!(
        out.producer.semantic_contract_id, SEMANTICS,
        "[{label}] producer semantics"
    );
    assert_eq!(
        out.formulation_basis.profile_id, PROFILE,
        "[{label}] formulation profile"
    );
    assert!(
        out.source_block_recovery.is_none(),
        "[{label}] ordinary route must not publish a retained-source receipt"
    );
    let ids = case_ids(request);
    assert_eq!(contract(&out, "load_reference_states").len(), ids.len());
    for id in &ids {
        let record = evidence(&out, id);
        assert_eq!(
            record["source_recovery"]["status"], "not_joined",
            "[{label}] {id}"
        );
        assert_eq!(
            record["source_recovery"]["code"], NOT_JOINED,
            "[{label}] {id}"
        );
        let carried: Vec<_> = out
            .diagnostics
            .iter()
            .filter(|d| d.code == NOT_JOINED && d.affected_refs.iter().any(|r| r == id))
            .collect();
        assert_eq!(carried.len(), 1, "[{label}] one {NOT_JOINED} for {id}");
        assert_eq!(carried[0].severity, "info");
    }
    out
}

/// Expect every case of the request to block after parse, with the code.
fn blocked_with(request: &Value, control: &str, codes: &[&str]) {
    let ids = case_ids(request);
    for mode in MODES {
        let label = mode.as_str();
        let out = run(request, mode).unwrap_or_else(|error| {
            panic!("[{control}/{label}] expected a blocking diagnostic; typed boundary returned Err: {error}")
        });
        let observed = blocking_codes(&out);
        println!(
            "OBSERVED_BLOCK control={control} mode={label} status={} codes={observed:?} details={:?}",
            out.status.mechanics,
            blocking_details(&out)
        );
        assert_ne!(
            out.status.mechanics, "MECHANICS_SOLVED",
            "[{control}/{label}] accepted"
        );
        assert!(
            observed.iter().any(|c| codes.contains(&c.as_str())),
            "[{control}/{label}] expected one of {codes:?}; diagnostics={:#?}",
            out.diagnostics
        );
        assert!(
            !out.results.iter().any(|r| r
                .basis_ref
                .as_ref()
                .is_some_and(|b| ids.iter().any(|id| id == &b.ref_id))),
            "[{control}/{label}] blocked case published result rows"
        );
        assert!(
            out.results.is_empty(),
            "[{control}/{label}] results published"
        );
    }
}

// ---------------------------------------------------------------------------
// Result rows
// ---------------------------------------------------------------------------

fn row<'a>(
    out: &'a MechanicsEnvelope,
    case: &str,
    kind: &str,
    entity: &str,
    component: &str,
    location: Option<&str>,
) -> &'a ResultItem {
    let matching: Vec<_> = out
        .results
        .iter()
        .filter(|r| {
            r.kind == kind
                && r.entity_ref == entity
                && r.basis_ref
                    .as_ref()
                    .is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case)
                && r.metadata.as_ref().is_some_and(|m| {
                    m.component == component && location.is_none_or(|l| m.location == l)
                })
        })
        .collect();
    assert_eq!(
        matching.len(),
        1,
        "expected one row case={case} kind={kind} entity={entity} component={component} location={location:?}"
    );
    matching[0]
}

fn displacement(out: &MechanicsEnvelope, case: &str, node: &str, axis: &str) -> f64 {
    let r = row(
        out,
        case,
        &format!("global_nodal_displacement_{axis}"),
        node,
        &format!("nodal_displacement_{axis}"),
        None,
    );
    assert_eq!(r.unit, "mm");
    r.value / 1000.0
}

fn rotation(out: &MechanicsEnvelope, case: &str, node: &str, axis: &str) -> f64 {
    let r = row(
        out,
        case,
        &format!("global_nodal_rotation_{axis}"),
        node,
        &format!("nodal_rotation_{axis}"),
        None,
    );
    assert_eq!(r.unit, "rad");
    r.value
}

fn reaction(out: &MechanicsEnvelope, case: &str, support: &str, component: &str) -> f64 {
    let r = row(
        out,
        case,
        "support_reaction_component_v2",
        support,
        component,
        Some("node"),
    );
    assert_eq!(
        r.unit,
        if component.starts_with('F') {
            "N"
        } else {
            "N*m"
        }
    );
    assert_eq!(r.metadata.as_ref().unwrap().coordinate_system, "global");
    r.value
}

fn member_axial(
    out: &MechanicsEnvelope,
    case: &str,
    pipe: &str,
    expected: f64,
    zero_scale: f64,
    context: &str,
) {
    for location in ["quarter_1", "midspan", "quarter_3"] {
        let r = row(
            out,
            case,
            "element_local_axial_force",
            pipe,
            "axial_force",
            Some(location),
        );
        close(
            r.value,
            expected,
            zero_scale,
            &format!("{context}: {pipe} station {location} N"),
        );
    }
    for (location, sign) in [("end_i", -1.0), ("end_j", 1.0)] {
        let r = row(
            out,
            case,
            "element_local_axial_force",
            pipe,
            "axial_force",
            Some(location),
        );
        close(
            r.value,
            sign * expected,
            zero_scale,
            &format!("{context}: {pipe} end force {location}"),
        );
    }
}

/// Exact-profile pressure rows: wall force, effective force and endpoint action.
fn pressure_rows(
    out: &MechanicsEnvelope,
    case: &str,
    pipe: &str,
    wall: f64,
    cap: f64,
    scale: f64,
    context: &str,
) {
    for location in ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"] {
        let r = row(
            out,
            case,
            "pipe_wall_axial_force_v2",
            pipe,
            "wall_axial_force",
            Some(location),
        );
        close(
            r.value,
            wall,
            scale,
            &format!("{context}: wall N {location}"),
        );
        let r = row(
            out,
            case,
            "pipe_effective_axial_force_v2",
            pipe,
            "effective_axial_force",
            Some(location),
        );
        close(
            r.value,
            wall - cap,
            scale,
            &format!("{context}: effective S {location}"),
        );
    }
    for (location, sign) in [("end_i", -1.0), ("end_j", 1.0)] {
        let r = row(
            out,
            case,
            "pipe_wall_endpoint_action_v2",
            pipe,
            "wall_axial_end_action",
            Some(location),
        );
        close(
            r.value,
            sign * wall,
            scale,
            &format!("{context}: wall end action {location}"),
        );
    }
}

// ---------------------------------------------------------------------------
// 1. Nonzero pressure with per-member E/nu (M06, M19, M23)
// ---------------------------------------------------------------------------

fn pressure_model(fixed: bool) -> Value {
    let (cold, hot) = (pair_cold(), pair_hot());
    let strain = q(&ann(SERIAL, "inputs/epsilon2"), "1");
    let mut shared = material("material:shared", INVENTED_BASE.0, INVENTED_BASE.1);
    // Invented point temperatures; the pairs are the fixture's serial companion inputs.
    shared["temperature_points"] = json!([
        point("point:a", (20.0, "degC"), cold),
        point("point:b", (300.0, "degC"), hot)
    ]);
    let mut model = Model::new("project:pressure-per-member")
        .node("node:root", 0.0)
        .node("node:far", 1.0)
        .material(shared)
        .pipe(
            "pipe:p",
            "node:root",
            "node:far",
            "material:shared",
            basis_direct(),
            fit_none(),
        )
        .support("support:root", "node:root", &ALL);
    if fixed {
        model = model.support("support:far", "node:far", &ALL);
    }
    for (case_id, point_ref, thermal) in [
        ("case:a", "point:a", th_unchanged()),
        ("case:b", "point:b", th_interval(strain)),
    ] {
        let mut case = Case::new(case_id)
            .element(
                "pipe:p",
                None,
                sel_point("material:shared", point_ref),
                thermal,
            )
            .support("support:root")
            .region(closed_region(
                &format!("region:{case_id}"),
                "pipe:p",
                "node:root",
                "node:far",
                INVENTED_PRESSURE_PA,
            ));
        if fixed {
            case = case.support("support:far");
        }
        model = model.case(case);
    }
    model.request()
}

#[test]
fn pressure_two_case_per_member_pairs_drive_wall_action_and_evidence() {
    // Wall-action relation, derived here from DESIGN.md section 4 (cold spring
    // and installation), "Nw = E(T) A [(u_j-u_i)/L - epsilon_star] +
    // pressure_Poisson_term", with the straight-annulus Lame state. For an
    // internal pressure p and zero external pressure, sigma_r + sigma_theta =
    // 2 p ri^2/(ro^2 - ri^2) through the wall. Then eps_z = (Nw/As -
    // nu(sigma_r + sigma_theta))/E + eps*, so the Poisson term is
    // 2 nu p Ai (Ai = pi ri^2).
    //
    // With both closures transferring to the wall, cap = p*Ai, and nodal
    // equilibrium gives the support-on-pipe actions root Fx = cap - Nw and
    // far Fx = Nw - cap.
    // - Fixed ends: u_j = u_i, so Nw = -E As eps* + 2 nu cap.
    // - Free tip: Nw = cap, and the tip UX = L [eps* + (1 - 2 nu) cap / (E As)].
    //
    // (E, nu) is each case's own resolved pair, never the base pair and never
    // the other case's pair.
    let a = annulus();
    let cap = INVENTED_PRESSURE_PA * a.bore;
    let strain = q(&ann(SERIAL, "inputs/epsilon2"), "1");
    let states = [("case:a", pair_cold(), 0.0), ("case:b", pair_hot(), strain)];
    let (base_e, base_nu) = INVENTED_BASE;
    for mode in MODES {
        for fixed in [true, false] {
            let out = solve(&pressure_model(fixed), mode);
            let ctx = format!("{} fixed={fixed}", mode.as_str());
            for (case, (e, nu), eps) in states {
                let sctx = format!("{ctx} {case}");
                let (wall, wrong_base, wrong_other) = if fixed {
                    let other = if case == "case:a" {
                        pair_hot()
                    } else {
                        pair_cold()
                    };
                    (
                        -e * a.area * eps + 2.0 * nu * cap,
                        -base_e * a.area * eps + 2.0 * base_nu * cap,
                        -other.0 * a.area * eps + 2.0 * other.1 * cap,
                    )
                } else {
                    (cap, f64::NAN, f64::NAN)
                };
                let scale = wall.abs().max(cap);
                pressure_rows(&out, case, "pipe:p", wall, cap, scale, &sctx);
                close(
                    reaction(&out, case, "support:root", "Fx"),
                    cap - wall,
                    scale,
                    &format!("{sctx}: root Fx"),
                );
                let observed = row(
                    &out,
                    case,
                    "pipe_wall_axial_force_v2",
                    "pipe:p",
                    "wall_axial_force",
                    Some("midspan"),
                )
                .value;
                if fixed {
                    close(
                        reaction(&out, case, "support:far", "Fx"),
                        wall - cap,
                        scale,
                        &format!("{sctx}: far Fx"),
                    );
                    close(
                        displacement(&out, case, "node:far", "x"),
                        0.0,
                        cap / (e * a.area),
                        &format!("{sctx}: far UX"),
                    );
                    differs(
                        observed,
                        wall,
                        wrong_base,
                        scale,
                        &format!("{sctx}: wall vs base pair (M06)"),
                    );
                    differs(
                        observed,
                        wall,
                        wrong_other,
                        scale,
                        &format!("{sctx}: wall vs other case pair"),
                    );
                } else {
                    let tip = eps + (1.0 - 2.0 * nu) * cap / (e * a.area);
                    let wrong_tip = eps + (1.0 - 2.0 * base_nu) * cap / (e * a.area);
                    let observed_tip = displacement(&out, case, "node:far", "x");
                    close(observed_tip, tip, tip, &format!("{sctx}: tip UX"));
                    differs(
                        observed_tip,
                        tip,
                        wrong_tip,
                        tip,
                        &format!("{sctx}: tip vs base Poisson (M06)"),
                    );
                }
                // Pressure evidence (M23) and exact-case evidence (M19): the case's own pair.
                let region = by_case(&out, "pressure", case);
                assert_eq!(region["region_id"], format!("region:{case}"));
                let pm = by_pipe(&region["materials"], "pipe:p", "pressure[].materials");
                let xm = by_pipe(
                    &by_case(&out, "exact_cases", case)["pipe_materials"],
                    "pipe:p",
                    "exact_cases.pipe_materials",
                );
                for (label, m) in [
                    ("pressure[].materials", pm),
                    ("exact_cases.pipe_materials", xm),
                ] {
                    close(
                        num(&m["E_pa"], label),
                        e,
                        0.0,
                        &format!("{sctx}: {label} E"),
                    );
                    close(
                        num(&m["nu"], label),
                        nu,
                        0.0,
                        &format!("{sctx}: {label} nu"),
                    );
                    close(
                        num(&m["G_pa"], label),
                        shear_modulus(e, nu),
                        0.0,
                        &format!("{sctx}: {label} G"),
                    );
                    differs(
                        num(&m["E_pa"], label),
                        e,
                        base_e,
                        0.0,
                        &format!("{sctx}: {label} E vs base"),
                    );
                    differs(
                        num(&m["nu"], label),
                        nu,
                        base_nu,
                        0.0,
                        &format!("{sctx}: {label} nu vs base"),
                    );
                }
                assert_eq!(xm["material_selection_kind"], "exact_point", "{sctx}");
                close(
                    num(&xm["resolved_eigenstrain"], "eigen"),
                    eps,
                    strain,
                    &format!("{sctx}: exact_cases eigenstrain"),
                );
                assert_eq!(
                    by_case(&out, "exact_cases", case)["material_basis"],
                    "resolved_per_member_load_reference_state_v1"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 2-6. Temperature identity, snapping, order and duplicate classes
// ---------------------------------------------------------------------------

const IDENTITY: &str = "cases/temperature_unit_identity/variants/exact_affine_identity";
/// Invented warm table point for the identity tables (above every group).
const WARM_K: f64 = 600.0;

/// Root-anchored cantilever with an invented tip force; one material with the
/// given points; unchanged thermal state on a direct-strain reference.
fn cantilever(points: Vec<Value>, selection: Value, operating: Option<(f64, &str)>) -> Value {
    let mut m = material("material:m", INVENTED_BASE.0, INVENTED_BASE.1);
    m["temperature_points"] = Value::Array(points);
    Model::new("project:temperature-identity")
        .node("node:root", 0.0)
        .node("node:tip", 1.0)
        .material(m)
        .pipe(
            "pipe:c",
            "node:root",
            "node:tip",
            "material:m",
            basis_direct(),
            fit_none(),
        )
        .support("support:root", "node:root", &ALL)
        .case(
            Case::new(CASE)
                .element("pipe:c", operating, selection, th_unchanged())
                .support("support:root")
                .load(nodal_force("source:tip", "node:tip", INVENTED_TIP_FORCE)),
        )
        .request()
}

fn consumed_point_ids(member: &Value) -> Vec<String> {
    member["consumed_material_points"]
        .as_array()
        .expect("consumed_material_points[]")
        .iter()
        .map(|p| p["point_id"].as_str().expect("point_id").to_string())
        .collect()
}

#[test]
fn temperature_identity_groups_select_same_point_across_units() {
    let a = annulus();
    let (cold, hot) = (pair_cold(), pair_hot());
    // Elementary axial cantilever: tip UX = F L / (E As), with E the cold point's E.
    let expected_tip = INVENTED_TIP_FORCE * 1.0 / (cold.0 * a.area);
    let groups = at(&format!("{IDENTITY}/inputs/identity_groups"))
        .as_array()
        .unwrap();
    let expected = at(&format!("{IDENTITY}/expected/identity_groups"))
        .as_array()
        .unwrap();
    for mode in MODES {
        let mut first_tip: Option<f64> = None;
        for (group, want) in groups.iter().zip(expected) {
            assert_eq!(group["id"], want["id"]);
            assert_eq!(want["compare_equal"], true);
            let kelvin = qval(&want["kelvin"], "K", "group kelvin");
            let members: Vec<(f64, String)> = group["authored"]
                .as_array()
                .unwrap()
                .iter()
                .map(authored)
                .collect();
            for (i, (point_t, point_unit)) in members.iter().enumerate() {
                for (j, (request_t, request_unit)) in members.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    let points = || {
                        vec![
                            point("point:cold", (*point_t, point_unit.as_str()), cold),
                            point("point:warm", (WARM_K, "K"), hot),
                        ]
                    };
                    for route in ["exact_point", "interpolation"] {
                        let request = if route == "exact_point" {
                            // Actual operating T authored in another unit; equal
                            // classes need no analysis-basis override.
                            cantilever(
                                points(),
                                sel_point("material:m", "point:cold"),
                                Some((*request_t, request_unit.as_str())),
                            )
                        } else {
                            // Interpolation to an exact endpoint of the same class.
                            cantilever(
                                points(),
                                sel_interp("material:m", *request_t, request_unit),
                                None,
                            )
                        };
                        let ctx = format!(
                            "{} {} point={point_t} {point_unit} request={request_t} {request_unit} {route}",
                            mode.as_str(),
                            group["id"]
                        );
                        let out = run(&request, mode).unwrap_or_else(|e| panic!("{ctx}: Err {e}"));
                        assert!(
                            blocking_codes(&out).is_empty(),
                            "{ctx}: equal classes refused: {:?}",
                            blocking_details(&out)
                        );
                        let out = solve(&request, mode);
                        let member = by_pipe(&evidence(&out, CASE)["members"], "pipe:c", "members");
                        assert_eq!(
                            consumed_point_ids(member),
                            vec!["point:cold".to_string()],
                            "{ctx}: consumed points"
                        );
                        assert_eq!(
                            num(&member["selected_E_pa"], "E").to_bits(),
                            cold.0.to_bits(),
                            "{ctx}: selected E"
                        );
                        assert_eq!(
                            num(&member["selected_nu"], "nu").to_bits(),
                            cold.1.to_bits(),
                            "{ctx}: selected nu"
                        );
                        let temperature_key = if route == "exact_point" {
                            assert!(
                                member["analysis_basis_override"].is_null(),
                                "{ctx}: no override"
                            );
                            "operating_temperature_k"
                        } else {
                            "material_selection_temperature_k"
                        };
                        close(
                            num(&member[temperature_key], temperature_key),
                            kelvin,
                            0.0,
                            &format!("{ctx}: {temperature_key}"),
                        );
                        let tip = displacement(&out, CASE, "node:tip", "x");
                        close(tip, expected_tip, expected_tip, &format!("{ctx}: tip UX"));
                        // Identical normalized mechanics across units, groups and routes.
                        match first_tip {
                            None => first_tip = Some(tip),
                            Some(first) => assert_eq!(
                                tip.to_bits(),
                                first.to_bits(),
                                "{ctx}: identical mechanics"
                            ),
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn temperature_identity_non_equal_control_is_not_snapped() {
    let control = at(&format!("{IDENTITY}/inputs/non_equal_control/authored"))
        .as_array()
        .unwrap();
    assert_eq!(
        at(&format!(
            "{IDENTITY}/expected/non_equal_control/compare_equal"
        )),
        false
    );
    let (near_t, near_unit) = authored(&control[0]); // -49.999999 degC
    let (end_t, end_unit) = authored(&control[1]); // 223.15 K
    let (cold, hot) = (pair_cold(), pair_hot());
    // Invented warm bracket point close enough that the fraction exceeds any
    // 1e-8 relative snap: (1e-6 K)/(300 K - 223.15 K).
    let warm = (300.0, "K");
    let points = || {
        vec![
            point("point:cold", (end_t, end_unit.as_str()), cold),
            point("point:warm", warm, hot),
        ]
    };
    for mode in MODES {
        let ctx = format!(
            "{} {near_t} {near_unit} vs {end_t} {end_unit}",
            mode.as_str()
        );
        let out = solve(
            &cantilever(points(), sel_interp("material:m", near_t, &near_unit), None),
            mode,
        );
        let member = by_pipe(&evidence(&out, CASE)["members"], "pipe:c", "members");
        let mut ids = consumed_point_ids(member);
        ids.sort();
        assert_eq!(
            ids,
            vec!["point:cold".to_string(), "point:warm".to_string()],
            "{ctx}: two consumed points"
        );
        let fraction = num(&member["interpolation_fraction"], "fraction");
        assert!(
            fraction > 0.0 && fraction < 1.0,
            "{ctx}: fraction {fraction} must be strictly inside (0, 1)"
        );
        let e = num(&member["selected_E_pa"], "E");
        assert!(
            e < cold.0 && e > hot.0,
            "{ctx}: E {e:.17e} strictly between the bracket points"
        );
    }
    // The endpoint point with this actual temperature is a different
    // temperature, so exact-point selection needs an explicit override.
    blocked_with(
        &cantilever(
            points(),
            sel_point("material:m", "point:cold"),
            Some((near_t, near_unit.as_str())),
        ),
        "non_equal_control_exact_point_without_override",
        &["LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED"],
    );
}

#[test]
fn temperature_identity_binary64_order_inversion_blocks() {
    // 467.6 degF is exactly 515.15 K, but its binary64 conversion is the
    // fixture's 515.1500000000001 K. A K point authored as that decimal is a
    // strictly greater exact temperature with the same binary64 representative,
    // so binary64 cannot keep the table's strict order.
    let fahrenheit = authored(&at(&format!(
        "{IDENTITY}/inputs/identity_groups/1/authored/1"
    )));
    assert_eq!(fahrenheit.1, "degF");
    let d = "cases/temperature_unit_identity/wrong_result_discriminators/binary64_affine_conversion/467p6_degF";
    assert_eq!(at(&format!("{d}/doubles_equal")), false);
    let collided = q(&format!("{d}/result"), "K");
    let (cold, hot) = (pair_cold(), pair_hot());
    let request = cantilever(
        vec![
            point("point:a", (fahrenheit.0, fahrenheit.1.as_str()), cold),
            point("point:b", (collided, "K"), hot),
            point("point:c", (WARM_K, "K"), hot),
        ],
        sel_interp("material:m", 550.0, "K"), // invented interior request
        None,
    );
    blocked_with(
        &request,
        "binary64_order_inversion",
        &["LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED"],
    );
}

#[test]
fn duplicate_class_material_points_are_refused() {
    let (cold, hot) = (pair_cold(), pair_hot());
    // Two exact-class-equal points in different units whose binary64
    // conversions differ (fixture discriminators), each with its own pair.
    for (group, first, second) in [(0, 0, 1), (1, 1, 2)] {
        let base = format!("{IDENTITY}/inputs/identity_groups/{group}/authored");
        let a = authored(at(&format!("{base}/{first}")));
        let b = authored(at(&format!("{base}/{second}")));
        let kelvin = q(
            &format!("{IDENTITY}/expected/identity_groups/{group}/kelvin"),
            "K",
        );
        let points = || {
            vec![
                point("point:first", (a.0, a.1.as_str()), cold),
                point("point:second", (b.0, b.1.as_str()), hot),
                point("point:warm", (WARM_K, "K"), hot),
            ]
        };
        for (label, request_t) in [
            ("at_duplicate_class", (a.0, a.1.as_str())),
            ("interior", ((kelvin + WARM_K) / 2.0, "K")),
        ] {
            blocked_with(
                &cantilever(
                    points(),
                    sel_interp("material:m", request_t.0, request_t.1),
                    None,
                ),
                &format!("duplicate_class_points/{}/{label}", a.1),
                &[
                    "LOAD_STATE_MATERIAL_TEMPERATURE_AMBIGUOUS",
                    "LOAD_STATE_MATERIAL_POINT_AMBIGUOUS",
                ],
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 3. thermal_datum_ratio.verification_two_point (M16)
// ---------------------------------------------------------------------------

const THERMAL: &str = "cases/thermal_datum_ratio/variants";

fn thermal_model(
    law: Value,
    install: (f64, &str),
    operating: (f64, &str),
    fixed: bool,
    e: f64,
    l: f64,
) -> Value {
    let mut m = material("material:thermal", e, INVENTED_NU);
    m["expansion_laws"] = json!([law]);
    let mut model = Model::new("project:thermal-extension")
        .node("node:root", 0.0)
        .node("node:far", l)
        .material(m)
        .pipe(
            "pipe:t",
            "node:root",
            "node:far",
            "material:thermal",
            basis_temperature(install.0, install.1),
            fit_none(),
        )
        .support("support:root", "node:root", &ALL);
    let mut case = Case::new(CASE)
        .element(
            "pipe:t",
            Some(operating),
            sel_base("material:thermal"),
            th_free("law:t"),
        )
        .support("support:root");
    if fixed {
        model = model.support("support:far", "node:far", &ALL);
        case = case.support("support:far");
    }
    model.case(case).request()
}

fn verification_law(install_c: f64) -> Value {
    let v = format!("{THERMAL}/verification_two_point/inputs");
    assert_eq!(
        text(&format!("{v}/coefficient_definition")),
        "engineering_secant"
    );
    assert_eq!(text(&format!("{v}/interpolation")), "linear_coefficient");
    let points: Vec<Value> = at(&format!("{v}/table_points"))
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            json!({"temperature": qv(qval(&p["temperature"], "degC", "T"), "degC"),
                "coefficient": qv(qval(&p["alpha"], "1/K", "alpha"), "1/K")})
        })
        .collect();
    assert_eq!(points.len(), 2, "control-4 exact table has two points");
    let datum = q(&format!("{v}/datum_temperature"), "degC");
    assert!(datum < install_c, "the datum lies outside the table");
    json!({"id": "law:t", "definition": "engineering_secant", "datum_temperature": qv(datum, "degC"),
        "data": {"kind": "table", "interpolation": "linear_coefficient", "points": points},
        "provenance": PROV})
}

#[test]
fn verification_two_point_secant_table_needs_only_install_to_operating_coverage() {
    let v = format!("{THERMAL}/verification_two_point");
    assert_eq!(
        at(&format!("{v}/admissibility_policy/datum_coverage_required")),
        false
    );
    let l = q(&format!("{v}/inputs/L"), "m");
    let e = q(&format!("{v}/inputs/E"), "Pa");
    let install = q(&format!("{v}/inputs/installation_temperature"), "degC");
    let operating = q(&format!("{v}/inputs/operating_temperature"), "degC");
    let x = |name: &str, unit: &str| q(&format!("{v}/expected/{name}"), unit);
    let strain = x("thermal_strain", "1");
    let fixed_wall = x("fixed_wall_N", "N");
    let force = fixed_wall.abs();
    close(
        x("required_coverage_low_K", "K"),
        install + KELVIN_OFFSET,
        0.0,
        "fixture coverage low",
    );
    close(
        x("required_coverage_high_K", "K"),
        operating + KELVIN_OFFSET,
        0.0,
        "fixture coverage high",
    );
    for mode in MODES {
        let ctx = mode.as_str();
        let out = solve(
            &thermal_model(
                verification_law(install),
                (install, "degC"),
                (operating, "degC"),
                true,
                e,
                l,
            ),
            mode,
        );
        member_axial(
            &out,
            CASE,
            "pipe:t",
            fixed_wall,
            force,
            &format!("{ctx} fixed"),
        );
        close(
            reaction(&out, CASE, "support:root", "Fx"),
            x("fixed_root_Fx", "N"),
            force,
            &format!("{ctx}: root Fx"),
        );
        close(
            reaction(&out, CASE, "support:far", "Fx"),
            x("fixed_far_Fx", "N"),
            force,
            &format!("{ctx}: far Fx"),
        );
        let m = by_pipe(&evidence(&out, CASE)["members"], "pipe:t", "members");
        close(
            num(&m["thermal_strain"], "strain"),
            strain,
            strain,
            &format!("{ctx}: evidence strain"),
        );
        close(
            num(&m["installation_temperature_k"], "Ti"),
            x("installation_temperature_K", "K"),
            0.0,
            &format!("{ctx}: T_install K"),
        );
        close(
            num(&m["operating_temperature_k"], "T"),
            x("operating_temperature_K", "K"),
            0.0,
            &format!("{ctx}: T K"),
        );
        close(
            num(&m["coefficient_datum_k"], "Tm"),
            x("datum_temperature_K", "K"),
            0.0,
            &format!("{ctx}: datum K"),
        );
        close(
            num(&m["installation_datum_stretch"], "l_i"),
            1.0 + x("dilation_install", "1"),
            0.0,
            &format!("{ctx}: lambda(T_install)"),
        );
        close(
            x("minimum_datum_stretch_over_required_interval", "1"),
            1.0 + x("dilation_install", "1"),
            0.0,
            "fixture minimum stretch",
        );
        close(
            num(&m["operating_datum_stretch"], "l"),
            1.0 + x("dilation_operating", "1"),
            0.0,
            &format!("{ctx}: lambda(T)"),
        );
        println!(
            "OBSERVED_LAW_DATA test=verification_two_point_secant mode={ctx} consumed_points={} consumed_segments={} consulted_points={} consulted_segments={}",
            m["consumed_law_point_indices"],
            m["consumed_law_segments"],
            m["consulted_law_point_indices"],
            m["consulted_law_segments"]
        );
        assert_eq!(
            indices(&m["consumed_law_point_indices"], "consumed"),
            vec![0, 1],
            "{ctx}: both table points enter the value"
        );
        let out = solve(
            &thermal_model(
                verification_law(install),
                (install, "degC"),
                (operating, "degC"),
                false,
                e,
                l,
            ),
            mode,
        );
        close(
            displacement(&out, CASE, "node:far", "x"),
            x("free_tip_UX", "m"),
            strain,
            &format!("{ctx}: free tip UX"),
        );
        member_axial(
            &out,
            CASE,
            "pipe:t",
            x("free_wall_N", "N"),
            force,
            &format!("{ctx} free"),
        );
    }
    // Outside [T_install, T]: invented offsets beyond the table edges.
    blocked_with(
        &thermal_model(
            verification_law(install),
            (install, "degC"),
            (operating + 10.0, "degC"),
            true,
            e,
            l,
        ),
        "verification_two_point_operating_above_table",
        &["LOAD_STATE_STRAIN_UNRESOLVED"],
    );
    blocked_with(
        &thermal_model(
            verification_law(install - 10.0),
            (install - 10.0, "degC"),
            (operating, "degC"),
            true,
            e,
            l,
        ),
        "verification_two_point_installation_below_table",
        &["LOAD_STATE_STRAIN_UNRESOLVED"],
    );
}

// ---------------------------------------------------------------------------
// 6. Dilation datum consulted, not consumed (M17)
// ---------------------------------------------------------------------------

fn within(actual: f64, expected: f64) -> bool {
    actual.is_finite() && (actual - expected).abs() <= RELATIVE * expected.abs()
}

/// CP3_WIRE_ADDENDUM section 1: every entry is `{use, lower_index,
/// upper_index, start_k, end_k}`, with upper = lower + 1 and
/// T_lower <= start_k <= end_k <= T_upper. An interpolation_sample has
/// start_k == end_k; an integration_interval has start_k < end_k. Entries are
/// unique and their order carries no meaning, so the observed list must equal
/// the expected entry set. Kelvin values use the protected relative criterion,
/// since they are binary64 conversions.
fn segments_match(
    observed: &Value,
    expected: &[(&str, u64, u64, f64, f64)],
    table_k: &[f64],
    context: &str,
) {
    let entries = observed
        .as_array()
        .unwrap_or_else(|| panic!("{context}: array expected, got {observed}"));
    for s in entries {
        let lower = s["lower_index"]
            .as_u64()
            .unwrap_or_else(|| panic!("{context}: lower_index {s}"));
        let upper = s["upper_index"]
            .as_u64()
            .unwrap_or_else(|| panic!("{context}: upper_index {s}"));
        let start = s["start_k"]
            .as_f64()
            .unwrap_or_else(|| panic!("{context}: start_k {s}"));
        let end = s["end_k"]
            .as_f64()
            .unwrap_or_else(|| panic!("{context}: end_k {s}"));
        assert_eq!(
            upper,
            lower + 1,
            "{context}: upper_index = lower_index + 1: {s}"
        );
        let (t_lo, t_hi) = (table_k[lower as usize], table_k[upper as usize]);
        assert!(
            start <= end
                && start >= t_lo - RELATIVE * t_lo.abs()
                && end <= t_hi + RELATIVE * t_hi.abs(),
            "{context}: start_k/end_k inside the segment: {s}"
        );
        match s["use"].as_str() {
            Some("interpolation_sample") => assert_eq!(
                start.to_bits(),
                end.to_bits(),
                "{context}: interpolation_sample has start_k == end_k: {s}"
            ),
            Some("integration_interval") => {
                assert!(
                    start < end,
                    "{context}: integration_interval has start_k < end_k: {s}"
                )
            }
            _ => panic!("{context}: unknown use label: {s}"),
        }
    }
    assert_eq!(
        entries.len(),
        expected.len(),
        "{context}: entry set size; observed {observed}"
    );
    let mut matched = vec![false; entries.len()];
    for &(use_label, lower, upper, start, end) in expected {
        let hit = entries.iter().enumerate().position(|(i, s)| {
            !matched[i]
                && s["use"] == use_label
                && s["lower_index"] == lower
                && s["upper_index"] == upper
                && s["start_k"].as_f64().is_some_and(|v| within(v, start))
                && s["end_k"].as_f64().is_some_and(|v| within(v, end))
        });
        let index = hit.unwrap_or_else(|| {
            panic!(
                "{context}: expected entry {{use: {use_label}, lower_index: {lower}, upper_index: {upper}, start_k: {start}, end_k: {end}}} missing; observed {observed}"
            )
        });
        matched[index] = true;
    }
}

#[test]
fn dilation_datum_zero_is_consulted_not_consumed() {
    let v = format!("{THERMAL}/annular_companion");
    let a = annulus();
    let l = q(&format!("{v}/inputs/L"), "m");
    let e = q(&format!("{v}/inputs/E"), "Pa");
    let datum = q(&format!("{v}/inputs/datum_temperature"), "degC");
    let install = q(&format!("{v}/inputs/installation_temperature"), "degC");
    let operating = q(&format!("{v}/inputs/operating_temperature"), "degC");
    let d_install = q(&format!("{v}/expected/dilation_install"), "1");
    let d_operating = q(&format!("{v}/expected/dilation_operating"), "1");
    let install_k = q(&format!("{v}/expected/installation_temperature_K"), "K");
    let operating_k = q(&format!("{v}/expected/operating_temperature_K"), "K");
    let table_k = [
        q(&format!("{v}/expected/datum_temperature_K"), "K"),
        install_k,
        operating_k,
    ];
    // engineering_dilation referred to the datum: lambda(T) = 1 + d(T), with
    // d(T_m) = 0 by definition, so the datum point is a zero-consistency check.
    let law = json!({"id": "law:t", "definition": "engineering_dilation",
        "datum_temperature": qv(datum, "degC"),
        "data": {"kind": "table", "interpolation": "linear_dilation", "points": [
            {"temperature": qv(datum, "degC"), "dilation": qv(0.0, "1")},
            {"temperature": qv(install, "degC"), "dilation": qv(d_install, "1")},
            {"temperature": qv(operating, "degC"), "dilation": qv(d_operating, "1")}]},
        "provenance": PROV});
    // Exact samples: strain = (1 + d(T))/(1 + d(T_install)) - 1, the fixture's
    // 43/25009. Interior variant, derived here: at the invented midpoint
    // T = (T_install + T)/2, linear dilation gives d = (d_i + d_o)/2.
    let midpoint = (install + operating) / 2.0;
    let d_mid = (d_install + d_operating) / 2.0;
    let variants = [
        (
            "exact_samples",
            operating,
            q(&format!("{v}/expected/thermal_strain"), "1"),
            false,
        ),
        (
            "interior_operating",
            midpoint,
            (1.0 + d_mid) / (1.0 + d_install) - 1.0,
            true,
        ),
    ];
    // Record the published law data for every variant before any assertion,
    // so one failing variant does not hide the others' evidence.
    for mode in MODES {
        for (label, t, _, _) in variants {
            let out = solve(
                &thermal_model(law.clone(), (install, "degC"), (t, "degC"), true, e, l),
                mode,
            );
            let m = by_pipe(&evidence(&out, CASE)["members"], "pipe:t", "members");
            println!(
                "OBSERVED_LAW_DATA test=dilation variant={label} mode={} consumed_points={} consumed_segments={} consulted_points={} consulted_segments={}",
                mode.as_str(),
                m["consumed_law_point_indices"],
                m["consumed_law_segments"],
                m["consulted_law_point_indices"],
                m["consulted_law_segments"]
            );
        }
    }
    for mode in MODES {
        for (label, t, strain, interior) in variants {
            let ctx = format!("{} {label}", mode.as_str());
            let out = solve(
                &thermal_model(law.clone(), (install, "degC"), (t, "degC"), true, e, l),
                mode,
            );
            let n = -e * a.area * strain;
            member_axial(&out, CASE, "pipe:t", n, n.abs(), &ctx);
            let m = by_pipe(&evidence(&out, CASE)["members"], "pipe:t", "members");
            close(
                num(&m["thermal_strain"], "strain"),
                strain,
                strain,
                &format!("{ctx}: evidence strain"),
            );
            assert_eq!(m["thermal_definition"], "engineering_dilation", "{ctx}");
            let consumed = indices(&m["consumed_law_point_indices"], "consumed");
            let consulted = indices(&m["consulted_law_point_indices"], "consulted");
            // Only the install (1) and operating (2 or bracket 1-2) samples enter the value.
            assert_eq!(
                consumed,
                vec![1, 2],
                "{ctx}: consumed points are exactly the value's inputs"
            );
            assert!(
                !consumed.contains(&0),
                "{ctx}: datum-zero point is not consumed"
            );
            assert!(
                consulted.contains(&0),
                "{ctx}: datum-zero point is consulted: {consulted:?}"
            );
            // CP3_WIRE_ADDENDUM.md (sha256 f69043b6...) section 2, engineering_dilation:
            // - consumed: a sample at T_i and at T, plus one integration_interval
            //   for every segment piece of [min(T_i,T), max(T_i,T)];
            // - consulted: the datum sample, plus positivity samples at the
            //   interval ends, T_m, T_i, T and the table points inside. There is
            //   never a consulted integration_interval.
            // Consumed and consulted are independent and may overlap (section 1).
            //
            // Table (fixture inputs): 0 = T_m (20 degC), 1 = T_i (50 degC),
            // 2 = 150 degC. T_i is point 1, so [T_i, T] spans only segment 1-2.
            // - T = 150 degC (point 2). Both samples are point indices.
            //   Consumed segments: [integration 1-2 over T_i..T].
            //   Consulted: the datum 0, T_i = 1 and T = 2 (the table points of
            //   [20, 150]) as points; no segments.
            // - T = 100 degC (inside 1-2). The T sample is interpolation_sample
            //   1-2 at T. Consumed segments: [that sample, integration 1-2 over
            //   T_i..T]. Consulted: the datum 0 and T_i = 1 as points, plus the
            //   same sample at T (its segment endpoints 1 and 2 enter as points).
            // Kelvin endpoints: the fixture's installation/operating K. The
            // midpoint K is their mean, because T = (T_i + 150 degC)/2.
            let ti_k = install_k;
            let t_k = if interior {
                (install_k + operating_k) / 2.0
            } else {
                operating_k
            };
            let sample = ("interpolation_sample", 1, 2, t_k, t_k);
            let increment = ("integration_interval", 1, 2, ti_k, t_k);
            let (want_consumed, want_consulted) = if interior {
                (vec![sample, increment], vec![sample])
            } else {
                (vec![increment], vec![])
            };
            assert_eq!(
                consulted,
                vec![0, 1, 2],
                "{ctx}: consulted points are the datum and positivity samples"
            );
            segments_match(
                &m["consumed_law_segments"],
                &want_consumed,
                &table_k,
                &format!("{ctx}: consumed_law_segments"),
            );
            segments_match(
                &m["consulted_law_segments"],
                &want_consulted,
                &table_k,
                &format!("{ctx}: consulted_law_segments"),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 7. Joined-route acceptance (item 7)
// ---------------------------------------------------------------------------

fn eigen_motion_request() -> Value {
    let bytes = std::fs::read(EIGEN_MOTION_REQUEST).expect("read eigen_motion.request.json");
    let digest = Sha256::digest(&bytes);
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        hex, EIGEN_MOTION_SHA256,
        "producer witness bytes are the reviewed ones"
    );
    serde_json::from_slice(&bytes).expect("eigen_motion request parses")
}

struct EigenMotion {
    e: f64,
    nu: f64,
    area: f64,
    polar: f64,
    l: f64,
    eps_star: f64,
    stop_ux: f64,
    root_uy: f64,
    root_rz: f64,
    torque: f64,
    spring: f64,
}

/// Declared witness inputs, read from the request and checked against the brief.
fn eigen_motion_inputs(request: &Value) -> EigenMotion {
    let model = &request["model"];
    let m = &model["materials"][0];
    let e = qval(&m["elastic_modulus"], "Pa", "E");
    let nu = qval(&m["poisson_ratio"], "1", "nu");
    let section = &model["pipe_segments"][0]["section"];
    let od = qval(&section["outside_diameter"], "m", "OD");
    let t = qval(&section["wall_thickness"], "m", "wall");
    let l = model["nodes"][1]["position"]["x"].as_f64().unwrap()
        - model["nodes"][0]["position"]["x"].as_f64().unwrap();
    let fit = qval(
        &model["reference_configurations"][0]["member_references"][0]["fit"]["strain"],
        "1",
        "fit",
    );
    let state = &model["load_cases"][0]["analysis_state"];
    let thermal = qval(
        &state["element_states"][0]["thermal_state"]["strain"],
        "1",
        "thermal",
    );
    let anchor = &state["support_states"][0]["boundary_motion"];
    let root_uy = qval(&anchor[0]["value"], "mm", "root UY") / 1000.0;
    let root_rz = qval(&anchor[1]["value"], "rad", "root RZ");
    let stop_ux = qval(
        &state["support_states"][2]["boundary_motion"][0]["value"],
        "m",
        "stop UX",
    );
    let torque = qval(
        &model["load_cases"][0]["primitive_loads"][0]["magnitude"],
        "N*m",
        "torque",
    );
    let spring = qval(
        &model["supports"][1]["stiffness"]["value"],
        "N*m/rad",
        "spring",
    );
    assert_eq!(anchor[0]["dof"], "UY");
    assert_eq!(anchor[1]["dof"], "RZ");
    for (name, actual, brief) in [
        ("E", e, 200.0e9),
        ("OD", od, 0.2),
        ("wall", t, 0.01),
        ("L", l, 2.0),
        ("thermal", thermal, 6.0e-5),
        ("fit", fit, 4.0e-5),
        ("stop UX", stop_ux, 5.0e-5),
        ("root UY", root_uy, 1.0e-3),
        ("root RZ", root_rz, 1.0e-4),
        ("torque", torque, 1.0e-8),
        ("spring", spring, 1.0e-4),
    ] {
        close(
            actual,
            brief,
            0.0,
            &format!("witness input {name} as stated in the brief"),
        );
    }
    // Annulus from the authored OD/wall (own formulas), cross-checked with the
    // maintained fixture annulus (same OD 0.20 / wall 0.01).
    let ro = od / 2.0;
    let ri = ro - t;
    let area = std::f64::consts::PI * (ro * ro - ri * ri);
    let polar = std::f64::consts::PI * (ro.powi(4) - ri.powi(4)) / 2.0;
    let fixture_annulus = annulus();
    close(area, fixture_annulus.area, 0.0, "annulus As");
    close(polar, fixture_annulus.polar, 0.0, "annulus J");
    EigenMotion {
        e,
        nu,
        area,
        polar,
        l,
        eps_star: (1.0 + thermal) * (1.0 + fit) - 1.0,
        stop_ux,
        root_uy,
        root_rz,
        torque,
        spring,
    }
}

#[test]
fn joined_eigen_motion_closed_form_and_source_publication() {
    let request = eigen_motion_request();
    let w = eigen_motion_inputs(&request);
    let case = "case:join";
    // Closed form (small-displacement straight member, root at x=0, tip at x=L):
    // - Bending: no transverse load, so tip UY/RZ follow the prescribed root
    //   rigid motion: UY_tip = UY_root + L*theta and RZ_tip = theta, with zero
    //   shear and moment.
    // - Axial: root UX is restrained at 0 and the tip stop prescribes delta, so
    //   N = E A (delta/L - eps*); anchor Fx = -N and stop Fx = +N.
    // - Torsion: the tip torque T acts through the member (GJ/L) in series with
    //   the root spring k: RX_root = T/k, RX_tip = T/k + T L/(G J), and the
    //   spring Mx = -T.
    let n = w.e * w.area * (w.stop_ux / w.l - w.eps_star);
    let g = shear_modulus(w.e, w.nu);
    let rx_root = w.torque / w.spring;
    let rx_tip = rx_root + w.torque * w.l / (g * w.polar);
    let tip_uy = w.root_uy + w.l * w.root_rz;
    let force = n.abs();
    for mode in MODES {
        let label = mode.as_str();
        let out = run(&request, mode).unwrap_or_else(|e| panic!("[{label}] Err {e}"));
        common_envelope(&out, label);
        assert_eq!(
            out.producer.semantic_contract_id, SOURCE_SEMANTICS,
            "[{label}]"
        );
        assert_eq!(
            out.formulation_basis.profile_id, SOURCE_PROFILE,
            "[{label}]"
        );
        let receipt = out
            .source_block_recovery
            .as_ref()
            .expect("retained-source receipt published");
        assert_eq!(
            receipt["body"]["policy"], SOURCE_POLICY,
            "[{label}] receipt policy"
        );
        assert_eq!(
            receipt["body"]["status"], "qualified",
            "[{label}] receipt status"
        );
        let record = evidence(&out, case);
        assert_eq!(record["source_recovery"]["status"], "selected", "[{label}]");
        assert_eq!(
            record["source_recovery"]["method"], SOURCE_METHOD,
            "[{label}]"
        );
        assert_eq!(
            record["solve"]["recovery_method"], SOURCE_METHOD,
            "[{label}]"
        );
        assert!(
            !out.diagnostics.iter().any(|d| d.code == NOT_JOINED),
            "[{label}] joined case must not carry {NOT_JOINED}"
        );
        assert!(
            out.diagnostics
                .iter()
                .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED" && d.severity == "info"),
            "[{label}] SOURCE_BLOCK_RECOVERY_SELECTED info"
        );
        let ctx = format!("{label} joined");
        close(
            displacement(&out, case, "tip", "x"),
            w.stop_ux,
            w.stop_ux,
            &format!("{ctx}: tip UX"),
        );
        close(
            displacement(&out, case, "tip", "y"),
            tip_uy,
            tip_uy,
            &format!("{ctx}: tip UY"),
        );
        close(
            rotation(&out, case, "tip", "z"),
            w.root_rz,
            w.root_rz,
            &format!("{ctx}: tip RZ"),
        );
        close(
            rotation(&out, case, "root", "x"),
            rx_root,
            rx_root,
            &format!("{ctx}: root RX"),
        );
        close(
            rotation(&out, case, "tip", "x"),
            rx_tip,
            rx_tip,
            &format!("{ctx}: tip RX"),
        );
        close(
            displacement(&out, case, "root", "x"),
            0.0,
            w.stop_ux,
            &format!("{ctx}: root UX"),
        );
        close(
            displacement(&out, case, "root", "y"),
            w.root_uy,
            w.root_uy,
            &format!("{ctx}: root UY"),
        );
        close(
            rotation(&out, case, "root", "z"),
            w.root_rz,
            w.root_rz,
            &format!("{ctx}: root RZ"),
        );
        close(
            displacement(&out, case, "tip", "z"),
            0.0,
            tip_uy,
            &format!("{ctx}: tip UZ"),
        );
        close(
            rotation(&out, case, "tip", "y"),
            0.0,
            w.root_rz,
            &format!("{ctx}: tip RY"),
        );
        member_axial(&out, case, "member", n, force, &ctx);
        close(
            reaction(&out, case, "anchor", "Fx"),
            -n,
            force,
            &format!("{ctx}: anchor Fx"),
        );
        close(
            reaction(&out, case, "stop", "Fx"),
            n,
            force,
            &format!("{ctx}: stop Fx"),
        );
        for component in ["Fy", "Fz"] {
            close(
                reaction(&out, case, "anchor", component),
                0.0,
                force,
                &format!("{ctx}: anchor {component}"),
            );
        }
        for component in ["My", "Mz"] {
            close(
                reaction(&out, case, "anchor", component),
                0.0,
                force * w.l,
                &format!("{ctx}: anchor {component}"),
            );
        }
        close(
            reaction(&out, case, "spring", "Mx"),
            -w.torque,
            w.torque,
            &format!("{ctx}: spring Mx"),
        );
        for (kind, component, scale) in [
            ("element_local_shear_force_y", "shear_force_y", force),
            (
                "element_local_bending_moment_z",
                "bending_moment_z",
                force * w.l,
            ),
        ] {
            for location in ["end_i", "midspan", "end_j"] {
                let r = row(&out, case, kind, "member", component, Some(location));
                close(
                    r.value,
                    0.0,
                    scale,
                    &format!("{ctx}: {component} {location}"),
                );
            }
        }
    }
}

#[test]
fn joined_eigen_motion_with_pressure_stays_ordinary_not_joined() {
    let mut request = eigen_motion_request();
    let w = eigen_motion_inputs(&request);
    let case = "case:join";
    // A nonzero pressure region makes the case ineligible for the join
    // (ADDENDUM_2 section 5.2); the ordinary response is published.
    request["model"]["load_cases"][0]["pressure_regions"] = json!([closed_region(
        "region:companion",
        "member",
        "root",
        "tip",
        INVENTED_PRESSURE_PA
    )]);
    // With both closures transferring: Nw = E A (delta/L - eps*) + 2 nu p Ai,
    // anchor Fx = cap - Nw and stop Fx = Nw - cap (derivation as in test 1).
    // The invented 2 MPa gives a Poisson term 2 nu p Ai of about 2.5e4 N
    // (and cap p Ai of about 5.1e4 N), against |Nw| of about 6.4e4 N. The
    // pressure contribution is therefore far above the 1e-9 * |Nw| floor
    // (about 6e-5 N) and discriminates a dropped or base-pair pressure term.
    let cap = INVENTED_PRESSURE_PA * annulus().bore;
    let wall = w.e * w.area * (w.stop_ux / w.l - w.eps_star) + 2.0 * w.nu * cap;
    let scale = wall.abs().max(cap);
    for mode in MODES {
        let label = mode.as_str();
        let out = solve(&request, mode);
        assert!(
            out.diagnostics
                .iter()
                .any(|d| d.code == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE" && d.severity == "info"),
            "[{label}] info SOURCE_BLOCK_RECOVERY_UNAVAILABLE; diagnostics={:#?}",
            out.diagnostics
        );
        assert_eq!(
            out.diagnostics
                .iter()
                .filter(|d| d.code == NOT_JOINED)
                .count(),
            1,
            "[{label}] exactly one {NOT_JOINED}"
        );
        let ctx = format!("{label} companion");
        pressure_rows(&out, case, "member", wall, cap, scale, &ctx);
        close(
            reaction(&out, case, "anchor", "Fx"),
            cap - wall,
            scale,
            &format!("{ctx}: anchor Fx"),
        );
        close(
            reaction(&out, case, "stop", "Fx"),
            wall - cap,
            scale,
            &format!("{ctx}: stop Fx"),
        );
    }
}
