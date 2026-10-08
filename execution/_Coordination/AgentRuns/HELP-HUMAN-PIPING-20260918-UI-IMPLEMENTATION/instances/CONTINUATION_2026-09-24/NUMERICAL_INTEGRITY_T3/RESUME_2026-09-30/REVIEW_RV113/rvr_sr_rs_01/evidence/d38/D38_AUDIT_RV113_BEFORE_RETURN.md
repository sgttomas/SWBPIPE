# RV113: my D38 audit of RS, written before reading I90's RETURN

Reader: RS = `RE/src/retained_precision.rs` at the head `cc81e78801` (functions cited by name).
Question (DESIGN_v2 §2, B1's obligation): every RS check that assumes a prepared source
has a Call or a Run; relaxed to (4b), or shown not to apply.

`validate` order: g0, g1, g2, (integral_receipt), g3, g4, g5_native, g5_ordinary,
g5_products, numeric_cases, g5a, g5b, g5c, G6 (row method), G7 (base), g8.

## Checks that assume a Run (or Call) for a prepared source

| # | Where | Check | (4b) disposition |
|---|---|---|---|
| A1 | g5_products | native entered ⇒ `run_ref` non-null and preparation completed | **Relaxed**: native `failed` with `run_ref` null goes to `d38_capture_before_run` (the one behavioural relaxation) |
| A2 | g5_products | `run_ref` non-null ⇒ `case.run.id == run_ref` and `a.source_ref == case.source_ref` (the case's source bound only through a Run) | **Relaxed by addition**: (4b) carries the equality itself (m8) |
| A3 | g5_products | `run_ref` null ⇒ `case.run` null | Applies unchanged; it is (4b)'s first conjunct |
| A4 | g5_products | native `completed` ⇒ the case's Run is selected; `(native == completed) == (Run selected)` | Holds for (4b) (both sides false); m2 refused here |
| A5 | g5_products | proof non-null ⇒ `run_ref` non-null and the Run selected | Does not apply: (4b) has proof null (refused otherwise) |
| A6 | g5_coverage | a complete coverage vector ⇒ the attempt's own source and Run | Does not apply: proof null ⇒ no coverage |
| A7 | reason_table | `native` error ⇒ the case's own non-selected Run; `capture` with no Run ⇒ (source_unavailable, preparation) | Already admits a capture with no Run (D4d); m1 refused here |
| A8 | g5_native (calls, groups, C5/N11, builds, run order, charged) | every Call position is bound to a Run of its own case and that case's source; every Group source is in its Call's sources; every Build is bound to its building record; `charged` is the Runs' debits | No check requires every source to have a Call: a source with no Run is simply absent. This is what enforces (4b)'s last conjunct structurally (m7 refused at G5 ATTEMPT; leftover Builds or stale `charged` at G5 WORK) |
| A9 | g3 | Run ids are `0..n` and `execution_order` equals the Runs' owners in id order | Applies; a case with no Run is absent (m4 refused at G3) |
| A10 | g5a | an unavailable case with a non-null coverage vector reads its Run's attempts (`coverage_g5a`) | Does not apply: proof null ⇒ coverage null ⇒ skipped |
| A11 | numeric_cases / g5a_selected / g5b / g5c | selected cases' Run and source | Does not apply: (4b) is unavailable |

## Checks that touch a prepared source but assume no Call or Run

- g1: a source's preparation digest binds its attempt (no Run); `source_identity_sha256` only when present.
- g3: the attempt's old members equal its source's member map; D29 non-empty body inventory; summary coverage (null here).
- g5_stages: preparation completed ⇔ a source; the not-entered tail.
- error_stages: `capture` ⇐ native failed first.
- g8: every source against the invocation (owner, maps, layout, K4SRC/K4STF digests via `verify_native_source_hashes`), and section terms per attempt — independent of any Call (DESIGN §2: G8 recomputes both digests from the binding).
- g5_ordinary: source_decline needs a null source and Run — not a (4b) shape.
- accounting_rules R3': a fault-bearing cause's owner defaults to the proof; with proof null a faulted cause is refused (WORK). Not a Call/Run assumption; the (4b) cause `CaptureError::Origin` has no fault spelling. A `CaptureError::accounting{event}` cause is refused by R1' for every attempt.

## (4b) conjuncts in `d38_capture_before_run` that a later check makes redundant at `validate`

result unavailable (later `ready` rules; D19), error kind capture (reason_table / error_stages), preparation completed (g5_stages), the not-entered tail and observables/G5a (proof-null block; g5_stages), case unavailable (D19 / `selected ⇒ ready`), cause kind and its attempt ref (D19; D4c), reason code and phase (reason_table), source non-null (g5_stages). All later failures are G5 PRODUCT_ATTEMPT_MISMATCH within the same attempt's iteration, so the first failure is unchanged. Not redundant: **source equality** (m8), checked nowhere else for a Run-less case.
