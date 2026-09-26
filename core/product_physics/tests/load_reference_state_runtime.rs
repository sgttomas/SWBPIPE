//! Independent public-API runtime oracle for the connected 0.4.0
//! load/reference-state route (checkpoint 2).
//!
//! Authored by the CP2_RUNTIME_TESTS TASK (not the implementer) from the frozen
//! wire `LOAD_STATE_IMPLEMENTATION/CP2_WIRE.md`, the reviewed design at Git
//! `9e8a55d` and the manager's stated evidence-record keys. Every request is
//! built to that wire and driven through
//! `run_linear_static_preview_value_with_mode` in both solver modes. No
//! production helper is an oracle here.
//!
//! Expected values are read at test time from
//! `tests/fixtures/load_reference_states/reference_cases.json` (the `value`
//! field, the binary64 projection of each exact analytical reference). Model
//! inputs come from the same file where it supplies them. Any other number is
//! an invented analytical input and is marked as one. None of it is library,
//! component, material-catalog or code-rule data.
//!
//! Numerical criterion: the protected relative criterion
//! `|actual - expected| <= 1e-9 * |expected|`. For an exact-zero reference the
//! relative test is undefined. The same existing zero-reference handling then
//! uses an absolute floor of `1e-9 * zero_scale`, where `zero_scale` is the
//! case's own force, moment, displacement or strain magnitude, named at each
//! call site. No looser tolerance is used anywhere.

use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode, ResultItem,
};
use serde_json::{json, Value};
use std::sync::OnceLock;

const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];
/// Protected relative criterion; never loosened.
const RELATIVE: f64 = 1.0e-9;
const SEMANTICS: &str = "openpipestress.result_semantics/0.3.0/load-reference-1";
const PROFILE: &str = "resolved_straight_load_state_v1";
const STATE_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
const REFERENCE: &str = "reference:installed";
const NOT_JOINED: &str = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED";
const SOURCE_DUPLICATE: &str = "LOAD_STATE_SOURCE_DUPLICATE";
const VERSION_MISMATCH: &str = "LOAD_STATE_CONTRACT_VERSION_MISMATCH";
const CASE: &str = "case:reference";
const ALL: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const TRANSVERSE: [&str; 5] = ["UY", "UZ", "RX", "RY", "RZ"];
const PROV: &str = "invented analytical test input; not library, component or code data";
/// Invented Poisson ratio for members whose checked axial/bending results do
/// not depend on it (no pressure, no torsion is checked on those members).
const INVENTED_NU: f64 = 0.3;
/// Exact unit definition (degC to K offset), used only for the all-Kelvin
/// variant and cross-checked against the fixture's own Kelvin values.
const KELVIN_OFFSET: f64 = 273.15;

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

/// Binary64 projection of one maintained quantity, with its unit checked.
fn qval(quantity: &Value, unit: &str, context: &str) -> f64 {
    assert_eq!(
        quantity["unit"].as_str(),
        Some(unit),
        "fixture unit at {context}: {quantity}"
    );
    quantity["value"]
        .as_f64()
        .unwrap_or_else(|| panic!("fixture value at {context}: {quantity}"))
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

fn generic(case: &str, rest: &str) -> String {
    format!("cases/{case}/variants/generic_reviewed/{rest}")
}

struct Annulus {
    od: f64,
    wall: f64,
    area: f64,
    polar: f64,
}

fn annulus() -> Annulus {
    Annulus {
        od: q("geometry/authored/outside_diameter", "m"),
        wall: q("geometry/authored/wall_thickness", "m"),
        area: q("geometry/derived/As", "m^2"),
        polar: q("geometry/derived/J", "m^4"),
    }
}

fn assert_annular_geometry(case: &str) {
    assert_eq!(
        text(&ann(case, "inputs/geometry_ref")),
        text("geometry/id"),
        "{case} annular companion uses the maintained OD 0.20 / wall 0.01 annulus"
    );
}

// ---------------------------------------------------------------------------
// Numerical checks
// ---------------------------------------------------------------------------

/// Protected relative criterion; exact-zero references use the stated floor.
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

/// A named wrong result must be distinguishable under the same criterion, and
/// the observation must not reproduce it.
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

// ---------------------------------------------------------------------------
// 0.4.0 request builder (CP2_WIRE.md)
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

fn fit_length(value: f64, unit: &str) -> Value {
    json!({"kind": "natural_length_change", "length_change": qv(value, unit)})
}

fn fit_strain(value: f64) -> Value {
    json!({"kind": "fit_strain", "strain": qv(value, "1")})
}

fn sel_base(material: &str) -> Value {
    json!({"kind": "explicit_base_properties", "material_ref": material,
        "applicability_reference": "invented analytical basis declared applicable to this member"})
}

fn sel_point(material: &str, point: &str) -> Value {
    json!({"kind": "exact_point", "material_ref": material, "point_ref": point})
}

fn th_unchanged() -> Value {
    json!({"kind": "unchanged_reference", "provenance": PROV})
}

fn th_interval(strain: f64) -> Value {
    json!({"kind": "explicit_interval_strain", "strain": qv(strain, "1"),
        "interval_reference": "invented reference-to-state interval", "provenance": PROV})
}

fn th_constant_alpha(
    coefficient: f64,
    coefficient_unit: &str,
    change: f64,
    change_unit: &str,
) -> Value {
    json!({"kind": "constant_alpha_interval", "coefficient": qv(coefficient, coefficient_unit),
        "temperature_change": qv(change, change_unit),
        "coefficient_meaning": "engineering_interval", "provenance": PROV})
}

fn th_free(law: &str) -> Value {
    json!({"kind": "free_length_state", "expansion_law_ref": law})
}

fn material(id: &str, e: f64, nu: f64) -> Value {
    json!({"id": id, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": qv(e, "Pa"), "poisson_ratio": qv(nu, "1"), "provenance": PROV})
}

fn point(id: &str, temperature: f64, unit: &str, e: f64, nu: f64) -> Value {
    json!({"id": id, "temperature": qv(temperature, unit), "elastic_modulus": qv(e, "Pa"),
        "poisson_ratio": qv(nu, "1"), "provenance": PROV})
}

fn law_secant_table(
    id: &str,
    datum: Value,
    points: &[(f64, f64)],
    temperature_unit: &str,
    coefficient_unit: &str,
) -> Value {
    let points: Vec<Value> = points
        .iter()
        .map(|(t, a)| json!({"temperature": qv(*t, temperature_unit), "coefficient": qv(*a, coefficient_unit)}))
        .collect();
    json!({"id": id, "definition": "engineering_secant", "datum_temperature": datum,
        "data": {"kind": "table", "interpolation": "linear_coefficient", "points": points},
        "provenance": PROV})
}

fn law_secant_constant(id: &str, datum: Value, coefficient: Value) -> Value {
    json!({"id": id, "definition": "engineering_secant", "datum_temperature": datum,
        "data": {"kind": "constant", "coefficient": coefficient}, "provenance": PROV})
}

fn nodal_force(id: &str, node: &str, value: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node},
        "direction": "global_x", "magnitude": qv(value, "N"), "dimension": "force",
        "provenance": PROV})
}

fn nodal_torque(id: &str, node: &str, value: f64) -> Value {
    json!({"id": id, "category": "concentrated_moment", "target": {"type": "node", "node": node},
        "direction": "rotation_x", "magnitude": qv(value, "N*m"), "dimension": "moment",
        "provenance": PROV})
}

fn zero_pressure_region(id: &str, pipe: &str, from: &str, to: &str) -> Value {
    json!({"id": id, "member_pipe_ids": [pipe],
        "pressure_basis": "internal_differential_zero_external_v1", "pressure": qv(0.0, "Pa"),
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

    /// Straight annular pipe plus its single member reference.
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

    fn support(mut self, id: &str, node: &str, family: &str, restraints: &[&str]) -> Self {
        self.supports
            .push(json!({"id": id, "node": node, "family": family,
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

    fn support(mut self, support: &str, motions: &[(&str, f64, &str)]) -> Self {
        let mut state =
            json!({"support_ref": support, "participation": {"kind": "active_model_device"}});
        if !motions.is_empty() {
            state["boundary_motion"] = Value::Array(
                motions
                    .iter()
                    .map(|(dof, value, unit)| {
                        json!({"dof": dof, "value": qv(*value, unit),
                            "meaning": "absolute_reference_displacement"})
                    })
                    .collect(),
            );
        }
        self.supports.push(state);
        self
    }

    fn primitive(mut self, primitive: Value) -> Self {
        self.primitives.push(primitive);
        self
    }

    fn source(mut self, source_ref: &str, factor: f64) -> Self {
        self.sources
            .push(json!({"source_ref": source_ref, "factor": factor}));
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
// Public-boundary execution and envelope contract
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

/// Blocking code and message pairs, printed as the observed-code record.
fn blocking_details(out: &MechanicsEnvelope) -> Vec<String> {
    out.diagnostics
        .iter()
        .filter(|d| d.severity == "blocking" || d.severity == "failure")
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect()
}

fn blocking_codes(out: &MechanicsEnvelope) -> Vec<String> {
    out.diagnostics
        .iter()
        .filter(|d| d.severity == "blocking" || d.severity == "failure")
        .map(|d| d.code.clone())
        .collect()
}

fn evidence_records(out: &MechanicsEnvelope) -> &Vec<Value> {
    out.contract_evidence
        .as_ref()
        .expect("contract_evidence is published")["load_reference_states"]
        .as_array()
        .expect("contract_evidence.load_reference_states[] is published")
}

fn evidence<'a>(out: &'a MechanicsEnvelope, case: &str) -> &'a Value {
    let matching: Vec<_> = evidence_records(out)
        .iter()
        .filter(|r| r["load_case_id"] == case)
        .collect();
    assert_eq!(
        matching.len(),
        1,
        "exactly one load_reference_states record for {case}"
    );
    matching[0]
}

fn member_evidence<'a>(record: &'a Value, pipe: &str) -> &'a Value {
    let matching: Vec<_> = record["members"]
        .as_array()
        .expect("evidence members[]")
        .iter()
        .filter(|m| m["pipe_id"] == pipe)
        .collect();
    assert_eq!(matching.len(), 1, "exactly one member evidence for {pipe}");
    matching[0]
}

fn num(value: &Value, context: &str) -> f64 {
    value
        .as_f64()
        .or_else(|| value["value"].as_f64())
        .unwrap_or_else(|| panic!("{context}: expected a number, got {value}"))
}

/// Evidence text search for identities whose record shape the wire leaves open.
fn mentions(value: &Value, id: &str) -> bool {
    value.to_string().contains(&format!("\"{id}\""))
}

/// Solve a supported reference and check the 0.4.0 envelope contract.
fn solve(request: &Value, mode: PreviewSolverMode) -> MechanicsEnvelope {
    let label = mode.as_str();
    let out = run(request, mode).unwrap_or_else(|error| {
        panic!("[{label}] valid 0.4.0 request rejected at the typed boundary: {error}")
    });
    assert_eq!(
        out.status.mechanics, "MECHANICS_SOLVED",
        "[{label}] supported reference did not solve; diagnostics={:#?}",
        out.diagnostics
    );
    assert!(
        blocking_codes(&out).is_empty(),
        "[{label}] solved reference carries blocking diagnostics: {:#?}",
        out.diagnostics
    );
    assert!(!out.accepted_model_state_mutated);
    assert!(out.results.iter().all(|r| r.value.is_finite()));
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
        "[{label}] 0.4.0 must not publish retained-source recovery"
    );
    let ids = case_ids(request);
    assert_eq!(
        evidence_records(&out).len(),
        ids.len(),
        "[{label}] one load_reference_states record per case"
    );
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
        assert_eq!(
            carried[0].severity, "info",
            "[{label}] {NOT_JOINED} severity"
        );
    }
    out
}

/// Expect every case of the request to block after parse, before assembly.
fn blocked(request: &Value, control: &str) -> Vec<String> {
    let ids = case_ids(request);
    let mut observed = Vec::new();
    for mode in MODES {
        let label = mode.as_str();
        let out = run(request, mode).unwrap_or_else(|error| {
            panic!("[{control}/{label}] expected a blocking diagnostic after parse; typed boundary returned Err: {error}")
        });
        let codes = blocking_codes(&out);
        println!(
            "OBSERVED_BLOCK control={control} mode={label} status={} codes={codes:?} details={:?}",
            out.status.mechanics,
            blocking_details(&out)
        );
        assert_ne!(
            out.status.mechanics, "MECHANICS_SOLVED",
            "[{control}/{label}] invalid state was accepted; diagnostics={:#?}",
            out.diagnostics
        );
        assert!(
            !codes.is_empty(),
            "[{control}/{label}] no blocking diagnostic; diagnostics={:#?}",
            out.diagnostics
        );
        assert!(
            !out.results.iter().any(|r| r
                .basis_ref
                .as_ref()
                .is_some_and(|b| ids.iter().any(|id| id == &b.ref_id))),
            "[{control}/{label}] blocked case published result rows"
        );
        observed = codes;
    }
    observed
}

fn blocked_with(request: &Value, control: &str, code: &str) {
    for mode in MODES {
        let out = run(request, mode).unwrap_or_else(|error| {
            panic!(
                "[{control}/{}] expected {code}; typed boundary returned Err: {error}",
                mode.as_str()
            )
        });
        assert!(
            out.diagnostics
                .iter()
                .any(|d| d.code == code && (d.severity == "blocking" || d.severity == "failure")),
            "[{control}/{}] expected targeted blocking {code}; diagnostics={:#?}",
            mode.as_str(),
            out.diagnostics
        );
    }
    blocked(request, control);
}

/// Expect a typed-boundary rejection (`Err`) before any solve.
fn rejected(request: &Value, control: &str) {
    for mode in MODES {
        match run(request, mode) {
            Err(error) => {
                println!(
                    "OBSERVED_REJECT control={control} mode={} error={error}",
                    mode.as_str()
                )
            }
            Ok(out) => panic!(
                "[{control}/{}] expected typed-boundary Err; got status={} diagnostics={:#?}",
                mode.as_str(),
                out.status.mechanics,
                out.diagnostics
            ),
        }
    }
}

/// For controls whose rejection channel the wire leaves open: either a typed
/// rejection or a blocking diagnostic, never a solve.
fn refused(request: &Value, control: &str) {
    for mode in MODES {
        match run(request, mode) {
            Err(error) => {
                println!(
                    "OBSERVED_REJECT control={control} mode={} error={error}",
                    mode.as_str()
                )
            }
            Ok(out) => {
                let codes = blocking_codes(&out);
                println!(
                    "OBSERVED_BLOCK control={control} mode={} status={} codes={codes:?} details={:?}",
                    mode.as_str(),
                    out.status.mechanics,
                    blocking_details(&out)
                );
                assert_ne!(
                    out.status.mechanics, "MECHANICS_SOLVED",
                    "[{control}] accepted"
                );
                assert!(!codes.is_empty(), "[{control}] no blocking diagnostic");
                assert!(
                    out.results.is_empty(),
                    "[{control}] refused request published rows"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Result rows (existing exact-profile kinds)
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
        "expected one row case={case} kind={kind} entity={entity} component={component} location={location:?}; entity rows={:?}",
        out.results
            .iter()
            .filter(|r| r.entity_ref == entity)
            .map(|r| (&r.kind, r.metadata.as_ref().map(|m| (&m.component, &m.location))))
            .collect::<Vec<_>>()
    );
    matching[0]
}

/// Global nodal translation in metres (rows are published in mm).
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

/// Support-on-pipe global action.
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

fn zero_other_reactions(
    out: &MechanicsEnvelope,
    case: &str,
    support: &str,
    skip: &[&str],
    force_scale: f64,
    moment_scale: f64,
    context: &str,
) {
    for component in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"] {
        if skip.contains(&component) {
            continue;
        }
        let scale = if component.starts_with('F') {
            force_scale
        } else {
            moment_scale
        };
        close(
            reaction(out, case, support, component),
            0.0,
            scale,
            &format!("{context}: {support} {component}"),
        );
    }
}

/// Tension-positive station N at the three interior stations and the
/// element-end force pair (end_i = -N, end_j = +N) for a member without a
/// pressure region.
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
        assert_eq!(r.unit, "N");
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
        assert_eq!(r.unit, "N");
        close(
            r.value,
            sign * expected,
            zero_scale,
            &format!("{context}: {pipe} end force {location}"),
        );
    }
}

/// Observed tension-positive member N at midspan.
fn midspan_axial(out: &MechanicsEnvelope, case: &str, pipe: &str) -> f64 {
    row(
        out,
        case,
        "element_local_axial_force",
        pipe,
        "axial_force",
        Some("midspan"),
    )
    .value
}

/// Exact-profile wall force rows for a member inside a (zero-pressure) region.
fn wall_axial(
    out: &MechanicsEnvelope,
    case: &str,
    pipe: &str,
    expected: f64,
    zero_scale: f64,
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
        assert_eq!(r.unit, "N");
        close(
            r.value,
            expected,
            zero_scale,
            &format!("{context}: {pipe} wall N {location}"),
        );
    }
}

// ---------------------------------------------------------------------------
// Base models
// ---------------------------------------------------------------------------

const PRESCRIBED_TWO_BAR: &str = "prescribed_translation_two_bar";

fn two_bar_model(reversed: bool, far_explicit: bool) -> Value {
    let c = PRESCRIBED_TWO_BAR;
    let l1 = q(&ann(c, "inputs/L1"), "m");
    let l2 = q(&ann(c, "inputs/L2"), "m");
    let e1 = q(&ann(c, "inputs/E1"), "Pa");
    let e2 = q(&ann(c, "inputs/E2"), "Pa");
    let root_ux = q(&ann(c, "inputs/root_UX"), "m");
    let far_ux = q(&ann(c, "inputs/far_UX"), "m");
    let (p1, p2) = if reversed {
        (("node:middle", "node:root"), ("node:far", "node:middle"))
    } else {
        (("node:root", "node:middle"), ("node:middle", "node:far"))
    };
    let far_motion: Vec<(&str, f64, &str)> = if far_explicit {
        vec![("UX", far_ux, "m")]
    } else {
        vec![]
    };
    Model::new("project:prescribed-two-bar")
        .node("node:root", 0.0)
        .node("node:middle", l1)
        .node("node:far", l1 + l2)
        .material(material("material:bar-1", e1, INVENTED_NU))
        .material(material("material:bar-2", e2, INVENTED_NU))
        .pipe(
            "pipe:1",
            p1.0,
            p1.1,
            "material:bar-1",
            basis_direct(),
            fit_none(),
        )
        .pipe(
            "pipe:2",
            p2.0,
            p2.1,
            "material:bar-2",
            basis_direct(),
            fit_none(),
        )
        .support("support:root", "node:root", "anchor", &ALL)
        .support("support:middle", "node:middle", "anchor", &TRANSVERSE)
        .support("support:far", "node:far", "anchor", &ALL)
        .case(
            Case::new(CASE)
                .element("pipe:1", None, sel_base("material:bar-1"), th_unchanged())
                .element("pipe:2", None, sel_base("material:bar-2"), th_unchanged())
                // Authored in mm (0.1 mm) to cross the length normalization seam.
                .support("support:root", &[("UX", root_ux * 1000.0, "mm")])
                .support("support:middle", &[])
                .support("support:far", &far_motion),
        )
        .request()
}

fn single_bar_model(root_motion: &[(&str, f64, &str)]) -> Value {
    let c = "prescribed_translation_all_fixed";
    let l = q(&ann(c, "inputs/L"), "m");
    let e = q(&ann(c, "inputs/E"), "Pa");
    Model::new("project:prescribed-all-fixed")
        .node("node:root", 0.0)
        .node("node:far", l)
        .material(material("material:bar", e, INVENTED_NU))
        .pipe(
            "pipe:bar",
            "node:root",
            "node:far",
            "material:bar",
            basis_direct(),
            fit_none(),
        )
        .support("support:root", "node:root", "anchor", &ALL)
        .support("support:far", "node:far", "anchor", &ALL)
        .case(
            Case::new(CASE)
                .element("pipe:bar", None, sel_base("material:bar"), th_unchanged())
                .support("support:root", root_motion)
                .support("support:far", &[]),
        )
        .request()
}

fn beam_model(case: &str, far_anchor: bool, root_motion: &[(&str, f64, &str)]) -> Value {
    let l = q(&ann(case, "inputs/L"), "m");
    let e = q(&ann(case, "inputs/E"), "Pa");
    let mut model = Model::new("project:prescribed-rotation")
        .node("node:root", 0.0)
        .node("node:far", l)
        .material(material("material:beam", e, INVENTED_NU))
        .pipe(
            "pipe:beam",
            "node:root",
            "node:far",
            "material:beam",
            basis_direct(),
            fit_none(),
        )
        .support("support:root", "node:root", "anchor", &ALL);
    let mut state = Case::new(CASE)
        .element("pipe:beam", None, sel_base("material:beam"), th_unchanged())
        .support("support:root", root_motion);
    if far_anchor {
        model = model.support("support:far", "node:far", "anchor", &ALL);
        state = state.support("support:far", &[]);
    }
    model.case(state).request()
}

const SERIAL: &str = "shared_material_serial_companion";
const INVENTED_TORQUE: f64 = 1000.0;

fn serial_model(torsion: bool) -> Value {
    let c = SERIAL;
    let shared = text(&ann(c, "inputs/material_id"));
    let cold = text(&ann(c, "inputs/material_point_cold"));
    let hot = text(&ann(c, "inputs/material_point_hot"));
    let l1 = q(&ann(c, "inputs/L1"), "m");
    let l2 = q(&ann(c, "inputs/L2"), "m");
    let mut shared_material = material(shared, 170.0e9, 0.28); // invented base, used by neither member
    shared_material["temperature_points"] = json!([
        // Invented point temperatures; E/nu come from the fixture.
        point(
            cold,
            20.0,
            "degC",
            q(&ann(c, "inputs/E1"), "Pa"),
            q(&ann(c, "inputs/nu1"), "1")
        ),
        point(
            hot,
            300.0,
            "degC",
            q(&ann(c, "inputs/E2"), "Pa"),
            q(&ann(c, "inputs/nu2"), "1")
        )
    ]);
    let middle_restraints: &[&str] = if torsion {
        &["UY", "UZ", "RY", "RZ"]
    } else {
        &TRANSVERSE
    };
    let mut state = Case::new(CASE)
        .element("pipe:1", None, sel_point(shared, cold), th_unchanged())
        .element(
            "pipe:2",
            None,
            sel_point(shared, hot),
            th_interval(q(&ann(c, "inputs/epsilon2"), "1")),
        )
        .support("support:root", &[])
        .support("support:middle", &[])
        .support("support:far", &[]);
    if torsion {
        state = state
            .primitive(nodal_torque(
                "source:torque",
                "node:middle",
                INVENTED_TORQUE,
            ))
            .source("source:torque", 1.0);
    }
    Model::new("project:shared-material-serial")
        .node("node:root", 0.0)
        .node("node:middle", l1)
        .node("node:far", l1 + l2)
        .material(shared_material)
        .pipe(
            "pipe:1",
            "node:root",
            "node:middle",
            shared,
            basis_direct(),
            fit_none(),
        )
        .pipe(
            "pipe:2",
            "node:middle",
            "node:far",
            shared,
            basis_direct(),
            fit_none(),
        )
        .support("support:root", "node:root", "anchor", &ALL)
        .support("support:middle", "node:middle", "anchor", middle_restraints)
        .support("support:far", "node:far", "anchor", &ALL)
        .case(state)
        .request()
}

const THERMAL: &str = "thermal_datum_ratio";

#[derive(Clone, Copy)]
enum TemperatureUnits {
    Celsius,
    Kelvin,
    Fahrenheit,
}

impl TemperatureUnits {
    fn label(self) -> &'static str {
        match self {
            Self::Celsius => "degC",
            Self::Kelvin => "K",
            Self::Fahrenheit => "degF",
        }
    }

    /// Absolute temperature authored in this unit from a fixture degC value.
    /// K: +273.15 (unit definition; the fixture's own K values are
    /// cross-checked). degF: t*9/5 + 32, exact in binary64 for these inputs.
    fn temperature(self, celsius: f64) -> f64 {
        match self {
            Self::Celsius => celsius,
            Self::Kelvin => celsius + KELVIN_OFFSET,
            Self::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        }
    }

    /// Inverse-interval coefficient authored in a unit paired with this
    /// variant: per K and per degC are the same interval; per degF is 5/9 of it.
    fn coefficient(self, per_kelvin: f64) -> (f64, &'static str) {
        match self {
            Self::Celsius => (per_kelvin, "1/K"),
            Self::Kelvin => (per_kelvin, "1/degC"),
            Self::Fahrenheit => (per_kelvin * 5.0 / 9.0, "1/degF"),
        }
    }
}

/// `reviewed_two_point` keeps only the reviewed VERIFICATION control-4 table
/// points (installation and operating temperatures); the fixture marks its
/// added datum-temperature point as invented coverage data, and the secant
/// definition needs no coefficient at the datum.
fn thermal_model(
    units: TemperatureUnits,
    fixed: bool,
    zero_region: bool,
    reviewed_two_point: bool,
) -> Value {
    let c = THERMAL;
    let l = q(&ann(c, "inputs/L"), "m");
    let e = q(&ann(c, "inputs/E"), "Pa");
    let t_unit = units.label();
    let install_c = q(&ann(c, "inputs/installation_temperature"), "degC");
    let datum = units.temperature(q(&ann(c, "inputs/datum_temperature"), "degC"));
    let install = units.temperature(install_c);
    let operating = units.temperature(q(&ann(c, "inputs/operating_temperature"), "degC"));
    let a_unit = units.coefficient(1.0).1;
    let points: Vec<(f64, f64)> = at(&ann(c, "inputs/table_points"))
        .as_array()
        .expect("table points")
        .iter()
        .map(|p| {
            (
                qval(&p["temperature"], "degC", "table temperature"),
                qval(&p["alpha"], "1/K", "table alpha"),
            )
        })
        .filter(|(t, _)| !reviewed_two_point || *t >= install_c)
        .map(|(t, alpha)| (units.temperature(t), units.coefficient(alpha).0))
        .collect();
    if reviewed_two_point {
        assert_eq!(points.len(), 2, "reviewed control-4 table has two points");
    }
    let mut thermal_material = material("material:thermal", e, INVENTED_NU);
    thermal_material["expansion_laws"] = json!([law_secant_table(
        "law:secant",
        qv(datum, t_unit),
        &points,
        t_unit,
        a_unit
    )]);
    let mut model = Model::new("project:thermal-datum-ratio")
        .node("node:root", 0.0)
        .node("node:far", l)
        .material(thermal_material)
        .pipe(
            "pipe:thermal",
            "node:root",
            "node:far",
            "material:thermal",
            basis_temperature(install, t_unit),
            fit_none(),
        )
        .support("support:root", "node:root", "anchor", &ALL);
    let mut state = Case::new(CASE)
        .element(
            "pipe:thermal",
            Some((operating, t_unit)),
            sel_base("material:thermal"),
            th_free("law:secant"),
        )
        .support("support:root", &[]);
    if fixed {
        model = model.support("support:far", "node:far", "anchor", &ALL);
        state = state.support("support:far", &[]);
    }
    if zero_region {
        state = state.region(zero_pressure_region(
            "region:zero",
            "pipe:thermal",
            "node:root",
            "node:far",
        ));
    }
    model.case(state).request()
}

const CONSTANT_ALPHA: &str = "constant_alpha_interval";

/// Constant-alpha interval on the thermal companion's annular bar (same L/E).
fn constant_alpha_model(fixed: bool, celsius_interval: bool) -> Value {
    let l = q(&ann(THERMAL, "inputs/L"), "m");
    let e = q(&ann(THERMAL, "inputs/E"), "Pa");
    let alpha = q(&generic(CONSTANT_ALPHA, "inputs/alpha"), "1/K");
    let change = q(&generic(CONSTANT_ALPHA, "inputs/deltaT"), "K");
    let (a_unit, dt_unit) = if celsius_interval {
        ("1/degC", "degC")
    } else {
        ("1/K", "K")
    };
    let mut model = Model::new("project:constant-alpha")
        .node("node:root", 0.0)
        .node("node:far", l)
        .material(material("material:interval", e, INVENTED_NU))
        .pipe(
            "pipe:interval",
            "node:root",
            "node:far",
            "material:interval",
            basis_direct(),
            fit_none(),
        )
        .support("support:root", "node:root", "anchor", &ALL);
    // No absolute temperature appears anywhere in this request.
    let mut state = Case::new(CASE)
        .element(
            "pipe:interval",
            None,
            sel_base("material:interval"),
            th_constant_alpha(alpha, a_unit, change, dt_unit),
        )
        .support("support:root", &[]);
    if fixed {
        model = model.support("support:far", "node:far", "anchor", &ALL);
        state = state.support("support:far", &[]);
    }
    model.case(state).request()
}

const FIT: &str = "signed_fit_states";
const FIT_CASES: [&str; 3] = ["case:cold", "case:hot", "case:return"];
const FIT_STATES: [&str; 3] = ["cold", "hot", "return"];

#[derive(Clone, Copy)]
enum FitRoute {
    /// Temperature reference + natural length change (signed factor on the
    /// fixture's cut) + free-length thermal state.
    Length(f64),
    /// Temperature reference, no fit, free-length thermal state.
    NoFit,
    /// Direct-strain reference + direct fit strain + explicit interval strain.
    DirectStrain,
}

fn fit_model(route: FitRoute, fixed: bool, zero_region: bool) -> Value {
    let c = FIT;
    let l = q(&ann(c, "inputs/L"), "m");
    let cut = q(&ann(c, "inputs/signed_fit_length_change"), "m");
    let install = q(&ann(c, "inputs/installation_temperature"), "degC");
    let hot_t = q(&ann(c, "inputs/hot_temperature"), "degC");
    let datum = q(&ann(c, "inputs/coefficient_datum_temperature"), "degC");
    let alpha = q(&ann(c, "inputs/alpha"), "1/K");
    let cold_e = q(&ann(c, "inputs/cold_E"), "Pa");
    let hot_e = q(&ann(c, "inputs/hot_E"), "Pa");
    let mut fit_material = material("material:fit", cold_e, INVENTED_NU);
    fit_material["temperature_points"] = json!([
        point("point:cold", install, "degC", cold_e, INVENTED_NU),
        point("point:hot", hot_t, "degC", hot_e, INVENTED_NU)
    ]);
    fit_material["expansion_laws"] = json!([law_secant_constant(
        "law:constant",
        qv(datum, "degC"),
        qv(alpha, "1/K")
    )]);
    let (basis, fit) = match route {
        // Authored in mm, as in the wire example.
        FitRoute::Length(sign) => (
            basis_temperature(install, "degC"),
            fit_length(sign * cut * 1000.0, "mm"),
        ),
        FitRoute::NoFit => (basis_temperature(install, "degC"), fit_none()),
        FitRoute::DirectStrain => (
            basis_direct(),
            fit_strain(q(&ann(c, "expected/cold/fit_strain"), "1")),
        ),
    };
    let mut model = Model::new("project:signed-fit")
        .node("node:root", 0.0)
        .node("node:far", l)
        .material(fit_material)
        .pipe(
            "pipe:fit",
            "node:root",
            "node:far",
            "material:fit",
            basis,
            fit,
        )
        .support("support:root", "node:root", "anchor", &ALL);
    if fixed {
        model = model.support("support:far", "node:far", "anchor", &ALL);
    } else {
        // Axially released far end; transverse DOFs held (all reactions zero).
        model = model.support("support:far", "node:far", "anchor", &TRANSVERSE);
    }
    for (case_id, state) in FIT_CASES.iter().zip(FIT_STATES) {
        let (operating, point_ref) = if state == "hot" {
            (hot_t, "point:hot")
        } else {
            (install, "point:cold")
        };
        let mut case = Case::new(case_id);
        case = match route {
            FitRoute::Length(_) | FitRoute::NoFit => case.element(
                "pipe:fit",
                Some((operating, "degC")),
                sel_point("material:fit", point_ref),
                th_free("law:constant"),
            ),
            FitRoute::DirectStrain => {
                let strain = q(&ann(c, &format!("expected/{state}/thermal_strain")), "1");
                let thermal = if strain == 0.0 {
                    th_unchanged()
                } else {
                    th_interval(strain)
                };
                case.element(
                    "pipe:fit",
                    None,
                    sel_point("material:fit", point_ref),
                    thermal,
                )
            }
        };
        case = case
            .support("support:root", &[])
            .support("support:far", &[]);
        if zero_region {
            case = case.region(zero_pressure_region(
                &format!("region:{state}"),
                "pipe:fit",
                "node:root",
                "node:far",
            ));
        }
        model = model.case(case);
    }
    model.request()
}

const SOURCE: &str = "persistent_source_once";

fn ledger_force(source_id: &str) -> f64 {
    let entry = at(&format!("cases/{SOURCE}/source_ledger"))
        .as_array()
        .expect("source ledger")
        .iter()
        .find(|e| e["source_id"] == source_id)
        .unwrap_or_else(|| panic!("ledger entry {source_id}"));
    entry["force_N"].as_f64().expect("ledger force_N")
}

fn source_model(primitives: &[(&str, f64)], sources: &[(&str, f64)]) -> Value {
    let l = q(&ann(SOURCE, "inputs/L"), "m");
    let e = q(&ann(SOURCE, "inputs/E"), "Pa");
    let mut state = Case::new(CASE)
        .element(
            "pipe:cantilever",
            None,
            sel_base("material:source"),
            th_unchanged(),
        )
        .support("support:root", &[]);
    for (id, value) in primitives {
        state = state.primitive(nodal_force(id, "node:tip", *value));
    }
    for (id, factor) in sources {
        state = state.source(id, *factor);
    }
    Model::new("project:persistent-source")
        .node("node:root", 0.0)
        .node("node:tip", l)
        .material(material("material:source", e, INVENTED_NU))
        .pipe(
            "pipe:cantilever",
            "node:root",
            "node:tip",
            "material:source",
            basis_direct(),
            fit_none(),
        )
        .support("support:root", "node:root", "anchor", &ALL)
        .case(state)
        .request()
}

fn state_mut<'a>(request: &'a mut Value, case: usize) -> &'a mut Value {
    &mut request["model"]["load_cases"][case]["analysis_state"]
}

// ---------------------------------------------------------------------------
// 1-2. Prescribed translation
// ---------------------------------------------------------------------------

#[test]
fn prescribed_translation_two_bar_couples_prescribed_root_motion() {
    let c = PRESCRIBED_TWO_BAR;
    assert_annular_geometry(c);
    let a = annulus();
    let l1 = q(&ann(c, "inputs/L1"), "m");
    let l2 = q(&ann(c, "inputs/L2"), "m");
    let e1 = q(&ann(c, "inputs/E1"), "Pa");
    let root_ux = q(&ann(c, "inputs/root_UX"), "m");
    let far_ux = q(&ann(c, "inputs/far_UX"), "m");
    let middle = q(&ann(c, "expected/middle_UX"), "m");
    let n1 = q(&ann(c, "expected/member1_N"), "N");
    let n2 = q(&ann(c, "expected/member2_N"), "N");
    let root_fx = q(&ann(c, "expected/root_Fx"), "N");
    let far_fx = q(&ann(c, "expected/far_Fx"), "N");
    let omitted_middle = q(
        &format!("cases/{c}/wrong_result_discriminators/omitted_Kfc_gc/middle_UX"),
        "m",
    );
    // Reduced-system-only root action K_cf*u_f - f_c: the root row couples to
    // the free middle UX through -k1, and no load is applied at the root, so
    // f_c = 0. It omits K_cc*g_c = k1*g_root.
    let reduced_only_root_fx = -(e1 * a.area / l1) * middle;
    let force = root_fx.abs(); // case force scale for exact-zero components
    let moment = force * (l1 + l2);
    for mode in MODES {
        for reversed in [false, true] {
            for far_explicit in [false, true] {
                let out = solve(&two_bar_model(reversed, far_explicit), mode);
                let ctx = format!(
                    "{} reversed={reversed} far_explicit={far_explicit}",
                    mode.as_str()
                );
                // Complete u includes the prescribed values.
                close(
                    displacement(&out, CASE, "node:root", "x"),
                    root_ux,
                    root_ux,
                    &format!("{ctx}: root UX"),
                );
                close(
                    displacement(&out, CASE, "node:middle", "x"),
                    middle,
                    root_ux,
                    &format!("{ctx}: middle UX"),
                );
                close(
                    displacement(&out, CASE, "node:far", "x"),
                    far_ux,
                    root_ux,
                    &format!("{ctx}: far UX"),
                );
                member_axial(&out, CASE, "pipe:1", n1, force, &ctx);
                member_axial(&out, CASE, "pipe:2", n2, force, &ctx);
                let observed_root = reaction(&out, CASE, "support:root", "Fx");
                close(observed_root, root_fx, force, &format!("{ctx}: root Fx"));
                close(
                    reaction(&out, CASE, "support:far", "Fx"),
                    far_fx,
                    force,
                    &format!("{ctx}: far Fx"),
                );
                for support in ["support:root", "support:far"] {
                    zero_other_reactions(&out, CASE, support, &["Fx"], force, moment, &ctx);
                }
                differs(
                    observed_root,
                    root_fx,
                    reduced_only_root_fx,
                    force,
                    &format!("{ctx}: root Fx vs K_cf*u_f - f_c"),
                );
                differs(
                    displacement(&out, CASE, "node:middle", "x"),
                    middle,
                    omitted_middle,
                    root_ux,
                    &format!("{ctx}: middle UX vs omitted K_fc*g_c"),
                );
                let id = &row(
                    &out,
                    CASE,
                    "global_nodal_displacement_x",
                    "node:middle",
                    "nodal_displacement_x",
                    None,
                )
                .id;
                assert!(
                    id.starts_with("result:disp:"),
                    "{ctx}: exact-profile displacement id {id}"
                );
            }
        }
    }
}

#[test]
fn prescribed_translation_all_fixed_single_bar_has_nonzero_reactions() {
    let c = "prescribed_translation_all_fixed";
    assert_annular_geometry(c);
    let l = q(&ann(c, "inputs/L"), "m");
    let root_ux = q(&ann(c, "inputs/root_UX"), "m");
    let far_ux = q(&ann(c, "inputs/far_UX"), "m");
    assert_eq!(far_ux, 0.0);
    assert!(at(&ann(c, "inputs/free_dofs"))
        .as_array()
        .unwrap()
        .is_empty());
    let n = q(&ann(c, "expected/N"), "N");
    let root_fx = q(&ann(c, "expected/root_Fx"), "N");
    let far_fx = q(&ann(c, "expected/far_Fx"), "N");
    let force = root_fx.abs();
    for mode in MODES {
        // Zero free DOFs: every DOF is restrained; the prescribed tuple alone drives the actions.
        let out = solve(&single_bar_model(&[("UX", root_ux, "m")]), mode);
        let ctx = mode.as_str().to_string();
        close(
            displacement(&out, CASE, "node:root", "x"),
            root_ux,
            root_ux,
            &format!("{ctx}: root UX"),
        );
        close(
            displacement(&out, CASE, "node:far", "x"),
            far_ux,
            root_ux,
            &format!("{ctx}: far UX"),
        );
        member_axial(&out, CASE, "pipe:bar", n, force, &ctx);
        let observed_root = reaction(&out, CASE, "support:root", "Fx");
        close(observed_root, root_fx, force, &format!("{ctx}: root Fx"));
        close(
            reaction(&out, CASE, "support:far", "Fx"),
            far_fx,
            force,
            &format!("{ctx}: far Fx"),
        );
        for support in ["support:root", "support:far"] {
            zero_other_reactions(&out, CASE, support, &["Fx"], force, force * l, &ctx);
        }
        // zero_size_free_problem_discarded: a zero-size free system is not a zero result.
        differs(
            observed_root,
            root_fx,
            0.0,
            force,
            &format!("{ctx}: root Fx vs discarded zero-size solve"),
        );
    }
}

// ---------------------------------------------------------------------------
// 3. Prescribed rotation
// ---------------------------------------------------------------------------

#[test]
fn prescribed_rotation_all_fixed_reproduces_signed_end_actions() {
    let c = "prescribed_rotation_all_fixed";
    assert_annular_geometry(c);
    let l = q(&ann(c, "inputs/L"), "m");
    let theta = q(&ann(c, "inputs/root_RZ"), "rad");
    assert_eq!(q(&ann(c, "inputs/root_UY"), "m"), 0.0);
    assert_eq!(q(&ann(c, "inputs/far_UY"), "m"), 0.0);
    assert_eq!(q(&ann(c, "inputs/far_RZ"), "rad"), 0.0);
    close(
        q(&ann(c, "inputs/E"), "Pa") * q("geometry/derived/I", "m^4"),
        q(&ann(c, "inputs/EI"), "N*m^2"),
        0.0,
        "fixture EI = E*I",
    );
    let root_fy = q(&ann(c, "expected/root_Fy"), "N");
    let root_mz = q(&ann(c, "expected/root_Mz"), "N*m");
    let far_fy = q(&ann(c, "expected/far_Fy"), "N");
    let far_mz = q(&ann(c, "expected/far_Mz"), "N*m");
    let force = root_fy.abs();
    let moment = root_mz.abs();
    for mode in MODES {
        let out = solve(&beam_model(c, true, &[("RZ", theta, "rad")]), mode);
        let ctx = mode.as_str().to_string();
        close(
            rotation(&out, CASE, "node:root", "z"),
            theta,
            theta,
            &format!("{ctx}: root RZ"),
        );
        close(
            rotation(&out, CASE, "node:far", "z"),
            0.0,
            theta,
            &format!("{ctx}: far RZ"),
        );
        let r_fy = reaction(&out, CASE, "support:root", "Fy");
        let r_mz = reaction(&out, CASE, "support:root", "Mz");
        let f_fy = reaction(&out, CASE, "support:far", "Fy");
        let f_mz = reaction(&out, CASE, "support:far", "Mz");
        close(r_fy, root_fy, force, &format!("{ctx}: root Fy"));
        close(r_mz, root_mz, moment, &format!("{ctx}: root Mz"));
        close(f_fy, far_fy, force, &format!("{ctx}: far Fy"));
        close(f_mz, far_mz, moment, &format!("{ctx}: far Mz"));
        for support in ["support:root", "support:far"] {
            zero_other_reactions(&out, CASE, support, &["Fy", "Mz"], force, moment, &ctx);
        }
        // translation_substitution_or_length_normalized_angle: signed force and moment balance.
        close(r_fy + f_fy, 0.0, force, &format!("{ctx}: sum Fy"));
        close(
            r_mz + f_mz + f_fy * l,
            0.0,
            moment,
            &format!("{ctx}: sum Mz about root"),
        );
    }
}

#[test]
fn prescribed_rotation_free_tip_is_rigid_radian_rotation() {
    let c = "prescribed_rotation_free_tip";
    assert_annular_geometry(c);
    let theta = q(&ann(c, "inputs/root_RZ"), "rad");
    let tip_uy = q(&ann(c, "expected/tip_UY"), "m");
    let tip_rz = q(&ann(c, "expected/tip_RZ"), "rad");
    let root_fy = q(&ann(c, "expected/root_Fy"), "N");
    let root_mz = q(&ann(c, "expected/root_Mz"), "N*m");
    let wall_n = q(&ann(c, "expected/wall_N"), "N");
    let d = format!("cases/{c}/wrong_result_discriminators");
    let missing_uy = q(&format!("{d}/missing_rotation_coupling/wrong_tip_UY"), "m");
    let normalized_uy = q(&format!("{d}/length_normalized_angle/wrong_tip_UY"), "m");
    let normalized_rz = q(&format!("{d}/length_normalized_angle/wrong_tip_RZ"), "rad");
    // Exact-zero actions: the same beam's all-fixed end actions are the case's
    // natural force and moment magnitudes.
    let force = q(
        &ann("prescribed_rotation_all_fixed", "expected/root_Fy"),
        "N",
    )
    .abs();
    let moment = q(
        &ann("prescribed_rotation_all_fixed", "expected/root_Mz"),
        "N*m",
    )
    .abs();
    let l = q(&ann(c, "inputs/L"), "m");
    for mode in MODES {
        let out = solve(&beam_model(c, false, &[("RZ", theta, "rad")]), mode);
        let ctx = mode.as_str().to_string();
        let uy = displacement(&out, CASE, "node:far", "y");
        let rz = rotation(&out, CASE, "node:far", "z");
        close(uy, tip_uy, tip_uy, &format!("{ctx}: tip UY"));
        close(rz, tip_rz, tip_rz, &format!("{ctx}: tip RZ"));
        close(
            displacement(&out, CASE, "node:far", "x"),
            0.0,
            tip_uy,
            &format!("{ctx}: tip UX"),
        );
        close(
            reaction(&out, CASE, "support:root", "Fy"),
            root_fy,
            force,
            &format!("{ctx}: root Fy"),
        );
        close(
            reaction(&out, CASE, "support:root", "Mz"),
            root_mz,
            moment,
            &format!("{ctx}: root Mz"),
        );
        zero_other_reactions(
            &out,
            CASE,
            "support:root",
            &["Fy", "Mz"],
            force,
            moment,
            &ctx,
        );
        member_axial(&out, CASE, "pipe:beam", wall_n, force, &ctx);
        for (kind, component, scale) in [
            ("element_local_shear_force_y", "shear_force_y", force),
            ("element_local_shear_force_z", "shear_force_z", force),
            ("element_local_torsional_moment", "torsional_moment", moment),
            ("element_local_bending_moment_y", "bending_moment_y", moment),
            ("element_local_bending_moment_z", "bending_moment_z", moment),
        ] {
            let r = row(&out, CASE, kind, "pipe:beam", component, Some("midspan"));
            close(r.value, 0.0, scale, &format!("{ctx}: midspan {component}"));
        }
        differs(
            uy,
            tip_uy,
            missing_uy,
            tip_uy,
            &format!("{ctx}: tip UY vs missing rotation coupling"),
        );
        differs(
            uy,
            tip_uy,
            normalized_uy,
            tip_uy,
            &format!("{ctx}: tip UY vs length-normalized angle"),
        );
        differs(
            rz,
            tip_rz,
            normalized_rz,
            tip_rz,
            &format!("{ctx}: tip RZ vs length-normalized angle"),
        );
        close(tip_uy, l * theta, 0.0, "fixture tip UY = L*theta");
    }
}

// ---------------------------------------------------------------------------
// 4. Shared material, serial companion
// ---------------------------------------------------------------------------

#[test]
fn shared_material_serial_companion_selects_point_per_member() {
    let c = SERIAL;
    assert_annular_geometry(c);
    let a = annulus();
    assert_eq!(
        q(&ann(c, "inputs/epsilon1"), "1"),
        0.0,
        "member 1 is the unchanged reference"
    );
    assert_eq!(q(&ann(c, "inputs/root_UX"), "m"), 0.0);
    assert_eq!(q(&ann(c, "inputs/far_UX"), "m"), 0.0);
    let l1 = q(&ann(c, "inputs/L1"), "m");
    let l2 = q(&ann(c, "inputs/L2"), "m");
    let (e1, e2) = (q(&ann(c, "inputs/E1"), "Pa"), q(&ann(c, "inputs/E2"), "Pa"));
    let (nu1, nu2) = (q(&ann(c, "inputs/nu1"), "1"), q(&ann(c, "inputs/nu2"), "1"));
    let (g1, g2) = (
        q(&ann(c, "expected/G1"), "Pa"),
        q(&ann(c, "expected/G2"), "Pa"),
    );
    let eps2 = q(&ann(c, "inputs/epsilon2"), "1");
    let middle = q(&ann(c, "expected/middle_UX"), "m");
    let n1 = q(&ann(c, "expected/member1_N"), "N");
    let n2 = q(&ann(c, "expected/member2_N"), "N");
    let root_fx = q(&ann(c, "expected/root_Fx"), "N");
    let far_fx = q(&ann(c, "expected/far_Fx"), "N");
    let wrong_middle = q(
        &format!(
            "cases/{c}/wrong_result_discriminators/material_ID_only_E_override/wrong_middle_UX"
        ),
        "m",
    );
    let force = root_fx.abs();
    let length_scale = middle.abs();
    for mode in MODES {
        for torsion in [false, true] {
            let out = solve(&serial_model(torsion), mode);
            let ctx = format!("{} torsion={torsion}", mode.as_str());
            let observed_middle = displacement(&out, CASE, "node:middle", "x");
            close(
                observed_middle,
                middle,
                length_scale,
                &format!("{ctx}: middle UX"),
            );
            member_axial(&out, CASE, "pipe:1", n1, force, &ctx);
            member_axial(&out, CASE, "pipe:2", n2, force, &ctx);
            close(
                reaction(&out, CASE, "support:root", "Fx"),
                root_fx,
                force,
                &format!("{ctx}: root Fx"),
            );
            close(
                reaction(&out, CASE, "support:far", "Fx"),
                far_fx,
                force,
                &format!("{ctx}: far Fx"),
            );
            differs(
                observed_middle,
                middle,
                wrong_middle,
                length_scale,
                &format!("{ctx}: middle UX vs material-ID-only E"),
            );
            let record = evidence(&out, CASE);
            for (pipe, e, nu, g, strain) in
                [("pipe:1", e1, nu1, g1, 0.0), ("pipe:2", e2, nu2, g2, eps2)]
            {
                let m = member_evidence(record, pipe);
                close(
                    num(&m["selected_E_pa"], pipe),
                    e,
                    0.0,
                    &format!("{ctx}: {pipe} selected E"),
                );
                close(
                    num(&m["selected_nu"], pipe),
                    nu,
                    0.0,
                    &format!("{ctx}: {pipe} selected nu"),
                );
                close(
                    num(&m["derived_G_pa"], pipe),
                    g,
                    0.0,
                    &format!("{ctx}: {pipe} derived G"),
                );
                close(
                    num(&m["total_eigenstrain"], pipe),
                    strain,
                    eps2,
                    &format!("{ctx}: {pipe} total eigenstrain"),
                );
            }
            if torsion {
                // Invented torque at the free middle RX; per-member derived G
                // from the fixture: theta = T / (G1*J/L1 + G2*J/L2).
                let k1 = g1 * a.polar / l1;
                let k2 = g2 * a.polar / l2;
                let theta = INVENTED_TORQUE / (k1 + k2);
                close(
                    rotation(&out, CASE, "node:middle", "x"),
                    theta,
                    theta,
                    &format!("{ctx}: middle RX"),
                );
                close(
                    reaction(&out, CASE, "support:root", "Mx"),
                    -k1 * theta,
                    INVENTED_TORQUE,
                    &format!("{ctx}: root Mx"),
                );
                close(
                    reaction(&out, CASE, "support:far", "Mx"),
                    -k2 * theta,
                    INVENTED_TORQUE,
                    &format!("{ctx}: far Mx"),
                );
            } else {
                for support in ["support:root", "support:far"] {
                    zero_other_reactions(
                        &out,
                        CASE,
                        support,
                        &["Fx"],
                        force,
                        force * (l1 + l2),
                        &ctx,
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 5. Thermal datum ratio and constant-alpha interval
// ---------------------------------------------------------------------------

#[test]
fn thermal_datum_ratio_secant_table_fixed_and_free() {
    let c = THERMAL;
    assert_annular_geometry(c);
    assert_eq!(
        text(&ann(c, "inputs/coefficient_definition")),
        "engineering_secant"
    );
    assert_eq!(text(&ann(c, "inputs/interpolation")), "linear_coefficient");
    for (name, kelvin) in [
        ("datum_temperature", "datum_temperature_K"),
        ("installation_temperature", "installation_temperature_K"),
        ("operating_temperature", "operating_temperature_K"),
    ] {
        close(
            q(&ann(c, &format!("inputs/{name}")), "degC") + KELVIN_OFFSET,
            q(&ann(c, &format!("expected/{kelvin}")), "K"),
            0.0,
            &format!("fixture {name} in K"),
        );
    }
    let a = annulus();
    let l = q(&ann(c, "inputs/L"), "m");
    let e = q(&ann(c, "inputs/E"), "Pa");
    let strain = q(&ann(c, "expected/thermal_strain"), "1");
    let free_tip = q(&ann(c, "expected/free_tip_UX"), "m");
    let free_wall = q(&ann(c, "expected/free_wall_N"), "N");
    let fixed_wall = q(&ann(c, "expected/fixed_wall_N"), "N");
    let fixed_root = q(&ann(c, "expected/fixed_root_Fx"), "N");
    let fixed_far = q(&ann(c, "expected/fixed_far_Fx"), "N");
    let d = format!("cases/{c}/wrong_result_discriminators");
    let wrong_strains = [
        (
            "alpha_hot_times_operating_minus_install",
            q(&format!("{d}/alpha_hot_times_operating_minus_install"), "1"),
        ),
        (
            "subtract_datum_dilations",
            q(&format!("{d}/subtract_datum_dilations"), "1"),
        ),
    ];
    let force = fixed_wall.abs();
    for mode in MODES {
        // Equivalent degC/K/degF absolute inputs and the reviewed two-point
        // table give identical normalized mechanics (VERIFICATION control 4).
        for (units, two_point) in [
            (TemperatureUnits::Celsius, false),
            (TemperatureUnits::Kelvin, false),
            (TemperatureUnits::Fahrenheit, false),
            (TemperatureUnits::Celsius, true),
            (TemperatureUnits::Kelvin, true),
        ] {
            let unit_label = format!("{} two_point={two_point}", units.label());
            for zero_region in [false, true] {
                let out = solve(&thermal_model(units, true, zero_region, two_point), mode);
                let ctx = format!(
                    "{} {unit_label} fixed zero_region={zero_region}",
                    mode.as_str()
                );
                if zero_region {
                    wall_axial(&out, CASE, "pipe:thermal", fixed_wall, force, &ctx);
                } else {
                    member_axial(&out, CASE, "pipe:thermal", fixed_wall, force, &ctx);
                    let n = midspan_axial(&out, CASE, "pipe:thermal");
                    for (name, wrong) in wrong_strains {
                        differs(
                            n,
                            fixed_wall,
                            -e * a.area * wrong,
                            force,
                            &format!("{ctx}: fixed N vs {name}"),
                        );
                    }
                }
                close(
                    reaction(&out, CASE, "support:root", "Fx"),
                    fixed_root,
                    force,
                    &format!("{ctx}: root Fx"),
                );
                close(
                    reaction(&out, CASE, "support:far", "Fx"),
                    fixed_far,
                    force,
                    &format!("{ctx}: far Fx"),
                );
                close(
                    displacement(&out, CASE, "node:far", "x"),
                    0.0,
                    free_tip,
                    &format!("{ctx}: far UX"),
                );
                let m = member_evidence(evidence(&out, CASE), "pipe:thermal");
                close(
                    num(&m["thermal_strain"], "thermal_strain"),
                    strain,
                    strain,
                    &format!("{ctx}: evidence thermal strain"),
                );
                close(
                    num(&m["fit_strain"], "fit_strain"),
                    0.0,
                    strain,
                    &format!("{ctx}: evidence fit strain"),
                );
                close(
                    num(&m["total_eigenstrain"], "total"),
                    strain,
                    strain,
                    &format!("{ctx}: evidence eigenstrain"),
                );
                close(
                    num(&m["selected_E_pa"], "E"),
                    e,
                    0.0,
                    &format!("{ctx}: evidence E"),
                );
            }
            let out = solve(&thermal_model(units, false, false, two_point), mode);
            let ctx = format!("{} {unit_label} free tip", mode.as_str());
            let tip = displacement(&out, CASE, "node:far", "x");
            close(tip, free_tip, free_tip, &format!("{ctx}: tip UX"));
            member_axial(&out, CASE, "pipe:thermal", free_wall, force, &ctx);
            close(
                reaction(&out, CASE, "support:root", "Fx"),
                0.0,
                force,
                &format!("{ctx}: root Fx"),
            );
            for (name, wrong) in wrong_strains {
                differs(
                    tip,
                    free_tip,
                    l * wrong,
                    free_tip,
                    &format!("{ctx}: tip UX vs {name}"),
                );
            }
        }
    }
}

#[test]
fn constant_alpha_interval_needs_no_absolute_temperature() {
    let c = CONSTANT_ALPHA;
    assert_eq!(text(&generic(c, "inputs/meaning")), "engineering_interval");
    let a = annulus();
    let l = q(&ann(THERMAL, "inputs/L"), "m");
    let e = q(&ann(THERMAL, "inputs/E"), "Pa");
    let strain = q(&generic(c, "expected/thermal_strain"), "1");
    // Annular observations derived from the maintained strain on the same bar.
    let fixed_n = -e * a.area * strain;
    let free_tip = l * strain;
    let force = fixed_n.abs();
    for mode in MODES {
        for celsius_interval in [false, true] {
            let ctx = format!("{} degC-interval={celsius_interval}", mode.as_str());
            let out = solve(&constant_alpha_model(true, celsius_interval), mode);
            member_axial(&out, CASE, "pipe:interval", fixed_n, force, &ctx);
            close(
                reaction(&out, CASE, "support:root", "Fx"),
                -fixed_n,
                force,
                &format!("{ctx}: root Fx"),
            );
            close(
                reaction(&out, CASE, "support:far", "Fx"),
                fixed_n,
                force,
                &format!("{ctx}: far Fx"),
            );
            let m = member_evidence(evidence(&out, CASE), "pipe:interval");
            close(
                num(&m["thermal_strain"], "thermal"),
                strain,
                strain,
                &format!("{ctx}: evidence thermal strain"),
            );
            close(
                num(&m["total_eigenstrain"], "total"),
                strain,
                strain,
                &format!("{ctx}: evidence eigenstrain"),
            );
            let out = solve(&constant_alpha_model(false, celsius_interval), mode);
            close(
                displacement(&out, CASE, "node:far", "x"),
                free_tip,
                free_tip,
                &format!("{ctx}: free tip UX"),
            );
            member_axial(
                &out,
                CASE,
                "pipe:interval",
                0.0,
                force,
                &format!("{ctx} free"),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 6. Signed fit
// ---------------------------------------------------------------------------

struct FitExpected {
    e: f64,
    thermal: f64,
    fit: f64,
    total: f64,
    fixed_n: f64,
    root: f64,
    far: f64,
    released_tip: f64,
    released_n: f64,
}

fn fit_expected(state: &str) -> FitExpected {
    let p = |name: &str| ann(FIT, &format!("expected/{state}/{name}"));
    FitExpected {
        e: q(&p("E"), "Pa"),
        thermal: q(&p("thermal_strain"), "1"),
        fit: q(&p("fit_strain"), "1"),
        total: q(&p("total_eigenstrain"), "1"),
        fixed_n: q(&p("fixed_wall_N"), "N"),
        root: q(&p("fixed_root_Fx"), "N"),
        far: q(&p("fixed_far_Fx"), "N"),
        released_tip: q(&p("released_tip_UX"), "m"),
        released_n: q(&p("released_wall_N"), "N"),
    }
}

/// Generic-area discriminators scaled to the annulus by As/area (both fixture values).
fn annular_scale() -> f64 {
    annulus().area / q(&generic(FIT, "inputs/area"), "m^2")
}

/// Re-author the installation and every operating temperature in K while
/// the property points and the law datum stay in degC. Exact temperature
/// identity across units must select the same points without an override.
fn fit_states_in_kelvin(request: &mut Value) {
    fn to_kelvin(quantity: &mut Value) {
        assert_eq!(quantity["unit"], "degC");
        let celsius = quantity["value"].as_f64().expect("temperature value");
        *quantity = qv(TemperatureUnits::Kelvin.temperature(celsius), "K");
    }
    to_kelvin(
        &mut request["model"]["reference_configurations"][0]["member_references"][0]["basis"]
            ["installation_temperature"],
    );
    for case in request["model"]["load_cases"].as_array_mut().unwrap() {
        to_kelvin(&mut case["analysis_state"]["element_states"][0]["operating_temperature"]);
    }
}

#[test]
fn signed_fit_cold_hot_return_fixed_and_released() {
    let c = FIT;
    assert_annular_geometry(c);
    let scale = annular_scale();
    let d = format!("cases/{c}/wrong_result_discriminators");
    let hot_additive = q(&format!("{d}/hot_additive_strains/wrong_generic_N"), "N") * scale;
    let hot_cold_e = q(&format!("{d}/hot_uses_cold_E/wrong_generic_N"), "N") * scale;
    let cold_twice = q(&format!("{d}/fit_applied_twice/wrong_cold_generic_N"), "N") * scale;
    let cold_flipped = q(&format!("{d}/sign_flipped/wrong_cold_generic_N"), "N") * scale;
    let expected: Vec<FitExpected> = FIT_STATES.iter().map(|s| fit_expected(s)).collect();
    let force = expected[1].fixed_n.abs(); // largest action of the three states
    for mode in MODES {
        for (zero_region, kelvin) in [(false, false), (true, false), (false, true)] {
            let mut request = fit_model(FitRoute::Length(1.0), true, zero_region);
            if kelvin {
                fit_states_in_kelvin(&mut request);
            }
            let out = solve(&request, mode);
            let ctx = format!(
                "{} fixed zero_region={zero_region} kelvin={kelvin}",
                mode.as_str()
            );
            let mut observed = Vec::new();
            for ((case, state), x) in FIT_CASES.iter().zip(FIT_STATES).zip(&expected) {
                let sctx = format!("{ctx} {state}");
                if zero_region {
                    wall_axial(&out, case, "pipe:fit", x.fixed_n, force, &sctx);
                } else {
                    member_axial(&out, case, "pipe:fit", x.fixed_n, force, &sctx);
                }
                let root = reaction(&out, case, "support:root", "Fx");
                let far = reaction(&out, case, "support:far", "Fx");
                close(root, x.root, force, &format!("{sctx}: root Fx"));
                close(far, x.far, force, &format!("{sctx}: far Fx"));
                let m = member_evidence(evidence(&out, case), "pipe:fit");
                let strain_scale = expected[1].total.abs();
                close(
                    num(&m["selected_E_pa"], "E"),
                    x.e,
                    0.0,
                    &format!("{sctx}: evidence E"),
                );
                close(
                    num(&m["thermal_strain"], "th"),
                    x.thermal,
                    strain_scale,
                    &format!("{sctx}: evidence thermal"),
                );
                close(
                    num(&m["fit_strain"], "fit"),
                    x.fit,
                    strain_scale,
                    &format!("{sctx}: evidence fit"),
                );
                close(
                    num(&m["total_eigenstrain"], "total"),
                    x.total,
                    strain_scale,
                    &format!("{sctx}: evidence total"),
                );
                let n = if zero_region {
                    row(
                        &out,
                        case,
                        "pipe_wall_axial_force_v2",
                        "pipe:fit",
                        "wall_axial_force",
                        Some("midspan"),
                    )
                    .value
                } else {
                    midspan_axial(&out, case, "pipe:fit")
                };
                observed.push((n, root, far));
            }
            differs(
                observed[1].0,
                expected[1].fixed_n,
                hot_additive,
                force,
                &format!("{ctx}: hot N vs additive strains"),
            );
            differs(
                observed[1].0,
                expected[1].fixed_n,
                hot_cold_e,
                force,
                &format!("{ctx}: hot N vs cold E"),
            );
            differs(
                observed[0].0,
                expected[0].fixed_n,
                cold_twice,
                force,
                &format!("{ctx}: cold N vs fit applied twice"),
            );
            differs(
                observed[0].0,
                expected[0].fixed_n,
                cold_flipped,
                force,
                &format!("{ctx}: cold N vs sign flipped"),
            );
            // Return reproduces cold exactly: no accumulated second cut.
            assert_eq!(
                observed[2].0.to_bits(),
                observed[0].0.to_bits(),
                "{ctx}: return N == cold N"
            );
            assert_eq!(
                observed[2].1.to_bits(),
                observed[0].1.to_bits(),
                "{ctx}: return root == cold root"
            );
            assert_eq!(
                observed[2].2.to_bits(),
                observed[0].2.to_bits(),
                "{ctx}: return far == cold far"
            );
        }
        let out = solve(&fit_model(FitRoute::Length(1.0), false, false), mode);
        let ctx = format!("{} released", mode.as_str());
        let mut tips = Vec::new();
        for ((case, state), x) in FIT_CASES.iter().zip(FIT_STATES).zip(&expected) {
            let tip = displacement(&out, case, "node:far", "x");
            close(
                tip,
                x.released_tip,
                x.released_tip,
                &format!("{ctx} {state}: tip UX"),
            );
            member_axial(
                &out,
                case,
                "pipe:fit",
                x.released_n,
                x.fixed_n.abs(),
                &format!("{ctx} {state}"),
            );
            close(
                reaction(&out, case, "support:root", "Fx"),
                0.0,
                x.fixed_n.abs(),
                &format!("{ctx} {state}: root Fx"),
            );
            tips.push(tip);
        }
        assert_eq!(
            tips[2].to_bits(),
            tips[0].to_bits(),
            "{ctx}: return tip == cold tip"
        );
    }
}

#[test]
fn signed_fit_baselines_cut_long_and_direct_fit_strain() {
    let c = FIT;
    let b = |name: &str| q(&ann(c, &format!("baselines/{name}")), "N");
    let (no_fit_cold, no_fit_hot, cut_long_cold) =
        (b("no_fit_cold_N"), b("no_fit_hot_N"), b("cut_long_cold_N"));
    let expected: Vec<FitExpected> = FIT_STATES.iter().map(|s| fit_expected(s)).collect();
    let force = no_fit_hot.abs();
    for mode in MODES {
        let ctx = mode.as_str();
        // No-fit baseline: cold/return stress-free, hot fully restrained thermal.
        let out = solve(&fit_model(FitRoute::NoFit, true, false), mode);
        for (case, n) in FIT_CASES.iter().zip([no_fit_cold, no_fit_hot, no_fit_cold]) {
            member_axial(
                &out,
                case,
                "pipe:fit",
                n,
                force,
                &format!("{ctx} no-fit {case}"),
            );
        }
        // Cut long: the opposite signed length change gives compression.
        let out = solve(&fit_model(FitRoute::Length(-1.0), true, false), mode);
        member_axial(
            &out,
            "case:cold",
            "pipe:fit",
            cut_long_cold,
            force,
            &format!("{ctx} cut-long cold"),
        );
        member_axial(
            &out,
            "case:return",
            "pipe:fit",
            cut_long_cold,
            force,
            &format!("{ctx} cut-long return"),
        );
        // Direct fit strain (delta_L/L) and explicit interval strain: the
        // alternative single representation reproduces the same states.
        let out = solve(&fit_model(FitRoute::DirectStrain, true, false), mode);
        for ((case, state), x) in FIT_CASES.iter().zip(FIT_STATES).zip(&expected) {
            member_axial(
                &out,
                case,
                "pipe:fit",
                x.fixed_n,
                force,
                &format!("{ctx} direct {state}"),
            );
            close(
                reaction(&out, case, "support:root", "Fx"),
                x.root,
                force,
                &format!("{ctx} direct {state}: root Fx"),
            );
        }
        let out = solve(&fit_model(FitRoute::DirectStrain, false, false), mode);
        for ((case, state), x) in FIT_CASES.iter().zip(FIT_STATES).zip(&expected) {
            close(
                displacement(&out, case, "node:far", "x"),
                x.released_tip,
                x.released_tip,
                &format!("{ctx} direct released {state}: tip UX"),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 7. Persistent source once
// ---------------------------------------------------------------------------

#[test]
fn persistent_source_counted_once_and_unreferenced_excluded() {
    let c = SOURCE;
    assert_annular_geometry(c);
    let a = annulus();
    let k = q(&ann(c, "inputs/structure_stiffness"), "N/m");
    close(
        q(&ann(c, "inputs/E"), "Pa") * a.area / q(&ann(c, "inputs/L"), "m"),
        k,
        0.0,
        "fixture k = E*As/L",
    );
    let preload = q(&ann(c, "inputs/preload"), "N");
    let weight = q(&ann(c, "inputs/weight"), "N");
    let independent = q(&ann(c, "inputs/independent_action"), "N");
    let stored = ledger_force("source:stored-unused");
    let combined_rhs = q(&ann(c, "expected/combined_rhs"), "N");
    let tip = q(&ann(c, "expected/combined_displacement"), "m");
    let root_fx = q(&ann(c, "expected/root_Fx"), "N");
    let naive_rhs = q(&ann(c, "expected/naive_total_sum_rhs"), "N");
    let naive_tip = q(&ann(c, "expected/naive_total_sum_displacement"), "m");
    let implicit_rhs = combined_rhs + stored;
    let request = source_model(
        &[
            ("source:preload", preload),
            ("source:weight", weight),
            ("source:independent", independent),
            ("source:stored-unused", stored),
        ],
        &[
            ("source:preload", 1.0),
            ("source:weight", 1.0),
            ("source:independent", 1.0),
        ],
    );
    let force = root_fx.abs();
    for mode in MODES {
        let out = solve(&request, mode);
        let ctx = mode.as_str();
        let observed_tip = displacement(&out, CASE, "node:tip", "x");
        let observed_root = reaction(&out, CASE, "support:root", "Fx");
        close(observed_tip, tip, tip, &format!("{ctx}: tip UX"));
        close(observed_root, root_fx, force, &format!("{ctx}: root Fx"));
        member_axial(&out, CASE, "pipe:cantilever", combined_rhs, force, ctx);
        differs(
            observed_tip,
            tip,
            naive_tip,
            tip,
            &format!("{ctx}: tip vs duplicated preload"),
        );
        differs(
            observed_root,
            root_fx,
            -naive_rhs,
            force,
            &format!("{ctx}: root vs duplicated preload"),
        );
        differs(
            observed_tip,
            tip,
            implicit_rhs / k,
            tip,
            &format!("{ctx}: tip vs implicit +999"),
        );
        differs(
            observed_root,
            root_fx,
            -implicit_rhs,
            force,
            &format!("{ctx}: root vs implicit +999"),
        );
        let record = evidence(&out, CASE);
        assert!(
            mentions(&record["excluded_sources"], "source:stored-unused"),
            "{ctx}: unreferenced primitive reported as excluded: {}",
            record["excluded_sources"]
        );
        for used in ["source:preload", "source:weight", "source:independent"] {
            assert!(
                !mentions(&record["excluded_sources"], used),
                "{ctx}: {used} wrongly excluded"
            );
            assert!(
                mentions(&record["contributions"], used),
                "{ctx}: {used} missing from contributions"
            );
        }
        assert!(
            !mentions(&record["contributions"], "source:stored-unused"),
            "{ctx}: excluded primitive contributed"
        );
    }
}

#[test]
fn duplicate_source_ref_blocks_and_distinct_equal_sources_both_apply() {
    let c = SOURCE;
    let k = q(&ann(c, "inputs/structure_stiffness"), "N/m");
    let weight = q(&ann(c, "inputs/weight"), "N");
    let action = q(&ann(c, "inputs/independent_action"), "N");
    let pair_rhs = q(&ann(c, "expected/two_distinct_equal_actions_rhs"), "N");
    close(
        2.0 * action,
        pair_rhs,
        0.0,
        "fixture pair = two equal +200 N actions",
    );
    for factors in [(1.0, 0.5), (1.0, 1.0)] {
        let request = source_model(
            &[("source:weight", weight)],
            &[("source:weight", factors.0), ("source:weight", factors.1)],
        );
        blocked_with(
            &request,
            &format!("duplicate_source_ref factors={factors:?}"),
            SOURCE_DUPLICATE,
        );
    }
    let pair = source_model(
        &[("source:action-a", action), ("source:action-b", action)],
        &[("source:action-a", 1.0), ("source:action-b", 1.0)],
    );
    // The factor scales the primitive magnitude: one source at factor 2.
    let scaled = source_model(&[("source:action-a", action)], &[("source:action-a", 2.0)]);
    for mode in MODES {
        for (label, request) in [("distinct pair", &pair), ("factor 2", &scaled)] {
            let out = solve(request, mode);
            let ctx = format!("{} {label}", mode.as_str());
            let tip = displacement(&out, CASE, "node:tip", "x");
            close(tip, pair_rhs / k, pair_rhs / k, &format!("{ctx}: tip UX"));
            close(
                reaction(&out, CASE, "support:root", "Fx"),
                -pair_rhs,
                pair_rhs,
                &format!("{ctx}: root Fx"),
            );
            differs(
                tip,
                pair_rhs / k,
                action / k,
                pair_rhs / k,
                &format!("{ctx}: numeric deduplication"),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 8. Negative contract controls
// ---------------------------------------------------------------------------

#[test]
fn negative_unit_dimension_controls_block() {
    let theta = q(
        &ann("prescribed_rotation_all_fixed", "inputs/root_RZ"),
        "rad",
    );
    let root_ux = q(
        &ann("prescribed_translation_all_fixed", "inputs/root_UX"),
        "m",
    );
    blocked(
        &beam_model("prescribed_rotation_all_fixed", true, &[("RZ", theta, "m")]),
        "rotation_with_length_units",
    );
    blocked(
        &single_bar_model(&[("UX", root_ux, "rad")]),
        "translation_with_angle_units",
    );
}

#[test]
fn negative_typed_boundary_rejections_return_err() {
    let mut unknown_definition = thermal_model(TemperatureUnits::Celsius, true, false, false);
    unknown_definition["model"]["materials"][0]["expansion_laws"][0]["definition"] =
        json!("engineering_unreviewed_coefficient");
    rejected(&unknown_definition, "unknown_coefficient_definition");

    // The fit union makes length change and fit strain a single choice: an
    // object carrying both, or neither kind, is not a member of the union.
    let length = qv(q(&ann(FIT, "inputs/signed_fit_length_change"), "m"), "m");
    let strain = qv(q(&ann(FIT, "expected/cold/fit_strain"), "1"), "1");
    for (control, fit) in [
        (
            "fit_both_without_kind",
            json!({"length_change": length, "strain": strain}),
        ),
        (
            "fit_length_with_extra_strain",
            json!({"kind": "natural_length_change", "length_change": length, "strain": strain}),
        ),
        (
            "fit_strain_with_extra_length",
            json!({"kind": "fit_strain", "strain": strain, "length_change": length}),
        ),
    ] {
        let mut request = fit_model(FitRoute::Length(1.0), true, false);
        request["model"]["reference_configurations"][0]["member_references"][0]["fit"] = fit;
        rejected(
            &request,
            &format!("same_member_fit_length_and_fit_strain/{control}"),
        );
    }

    let mut unknown_field = two_bar_model(false, false);
    state_mut(&mut unknown_field, 0)["unreviewed_field"] = json!(true);
    rejected(&unknown_field, "unknown_analysis_state_field");

    let mut unknown_thermal = two_bar_model(false, false);
    state_mut(&mut unknown_thermal, 0)["element_states"][0]["thermal_state"] =
        json!({"kind": "ambient_guess", "provenance": PROV});
    rejected(&unknown_thermal, "unknown_thermal_state_kind");

    let mut unknown_basis = two_bar_model(false, false);
    unknown_basis["model"]["reference_configurations"][0]["member_references"][0]["basis"] =
        json!({"kind": "assumed_ambient_reference"});
    rejected(&unknown_basis, "unknown_member_basis_kind");
}

#[test]
fn negative_legacy_thermal_referenced_with_resolved_state_blocks() {
    let a = annulus();
    let e = q(&ann(THERMAL, "inputs/E"), "Pa");
    let strain = q(&generic(CONSTANT_ALPHA, "expected/thermal_strain"), "1");
    // Invented legacy record: base alpha 1.2e-5/degC over a 50 degC interval.
    let legacy_alpha = 1.2e-5;
    let legacy_interval = 50.0;
    let legacy = json!({"id": "load:legacy-thermal", "category": "thermal",
        "target": {"type": "element", "pipe": "pipe:interval"}, "direction": "global_x",
        "magnitude": qv(legacy_interval, "degC"), "dimension": "temperature_interval",
        "provenance": PROV});
    let with_legacy = |referenced: bool, thermal: Option<Value>| {
        let mut request = constant_alpha_model(true, false);
        request["model"]["materials"][0]["thermal_expansion_coefficient"] =
            qv(legacy_alpha, "1/degC");
        request["model"]["load_cases"][0]["primitive_loads"] = json!([legacy.clone()]);
        if referenced {
            state_mut(&mut request, 0)["load_sources"] =
                json!([{"source_ref": "load:legacy-thermal", "factor": 1.0}]);
        }
        if let Some(thermal) = thermal {
            state_mut(&mut request, 0)["element_states"][0]["thermal_state"] = thermal;
        }
        request
    };
    blocked(
        &with_legacy(true, None),
        "same_member_legacy_thermal_plus_resolved_thermal",
    );
    blocked(
        &with_legacy(true, Some(th_unchanged())),
        "referenced_legacy_thermal_with_unchanged_reference",
    );
    // Unreferenced legacy thermal is excluded and reported; only the resolved state acts.
    let expected = -e * a.area * strain;
    let doubled = -e * a.area * (strain + legacy_alpha * legacy_interval);
    for mode in MODES {
        let out = solve(&with_legacy(false, None), mode);
        let ctx = format!("{} unreferenced legacy thermal", mode.as_str());
        member_axial(&out, CASE, "pipe:interval", expected, expected.abs(), &ctx);
        differs(
            midspan_axial(&out, CASE, "pipe:interval"),
            expected,
            doubled,
            expected.abs(),
            &format!("{ctx}: double consumption"),
        );
        let record = evidence(&out, CASE);
        assert!(
            mentions(&record["excluded_sources"], "load:legacy-thermal"),
            "{ctx}: legacy thermal reported excluded: {}",
            record["excluded_sources"]
        );
    }
}

#[test]
fn negative_legacy_version_documents_carrying_load_state_block() {
    let strip_cases = |request: &mut Value| {
        for case in request["model"]["load_cases"].as_array_mut().unwrap() {
            case.as_object_mut().unwrap().remove("analysis_state");
        }
    };
    let strip_configurations = |request: &mut Value| {
        request["model"]
            .as_object_mut()
            .unwrap()
            .remove("reference_configurations");
    };
    let strip_laws = |request: &mut Value| {
        for m in request["model"]["materials"].as_array_mut().unwrap() {
            m.as_object_mut().unwrap().remove("expansion_laws");
        }
    };
    let base = || {
        let mut request = thermal_model(TemperatureUnits::Celsius, true, false, false);
        request["model"]["schema_version"] = json!("0.3.0");
        request
    };
    blocked_with(
        &base(),
        "0.3.0_with_all_load_state_owners",
        VERSION_MISMATCH,
    );
    let mut only_state = base();
    strip_configurations(&mut only_state);
    strip_laws(&mut only_state);
    blocked_with(&only_state, "0.3.0_with_analysis_state", VERSION_MISMATCH);
    let mut only_configurations = base();
    strip_cases(&mut only_configurations);
    strip_laws(&mut only_configurations);
    blocked_with(
        &only_configurations,
        "0.3.0_with_reference_configurations",
        VERSION_MISMATCH,
    );
    let mut only_laws = base();
    strip_cases(&mut only_laws);
    strip_configurations(&mut only_laws);
    blocked_with(&only_laws, "0.3.0_with_expansion_laws", VERSION_MISMATCH);
    let mut legacy_020 = only_state.clone();
    legacy_020["model"]["schema_version"] = json!("0.2.0");
    legacy_020["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    legacy_020["model"]["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("pressure_regions");
    blocked_with(&legacy_020, "0.2.0_with_analysis_state", VERSION_MISMATCH);
}

#[test]
fn negative_boundary_motion_on_unrestrained_dof_blocks() {
    let mut request = two_bar_model(false, false);
    // support:middle is a guide that does not restrain UX.
    state_mut(&mut request, 0)["support_states"][1]["boundary_motion"] = json!([{"dof": "UX", "value": qv(0.05, "mm"), "meaning": "absolute_reference_displacement"}]);
    blocked(&request, "boundary_motion_on_unrestrained_dof");
}

#[test]
fn negative_missing_element_state_blocks() {
    let mut missing = two_bar_model(false, false);
    state_mut(&mut missing, 0)["element_states"]
        .as_array_mut()
        .unwrap()
        .remove(1);
    blocked(&missing, "missing_element_state");
    let mut duplicate = two_bar_model(false, false);
    let first = state_mut(&mut duplicate, 0)["element_states"][0].clone();
    state_mut(&mut duplicate, 0)["element_states"][1] = first;
    blocked(&duplicate, "duplicate_element_state_and_missing_second");
}

#[test]
fn negative_free_length_state_without_operating_temperature_blocks() {
    let mut no_temperature = thermal_model(TemperatureUnits::Celsius, true, false, false);
    state_mut(&mut no_temperature, 0)["element_states"][0]
        .as_object_mut()
        .unwrap()
        .remove("operating_temperature");
    blocked(
        &no_temperature,
        "free_length_state_without_operating_temperature",
    );
    let mut direct_basis = thermal_model(TemperatureUnits::Celsius, true, false, false);
    direct_basis["model"]["reference_configurations"][0]["member_references"][0]["basis"] =
        basis_direct();
    blocked(
        &direct_basis,
        "free_length_state_with_direct_strain_reference",
    );
    let mut unknown_law = thermal_model(TemperatureUnits::Celsius, true, false, false);
    state_mut(&mut unknown_law, 0)["element_states"][0]["thermal_state"] = th_free("law:absent");
    blocked(&unknown_law, "free_length_state_law_not_on_material");
}

#[test]
fn negative_thermal_law_coverage_controls_block() {
    let mut duplicate = thermal_model(TemperatureUnits::Celsius, true, false, false);
    let points = &mut duplicate["model"]["materials"][0]["expansion_laws"][0]["data"]["points"];
    let mut repeated = points[1].clone();
    repeated["coefficient"]["value"] = json!(1.3e-5); // invented conflicting value
    points.as_array_mut().unwrap().insert(2, repeated);
    blocked(&duplicate, "duplicate_temperature");

    let table = at(&ann(THERMAL, "inputs/table_points")).as_array().unwrap();
    let bottom = qval(&table[0]["temperature"], "degC", "first table temperature");
    let top = qval(
        &table[table.len() - 1]["temperature"],
        "degC",
        "last table temperature",
    );
    let mut above = thermal_model(TemperatureUnits::Celsius, true, false, false);
    state_mut(&mut above, 0)["element_states"][0]["operating_temperature"] = qv(top + 50.0, "degC");
    blocked(
        &above,
        "missing_thermal_interval_bracket/operating_above_table",
    );
    let mut below = thermal_model(TemperatureUnits::Celsius, true, false, false);
    below["model"]["reference_configurations"][0]["member_references"][0]["basis"] =
        basis_temperature(bottom - 10.0, "degC");
    blocked(
        &below,
        "missing_thermal_interval_bracket/installation_below_table",
    );

    let mut datum_outside = thermal_model(TemperatureUnits::Celsius, true, false, false);
    let law = &mut datum_outside["model"]["materials"][0]["expansion_laws"][0];
    law["definition"] = json!("differential_per_datum_length");
    law["datum_temperature"] = qv(bottom - 10.0, "degC");
    blocked(&datum_outside, "datum_outside_thermal_coverage");

    let l = q(&ann(FIT, "inputs/L"), "m");
    for (control, fit) in [
        (
            "nonpositive_free_stretch/cut_equals_length",
            fit_length(-l, "m"),
        ),
        (
            "nonpositive_free_stretch/fit_strain_minus_one",
            fit_strain(-1.0),
        ),
        (
            "nonpositive_free_stretch/fit_strain_below_minus_one",
            fit_strain(-1.5),
        ),
    ] {
        let mut request = fit_model(FitRoute::Length(1.0), true, false);
        request["model"]["reference_configurations"][0]["member_references"][0]["fit"] = fit;
        blocked(&request, control);
    }
}

#[test]
fn negative_additional_wire_rules_block() {
    let root_ux = q(&ann(PRESCRIBED_TWO_BAR, "inputs/root_UX"), "m");
    let motion = |dof: &str, value: f64, unit: &str| json!({"dof": dof, "value": qv(value, unit), "meaning": "absolute_reference_displacement"});

    let mut duplicate_dof = two_bar_model(false, false);
    state_mut(&mut duplicate_dof, 0)["support_states"][0]["boundary_motion"] =
        json!([motion("UX", root_ux, "m"), motion("UX", root_ux, "m")]);
    blocked(&duplicate_dof, "boundary_motion_duplicate_dof");

    // A spring on the free middle UX solves without motion, and blocks with one.
    let spring = |with_motion: bool| {
        let mut request = two_bar_model(false, false);
        request["model"]["supports"]
            .as_array_mut()
            .unwrap()
            .push(json!({
            "id": "support:spring", "node": "node:middle", "family": "spring", "restraints": ["UX"],
            "stiffness": {"dof": "UX", "value": qv(1.0e6, "N/m")}, "provenance": PROV}));
        let mut state = json!({"support_ref": "support:spring", "participation": {"kind": "active_model_device"}});
        if with_motion {
            state["boundary_motion"] = json!([motion("UX", root_ux, "m")]);
        }
        state_mut(&mut request, 0)["support_states"]
            .as_array_mut()
            .unwrap()
            .push(state);
        request
    };
    for mode in MODES {
        solve(&spring(false), mode);
    }
    blocked(&spring(true), "spring_boundary_motion");

    let mut material_mismatch = two_bar_model(false, false);
    state_mut(&mut material_mismatch, 0)["element_states"][0]["material_selection"] =
        sel_base("material:bar-2");
    blocked(&material_mismatch, "material_ref_not_pipe_material");

    let mut missing_support = two_bar_model(false, false);
    state_mut(&mut missing_support, 0)["support_states"]
        .as_array_mut()
        .unwrap()
        .remove(2);
    blocked(&missing_support, "missing_support_state");
    let mut duplicate_support = two_bar_model(false, false);
    let far = state_mut(&mut duplicate_support, 0)["support_states"][2].clone();
    state_mut(&mut duplicate_support, 0)["support_states"]
        .as_array_mut()
        .unwrap()
        .push(far);
    blocked(&duplicate_support, "duplicate_support_state");

    let weight = q(&ann(SOURCE, "inputs/weight"), "N");
    blocked(
        &source_model(&[("source:weight", weight)], &[("source:not-stored", 1.0)]),
        "unknown_source_ref",
    );
    blocked(
        &source_model(&[("source:weight", weight)], &[("source:weight", 0.0)]),
        "zero_source_factor",
    );
    // Cross-case: case B names a primitive stored only in case A.
    let mut cross = source_model(&[("source:weight", weight)], &[("source:weight", 1.0)]);
    let mut other = cross["model"]["load_cases"][0].clone();
    other["id"] = json!("case:other");
    other["primitive_loads"] = json!([]);
    cross["model"]["load_cases"]
        .as_array_mut()
        .unwrap()
        .push(other);
    for mode in MODES {
        let out = run(&cross, mode).unwrap_or_else(|e| panic!("cross-case source: typed Err {e}"));
        let codes = blocking_codes(&out);
        println!(
            "OBSERVED_BLOCK control=source_ref_from_other_case mode={} status={} codes={codes:?} details={:?}",
            mode.as_str(),
            out.status.mechanics,
            blocking_details(&out)
        );
        assert!(
            !codes.is_empty(),
            "cross-case source_ref accepted: {:#?}",
            out.diagnostics
        );
        assert!(
            !out.results.iter().any(|r| r
                .basis_ref
                .as_ref()
                .is_some_and(|b| b.ref_id == "case:other")),
            "case:other published rows"
        );
    }

    let mut basis_ref = two_bar_model(false, false);
    basis_ref["model"]["load_cases"][0]["modulus_basis_ref"] = json!("point:any");
    blocked(&basis_ref, "modulus_basis_ref_present");
    let mut basis_temperature_field = two_bar_model(false, false);
    basis_temperature_field["model"]["load_cases"][0]["modulus_basis_temperature"] =
        qv(20.0, "degC");
    blocked(
        &basis_temperature_field,
        "modulus_basis_temperature_present",
    );

    let mut request_materials = two_bar_model(false, false);
    request_materials["materials"] = json!([material("material:bar-1", 1.0e11, INVENTED_NU)]);
    blocked(&request_materials, "request_level_materials_non_empty");

    let mut mass = two_bar_model(false, false);
    state_mut(&mut mass, 0)["element_states"][0]["mass_state_ref"] = json!("mass:unimplemented");
    blocked(&mass, "mass_state_ref_not_implemented");
    let mut inactive = two_bar_model(false, false);
    state_mut(&mut inactive, 0)["support_states"][2]["participation"] = json!({"kind": "inactive"});
    blocked(&inactive, "inactive_participation_not_implemented");
    let mut base_motion = two_bar_model(false, false);
    state_mut(&mut base_motion, 0)["support_states"][2]["base_motion"] =
        json!([motion("UX", root_ux, "m")]);
    blocked(&base_motion, "base_motion_not_implemented");

    let mut missing_member = two_bar_model(false, false);
    missing_member["model"]["reference_configurations"][0]["member_references"]
        .as_array_mut()
        .unwrap()
        .remove(1);
    blocked(&missing_member, "missing_member_reference");
    let mut duplicate_member = two_bar_model(false, false);
    let member =
        duplicate_member["model"]["reference_configurations"][0]["member_references"][0].clone();
    duplicate_member["model"]["reference_configurations"][0]["member_references"]
        .as_array_mut()
        .unwrap()
        .push(member);
    blocked(&duplicate_member, "duplicate_member_reference");
    let mut unknown_reference = two_bar_model(false, false);
    state_mut(&mut unknown_reference, 0)["reference_configuration_ref"] = json!("reference:absent");
    blocked(&unknown_reference, "unknown_reference_configuration_ref");

    // Rejection channel left open by the wire: Err or blocking diagnostic.
    let mut history = two_bar_model(false, false);
    state_mut(&mut history, 0)["history"] = json!({"kind": "continuation"});
    refused(&history, "history_not_independent_equilibrium");
    let mut no_state = two_bar_model(false, false);
    no_state["model"]["load_cases"][0]
        .as_object_mut()
        .unwrap()
        .remove("analysis_state");
    refused(&no_state, "analysis_state_missing_on_0.4.0_case");
    let mut contract = two_bar_model(false, false);
    state_mut(&mut contract, 0)["contract"] = json!("openpipestress.load_reference_state/9.9.9");
    refused(&contract, "unknown_load_state_contract");

    // A known actual-versus-selected temperature mismatch needs an override.
    let hot_t = q(&ann(FIT, "inputs/hot_temperature"), "degC");
    let mismatch = |override_basis: bool| {
        let mut request = fit_model(FitRoute::Length(1.0), true, false);
        request["model"]["load_cases"]
            .as_array_mut()
            .unwrap()
            .truncate(1); // case:cold
        let element = &mut state_mut(&mut request, 0)["element_states"][0];
        element["operating_temperature"] = qv(hot_t, "degC"); // point:cold is at installation
        if override_basis {
            element["analysis_basis_override"] =
                json!({"reason": "invented deliberate cold-basis check", "provenance": PROV});
        }
        request
    };
    blocked(
        &mismatch(false),
        "selected_point_temperature_mismatch_without_override",
    );
    let a = annulus();
    let cold_e = q(&ann(FIT, "inputs/cold_E"), "Pa");
    let hot_total = q(&ann(FIT, "expected/hot/total_eigenstrain"), "1");
    let overridden_n = -cold_e * a.area * hot_total;
    for mode in MODES {
        let out = solve(&mismatch(true), mode);
        member_axial(
            &out,
            "case:cold",
            "pipe:fit",
            overridden_n,
            overridden_n.abs(),
            &format!("{} explicit override", mode.as_str()),
        );
    }
}
