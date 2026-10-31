//! C1 native custody, slice 1 (DEL-04-05 `c1-native-custody-plan.md`;
//! design `result-volume-design.md` §3.2, §4.1): the base readers' dispatch
//! verdict on a retained solve result, read in this process from the held
//! bytes, so the webview need not hold the result to know its verdict.
//!
//! - **The same reader.** The verdict is
//!   `operation_applier::reader_dispatch_json`, the function the wasm export
//!   `reader_dispatch_json` wraps (`resultReader.ts` `readerDispatch`), on the
//!   held text and the model and invocation texts the webview would hand the
//!   wasm engine. Its outcome text is returned unchanged, and an input that
//!   is not JSON returns that function's `WASM-ENGINE-INPUT-JSON-INVALID`
//!   text, which the desktop reads as `JSON_PARSE_REJECTED`.
//! - **Admission.** The arguments are admitted as an interactive command's
//!   (`command_ingress`, raw args body), and the lease grows by each argument
//!   text's priced parse ([`command_ingress::text_parse_bytes`]). The held
//!   bytes are lent by [`SolveJobRegistry::held_result`] (no copy; their
//!   result lease stays with them), and their parse is admitted as a pool
//!   lease of the `Value` materialization the ingress prescan prices
//!   ([`source_parse_bytes`]): adaptive and re-sampled, like the result and
//!   custody leases. A refusal is the typed `RESOURCE_UNAVAILABLE`.
//! - **Residual.** The readers' own working memory runs under these leases
//!   but is not separately priced, as `command_ingress` states for every
//!   command's working memory (plan §7, item 3).
//! - **One named divergence.** The prescan refuses a text holding serde_json's
//!   `RawValue` token as an object key, which the wasm engine (built without
//!   `raw_value`) would read; it is refused here as not JSON rather than
//!   misread. The engine never publishes such a key.

use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::ingress;
use crate::solve_jobs::SolveJobRegistry;
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

/// The command's name, as `command_ingress` reports it.
pub(crate) const COMMAND: &str = "read_preview_mechanics_job_verdict";
/// The refusal prefix of an input that is not JSON: the text
/// `operation_applier`'s reader entries (and so the wasm engine) return.
pub(crate) const INPUT_JSON_INVALID: &str = "WASM-ENGINE-INPUT-JSON-INVALID";

/// The pool lease of the held text's parse: the `Value` materialization the
/// ingress prescan bounds, without allocating. A text the prescan refuses is
/// refused as `operation_applier`'s parse would refuse it.
pub(crate) fn source_parse_bytes(text: &str) -> Result<u64, String> {
    ingress::prescan(text.as_bytes())
        .map(|bound| bound.materialization_bytes)
        .map_err(|invalid| not_json("source", invalid))
}

fn not_json(label: &str, invalid: ingress::IngressInvalid) -> String {
    format!("{INPUT_JSON_INVALID}: {label} is not valid JSON: {} at byte {}", invalid.reason, invalid.offset)
}

/// The command on a raw args body `{jobId, modelJson?, invocationJson?}`:
/// the dispatch outcome text, read from the held result.
pub(crate) fn verdict(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<String, CommandError<String>> {
    let (mut args, mut lease) = command_ingress::admit_body(registry, body, headers, COMMAND, ArgsDecoding::Moved)?;
    let job_id: String = args.decode("jobId")?;
    let model_json: Option<String> = args.decode("modelJson")?;
    let invocation_json: Option<String> = args.decode("invocationJson")?;
    drop(args);
    for (label, text) in [("model", &model_json), ("invocation", &invocation_json)] {
        let Some(text) = text else { continue };
        let bytes = command_ingress::text_parse_bytes(text, ArgsDecoding::Moved)
            .map_err(|invalid| CommandError::Command(not_json(label, invalid)))?;
        lease
            .grow_to(lease.bytes().saturating_add(bytes))
            .map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
    }
    let held = registry.held_result(&job_id)?;
    let parse = source_parse_bytes(held.text()).map_err(CommandError::Command)?;
    let reading = registry.acquire_result_parse(parse)?;
    let outcome = open_pipe_stress_operation_applier::reader_dispatch_json(held.text(), model_json.as_deref(), invocation_json.as_deref())
        .map_err(CommandError::Command);
    drop((reading, held, lease));
    outcome
}

#[cfg(test)]
#[path = "native_verdict_tests.rs"]
mod tests;
