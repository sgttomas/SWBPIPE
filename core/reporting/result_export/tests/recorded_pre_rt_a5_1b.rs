//! Operating-case slice 1b (DEL-05-02 Work `operating-case.md` "As built (1b)", recorded
//! runs): 1b restated the preview-physics combination policy (v2 -> v3), so the
//! preview-physics-2, -timoshenko-1 and -retained-1 digests moved. Main's statements and
//! records from before it (`fixtures/persistence/recorded_pre_rt_a5_1b_pf`, main at
//! bbf384dec verbatim, unchanged through 800fae7e5) read on the recorded-run entries under
//! the registry variant their record binds, exactly as they read then, and the current
//! binding names the new tables.
use open_pipe_stress_result_export::reader_verdict as rv;
use open_pipe_stress_result_export::recorded_tables::{self, SOURCE_RECORDED_BINDING_IDENTITY, SOURCE_RECORDED_BINDING_UNKNOWN};
use open_pipe_stress_result_export::retained_precision as rp;
use open_pipe_stress_result_export::semantic_contract as sc;
use serde_json::{json, Value};

const DIR: &str = "../../../../fixtures/persistence/recorded_pre_rt_a5_1b_pf";
const PF2: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a5_1b_pf/preview_physics_2/preview_physics_connected_sparse.json");
const WITNESS: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a5_1b_pf/preview_physics_2/recorded_run_witness.json");
const WITNESS_RULE_PACKS: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a5_1b_pf/preview_physics_2/recorded_run_witness_rule_packs.json");
const TIMOSHENKO: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a5_1b_pf/preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.json");
const PF_RETAINED: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a5_1b_pf/preview_physics_retained_1/retained_precision_milestone_successor_sparse_interactive.json");
const PF2_SHA: &str = "e7b93f416f700f8efdb31bf1ba26dedde52963d768d6ead5a73dbf1c645a360c";
const TIMOSHENKO_SHA: &str = "ce49ebe07bc05431ea1f770c1933b5bf58f4b1c8b66fb6f36810f2ef7a58cd43";
const PF_RETAINED_SHA: &str = "26dce9c21261b1ca894af05a3b6f4a94db88904894cb47f7296f892e72ecfb7e";

fn binding(id: &str, sha: &str) -> Value {
    json!({"id": id, "sha256": sha})
}
fn code(outcome: &Value) -> &str {
    outcome["error"]["code"].as_str().unwrap_or_default()
}

/// The records main wrote bind the pre-1b table; their statement reads on the recorded
/// entries under it, and the current entries name the new table.
#[test]
fn main_recorded_preview_physics_runs_read_under_their_recorded_table() {
    let _ = DIR;
    let source: Value = serde_json::from_str(PF2).unwrap();
    for record in [WITNESS, WITNESS_RULE_PACKS] {
        let record: Value = serde_json::from_str(record).unwrap();
        assert!(record.to_string().contains(PF2_SHA), "main's record binds e7b93f41");
        assert!(!record.to_string().contains(sc::PREVIEW_PHYSICS_SHA256), "and not the current table");
    }
    let recorded = binding(sc::PREVIEW_PHYSICS_ID, PF2_SHA);
    assert_ne!(rv::current_binding(sc::PREVIEW_PHYSICS_ID), Some(PF2_SHA));
    assert_eq!(rv::validate_recorded("preview-physics", &source, &recorded, None, None), json!({"ok": true}));
    assert_eq!(rv::transport_recorded("preview-physics", &source, &recorded), json!({"ok": true}));
    let recorded_read = rv::dispatch_recorded(&source, &recorded, None, None);
    assert_eq!((recorded_read["ok"].clone(), recorded_read["binding"].clone()), (json!(true), recorded.clone()));
    // Its rows and limitations are the current tables': the standing it reads with is the
    // one it read with on main, and the rows read the same.
    let current = rv::dispatch(&source, None, None);
    assert_eq!(current["binding"], binding(sc::PREVIEW_PHYSICS_ID, sc::PREVIEW_PHYSICS_SHA256));
    assert_eq!(recorded_read["standing"], current["standing"]);
    let rows = source["results"].as_array().unwrap();
    assert_eq!(rv::row_semantics_each_recorded(rows, &source, &recorded), rv::row_semantics_each(rows, Some(&source)));
    // Refused bindings stay refused.
    for bad in [binding(sc::PREVIEW_PHYSICS_ID, &"0".repeat(64)), json!({"id": sc::PREVIEW_PHYSICS_ID, "sha256": PF2_SHA, "x": 1})] {
        assert_eq!(code(&rv::validate_recorded("preview-physics", &source, &bad, None, None)), SOURCE_RECORDED_BINDING_UNKNOWN);
    }
    assert_eq!(code(&rv::validate_recorded("preview-physics", &source, &binding(sc::PREVIEW_PHYSICS_TIMOSHENKO_ID, TIMOSHENKO_SHA), None, None)), SOURCE_RECORDED_BINDING_IDENTITY);
}

#[test]
fn main_timoshenko_and_successor_statements_read_under_their_recorded_tables() {
    let source: Value = serde_json::from_str(TIMOSHENKO).unwrap();
    let recorded = binding(sc::PREVIEW_PHYSICS_TIMOSHENKO_ID, TIMOSHENKO_SHA);
    assert_eq!(rv::validate_recorded("preview-physics", &source, &recorded, None, None), json!({"ok": true}));
    assert_eq!(rv::dispatch_recorded(&source, &recorded, None, None)["binding"], recorded);

    let successor: Value = serde_json::from_str(PF_RETAINED).unwrap();
    let (source, invocation) = (successor["source"].clone(), successor["invocation"].clone());
    let recorded = binding(sc::PREVIEW_PHYSICS_RETAINED_ID, PF_RETAINED_SHA);
    assert_eq!(rv::retained_recorded(&source, &recorded, None)["ok"], true, "{}", rv::retained_recorded(&source, &recorded, None));
    assert_eq!(rv::retained_recorded(&source, &recorded, Some(&invocation))["ok"], true);
    assert_eq!(rv::retained_transport_recorded(&source, &recorded)["ok"], true);
    assert_eq!(rv::dispatch_recorded(&source, &recorded, None, None)["binding"], recorded);
    // Its base variant is main's preview-physics-2 table.
    let variant = recorded_tables::variant_of(&recorded).unwrap();
    assert_eq!(variant.base().unwrap().sha256, PF2_SHA);
    assert_eq!(variant.table()["inherited_semantic_contract_sha256"], PF2_SHA);
    // Nothing leaks: the current reader's verdict is unchanged by the recorded read.
    let before = rp::outcome_json(&rp::validate(&source, None));
    let _ = rv::retained_recorded(&source, &recorded, None);
    assert_eq!(rp::outcome_json(&rp::validate(&source, None)), before);
}
