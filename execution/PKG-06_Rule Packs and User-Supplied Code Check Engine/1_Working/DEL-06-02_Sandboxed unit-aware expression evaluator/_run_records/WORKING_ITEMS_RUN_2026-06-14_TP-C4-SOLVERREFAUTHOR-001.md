---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C4-SOLVERREFAUTHOR-001
timestamp: 2026-06-14T23:05:00-0600
completed: 2026-06-15T00:05:00-0600
run-status: SUCCESS_PUSH_PENDING
closeout: COMMITTED_NOT_PUSHED — DEC-025 gate blocked on external machine load (see Closeout status)
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C4-SOLVERREFAUTHOR-001 — author `solver_result_ref` in the rule-pack declarations form-builder (C4 residual, authoring slice)

## Tranche and authority basis

- Tranche: the completion-plan **C4** residual named first in "Remaining scope
  (non-GUI follow-ups)" — "GUI authoring of `solver_result_ref` in the C2
  declarations form-builder (mirrors `TP-C3-LIBREFAUTHOR-001`)." It is the
  **authoring half** of the solver-result binding whose **resolution half**
  landed in `TP-C4-SOLVERREF-001` (TP-MAC-164): the backend already resolves a
  `solver_result` required input's authored reference from the solved result
  envelope at run time, but the reference could only be set by hand-editing raw
  document JSON. This slice replaces that hand-editing with a structured form
  control, mirroring `TP-C3-LIBREFAUTHOR-001` (`library_value_ref`).
- Selection: earliest unblocked item on the Phase C / R3 dependency spine
  (`DEC-035` target stage). Unblocked — the optional `solver_result_ref` schema
  member and its run-time resolution already landed (`TP-C4-SOLVERREF-001`) and
  were **ratified** at rule-pack `schema_version` 0.4.0 (`DEC-039`,
  `TP-C4-RATIFY-001`); the Value-based `validate_rule_pack_document` tolerates
  the member (read by JSON pointer, no `deny_unknown_fields`); **D-02b** (`DEC-037`
  Option O-C, RULED) gates only writable expression *text* syntax, not structured
  form authoring — this is form-builder work exactly like the rest of
  `DeclarationsEditor`.
- Scope: **frontend-only**, one component + its two test surfaces. No schema,
  Rust, or Python change. Write scope: `DeclarationsEditor.tsx`,
  `DeclarationsEditor.test.tsx`, `e2e/r2-smoke.spec.ts`, `apps/desktop/SMOKE.md`,
  this run record, and the completion-plan C4 row + completion log.

## Design decisions

- **Seed a complete reference on entry, mirroring the library pattern.**
  `SolverResultRef` requires a single member `result_id`. When the user sets
  `source_kind` to `solver_result` and no reference exists, the editor seeds a
  complete reference (`result_id` → the visible uppercase `"TBD"` placeholder the
  `Id` pattern accepts), so the document is always schema-valid and an unfilled
  reference matches no result row, leaving the input unsupplied so the check
  **blocks** at `RULE_INPUTS_INCOMPLETE` — never a partial shape, never a silent
  pass (CONTRACT no-silent-defaults; the backend treats `"TBD"` as a non-matching
  id, same as the empty case).
- **Supersede semantics surfaced, not hidden.** The backend
  (`resolve_authored_solver_result_bindings`) makes an authored reference the
  **canonical** binding for its input: it supersedes the legacy run-panel
  caller-supplied `{input_id, result_id}` selector and governs the input alone,
  even when unresolved (PRD §12.5). Because `solver_result` — unlike
  `private_library_value` — has a real run-panel fallback, the sub-form note
  states the supersede explicitly and offers a **"Remove solver-result
  reference"** control that returns the input to the run-panel binding path. The
  default (seed on entry) favors in-pack authoring; Remove is the visible
  escape hatch to the run-time selector. Both workflows remain reachable; the
  choice is never silent.
- **First-edit completion guard.** `updateInputSolverRef` merges into the
  existing reference *or a fresh default* when none exists, so even an opened
  pack with `source_kind: solver_result` but no reference (authored before this
  slice, or relying on the run-panel binding) yields a complete reference on the
  first field edit — never an empty object.
- **Never silently hidden; explicitly removable; independent of the library
  ref.** The sub-form shows when `source_kind === solver_result` **or** a
  reference is present, so a reference left after the source_kind is changed away
  stays visible and removable. Seeding a solver reference does not also seed a
  library reference (the two reference kinds are independent; switching source
  kinds leaves any other-kind reference in place, lossless).

## Changes

### `apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx`
- New exported `defaultSolverResultRef()` helper (`{ result_id: "TBD" }`).
- Generalized `changeInputSourceKind` to seed the matching reference on switch
  to either `private_library_value` (library ref) or `solver_result` (solver
  ref); added `updateInputSolverRef` (lossless merge over the existing ref or a
  fresh default) and `removeInputSolverRef` (drops the member).
- Required-input row: a `solver_result_ref` sub-form (`result_id` text field +
  supersede/boundary note + remove control) gated on
  `source_kind === solver_result || ref present`, rendered after the library
  sub-form.
- Component doc-comment updated to record the slice, the supersede semantics, and
  the now-ratified status of both reference members (`DEC-038` /
  `DEC-039`, `schema_version` 0.4.0). Corrected the stale "library_value_ref …
  awaiting ratification" note left from the pre-ratification slice.

### `apps/desktop/src/features/rule-packs/DeclarationsEditor.test.tsx` (+5)
- `defaultSolverResultRef()` exact-shape pin (`{ result_id: "TBD" }`).
- Component: seed-on-switch-to-`solver_result` (complete ref + sub-form + note +
  no library-ref seeded); lossless `result_id` edit keeping the rest of the input
  verbatim; first-edit completion when an opened input lacks a ref; stale
  reference visible + removable, removal preserving siblings and hiding the
  sub-form.

### `apps/desktop/e2e/r2-smoke.spec.ts`
- Extended the existing rule-pack manager draft flow: add an input, set its
  `source_kind` to `solver_result`, assert the sub-form + supersede note, fill
  `result_id`, and assert a complete `solver_result_ref` in the canonical draft
  JSON. Real-browser default UI evidence per the H4 amendment.

## Evidence

Per-surface evidence (all green):

- `npm test --workspace apps/desktop` (Vitest): **372/372 pass** (+5; was 367 at
  the prior tranche head — DeclarationsEditor 21 → 26). Green standalone (107s)
  and inside sweep attempt 3 (with reduced fork parallelism, 372/372).
- `npm run build --workspace apps/desktop`: clean (tsc typecheck + vite;
  pre-existing chunk-size advisory only).
- `npx playwright test` (rule-pack manager journey): **2/2** targeted (dev
  variant, two viewports) and **10/10** inside sweep attempt 3 (the rule-pack
  manager test at 6.3/6.5s carries the new authoring assertions).
- cargo crate sweep: **ok** (28 crates, all attempts). pytest: **359** (all
  attempts).
- No Rust/Python/schema surfaces touched (frontend-only slice); the optional
  member and its resolution were proven end-to-end and ratified in
  `TP-C4-SOLVERREF-001` / `TP-C4-RATIFY-001`.

### Five-surface DEC-025 gate — BLOCKED ON EXTERNAL MACHINE LOAD (push pending)

The combined commit-bound five-surface sweep did **not** produce a single
all-green run this session. Across three `tools/release/run_evidence_sweep.py
--execute` attempts at HEAD `2285e1dc8`, every actual test/assertion passed, but
**rotating infrastructure timeouts** failed the sweep — all attributable to
sustained external machine load (a concurrent coding agent + git + file sync;
`uptime` load average oscillating **6–21 on 8 cores**, never settling):

1. **Attempt 1** (default parallelism): `desktop_vitest` failed — 3 `App.test.tsx`
   tests hit their explicit per-test timeouts (`}, 10000)` / `}, 15000)`) under
   load; the file took **292s** (vs. 62s standalone). The 3 tests
   (`App.test.tsx:2924/4131/4562` — report-packet, save/open round-trip,
   mechanics diagnostics) are unrelated to this rule-pack slice and pass **52/52
   standalone**. (`SWEEP_20260615T053315Z` — discarded.)
2. **Attempt 2** (default parallelism): identical `desktop_vitest` failure (same
   3 timeouts; `App.test.tsx` 263s). (`SWEEP_20260615T054117Z` — discarded.)
3. **Attempt 3** (`VITEST_MAX_FORKS=3` — a non-invasive runtime env tweak, no
   committed-file change, identical test set/assertions): `desktop_vitest`
   **372/372 PASS**; Playwright ran **10/10 tests PASS** — then failed at
   `desktop_playwright_e2e` because the Playwright worker processes "did not exit
   within 300000ms after stop, force-killed" (a cleanup-phase teardown hang under
   load, *after* all tests passed — "3 errors were not a part of any test").
   `desktop_production_build` recorded `not_run` (gated behind playwright).
   (`SWEEP_20260615T055704Z` — discarded.)

Diagnosis: a real validation/infrastructure blocker caused by external-scope
machine contention (coordination doc: external load is "external-scope noise").
No code defect in this tranche; no test or assertion failed in any attempt. The
only code-level "fix" (bumping the `App.test.tsx` per-test timeout literals)
is out-of-scope (DEL-07/DEL-14, against the documented `TP-C3-LIBREFAUTHOR-001`
hand-off precedent) and would not clear the separate Playwright teardown hang.

## Closeout status — COMMITTED, NOT PUSHED

Tranche committed as the `TP-C4-SOLVERREFAUTHOR-001` commit (7 files; the local
`main` HEAD at session end). **Not pushed**: the commit-bound DEC-025 gate has
not produced an all-green run (above). Per the coordination
loop step 9 — push "unless ... a real validation/git blocker prevents it" — git
closeout is paused pending one of: (a) a quiet-machine sweep re-run (recommended;
attempt 3 was one teardown-hang away from green), or (b) a human ruling to accept
the per-surface evidence and authorize push despite the externally-induced infra
flake. No transient FAIL sweep summary is committed (per precedent).

## Residuals and hand-offs

- **C4 run-panel resolution preview for `solver_result_ref`** — the second of
  C4's two named non-GUI follow-ups: a run-panel preview of whether each
  `solver_result` input's authored reference resolves against the solved
  envelope (mirrors `TP-C3-LIBREFPICKER-001`). Not in this slice's scope.
- **Run-time override of an authored reference** — deliberately a non-goal
  (carried from the `library_value_ref` ruling): an authored reference governs
  its input alone; the run panel cannot override it. No change here.

## Boundary compliance

Local-only (no network/daemon/telemetry); the reference is authored into the
local draft and no solver result value is embedded or committed.
Status-vocabulary-only; no compliance/certification/sealing/authentication/
approval/code-compliance or professional-acceptance claim. Deliverables stay
`CHECKING`. Git/test evidence is source-control hygiene only.

## Open decisions awaiting human ruling

- None introduced by this slice. The members it authors are ratified
  (`DEC-039`); **D-02b** is RULED (`DEC-037`, AST-only) and unrelated to this
  structured-form slice. The standing not-yet-prepared decision packets are
  unchanged: `D-06`, `D-11`, `D-12` (and the deferred `D-04b`, `D-05b`, `D-07b`,
  `D-10b`).
