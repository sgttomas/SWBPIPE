# WORKING_ITEMS Run Record — H1 Canonical-Hash Unification (TP-H1-HASHUNIFY-001)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), executing completion-plan §3
  hardening-lane item H1 (verification finding F-5a) under the human's
  standing approval for the unblocked ruled lanes.
- Owning deliverable: DEL-08-02 (audit manifest and model hash — the hash
  evidence surface this tranche unifies).
- Commits: `6e36f5da4` (part 1: engine exports + parity corpus),
  part 2 (adapter swap, corpus tightening, fixtures re-bless) in the
  closing commit of this record.

## What changed

- **Engine (part 1).** `canonical_json` / `sha256_hex` are public;
  `wasm_api` exports `canonical_json_string` / `canonical_sha256_hex`
  (throwing `WASM-ENGINE-INPUT-JSON-INVALID` on malformed input). A
  14-case Rust-blessed parity corpus (`fixtures/canonical_hash/`,
  `CANONICAL_HASH_BLESS=1` regeneration, 12-case floor) pins canonical
  form and hash; the Vitest lane replays the same raw input text through
  the wasm exports.
- **Frontend (part 2).** `hashService.ts` no longer contains any
  canonicalization or digest code: `canonicalJsonString` /
  `canonicalSha256Hex` call the wasm engine, and the evidence-record
  constructors (`computeModelHash`, `computeProjectEnvelopeHash`,
  `computePackageHash`) wrap them. `usePackageHash` passes payloads (not
  pre-canonicalized strings); the A7 `renderableReportInput.ts` local
  WebCrypto helper — including its silent `"TBD"`-on-missing-subtle soft
  fallback — is deleted in favor of the same seam. No fallback hashing
  path exists anywhere in the frontend.

## The allowlist-tightening rider: measured, partially refuted

The H1 rider hypothesized (from the T4 caveat) that the corpus
engine-identity exclusions for `backend_model_hash` fields could be
removed now that both lanes run the same Rust code. Execution measured
two refutations, now documented in the corpus README and pinned by
assertions:

1. **JS transport erases numeric text.** `JSON.stringify` renders the
   file text `200.0` as `200`; the engine hashes the text it receives, so
   the native-file lane and JS-fed lanes produce different — equally
   honest — input-model hashes. `model_basis.backend_model_hash` stays
   excluded for this measured reason (not the retired TS engine).
2. **`applied_model_backend_hash` is Rust-side-verifiable only.** The
   engine hashes its in-process document, where self-produced integral
   doubles render as `...0.0`; that text cannot survive any JS
   round-trip. Consequence recorded for future work: no frontend can
   recompute a receipt's backend hash from the JSON it holds —
   verification must re-ask the engine.

What WAS tightened: `model_basis.backend_canonicalization` (invariant
label) joined the blessed semantic projection across all 44 re-blessed
cases; the native runner gained the engine self-consistency assertion
(`applied_model_backend_hash` = `canonical_json`+`sha256_hex` over the
returned applied document); JS lanes assert presence plus the
transport-invariant harness hash. The ECMA harness renderer survives in
both runners as explicitly harness-only code with the measured rationale
attached; the engine's real canonicalization is pinned by the new parity
corpus instead.

## Evidence (2026-06-11, this host)

- Native: `cargo test` in `operation_applier` green (unit + corpus +
  parity suites), with fixtures re-blessed and reviewed.
- Desktop Vitest: 166/166 (8 files) — was 140; +14 parity cases and new
  adapter/seam tests; the full App suite hashes through the wasm engine.
- DEC-025 five-surface sweep (`tools/release/run_evidence_sweep.py
  --execute`): summary artifact in `validation/evidence/sweeps/` bound to
  the closing commit (see SMOKE TP-MAC-114 for the result line).

## Boundary review

Local-only; invented fixture values; no protected standards content; no
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claims. Hash labels remain
`computed_local_preview`; nothing here creates evidence reliance.
