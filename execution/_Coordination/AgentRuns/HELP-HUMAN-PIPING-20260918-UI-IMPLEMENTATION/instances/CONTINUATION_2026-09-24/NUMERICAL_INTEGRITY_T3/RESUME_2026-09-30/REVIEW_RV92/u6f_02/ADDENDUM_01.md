# RV92 addendum 01: N-9 against I66's scope-sentence round

RV92 (TASK, Type 2, under ROOT) checked I66's scope-sentence round, as ROOT asked. The round is uncommitted in `WT/f2a-carriers` on `b10ee5cf08`, and I66's records are `R/I66/u6_postf_02/`.

## Verdict: PASS

The new sentence declares N-9 (a) and (b) truthfully and in full; every claim in it matches the actual Rust, TS and Python behaviour. The three assertions pin its presence, and no outcome changed. There are two optional notes, and nothing needs fixing.

## The diff

`git diff` shows 4 files, one line each, with sha256 prefixes matching ROOT's:

| File | sha256 prefix |
|---|---|
| case file | `bbc05bd2` |
| `retained_precision_carriers.rs` | `3655d7db` |
| `test_retained_precision_carriers.py` | `21d7042b` |
| `retainedPrecisionIntegration.test.tsx` | `b02d7128` |

In the case file, only the `scope` key changed (checked against `b10ee5cf08`); the cases and the five entries are unchanged. RV92 copied the 4 files into a clone of its `b10ee5cf08` lane (`WT/rv92/cand3`).

## Each claim against the source and behaviour

**"Rust's header dispatch checks only that a preview `contract_evidence` is an object."** True.
- The preview branch of `for_source_metadata` (semantic_contract.rs, `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED`) is its only `contract_evidence` check.
- Rust's successor transport runs it on the reader's projection (retained_precision.rs:4331–4337).

**"TS's transport route, like Python's transport check, refuses evidence content that Rust's transport accepts."** True.
- On the 2 probes (`contract_evidence_edit_resealed`, both modes):
  - Rust: `ok`;
  - TS's `sourceContractTransport`: refused;
  - Python: refused (F-U6b-2's `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`).
- At base, on preview-physics-1, Python's and TS's transport checks refuse the same edit ("transport evidence shape") while Rust admits it (u6f_02 `transport/base_ts_preview_transport.json`).

**"…which TS refuses at the reader's G7."** True. RV92's probe (`addendum_01/ts_scope_check.json`) shows that TS's route rejects with a `RetainedPrecisionError` of gate **G7** and code `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, in both modes.

**"A transport refused by a header check before the reader runs carries each language's own code."** True on all 28 probes. TS's route rejects at the header, and the reader is not reached (the error is not a reader error):

| Language | Code |
|---|---|
| TS | `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED` (28/28) |
| Rust | the reader's G0 code `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (24); its base header code `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED` (2) or `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` (2) |
| Python | F-U6b-2's `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (28/28) |

The TS reader's own transport validator gives gate G0 on the same 24, so Rust's 24 are indeed G0 codes.

**Sufficiency.** The sentence covers exactly the 30 transport probes N-9 listed (2 + 28). In RV92's 204-probe table, every other difference was already one of the five entries or in the earlier scope list. So with this sentence, **every difference RV92 observed is declared.** Item 3 of u6f_02 is now confirmed.

## The pins

The Rust, Python and TS tests each assert that the scope contains "parity there compares only accept against refuse", beside the existing G7 phrase; TS's regex also requires the order. They pin the sentence's presence. TS's behaviour is pinned separately by I67's test of a G7 refusal on a receipt-consistent, base-inconsistent transport.

## No other outcome changed (read-only reruns, both lanes)

The three affected test files, plus the case file's two other TS consumers:

| Suite | `b10ee5cf08` | With the round |
|---|---|---|
| Rust `--test retained_precision_carriers` | 14/14 | 14/14, identical per test |
| Python `test_retained_precision_carriers.py` + `test_retained_precision_schema.py` | 78 passed | 78 passed, identical per test |
| Vitest: `retainedPrecisionIntegration`, `retainedPrecisionOutputRefusal`, `retainedPrecisionAnalysisRun` | 224/224 | 224/224, same titles, 0 outcome changes |

The case file is read by nothing else (git grep). Cargo ran one job at a time (`--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`), with the memory guard (PID 5387) checked. There were no installs, no Git writes, and nothing native or DEC-025. The review probe file was used only in RV92's copy and removed after.

## Optional notes (no action needed)

- **O-1:** the pins check the concluding phrase only. A later edit to the example or the code list would not fail them. Pinning `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED` in the scope text would close that.
- **O-2:** Rust reaches its "base header code" after the reader's G0–G2, on the reader's projection (labelled G2 by Rust's transport), not before the reader. The sentence attributes "before the reader runs" to the refusing header check, which is TS's, so it is accurate. A reader could misread it, though.

**When:** 2026-10-04, about 15:00Z to 15:08Z.
