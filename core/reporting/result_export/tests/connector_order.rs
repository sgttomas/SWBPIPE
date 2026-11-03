//! RT-U4 (order): the connector reader's first refusal is a function of the
//! source, never of a hash seed. Each envelope below has two defects with
//! different refusal codes in one scan of `connector_evidence`; before the
//! repair the scan walked a `HashMap`/`HashSet`, so the reported code varied
//! across processes and between `dispatch`'s two evidence passes. Every call
//! below builds fresh maps and sets (a new `RandomState` each), so repeated
//! calls in one process see different hash orders; the first refusal must not
//! move, and it follows the documented source array order.
//!
//! The two `find` lookups in `validate_pressure` select one row per slot. A
//! second candidate (equal or conflicting) is refused as `ROW_DUPLICATE` by
//! the row loop before either lookup runs, in both insertion orders.
use open_pipe_stress_result_export::reader_verdict as rv;
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};

const REPEATS: usize = 32;

fn connector_envelope() -> Value {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_connector_reader_corpus.json")).unwrap();
    corpus["cases"][0]["envelope"].clone()
}

/// Two tie rods (`rod-yp`, `rod-ym`), unpressurized, cases `load:U5A` then
/// `case:second` in `exact_cases` order, with a combination (W16B MO-F).
fn rods_envelope() -> Value {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_combination_reader_corpus.json")).unwrap();
    let entry = corpus["cases"].as_array().unwrap().iter().find(|c| c["label"] == "W16B MO-F: two tie rods withheld by name").unwrap();
    entry["envelope"].clone()
}

fn rows(envelope: &mut Value) -> &mut Vec<Value> {
    envelope["results"].as_array_mut().unwrap()
}

fn position(envelope: &Value, id: &str) -> usize {
    envelope["results"].as_array().unwrap().iter().position(|r| r["id"] == id).unwrap_or_else(|| panic!("no row {id}"))
}

/// The first refusal of `validate_pressure_evidence`, the same on every call,
/// and `dispatch`'s verdict, the same on every call.
fn stable_refusal(envelope: &Value) -> String {
    let first = s::validate_pressure_evidence(envelope).expect_err("the envelope is refused");
    let verdict = rv::dispatch(envelope, None, None);
    for _ in 0..REPEATS {
        assert_eq!(s::validate_pressure_evidence(envelope).unwrap_err(), first);
        assert_eq!(rv::dispatch(envelope, None, None), verdict);
    }
    first
}

const CONNECTOR: &str = "result:connector:component-C-J:";
const SECOND: &str = "result:loadcase:case-second:connector:component-C-J:";
const ROD_YP: &str = "tie-rod-force:tie_rod:rod-yp:t_r";
const END_FX: &str = "connector-endpoint-force:end_i:fx";
const S_X: &str = "connector-effective-axial-force:connector_local:s_x";

#[test]
fn controls_admit() {
    s::validate_pressure_evidence(&connector_envelope()).unwrap();
    s::validate_pressure_evidence(&rods_envelope()).unwrap();
}

/// `validate`'s row loop (`for row in published`): a second row in an
/// occupied slot (ROW_DUPLICATE) and a row bound to no record (ROW_UNBOUND).
/// The one later in `results` is reported second.
#[test]
fn row_loop_reports_the_first_defect_in_results_order() {
    let base = connector_envelope();
    let original = base["results"].as_array().unwrap().iter().filter(|r| r["kind"] == "connector_generalized_force_v1").cloned().collect::<Vec<_>>();
    let mut duplicate = original[0].clone();
    duplicate["id"] = json!("result:probe:duplicate");
    let mut unbound = original[1].clone();
    unbound["id"] = json!("result:probe:unbound");
    unbound["entity_ref"] = json!("component:probe-unknown");
    for (order, expected) in [([&duplicate, &unbound], "SOURCE_PHYSICS_CONNECTOR_ROW_DUPLICATE"), ([&unbound, &duplicate], "SOURCE_PHYSICS_CONNECTOR_ROW_UNBOUND")] {
        let mut envelope = base.clone();
        rows(&mut envelope).extend(order.into_iter().cloned());
        assert_eq!(stable_refusal(&envelope), expected);
    }
}

/// `validate`'s coverage loop (cases in `exact_cases` order, then records): a
/// rod row missing in one case (TIE_ROD_ROWS) and an end row missing in the
/// other (ROW_COVERAGE). The earlier case's defect is reported.
#[test]
fn coverage_loop_reports_the_first_case_in_exact_cases_order() {
    for (rod_case, end_case, expected) in [
        (CONNECTOR, SECOND, "SOURCE_PHYSICS_CONNECTOR_TIE_ROD_ROWS"),
        (SECOND, CONNECTOR, "SOURCE_PHYSICS_CONNECTOR_ROW_COVERAGE"),
    ] {
        let mut envelope = rods_envelope();
        let rod = position(&envelope, &format!("{rod_case}{ROD_YP}"));
        rows(&mut envelope).remove(rod);
        let end = position(&envelope, &format!("{end_case}{END_FX}"));
        rows(&mut envelope).remove(end);
        assert_eq!(stable_refusal(&envelope), expected);
    }
}

/// `validate_pressure`'s loop (records, then cases in `exact_cases` order): a
/// `pe_x` row on the unpressurized record in one case (UNPRESSURIZED_ROWS; an
/// end row is removed so the row count still covers) and an `s_x` value off
/// its binding in the other (EFFECTIVE_ROW_BINDING).
#[test]
fn pressure_loop_reports_the_first_case_in_exact_cases_order() {
    for (pe_case, s_case, case, expected) in [
        (CONNECTOR, SECOND, "load:U5A", "SOURCE_PHYSICS_CONNECTOR_UNPRESSURIZED_ROWS"),
        (SECOND, CONNECTOR, "case:second", "SOURCE_PHYSICS_CONNECTOR_EFFECTIVE_ROW_BINDING"),
    ] {
        let mut envelope = rods_envelope();
        let end = position(&envelope, &format!("{pe_case}{END_FX}"));
        rows(&mut envelope).remove(end);
        let mut pe = envelope["results"][position(&envelope, &format!("{pe_case}{S_X}"))].clone();
        pe["id"] = json!(format!("{pe_case}connector-pressure-force:connector_local:pe_x"));
        pe["kind"] = json!("connector_pressure_force_v1");
        pe["metadata"]["component"] = json!("pe_x");
        pe["value"] = json!(0.0);
        assert_eq!(pe["basis_ref"]["ref_id"], case);
        rows(&mut envelope).push(pe);
        let s = position(&envelope, &format!("{s_case}{S_X}"));
        let value = envelope["results"][s]["value"].as_f64().unwrap();
        envelope["results"][s]["value"] = json!(value * 1.5 + 1000.0);
        assert_eq!(stable_refusal(&envelope), expected);
    }
}

/// The `s_x` lookup and the tie-rod lookup of `validate_pressure` see at most
/// one candidate: a second row matching either predicate, equal to the first
/// in every consumed field or conflicting with it, inserted before or after
/// it, is refused by the row loop's slot as ROW_DUPLICATE.
#[test]
fn a_second_lookup_candidate_is_refused_before_selection() {
    let base = rods_envelope();
    let aliases = [
        // Another ID for `load:U5A`'s s_x row.
        (format!("{CONNECTOR}{S_X}"), "result:probe:s_x".to_string()),
        // The case-qualified ID form of `load:U5A`'s rod-yp row: it binds the
        // same rod (both forms pass `is_tie_rod_row_id`).
        (format!("{CONNECTOR}{ROD_YP}"), format!("result:loadcase:load-U5A:connector:component-C-J:{ROD_YP}")),
    ];
    for (original, alias) in &aliases {
        let at = position(&base, original);
        for conflicting in [false, true] {
            for before in [false, true] {
                let mut envelope = base.clone();
                let mut second = envelope["results"][at].clone();
                second["id"] = json!(alias);
                if conflicting {
                    second["value"] = json!(second["value"].as_f64().unwrap() + 1.0);
                }
                rows(&mut envelope).insert(if before { at } else { at + 1 }, second);
                assert_eq!(stable_refusal(&envelope), "SOURCE_PHYSICS_CONNECTOR_ROW_DUPLICATE", "{alias} conflicting={conflicting} before={before}");
            }
        }
    }
}
