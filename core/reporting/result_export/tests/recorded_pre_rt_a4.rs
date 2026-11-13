//! RT-A4 §10.4 (DEL-05-02 Design `absolute-sum-srss-combinations.md`, "Recorded results from
//! before RT-A4 (historical read)"): main's statements recorded before RT-A4 read under their
//! recorded table variants on the recorded-run read entries only, and stay refused on every
//! current entry. RT-L2 (DEL-04-07): the recorded exact successors are successors of
//! pressure-2 results, which predate the pressure-elongation setting; they are refused by
//! name (`SOURCE_PRESSURE_2_RETIRED`) on every entry, recorded or current. Fixtures: `fixtures/persistence/recorded_pre_rt_a4_pf` (main 6c3cb46d3,
//! #101 01e05a486 and, for the exact successor RT-A3 increment 4 makes history, main
//! bbf384dec, verbatim). At a4ff570dc there was no recorded entry: every current entry
//! below refused them, as it still does.
use open_pipe_stress_result_export::reader_verdict as rv;
use open_pipe_stress_result_export::recorded_tables::{self, SOURCE_RECORDED_BINDING_IDENTITY, SOURCE_RECORDED_BINDING_UNKNOWN};
use open_pipe_stress_result_export::retained_precision as rp;
use open_pipe_stress_result_export::semantic_contract as sc;
use serde_json::{json, Value};

const PF2: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a4_pf/preview_physics_2/preview_physics_connected_sparse.json");
const TIMOSHENKO: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a4_pf/preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.json");
const PF_RETAINED: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a4_pf/preview_physics_retained_1/retained_precision_milestone_successor_sparse_interactive.json");
const EXACT_MAIN: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/main/retained_precision_exact_successor_sparse_interactive.json");
const EXACT_101: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/pr101/retained_precision_exact_successor_sparse_interactive.json");
const EXACT_RT_A4: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/main_bbf384dec/retained_precision_exact_successor_sparse_interactive.json");
// RT-A3 §4 (the combination certificate) restates pressure-2, so main's exact successor table at 252bef969 is history.
const EXACT_CERT: &str = include_str!("../../../../fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/main_252bef969/retained_precision_exact_successor_sparse_interactive.json");
const CURRENT_PF2: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");

fn statement(text: &str) -> Value {
    let document: Value = serde_json::from_str(text).unwrap();
    if document.get("producer").is_some() { document } else { document["source"].clone() }
}
fn binding(id: &str, sha: &str) -> Value {
    json!({"id": id, "sha256": sha})
}
const PF2_SHA: &str = "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2";
const TIMOSHENKO_SHA: &str = "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b";
const PF_RETAINED_SHA: &str = "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46";
const EXACT_MAIN_SHA: &str = "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd";
const EXACT_101_SHA: &str = "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b";
const EXACT_RT_A4_SHA: &str = "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034";
const EXACT_CERT_SHA: &str = "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691";

fn code(outcome: &Value) -> &str {
    outcome["error"]["code"].as_str().unwrap_or_default()
}

#[test]
fn preview_physics_statements_recorded_before_rt_a4_read_under_their_variant_only() {
    for (text, sha) in [(PF2, PF2_SHA), (TIMOSHENKO, TIMOSHENKO_SHA)] {
        let source = statement(text);
        let id = source["producer"]["semantic_contract_id"].as_str().unwrap();
        let recorded = binding(id, sha);
        // Current entries (a4ff570dc's only entries): refused.
        assert_eq!(code(&rv::validate("preview-physics", &source, None, None)), "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
        assert!(code(&rv::transport("preview-physics", &source)).starts_with("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID"));
        let current = rv::dispatch(&source, None, None);
        assert_eq!(current["binding"], binding(id, rv::current_binding(id).unwrap()));
        assert!(current["standing"]["findings"].as_array().unwrap().contains(&json!("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS")));
        assert_eq!(sc::numerical_use_standing(&source, &[]), "unsupported");
        assert!(sc::for_source(&source).is_err());
        // The recorded entries under the record's binding: read as main read them.
        assert_eq!(rv::validate_recorded("preview-physics", &source, &recorded, None, None), json!({"ok": true}));
        assert_eq!(rv::transport_recorded("preview-physics", &source, &recorded), json!({"ok": true}));
        let dispatched = rv::dispatch_recorded(&source, &recorded, None, None);
        assert_eq!((dispatched["ok"].clone(), dispatched["binding"].clone()), (json!(true), recorded.clone()));
        assert!(!dispatched["standing"]["findings"].as_array().unwrap().contains(&json!("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS")));
        let rows = source["results"].as_array().unwrap();
        let each = rv::row_semantics_each_recorded(rows, &source, &recorded);
        assert_eq!(each, rv::row_semantics_each(rows, Some(&source)), "rows read the same: the variant's rows are the current table's");
        // Nothing leaks past the recorded read.
        assert_eq!(code(&rv::validate("preview-physics", &source, None, None)), "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
        assert_eq!(rv::dispatch(&source, None, None)["binding"], current["binding"]);
        // The current binding on the recorded entry reads as the current entry does.
        let present = binding(id, rv::current_binding(id).unwrap());
        assert_eq!(code(&rv::validate_recorded("preview-physics", &source, &present, None, None)), "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
        // Wrong or unknown digest, an extra member, a variant of another identity.
        for bad in [binding(id, &"0".repeat(64)), json!({"id": id, "sha256": sha, "signature_id": null}), json!(sha)] {
            assert_eq!(code(&rv::validate_recorded("preview-physics", &source, &bad, None, None)), SOURCE_RECORDED_BINDING_UNKNOWN);
            assert_eq!(rv::dispatch_recorded(&source, &bad, None, None)["route"], "unsupported");
        }
        let other = if sha == PF2_SHA { binding(sc::PREVIEW_PHYSICS_TIMOSHENKO_ID, TIMOSHENKO_SHA) } else { binding(sc::PREVIEW_PHYSICS_ID, PF2_SHA) };
        assert_eq!(code(&rv::validate_recorded("preview-physics", &source, &other, None, None)), SOURCE_RECORDED_BINDING_IDENTITY);
        // A source byte tampered: the variant's reader still checks every evidence member.
        let mut tampered = source.clone();
        tampered["formulation_basis"]["limitations"][6] = json!("restated");
        assert_eq!(code(&rv::validate_recorded("preview-physics", &tampered, &recorded, None, None)), "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
    }
}

#[test]
fn a_current_statement_under_a_variant_binding_is_refused() {
    // Old digest, new limitations: today's statement does not read under a pre-RT-A4 variant.
    let source = statement(CURRENT_PF2);
    let id = source["producer"]["semantic_contract_id"].as_str().unwrap();
    assert_eq!(rv::validate("preview-physics", &source, None, None), json!({"ok": true}));
    assert_eq!(code(&rv::validate_recorded("preview-physics", &source, &binding(id, PF2_SHA), None, None)), "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
    assert_eq!(rv::validate_recorded("preview-physics", &source, &binding(id, rv::current_binding(id).unwrap()), None, None), json!({"ok": true}));
}

#[test]
fn successors_recorded_before_rt_a4_read_under_their_variant_only() {
    let source = statement(PF_RETAINED);
    let invocation: Value = serde_json::from_str::<Value>(PF_RETAINED).unwrap()["invocation"].clone();
    let recorded = binding(sc::PREVIEW_PHYSICS_RETAINED_ID, PF_RETAINED_SHA);
    let current = rp::outcome_json(&rp::validate(&source, None));
    assert_eq!((current["ok"].clone(), current["error"]["code"].clone()), (json!(false), json!("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS")));
    assert_eq!(rp::outcome_json(&rp::validate_transport_metadata(&source))["ok"], false);
    assert_eq!(rv::retained_recorded(&source, &recorded, None)["ok"], true);
    assert_eq!(rv::retained_recorded(&source, &recorded, Some(&invocation))["ok"], true);
    assert_eq!(rv::retained_transport_recorded(&source, &recorded)["ok"], true);
    assert_eq!(rv::dispatch_recorded(&source, &recorded, None, None)["binding"], recorded);
    // Under the wrong variant (the base's, or an exact successor's), refused at G0 by name.
    for (wrong, expected) in [(binding(sc::PREVIEW_PHYSICS_ID, PF2_SHA), SOURCE_RECORDED_BINDING_IDENTITY),
                              (binding(sc::PHYSICS_RETAINED_ID, EXACT_MAIN_SHA), SOURCE_RECORDED_BINDING_IDENTITY),
                              (binding(sc::PREVIEW_PHYSICS_RETAINED_ID, &"0".repeat(64)), SOURCE_RECORDED_BINDING_UNKNOWN)] {
        let outcome = rv::retained_recorded(&source, &wrong, None);
        assert_eq!((outcome["error"]["gate"].clone(), outcome["error"]["code"].clone()), (json!("G0"), json!(expected)), "{wrong}");
    }
    // Nothing leaks: the current reader still refuses it after a recorded read.
    assert_eq!(rp::outcome_json(&rp::validate(&source, None))["ok"], false);

    // The exact successors recorded here are every one a successor of a pressure-2 result
    // (their base identity statement names pressure-2 semantics). RT-L2 (DEL-04-07): they
    // predate the pressure-elongation setting and are refused by name with the pressure-2
    // message, on the current read and on the recorded read under their variant alike.
    let retired = json!({"gate": "G0", "code": sc::SOURCE_PRESSURE_2_RETIRED, "detail": sc::SOURCE_PRESSURE_2_RETIRED_TEXT});
    assert_eq!(
        sc::SOURCE_PRESSURE_2_RETIRED_TEXT,
        "SOURCE_PRESSURE_2_RETIRED: pressure-2 results predate the pressure elongation setting and are no longer read; re-solve the model"
    );
    for (text, sha) in [(EXACT_MAIN, EXACT_MAIN_SHA), (EXACT_101, EXACT_101_SHA), (EXACT_RT_A4, EXACT_RT_A4_SHA), (EXACT_CERT, EXACT_CERT_SHA)] {
        let source = statement(text);
        assert!(source["formulation_basis"]["limitations"][0].as_str().unwrap().starts_with("3.0.0/exact_pressure_v3 under pressure-2 semantics."));
        let recorded = binding(sc::PHYSICS_RETAINED_ID, sha);
        let current = rp::outcome_json(&rp::validate(&source, None));
        assert_eq!((current["ok"].clone(), current["error"].clone()), (json!(false), retired.clone()), "{sha}: current read");
        let read = rv::retained_recorded(&source, &recorded, None);
        assert_eq!((read["ok"].clone(), read["error"].clone()), (json!(false), retired.clone()), "{sha}: recorded read");
        for dispatched in [rv::dispatch_recorded(&source, &recorded, None, None), rv::dispatch(&source, None, None)] {
            assert_eq!(dispatched["ok"], false, "{sha}: {dispatched}");
            assert_eq!(dispatched["error"]["code"], sc::SOURCE_PRESSURE_2_RETIRED, "{sha}: {dispatched}");
        }
        // The binding is still checked first: under another identity's variant, G0 refuses the binding.
        let wrong = binding(sc::PREVIEW_PHYSICS_RETAINED_ID, PF_RETAINED_SHA);
        assert_eq!(rv::retained_recorded(&source, &wrong, None)["error"]["code"], SOURCE_RECORDED_BINDING_IDENTITY);
    }
}

#[test]
fn the_registry_is_closed_and_each_variant_is_its_pinned_bytes() {
    let shas: Vec<&str> = recorded_tables::VARIANTS.iter().map(|v| v.sha256).collect();
    assert_eq!(shas, [PF2_SHA, TIMOSHENKO_SHA, PF_RETAINED_SHA, EXACT_MAIN_SHA, EXACT_101_SHA, EXACT_RT_A4_SHA,
        // Operating-case slice 1b: main's tables at bbf384dec (`recorded_pre_rt_a5_1b.rs`).
        "e7b93f416f700f8efdb31bf1ba26dedde52963d768d6ead5a73dbf1c645a360c",
        "ce49ebe07bc05431ea1f770c1933b5bf58f4b1c8b66fb6f36810f2ef7a58cd43",
        "26dce9c21261b1ca894af05a3b6f4a94db88904894cb47f7296f892e72ecfb7e",
        // RT-A5 minuend_final: main's tables at b32a17761 (`recorded_pre_rt_a5_mf.rs`).
        "f41b9d3ed0c1b7abaee0b98ca1a28aa1a2d109bce443b98d53a2080c1c504ce9",
        "6f6494907e3ead979ea27e5ff85f54617589c9195dc84d40a02752a0b006ef44",
        "b4377e73e8a83edb57b4f0c41dbaef384385e0f6b4810103a106a334c8edbe14",
        // RT-A3 §4: main's exact successor table at 252bef969.
        EXACT_CERT_SHA,
        // R-6/R-7 S-K1: main's tables at 9e5c69797, before S-K1 (`recorded_pre_sk1/`).
        "563db3c7c44279face3adc60a76a512de38520357e89ea4b49ad11d20946db89",
        "f64473f5a60ac2bb34d8006b40bf044800f0c97454c570bbe88d1f1d96b7bdfc",
        "d0bab3502b4018ce0596757a933bd6306f402dc3741a10af24a094e8c66544e8"]);
    for variant in &recorded_tables::VARIANTS {
        assert_eq!(variant.table()["semantic_contract_id"], variant.id);
        assert!(recorded_tables::variant_of(&binding(variant.id, variant.sha256)).is_some());
        assert_ne!(rv::current_binding(variant.id), Some(variant.sha256));
    }
}
