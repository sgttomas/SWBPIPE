# RV113: my D38 audit of TS, written before reading I92's RETURN

Reader: TS = `P/apps/desktop/src/features/results/retainedPrecision.ts` at the head `7e47e51b5d` (functions by name).
Question (DESIGN_v2 §2): every TS check that assumes a prepared source has a Call or a Run; relaxed to (4b), or shown not to apply.

`validateRetainedPrecision` order: header (G0), integrity (G1), conversionEncoding (G2), coverage (G3), diagnostics (G4),
G5 = nativeRuns (class 1, `nativeClass`), ordinaryAttempts, productAttempts; G5a numericalCases, numericSummaries,
unselectedCoverage; G5b; G5c; G6; G7; G8 `invocationBinding`.

## Checks that assume a Run (or Call)

| # | Where | Check | (4b) disposition |
|---|---|---|---|
| T1 | productAttempts | `(native === 'not_entered') === (run_ref === null)` (I1) | **Relaxed**: native `failed` with `run_ref` null must satisfy `d38CaptureBeforeRun`; native `completed` still needs a Run |
| T2 | productAttempts | `(run_ref === null) === !c.run`; with a Run, its id, origin source and owner | Holds: both null ((4b)'s first conjunct) |
| T3 | productAttempts | `run_ref` non-null ⇒ native completed iff the Run is selected | Does not apply |
| T4 | productAttempts | `if (s)`: owner, basis, `preparation.attempt_ref`, **`c.source_ref === a.source_ref`** | No Run assumption: TS already binds the case's source for every sourced attempt (DESIGN §2: "TS already requires it"), so (4b)'s equality is redundant in TS |
| T5 | productAttempts | proof ⇔ `proof_start` entered; `proof_start` entered ⇒ native completed | Does not apply (proof null) |
| T6 | productAttempts | coverage non-null ⇒ own selected Run | Does not apply |
| T7 | productAttempts | Ready ⇒ selected Run | Does not apply |
| T8 | productAttempts (reason table) | `native` needs the case's non-selected Run; `capture` without a Run gives (`source_unavailable`, `preparation`) | Already admits a capture without a Run |
| T9 | nativeClass | calls' `run_refs` are `0..runs`; each position binds its Run, case, source and owner; group sources ⊆ the call's and equal its Runs' sources; C5 partition; Builds; meter and `charged` | No source needs a Call; these enforce (4b)'s "nothing names it" clause |
| T10 | coverage (G3) | Run ids `0..n`; `execution_order` = the Runs' owners | Enforces the `execution_order` clause |
| T11 | unselectedCoverage (G5a) | an unavailable attempt with a coverage array reads `c.run` | Does not apply (proof null) |
| T12 | numericalCases, selectedCoverage | selected cases' Runs | Do not apply |
| T13 | ordinaryAttempts (C2 branches) | an unavailable case whose cause is not `prepared_product_failure`: the kernel-cause branch needs a Run | Does not apply ((4b)'s cause is `prepared_product_failure`) |

## Checks that touch a prepared source but assume no Call or Run

integrity (G1) preparation digests and selected identities; coverage's D22 member association, unsourced complete
coverage, D29 and the source owner; productAttempts: `if (s)` the members and `old_coverage`; native entered ⇒
preparation completed; preparation completed ⇔ a source; D19; D37 (`errorStageRecordAgrees` for `capture`); P9's
`allowed` (native: native or capture); invocationBinding's source loop (maps, layout, digests) and attempt loop
(sections against the source, `s.preparation.attempt_ref`); ordinaryAttempts O5 (source decline: no source, no Run);
eligibility.

## Redundancy of `d38CaptureBeforeRun`'s conjuncts in TS

All are repeated by later or earlier checks with the same gate and code, as in RS, **except** that in TS the
equality and the source binding are also checked **earlier** (T4), so they are redundant here too; the `native ===
'failed'` guard on the branch is **not** redundant in TS (no other check refuses a completed native stage with no Run
and a capture before proof_start: D37 admits that record). The m2 test pins it.
