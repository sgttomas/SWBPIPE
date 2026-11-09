//! DEL-04-09 (Design, selected mass correction): the reader's model-derived
//! mass. With the bound request supplied, the reader forms the full upper M
//! from the model (rotary, Rayleigh and coupling entries included) and holds
//! every published entry, T_k, the free-DOF mass, the mass centre and the
//! participation to it. The threat is a coherently wrong M: the payload
//! changed and resealed with its totals and participation re-derived to agree,
//! which the payload relations alone accept. Each mutation below is checked
//! both ways: the payload relations alone (no request) do not fail it, and the
//! model-derived checks (bound request) do. All values are invented.
use open_pipe_stress_modal_result::{
    modal_reader_sha256, read_modal_result, ModalReadStanding, ModalReadStatus, ModalSupplied, MODAL_RELATIONS,
};
use serde_json::{json, Value};

macro_rules! pair {
    ($name:literal) => {
        (
            $name,
            include_str!(concat!("../../../../fixtures/results/", $name, ".json")),
            include_str!(concat!("../../../../fixtures/results/", $name, ".request.json")),
        )
    };
}

/// The solved fixtures whose bound models carry no nonlinear support.
const LINEAR: [(&str, &str, &str); 8] = [
    pair!("modal_component_body"),
    pair!("modal_arc"),
    pair!("modal_ex_n05"),
    pair!("modal_cantilever_whole_spectrum"),
    pair!("modal_cantilever_cluster_extension"),
    pair!("modal_cantilever_below_cutoff"),
    pair!("modal_cantilever_subnormal_cutoff"),
    pair!("modal_cantilever_sensitive"),
];
/// The solved fixtures linearized at a reported nonlinear state.
const STATES: [(&str, &str, &str); 2] = [pair!("modal_state_one_way_pressed"), pair!("modal_state_friction_sliding")];
const BODY: (&str, &str, &str) = pair!("modal_component_body");
const ARC: (&str, &str, &str) = pair!("modal_arc");

fn load(fixture: (&str, &str, &str)) -> (Value, Value) {
    (serde_json::from_str(fixture.1).unwrap(), serde_json::from_str(fixture.2).unwrap())
}

struct Read {
    binding: ModalReadStanding,
    mass: ModalReadStanding,
    participation: ModalReadStanding,
}

fn read(doc: &Value, request: Option<&Value>) -> Read {
    let verdict = read_modal_result(doc, ModalSupplied { request });
    let at = |name: &str| verdict.relations[MODAL_RELATIONS.iter().position(|r| *r == name).unwrap()].clone();
    Read { binding: verdict.binding.clone(), mass: at("mass"), participation: at("participation") }
}

fn sha(value: &Value) -> String {
    modal_reader_sha256(value).unwrap()
}

/// Recomputes every row checksum and table digest (as `tests/corpus.rs`).
fn reseal(doc: &mut Value) {
    for table in doc["tables"].as_object_mut().unwrap().values_mut() {
        if table["status"] != "published" {
            continue;
        }
        let mut checksums = Vec::new();
        for row in table["rows"].as_array_mut().unwrap() {
            let mut body = row.as_object().unwrap().clone();
            body.remove("checksum");
            let checksum = sha(&Value::Object(body));
            row["checksum"] = json!(checksum);
            checksums.push(json!(checksum));
        }
        table["row_count"] = json!(checksums.len());
        table["digest"] = json!(sha(&Value::Array(checksums)));
    }
}

/// Binds the document to a changed request: its hashes, run id and state
/// case, so that only the model-derived checks can see the change.
fn rebind(doc: &mut Value, request: &Value) {
    let b = &mut doc["binding"];
    b["request_sha256"] = json!(sha(request));
    b["model_sha256"] = json!(sha(&request["model"]));
    b["materials_sha256"] = json!(sha(request.get("materials").unwrap_or(&json!([]))));
    let run = sha(&json!({"request_sha256": b["request_sha256"], "integrity_parameters_sha256": b["integrity_parameters_sha256"]}));
    if !b["state_case"].is_null() {
        let index = b["state_case"]["case_index"].as_u64().unwrap() as usize;
        b["state_case"]["case_sha256"] = json!(sha(&request["model"]["load_cases"][index]));
    }
    doc["run"]["run_id"] = json!(format!("modal-run:{}", &run[..16]));
}

fn not_checked(standing: &ModalReadStanding) -> bool {
    standing.scope.iter().any(|s| s.starts_with("model-derived mass not checked") || s.starts_with("model-derived participation not checked"))
}

fn assert_failed(standing: &ModalReadStanding, needle: &str, what: &str) {
    assert_eq!(standing.status, ModalReadStatus::Failed, "{what}: {standing:?}");
    let difference = standing.difference.as_deref().unwrap_or("");
    assert!(difference.starts_with("model-derived") && difference.contains(needle), "{what}: `{difference}` lacks `{needle}`");
}

// ---------------------------------------------------------------------------
// Intact controls.

#[test]
fn intact_linear_fixtures_pass_the_model_derived_checks() {
    for fixture in LINEAR {
        let (doc, request) = load(fixture);
        let r = read(&doc, Some(&request));
        assert_eq!(r.binding.status, ModalReadStatus::Passed, "{}", fixture.0);
        for standing in [&r.mass, &r.participation] {
            assert_eq!(standing.status, ModalReadStatus::Passed, "{}: {standing:?}", fixture.0);
        }
    }
}

#[test]
fn reported_states_make_the_assurance_conditional_never_passed() {
    for fixture in STATES {
        let (doc, request) = load(fixture);
        let r = read(&doc, Some(&request));
        assert_eq!(r.binding.status, ModalReadStatus::Passed, "{}", fixture.0);
        for standing in [&r.mass, &r.participation] {
            assert_eq!(standing.status, ModalReadStatus::ScopedUnsupported, "{}: {standing:?}", fixture.0);
            assert!(standing.unresolved.is_empty() && !not_checked(standing), "{}: {standing:?}", fixture.0);
            assert!(standing.scope.iter().any(|s| s.contains("conditional on them")), "{}: {standing:?}", fixture.0);
        }
    }
}

#[test]
fn without_the_bound_request_the_model_derived_checks_are_scoped() {
    for fixture in LINEAR.into_iter().chain(STATES) {
        let (doc, _) = load(fixture);
        let r = read(&doc, None);
        for standing in [&r.mass, &r.participation] {
            assert_eq!(standing.status, ModalReadStatus::ScopedUnsupported, "{}: {standing:?}", fixture.0);
            assert!(standing.scope.iter().any(|s| s.contains("no request is supplied")), "{}: {standing:?}", fixture.0);
        }
    }
    // A request that is not the bound one fails the binding and derives nothing.
    let (doc, mut request) = load(BODY);
    request["model"]["pipe_segments"][0]["section"]["contents_density"]["value"] = json!(1001.0);
    let r = read(&doc, Some(&request));
    assert_eq!(r.binding.status, ModalReadStatus::Failed);
    for standing in [&r.mass, &r.participation] {
        assert_eq!(standing.status, ModalReadStatus::ScopedUnsupported, "{standing:?}");
        assert!(standing.scope.iter().any(|s| s.contains("not the bound request")), "{standing:?}");
    }
}

// ---------------------------------------------------------------------------
// Coherent payload mutations: the payload relations alone accept each one.

fn mass_row<'a>(doc: &'a mut Value, row: &str, col: &str) -> &'a mut Value {
    doc["tables"]["mass_matrix"]["rows"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["row_dof_ref"] == row && r["col_dof_ref"] == col)
        .unwrap_or_else(|| panic!("no mass entry {row}|{col}"))
}

fn shape(doc: &Value, mode: &str, dof: &str) -> f64 {
    // A withheld table states no shape: the change then moves no published m.
    doc["tables"]["shapes"]["rows"]
        .as_array()
        .and_then(|rows| rows.iter().find(|r| r["mode_ref"] == mode && r["dof_ref"] == dof))
        .map_or(0.0, |r| r["value"]["value"].as_f64().unwrap())
}

fn scale_value(vo: &mut Value, factor: f64) {
    if let Some(v) = vo["value"].as_f64() {
        vo["value"] = json!(v * factor);
    }
}

/// Changes one entry by a factor and re-derives m_i = φᵀMφ, Γ, m_eff, the
/// fractions and the cumulative fractions to agree with it.
fn coherent_entry(doc: &mut Value, row: &str, col: &str, factor: f64) {
    let slot = &mut mass_row(doc, row, col)["value"]["value"];
    let old = slot.as_f64().unwrap();
    let new = old * factor;
    *slot = json!(new);
    let delta = new - old;
    let w = if row == col { 1.0 } else { 2.0 };
    let mut ratios = Vec::new();
    for mode in doc["tables"]["modes"]["rows"].as_array().unwrap() {
        let id = mode["row_id"].as_str().unwrap().to_string();
        let m = mode["generalized_mass"]["value"].as_f64().unwrap();
        let m_new = m + w * shape(doc, &id, row) * shape(doc, &id, col) * delta;
        ratios.push((id, m, m_new));
    }
    for (mode, (_, _, m_new)) in doc["tables"]["modes"]["rows"].as_array_mut().unwrap().iter_mut().zip(&ratios) {
        mode["generalized_mass"]["value"] = json!(m_new);
    }
    let mut cumulative = std::collections::HashMap::new();
    for p in doc["tables"]["participation"]["rows"].as_array_mut().unwrap() {
        let (_, m, m_new) = ratios.iter().find(|(id, _, _)| p["mode_ref"] == id.as_str()).unwrap();
        let ratio = m / m_new;
        for key in ["participation_factor", "effective_mass", "effective_mass_fraction"] {
            scale_value(&mut p[key], ratio);
        }
        if let Some(f) = p["effective_mass_fraction"]["value"].as_f64() {
            let sum = cumulative.entry(p["direction"].as_str().unwrap().to_string()).or_insert(0.0);
            *sum += f;
            p["cumulative_effective_mass_fraction"]["value"] = json!(*sum);
        }
    }
}

/// Doubles M with T_k, the free-DOF masses, m_i and m_eff (Γ and the
/// fractions are unchanged): every relation among them still holds exactly.
fn double_everything(doc: &mut Value) {
    let tables = &mut doc["tables"];
    for r in tables["mass_matrix"]["rows"].as_array_mut().unwrap() {
        scale_value(&mut r["value"], 2.0);
    }
    for r in tables["mass_totals"]["rows"].as_array_mut().unwrap() {
        scale_value(&mut r["total_mass"], 2.0);
        scale_value(&mut r["free_dof_mass"], 2.0);
    }
    for r in tables["modes"]["rows"].as_array_mut().unwrap() {
        scale_value(&mut r["generalized_mass"], 2.0);
    }
    for r in tables["participation"]["rows"].as_array_mut().unwrap() {
        scale_value(&mut r["effective_mass"], 2.0);
    }
}

fn withhold(doc: &mut Value, table: &str) {
    doc["tables"][table] = json!({"status": "withheld", "reason": "corpus: payload withheld"});
}

/// The payload alone accepts `doc` (its mass standing is scoped only for the
/// absent request); the bound request fails it.
fn payload_case(what: &str, mutate: impl Fn(&mut Value), needle: &str) -> Read {
    let (mut doc, request) = load(BODY);
    mutate(&mut doc);
    reseal(&mut doc);
    let alone = read(&doc, None);
    assert_eq!(alone.mass.status, ModalReadStatus::ScopedUnsupported, "{what}: payload alone {:?}", alone.mass);
    assert_ne!(alone.participation.status, ModalReadStatus::Failed, "{what}: payload alone {:?}", alone.participation);
    let bound = read(&doc, Some(&request));
    assert_eq!(bound.binding.status, ModalReadStatus::Passed, "{what}");
    assert_failed(&bound.mass, needle, what);
    bound
}

#[test]
fn a_rotary_diagonal_invisible_to_the_totals_fails() {
    let r = payload_case(
        "rx diagonal ×1.25",
        |d| coherent_entry(d, "dof:c:rx", "dof:c:rx", 1.25),
        "entry mass:dof:c:rx|dof:c:rx",
    );
    assert_failed(&r.participation, "mode 1: m", "rx diagonal ×1.25 participation");
}

#[test]
fn a_rotational_coupling_invisible_to_the_totals_and_m_r_fails() {
    payload_case(
        "rx|ry coupling ×1.25",
        |d| coherent_entry(d, "dof:c:rx", "dof:c:ry", 1.25),
        "entry mass:dof:c:rx|dof:c:ry",
    );
}

#[test]
fn a_change_far_below_the_payload_tolerances_but_above_rounding_fails() {
    payload_case(
        "rx diagonal ×(1 + 2⁻³⁰)",
        |d| coherent_entry(d, "dof:c:rx", "dof:c:rx", 1.0 + 2f64.powi(-30)),
        "entry mass:dof:c:rx|dof:c:rx",
    );
}

#[test]
fn a_uniformly_scaled_mass_fails_on_entries_and_without_them_on_the_totals() {
    let r = payload_case("M ×2", double_everything, "entry mass:");
    assert_failed(&r.participation, "mode 1: m", "M ×2 participation");
    let r = payload_case(
        "M ×2, mass_matrix withheld",
        |d| {
            double_everything(d);
            withhold(d, "mass_matrix");
        },
        "T_x",
    );
    assert_failed(&r.participation, "mode 1: m", "M ×2 withheld participation");
}

#[test]
fn a_moved_mass_centre_fails() {
    payload_case(
        "mass centre x + 1 mm",
        |d| {
            let x = &mut d["basis"]["mass_center"]["x"]["value"];
            *x = json!(x.as_f64().unwrap() + 1e-3);
        },
        "the mass centre x",
    );
}

#[test]
fn withheld_shapes_scope_the_participation_but_not_the_mass() {
    let (mut doc, request) = load(BODY);
    withhold(&mut doc, "shapes");
    reseal(&mut doc);
    let r = read(&doc, Some(&request));
    assert_ne!(r.mass.status, ModalReadStatus::Failed, "{:?}", r.mass);
    assert!(!not_checked(&r.mass), "{:?}", r.mass);
    assert_eq!(r.participation.status, ModalReadStatus::ScopedUnsupported, "{:?}", r.participation);
    // The entries remain checked: a coherent rotary change still fails.
    coherent_entry(&mut doc, "dof:c:rx", "dof:c:rx", 1.25);
    reseal(&mut doc);
    assert_failed(&read(&doc, Some(&request)).mass, "entry mass:dof:c:rx|dof:c:rx", "withheld shapes, rx ×1.25");
}

// ---------------------------------------------------------------------------
// Request-side mutations, the document rebound to them.

#[test]
fn a_gap_support_in_the_bound_model_fails() {
    // The static path never confirms a gap state and modal refuses every model with a gap
    // support, so a solved document bound to one is a contradiction, never conditional.
    let r = request_case(STATES[0], "one-way support made a gap", |r| {
        r["model"]["supports"][1]["nonlinear"]["behavior"] = json!("gap");
    });
    assert_failed(&r.mass, "gap support", "gap support");
}

fn request_case(fixture: (&str, &str, &str), what: &str, mutate: impl Fn(&mut Value)) -> Read {
    let (mut doc, mut request) = load(fixture);
    mutate(&mut request);
    rebind(&mut doc, &request);
    let r = read(&doc, Some(&request));
    assert_eq!(r.binding.status, ModalReadStatus::Passed, "{what}: {:?}", r.binding);
    r
}

fn section(request: &mut Value) -> &mut Value {
    &mut request["model"]["pipe_segments"][0]["section"]
}

fn shared_section(request: &mut Value, outside_diameter: f64) {
    request["model"]["sections"] = json!([{"id": "section:p", "section_type": "pipe", "properties": {
        "outside_diameter": {"unit": "m", "value": outside_diameter}, "wall_thickness": {"unit": "m", "value": 0.01}}}]);
    for pipe in request["model"]["pipe_segments"].as_array_mut().unwrap() {
        pipe["section_ref"] = json!("section:p");
    }
}

#[test]
fn a_rebound_model_whose_mass_differs_fails() {
    let cases: [(&str, fn(&mut Value), &str); 9] = [
        ("restraint removed", |r| {
            let restraints = r["model"]["supports"][0]["restraints"].as_array_mut().unwrap();
            restraints.retain(|t| t != "RX");
        }, "dof:a:rx is restrained in the document but free"),
        ("insulation added", |r| {
            section(r)["insulation_thickness"] = json!({"unit": "m", "value": 0.05});
            section(r)["insulation_density"] = json!({"unit": "kg/m^3", "value": 100.0});
        }, "entry mass:"),
        ("mill tolerance", |r| section(r)["mill_tolerance"] = json!({"unit": "m", "value": 0.001}), "entry mass:"),
        ("contents density 1001", |r| section(r)["contents_density"]["value"] = json!(1001.0), "entry mass:"),
        ("body not consumed", |r| {
            r["model"]["components"][0]["mechanics_interface"]["solver_consumption"] = json!("not_solver_consumed");
        }, "entry mass:"),
        ("material density absent", |r| {
            section(r).as_object_mut().unwrap().remove("material_density");
        }, "no positive material density"),
        ("centre of gravity y 0.26", |r| r["model"]["components"][0]["geometry"]["center_of_gravity"]["y"] = json!(0.26), "entry mass:"),
        ("stale shared section", |r| shared_section(r, 0.21), "bound section"),
        // The producer blocks every solve of a model with a snubber, so a solved
        // document bound to one is a contradiction, never scoped.
        ("a support made a snubber", |r| r["model"]["supports"][0]["family"] = json!("snubber"), "SNUBBER_DYNAMIC_ONLY_UNSUPPORTED"),
    ];
    for (what, mutate, needle) in cases {
        let r = request_case(BODY, what, mutate);
        assert_failed(&r.mass, needle, what);
    }
    let r = request_case(ARC, "arc bow side flipped", |r| {
        for pipe in r["model"]["pipe_segments"].as_array_mut().unwrap() {
            for v in pipe["y_reference"].as_object_mut().unwrap().values_mut() {
                let x = v.as_f64().unwrap();
                if x != 0.0 {
                    *v = json!(-x);
                }
            }
        }
    });
    assert_failed(&r.mass, "entry mass:", "arc bow side flipped");
}

#[test]
fn a_rebound_model_with_the_same_mass_passes() {
    let r = request_case(BODY, "alternate catalog units", |r| {
        let g = &mut r["model"]["components"][0]["geometry"];
        g["weight"] = json!({"unit": "kN", "value": 0.5});
        g["center_of_gravity"] = json!({"unit": "mm", "x": -500.0, "y": 250.0, "z": 0});
        section(r)["outside_diameter"] = json!({"unit": "mm", "value": 200.0});
        section(r)["wall_thickness"] = json!({"unit": "mm", "value": 10.0});
    });
    assert_eq!(r.mass.status, ModalReadStatus::Passed, "{:?}", r.mass);
    assert_eq!(r.participation.status, ModalReadStatus::Passed, "{:?}", r.participation);
    let r = request_case(BODY, "matching shared section", |r| shared_section(r, 0.2));
    assert_eq!(r.mass.status, ModalReadStatus::Passed, "{:?}", r.mass);
    assert_eq!(r.participation.status, ModalReadStatus::Passed, "{:?}", r.participation);
}
