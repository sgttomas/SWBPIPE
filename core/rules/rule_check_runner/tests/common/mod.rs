//! Shared test support: a single-basis rule-pack format 0.5.0 frame for the
//! runner's evaluation tests (DEL-06-01 Design `rule-pack-0.5.0.md` §2.3,
//! §2.4). Every name is invented; no category of any standard.
#![allow(dead_code)]

use open_pipe_stress_rule_check_runner::BasisRef;
use serde_json::{json, Value};

/// The one basis of a single-basis pack: the invented demo model's first load
/// case.
pub fn basis() -> BasisRef {
    BasisRef::new("load_case", "load:L-100")
}

/// `document` as a format 0.5.0 pack with one category mapped to one required
/// load case ([`basis`]), every check in that category: each check has exactly
/// one outcome, so evaluation tests keep one outcome per check.
pub fn single_basis(mut document: Value) -> Value {
    document["schema_version"] = json!("0.5.0");
    if document.get("metadata").is_some_and(Value::is_object) {
        document["metadata"]["schema_version"] = json!("0.5.0");
    }
    document["required_load_cases"] = json!([{
        "required_load_case_id": "demo_required_case_1",
        "name": "Invented required load case 1",
        "basis_ref": { "ref_type": "load_case", "ref_id": "load:L-100" }
    }]);
    document["stress_categories"] = json!([{
        "category_id": "demo_category_a",
        "name": "Invented category A"
    }]);
    document["load_case_mappings"] = json!([{
        "mapping_id": "demo_mapping_a",
        "category_ref": { "ref_id": "demo_category_a", "ref_type": "stress_category" },
        "required_load_case_refs": [{ "ref_id": "demo_required_case_1", "ref_type": "required_load_case" }]
    }]);
    if let Some(checks) = document
        .get_mut("check_definitions")
        .and_then(Value::as_array_mut)
    {
        for check in checks {
            check["category_ref"] =
                json!({ "ref_id": "demo_category_a", "ref_type": "stress_category" });
        }
    }
    document
}
