//! The engine's column of the hanger-kind case table
//! (`fixtures/results/hanger_sizing_engine_kind_corpus.json`), observed through the
//! product's public solve entry, the one this crate's derived solves use. The
//! desktop's hanger-write predicate test asserts its mirror against the same table;
//! this test checks that the table's "engine" value is what the engine does.
//!
//! Each case's support value is put, unchanged, at the free end of a 2 m cantilever
//! anchored at its other end, with one load case. Two solves observe it:
//!
//! 1. **Hanger kind or not.** The plain model. The engine validates a support's
//!    hanger data exactly when it classifies the support as a variable spring
//!    hanger or a constant-effort support (`validate_spring_hangers`), and every
//!    finding of that validation is a `SPRING_HANGER_*` or `CONSTANT_EFFORT_*`
//!    diagnostic naming the support. So a hanger kind is seen by at least one such
//!    diagnostic. A support of neither kind gets none and is solved as an ordinary
//!    support, publishing its reaction rows.
//! 2. **Which kind.** The same model with an imposed displacement on the support's
//!    UX, a DOF no case's support acts on. The engine refuses it before solving,
//!    and how it refuses follows from the classification: a constant-effort device
//!    has no base on any DOF (`IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID`, "a
//!    constant-effort device"); a spring-family record, which a variable spring
//!    hanger is, moves only its own spring axis
//!    (`IMPOSED_DISPLACEMENT_DOF_UNRESTRAINED`, "spring support"); any other record
//!    is a rigid support that does not restrain UX (the same code, "does not
//!    restrain"). A hanger kind that is not a constant-effort device is therefore
//!    a variable spring hanger, and its spring wording corroborates it.
//!
//! These suffice even where validation stops early (no hanger member, or a blank
//! hanger type), where the shared validation codes alone could not tell the two
//! hanger kinds apart. `document_refused` is observed as the entry's refusal of
//! the document at decode ("invalid type", or "invalid length" for a hanger given
//! as an array longer than its members). A hanger given as a shorter array is
//! decoded by position, its first element the hanger type, and is classified like
//! any other value.

#[path = "common/model_builder.rs"]
mod model_builder;

use model_builder::*;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, MechanicsEnvelope, PreviewSolverMode};
use serde_json::{json, Value};

const CORPUS: &str = include_str!("../../../../fixtures/results/hanger_sizing_engine_kind_corpus.json");

fn model_with(support_value: &Value, imposed_ux: bool) -> Value {
    let member = Member { id: "pipe:AB", from: "node:A", to: "node:B", outside_diameter_m: 0.2, wall_m: 0.01,
                          elastic_modulus_pa: 2.0e11, expansion_per_k: 1.2e-5 };
    let mut loads = vec![nodal_fz("load:W-f", "node:B", -1000.0)];
    if imposed_ux {
        loads.push(json!({"id": "load:W-ux", "category": "imposed_displacement",
            "target": {"type": "support", "support": support_value["id"], "dof": "UX"}, "direction": "UX",
            "magnitude": {"value": 0.001, "unit": "m"}, "dimension": "displacement", "provenance": "invented_test_value"}));
    }
    request(
        &[("node:A", [0.0, 0.0, 0.0]), ("node:B", [2.0, 0.0, 0.0])],
        &[member],
        vec![support("support:anchor", "node:A", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"]), support_value.clone()],
        vec![case("load:W", loads)],
    )
}

fn solve(doc: Value) -> Result<MechanicsEnvelope, String> {
    run_linear_static_preview_value_with_mode(doc, PreviewSolverMode::DenseScrutiny)
}

fn naming<'a>(envelope: &'a MechanicsEnvelope, id: &'a str) -> impl Iterator<Item = &'a open_pipe_stress_product_physics::Diagnostic> + 'a {
    envelope.diagnostics.iter().filter(move |d| d.affected_refs.iter().any(|r| r == id))
}

/// The engine's classification of one support value, by the two observations.
fn observed(support_value: &Value, name: &str) -> &'static str {
    let id = support_value["id"].as_str().unwrap();
    let plain = match solve(model_with(support_value, false)) {
        Err(message) => {
            assert!(message.contains("invalid type") || message.contains("invalid length"), "{name}: refused, but not at decode: {message}");
            return "document_refused";
        }
        Ok(envelope) => envelope,
    };
    let hanger_kind = naming(&plain, id).any(|d| d.code.starts_with("SPRING_HANGER_") || d.code.starts_with("CONSTANT_EFFORT_"));
    if !hanger_kind {
        assert!(plain.results.iter().any(|r| r.entity_ref == id && r.kind == "support_reaction_component_v2"),
            "{name}: neither hanger kind, yet not solved as an ordinary support: {:?}", plain.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    }
    let moved = solve(model_with(support_value, true)).unwrap_or_else(|e| panic!("{name}: the imposed-UX model is refused: {e}"));
    let refusal = naming(&moved, id)
        .find(|d| d.code.starts_with("IMPOSED_DISPLACEMENT_"))
        .unwrap_or_else(|| panic!("{name}: no imposed-displacement refusal names the support"));
    let constant_device = refusal.code == "IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID";
    if constant_device {
        assert!(refusal.message.contains("constant-effort device"), "{name}: {}", refusal.message);
        assert!(hanger_kind, "{name}: a constant-effort device whose hanger data was not validated");
        return "constant_effort";
    }
    assert_eq!(refusal.code, "IMPOSED_DISPLACEMENT_DOF_UNRESTRAINED", "{name}: {}", refusal.message);
    if hanger_kind {
        assert!(refusal.message.contains("spring support"), "{name}: a hanger kind not treated as a spring: {}", refusal.message);
        "variable_spring"
    } else {
        "none"
    }
}

#[test]
fn the_engine_classifies_every_support_value_of_the_case_table_as_the_table_states() {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let cases = corpus["cases"].as_array().unwrap();
    // The 26 cases of the table's first form, and five with the hanger given as an array.
    assert_eq!(cases.len(), 31);
    let mut disagreements = Vec::new();
    for case in cases {
        let name = case["name"].as_str().unwrap();
        let engine = observed(&case["support"], name);
        if case["engine"] != engine {
            disagreements.push(format!("{name}: the table says {}, the engine did {engine}", case["engine"]));
        }
    }
    assert!(disagreements.is_empty(), "{disagreements:#?}");
}
