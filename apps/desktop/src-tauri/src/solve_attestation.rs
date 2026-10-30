//! Solve attestation (W-18b (r2a); DEL-08-01 Design `report-user-release.md`
//! §2.4): what this session's own solver produced, kept so that a later
//! admission can check a user's per-report statement against it.
//!
//! The worker ([`crate::solve_worker`]) computes two digests and sends them in
//! its `ATST` frame before the result:
//! - `d_M`: the SHA-256 of `checked_v1_domain_canonical(`[`SOLVED_MODEL_DOMAIN`]`,
//!   model)` over the model value it materialized, before the solve;
//! - `d_S`: the same over [`SOLVED_RESULT_DOMAIN`] and the result value, the
//!   producer's exact bytes parsed back into a value after the unchanged solve
//!   (the solver entry returns a typed envelope, not a value), before the
//!   result frame is sent.
//!
//! Each digest streams the canonical text through `ByteSink` into a hasher, so
//! no canonical text is held ([`streamed_digest`]). A value the checked profile
//! refuses gives no digest, the worker sends status 0, and that result is not
//! attested. The solve and its result bytes are unchanged either way.
//!
//! The registry ([`crate::solve_jobs`]) records each published attested solve
//! in an [`AttestationTable`] of the [`MAX_ATTESTED_SOLVES`] most recent,
//! evicting the oldest, cleared on session reset. It is separate from the job
//! records' custody digests: it is not pruned with a record and not released
//! with custody. It is never persisted and never sent to the webview: nothing
//! here serializes, writes a file or returns a digest across IPC; the lookup
//! answers only whether an attested solve is held.
//!
//! Admission (route 010, a later slice) computes the same two digests with
//! [`SolveAttestation::of`] over the model and source result it parses and asks
//! [`crate::solve_jobs::SolveJobRegistry::holds_attested_solve`] for the
//! input manifest's solver mode.

use open_pipe_stress_canonical_json::{checked_v1_domain_canonical, ByteSink, DomainPayload};
use open_pipe_stress_product_physics::PreviewSolverMode;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// The domain of the solved model's digest `d_M`.
pub(crate) const SOLVED_MODEL_DOMAIN: &str = "openpipestress.user_release.solved_model.v1";
/// The domain of the solved result's digest `d_S`.
pub(crate) const SOLVED_RESULT_DOMAIN: &str = "openpipestress.user_release.solved_result.v1";
/// The attestation table's bound: the most recent attested solves kept.
pub(crate) const MAX_ATTESTED_SOLVES: usize = 32;
/// The `ATST` frame's body when its status is 1: `d_M ‖ d_S`.
pub(crate) const ATTESTATION_BYTES: usize = 64;

/// The SHA-256 of `checked_v1_domain_canonical(domain, value)`, streamed into
/// the hasher (no canonical text is held); `None` when the checked profile
/// refuses the value.
pub(crate) fn streamed_digest(domain: &str, value: &Value) -> Option<[u8; 32]> {
    let mut hasher = Sha256::new();
    checked_v1_domain_canonical(domain, DomainPayload::Value(value), &mut ByteSink(|bytes: &[u8]| hasher.update(bytes)))
        .ok()?;
    Some(hasher.finalize().into())
}

/// One solve's attestation: `d_M` and `d_S`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SolveAttestation {
    pub model_sha256: [u8; 32],
    pub result_sha256: [u8; 32],
}

impl SolveAttestation {
    /// Both digests over a model and a result value, by the same streaming
    /// function the worker uses; `None` if the checked profile refuses either.
    pub(crate) fn of(model: &Value, result: &Value) -> Option<Self> {
        Some(Self {
            model_sha256: streamed_digest(SOLVED_MODEL_DOMAIN, model)?,
            result_sha256: streamed_digest(SOLVED_RESULT_DOMAIN, result)?,
        })
    }

    /// The `ATST` body: `d_M ‖ d_S`.
    pub(crate) fn to_bytes(self) -> [u8; ATTESTATION_BYTES] {
        let mut bytes = [0u8; ATTESTATION_BYTES];
        bytes[..32].copy_from_slice(&self.model_sha256);
        bytes[32..].copy_from_slice(&self.result_sha256);
        bytes
    }

    pub(crate) fn from_bytes(bytes: &[u8; ATTESTATION_BYTES]) -> Self {
        let mut model_sha256 = [0u8; 32];
        let mut result_sha256 = [0u8; 32];
        model_sha256.copy_from_slice(&bytes[..32]);
        result_sha256.copy_from_slice(&bytes[32..]);
        Self { model_sha256, result_sha256 }
    }
}

/// One attested solve: its job number (the job id is `backend-solve-job-<n>`),
/// the admitted solver mode and its attestation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AttestedSolve {
    job_number: u64,
    solver_mode: PreviewSolverMode,
    attestation: SolveAttestation,
}

/// The bounded session table of the most recent attested solves: a fixed ring
/// of [`MAX_ATTESTED_SOLVES`] entries held inline (no allocation; under 8 KiB),
/// evicting the oldest.
#[derive(Debug)]
pub(crate) struct AttestationTable {
    entries: [Option<AttestedSolve>; MAX_ATTESTED_SOLVES],
    /// The slot the next record takes (the oldest entry once the ring is full).
    next: usize,
}

impl Default for AttestationTable {
    fn default() -> Self {
        Self { entries: [None; MAX_ATTESTED_SOLVES], next: 0 }
    }
}

impl AttestationTable {
    /// Records a published attested solve, evicting the oldest when full.
    pub(crate) fn record(&mut self, job_number: u64, solver_mode: PreviewSolverMode, attestation: SolveAttestation) {
        self.entries[self.next] = Some(AttestedSolve { job_number, solver_mode, attestation });
        self.next = (self.next + 1) % MAX_ATTESTED_SOLVES;
    }

    /// Forgets every attested solve (session reset).
    pub(crate) fn clear(&mut self) {
        *self = Self::default();
    }

    /// Whether an attested solve with these digests was solved in this mode.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn holds(&self, solver_mode: PreviewSolverMode, attestation: &SolveAttestation) -> bool {
        self.entries
            .iter()
            .flatten()
            .any(|entry| entry.solver_mode == solver_mode && entry.attestation == *attestation)
    }

    /// The recorded job numbers, oldest first.
    #[cfg(test)]
    pub(crate) fn job_numbers(&self) -> Vec<u64> {
        (0..MAX_ATTESTED_SOLVES)
            .filter_map(|offset| self.entries[(self.next + offset) % MAX_ATTESTED_SOLVES])
            .map(|entry| entry.job_number)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ingress::{allocation_envelope, counting::measure};
    use open_pipe_stress_canonical_json::{canonical_json, canonical_json_checked_v1_text};
    use serde_json::{json, Map};

    fn attestation(seed: u8) -> SolveAttestation {
        SolveAttestation { model_sha256: [seed; 32], result_sha256: [seed.wrapping_add(1); 32] }
    }

    /// The streamed digest is the SHA-256 of the checked profile's text of
    /// `{"domain": …, "payload": value}`, and the two domains separate.
    #[test]
    fn the_streamed_digest_is_the_checked_canonical_texts_sha256() {
        let value = json!({"b": [1, 2.5, -0.0, 1e-7, 1.5e-300, "é\u{1}"], "a": {"z": null, "y": true}});
        for domain in [SOLVED_MODEL_DOMAIN, SOLVED_RESULT_DOMAIN] {
            let text = canonical_json_checked_v1_text(&json!({"domain": domain, "payload": value}).to_string()).unwrap();
            assert_eq!(streamed_digest(domain, &value), Some(<[u8; 32]>::from(Sha256::digest(text.as_bytes()))));
        }
        assert_ne!(streamed_digest(SOLVED_MODEL_DOMAIN, &value), streamed_digest(SOLVED_RESULT_DOMAIN, &value));
        let both = SolveAttestation::of(&value, &value).unwrap();
        assert_eq!(SolveAttestation::from_bytes(&both.to_bytes()), both);
        assert_eq!(&both.to_bytes()[..32], &both.model_sha256);
    }

    /// A value the checked profile refuses has no digest, so no attestation.
    #[test]
    fn a_value_the_checked_profile_refuses_has_no_digest() {
        let unsafe_integer = json!({"n": 9_007_199_254_740_993u64});
        let integral_float = json!([1e300]);
        let mut deep = json!(0);
        for _ in 0..127 {
            deep = json!([deep]);
        }
        for refused in [&unsafe_integer, &integral_float, &deep] {
            assert_eq!(streamed_digest(SOLVED_RESULT_DOMAIN, refused), None);
            assert_eq!(SolveAttestation::of(&json!({}), refused), None);
            assert_eq!(SolveAttestation::of(refused, &json!({})), None);
        }
        let mut deepest_admitted = json!(0);
        for _ in 0..126 {
            deepest_admitted = json!([deepest_admitted]);
        }
        assert!(streamed_digest(SOLVED_MODEL_DOMAIN, &deepest_admitted).is_some());
    }

    /// Values equal after the webview round trip (parse, RFC 8785 text, parse)
    /// have one canonical form and so one digest: `5.0` and `5`, exponent forms.
    #[test]
    fn a_round_tripped_value_has_the_same_digest() {
        let produced = r#"{"x":5.0,"y":1e-7,"z":[1.5e-300,-0.0,123456789012.5,4.0e15],"k":{"é":"\u0001"}}"#;
        let value: Value = serde_json::from_str(produced).unwrap();
        let resubmitted: Value = serde_json::from_str(&canonical_json(&value)).unwrap();
        assert_ne!(value, resubmitted, "5.0 comes back as the integer 5");
        let digest = streamed_digest(SOLVED_RESULT_DOMAIN, &value);
        assert!(digest.is_some());
        assert_eq!(digest, streamed_digest(SOLVED_RESULT_DOMAIN, &resubmitted));
    }

    #[test]
    fn the_table_is_bounded_evicts_the_oldest_and_clears() {
        let mut table = AttestationTable::default();
        assert!(std::mem::size_of::<AttestationTable>() < 8 * 1024, "{}", std::mem::size_of::<AttestationTable>());
        let mode = PreviewSolverMode::SparseInteractive;
        let total = MAX_ATTESTED_SOLVES as u64 + 5;
        for job in 1..=total {
            table.record(job, mode, attestation(job as u8));
        }
        assert_eq!(table.job_numbers(), (6..=total).collect::<Vec<_>>(), "the five oldest were evicted");
        assert!(!table.holds(mode, &attestation(5)), "evicted");
        assert!(table.holds(mode, &attestation(6)));
        assert!(!table.holds(PreviewSolverMode::DenseScrutiny, &attestation(6)), "another mode");
        let mut model_only = attestation(6);
        model_only.result_sha256 = attestation(7).result_sha256;
        assert!(!table.holds(mode, &model_only), "both digests must match one entry");
        table.clear();
        assert!(table.job_numbers().is_empty());
        assert!(!table.holds(mode, &attestation(6)));
    }

    /// Nothing in this module persists or serializes the table or a digest.
    #[test]
    fn the_table_is_never_persisted_or_serialized() {
        let source = include_str!("solve_attestation.rs");
        let body = &source[..source.find("#[cfg(test)]\nmod tests").unwrap()];
        for forbidden in ["std::fs", "File", "Serialize", "serde::", "tauri::", "rusqlite"] {
            assert!(!body.contains(forbidden), "{forbidden}");
        }
    }

    // --- allocation checks (§2.4, §6) -----------------------------------------

    /// The price of a `Vec` of `n` 8-byte entries (`Vec<&String>`).
    fn pointers(n: usize) -> u64 {
        allocation_envelope(8 * n as u64).unwrap()
    }

    /// The stable sort's scratch for an object of `m` members: std's driftsort
    /// asks for max(m, 48) entries (for m below 10^6), on the stack up to
    /// 4 KiB and on the heap above; insertion sort (m <= 20) asks for none.
    fn sort_scratch(m: usize) -> u64 {
        if m <= 20 {
            0
        } else {
            pointers(m.max(48))
        }
    }

    /// `write_string`'s escaped form of a string of `len` bytes: serde_json's
    /// `to_string` starts a 128-byte buffer and doubles it; the text is at most
    /// 6 len + 2 bytes. A growth holds the old buffer (< the text) and the new
    /// one (<= twice the text) at once.
    fn escaped(len: usize) -> u64 {
        let text = 6 * len as u64 + 2;
        allocation_envelope(text.max(128) * 2).unwrap() + allocation_envelope(text.max(128)).unwrap()
    }

    /// `canonical_number`, `ecma_number_to_string` and `shortest_digits`: the
    /// significand digits (<= 17 bytes, priced 32), the rendered text (<= 24
    /// bytes) during one growth (19 to 38 bytes, priced 32 + 64) and the
    /// exponent text (priced 16) are the most live at once (144); inside
    /// `shortest_digits` the digit text with one growth and the digits (80).
    const NUMBER_TEMPORARIES: u64 = 160;

    /// The §2.4 account of a value's streamed digest: the key vectors of the
    /// objects on the deepest-priced open path, plus the largest single
    /// transient (one object's sort scratch, one escaped string or the number
    /// temporaries), and the domain string's escaped form.
    fn account(value: &Value) -> u64 {
        fn walk(value: &Value) -> (u64, u64) {
            match value {
                Value::Object(map) => {
                    let mut path = 0;
                    let mut transient = sort_scratch(map.len());
                    for (key, member) in map {
                        let (inner_path, inner_transient) = walk(member);
                        path = path.max(inner_path);
                        transient = transient.max(inner_transient).max(escaped(key.len()));
                    }
                    (pointers(map.len()) + path, transient)
                }
                Value::Array(items) => items.iter().map(walk).fold((0, 0), |(p, t), (ip, it)| (p.max(ip), t.max(it))),
                Value::String(text) => (0, escaped(text.len())),
                Value::Number(_) => (0, NUMBER_TEMPORARIES),
                _ => (0, 0),
            }
        }
        let (path, transient) = walk(value);
        path + transient.max(escaped(SOLVED_RESULT_DOMAIN.len()))
    }

    /// Objects nested to the checked profile's limit (the payload is the
    /// wrapper's member, so 126 levels), each with `members` members.
    fn deep_wide(members: usize) -> Value {
        let mut value = json!(0);
        for level in 0..126 {
            let mut map = Map::new();
            for index in 0..members - 1 {
                map.insert(format!("k{level}_{index:04}"), json!(index));
            }
            map.insert("~nested".into(), value);
            value = Value::Object(map);
        }
        value
    }

    fn widest(members: usize) -> Value {
        Value::Object((0..members).map(|index| (format!("member{index:07}"), json!(true))).collect())
    }

    fn escaping_string(len: usize) -> Value {
        json!({"s": "\u{1}".repeat(len)})
    }

    /// Seventeen-digit significands from 1e-300 to 1e8 (exponential below
    /// 1e-6, decimal above; the checked profile refuses integral floats past
    /// 2^53, so no larger value is admitted), alternating in sign.
    fn numeric(count: usize) -> Value {
        Value::Array(
            (0..count)
                .map(|index| {
                    let sign = if index % 2 == 0 { "" } else { "-" };
                    let exponent = -300 + (index % 309) as i32;
                    json!(format!("{sign}1.2345678901234567e{exponent}").parse::<f64>().unwrap())
                })
                .collect(),
        )
    }

    /// §2.4 and §6: each transient piece, maximized on its own invented value,
    /// stays within the account. The numeric value, whose pieces are all
    /// small, shows that no canonical text is held.
    #[test]
    fn streamed_digest_peaks_stay_within_the_transient_account() {
        let cases = [
            ("key vectors on the open path", deep_wide(600)),
            ("one object's sort scratch", widest(200_000)),
            ("one escaped string", escaping_string(1 << 20)),
            ("number formatting", numeric(50_000)),
        ];
        for (piece, value) in cases {
            let text_len = canonical_json(&value).len() as u64;
            let (digest, measured) = measure(|| streamed_digest(SOLVED_RESULT_DOMAIN, &value));
            assert!(digest.is_some(), "{piece}");
            let bound = account(&value);
            eprintln!("ATTESTATION allocation check, {piece}: peak {} of account {bound} (canonical text {text_len} bytes)", measured.peak_priced);
            assert!(measured.peak_priced <= bound, "{piece}: {measured:?} > {bound}");
            if piece == "number formatting" {
                assert!(value.as_array().unwrap().iter().all(Value::is_f64));
                assert!(measured.peak_priced <= NUMBER_TEMPORARIES + escaped(SOLVED_RESULT_DOMAIN.len()), "{measured:?}");
                assert!(measured.peak_priced * 1000 < text_len, "no canonical text is held ({measured:?}, text {text_len})");
            }
        }
    }

    /// §2.4 and §6, in the worker: the `d_S` pass parses the producer's bytes
    /// into a value and digests it, so its peak is within the text's
    /// materialization bound (the ingress prescan's, over the same requested
    /// sizes) plus the streamed digest's account; the `d_M` pass over the
    /// already materialized model is within the account alone. Checked on each
    /// invented value of the account checks and on the solved demo in both modes.
    #[test]
    fn the_workers_digest_passes_stay_within_their_bounds() {
        let model = crate::load_preview_model().unwrap();
        let mut cases: Vec<(String, Value)> = vec![
            ("key vectors on the open path".into(), deep_wide(600)),
            ("one object's sort scratch".into(), widest(200_000)),
            ("one escaped string".into(), escaping_string(1 << 20)),
            ("number formatting".into(), numeric(50_000)),
        ];
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let text = crate::solve_preview_mechanics_with_mode(model.clone(), mode).unwrap();
            cases.push((format!("solved demo, {}", mode.as_str()), serde_json::from_str(&text).unwrap()));
        }
        for (case, value) in &cases {
            let text = serde_json::to_string(value).unwrap();
            let materialization = crate::ingress::prescan(text.as_bytes()).unwrap().materialization_bytes;
            let (digest, measured) = measure(|| crate::solve_worker::result_digest(&text));
            assert_eq!(digest, streamed_digest(SOLVED_RESULT_DOMAIN, value), "{case}");
            assert!(digest.is_some(), "{case}");
            let bound = materialization + account(value);
            eprintln!("ATTESTATION worker d_S pass, {case}: peak {} of {bound} (materialization {materialization}, text {} bytes)", measured.peak_priced, text.len());
            assert!(measured.peak_priced <= bound, "{case}: {measured:?} > {bound}");
        }
        let (digest, measured) = measure(|| streamed_digest(SOLVED_MODEL_DOMAIN, &model));
        assert!(digest.is_some());
        eprintln!("ATTESTATION worker d_M pass, demo model: peak {} of account {}", measured.peak_priced, account(&model));
        assert!(measured.peak_priced <= account(&model), "{measured:?}");
    }
}
