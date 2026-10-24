//! No silent wrong results (Agent 0, 2026-10-23): authored input or combination
//! rows are never dropped without a name. Every request goes through the captured
//! value entry the desktop uses, in both solver modes.
//!
//! - Closed model records: an unknown key on the model root, a load case (its
//!   equivalent_static inputs and solve temperature), a primitive load (its
//!   target and magnitude), a combination or a combination term refuses by name
//!   (`UNKNOWN_MODEL_FIELD`) before anything is solved.
//! - A `mechanics` combination naming one load case twice blocks by name
//!   (`LOAD_COMBINATION_DUPLICATE_TERM`) instead of losing every combined row.
//! - Controls: well-formed documents (including the declared, solve-ignored
//!   `data_boundary`, `diagnostics` and load-case `kind`/`label`/`status`, and
//!   model document 0.7.0's root `lines`, `unresolved_assumptions`,
//!   `constraints`, `equipment_interfaces` and `rule_pack_refs` and load-case and
//!   combination `tags`/`notes`) publish; a combination that publishes keeps its rows and diagnostics.
//!   Setting `SILENT_DROP_GUARD_DUMP=<dir>` writes the control envelopes' exact
//!   bytes there, for a byte comparison against another tree.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope,
    PreviewSolverMode,
};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const DEMO: &str = include_str!("../../../fixtures/product_preview/invented_demo_model.json");
const COMBINATION: &str = "combination:C-OPER-ALT";

/// The bundled demo: the desktop's default session model (nonlinear supports,
/// so its combination is gated).
fn demo() -> Value {
    serde_json::from_str(DEMO).unwrap()
}

/// The demo with only its linear (anchor and guide) supports, so its
/// combination publishes combined rows.
fn linear() -> Value {
    let mut model = demo();
    model["supports"]
        .as_array_mut()
        .unwrap()
        .retain(|support| matches!(support["family"].as_str(), Some("anchor" | "guide")));
    model
}

fn request(model: &Value) -> Value {
    json!({"model": model, "materials": []})
}

fn solve(model: &Value, mode: PreviewSolverMode) -> Result<MechanicsEnvelope, String> {
    run_linear_static_preview_value_with_mode(request(model), mode)
}

fn refusal(model: &Value, mode: PreviewSolverMode, what: &str) -> String {
    match solve(model, mode) {
        Ok(envelope) => panic!(
            "{what}/{mode:?}: an unknown key must refuse, not publish (mechanics {}, {} rows)",
            envelope.status.mechanics,
            envelope.results.len()
        ),
        Err(error) => error,
    }
}

fn combination_rows(envelope: &MechanicsEnvelope) -> usize {
    envelope
        .results
        .iter()
        .filter(|row| row.basis_ref.as_ref().is_some_and(|basis| basis.ref_type == "combination"))
        .count()
}

fn codes(envelope: &MechanicsEnvelope) -> Vec<&str> {
    envelope.diagnostics.iter().map(|d| d.code.as_str()).collect()
}

fn dump(name: &str, mode: PreviewSolverMode, envelope: &MechanicsEnvelope) {
    if let Some(dir) = std::env::var_os("SILENT_DROP_GUARD_DUMP") {
        let path = std::path::Path::new(&dir).join(format!("{name}.{}.json", mode.as_str()));
        std::fs::write(path, serde_json::to_vec(envelope).unwrap()).unwrap();
    }
}

/// (what, edit, named field, named record, path)
type Probe = (&'static str, fn(&mut Value), &'static str, &'static str, &'static str);

fn probes() -> Vec<Probe> {
    vec![
        (
            "misspelled primitive_loads empties a case",
            |m| {
                let case = m["load_cases"][1].as_object_mut().unwrap();
                let loads = case.remove("primitive_loads").unwrap();
                case.insert("primitive_load".into(), loads);
            },
            "primitive_load",
            "load case `load:L-200`",
            "model.load_cases[1]",
        ),
        (
            "misspelled combinations drops every combination",
            |m| {
                let root = m.as_object_mut().unwrap();
                let combinations = root.remove("combinations").unwrap();
                root.insert("combination".into(), combinations);
            },
            "combination",
            "the model",
            "model",
        ),
        (
            "unknown key on a primitive load",
            |m| m["load_cases"][0]["primitive_loads"][1]["scale"] = json!(2.0),
            "scale",
            "primitive load `load:L-100-Y` in load case `load:L-100`",
            "model.load_cases[0].primitive_loads[1]",
        ),
        (
            "unknown key on a primitive-load target",
            |m| m["load_cases"][0]["primitive_loads"][1]["target"]["pipe"] = json!("pipe:P-120"),
            "pipe",
            "the target of primitive load `load:L-100-Y` in load case `load:L-100`",
            "model.load_cases[0].primitive_loads[1].target",
        ),
        (
            "unknown key on a primitive-load magnitude",
            |m| m["load_cases"][0]["primitive_loads"][1]["magnitude"]["factor"] = json!(0.5),
            "factor",
            "the magnitude of primitive load `load:L-100-Y` in load case `load:L-100`",
            "model.load_cases[0].primitive_loads[1].magnitude",
        ),
        (
            "misspelled exposed_pipe_refs drops the wind span",
            |m| m["load_cases"][0]["equivalent_static"] = json!({"wind": {"pressure": {"value": 500.0, "unit": "Pa"}, "shape_factor": {"value": 1.0, "unit": "1"}, "direction": "global_y", "exposed_pipe_ref": ["pipe:P-120"]}, "provenance": "invented"}),
            "exposed_pipe_ref",
            "the wind input of load case `load:L-100`",
            "model.load_cases[0].equivalent_static.wind",
        ),
        (
            "unknown key on a solve temperature",
            |m| m["load_cases"][0]["modulus_basis_temperature"] = json!({"value": 150.0, "unit": "degC", "basis": "guessed"}),
            "basis",
            "the modulus_basis_temperature of load case `load:L-100`",
            "model.load_cases[0].modulus_basis_temperature",
        ),
        (
            "unknown key on a combination",
            |m| m["combinations"][0]["factors"] = json!([1.0]),
            "factors",
            "combination `combination:C-OPER-ALT`",
            "model.combinations[0]",
        ),
        (
            "unknown key on a combination term",
            |m| m["combinations"][0]["terms"][1]["load_case_ref"] = json!("load:L-100"),
            "load_case_ref",
            "term 1 of combination `combination:C-OPER-ALT`",
            "model.combinations[0].terms[1]",
        ),
    ]
}

#[test]
fn an_unknown_key_on_a_closed_record_refuses_by_field_and_record() {
    for base in [demo(), linear()] {
        for (what, edit, field, record, path) in probes() {
            let mut model = base.clone();
            edit(&mut model);
            for mode in MODES {
                let error = refusal(&model, mode, what);
                assert!(error.starts_with("UNKNOWN_MODEL_FIELD: "), "{what}/{mode:?}: {error}");
                let named = format!("unknown field `{field}` in {record}");
                assert!(error.contains(&named), "{what}/{mode:?}: expected `{named}` in {error}");
                assert!(error.contains(&format!("{named} ({path})")), "{what}/{mode:?}: {error}");
            }
            // Every typed reader refuses the same key (the records deny unknown fields).
            let typed = serde_json::from_value::<LinearStaticPreviewRequest>(request(&model)).unwrap_err().to_string();
            assert!(typed.contains(&format!("unknown field `{field}`")), "{what}: {typed}");
        }
    }
}

#[test]
fn every_unknown_key_is_named_with_a_bounded_list() {
    let mut model = linear();
    for index in 0..12 {
        model["combinations"][0][format!("extra_{index:02}")] = json!(index);
    }
    for mode in MODES {
        let error = refusal(&model, mode, "twelve unknown keys");
        assert_eq!(error.matches("unknown field `extra_").count(), 8, "{error}");
        assert!(error.contains("; and 4 more."), "{error}");
    }
}

#[test]
fn a_duplicate_mechanics_term_blocks_by_name() {
    let mut model = linear();
    model["combinations"][0]["terms"][1]["load_case"] = json!("load:L-100");
    for mode in MODES {
        let envelope = solve(&model, mode).unwrap();
        let duplicate = envelope
            .diagnostics
            .iter()
            .find(|d| d.code == "LOAD_COMBINATION_DUPLICATE_TERM")
            .unwrap_or_else(|| panic!("{mode:?}: no duplicate-term refusal; diagnostics {:?}", codes(&envelope)));
        assert_eq!(duplicate.severity, "blocking", "{mode:?}");
        assert_eq!(duplicate.affected_refs, vec![COMBINATION.to_string(), "load:L-100".to_string()], "{mode:?}");
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{mode:?}");
        assert!(envelope.results.is_empty(), "{mode:?}: a blocked request publishes no rows");
    }
}

#[test]
fn three_repeated_terms_emit_unique_diagnostic_ids() {
    let mut model = linear();
    let term = model["combinations"][0]["terms"][0].clone();
    model["combinations"][0]["terms"] = json!([term.clone(), term.clone(), term]);
    for mode in MODES {
        let envelope = solve(&model, mode).unwrap();
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE");
        assert!(codes(&envelope).contains(&"LOAD_COMBINATION_DUPLICATE_TERM"));
        let ids: Vec<&str> = envelope.diagnostics.iter().map(|d| d.id.as_str()).collect();
        let unique: std::collections::HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len(), "{mode:?}: emitted diagnostic ids must be unambiguous");
    }
}

#[test]
fn three_repeated_range_operands_emit_unique_diagnostic_ids() {
    let mut model = linear();
    let case = model["combinations"][0]["terms"][0]["load_case"].clone();
    model["combinations"] = json!([{"id": "combination:range", "basis": "range_envelope",
        "operand_ids": [case.clone(), case.clone(), case], "mode": "max", "provenance": "silent-drop"}]);
    for mode in MODES {
        let envelope = solve(&model, mode).unwrap();
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE");
        let duplicates = envelope.diagnostics.iter().filter(|d| d.code == "LOAD_COMBINATION_DUPLICATE_TERM").count();
        assert_eq!(duplicates, 1, "{mode:?}: one diagnostic per repeated operand");
        let blocking: Vec<&str> = envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect();
        assert_eq!(blocking, ["LOAD_COMBINATION_DUPLICATE_TERM"], "{mode:?}: the repeat is the only refusal");
        let ids: Vec<&str> = envelope.diagnostics.iter().map(|d| d.id.as_str()).collect();
        let unique: std::collections::HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len(), "{mode:?}: emitted diagnostic ids must be unambiguous");
    }
}

#[test]
fn well_formed_documents_with_declared_ignored_members_publish() {
    // The demo carries data_boundary, diagnostics and load-case kind/label/status.
    let demo = demo();
    assert!(demo.get("data_boundary").is_some() && demo.get("diagnostics").is_some());
    for key in ["kind", "label", "status"] {
        assert!(demo["load_cases"][0].get(key).is_some(), "{key}");
    }
    for (name, model, combined) in [("demo", demo.clone(), false), ("linear", linear(), true)] {
        for mode in MODES {
            let envelope = solve(&model, mode).unwrap_or_else(|e| panic!("{name}/{mode:?}: {e}"));
            assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{name}/{mode:?}");
            let diagnostics = codes(&envelope);
            for code in ["LOAD_COMBINATION_ROW_WITHHELD", "LOAD_COMBINATION_SOURCE_RESULT_MISSING", "LOAD_COMBINATION_VECTOR_INVALID"] {
                assert!(!diagnostics.contains(&code), "{name}/{mode:?}: {code}");
            }
            assert_eq!(combination_rows(&envelope) > 0, combined, "{name}/{mode:?}");
            dump(name, mode, &envelope);
        }
    }
    // The declared members are ignored by the solve: their values do not move a byte.
    let mut varied = linear();
    varied["data_boundary"] = json!({"public_examples_policy": "varied"});
    varied["diagnostics"] = json!([]);
    for case in varied["load_cases"].as_array_mut().unwrap() {
        case["kind"] = json!("varied");
        case["label"] = json!("varied");
        case["status"] = json!("varied");
        case["tags"] = json!(["varied"]);
        case["notes"] = json!("varied");
    }
    // Model document 0.7.0's declared members (accepted and ignored).
    for member in ["lines", "unresolved_assumptions", "constraints", "equipment_interfaces", "rule_pack_refs"] {
        varied[member] = json!([{"id": format!("{member}:varied")}]);
    }
    for combination in varied["combinations"].as_array_mut().unwrap() {
        combination["tags"] = json!(["varied"]);
        combination["notes"] = json!("varied");
    }
    for mode in MODES {
        let base = serde_json::to_vec(&solve(&linear(), mode).unwrap()).unwrap();
        let other = serde_json::to_vec(&solve(&varied, mode).unwrap()).unwrap();
        assert_eq!(base, other, "{mode:?}");
    }
}
