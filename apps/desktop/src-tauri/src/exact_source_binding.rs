//! The native source binding of an exact-route report (W-11 slice (b),
//! DEL-08-01 Design `exact-route-report-sections.md` §3.6), shared by the
//! native render and the native save.
//!
//! Steps, in order, for an input that names an exact-route admission:
//! 1. the named solve job's result is still in this app's custody
//!    ([`crate::solve_jobs::SolveJobRegistry::custody_binding`]), else
//!    `REPORT-EXACT-CUSTODY-RELEASED`;
//! 2. the resubmitted result and intake texts hash, in place and without
//!    allocating, to the digests the host recorded for that job, and the
//!    result text to the admission's `received_bytes_sha256`, else
//!    `REPORT-EXACT-SOURCE-NOT-DELIVERED: <result|model>`;
//! 3. (after the command has grown its lease by the priced parse) both texts
//!    are parsed and guarded;
//! 4. the native reader's standing for the delivered result, with the solved
//!    model and the invocation built from the recorded mode, is numerically
//!    eligible, else `REPORT-EXACT-STANDING-NOT-ELIGIBLE`;
//! 5. the admission's `received_result_sha256` is `digest(source)` and its
//!    `model_sha256` is `digest(model)`, else `REPORT-EXACT-SOURCE-NOT-DELIVERED`.
//!
//! The host keeps no result or model bytes for this: only the job's 65 bytes of
//! digest metadata. Before the owner opens exact-route reports, the native
//! render refuses every exact identity at its step 0 and the package's Rust
//! boundary at item 0, so a production render never reaches these steps and a
//! production save reaches them only to refuse at item 0 after them. Equality
//! checks, not signatures. Standard claim fence applies (claims taxonomy per
//! DEC-081).

use crate::solve_jobs::{sha256_in_place, CustodyDigests};
use open_pipe_stress_result_export::derivative::{digest, guard_json};
use open_pipe_stress_report_standing::reader_readability;
use open_pipe_stress_result_export::reader_verdict;
use serde_json::{json, Value};

pub(crate) const CUSTODY_RELEASED: &str = "REPORT-EXACT-CUSTODY-RELEASED";
pub(crate) const SOURCE_NOT_DELIVERED: &str = "REPORT-EXACT-SOURCE-NOT-DELIVERED";
/// The package save's and user release's refusal of a result that is not
/// numerically eligible. DEL-08-01 REQ-024's present account: until those
/// emissions state numerical standing, they keep this refusal (LIM-016); the
/// rendered report and its print view do not.
pub(crate) const STANDING_NOT_ELIGIBLE: &str = "REPORT-EXACT-STANDING-NOT-ELIGIBLE";
/// A result the product reader cannot read with its model (DEL-05-02 Standing).
pub(crate) const RESULT_NOT_READABLE: &str = "REPORT-RESULT-NOT-READABLE";
/// A result whose mechanics are not solved.
pub(crate) const MECHANICS_NOT_SOLVED: &str = "REPORT-MECHANICS-NOT-SOLVED";
pub(crate) const SECTIONS_MISMATCH: &str = "REPORT-EXACT-SECTIONS-MISMATCH";

/// A refusal of the binding: its code and its full message (`<code>: ...`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactRefusal {
    pub code: &'static str,
    pub message: String,
}

impl std::fmt::Display for ExactRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

pub(crate) fn custody_released() -> ExactRefusal {
    ExactRefusal {
        code: CUSTODY_RELEASED,
        message: format!("{CUSTODY_RELEASED}: the result this report was admitted for is no longer in the app's custody."),
    }
}

/// `what` is `result` or `model`.
pub(crate) fn not_delivered(what: &str) -> ExactRefusal {
    ExactRefusal {
        code: SOURCE_NOT_DELIVERED,
        message: format!(
            "{SOURCE_NOT_DELIVERED}: {what}: the submitted text is not the bytes this app's solve job delivered or received."
        ),
    }
}

pub(crate) fn sections_mismatch(section: &str) -> ExactRefusal {
    ExactRefusal { code: SECTIONS_MISMATCH, message: format!("{SECTIONS_MISMATCH}: {section}") }
}

/// Whether `hex` is the lowercase hex form of `digest`, compared in place.
pub(crate) fn hex_equals(digest: &[u8; 32], hex: &str) -> bool {
    let hex = hex.as_bytes();
    if hex.len() != 64 {
        return false;
    }
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    digest
        .iter()
        .enumerate()
        .all(|(index, byte)| hex[2 * index] == DIGITS[usize::from(byte >> 4)] && hex[2 * index + 1] == DIGITS[usize::from(byte & 0x0f)])
}

/// The admission binding's text at `field`, or "".
fn binding_text<'a>(admission: &'a Value, field: &str) -> &'a str {
    admission.get(field).and_then(Value::as_str).unwrap_or("")
}

/// Step 1: the admission's `native_job_id` names a result still in
/// custody; returns its digests. `custody` is the registry's
/// `custody_binding`. An admission without a job id names no custody.
pub(crate) fn check_custody(
    admission: &Value,
    custody: &dyn Fn(&str) -> Option<CustodyDigests>,
) -> Result<CustodyDigests, ExactRefusal> {
    let job = binding_text(admission, "native_job_id");
    if job.is_empty() {
        return Err(custody_released());
    }
    custody(job).ok_or_else(custody_released)
}

/// Step 2: hashed in place, without allocating. The result text must
/// be the bytes the job delivered (and the admission's
/// `received_bytes_sha256`), the intake text the body it admitted.
pub(crate) fn check_delivered_texts(
    admission: &Value,
    digests: &CustodyDigests,
    result_text: Option<&str>,
    intake_text: Option<&str>,
) -> Result<(), ExactRefusal> {
    let result = result_text.ok_or_else(|| not_delivered("result"))?;
    let result_sha256 = sha256_in_place(result.as_bytes());
    if result_sha256 != digests.result_sha256 || !hex_equals(&result_sha256, binding_text(admission, "received_bytes_sha256")) {
        return Err(not_delivered("result"));
    }
    let intake = intake_text.ok_or_else(|| not_delivered("model"))?;
    if sha256_in_place(intake.as_bytes()) != digests.intake_sha256 {
        return Err(not_delivered("model"));
    }
    Ok(())
}

/// The delivered result and the solved model, read natively (steps 3-5).
#[derive(Debug)]
pub(crate) struct BoundSource {
    pub source: Value,
    pub model: Value,
    /// `digest(model)`: the solved model's canonical SHA-256.
    pub model_sha256: String,
    /// The native reading's standing (`reader_verdict::standing`), whose
    /// standing findings the report's standing record carries.
    pub standing: Value,
}

/// Steps 3-5, after the command's lease covers the priced parse of
/// both texts and its working memory: parse and guard both texts, require
/// that the native reader reads the result and that its mechanics are solved
/// (DEL-08-01 REQ-017; eligibility is recorded, not required: see
/// [`require_eligible`]), with the invocation built from the recorded mode
/// (`{request: {model, materials: []}, solver_mode}`, the form of the
/// desktop's `NativeMechanicsInvocation`), then bind the admission's result
/// and model digests to them.
/// What a binding admits the delivered result for (DEL-08-01 REQ-017 and
/// REQ-024's present account).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceAdmission {
    /// The rendered report and its print view: any solved result the reader reads.
    Render,
    /// The package save and the user release: still a numerically eligible result,
    /// refused at step 4 as before (LIM-016).
    Eligible,
}

pub(crate) fn read_delivered_source(
    admission: &Value,
    digests: &CustodyDigests,
    result_text: &str,
    intake_text: &str,
    kind: SourceAdmission,
) -> Result<BoundSource, ExactRefusal> {
    let source: Value = serde_json::from_str(result_text).map_err(|_| not_delivered("result"))?;
    guard_json(&source).map_err(|_| not_delivered("result"))?;
    let model: Value = serde_json::from_str(intake_text).map_err(|_| not_delivered("model"))?;
    guard_json(&model).map_err(|_| not_delivered("model"))?;
    let invocation = json!({"request": {"model": model.clone(), "materials": []}, "solver_mode": digests.solver_mode.as_str()});
    let standing = reader_verdict::standing(&source, Some(&model), Some(&invocation));
    drop(invocation);
    // DEL-08-01 REQ-017: the result is admitted when the reader reads it with its
    // model and its mechanics are solved, whatever its numerical standing, which
    // the report states (REQ-024). The reader's findings are quoted, never read
    // as codes: a route refusal's finding can be the reader's own text.
    let readability = reader_readability::readability(&source, &standing);
    if readability["readable"] != Value::Bool(true) {
        let findings: Vec<&str> = readability["unreadable"].as_array().map(Vec::as_slice).unwrap_or(&[]).iter().filter_map(Value::as_str).collect();
        return Err(ExactRefusal {
            code: RESULT_NOT_READABLE,
            message: format!("{RESULT_NOT_READABLE}: the product reader cannot read the delivered result with its model (reader findings: {findings:?})."),
        });
    }
    if readability["solved"] != Value::Bool(true) {
        return Err(ExactRefusal {
            code: MECHANICS_NOT_SOLVED,
            message: format!("{MECHANICS_NOT_SOLVED}: the delivered result's mechanics are not solved."),
        });
    }
    let eligible = standing["eligible"] == Value::Bool(true);
    if kind == SourceAdmission::Eligible && !eligible {
        return Err(not_eligible());
    }
    let source_sha256 = digest(&source).map_err(|_| not_delivered("result"))?;
    if source_sha256 != binding_text(admission, "received_result_sha256") {
        return Err(not_delivered("result"));
    }
    let model_sha256 = digest(&model).map_err(|_| not_delivered("model"))?;
    if model_sha256 != binding_text(admission, "model_sha256") {
        return Err(not_delivered("model"));
    }
    Ok(BoundSource { source, model, model_sha256, standing })
}

/// DEL-08-01 REQ-024's present account (LIM-016): the package save and the
/// user release still refuse a result that is not numerically eligible, with
/// today's refusal at today's step; the rendered report does not.
fn not_eligible() -> ExactRefusal {
    ExactRefusal {
        code: STANDING_NOT_ELIGIBLE,
        message: format!("{STANDING_NOT_ELIGIBLE}: the native reading of the delivered result is not numerically eligible."),
    }
}
