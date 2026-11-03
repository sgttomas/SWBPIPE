//! `Index::any` against the linear scan it replaced, on the same `Index`. These tests
//! live in this file, declared at the end of `u4b_evidence.rs` (a child module, so
//! they read its private `rows`, `first` and `any`), keeping the test bodies out of
//! the file that retained pricing reads and outside U4B-RD's pinned Index span.
use super::{Index, Route};
use serde_json::{json, Value};

const SHEAR_Y: &str = "element_local_shear_force_y";

/// The scan `any` replaced, verbatim: the first row of `rows` in its iteration order
/// whose slot matches without the component.
fn scan(index: &Index<'_>, case: &str, entity: &str, kind: &str, location: &str, basis: &str) -> Option<f64> {
    index.rows.iter().find(|(s, _)| s.0 == case && s.1 == entity && s.2 == kind && s.4 == location && s.5 == basis).and_then(|(_, r)| r["value"].as_f64())
}

fn row(case: &str, entity: &str, kind: &str, component: &str, location: &str, basis: Option<&str>, value: Value) -> Value {
    let mut metadata = json!({"component": component, "location": location});
    if let Some(basis) = basis {
        metadata["basis"] = json!(basis);
    }
    json!({"basis_ref": {"ref_type": "load_case", "ref_id": case}, "entity_ref": entity, "kind": kind,
        "metadata": metadata, "value": value})
}

/// Rows whose slots differ only by component, with different values, and slots whose
/// first match may be missing, null or non-numeric beside a numeric variant.
fn rows() -> Vec<Value> {
    let outer = Some("token; section_basis=outer");
    vec![
        // Three numeric component variants, each with its own value.
        row("case:a", "pipe:p", SHEAR_Y, "y1", "midspan", None, json!(1.0)),
        row("case:a", "pipe:p", SHEAR_Y, "y2", "midspan", None, json!(2.0)),
        row("case:a", "pipe:p", SHEAR_Y, "y3", "midspan", None, json!(3.0)),
        // One numeric variant beside a null, a string and a missing value.
        row("case:a", "pipe:p", SHEAR_Y, "a", "end_i", None, json!(null)),
        row("case:a", "pipe:p", SHEAR_Y, "b", "end_i", None, json!("1.5")),
        {
            let mut r = row("case:a", "pipe:p", SHEAR_Y, "c", "end_i", None, json!(0));
            r.as_object_mut().unwrap().remove("value");
            r
        },
        row("case:a", "pipe:p", SHEAR_Y, "d", "end_i", None, json!(5.0)),
        // No numeric variant at all.
        row("case:a", "pipe:p", SHEAR_Y, "a", "end_j", None, json!(null)),
        row("case:a", "pipe:p", SHEAR_Y, "b", "end_j", None, json!({"v": 1})),
        // A section basis, and a second case and member with one row each.
        row("case:a", "pipe:p", SHEAR_Y, "y1", "midspan", outer, json!(7.0)),
        row("case:a", "pipe:p", SHEAR_Y, "y2", "midspan", outer, json!(8.0)),
        row("case:b", "pipe:p", SHEAR_Y, "y1", "midspan", None, json!(9.0)),
        row("case:a", "pipe:q", SHEAR_Y, "y1", "midspan", None, json!(10.0)),
    ]
}

/// (case, entity, kind, location, basis): every populated slot and absent ones.
const LOOKUPS: [(&str, &str, &str, &str, &str); 11] = [
    ("case:a", "pipe:p", SHEAR_Y, "midspan", ""),
    ("case:a", "pipe:p", SHEAR_Y, "end_i", ""),
    ("case:a", "pipe:p", SHEAR_Y, "end_j", ""),
    ("case:a", "pipe:p", SHEAR_Y, "midspan", "outer"),
    ("case:b", "pipe:p", SHEAR_Y, "midspan", ""),
    ("case:a", "pipe:q", SHEAR_Y, "midspan", ""),
    // Absent: another case, entity, kind, location and basis.
    ("case:c", "pipe:p", SHEAR_Y, "midspan", ""),
    ("case:a", "pipe:r", SHEAR_Y, "midspan", ""),
    ("case:a", "pipe:p", "element_local_shear_force_z", "midspan", ""),
    ("case:a", "pipe:p", SHEAR_Y, "quarter_1", ""),
    ("case:a", "pipe:p", SHEAR_Y, "end_i", "outer"),
];

/// On many Indexes (each `HashMap::new` draws its own iteration order), `any` returns
/// the scan's row on every lookup, before and after its map is built and on repeated
/// calls; both orders of a non-numeric and a numeric first match occur.
#[test]
fn any_is_the_scan_it_replaced_on_the_same_index() {
    let rows = rows();
    let (mut non_numeric_first, mut numeric_first, mut variants_seen) = (0, 0, [false; 3]);
    for _ in 0..64 {
        let index = Index::new(&rows, Route::Preview).unwrap();
        assert!(index.first.get().is_none(), "the map is built on first use only");
        let expected: Vec<Option<f64>> = LOOKUPS.iter().map(|&(c, e, k, l, b)| scan(&index, c, e, k, l, b)).collect();
        for round in 0..3 {
            let got: Vec<Option<f64>> = LOOKUPS.iter().map(|&(c, e, k, l, b)| index.any(c, e, k, l, b)).collect();
            assert_eq!(got, expected, "round {round}");
            assert!(index.first.get().is_some());
        }
        // The scan's own answer is unchanged after the map exists.
        assert_eq!(LOOKUPS.iter().map(|&(c, e, k, l, b)| scan(&index, c, e, k, l, b)).collect::<Vec<_>>(), expected);
        // Absent slots, and a slot with no numeric variant, are None on both.
        assert!(expected[6..].iter().all(Option::is_none));
        assert_eq!(expected[2], None);
        match expected[1] {
            None => non_numeric_first += 1,
            Some(v) => {
                assert_eq!(v, 5.0);
                numeric_first += 1;
            }
        }
        let midspan = expected[0].unwrap();
        variants_seen[midspan as usize - 1] = true;
    }
    assert!(non_numeric_first > 0 && numeric_first > 0, "{non_numeric_first} {numeric_first}");
    assert!(variants_seen.iter().all(|seen| *seen), "every component variant was some Index's first");
}

/// The map holds one entry per component-free slot, each the scan's first row (the
/// same `&Value`, not only an equal value).
#[test]
fn the_map_keeps_the_scans_first_row_per_component_free_slot() {
    let rows = rows();
    let index = Index::new(&rows, Route::Preview).unwrap();
    let _ = index.any("case:a", "pipe:p", SHEAR_Y, "midspan", "");
    let first = index.first.get().unwrap();
    assert_eq!(first.len(), 6);
    for (&(case, entity, kind, location, basis), &kept) in first {
        let found = index.rows.iter().find(|(s, _)| s.0 == case && s.1 == entity && s.2 == kind && s.4 == location && s.5 == basis).unwrap().1;
        assert!(std::ptr::eq(kept, *found));
    }
}
