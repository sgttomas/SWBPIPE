//! The native render's identity boundary (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §3.6 step 0 and §5.5).
//!
//! Owner ruling 10A kept exact-route report outputs refused until the owner
//! opened them; the owner opened the rendered report and the report package
//! for pressure-2 and pressure-source-1 on 2026-10-26 (§5.1: "My two
//! decisions are to approve your recommendations in both cases. Proceed
//! accordingly."). The report package enforces its admitted set at its Rust
//! identity boundary (item 0); the native render command enforces its own
//! here, and still refuses every other exact identity (physics-retained-1).
//! Step 0 runs first for
//! every render input, on the argument value that `command_ingress::admit`
//! already priced and decoded, before any further parse, custody query, lease
//! growth or render, and it allocates nothing before it decides.
//!
//! Step 0 recognizes an exact-route input only by structured markers at
//! defined paths, never by free text such as a load-case label.
//!
//! A [`RenderAdmittedSet`] names the exact identities the render admits. Its
//! field is private to this module, so no other code can construct one.
//! [`PRODUCTION`], which wraps [`RENDER_ADMITTED_EXACT_CONTRACTS`], is the only
//! value there is, in every build, test builds included (the test-only witness
//! set used before opening is removed). The set reaches step 0 only along the
//! graph that
//! `report_render_exact_tests::render_admitted_set_forwarding_graph_is_closed`
//! verifies, which also asserts that no other set exists.
//!
//! Behind step 0 (slice (b), §3.6), an input with `exact_route` that step 0
//! admits is bound to the
//! source natively before it is rendered: the named job's custody, the
//! resubmitted result, intake and input-manifest texts against the digests
//! the host recorded, a priced parse under the command's lease, the native
//! reader's eligible standing, and the re-derivation of the rows, the exact
//! limitation entries and the `exact_route` section from the delivered result
//! with the report package's own functions (`exact_source_binding.rs`). An
//! input that carries the texts without `exact_route` refuses. Standard claim
//! fence applies (claims taxonomy per DEC-081).

use crate::command_ingress::{text_parse_bytes, Args, ArgsDecoding, CommandError, REPORT_PACKAGE_WORKING_MEMORY_FACTOR};
use crate::exact_source_binding::{self as binding, sections_mismatch, ExactRefusal};
use crate::memory_pool::Lease;
use crate::report_package_save::{MAX_INPUT_MANIFEST_TEXT_BYTES, MAX_MODEL_TEXT_BYTES, MAX_REQUEST_TEXT_BYTES, TOO_LARGE};
use crate::solve_jobs::SolveJobRegistry;
use open_pipe_stress_report_package::bindings::expected_report_result_rows;
use open_pipe_stress_report_package::exact_sections::{check_exact_limitation_entries, expected_exact_limitations, expected_exact_route_core};
use open_pipe_stress_result_export::semantic_contract::{PHYSICS_RETAINED_ID, PRESSURE_ID, PRESSURE_SOURCE_ID};
use serde_json::Value;
use std::fmt::Write as _;

/// The exact identities whose `exact_route` the native render admits:
/// pressure-2 and pressure-source-1, since the owner opened exact-route report
/// outputs (§5.1; owner, 2026-10-26).
pub(crate) const RENDER_ADMITTED_EXACT_CONTRACTS: [&str; 2] = [PRESSURE_ID, PRESSURE_SOURCE_ID];

/// The exact-route identities step 0 recognizes at the input's identity
/// fields.
pub(crate) const EXACT_IDENTITIES: [&str; 3] = [PRESSURE_ID, PRESSURE_SOURCE_ID, PHYSICS_RETAINED_ID];

pub(crate) const RENDER_ROUTE_NOT_ADMITTED: &str = "REPORT-RENDER-ROUTE-NOT-ADMITTED";

/// An exact-route render whose `report_context.export_context` is not
/// `local_private` (Codex review round 1 of #1648): refused natively before
/// the source binding and the render, whatever the input's provenance flags
/// say, so no body or print view exists for it.
pub(crate) const RENDER_EXACT_CONTEXT_NOT_LOCAL_PRIVATE: &str = "REPORT-RENDER-EXACT-CONTEXT-NOT-LOCAL-PRIVATE";

/// A render input, or a member the native checks read, in positional (JSON
/// array) form (Codex review round 2 of #1648). The renderer's decode reads a
/// struct written as an array by field order, while step 0, the context check
/// and the source binding read members by key, so a positional form could
/// carry an exact section past them. The command refuses it before step 0;
/// the webview always sends objects.
pub(crate) const RENDER_INPUT_POSITIONAL: &str = "REPORT-RENDER-INPUT-POSITIONAL";

/// The exact identities step 0 admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderAdmittedSet(&'static [&'static str]);

impl RenderAdmittedSet {
    fn contains(&self, id: &str) -> bool {
        self.0.contains(&id)
    }
}

/// The production set: every build's only render set.
pub(crate) const PRODUCTION: RenderAdmittedSet = RenderAdmittedSet(&RENDER_ADMITTED_EXACT_CONTRACTS);

/// The render command's path after ingress: step 0 against `admitted`, then,
/// for an input with `exact_route`, the native source binding (§3.6) under
/// the command's `lease`, then the renderer.
pub(crate) fn render_calculation_report_args_in(
    args: &mut Args,
    registry: &SolveJobRegistry,
    lease: &mut Lease,
    admitted: RenderAdmittedSet,
) -> Result<Value, CommandError> {
    let input = args.value("input")?;
    check_object_form(&input).map_err(CommandError::Command)?;
    check_render_route_admitted(&input, admitted).map_err(CommandError::Command)?;
    let texts = ExactRenderTexts {
        result: args.decode("exactResultText")?,
        intake: args.decode("exactIntakeText")?,
        input_manifest: args.decode("inputManifestText")?,
    };
    let exact = input.get("exact_route").is_some_and(|route| !route.is_null());
    if exact {
        check_exact_render_context(&input).map_err(CommandError::Command)?;
        bind_exact_render_input(&input, &texts, registry, lease)?;
    } else if texts.any() {
        return Err(CommandError::Command(sections_mismatch("exact_route").message));
    }
    drop(texts);
    crate::render_calculation_report_value(input).map_err(CommandError::Command)
}

/// Object form (Codex review round 2 of #1648): the input is a JSON object,
/// and so is every struct member the native checks or the renderer's exact
/// sections read, where present and not null: `calculation_report`,
/// `report_sections`, `report_context`, `user_rule_checks`, `exact_route` and
/// its `admission`, and each element of `result_rows`,
/// `report_sections.limitations` (with its `source` and `affected_scope`),
/// `exact_route.headlines` and `library_references`. A positional (array)
/// form anywhere there refuses, naming the fixed path only; an admitted
/// input's walk reads the value in place and allocates nothing.
pub(crate) fn check_object_form(input: &Value) -> Result<(), String> {
    fn object(value: Option<&Value>, path: &str) -> Result<(), String> {
        match value {
            None | Some(Value::Null) | Some(Value::Object(_)) => Ok(()),
            Some(_) => Err(format!("{RENDER_INPUT_POSITIONAL}: the render input's {path} is not a JSON object.")),
        }
    }
    fn elements(value: Option<&Value>, path: &str, each: &dyn Fn(&Value) -> Result<(), String>) -> Result<(), String> {
        match value {
            Some(Value::Array(items)) => items.iter().try_for_each(|item| {
                object(Some(item), path)?;
                each(item)
            }),
            _ => Ok(()),
        }
    }
    if !input.is_object() {
        return Err(format!("{RENDER_INPUT_POSITIONAL}: the render input is not a JSON object."));
    }
    for member in ["calculation_report", "report_sections", "report_context", "user_rule_checks", "exact_route"] {
        object(input.get(member), member)?;
    }
    let none = |_: &Value| Ok(());
    elements(input.get("result_rows"), "result_rows element", &none)?;
    elements(input.get("library_references"), "library_references element", &none)?;
    if let Some(sections) = input.get("report_sections") {
        elements(sections.get("limitations"), "report_sections.limitations element", &|limitation| {
            object(limitation.get("source"), "report_sections.limitations element's source")?;
            object(limitation.get("affected_scope"), "report_sections.limitations element's affected_scope")
        })?;
    }
    if let Some(route) = input.get("exact_route") {
        object(route.get("admission"), "exact_route.admission")?;
        elements(route.get("headlines"), "exact_route.headlines element", &none)?;
    }
    Ok(())
}

/// An exact-route report renders only as a local private document (owner
/// ruling of 2026-10-26, §5.1; DEL-08-01 Design `exact-route-report-sections.md`
/// §6.3): its `report_context.export_context` must be `local_private`. The
/// check reads that one field, never the provenance classifications, which
/// the caller supplies; the refusal names no value from the input.
pub(crate) fn check_exact_render_context(input: &Value) -> Result<(), String> {
    match input.get("report_context").and_then(|context| context.get("export_context")) {
        Some(Value::String(context)) if context == "local_private" => Ok(()),
        found => Err(format!(
            "{RENDER_EXACT_CONTEXT_NOT_LOCAL_PRIVATE}: an exact-route report renders only under the local_private export context; the input names {}.",
            if found.is_none_or(Value::is_null) { "no export context" } else { "another export context" }
        )),
    }
}

/// The texts an exact render resubmits (§3.6); none for any other input.
pub(crate) struct ExactRenderTexts {
    pub result: Option<String>,
    pub intake: Option<String>,
    pub input_manifest: Option<String>,
}

impl ExactRenderTexts {
    fn any(&self) -> bool {
        self.result.is_some() || self.intake.is_some() || self.input_manifest.is_some()
    }
}

fn refused(refusal: ExactRefusal) -> CommandError {
    CommandError::Command(refusal.message)
}

/// The render's lease growth before its parse (§3.6, "Pricing"): the three
/// texts' priced parses ([`text_parse_bytes`]) plus W M_r, the render's
/// working-memory provision, with M_r the result text's materialization bound
/// and W [`REPORT_PACKAGE_WORKING_MEMORY_FACTOR`]. A measured provision for
/// the witness inputs, not a proven bound.
pub(crate) fn exact_render_growth_bytes(result: &str, intake: &str, input_manifest: &str) -> Result<u64, String> {
    let priced = |what: &str, text: &str| {
        text_parse_bytes(text, ArgsDecoding::Moved)
            .map_err(|invalid| format!("{}: {what}: the text is not JSON ({} at byte {}).", binding::SOURCE_NOT_DELIVERED, invalid.reason, invalid.offset))
    };
    let working = crate::ingress::prescan(result.as_bytes())
        .map_err(|invalid| format!("{}: result: the text is not JSON ({} at byte {}).", binding::SOURCE_NOT_DELIVERED, invalid.reason, invalid.offset))?
        .materialization_bytes
        .saturating_mul(REPORT_PACKAGE_WORKING_MEMORY_FACTOR);
    Ok(priced("result", result)?
        .saturating_add(priced("model", intake)?)
        .saturating_add(priced("input_manifest", input_manifest)?)
        .saturating_add(working))
}

/// The size caps of the save, before any parse (§3.6).
fn check_render_text_sizes(texts: &ExactRenderTexts) -> Result<(), String> {
    for (what, text, cap) in [
        ("result", texts.result.as_deref(), MAX_REQUEST_TEXT_BYTES),
        ("model", texts.intake.as_deref(), MAX_MODEL_TEXT_BYTES),
        ("input_manifest", texts.input_manifest.as_deref(), MAX_INPUT_MANIFEST_TEXT_BYTES),
    ] {
        let bytes = text.map_or(0, str::len) as u64;
        if bytes > cap {
            return Err(format!("{TOO_LARGE}: /{what}: The {what} text is {bytes} bytes; at most {cap} are admitted."));
        }
    }
    Ok(())
}

/// The native source binding of an exact render (§3.6): steps 1-5, then the
/// input manifest and the re-derivation of the rows, the exact limitation
/// entries and the `exact_route` section from the delivered result. Refuses
/// with the first difference; the renderer's own checks (§3.2, §3.5) follow.
fn bind_exact_render_input(
    input: &Value,
    texts: &ExactRenderTexts,
    registry: &SolveJobRegistry,
    lease: &mut Lease,
) -> Result<(), CommandError> {
    let route = &input["exact_route"];
    let admission = &route["admission"];
    // Step 1: custody.
    let digests = binding::check_custody(admission, &|job| registry.custody_binding(job)).map_err(refused)?;
    // The caps, then step 2: the delivered bytes, hashed in place.
    check_render_text_sizes(texts).map_err(CommandError::Command)?;
    binding::check_delivered_texts(admission, &digests, texts.result.as_deref(), texts.intake.as_deref()).map_err(refused)?;
    let (result, intake) = (texts.result.as_deref().unwrap_or(""), texts.intake.as_deref().unwrap_or(""));
    let manifest_text = texts.input_manifest.as_deref().ok_or_else(|| refused(sections_mismatch("input_manifest")))?;
    // Step 3: the lease grows by the priced parse and W M_r before any parse.
    let growth = exact_render_growth_bytes(result, intake, manifest_text).map_err(CommandError::Command)?;
    lease.grow_to(lease.bytes().saturating_add(growth)).map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
    // Steps 3-5: parse, the native standing, the result and model digests.
    let bound = binding::read_delivered_source(admission, &digests, result, intake).map_err(refused)?;
    // The input manifest, parsed as the save parses its own: its SHA-256 is the
    // binding's, and its model is the bound model.
    let manifest = crate::report_package_save::parse_canonical("input_manifest", manifest_text)
        .map_err(|_| refused(sections_mismatch("admission.input_manifest_sha256")))?;
    let manifest_sha256 = crate::solve_jobs::sha256_in_place(manifest_text.as_bytes());
    if !binding::hex_equals(&manifest_sha256, admission["input_manifest_sha256"].as_str().unwrap_or("")) {
        return Err(refused(sections_mismatch("admission.input_manifest_sha256")));
    }
    crate::report_package_save::check_manifest_model(&manifest, &bound.model, &bound.model_sha256)
        .map_err(|_| refused(sections_mismatch("input_manifest.model_basis")))?;
    drop(manifest);
    rederive(input, &bound.source, &bound.model_sha256, &digests).map_err(|section| refused(sections_mismatch(&section)))
}

/// The re-derivation (§3.6) with the functions the package's item 9 uses: the
/// result rows, exactly (an exact report carries no numerical-health line, so
/// no row is admitted that the delivered result does not derive), the exact
/// limitation entries with the profile reference, and the `exact_route`
/// section. `input_manifest_ref` and
/// `derivative_sha256` are checked for form here and against the report's own
/// records by the renderer (§3.2). `Err` names the section.
fn rederive(input: &Value, source: &Value, model_sha256: &str, digests: &crate::solve_jobs::CustodyDigests) -> Result<(), String> {
    let rows = expected_report_result_rows(source).map_err(|_| "result_rows".to_string())?;
    let found = input["result_rows"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    if found != rows.as_slice() {
        return Err("result_rows".to_string());
    }
    let core = expected_exact_route_core(source).map_err(|_| "exact_route".to_string())?;
    let route = &input["exact_route"];
    let admission = &route["admission"];
    let text = |value: &Value| value.as_str().unwrap_or("").to_string();
    for (section, found, expected) in [
        ("semantic_contract_id", text(&route["semantic_contract_id"]), core.semantic_contract_id.clone()),
        ("admission.semantic_contract_id", text(&admission["semantic_contract_id"]), core.semantic_contract_id.clone()),
        ("admission.run_ref", text(&admission["run_ref"]), core.run_ref.clone()),
        ("admission.result_envelope_id", text(&admission["result_envelope_id"]), core.result_envelope_id.clone()),
        ("admission.received_result_sha256", text(&admission["received_result_sha256"]), core.received_result_sha256.clone()),
        ("admission.model_sha256", text(&admission["model_sha256"]), model_sha256.to_string()),
    ] {
        if found != expected {
            return Err(section.to_string());
        }
    }
    if !binding::hex_equals(&digests.result_sha256, admission["received_bytes_sha256"].as_str().unwrap_or("")) {
        return Err("admission.received_bytes_sha256".to_string());
    }
    if text(&admission["input_manifest_ref"]).trim().is_empty() {
        return Err("admission.input_manifest_ref".to_string());
    }
    let derivative = text(&admission["derivative_sha256"]);
    if derivative.len() != 64 || !derivative.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err("admission.derivative_sha256".to_string());
    }
    let load_reference = match &route["load_reference_states"] {
        Value::Null => None,
        Value::String(line) => Some(line.clone()),
        _ => return Err("load_reference_states".to_string()),
    };
    if load_reference != core.load_reference_states {
        return Err("load_reference_states".to_string());
    }
    let headlines = serde_json::to_value(&core.headlines).map_err(|_| "headlines".to_string())?;
    if route["headlines"] != headlines {
        return Err("headlines".to_string());
    }
    let entries = expected_exact_limitations(source, &core.run_ref).map_err(|_| "limitations".to_string())?;
    let limitations = input.pointer("/report_sections/limitations").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    check_exact_limitation_entries(&entries, &core.run_ref, limitations).map_err(|detail| format!("limitations: {detail}"))
}

/// Step 0 (§3.6). It reads only structured markers at the input's defined
/// paths, never free text (labels, notes, statements, descriptions,
/// provenance or any other user-supplied value). An input is an exact-route
/// input when it carries:
/// - a non-null `exact_route`;
/// - an exact identity at an identity field: `exact_route.semantic_contract_id`,
///   `exact_route.admission.semantic_contract_id`, or the `ref_id` of a
///   `report_sections.limitations[].source` whose `ref_type` is
///   `semantic_contract`;
/// - an exact limitation entry: a `report_sections.limitations[].limitation_id`
///   `limitation:…:exact:…` (the renderer's predicate).
///
/// Such an input passes only when its `exact_route` names an identity in
/// `admitted` and every exact identity at its identity fields is in
/// `admitted`. An input with no marker renders as before: it has no
/// `exact_route`, so the renderer emits no exact block, standing line or
/// headlines, and it has no exact limitation entry. Free text equal to an
/// exact identity is therefore not route evidence. The walk reads the decoded
/// value in place and allocates nothing; only a refusal formats its message.
pub(crate) fn check_render_route_admitted(input: &Value, admitted: RenderAdmittedSet) -> Result<(), String> {
    let route = input.get("exact_route").filter(|route| !route.is_null());
    let Some(route) = route else {
        if let Some(id) = first_named(input, &|_| true) {
            return Err(refusal(admitted, &format_args!("{id}")));
        }
        if let Some(id) = first_exact_limitation(input) {
            return Err(refusal(admitted, &format_args!("the exact-route limitation entry {id} without an exact_route section")));
        }
        return Ok(());
    };
    match route.get("semantic_contract_id").and_then(Value::as_str) {
        Some(id) if admitted.contains(id) => {}
        Some(id) => return Err(refusal(admitted, &format_args!("{id}"))),
        None => return Err(refusal(admitted, &format_args!("an exact_route section that names no identity"))),
    }
    match first_named(input, &|id| !admitted.contains(id)) {
        Some(id) => Err(refusal(admitted, &format_args!("{id}"))),
        None => Ok(()),
    }
}

/// The refusal message names the admitted set (§5.1 item 4).
fn refusal(admitted: RenderAdmittedSet, found: &dyn std::fmt::Display) -> String {
    let mut message = format!("{RENDER_ROUTE_NOT_ADMITTED}: the rendered report admits ");
    if admitted.0.is_empty() {
        message.push_str("no exact-route identity");
    } else {
        message.push_str("only the exact-route identities ");
        message.push_str(&admitted.0.join(", "));
    }
    let _ = write!(message, "; found {found}.");
    message
}

/// The first exact identity at one of `input`'s identity fields, in the order
/// `exact_route.semantic_contract_id`, `exact_route.admission.semantic_contract_id`,
/// then each `semantic_contract` limitation source, for which `counts` holds.
fn first_named<'a>(input: &'a Value, counts: &dyn Fn(&str) -> bool) -> Option<&'a str> {
    let route = input.get("exact_route");
    let route_identities = [
        route.and_then(|route| route.get("semantic_contract_id")),
        route.and_then(|route| route.get("admission")).and_then(|admission| admission.get("semantic_contract_id")),
    ];
    let limitation_sources = limitations(input)
        .filter_map(|limitation| limitation.get("source"))
        .filter(|source| source.get("ref_type").and_then(Value::as_str) == Some("semantic_contract"))
        .map(|source| source.get("ref_id"));
    route_identities
        .into_iter()
        .chain(limitation_sources)
        .filter_map(|field| field.and_then(Value::as_str))
        .find(|id| EXACT_IDENTITIES.contains(id) && counts(id))
}

/// The entries of `report_sections.limitations`.
fn limitations(input: &Value) -> impl Iterator<Item = &Value> {
    // `get`, not `pointer`: a pointer lookup allocates its unescaped tokens.
    input
        .get("report_sections")
        .and_then(|sections| sections.get("limitations"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

/// The first exact-route limitation entry id in `report_sections.limitations`.
fn first_exact_limitation(input: &Value) -> Option<&str> {
    limitations(input)
        .filter_map(|limitation| limitation.get("limitation_id").and_then(Value::as_str))
        .find(|id| id.starts_with("limitation:") && id.contains(":exact:"))
}
