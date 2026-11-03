//! `Index::any`'s shear operand rule (DEL-04-07 Work `u4b-u5b-implementation.md` §5.1):
//! a component-free shear slot is one operand when its rows agree, and the envelope is
//! refused by name (`SHEAR_OPERAND_AMBIGUOUS`) when they do not, in every row order and
//! on every Index (each `HashMap::new` draws its own hash keys). These tests live in this
//! file, declared at the end of `u4b_evidence.rs` (a child module, so they read its
//! private `any`), keeping the test bodies out of the file that retained pricing reads
//! and outside U4B-RD's pinned Index span.
use super::{Index, Route};
use serde_json::{json, Value};

const SHEAR_Y: &str = "element_local_shear_force_y";
const SHEAR_Z: &str = "element_local_shear_force_z";

fn row(case: &str, entity: &str, kind: &str, component: &str, location: &str, value: Value) -> Value {
    json!({"basis_ref": {"ref_type": "load_case", "ref_id": case}, "entity_ref": entity, "kind": kind,
        "metadata": {"component": component, "location": location, "basis": "recovered_from_local_element_stiffness"}, "value": value})
}
fn shear_y(component: &str, value: Value) -> Value {
    row("case:a", "pipe:p", SHEAR_Y, component, "midspan", value)
}
fn without_value(mut r: Value) -> Value {
    r.as_object_mut().unwrap().remove("value");
    r
}

/// Rows beside the slot under test: its z twin, the same slot on another member and
/// case, and a sectioned stress row.
fn context() -> Vec<Value> {
    vec![
        row("case:a", "pipe:p", SHEAR_Z, "shear_force_z", "midspan", json!(3.0)),
        row("case:a", "pipe:q", SHEAR_Y, "shear_force_y", "midspan", json!(7.0)),
        row("case:b", "pipe:p", SHEAR_Y, "", "midspan", json!(9.0)),
        row("case:a", "pipe:p", "element_local_bending_normal_stress_y", "bending_normal_stress_y", "midspan", json!(1.0)),
    ]
}

/// Every order of `rows` (Heap's algorithm).
fn permutations(rows: Vec<Value>) -> Vec<Vec<Value>> {
    fn heap(k: usize, a: &mut Vec<Value>, out: &mut Vec<Vec<Value>>) {
        if k <= 1 {
            out.push(a.clone());
            return;
        }
        for i in 0..k {
            heap(k - 1, a, out);
            a.swap(if k % 2 == 0 { i } else { 0 }, k - 1);
        }
    }
    let mut a = rows;
    let mut out = Vec::new();
    heap(a.len(), &mut a, &mut out);
    out
}

/// The slot under test's variants, each mixed with `context()`, in every order of the
/// variants and of two context rows around them, on four Indexes per order and both
/// routes: `check` sees each outcome.
fn every_order(variants: &[Value], mut check: impl FnMut(Route, Result<Index<'_>, String>)) -> usize {
    let ctx = context();
    let mut seen = 0;
    let mut rows: Vec<Value> = variants.to_vec();
    rows.extend(ctx[..2].iter().cloned());
    for mut order in permutations(rows) {
        order.extend(ctx[2..].iter().cloned());
        for route in [Route::Exact, Route::Preview] {
            for _ in 0..4 {
                check(route, Index::new(&order, route));
                seen += 1;
            }
        }
    }
    seen
}

fn ambiguous(route: Route) -> String {
    route.fail("SHEAR_OPERAND_AMBIGUOUS")
}

/// Variants that differ in a consumed field are refused by name in every order and on
/// every Index: two values, a value beside a null, a string, an absent value, a
/// same-component duplicate with another value, -0 beside +0, and a third row that
/// differs from two agreeing ones. A non-finite value cannot reach the reader: JSON
/// has none, `serde_json` refuses an out-of-range literal and stores a non-finite
/// `f64` as null, the null case here.
#[test]
fn conflicting_variants_are_refused_in_every_order() {
    assert!(json!(f64::INFINITY).is_null() && json!(f64::NAN).is_null());
    assert!(serde_json::from_str::<Value>("1e400").is_err());
    let sets: Vec<Vec<Value>> = vec![
        vec![shear_y("shear_force_y", json!(2.0)), shear_y("", json!(1002.0))],
        vec![shear_y("shear_force_y", json!(2.0)), shear_y("", json!(null))],
        vec![shear_y("shear_force_y", json!(2.0)), shear_y("other", json!("2.0"))],
        vec![shear_y("shear_force_y", json!(2.0)), without_value(shear_y("", json!(0)))],
        vec![shear_y("shear_force_y", json!(2.0)), shear_y("shear_force_y", json!(4.0))],
        vec![shear_y("shear_force_y", json!(0.0)), shear_y("", json!(-0.0))],
        vec![shear_y("shear_force_y", json!(2.0)), shear_y("", json!(2.0)), shear_y("x", json!(5.0))],
    ];
    for set in &sets {
        let n = every_order(set, |route, got| assert_eq!(got.err(), Some(ambiguous(route)), "{set:?}"));
        assert!(n >= 2 * 4 * 24, "{n}");
    }
}

/// A duplicate slot of a new kind is refused first (`ROW_SLOT_DUPLICATE`), in every
/// order, whether it precedes or follows the ambiguous operand rows.
#[test]
fn a_new_kind_duplicate_precedes_the_operand_refusal() {
    let tau = |v: f64| row("case:a", "pipe:p", super::SHEAR_MAX, "tau_v_max", "midspan", json!(v));
    let set = [shear_y("shear_force_y", json!(2.0)), shear_y("", json!(5.0)), tau(1.0), tau(1.0)];
    every_order(&set, |route, got| assert_eq!(got.err(), Some(route.fail("ROW_SLOT_DUPLICATE"))));
}

/// Variants agreeing in every consumed field are one operand in every order: one value
/// under two components, a same-component repeat, an integer beside its binary64, and
/// rows that all lack a number (absent to the caller, as one such row is).
#[test]
fn equivalent_duplicates_are_one_operand() {
    let sets: Vec<(Vec<Value>, Option<f64>)> = vec![
        (vec![shear_y("shear_force_y", json!(2.0)), shear_y("", json!(2.0))], Some(2.0)),
        (vec![shear_y("shear_force_y", json!(-2.5)), shear_y("shear_force_y", json!(-2.5)), shear_y("x", json!(-2.5))], Some(-2.5)),
        (vec![shear_y("shear_force_y", json!(2)), shear_y("", json!(2.0))], Some(2.0)),
        (vec![shear_y("shear_force_y", json!(null)), shear_y("", json!("2.0")), without_value(shear_y("x", json!(0)))], None),
    ];
    for (set, want) in &sets {
        every_order(set, |_, got| {
            let index = got.unwrap();
            assert_eq!(index.any("case:a", "pipe:p", SHEAR_Y, "midspan", "").map(f64::to_bits), want.map(f64::to_bits), "{set:?}");
            // The context's slots read as their one row.
            assert_eq!(index.any("case:a", "pipe:p", SHEAR_Z, "midspan", ""), Some(3.0));
            assert_eq!(index.any("case:a", "pipe:q", SHEAR_Y, "midspan", ""), Some(7.0));
            assert_eq!(index.any("case:b", "pipe:p", SHEAR_Y, "midspan", ""), Some(9.0));
            // Absent: another case, entity, location and basis.
            for (c, e, l, b) in [("case:c", "pipe:p", "midspan", ""), ("case:a", "pipe:r", "midspan", ""), ("case:a", "pipe:p", "end_i", ""), ("case:a", "pipe:p", "midspan", "nominal_section")] {
                assert_eq!(index.any(c, e, SHEAR_Y, l, b), None);
            }
        });
    }
}

/// The map holds the shear operand slots only, one entry each.
#[test]
fn the_operand_map_holds_shear_slots_only() {
    let mut rows = context();
    rows.push(shear_y("shear_force_y", json!(2.0)));
    rows.push(shear_y("", json!(2.0)));
    let index = Index::new(&rows, Route::Preview).unwrap();
    assert_eq!(index.operands.len(), 4);
    assert!(index.operands.keys().all(|k| super::SHEAR_OPERANDS.contains(&k.2)));
    // The row index alone (`directional_model_checks`) holds no operand map and reads
    // no operand, so it does not refuse an ambiguous slot.
    rows.push(shear_y("", json!(5.0)));
    assert!(Index::new(&rows, Route::Exact).is_err());
    let alone = Index::rows_only(&rows, Route::Exact).unwrap();
    assert!(alone.operands.is_empty() && alone.operands.capacity() == 0);
}

fn read(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

/// `source` with a component-free copy of its first case-scoped midspan shear-y row,
/// its value moved by `delta`, inserted first or last.
fn with_variant(source: &Value, delta: f64, first: bool) -> Value {
    let mut source = source.clone();
    let rows = source["results"].as_array_mut().unwrap();
    let original = rows.iter().find(|r| r["kind"] == SHEAR_Y && r["metadata"]["location"] == "midspan" && r["basis_ref"]["ref_type"] == "load_case").unwrap();
    let mut extra = original.clone();
    extra["id"] = json!(format!("{}:variant", original["id"].as_str().unwrap()));
    extra["metadata"].as_object_mut().unwrap().remove("component");
    extra["value"] = json!(original["value"].as_f64().unwrap() + delta);
    if first {
        rows.insert(0, extra);
    } else {
        rows.push(extra);
    }
    source
}

/// The readers on an ambiguous source (the defect's probe: a component-free copy of a
/// midspan shear row with another value) refuse it by name on every read and row order,
/// and one `dispatch` states the same refusal in its token and its findings, call after
/// call. An equivalent copy reads as the source does.
#[test]
fn the_readers_and_dispatch_refuse_an_ambiguous_source_by_name() {
    use crate::reader_verdict::dispatch;
    let corpus = read(include_str!("../../../../fixtures/results/pressure_v3_straight_reader_corpus.json"));
    // The corpus's inputs: X0 the connected request, Y0 the load/reference-state one.
    let x0 = read(include_str!("../../../../core/product_physics/tests/fixtures/exact_pressure_connected_request.json"))["model"].clone();
    let y0 = read(include_str!("../../../../fixtures/product_preview/pressure/load_reference/pressure.request.json"))["model"].clone();
    let arc = read(include_str!("../../../../fixtures/results/pressure_u4b_arc_sparse.json"));
    let arc_model = read(include_str!("../../../../core/product_physics/tests/fixtures/u4b_l90_p_model.json"));
    let preview = read(include_str!("../../../../fixtures/results/preview_physics_u4b_arc_sparse.json"));
    let preview_model = read(include_str!("../../../../core/product_physics/tests/fixtures/u4b_z_preview_rev_model.json"));
    let exact = "SOURCE_PHYSICS_SHEAR_OPERAND_AMBIGUOUS";
    let mut sources: Vec<(Value, &Value, &str)> = corpus["cases"].as_array().unwrap().iter()
        .map(|c| (c["envelope"].clone(), if c["label"].as_str().unwrap().starts_with("X0") { &x0 } else { &y0 }, exact))
        .collect();
    sources.push((arc, &arc_model, exact));
    sources.push((preview, &preview_model, "SOURCE_PREVIEW_PHYSICS_SHEAR_OPERAND_AMBIGUOUS"));
    let validate = |s: &Value| match s["contract_evidence"].get("preview_cases") {
        Some(_) => crate::preview_physics_evidence::validate_preview_physics_evidence(s),
        None => crate::semantic_contract::validate_pressure_evidence(s),
    };
    for (source, model, code) in &sources {
        assert_eq!(validate(source), Ok(()), "the unmodified source reads");
        let clean = dispatch(source, Some(model), None);
        assert_eq!(clean["standing"]["eligible"], true, "{clean}");
        for first in [true, false] {
            let bad = with_variant(source, 1000.0, first);
            // The load/reference-state reader prefixes the evidence reader's code.
            let refusal = validate(&bad).unwrap_err();
            assert!(refusal.ends_with(code), "{refusal}");
            for _ in 0..8 {
                assert_eq!(validate(&bad), Err(refusal.clone()));
            }
            let d = dispatch(&bad, Some(model), None);
            assert_eq!(d["standing"]["eligible"], false);
            let findings = d["standing"]["findings"].as_array().unwrap();
            assert!(findings.len() == 1 && findings[0].as_str().unwrap().ends_with(code), "{d}");
            assert_ne!(d["standing"]["token"], clean["standing"]["token"]);
            for _ in 0..8 {
                assert_eq!(dispatch(&bad, Some(model), None), d);
            }
            let same = with_variant(source, 0.0, first);
            assert_eq!(validate(&same), Ok(()));
            assert_eq!(dispatch(&same, Some(model), None)["standing"], clean["standing"]);
        }
    }
}
