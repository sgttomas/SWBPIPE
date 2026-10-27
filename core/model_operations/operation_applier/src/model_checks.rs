//! On-demand model checks for the desktop (DEL-04-06 Design
//! `on-demand-model-checks.md`): the solve's own checks before stiffness
//! formation (`LinearStaticPreviewRequest::pre_formation_checks_value`)
//! and the W-13 model-data slot checks (`document_validator`), run on the
//! session model as the desktop's solve request carries it
//! (`{"model": …, "materials": []}`). Nothing here writes to the model, adds a
//! rule or solves.
//!
//! The report is bounded whatever the model (Codex reviews r1 and r2 of
//! #1665): at most [`MODEL_CHECKS_MAX_RESPONSE_BYTES`] of serialized findings,
//! the solve's first and then the slot checks', each list in its checks'
//! order. Only display text (a finding's `message`, a refusal's text) is ever
//! shortened, at [`MODEL_CHECKS_MAX_TEXT_BYTES`] on a character boundary, and
//! then marked `truncated`; machine identifiers (ids, codes, severities,
//! sources, paths and affected references) are kept whole, and a finding whose
//! identifiers would pass the remaining budget is counted as not shown rather
//! than altered. The native command runs it in a capped worker; the browser
//! engine also refuses a model text above [`MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES`]
//! before parsing it.
use open_pipe_stress_product_physics::LinearStaticPreviewRequest;
use serde_json::{json, Map, Value};

pub const MODEL_CHECKS_DOCUMENT_KIND: &str = "openpipestress.desktop.model_checks";
/// The serialized findings a report carries at most.
pub const MODEL_CHECKS_MAX_RESPONSE_BYTES: usize = 1024 * 1024;
/// The longest text of a finding or a refusal, in UTF-8 bytes.
pub const MODEL_CHECKS_MAX_TEXT_BYTES: usize = 2048;
/// The references a finding names at most.
/// The longest model text the browser engine checks (4 MiB); a longer one is
/// refused `MODEL-CHECKS-INPUT-TOO-LARGE` before it is parsed.
pub const MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES: usize = 4 * 1024 * 1024;
pub const MODEL_CHECKS_INPUT_TOO_LARGE: &str = "MODEL-CHECKS-INPUT-TOO-LARGE";

/// [`model_checks_owned`] on a copy of `model`.
pub fn model_checks(model: &Value) -> Value {
    model_checks_owned(model.clone())
}

/// `{document_kind, checked, diagnostics, diagnostics_not_shown, slot_findings,
/// slot_findings_not_shown}`: with `checked: true`, the solve's diagnostics as
/// it publishes them; with `checked: false`, the solve's own refusal text in
/// `refusal` (and `refusal_truncated` when it was shortened) and no solve
/// findings. The slot checks run either way. Bounded as the module says. The
/// model is consumed.
pub fn model_checks_owned(model: Value) -> Value {
    let slots = crate::document_validator::validate_model_document(&model);
    let mut request = Map::new();
    request.insert("model".to_string(), model);
    request.insert("materials".to_string(), Value::Array(Vec::new()));
    let solve = LinearStaticPreviewRequest::pre_formation_checks_value(Value::Object(request));
    let mut budget = Budget { remaining: MODEL_CHECKS_MAX_RESPONSE_BYTES };
    let mut report = Map::new();
    report.insert("document_kind".to_string(), json!(MODEL_CHECKS_DOCUMENT_KIND));
    match solve {
        Ok(diagnostics) => {
            let (shown, not_shown) = budget.list(diagnostics.into_iter().map(|diagnostic| serde_json::to_value(diagnostic).ok()));
            report.insert("checked".to_string(), Value::Bool(true));
            report.insert("diagnostics".to_string(), Value::Array(shown));
            report.insert("diagnostics_not_shown".to_string(), json!(not_shown));
        }
        Err(refusal) => {
            let (refusal, truncated) = cut(refusal);
            budget.remaining = budget.remaining.saturating_sub(refusal.len());
            report.insert("checked".to_string(), Value::Bool(false));
            report.insert("refusal".to_string(), Value::String(refusal));
            if truncated {
                report.insert("refusal_truncated".to_string(), Value::Bool(true));
            }
            report.insert("diagnostics".to_string(), Value::Array(Vec::new()));
            report.insert("diagnostics_not_shown".to_string(), json!(0));
        }
    }
    let (shown, not_shown) = budget.list(slots.into_iter().enumerate().map(|(index, finding)| Some(slot_diagnostic(index, finding))));
    report.insert("slot_findings".to_string(), Value::Array(shown));
    report.insert("slot_findings_not_shown".to_string(), json!(not_shown));
    Value::Object(report)
}

/// Where the slot findings come from, as the desktop's Diagnostics list names it.
pub const DOCUMENT_VALIDATOR_SOURCE: &str = "core/model_operations/operation_applier/src/document_validator.rs";

/// A slot finding in the shape the desktop lists it (`modelDocumentDiagnostics`):
/// its id from its code, path and position, its message with the path, and the
/// validator as its source, so the bound counts what is shown.
fn slot_diagnostic(index: usize, finding: crate::document_validator::DocumentDiagnostic) -> Value {
    let path = if finding.path.is_empty() { "document".to_string() } else { finding.path.clone() };
    let message = if finding.path.is_empty() { finding.message } else { format!("{} (at {})", finding.message, finding.path) };
    json!({
        "id": format!("model-document:{}:{path}:{index}", finding.code),
        "code": finding.code,
        "severity": if finding.severity == "blocking" { "blocking" } else { "warning" },
        "message": message,
        "affected_refs": finding.affected_refs,
        "source": DOCUMENT_VALIDATOR_SOURCE,
        "path": finding.path,
    })
}

/// The serialized bytes the report may still carry.
struct Budget {
    remaining: usize,
}

impl Budget {
    /// The findings that fit, in order, each with only its message shortened;
    /// the first that does not fit, and every one after it, is counted instead.
    fn list(&mut self, findings: impl Iterator<Item = Option<Value>>) -> (Vec<Value>, usize) {
        let mut shown = Vec::new();
        let mut not_shown = 0usize;
        for finding in findings {
            if not_shown > 0 {
                not_shown += 1;
                continue;
            }
            let Some(Value::Object(fields)) = finding else {
                not_shown += 1;
                continue;
            };
            let finding = shortened(fields);
            let length = serde_json::to_string(&finding).map(|text| text.len() + 1).unwrap_or(usize::MAX);
            if length > self.remaining {
                not_shown += 1;
                continue;
            }
            self.remaining -= length;
            shown.push(finding);
        }
        (shown, not_shown)
    }
}

/// A finding with only its display text shortened (and then marked `truncated`).
fn shortened(mut fields: Map<String, Value>) -> Value {
    if let Some(Value::String(message)) = fields.remove("message") {
        let (message, truncated) = cut(message);
        fields.insert("message".to_string(), Value::String(message));
        if truncated {
            fields.insert("truncated".to_string(), Value::Bool(true));
        }
    }
    Value::Object(fields)
}

/// The browser engine's entry: the model text, refused unparsed above
/// [`MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES`]; `Err` carries a text that is not JSON.
pub fn model_checks_text(model_json: &str) -> Result<Value, serde_json::Error> {
    if model_json.len() > MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES {
        return Ok(json!({
            "document_kind": MODEL_CHECKS_DOCUMENT_KIND,
            "checked": false,
            "refusal_code": MODEL_CHECKS_INPUT_TOO_LARGE,
            "refusal": format!(
                "{MODEL_CHECKS_INPUT_TOO_LARGE}: the browser preview checks a model text of at most {MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES} bytes; this one is {} bytes. The desktop app checks larger models under its memory pool.",
                model_json.len()
            ),
        }));
    }
    serde_json::from_str::<Value>(model_json).map(model_checks_owned)
}

/// `text` cut at [`MODEL_CHECKS_MAX_TEXT_BYTES`] on a character boundary, and whether it was cut.
fn cut(mut text: String) -> (String, bool) {
    if text.len() <= MODEL_CHECKS_MAX_TEXT_BYTES {
        return (text, false);
    }
    let mut end = MODEL_CHECKS_MAX_TEXT_BYTES;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    text.truncate(end);
    (text, true)
}
