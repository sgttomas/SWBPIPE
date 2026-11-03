//! RT-U4: `SOURCE_PHYSICS_REGION_RESULT_BINDING`, the binding of a pressure
//! region's `result_ids` to the published pipe rows of its case and members.
//! The reader counts those rows once per document and checks each region's ids
//! against the counts (`physics_evidence.rs`); before RT-U4 it rescanned every
//! row for every region. Each mutation's expected outcome below is the pre-change
//! reader's (taken on main 6c3cb46d3 before the change): the evidence reader's
//! first refusal, and the `pressure` verdict and dispatch standing it gives.
use open_pipe_stress_result_export::reader_verdict as rv;
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};

fn corpus(text: &str, index: usize) -> Value {
    let corpus: Value = serde_json::from_str(text).unwrap();
    corpus["cases"][index]["envelope"].clone()
}
/// One straight member, two cases, a region in one (X0).
fn straight() -> Value {
    corpus(include_str!("../../../../fixtures/results/pressure_v3_straight_reader_corpus.json"), 0)
}
/// A load-state document: one member, a region in each of its two cases (Y0).
fn load_state() -> Value {
    corpus(include_str!("../../../../fixtures/results/pressure_v3_straight_reader_corpus.json"), 2)
}
/// One region of three members, one a realized arc.
fn arc() -> Value {
    corpus(include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json"), 0)
}
/// Three cases; case C has two one-member regions (m1, m2), case A a region of m1.
fn combination() -> Value {
    corpus(include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json"), 0)
}

fn region<'a>(doc: &'a mut Value, case: &str, member: &str) -> &'a mut Value {
    doc["contract_evidence"]["pressure"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["load_case_id"] == case && r["member_pipe_ids"][0] == member)
        .unwrap()
}
fn ids<'a>(doc: &'a mut Value, case: &str, member: &str) -> &'a mut Vec<Value> {
    region(doc, case, member)["result_ids"].as_array_mut().unwrap()
}
/// The id of the first published row of `case` and `entity` whose kind `pick` takes.
fn row_id(doc: &Value, case: &str, entity: &str, pick: impl Fn(&str) -> bool) -> Value {
    doc["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["basis_ref"]["ref_id"] == case && r["entity_ref"] == entity && pick(r["kind"].as_str().unwrap()))
        .map(|r| r["id"].clone())
        .unwrap()
}
/// The region binding's kinds (`physics_evidence::pipe_kind`).
const PIPE: fn(&str) -> bool = |k| {
    matches!(k, "pipe_wall_endpoint_action_v2" | "pipe_wall_axial_force_v2" | "pipe_effective_axial_force_v2"
        | "pipe_axial_membrane_stress_v2" | "pipe_lame_radial_stress_v2" | "pipe_lame_hoop_stress_v2"
        | "pipe_elastic_normal_stress_maximum_v2")
};
const WALL: fn(&str) -> bool = |k| k == "pipe_wall_axial_force_v2";
const NON_PIPE: fn(&str) -> bool = |k| k == "element_local_shear_force_y";

/// What the pre-change reader gives: the evidence reader's outcome, the `pressure`
/// verdict and the dispatch standing.
fn outcome(doc: &Value) -> Value {
    let evidence = match s::validate_pressure_evidence(doc) {
        Ok(()) => Value::Null,
        Err(e) => Value::from(e),
    };
    json!({"evidence": evidence, "verdict": rv::validate("pressure", doc, None, None),
        "dispatch": rv::dispatch(doc, None, None)})
}

/// (label, document, the pre-change evidence outcome: null for admitted, else its text).
fn cases() -> Vec<(&'static str, Value, Value)> {
    const BINDING: &str = "SOURCE_PHYSICS_REGION_RESULT_BINDING";
    let lr = |code: &str| json!(format!("SOURCE_LOAD_REFERENCE_PHYSICS_EVIDENCE: {code}"));
    let mut out: Vec<(&'static str, Value, Value)> = Vec::new();
    let mut add = |label, base: Value, edit: &dyn Fn(&mut Value), expected: Value| {
        let mut doc = base;
        edit(&mut doc);
        out.push((label, doc, expected));
    };
    // Admitted sources, including a member in a region of each of two cases and a
    // three-member region (the per-region sum over members).
    add("straight admitted", straight(), &|_| {}, Value::Null);
    add("load state admitted", load_state(), &|_| {}, Value::Null);
    add("arc admitted", arc(), &|_| {}, Value::Null);
    add("combination admitted", combination(), &|_| {}, Value::Null);
    // An id missing; none at all (the member's count is not zero).
    add("missing id", straight(), &|d| {
        ids(d, "case:closed-pressure", "pipe:fixture-span").pop();
    }, json!(BINDING));
    add("no ids", straight(), &|d| ids(d, "case:closed-pressure", "pipe:fixture-span").clear(), json!(BINDING));
    add("missing id, three members", arc(), &|d| {
        ids(d, "case:u2", "pipe:S1").remove(0);
    }, json!(BINDING));
    // An unexpected id, in place of a bound one (same count) and added (one more).
    add("another case's id, replaced", straight(), &|d| {
        let other = row_id(d, "case:six-component-load", "pipe:fixture-span", PIPE);
        ids(d, "case:closed-pressure", "pipe:fixture-span")[0] = other;
    }, json!(BINDING));
    add("another case's id, added", straight(), &|d| {
        let other = row_id(d, "case:six-component-load", "pipe:fixture-span", PIPE);
        ids(d, "case:closed-pressure", "pipe:fixture-span").push(other);
    }, json!(BINDING));
    add("another case's id, same member in both cases' regions", load_state(), &|d| {
        let other = row_id(d, "case:hot-pressure", "pipe:first", PIPE);
        ids(d, "case:cold-pressure", "pipe:first")[0] = other;
    }, lr(BINDING));
    add("another member's id, replaced", combination(), &|d| {
        let other = row_id(d, "case:C", "pipe:m2", PIPE);
        ids(d, "case:C", "pipe:m1")[0] = other;
    }, json!(BINDING));
    add("another member's id, added", combination(), &|d| {
        let other = row_id(d, "case:C", "pipe:m2", PIPE);
        ids(d, "case:C", "pipe:m1").push(other);
    }, json!(BINDING));
    add("another case's id, multi-region case", combination(), &|d| {
        let other = row_id(d, "case:A", "pipe:m1", PIPE);
        ids(d, "case:C", "pipe:m1")[0] = other;
    }, json!(BINDING));
    add("non-pipe kind, replaced", straight(), &|d| {
        let other = row_id(d, "case:closed-pressure", "pipe:fixture-span", NON_PIPE);
        ids(d, "case:closed-pressure", "pipe:fixture-span")[0] = other;
    }, json!(BINDING));
    add("non-pipe kind, added", arc(), &|d| {
        let other = row_id(d, "case:u2", "pipe:S2", NON_PIPE);
        ids(d, "case:u2", "pipe:S1").push(other);
    }, json!(BINDING));
    add("unknown id, replaced", straight(), &|d| {
        ids(d, "case:closed-pressure", "pipe:fixture-span")[0] = json!("result:unknown");
    }, json!(BINDING));
    add("unknown id, added", straight(), &|d| {
        ids(d, "case:closed-pressure", "pipe:fixture-span").push(json!("result:unknown"));
    }, json!(BINDING));
    // Malformed and duplicated references: refused by the id list's own reading,
    // before the binding.
    add("duplicate id", straight(), &|d| {
        let list = ids(d, "case:closed-pressure", "pipe:fixture-span");
        list.push(list[0].clone());
    }, json!("SOURCE_PHYSICS_DUPLICATE_ID"));
    add("duplicate id in place of another", straight(), &|d| {
        let list = ids(d, "case:closed-pressure", "pipe:fixture-span");
        list[1] = list[0].clone();
    }, json!("SOURCE_PHYSICS_DUPLICATE_ID"));
    add("non-string id", straight(), &|d| {
        ids(d, "case:closed-pressure", "pipe:fixture-span")[0] = json!(7);
    }, json!("SOURCE_PHYSICS_STRING_INVALID"));
    add("empty id", straight(), &|d| {
        ids(d, "case:closed-pressure", "pipe:fixture-span")[0] = json!("");
    }, json!("SOURCE_PHYSICS_STRING_INVALID"));
    add("result_ids not an array", straight(), &|d| {
        region(d, "case:closed-pressure", "pipe:fixture-span")["result_ids"] = json!({"a": 1});
    }, json!("SOURCE_PHYSICS_ARRAY_INVALID"));
    add("result_ids absent", straight(), &|d| {
        region(d, "case:closed-pressure", "pipe:fixture-span").as_object_mut().unwrap().remove("result_ids");
    }, json!("SOURCE_PHYSICS_REGION_SHAPE"));
    // Members shared across regions of one case: refused before the binding.
    add("one member in two regions of a case", combination(), &|d| {
        let r = region(d, "case:C", "pipe:m2");
        r["member_pipe_ids"] = json!(["pipe:m1"]);
    }, json!("SOURCE_PHYSICS_REGION_MEMBER_OVERLAP"));
    // Rows missing and extra: a bound row removed, a pipe row added.
    add("bound maximum row removed", straight(), &|d| {
        let id = ids(d, "case:closed-pressure", "pipe:fixture-span")[0].clone();
        d["results"].as_array_mut().unwrap().retain(|r| r["id"] != id);
    }, json!("SOURCE_PHYSICS_EXTREMA_RESULT_MISSING"));
    add("bound wall-force row removed", straight(), &|d| {
        let id = row_id(d, "case:closed-pressure", "pipe:fixture-span", WALL);
        d["results"].as_array_mut().unwrap().retain(|r| r["id"] != id);
    }, json!(BINDING));
    add("pipe row added", straight(), &|d| {
        let rows = d["results"].as_array_mut().unwrap();
        let mut extra = rows.iter().find(|r| WALL(r["kind"].as_str().unwrap())).unwrap().clone();
        extra["id"] = json!("result:extra");
        rows.push(extra);
    }, json!("SOURCE_PHYSICS_ROW_SLOT_DUPLICATE"));
    add("wall-force row of another case moved to the region's", load_state(), &|d| {
        let id = row_id(d, "case:hot-pressure", "pipe:first", WALL);
        ids(d, "case:cold-pressure", "pipe:first").push(id.clone());
        for r in d["results"].as_array_mut().unwrap() {
            if r["id"] == id {
                r["basis_ref"]["ref_id"] = json!("case:cold-pressure");
            }
        }
    }, lr("SOURCE_PHYSICS_ROW_SLOT_DUPLICATE"));
    // First-error order: across regions (case C's regions in array order), and
    // against the same region's earlier checks and the checks after the loop.
    add("binding in the first region, profile in the second", combination(), &|d| {
        ids(d, "case:C", "pipe:m1").pop();
        region(d, "case:C", "pipe:m2")["profile_mode"] = json!("other");
    }, json!(BINDING));
    add("profile in the first region, binding in the second", combination(), &|d| {
        region(d, "case:C", "pipe:m1")["profile_mode"] = json!("other");
        ids(d, "case:C", "pipe:m2").pop();
    }, json!("SOURCE_PHYSICS_REGION_PROFILE"));
    add("binding in the first case's region, duplicate in the second's", load_state(), &|d| {
        ids(d, "case:cold-pressure", "pipe:first").pop();
        let list = ids(d, "case:hot-pressure", "pipe:first");
        list.push(list[0].clone());
    }, lr(BINDING));
    add("binding, then the same region's terminals", straight(), &|d| {
        ids(d, "case:closed-pressure", "pipe:fixture-span").pop();
        region(d, "case:closed-pressure", "pipe:fixture-span")["terminals"] = json!([]);
    }, json!("SOURCE_PHYSICS_TERMINAL_COVERAGE"));
    add("binding before a later region's overlap", combination(), &|d| {
        ids(d, "case:C", "pipe:m1").pop();
        region(d, "case:C", "pipe:m2")["member_pipe_ids"] = json!(["pipe:m1"]);
    }, json!(BINDING));
    out
}

#[test]
fn region_result_binding_keeps_the_pre_change_outcomes() {
    let (mut dump, mut mismatches) = (Vec::new(), Vec::new());
    for (label, doc, expected) in cases() {
        let got = outcome(&doc);
        if std::env::var_os("RT_U4_PRINT").is_some() {
            println!("{label}: {}", got["evidence"]);
        }
        dump.push(json!({"label": label, "outcome": got}));
        if got["evidence"] != expected {
            mismatches.push(format!("{label}: {} (expected {expected})", got["evidence"]));
            continue;
        }
        // The verdict carries the evidence outcome unchanged; the standing names it.
        let verdict = &got["verdict"];
        assert_eq!(verdict["ok"], expected.is_null(), "{label}");
        let findings = got["dispatch"]["standing"]["findings"].as_array().unwrap();
        if let Some(code) = expected.as_str() {
            assert_eq!(rv::error_json(code), verdict["error"], "{label}");
            assert_eq!(findings[0], code, "{label}");
            assert_eq!(got["dispatch"]["standing"]["token"], "unsupported", "{label}");
        } else {
            assert!(findings.iter().all(|f| !f.as_str().unwrap().starts_with("SOURCE_")), "{label}");
        }
    }
    // The whole outcome set, for a byte comparison against another build.
    if let Some(path) = std::env::var_os("RT_U4_DUMP") {
        std::fs::write(path, serde_json::to_vec_pretty(&dump).unwrap()).unwrap();
    }
    assert!(mismatches.is_empty(), "{mismatches:#?}");
}
