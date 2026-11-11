//! The native size-and-read entry (DEL-04-11 design §8.3,
//! increment 3c part 1): the crate API the desktop's capped worker process runs
//! as its hanger-sizing task, whose reply the webview's display adapter
//! (`apps/desktop/src/features/results/hangerSizingView.ts`) takes as the
//! document and its `HangerSizingReaderEvidence`.
//!
//! The input is the raw bytes of exactly the headless input (§8.3.2),
//! `{"request": {"model", "materials"}, "sizing", "library"}`:
//! - more than [`MAX_INPUT_BYTES`] bytes is refused before anything else
//!   (`HANGER_SIZING_INPUT_TOO_LARGE`), as the headless verb refuses it;
//! - bytes that are not UTF-8 are refused (`HANGER_SIZING_INPUT_NOT_UTF8`);
//! - the text is then admitted once by the path-aware admission layer (§8.3.1)
//!   and published by the headless pipeline itself, so the document is the one
//!   [`size_hangers`](crate::publish::size_hangers) publishes for the same text.
//!
//! Either refusal forms no document and no reply ([`SizeAndReadError`]).
//! Otherwise the reply is one JSON text:
//!
//! ```text
//! {"document": <the published document, sized or refused>,
//!  "evidence": {"document_sha256": <checked canonical sha256 of document>,
//!               "verdict": <Verdict::to_json()>}}
//! ```
//!
//! The reader reads the published document in-process, given the admitted
//! `request` and `library` members of the same input (§8.3.7). It runs closure,
//! binding, arithmetic and derivation; reproduction would re-run every solve, so
//! it is skipped by [`ReadOptions::WITHOUT_REPRODUCTION`] and reported as
//! `not_checked`. A refused document gets closure and binding only, by the
//! reader's own rule.
//!
//! The reply is written compactly by `serde_json`, which writes each binary64
//! number in its shortest text that parses back to the same number, as the
//! crate already writes documents. The checked hash is of the document's
//! canonical value (`openpipestress_jcs_ijson_v1`), so it is the hash the
//! desktop's `canonicalSha256HexCheckedV1` forms on the parsed `document`.
//! The reply's byte size is [`SizeAndRead::reply_len`]; a bound on the reply
//! applies to that whole text.

use crate::admission::admit;
use crate::publish::{self, checked_sha256, Published, INPUT_TOO_LARGE, MAX_INPUT_BYTES, RESULT_NOT_PUBLISHABLE};
use crate::reader::{read_document_with, ReadOptions, Supplied, Verdict};
use serde_json::{json, Map, Value};

/// The entry's refusal of input bytes that are not UTF-8. It forms no document,
/// so it is not one of the refused document's codes (`REFUSAL_CODES`).
pub const INPUT_NOT_UTF8: &str = "HANGER_SIZING_INPUT_NOT_UTF8";

/// Why the entry formed no reply. Its message quotes no input content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SizeAndReadError {
    /// The input has more than [`MAX_INPUT_BYTES`] bytes; nothing was parsed.
    TooLarge { bytes: usize },
    /// The input is not UTF-8; `valid_up_to` is the byte offset of the first
    /// invalid sequence.
    NotUtf8 { valid_up_to: usize },
    /// The published document has no checked canonical hash. The pipeline
    /// publishes only values inside the profile, so this is not expected; it is
    /// refused rather than replied to without the evidence's binding.
    DocumentNotHashable,
}

impl SizeAndReadError {
    pub fn code(&self) -> &'static str {
        match self {
            SizeAndReadError::TooLarge { .. } => INPUT_TOO_LARGE,
            SizeAndReadError::NotUtf8 { .. } => INPUT_NOT_UTF8,
            SizeAndReadError::DocumentNotHashable => RESULT_NOT_PUBLISHABLE,
        }
    }

    pub fn message(&self) -> String {
        match self {
            SizeAndReadError::TooLarge { bytes } => {
                format!("the hanger-sizing input has {bytes} bytes, more than the {MAX_INPUT_BYTES}-byte limit")
            }
            SizeAndReadError::NotUtf8 { valid_up_to } => {
                format!("the hanger-sizing input is not UTF-8 (the first invalid sequence is at byte {valid_up_to})")
            }
            SizeAndReadError::DocumentNotHashable => {
                "the published document cannot be put in openpipestress_jcs_ijson_v1 canonical form".to_string()
            }
        }
    }
}

/// A published document, its checked hash, the reader's verdict on it, and the
/// reply text that carries all three.
#[derive(Debug, Clone)]
pub struct SizeAndRead {
    /// The document, sized or refused, exactly as the headless pipeline publishes it.
    pub published: Published,
    /// The lowercase hex SHA-256 of the document's checked canonical text.
    pub document_sha256: String,
    /// The reader's verdict on the document, reproduction `not_checked`.
    pub verdict: Verdict,
    reply: String,
}

impl SizeAndRead {
    /// The complete reply, one JSON text.
    pub fn reply(&self) -> &str {
        &self.reply
    }

    /// The reply's size in bytes (UTF-8), which a reply bound applies to.
    pub fn reply_len(&self) -> usize {
        self.reply.len()
    }

    pub fn into_reply(self) -> String {
        self.reply
    }
}

/// Sizes the hangers of one headless input and reads the published document.
pub fn size_and_read(raw: &[u8]) -> Result<SizeAndRead, SizeAndReadError> {
    if raw.len() > MAX_INPUT_BYTES {
        return Err(SizeAndReadError::TooLarge { bytes: raw.len() });
    }
    let text = std::str::from_utf8(raw).map_err(|e| SizeAndReadError::NotUtf8 { valid_up_to: e.valid_up_to() })?;
    let (published, verdict) = match admit(text) {
        Ok(input) => {
            let published = publish::size_admitted(&input);
            let supplied = Supplied { solve_request: input.get("request"), library: input.get("library") };
            let verdict = read_document_with(published.document(), supplied, ReadOptions::WITHOUT_REPRODUCTION);
            (published, verdict)
        }
        Err(finding) => {
            // No member of an input outside the profile is admitted, so none is supplied.
            let published = publish::not_admitted(finding);
            let verdict = read_document_with(published.document(), Supplied::default(), ReadOptions::WITHOUT_REPRODUCTION);
            (published, verdict)
        }
    };
    let document_sha256 = checked_sha256(published.document()).ok_or(SizeAndReadError::DocumentNotHashable)?;
    let mut evidence = Map::new();
    evidence.insert("document_sha256".into(), json!(document_sha256));
    evidence.insert("verdict".into(), verdict.to_json());
    let mut reply = Map::new();
    reply.insert("document".into(), published.document().clone());
    reply.insert("evidence".into(), Value::Object(evidence));
    let reply = serde_json::to_string(&Value::Object(reply)).expect("a JSON value with string keys serializes");
    Ok(SizeAndRead { published, document_sha256, verdict, reply })
}
