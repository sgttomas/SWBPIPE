//! On-demand model checks for the desktop (DEL-04-06 Design
//! `on-demand-model-checks.md`): the solve's own checks before stiffness
//! formation (`open_pipe_stress_product_physics::model_checks`), run on the
//! session model as the desktop's solve request carries it
//! (`{"model": …, "materials": []}`). Nothing here writes to the model, adds a
//! rule or solves.
use open_pipe_stress_product_physics::model_checks::preview_model_checks_value;
use serde_json::{json, Value};

pub const MODEL_CHECKS_DOCUMENT_KIND: &str = "openpipestress.desktop.model_checks";

/// `{document_kind, checked: true, diagnostics}` with the solve's diagnostics as
/// it publishes them, or `{document_kind, checked: false, refusal}` with the
/// solve's own refusal text when the request cannot be read.
pub fn model_checks(model: &Value) -> Value {
    match preview_model_checks_value(json!({ "model": model, "materials": [] })) {
        Ok(diagnostics) => json!({
            "document_kind": MODEL_CHECKS_DOCUMENT_KIND,
            "checked": true,
            "diagnostics": diagnostics,
        }),
        Err(refusal) => json!({
            "document_kind": MODEL_CHECKS_DOCUMENT_KIND,
            "checked": false,
            "refusal": refusal,
        }),
    }
}
