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
//! - **Admission and memory** (plan §2, slice 1, and §8). Every lease this
//!   command takes is pool-admitted before what it covers is allocated, and
//!   none exceeds the per-invocation budget or, whatever budget is
//!   configured, the owner-adopted 12 GiB default:
//!   - the args lease ([`SolveJobRegistry::acquire_interactive_ingress_capped`]):
//!     first the raw body's copy; grown, before the args are parsed, by the
//!     prescan's admission bound (`command_ingress::admit_raw`); and grown,
//!     before the readers parse them, by each argument text's priced parse
//!     ([`command_ingress::text_parse_bytes`]). The cap is kept on the lease,
//!     so each growth is checked under the pool lock;
//!   - a diagnostic lease for each text parsed to find the reader's refusal,
//!     of that parse's priced allocations ([`ingress::prescan_allowances`], or
//!     [`ingress::length_allowances`] for the refused text);
//!   - the held text's parse ([`SolveJobRegistry::acquire_result_parse`]), of
//!     the `Value` materialization the prescan bounds. The held bytes are lent
//!     by [`SolveJobRegistry::held_result`] (no copy; their result lease stays
//!     with them).
//!
//!   The diagnostic and parse leases are adaptive and re-sampled, like the
//!   result and custody leases. A refusal is the typed `RESOURCE_UNAVAILABLE`.
//! - **Off the executor.** On the async runtime the command only checks the
//!   header, reserves the raw body's copy and copies it ([`reserve`]). The
//!   admission, the diagnostics and the read run in a blocking task ([`run`]),
//!   so neither input-sized admission nor reads occupy the runtime's workers.
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

/// The executor's part of the command: O(1) checks, the reservation of the
/// raw body's copy, then the copy itself (one memcpy, under its reservation;
/// Tauri lends the body only by reference, so a blocking task needs its own).
/// No prescan, parse or materialization runs here.
pub(crate) fn reserve(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<(Vec<u8>, Lease), CommandError<String>> {
    let raw = command_ingress::raw_args_body(body, headers, COMMAND)?;
    let lease = registry.acquire_interactive_ingress_capped(ingress::allocation_envelope(raw.len() as u64).unwrap_or(u64::MAX))?;
    Ok((raw.to_vec(), lease))
}

/// The command: [`reserve`] on the caller's executor, then [`run`] in a
/// blocking task that owns the copy, its lease and a registry handle.
pub(crate) async fn verdict(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<String, CommandError<String>> {
    let (raw, lease) = reserve(registry, body, headers)?;
    let registry = registry.clone();
    tauri::async_runtime::spawn_blocking(move || run(registry, raw, lease))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(VERDICT_READ_FAILED.into())))
}

/// The blocking part: the arguments' admission, then the dispatch outcome
/// text of the held result.
pub(crate) fn run(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<String, CommandError<String>> {
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(&registry, "admit"));
    // `raw` is the copy `copy` reserves; the args lease adds the prescan's
    // admission bound to it, before any deserialization.
    let copy_bytes = copy.bytes();
    let (mut args, mut lease) = command_ingress::admit_raw(&registry, &raw, COMMAND, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        lease.grow_to(copy_bytes.saturating_add(bytes)).map_err(|refusal| registry.refuse(refusal))?;
        Ok(lease)
    })?;
    drop(raw);
    let job_id: String = args.decode("jobId")?;
    let model_json: Option<String> = args.decode("modelJson")?;
    let invocation_json: Option<String> = args.decode("invocationJson")?;
    drop(args);
    let held = registry.held_result(&job_id)?;
    // The reader's order: the source, the model, the invocation. Each is
    // prescanned without allocating. If one is refused, the reader's own
    // refusal is found by the diagnostic parse (`reader_parse_error`) of each
    // text up to it, in turn (an earlier text the prescan admits can still
    // fail serde_json's parse: a number beyond binary64), each under a lease
    // of that parse's priced allocations, admitted first.
    let texts = [("source", Some(held.text())), ("model", model_json.as_deref()), ("invocation", invocation_json.as_deref())];
    let mut bounds = [None; 3];
    for (index, (label, text)) in texts.iter().enumerate() {
        let Some(text) = text else { continue };
        match ingress::prescan_allowances(text.as_bytes()) {
            Ok(bound) => bounds[index] = Some(bound),
            Err(invalid) => {
                for (earlier, (label, text)) in texts[..=index].iter().enumerate() {
                    let Some(text) = text else { continue };
                    let allowances = match bounds[earlier] {
                        Some((_, allowances)) => allowances,
                        None => ingress::length_allowances(text.len() as u64).unwrap_or(u64::MAX),
                    };
                    let diagnostic = registry.acquire_result_parse(allowances)?;
                    let refusal = reader_parse_error(label, text);
                    drop(diagnostic);
                    if let Some(refusal) = refusal {
                        return Err(CommandError::Command(refusal));
                    }
                }
                return Err(CommandError::Command(prescan_refusal(label, invalid)));
            }
        }
    }
    let (source_bound, _) = bounds[0].expect("the source is always prescanned");
    let mut argument_bytes = 0u64;
    for (label, text) in &texts[1..] {
        let Some(text) = text else { continue };
        let bytes = command_ingress::text_parse_bytes(text, ArgsDecoding::Moved)
            .map_err(|invalid| CommandError::Command(prescan_refusal(label, invalid)))?;
        argument_bytes = argument_bytes.saturating_add(bytes);
    }
    // Capped as the args lease was admitted (`acquire_interactive_capped`).
    lease
        .grow_to(lease.bytes().saturating_add(argument_bytes))
        .map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
    let reading = registry.acquire_result_parse(source_bound.materialization_bytes)?;
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(&registry, "read"));
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
