# NOTES — DEL-04-02 Straight pipe element (W2)

Wave W2 · frozen SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7` · binding set
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13) plus the W1 calibration
items (rev-drift STALE-side encoding; overtaken-review-prose encoding;
addendum-10 exclusion discipline; byte-exact bootstrap transcription;
PKG-04/05 verification-vs-validation strictness). Ledger:
`CLAIM_CONCORDANCE_DEL-04-02.csv` (16 rows).

## Run-level path aliases (addendum 12)

- `KIT/` = `projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/`
- Bare code/test/validation paths (`core/…`, `validation/…`, `tools/…`) are
  relative to the working root `projects/chirality-piping/`.

## Requirement-ID normalization (addendum 12)

Native scheme `DEL-04-02-RQ-*` (Specification requirements table) maps 1:1 to
ClaimIDs `DEL-04-02-REQ-001..007` (RQ-001→REQ-001 … RQ-007→REQ-007). All 7
native requirement IDs mapped; none dropped or merged. This matches the
inventory row (`DEL-04-02-RQ-001;…;-RQ-007`, RequirementScheme PRESENT).

## Disposition histogram (reproduces from CSV column `Disposition`)

| Disposition | Count |
|---|---|
| ALIGNED | 11 |
| STALE_SETUP_SPECIFICATION | 4 |
| PARTIALLY_IMPLEMENTED | 1 |
| **Total** | **16** |

## ClaimType histogram (reproduces from CSV column `ClaimType`)

| ClaimType | Count |
|---|---|
| REQUIREMENT | 7 |
| EXCLUSION | 3 |
| DECLARED_STATE | 6 |
| **Total** | **16** |

## Census decisions

- **Requirements (7):** one row per current requirement ID; substance
  dispositions only (convention 1 — no requirement row takes
  `STALE_SETUP_SPECIFICATION` even though the Specification is setup-era).
- **Acceptance rows: none.** The Specification `## Verification` table
  ("Minimum setup expectation" rows) and the Procedure `## Verification` table
  restate the requirement areas (solver boundary, unit safety, missing inputs,
  force recovery, IP/data boundary) at requirement grain; no
  addendum-12-grain acceptance criteria exist, so no mirrored ACCEPTANCE rows
  (same call as the W1 sibling DEL-03-08).
- **Exclusions (3):** EXC-001 (code-compliance / rule-pack / stress-check /
  professional-approval decisions out of scope), EXC-002 (no protected or
  bundled data — tables, allowables, SIF/flexibility, conversion constants,
  proprietary/private data), EXC-003 (adjacent-mechanics scope boundary —
  global assembly/sparse solve in DEL-04-01, load-case formation/application
  in PKG-05, downstream stress recovery in DEL-05-03). All are stably and
  repeatedly declared (Datasheet Attributes, _CONTEXT package exclusions,
  Guidance trade-offs, crate README, MEMORY boundary statements).
- **DECLARED_STATE (6):** exactly one per four-document kit surface +
  `_STATUS.md` + `MEMORY.md` (addendum 1). **No row for
  `core/solver/straight_pipe/README.md`:** following the W1 precedent
  (DEL-03-07/DEL-03-08), a code-surface README is not a "deliverable-owned
  in-tree README" for the addendum-1 census; the deliverable folder itself has
  no README. The crate README is treated as implementation evidence.
- **REMAINING_WORK rows: none.** The frozen `_STATUS.md ## Remaining` carries
  only the seeded `(gated: D-41)` bootstrap item — re-verified against the
  frozen file by this pilot; matches `DELIVERABLE_INVENTORY.csv`
  (RemainingItemCount=1, NonBootstrapItems=NONE). The bootstrap item is
  transcribed byte-exact into DECL-005 `RecordedRemaining` only (addendum 2;
  W1 calibration item 5 — gate/source cells left at column defaults
  `NONE_RECORDED`). Doc-level TBDs (canonical unit basis, solver library,
  result-envelope integration, release thresholds, professional reliance) are
  declared open items, not `_STATUS ## Remaining` residuals; they are carried
  in `RemainingWork` cells. None is permitted by a named ruling as a bounded
  transitional state in the addendum-11 sense, so none takes
  `ACCEPTED_DIVERGENCE` and none earns its own row.
- **IMPLEMENTED_UNMAPPED rows: none.** The material surface
  `core/solver/straight_pipe` (SURF-139) is attributed to DEL-04-02 — mapped.
  Shared crates in the orbit (`core/loads/primitive_loads` SURF-094,
  `core/loads/stress_recovery` SURF-095, `core/solver/frame_kernel` SURF-132,
  `core/product_physics` SURF-102, `core/solver/performance_harness`
  SURF-136) are broadly attributed across many deliverables and are not in
  this deliverable's exclusive orbit; no unmapped row is owed (addendum 8).
- `SelectableUnderCurrentLoop = NO` on every row: no recorded non-bootstrap
  item exists (conventions 6/12). The owner suspension is a run-level caveat,
  never per-row.
- No SECURITY-class claims exist in this deliverable; the W1 SECURITY
  marker/routing harmonization did not arise.

## Central evidence finding — setup-era kit vs mature implemented crate

The four-document kit is still the 2026-04-30 setup kit: the Specification
declares "setup evidence for the future straight pipe element backend slice …
This setup pass does not implement solver code"; the Datasheet declares "the
setup kit describes the future implementation boundary only"; Guidance says
upstream contracts are "not resolved in this setup pass" and examples are TBD;
the Procedure defines a *future* implementation task gated on separate
authorization. The frozen tree meanwhile carries a mature implemented slice:
`core/solver/straight_pipe` (2,369-line crate, 33 deterministic unit tests,
frame-kernel-boundary delegation, boundary metadata with human-accepted unit
binding, end/station-resultant recovery, spanned loads, axial effects) plus
witness-backed mechanics benchmarks. Per the widened addendum-4 definition,
all four kit DECLARED_STATE rows take `STALE_SETUP_SPECIFICATION`
(DECL-001..004); the requirement rows keep substance dispositions.

**Owner-calibration caveat (W1 calibration item 1, stated once):** the
Datasheet cites SOFTWARE_DECOMP revision 0.7 architecture-basis IDs while the
frozen decomp header is revision 0.8, `status: current_basis` — pure
authority-pointer drift encoded STALE-side in-row on DECL-002 per the W1
adjudication; if the owner later calibrates the corpus-wide rev-drift pattern
to ALIGNED-with-note, the drift facts are recorded in-row either way. DECL-002
routes `AuthorityNeeded=OWNER` because the kit also carries overtaken TBD
registers (Conditions and Open Setup Questions declare implemented contracts
TBD); DECL-001/003/004 route REVIEW (doc reconciliation without a rev
pointer).

## Central evidence finding — MECHANICS validation posture (PKG-04 strictness)

Unlike the W1 DEL-03-08 case, genuine validation-class evidence exists here
and *executes* the production crate: `validation/benchmarks/mechanics`
(RUST-34) constructs `StraightPipeElement` and checks recovered values against
project-original open-mechanics hand-calc witnesses
(`validation/hand_calcs/mechanics/straight_pipe_weight_recovery.md` =
HC-MECH-004, plus the tp_phys_002/004–009 witness set). This pilot re-executed
that suite at the frozen SHA (33/33 pass). No unit test was promoted to
validation: unit-test evidence is cited only as verification; the
witness-backed benchmark executions are cited as validation. The witnesses are
agent-generated/agent-audited with the human ruling still TBD under the
DEC-024/DEC-026 class-tiered convention (VALIDATION_AND_PROVENANCE_INDEX
HC-MECH-004: `UNVERIFIED`), so the MECHANICS rows resting on them (REQ-002,
REQ-004, REQ-005) are capped at Confidence MEDIUM and routed
`AuthorityNeeded=OWNER` by analogy to addendum 13; each disposition also rests
on independent implementation + re-executed verification grounds. No numeric
acceptance threshold beyond the recorded accepted witness sets was assumed
(W1 calibration item 8); no ENGINEERING routing was needed because no
unsettled formulation/tolerance question is asserted by any row.

## Self-flagged rows

- **DEL-04-02-REQ-002** — validation via pending-human witnesses: MEDIUM cap +
  OWNER routing applied by *analogy* to addendum 13 (the witness state is
  "ruling TBD", not literally `TECHNICALLY_ADDRESSED_PENDING_HUMAN`).
  Reviewer eyes on whether the cap/routing is owed or HIGH/NO suffices.
- **DEL-04-02-REQ-004** — same pending-witness cap/routing judgment as
  REQ-002 (HC-MECH-004 weight-recovery witness).
- **DEL-04-02-REQ-005** — W1 calibration item 6 grain call: encoded
  **ALIGNED at the human-accepted boundary-metadata contract grain** (unit
  identifiers bound at the external boundary via `StraightPipeBoundaryMetadata`
  per the ACCEPT_AS_IS/RESOLVED finding PKG04-DEL0402-PKG02-001), not
  PARTIALLY_IMPLEMENTED for the internal plain-f64 components. Grain stated
  in-row.
- **DEL-04-02-REQ-006** — ClaimClass judgment: classed VALIDATION (suite
  coverage/harness claim per addendum 7) rather than WORKFLOW; "before release
  use" read as no-release-claim-owed at the frozen state (no release use
  exists or is claimed).
- **DEL-04-02-REQ-007** — W1 calibration item 6 grain call, opposite pick from
  REQ-005: encoded **PARTIALLY_IMPLEMENTED** because the requirement's
  result-envelope clause is expressly recorded as unfinished (MEMORY Open
  TBDs; 2026-06-05 review residual "final result-envelope integration"), while
  the no-compliance-claim clause fully holds. Grain stated in-row.
- **DEL-04-02-EXC-003** — boundary-growth judgment: the implemented
  load-recovery surface (spanned/point loads, axial effects, station sweeps)
  sits close to the PKG-05 load-application boundary; judged recovery-side
  (no load-case algebra/combination/hidden application) per the recorded
  approved TP-PHYS briefs. Confidence MEDIUM.
- **DEL-04-02-DECL-003** — STALE-vs-ALIGNED-with-note judgment: Guidance's
  principles still describe the implemented posture, but its "not resolved in
  this setup pass" and "examples are TBD" declarations are overtaken; judged
  STALE (kit-wide setup-era framing), Confidence MEDIUM. The conservative
  ALIGNED-with-note reading is defensible.
- **DEL-04-02-DECL-002** — AuthorityNeeded routing: OWNER (rev-0.7 pointer +
  overtaken TBD registers per W1 calibration item 1) vs REVIEW on the other
  three kit surfaces; flagging the split for reviewer consistency check.
- **DEL-04-02-DECL-006** — MEMORY historical-drift notes (addendum 1, notes
  not dispositions): undated header sections describe only the initial
  2026-05-01 slice ("6 tests" vs 33 at the frozen SHA); Open TBDs partially
  overtaken (force-per-length TBD closed 2026-06-05); 2026-06-17 (MEMORY) vs
  2026-06-16 (K-CONFLICT-1 ruling) housekeeping-reset date discrepancy.

## Evidence-execution log

- **Frozen-tree porcelain:** `git -C <FROZEN> status --porcelain` empty
  BEFORE discovery reads, after the benchmark re-execution, and AFTER all
  work (final check below).
- **Re-executed (addendum 9), in-tree with redirected artifacts:**
  `cargo test --locked --manifest-path
  validation/benchmarks/mechanics/Cargo.toml` at the frozen worktree with
  `CARGO_TARGET_DIR` pointed at this pilot's scratchpad → **33 passed, 0
  failed** (`--locked` prevented any lockfile write; porcelain empty
  before/after). This is the validation-executing suite (RUST-34).
- **Re-executed via byte-identical out-of-tree copy:** `core/solver/
  straight_pipe` has **no `Cargo.lock` in-tree**, so an in-place `cargo test`
  would have created one inside the frozen tree (forbidden even git-ignored,
  addendum 9). The crate and its sole path dependency `core/solver/
  frame_kernel` were copied to scratchpad, byte-identity confirmed with
  `diff -r` (clean, both crates), then `cargo test` (33/33 pass) and
  `cargo fmt --check` (clean) run on the copy with external
  `CARGO_TARGET_DIR`. Cited on ledger rows as re-execution at frozen content
  via byte-identical copy — disclosed as such, not represented as an in-tree
  run.
- **Addendum-10 qualifier re-diffed by this pilot:**
  `git merge-base --is-ancestor e648462f1d05… 551f84ef6…` → ancestor
  confirmed; `git diff --name-only e648462f1d0521e26df15d04a988391343018886
  551f84ef6be656f1603ce0acfa5e3935aa9683c7` over `core/`,
  `validation/benchmarks/`, `validation/hand_calcs/`,
  `tools/release/check_release_readiness.py` → **empty** (0 paths). The
  ledger's content-identical qualifier names exactly this diffed path set; no
  exclusion clause is needed because no diffed path differed.
- **Cited as recorded (not re-executed):** VERIFICATION_INDEX RUST-32/RUST-34
  sweep passes at ancestor `e648462f1d05`
  (`validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`),
  carried beside the re-executions with the re-diffed qualifier.
- **In-tree ruling records confirmed present:**
  `execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-05-11_2052/Decision_Log.md`;
  `…/LIFECYCLE_CORRECTION_2026-07-02_2050/Decision_Log.md` (DecisionBasis for
  DECL-005); `execution/_Reconciliation/Reviews/PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md`
  and review snapshot `…/Reviews/REV_DEL-04-02_2026-06-05_2120/`
  (DecisionBasis for REQ-001/REQ-005); `KIT/Review_Findings.csv` rows
  PKG04-DEL0402-PKG02-001/002 (ACCEPT_AS_IS/RESOLVED).
- **Validation artifacts inspected, not executed separately:**
  `validation/hand_calcs/mechanics/straight_pipe_weight_recovery.md`
  (HC-MECH-004 provenance block confirmed project-original with invented
  inputs); benchmark fixture linkage `straight_pipe_weight_recovery_fixture`
  (`validation/benchmarks/mechanics/src/lib.rs` L1207-1224 cites the hand-calc
  path).
- **Scratch script:** `build_csv_DEL-04-02.py` (deliverable-unique name per
  W1 calibration item 7), scratchpad-only.

## Convention friction notes

- **Addendum 9 vs lockfile-less crates:** "re-execute in the frozen tree with
  redirected build artifacts" cannot be satisfied for a Rust crate with no
  committed `Cargo.lock` — cargo insists on writing one next to the manifest.
  The byte-identical out-of-tree copy (identity proven by `diff -r`) is the
  closest side-effect-free equivalent; the binding set does not name this
  case. Disclosed in-row and here; reviewer may prefer the recorded-pass-only
  citation.
- **Addendum 13 scope:** the addendum names
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN` validation evidence; the hand-calc
  witnesses here are in a different pending-human state ("ruling TBD under
  DEC-024/DEC-026"). I applied the cap/routing by analogy (MEDIUM + OWNER on
  REQ-002/004/005). The binding set is silent on this witness state.
- **Setup-era kits in PKG-04:** unlike W1 PKG-03 kits (reconciled to
  implementation in June, drift localized), this kit was never reconciled
  after implementation. The widened addendum-4 STALE definition covers it
  cleanly, but produces a 4-of-4 STALE kit census — expect this pattern on
  other PKG-04/05 deliverables whose implementation ran ahead of doc
  reconciliation.
- **"Before release use" (RQ-006):** the binding set gives no guidance on
  requirements conditioned on a future event (release) that has not occurred;
  I read the requirement as satisfied when the tests exist and are
  deterministic at the frozen state, with release gating carried as
  RemainingWork and no release-readiness assessment made (F-PIP fence).

## Boundary-compliance statement

- All fences held. Discovery was READ-ONLY outside the two output files
  (`CLAIM_CONCORDANCE_DEL-04-02.csv`, `NOTES_DEL-04-02.md`, both under
  `RUN/WAVES/W2/`). No lifecycle transition applied; no DAG mutation; no
  cross-project edit; no edit to any `_STATUS.md`, register, schema, test, or
  product file. Scratch work confined to the session scratchpad.
- Nothing in these outputs makes a release-readiness, issuance, certification,
  sealing, professional-approval, or code-compliance claim (F-PIP-1..5).
  REQ-007/EXC-001 record the *deliverable's* no-compliance-claim posture as an
  evidence finding, not a compliance assertion by this pilot.
- All dispositions are agent judgments routed via `AuthorityNeeded`; none is
  phrased as an owner or engineering ruling. No `DEFERRED_AGENT_WORKFLOW`
  condition arose.
- Frozen evidence worktree `.claude-worktrees/piping-frozen-551f84ef6`:
  `git status --porcelain` empty before discovery, after the sandboxed
  benchmark re-execution, and after all writes. No write of any kind was made
  under the frozen tree, including git-ignored paths (`CARGO_TARGET_DIR`
  external; no `Cargo.lock` created — see evidence-execution log).
