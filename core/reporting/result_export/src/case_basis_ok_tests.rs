//! RT-U4: `case_basis_ok` is `case_basis(..).ok()` on every basis shape. Declared at
//! the end of `physics_evidence.rs` (a child module, so it reads both private
//! functions), keeping the test body out of the file retained pricing reads.
use super::{case_basis, case_basis_ok};
use serde_json::{json, Value};

#[test]
fn case_basis_ok_is_case_basis_ok() {
    let bases = [
        json!({"ref_type": "load_case", "ref_id": "case:a"}),
        json!({"ref_type": "load_case", "ref_id": ""}),
        json!({"ref_type": "load_case", "ref_id": 7}),
        json!({"ref_type": "load_case", "ref_id": null}),
        json!({"ref_type": "load_case", "ref_id": ["case:a"]}),
        json!({"ref_type": "load_case"}),
        json!({"ref_id": "case:a"}),
        json!({"ref_type": "load_case", "ref_id": "case:a", "extra": 1}),
        json!({"ref_type": "load_combination", "ref_id": "comb:a"}),
        json!({"ref_type": "Load_Case", "ref_id": "case:a"}),
        json!({"ref_type": null, "ref_id": "case:a"}),
        json!({}),
        json!(null),
        json!("load_case"),
        json!(["load_case", "case:a"]),
    ];
    let mut rows: Vec<Value> = bases.iter().map(|b| json!({"id": "r", "basis_ref": b})).collect();
    rows.push(json!({"id": "r"}));
    rows.push(json!(null));
    rows.push(json!([1, 2]));
    let mut admitted = 0;
    for row in &rows {
        assert_eq!(case_basis_ok(row), case_basis(row).ok(), "{row}");
        admitted += usize::from(case_basis_ok(row).is_some());
    }
    assert_eq!(admitted, 1);
}
