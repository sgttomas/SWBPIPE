# I61 receipt experiment 02: mismatch ledger

**There are no reader mismatches and no contract tensions on the main line.**
- The milestone receipts (sparse and dense) emitted with T1 option (a) pass G0–G8 in all three draft readers at READER `abcb16fd27`, with standing `needs_recompute`.
- The request is the unchanged model-0.1.0 request (D31), and no counterfactual is applied.
- No item needed a contract reading.

## Resolved since experiment 01

| Id | Was | Now |
|---|---|---|
| T1 (G4) | `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` naming the retained-selected case | Emitted as the owner confirmed for option (a): omitted from the successor envelope and from `diagnostic_refs`. `legacy_source` is `{unavailable, null, work_ref:0}`; `legacy_source_work[0]` is `{case_index:0, stage:"source closure", helper_stage:"source_closure", charged:46628, rejected:0, limit:4000000, settlement:"booked"}`. G4, O4 and D6d pass in all three readers |
| T2 (G8) | Readers admitted only model 0.2.0/0.3.0 | D31 admits 0.1.0. G8 passes with the request unchanged at 0.1.0 |
| E1 | `absolute_verified` taken from native evidence | Fixed in 01 and carried over; G5c passes |

## Recorded limit (decided)

- **T3.** The refusal receipt (an actual preparation-refusal path, with an injected trigger) cannot be a successor; that is wider F2a. It is kept as a G0–G2 check: 0 schema violations, and G0–G2 pass in all three readers. Each reader's first failure is G3 `COVERAGE_MISMATCH`, as the contract requires. As an unselected case it keeps its legacy disclosure (`diagnostic_ref` names it), and `work_ref` points to its `legacy_source_work` entry.

## Producer gaps (no reader failure; inputs to the real serializer brief)

Unchanged from experiment 01:
- **G-a.** The envelope transformation and the `RETAINED_PRECISION_*` diagnostic text.
- **G-b.** The Ordinary members `initial`, `w2` and `formation` are read back from the envelope.
- **G-c.** D6a attribution of invocation-level diagnostics (A2).
- **G-d.** `support_indices` are derived by the emitter.
- **G-e.** `not_covered` has no producer class.
- **G-f.** The fixtures exist only in READER (A4).
- **G-g.** `retained_state_sha256` (A1) is not checked by any reader.
- **G-h.** C1 §2's no-wrap premise is not established.
- **G-i.** The closed translations for unreached error paths are unimplemented.
- **G-j.** Indexing is tested for one case only.
- **G-k.** The owner binding is by custody only (RV77-N4).

New:
- **G-l. The legacy WorkReport is untyped in the producer.**
  - **What it affects:** T1 (a) requires `legacy_source_work[k]` from the actual WorkReport.
  - **The gap:** the private driver does not capture the typed `RecoveryFailure` (stage, helper_stage, WorkReport). The values exist in PP/lib.rs at the attempt's `Err` arm (:3747–3760), where `failure.work.charged` is debited and the diagnostic is formatted.
  - **The workaround:** the emitter reads the values back from the actual diagnostic's message. C2:160–162 forbids that for the real serializer ("does not … reinterpret Debug error text"; "WorkReport contents and settlement are copied from actual source accounting").
  - **What the serializer must do:** capture the typed `RecoveryFailure` and its WorkReport at that arm. The same capture also gives `legacy_source` its typed basis, closing part of G-b.
