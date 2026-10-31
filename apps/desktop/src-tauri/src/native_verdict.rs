//! C1 native custody, slice 1 (DEL-04-05 `c1-native-custody-plan.md`;
//! design `result-volume-design.md` §3.2, §4.1): the base readers' dispatch
//! verdict on a retained solve result, read in this process from the held
//! bytes, so the webview need not hold the result to know its verdict.
//!
//! - **The same reader.** The verdict is
//!   `operation_applier::reader_dispatch_json`, the function the wasm export
//!   `reader_dispatch_json` wraps (`resultReader.ts` `readerDispatch`), on the
//!   held text and the model and invocation texts the webview would hand the
//!   wasm engine. Its outcome text is returned unchanged.
//! - **Inputs that are not JSON.** Each text is first prescanned without
//!   allocating, in the reader's own order (source, model, invocation). A text
//!   the prescan refuses is refused with the reader's own message: serde_json
//!   reads it into [`Discard`], which goes through the same parser path as
//!   `Value` and so gives the same error, but builds nothing (only its
//!   escaped-string scratch and the error). The text is
//!   `WASM-ENGINE-INPUT-JSON-INVALID: <label> is not valid JSON: <error>`, byte
//!   for byte the wasm engine's, which the desktop reads as `JSON_PARSE_REJECTED`.
//! - **One exception: the `RawValue` token.** This crate's serde_json has
//!   `raw_value`, under which `Value` treats an object key equal to serde_json's
//!   private `RawValue` token as an embedded raw value; the wasm engine's has
//!   not, and reads the key as an ordinary key. The prescan refuses the key, in
//!   the held result and in the caller's model and invocation texts alike, so
//!   no text is read differently from the wasm engine: it is refused here
//!   (`<label> is not valid JSON: <prescan reason> at byte <offset>`) where the
//!   wasm engine would read it. The engine never publishes such a key, and a
//!   solve's model body holding it is refused at the solve's own ingress
//!   prescan, so only a caller-supplied text that was never solved can meet
//!   this refusal (`a_raw_value_token_key_is_the_one_refused_exception`).
//! - **Admission and memory** (plan §2, slice 1, and §8). The arguments are
//!   admitted as an interactive command's (`command_ingress`, raw args body),
//!   and that lease grows by each argument text's priced parse
//!   ([`command_ingress::text_parse_bytes`]), within the per-invocation budget.
//!   The held bytes are lent by [`SolveJobRegistry::held_result`] (no copy;
//!   their result lease stays with them). Their parse is admitted, after the
//!   prescan and before the parse allocates anything, as a pool lease of the
//!   `Value` materialization the prescan bounds
//!   ([`SolveJobRegistry::acquire_result_parse`]): adaptive and re-sampled, like
//!   the result and custody leases, and refused above the per-invocation
//!   budget and above the 12 GiB default. A refusal is the typed
//!   `RESOURCE_UNAVAILABLE`.
//! - **Off the executor.** The command admits its arguments on the async
//!   runtime, then moves them, their lease and a registry handle into a
//!   blocking task ([`verdict`]), so reads never occupy the runtime's workers.
//! - **Residual.** The readers' own working memory runs under these leases
//!   but is not separately priced, as `command_ingress` states for every
//!   command's working memory (plan §7, item 3).

use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::ingress;
use crate::memory_pool::Lease;
use crate::solve_jobs::SolveJobRegistry;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use std::fmt;
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

/// The command's name, as `command_ingress` reports it.
pub(crate) const COMMAND: &str = "read_preview_mechanics_job_verdict";
/// The refusal prefix of an input that is not JSON: the text
/// `operation_applier`'s reader entries (and so the wasm engine) return.
pub(crate) const INPUT_JSON_INVALID: &str = "WASM-ENGINE-INPUT-JSON-INVALID";
/// The blocking read ended without an outcome (it panicked).
pub(crate) const VERDICT_READ_FAILED: &str = "NATIVE-VERDICT-READ-FAILED";

/// Admitted arguments: the registry handle, the args lease and the texts.
pub(crate) struct Admitted {
    registry: SolveJobRegistry,
    lease: Lease,
    job_id: String,
    model_json: Option<String>,
    invocation_json: Option<String>,
}

/// Admits the raw args body `{jobId, modelJson?, invocationJson?}`.
pub(crate) fn admit(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<Admitted, CommandError<String>> {
    let (mut args, lease) = command_ingress::admit_body(registry, body, headers, COMMAND, ArgsDecoding::Moved)?;
    let job_id: String = args.decode("jobId")?;
    let model_json: Option<String> = args.decode("modelJson")?;
    let invocation_json: Option<String> = args.decode("invocationJson")?;
    Ok(Admitted { registry: registry.clone(), lease, job_id, model_json, invocation_json })
}

/// The command: admission on the caller's executor, then the read in a
/// blocking task that owns the arguments, their lease and the registry handle.
pub(crate) async fn verdict(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<String, CommandError<String>> {
    let admitted = admit(registry, body, headers)?;
    tauri::async_runtime::spawn_blocking(move || read(admitted))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(VERDICT_READ_FAILED.into())))
}

/// The blocking read: the dispatch outcome text of the held result.
pub(crate) fn read(admitted: Admitted) -> Result<String, CommandError<String>> {
    let Admitted { registry, mut lease, job_id, model_json, invocation_json } = admitted;
    let held = registry.held_result(&job_id)?;
    // The reader's order: the source, the model, the invocation. A text the
    // prescan admits can still fail serde_json's parse (a number beyond
    // binary64), so before a later text's refusal the earlier texts are
    // parsed into `Discard`: the first text the reader refuses is refused.
    let texts = [("source", Some(held.text())), ("model", model_json.as_deref()), ("invocation", invocation_json.as_deref())];
    let mut bounds = [None; 3];
    for (index, (label, text)) in texts.iter().enumerate() {
        let Some(text) = text else { continue };
        bounds[index] = Some(ingress::prescan(text.as_bytes()).map_err(|invalid| {
            let earlier = texts[..index].iter().find_map(|(label, text)| text.and_then(|text| reader_parse_error(label, text)));
            CommandError::Command(earlier.or_else(|| reader_parse_error(label, text)).unwrap_or_else(|| prescan_refusal(label, invalid)))
        })?);
    }
    let source_bound = bounds[0].expect("the source is always prescanned");
    let mut argument_bytes = 0u64;
    for (label, text) in &texts[1..] {
        let Some(text) = text else { continue };
        let bytes = command_ingress::text_parse_bytes(text, ArgsDecoding::Moved)
            .map_err(|invalid| CommandError::Command(prescan_refusal(label, invalid)))?;
        argument_bytes = argument_bytes.saturating_add(bytes);
    }
    lease
        .grow_to(lease.bytes().saturating_add(argument_bytes))
        .map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
    let reading = registry.acquire_result_parse(source_bound.materialization_bytes)?;
    #[cfg(test)]
    tests::gate::pass(std::sync::Arc::as_ptr(&held) as usize);
    let outcome = open_pipe_stress_operation_applier::reader_dispatch_json(held.text(), model_json.as_deref(), invocation_json.as_deref())
        .map_err(CommandError::Command);
    drop((reading, held, lease));
    outcome
}

/// The reader's refusal of a text that is not JSON (`operation_applier`'s
/// `parse_reader_input`), or none if serde_json reads it.
pub(crate) fn reader_parse_error(label: &str, text: &str) -> Option<String> {
    serde_json::from_str::<Discard>(text).err().map(|error| format!("{INPUT_JSON_INVALID}: {label} is not valid JSON: {error}"))
}

/// The prescan's own refusal: the `RawValue` exception (module docs).
fn prescan_refusal(label: &str, invalid: ingress::IngressInvalid) -> String {
    format!("{INPUT_JSON_INVALID}: {label} is not valid JSON: {} at byte {}", invalid.reason, invalid.offset)
}

/// A JSON value read and discarded through serde_json's `deserialize_any`,
/// the path `Value` takes (numbers parsed, strings decoded, map keys read as
/// strings), so its errors are `Value`'s, without building anything.
pub(crate) struct Discard;

impl<'de> Deserialize<'de> for Discard {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(DiscardVisitor)
    }
}

struct DiscardVisitor;

impl<'de> Visitor<'de> for DiscardVisitor {
    type Value = Discard;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any JSON value")
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> Result<Discard, E> {
        Ok(Discard)
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> Result<Discard, E> {
        Ok(Discard)
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> Result<Discard, E> {
        Ok(Discard)
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> Result<Discard, E> {
        Ok(Discard)
    }
    fn visit_str<E: de::Error>(self, _: &str) -> Result<Discard, E> {
        Ok(Discard)
    }
    fn visit_unit<E: de::Error>(self) -> Result<Discard, E> {
        Ok(Discard)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Discard, A::Error> {
        while seq.next_element::<Discard>()?.is_some() {}
        Ok(Discard)
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Discard, A::Error> {
        while map.next_key::<Discard>()?.is_some() {
            map.next_value::<Discard>()?;
        }
        Ok(Discard)
    }
}

#[cfg(test)]
#[path = "native_verdict_tests.rs"]
pub(crate) mod tests;
