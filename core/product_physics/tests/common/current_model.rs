//! B7b's one re-authoring of a copied integration-test model or request (the
//! lib twin is `src/b7b_admission_tests.rs`'s `reauthor_current_model`).
//!
//! Model document 0.9.0 is the only current version. A 0.5.0, 0.6.0, 0.7.0 or
//! 0.8.0 copy is re-authored by its `schema_version` alone (0.6.0, 0.7.0, 0.8.0
//! and 0.9.0 add members and change none, DEL-04-07 Design
//! `u4b-u5b-implementation.md` §1.1, DEL-02-01 Design `model-document-0.7.0.md`
//! §2, `model-document-0.8.0.md` §1 and `model-document-0.9.0.md` §3.1). An older copy is
//! re-authored by: `schema_version` -> 0.9.0; a declared v2 or v3 exact contract
//! -> the current
//! `{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"}`;
//! and, on a contract-present document, an explicit zero `corrosion_allowance`
//! (`{"value":0.0,"unit":"m"}`) on every pipe section lacking one. Nothing else
//! changes: no physical or numerical input, and no other contract (a retired
//! contract's refusal control keeps its form). SHA-bound frozen requests under
//! `validation/**` are never edited; their consuming tests check the recorded
//! SHA and re-author only the copy.
#![allow(dead_code)]
use serde_json::{json, Value};

pub const CURRENT_MODEL_VERSION: &str = "0.9.0";
pub const RETIRED_MODEL_VERSIONS: [&str; 8] = ["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0", "0.8.0"];
/// The previous current versions, re-authored by their version string alone.
pub const PREVIOUS_MODEL_VERSIONS: [&str; 4] = ["0.5.0", "0.6.0", "0.7.0", "0.8.0"];

/// The current exact contract object.
pub fn current_contract() -> Value {
    json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"})
}

/// Re-author a copied model document in place (see the module comment).
pub fn reauthor_model(model: &mut Value) {
    if model["schema_version"]
        .as_str()
        .is_some_and(|version| PREVIOUS_MODEL_VERSIONS.contains(&version))
    {
        model["schema_version"] = json!(CURRENT_MODEL_VERSION);
        return;
    }
    if model["schema_version"]
        .as_str()
        .is_some_and(|version| RETIRED_MODEL_VERSIONS.contains(&version))
    {
        model["schema_version"] = json!(CURRENT_MODEL_VERSION);
    }
    let declared = model.get("pressure_contract").and_then(|contract| {
        Some((contract.get("version")?.as_str()?, contract.get("mode")?.as_str()?))
    });
    if matches!(
        declared,
        Some(("2.0.0", "exact_straight_pressure_v2") | ("3.0.0", "exact_pressure_v3"))
    ) {
        model["pressure_contract"] = current_contract();
    }
    if model.get("pressure_contract") == Some(&current_contract()) {
        for pipe in model["pipe_segments"].as_array_mut().into_iter().flatten() {
            pipe["section"]
                .as_object_mut()
                .expect("pipe section object")
                .entry("corrosion_allowance")
                .or_insert(json!({"value": 0.0, "unit": "m"}));
        }
    }
}

/// [`reauthor_model`] applied to a copied request's `model`.
pub fn reauthor_request(mut request: Value) -> Value {
    reauthor_model(&mut request["model"]);
    request
}

/// Whether a committed current exact request is already in the re-authored
/// form (the re-authoring is the identity on it).
pub fn assert_current_exact_request(request: &Value, label: &str) {
    assert_eq!(request["model"]["schema_version"], CURRENT_MODEL_VERSION, "{label}");
    assert_eq!(request["model"]["pressure_contract"], current_contract(), "{label}");
    assert_eq!(&reauthor_request(request.clone()), request, "{label}: already current");
}

/// The project root (`projects/chirality-piping`).
pub fn project_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
