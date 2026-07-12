# NOTES — DEL-07-01 (3D viewport and centerline editor), R2 wave W3

Deliverable: PKG-07 / DEL-07-01. Ledger: `WAVES/W3/CLAIM_CONCORDANCE_DEL-07-01.csv`
(25 rows). Frozen tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

Run-level `NormativeSource` alias (addendum 12): native 2-digit requirement ids
are `DEL-07-01-REQ-01 .. -10`; ClaimIDs use the addendum-12 fixed 3-digit form
`DEL-07-01-REQ-001 .. -010` and carry the native id in `NormativeSource`.

## Central finding

DEL-07-01 is a setup-documentation deliverable whose four-document kit is still
written in setup-era future tense ("the future implementation shall ...",
"defines setup documentation for the future 3D viewport ... does not create or
modify GUI source files"), while a substantial app-layer viewport/editor slice
actually landed under separate sealed dispatch briefs (MEMORY.md 2026-05-02
onward): `apps/desktop/src/features/viewport/PipeViewport.tsx` (2049 lines;
create-node / connect-pipe-run / component-symbol intents, canvas node drafting,
endpoint picking, deformation overlay, unit selectors), `core/gui/viewport_editor`
Rust crate, `schemas/viewport_editor.schema.yaml`, `apps/desktop/src/App.test.tsx`,
`tests/test_viewport_editor_contract.py`, and the invented fixture
`fixtures/gui/invented/viewport_editor_session.json`. Per convention 1 the
requirement rows take substance dispositions (never STALE); the four kit
declared-state surfaces take `STALE_SETUP_SPECIFICATION` under widened
addendum 4.

## 1. Histograms (recount from the CSV)

Disposition histogram (25 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 20 |
| STALE_SETUP_SPECIFICATION | 4 |
| PARTIALLY_IMPLEMENTED | 1 |

ClaimType histogram (25 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 10 |
| ACCEPTANCE | 7 |
| EXCLUSION | 2 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 0 |
| IMPLEMENTED_UNMAPPED | 0 |

Row census rationale:
- 10 REQUIREMENT rows (re-verified against the frozen kit: `DEL-07-01-REQ-01..10`).
- 7 ACCEPTANCE rows at addendum-12 grain from the Specification Verification
  table `DEL-07-01-VER-01..07` — these are genuine setup-acceptance gates
  (kit existence, semantic artifacts, dependency-schema validity, enum
  canonicality, write-scope, boundary visibility, TBD preservation), not
  mere restatements of the product requirements.
- 2 EXCLUSION rows for the durable scope exclusions (adjacent PKG-07 surfaces;
  protected/proprietary data + professional approval). The setup-era exclusions
  in the same Scope paragraph ("implementation of React, Tauri, Three.js";
  "final selection of exact dependency versions") are overtaken by the
  implemented slice and are folded into DECL-001, not given their own rows.
- 6 DECLARED_STATE rows: the four-document kit + `_STATUS.md` + `MEMORY.md`.
  No deliverable-owned README exists in the folder (verified), so no README
  DECL row.
- 0 REMAINING_WORK rows: `_STATUS.md ## Remaining` carries only the seeded
  `(gated: D-41)` bootstrap item, recorded verbatim in DECL-005's
  `RecordedRemaining` and excluded from all residual/gate/selectability
  analysis (addendum 2). No other residual is homed in DEL-07-01's own
  `## Remaining`.
- 0 IMPLEMENTED_UNMAPPED rows: every material surface in DEL-07-01's orbit
  (SURF-210 schema, SURF-079 crate, SURF-153 fixtures, SURF-052 viewport
  feature) is already attributed to DEL-07-01 in `IMPLEMENTATION_SURFACES.csv`;
  none of R1's 8 `NONE_FOUND`-attributed surfaces are in this orbit.

## 2. Self-flagged rows

- **DEL-07-01-REQ-001** — item-6 forward-looking-gap split: I disposed this
  `PARTIALLY_IMPLEMENTED` at *per-entity-authoring-capability* grain. Nodes and
  straight pipe-runs are fully authored through the structured operation seam;
  bends have no dedicated authoring tool and component symbols are reference-only
  (dimensionless) placeholders. A reviewer could instead read the requirement at
  contract grain ("a viewport/editor exists across these entity families") and
  call it ALIGNED. I chose the stricter reading.
- **DEL-07-01-REQ-002** — the item-6 counterpart: I disposed this `ALIGNED` at
  contract grain because the requirement carries its *own* explicit deferral
  clause ("items beyond initial centerline editing MAY remain deferred"), and
  the deferred categories (reaction arrows, stress-ratio color maps) are exactly
  what remains deferred. Reviewer eyes welcome on the REQ-001/REQ-002 split.
- **DEL-07-01-REQ-006** — classification call: I classed the "no protected /
  proprietary data" requirement as `GOVERNANCE` (matching the sibling exemplar
  DEL-07-05-REQ-008), not `SECURITY`. Consequently I did **not** apply the
  W1 calibration-item-2 owner-gated SECURITY em-dash marker
  (`NONE_FOUND — sufficiency review deferred, owner-gated`): this is a negative
  IP-boundary constraint that is directly code-verifiable (TBD placeholders,
  no dimensional tables, schema asserts no protected data), not a deferred
  sufficiency review. AuthorityNeeded=NO. Flagging in case the reviewer prefers
  SECURITY-class encoding with the deferral marker.
- **DEL-07-01-REQ-009** — ALIGNED at contract grain, but the GUI e2e/browser
  smoke legs were never executed at the frozen SHA and are cited recorded-only
  with the `not re-executed at frozen SHA 551f84ef6` marker; only the unit
  (Vitest) and contract (pytest) legs are content-identical / re-executed.
  Confidence set MEDIUM for that reason.
- **DEL-07-01-DECL-003** (Guidance) — STALE call is softer here: the Guidance
  *principles/considerations/trade-offs* remain valid; only its setup-only /
  future declared framing is overtaken. Marked STALE_SETUP_SPECIFICATION for
  consistency with the other kit surfaces (Confidence MEDIUM), with the
  narrowed staleness scope stated in-row.
- **DEL-07-01-DECL-005** (`_STATUS.md`) — ALIGNED, but flagged: DEL-07-01's own
  `## Remaining` carries no non-bootstrap residual, while the viewport A3
  residual (broader canvas gestures / component-rigid authoring) and a
  PipeViewport unit-test backfill are recorded in the **adjacent** DEL-07-02
  `_STATUS.md ## Remaining`. The residual is recorded (homed adjacent), not
  omitted, so I did not force REMAINING_STATE_MISMATCH — but a reviewer may wish
  to confirm the cross-deliverable homing is intended.
- **DEL-07-01-DECL-006** (`MEMORY.md`) — ALIGNED-with-note under W2 calibration
  item 9. The undated "## Boundaries Preserved" block ("No Tauri/React/Vite app
  shell ... No Three.js runtime renderer, or Playwright rendering tests")
  contradicts the frozen slice, but is corrected **in the same file** by the
  dated 2026-06-06 "Viewport closure tranche" and later entries. The reading of
  those blocks as undated-current (item 9) vs part of the dated 2026-05-02
  session (addendum 1) is a judgment; both readings land at "not a staleness
  disposition," so the outcome is stable.

## 3. Evidence-execution log

Frozen-tree porcelain (`git -C <FROZEN> status --porcelain`) was **empty before
and after** every re-execution and throughout the run.

Re-executed side-effect-free at the frozen SHA (cwd outside the frozen tree,
`PYTHONDONTWRITEBYTECODE=1`, stdlib-only / no cache writes; porcelain empty
before and after each):
- `python3 tests/test_viewport_editor_contract.py` → **PASS** (schema +
  invented-fixture contract; confirms the `DiagnosticClass` / `CommandType`
  defs). Cited on REQ-006, REQ-007, REQ-008, REQ-009, EXC-002.
- `python3 tools/validation/validate_dependencies_schema.py <Dependencies.csv>`
  → **VALID** (29 columns, 18 data rows). Cited on ACC-003 (and subsumes
  ACC-004 enum canonicality).

Cited as recorded passes (NOT re-executed at frozen SHA), with addendum-10
content-identical qualifiers I independently verified: R1 `VERIFICATION_INDEX.csv`
records the DEC-025 evidence sweep
`validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` bound to
commit `e648462f1d0521e26df15d04a988391343018886` (confirmed by
`git merge-base --is-ancestor` to be an ancestor of the frozen SHA):
- RUST-01 `cargo test` `core/gui/viewport_editor` (6 unit tests), surface
  `cargo_crate_sweep=pass`.
- PY-76 `pytest` `tests/test_viewport_editor_contract.py` (1), surface
  `python_pytest=pass`.
- VT-02 Vitest `apps/desktop/src/App.test.tsx` (57), surface `desktop_vitest=pass`.

I ran `git diff e648462f1d05..551f84ef6` over exactly the six load-bearing
paths (`core/gui/viewport_editor/`, `schemas/viewport_editor.schema.yaml`,
`tests/test_viewport_editor_contract.py`,
`fixtures/gui/invented/viewport_editor_session.json`,
`apps/desktop/src/features/viewport/PipeViewport.tsx`,
`apps/desktop/src/App.test.tsx`) — **diff empty**. The content-identical
qualifier in the ledger is scoped to exactly those paths (calibration item 4);
no differing path is claimed, so no exclusion clause is needed.

GUI e2e / browser-smoke evidence (Playwright `test:e2e:desktop`, in-app browser
smokes; SMOKE.md TP-MAC-84/86/89/92/93/94/181/209/218/225 recorded in MEMORY.md)
was **never executed at the frozen SHA** and is cited recorded-only with the
exact marker `not re-executed at frozen SHA 551f84ef6`. It is **not promoted**
to a frozen-SHA pass. I did not attempt to re-execute Vitest/Playwright: those
runners write `node_modules`/cache and cannot be guaranteed write-free against
the frozen tree, so per the brief I cited the recorded sweep passes instead
(addendum 9 / calibration item 12).

## 4. Convention friction notes

- **SourceReliability under calibration item 13 vs the exemplar.** The R0b
  exemplar DEL-07-05 marked most requirement rows `REVIEWED`. Under the adopted
  item 13 (REVIEWED requires *every* load-bearing leg human-ruled), DEL-07-01's
  evidence legs are agent-generated (DEC-025 sweeps, the 2026-05-16 PKG-02 audit
  with `Review_Findings.csv` header-only / `HumanDisposition=TBD`); the human
  rulings on file (the two LifecycleCorrection Decision_Logs) govern lifecycle
  state, not the technical requirement evidence. I therefore keyed all
  REQ/ACC/EXC rows to `UNVERIFIED`. Per adopted addendum 6, all DECLARED_STATE
  prose rows are `NOT_APPLICABLE`.
- **AuthorityNeeded routing for the STALE kit surfaces.** DECL-001..004 route
  `OWNER` because the fix is a documentation-refresh repair that only the owner
  can authorize (the run STOPs before R4/R5; F-PIP-5). This follows the sibling
  exemplar's OWNER routing for setup-prose staleness. Calibration item 1's
  "AuthorityNeeded=NO for pure pointer drift" applies to the narrow rev-0.7→0.8
  decomp-pointer drift specifically; here the pointer drift is a *subset* of a
  broader setup-era-prose-vs-implemented-slice staleness that is an owner-gated
  repair candidate, so OWNER governs and the pointer drift is noted in-row.
- **Rev-0.7 decomp authority-pointer drift (owner-calibration caveat, stated
  once).** `_CONTEXT.md` and `_REFERENCES.md` cite `SOFTWARE_DECOMP.md` revision
  0.7 (`current_basis`) while the frozen decomp header is revision **0.8**
  `status: current_basis` (verified). Neither `_CONTEXT.md` nor `_REFERENCES.md`
  is an addendum-1 census surface, so this drift gets no standalone DECL row; it
  is recorded as an in-row drift fact on DECL-001/DECL-002 (which the kit points
  to for the accepted revision). The decision tokens the kit cites (DEC-009,
  DEC-012, DEC-018, AB-00-03/05/06/08, SCA-001) all still resolve in the rev-0.8
  frozen decomp.
- **AuthorityNeeded as a router, not a work queue (item 14).** REQ-001's bend /
  component-symbol authoring gap and the adjacent-homed A3 residual are recorded
  remaining work, not adjudication questions, so AuthorityNeeded=NO. No numeric
  or engineering-authority claim is being promoted anywhere in this ledger, and
  no gate-named token beyond the excluded D-41 bootstrap appears.
- **Named setup checkers absent at frozen SHA.** `tools/validation/check_four_documents.sh`
  (VER-01 / ACC-001) and `tools/validation/validate_enum.py` (VER-04 / ACC-004)
  do not exist in the frozen tree (verified by listing). The acceptances are
  still satisfied — kit existence by direct inspection, enum canonicality
  subsumed by the passing dependency-schema validator — and the tool absence is
  disclosed in-row (and reinforces the Procedure staleness on DECL-004).

## 5. Boundary-compliance statement

- Writes were confined to exactly the two output files
  `WAVES/W3/CLAIM_CONCORDANCE_DEL-07-01.csv` and `WAVES/W3/NOTES_DEL-07-01.md`
  in the working run folder. (A generator misstep briefly created a spurious
  `RUN/` subdirectory in the run folder; it was removed immediately and no
  tracked file was affected.)
- The frozen evidence worktree was read-only throughout;
  `git -C <FROZEN> status --porcelain` was empty before and after every read
  and every side-effect-free re-execution.
- No lifecycle transition was applied (`LIFECYCLE_REASSESSMENT_REQUIRED` not
  used; STALE/ALIGNED dispositions recorded only). No DAG mutation, no
  cross-project edit, no edit to any `_STATUS.md`, register, or product file.
- No release-readiness, issuance, certification, sealing, professional-approval,
  or code-compliance claim is made anywhere in these outputs (F-PIP-1..5). All
  dispositions are agent judgments, routed via `AuthorityNeeded`, never phrased
  as owner or engineering rulings.
- `SelectableUnderCurrentLoop` is `NO` on every row (mechanical: no non-bootstrap
  recorded item; addendum 12). The owner suspension is a run-level caveat and is
  not encoded per-row.
- No STOP-worthy contradiction was found (D-41 gate mechanics per RUN_BASIS are
  ruling-after-freeze, not a conflict; not re-derived).
