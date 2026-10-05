# I67 return: U7 slice T (TypeScript pre-flip)

I67 is a TASK (Type 2) under ROOT, working to ROOT's slice T message and its slice A addition. The basis is I61's plan `R/I61/u7_scoping_01/PLAN.md` §1–4 and §2 rows 5–7, and RR "RV93 on U3 grant 2: PASS; RV89 confirms the final basis re-qualified; U7 planned and ruled" (D-U7-1 to D-U7-6). I also read I61's `u7_slice_a_01/RETURN.md` §1.4. I did not delegate.

**Verdict: all three preconditions, and slice A's added item, are done in TS. Every control passes.**
- **The flags do not move.** `retainedPrecision.ts` is unchanged.
- **Suite:** Vitest gives 3,531/3,531, and `tsc` is clean.
- **Existing outcomes:** every one of the 3,494 base tests keeps its outcome.
- **Unforced paths are unchanged** (the 80-envelope sweep is identical), apart from one intended display change: the stress-neutral panel's text for a successor (N-5).
- **Mutants:** 136 of 136 are killed, all by assertion.
- **D-U7-4's declared-difference entry is drafted,** not applied. Two small rulings are listed at the end.

## Basis, host and fence

- **Worktree:** `WT/f2a-u7`, at `071eec5c04` on `codex/piping-f2a-u7-20261004`. The work is uncommitted, and I made no Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`. I61 is reading it for slice A and made no edits.
- **When:** 2026-10-04, about 16:45Z to 17:19Z. The memory guard (PID 5387) ran throughout.
- **Runtime** (`_run_records/runtime.txt`):
  - I created an untracked symlink `P/node_modules` → `REPO_ROOT/P/node_modules`. It shows as `??`; never commit it.
  - The prebuilt WASM was **copied** from `WT/f2a-readers` into the worktree's ignored `apps/desktop/public/`, with hashes recorded. Nothing was built.
  - Only Vitest and `tsc` were run: no installs, no Cargo, no Python, nothing native or DEC-025. `TMPDIR` was set to `WT/scratch/i67_u6d/tmp`.
- **Lanes:**
  - `base5` is a `git archive` of `071eec5c04`;
  - `mut5` is the candidate. It differs from the final worktree only by the slice A notice test, which was added after the mutant run started (`lane_mut5_vs_worktree.txt`). The product files are identical.
- **The fence:** `apps/desktop/src/**` only, 7 files. The fixtures, the case file, core (Rust and Python), tests, schemas and the TS reader's flag are untouched (`out_of_fence_unchanged.txt`).

| File (TS = P/apps/desktop/src) | sha256 | Change |
|---|---|---|
| TS/features/results/retainedPrecisionStanding.ts | `fe196253…` | N-2: the live-capture binding |
| TS/services/previewService.ts | `8de1b6a8…` | N-2: passes the native capture's liveness at registration |
| TS/features/results/numericalResultQuality.ts | `af72a4d5…` | N-8: the pinned `RETAINED_STANDING_STATUS` (token to status) |
| TS/features/result-export/ResultExportPanel.tsx | `17f7a686…` | N-5: the explicit gate |
| TS/features/stress-neutral/StressNeutralExportPanel.tsx | `fae61807…` | N-5: the explicit gate and its refusal display; `liveStressBinding` exported for its test |
| TS/features/results/retainedPrecisionIntegration.test.tsx | `26783147…` | 37 new tests; the harness's standing model; T6 knobs |
| TS/features/results/retainedPrecisionOutputRefusal.test.tsx | `5ca651f0…` | The stress-neutral display expectation (the intended N-5 change) |

## 1. RV91 N-2 = RV88 U6d S-1: standing bound to the live native capture and the current model

**The change.**
- **Registration:** `registerRetainedPrecision(source, invocation, live?)` stores a liveness predicate.
  - `previewService.validateCapturedSource` passes `model => hasNativeMechanicsInvocation(source, model)`: an unchanged, not invalidated capture; unchanged source bytes; and a model equal to the captured one.
  - That is the same check every eligible-standing consumer already makes.
  - Passing a closure keeps `features/results` free of an import of `services/previewService`.
- **Standing:** `retainedPrecisionStanding` stays as it was through D2 §4.9.4. When the statement *would* be eligible, it then requires `live(model) === true`. Otherwise it gives `needs_recompute` with the new finding `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED`.
- **Unforced outcomes are identical by construction,** because the check runs only on the would-be-eligible branch. A registration without a capture, which no product path makes, can never stand eligible.

**Tests,** with eligibility forced through the test-only reader wrapper (`u7.simulate`), in both modes:
- the captured model reads `numerically_eligible`;
- **another model with the same case ids** (a node moved) reads `needs_recompute`, `…NATIVE_CAPTURE_REQUIRED`;
- so does a model carrying only the case ids, and `retainedPrecisionInvocation` gives `null`;
- **an edit of the caller's model object after capture** voids it, for that model and for the original;
- **a job cancelled while the reader validates:** the reader's validation is recorded, but the native registration is not, so it reads `…NATIVE_CAPTURE_REQUIRED`;
- **a direct registration with no capture** is never eligible;
- **with eligibility held,** all of these read exactly as before (`…NOT_NUMERICALLY_ELIGIBLE`).

**The harness:** the shared-case harness now gives standing the **captured model** when a capture exists and the requested refs are its cases, as the session would. Before the flip that changes no outcome. RV03 and RV04's seam test passes a live predicate.

### D-U7-4's declared-difference entry (drafted, for slice F)

The draft is `_run_records/declared_difference_D-U7-4.draft.json`.
- **Id:** `D-U7-4:ts_requires_live_native_capture`.
- **Its ruling cites:** RR "RV93 …" (D-U7-4), RV91 N-2, RV88 U6d S-1, PLAN §2 row 5, and this return.
- **Its two forms, both milestones:**
  - `invocation_without_native_capture`;
  - `stale_current_model_same_case_ids`.
- **Expectations:** Rust and Python read `numerically_eligible` after U7; TS reads `needs_recompute` with `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED`.
- **Before the flip,** every language reads `needs_recompute` on both forms, so slice F applies it with the flags.
- **Format v3 cannot express either form.** The draft proposes one field each:
  - **`"capture": "none"`:** TS registers the invocation without an IPC capture;
  - **`"current_model_edits"`:** edits applied only to TS's current model.
  
  That is a format question for slice F (I66).

## 2. RV91 N-5: explicit panel gates

**The change.**
- **The gate:** `ResultExportPanel`'s `liveResultBinding` and `StressNeutralExportPanel`'s `liveStressBinding` now gate on the exported `loadReferenceOutputRefusal(result) !== null`, in place of `isLoadReferenceRoute`. For load/reference-state routes the outcome is the same, and it now also covers the successor.
- **The stress-neutral display** shows the shared refusal (`RETAINED_PRECISION_OUTPUT_REFUSAL` for a successor) where it showed the generic empty text. This is the plan's intended change. For load/reference-state it is the same text as before.
- **The export:** `liveStressBinding` is exported only so its gate can be tested directly. Its packet builder refuses a successor on its own (`SN-PRECISION-CONTRACT-MISMATCH`: the header cannot carry the receipt), so the panel's display alone cannot reveal the gate.
- No other T6 file is edited. **ROOT posts the T6 notice.**

**Tests,** with eligibility forced, a native registration made with the pinned model, and the builders allowed to proceed (test-only knobs `t6.throwless` and `t6.builderDoc`), in both modes:
- **Result export:**
  - no packet is offered even when its builder returns a document;
  - **positive control:** with the shared refusal removed (`t6.noRefusal`), the packet is offered (`result-export-summary` shows "available").
- **Stress-neutral:**
  - `liveStressBinding` is `null`, and the panel shows the shared refusal, not the empty text;
  - **positive control:** with the refusal removed, the binding is open.
- **The existing output-refusal test** now expects the refusal text for a successor (`retainedPrecisionOutputRefusal.test.tsx`).

## 3. RV92 N-8: the token pin

**The change.**
- `numericalResultQuality.ts` now has a frozen, exported `RETAINED_STANDING_STATUS`:

  | Token | TS status |
  |---|---|
  | `numerically_eligible` | `integrity_checked` |
  | `needs_recompute` | `needs_recompute` |
  | `unsupported` | `needs_recompute` |

- The successor branch maps its token through that table. The value is equal to the previous `eligible ? … : …`.

**Tests:**
- the table is pinned separately (exact and frozen);
- **all 20 shared cases are rerun with eligibility forced, comparing the carrier token:**
  - exactly `sparse_interactive:invocation` and `dense_scrutiny:invocation` become `numerically_eligible`. That expected set is written out, not derived;
  - the other 18 keep the file's token;
  - each status equals the table's;
- **the declared-differences consumer** now compares TS's status through the table, not against the token string.

## 4. Slice A's item: the notices' `classificationSummary(source)` without the model

**Decision: it does not pass the model, by design.**
- `knownSemanticNotices` shows only each case's `absolute_verified` and `not_covered` counts. Those are the reader's G5c classes and do not depend on standing.
- The refusals the notices state are per-quantity, and hold whether or not the envelope is Current, since S-I has not landed.
- The one model-dependent field, `withheld`, is not displayed.
- Passing a model would also mean widening `knownSemanticNotices`'s signature and every `KnownSemanticNotices` caller, with no change in any shown text.

**A test pins it** (both modes):
- an eligible successor (forced) shows exactly the notices a held one does;
- `classificationSummary` with and without the model differs only in `withheld` (69 against 97).

**Not applied, as instructed:** I61's D-U7-6 sentence for the standing text's `tail` is slice F's.

## Controls

**Suites:**

| Run | Result |
|---|---|
| Base `071eec5c04` (lane) | 3,494/3,494; `tsc` 0 |
| Candidate (worktree, final) | **3,531/3,531**; `tsc` 0 |

**Per test** (`compare_base_vs_candidate.txt`): **all 3,494 base tests keep their outcome,** with no title changed. The 37 new tests are all in the integration file:
- N-2: 10;
- N-5: 4;
- the token pin: 21;
- the notices: 2.

**The unforced-path sweep** (base `071eec5c04` against the candidate's product files; `sweep_compare.txt`): **all 80 envelopes are identical,** the 63 existing-identity ones and the 17 successors. The sweep covers route, standing with and without a model, notices, labels, binding, refusals, the report and the AnalysisRun builds.

**Mutants** (`mutants_r5.py`, `.json`, `.log`; the mutant lane against the 3 U6d test files and 8 related ones; the control passes 396/396):
- **136 of 136 killed, all by assertion.**
- That is the U6d set re-pointed to this basis: 123, with N17, P01 and P02 rewritten for the changed lines.
- **Plus 13 new:**
  - **binding:**
    - **B01:** the live check removed;
    - **B02:** presence instead of liveness;
    - **B03:** previewService passes no capture;
    - **B04:** an always-live predicate;
    - **B05:** liveness checked before eligibility (it would change unforced outcomes);
  - **gate:**
    - **G01:** the result-export gate removed;
    - **G02:** the stress-neutral gate removed;
    - **G03:** the refusal not displayed;
  - **status:**
    - **T01s:** eligible mapped to `needs_recompute`;
    - **T02s:** the token reported as the status;
    - **T03s:** an `unsupported` status invented;
  - **comparing by status string,** in the test harnesses:
    - **Q01:** the forced-eligibility parity;
    - **Q02:** the shared parity.
- The harness mutants were first labelled H01 and H02. That clashed with the reopen mutants, so they were relabelled Q01 and Q02 after the run; the log notes it.

## For ROOT to rule

1. **The D-U7-4 entry's format.** Both forms need a field that format v3 lacks: `capture: "none"` and `current_model_edits`. Slice F (I66) adopts them in the next format, or proposes another expression.
2. **`liveStressBinding` is now exported** from a T6 panel module, for its gate's test. It is the smallest observable seam, because the packet builder refuses a successor on its own. ROOT may prefer a different seam.

**Also new names, all absent from committed trees** (NUM HEAD, `origin/main`, the U7 branch and the memory branch): `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED`, `LiveNativeCapture` and `RETAINED_STANDING_STATUS`.

## Records

`_run_records/` holds:
- basis and runtime;
- the diff and the changed-file hashes;
- the out-of-fence check;
- the D-U7-4 draft;
- the run scripts;
- base and candidate outcomes, exit codes and `tsc`;
- the outcome comparison;
- the sweep and its comparison;
- the lane note;
- the mutant programme, results and log.

All paths in the records are placeholders. SHA256SUMS covers this folder.
