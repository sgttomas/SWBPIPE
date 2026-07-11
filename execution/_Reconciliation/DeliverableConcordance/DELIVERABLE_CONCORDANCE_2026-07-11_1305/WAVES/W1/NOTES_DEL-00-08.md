# NOTES — DEL-00-08 Layered software test and acceptance strategy (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen source tree
`main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Binding encoding rules:
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13). Ledger:
`CLAIM_CONCORDANCE_DEL-00-08.csv` (19 rows, 20 columns, RFC-4180 clean, CRLF).

## Deliverable character

PKG-00 architecture-runway **documentation** deliverable (Datasheet Type
`TEST_SUITE`), lifecycle `IN_PROGRESS`. Despite the `TEST_SUITE` type token, the
Datasheet Scope Boundary and Guidance Design Rationale are explicit that it
**"does not implement tests, CI jobs, benchmarks, solvers, GUI tests, or
packaging automation"** — it defines the layered test architecture and the
architecture acceptance gates only. Requirements `REQ-08-01..05` are
architecture-definition obligations whose evidence bar is "Acceptance review" /
"Human review"; the deliverable is prepared for review (`SEMANTIC_READY` target),
not accepted. The two anticipated artifacts (`docs/architecture/test_strategy.md`
and the acceptance gate matrix) do **not** exist in the frozen tree; they are
forward runway artifacts, so the concrete per-layer test definition and gate
thresholds are deferred (parallels DEL-00-03's unbuilt `application_services.md`,
which the peer pilot treated as forward, not a residual).

## Requirement-ID scheme mapping (addendum 12)

Requirement scheme `REQ-08-*` is non-self-identifying (tokens do not embed the
deliverable ID). ClaimID uses the fixed addendum-12 form `DEL-00-08-<TYPE>-NNN`.
Requirement mapping:

| Deliverable requirement ID | Ledger ClaimID |
|---|---|
| REQ-08-01 | DEL-00-08-REQ-001 |
| REQ-08-02 | DEL-00-08-REQ-002 |
| REQ-08-03 | DEL-00-08-REQ-003 |
| REQ-08-04 | DEL-00-08-REQ-004 |
| REQ-08-05 | DEL-00-08-REQ-005 |

## NormativeSource path alias (addendum 12)

`<DELIV>` = `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/`.
All bare `*.md` surface names in the ledger (`Specification.md`, `Datasheet.md`,
`Guidance.md`, `Procedure.md`, `_STATUS.md`, `MEMORY.md`, `_SEMANTIC.md`,
`_SEMANTIC_LENSING.md`, `_CONTEXT.md`) are relative to `<DELIV>`.

## Disposition histogram (reproduces from the CSV)

- ALIGNED — 16
- STALE_SETUP_SPECIFICATION — 3

Total: 19.

## ClaimType histogram (reproduces from the CSV)

- REQUIREMENT — 5
- ACCEPTANCE — 4
- EXCLUSION — 4
- DECLARED_STATE — 6

Total: 19.

## Row census rationale

- **REQUIREMENT (5):** one per current requirement ID `REQ-08-01..05`. Each is a
  documentation obligation at architecture-runway grain; substance disposition
  per convention 1 (requirement rows never take `STALE_SETUP_SPECIFICATION`).
  All ALIGNED: the obligation is articulated in the kit (Datasheet Architecture
  Roles / Procedure steps / Guidance) and consistent with the declared
  `SEMANTIC_READY`/`IN_PROGRESS` posture, with concrete realization deferred to
  visible TBDs and to the unbuilt forward artifacts. REQ-001/002/004 carry
  MEDIUM confidence (broad layer/gate lists articulated at runway grain, detail
  deferred to `test_strategy.md`/gate-matrix); REQ-003/005 HIGH (protected-content
  obligation and TBD-recording obligation are fully realized in the kit).
- **ACCEPTANCE (4):** the Specification "Acceptance Criteria" bullets that are
  genuine acceptance gates and not requirement restatements — kit-exists+identity
  (ACC-001), scope-limited-to-PKG-00 (ACC-002), TBDs-visible-and-routed
  (ACC-003), semantic-artifacts-exist (ACC-004). The two Acceptance-Criteria
  bullets "no implementation code/tests created" and "no protected data" are
  **not** duplicated as ACCEPTANCE rows; they are ledgered as EXCLUSION rows
  (EXC-001, EXC-003) to avoid acceptance/exclusion double-counting (same
  resolution as the DEL-00-03 peer pilot).
- **EXCLUSION (4):** the deliverable's distinct scope exclusions — no product
  implementation code / no test-CI-benchmark-solver-GUI-packaging implementation
  (EXC-001), no PKG-01..12 authorization/advancement (EXC-002), no
  protected standards/proprietary data (EXC-003, OPS-K-IP-1), no
  code-compliance/certification/professional-approval claim (EXC-004,
  OPS-K-AUTH-1). Recording these asserts no positive certification or release
  claim (F-PIP-1..5 held).
- **DECLARED_STATE (6):** exactly one per four-document kit surface plus
  `_STATUS.md` and `MEMORY.md` (both carry state declarations), per addendum 1.
  No deliverable-owned in-tree README exists (checked). `_SEMANTIC.md` /
  `_SEMANTIC_LENSING.md` / `_CONTEXT.md` / `_REFERENCES.md` / `_DEPENDENCIES.md`
  are outside the addendum-1 census and get no DECLARED_STATE row (their content
  is cross-referenced where relevant).
- **REMAINING_WORK (0):** the only recorded `## Remaining` item is the seeded
  `(gated: D-41)` concordance bootstrap item, recorded verbatim only in the
  `_STATUS.md` surface row's `RecordedRemaining` (DECL-005) and excluded from all
  residual/gate/selectability analysis (addendum 2). R1 `DELIVERABLE_INVENTORY.csv`
  records `RemainingItemCount=1, BootstrapItemPresent=YES, NonBootstrapItems=NONE,
  SelectableUnderCurrentLoop=NO`. No real residual.
- **IMPLEMENTED_UNMAPPED (0):** see the material-surface analysis below. No
  unmapped material surface is in DEL-00-08's orbit; the 14 co-attributed
  surfaces in `IMPLEMENTATION_SURFACES.csv` are already mapped (to their
  implementing deliverables) — they are not unmapped, and DEL-00-08's scope
  excludes owning them.

## The three STALE_SETUP_SPECIFICATION calls (rev-0.7 authority-citation drift)

DECL-001 (Specification, line 32), DECL-002 (Datasheet, line 34), and DECL-004
(Procedure, line 8) each cite `SOFTWARE_DECOMP.md` **revision 0.7** as the
current/upstream basis. The frozen tree's `SOFTWARE_DECOMP.md` header is
**revision 0.8** (RUN_BASIS confirms v0.8 with DEC-072 carried through). Under the
widened `STALE_SETUP_SPECIFICATION` definition (addendum 4 — "post-alignment
drift"), these declared-state surfaces carry a stale authority-revision citation.
Context recorded for the reviewer:

- The lag is a **pending owning-workflow refresh**, not an authority question:
  the deliverable's last authority refresh (`TASK_RUN_2026-06-04_authority-
  refresh-0.7-dag006_DEL-00-08`) refreshed to 0.7/DAG-006; DAG advanced to
  DAG-007 on most surfaces but no 0.8 refresh has run. `AuthorityNeeded=NO`.
- The drift is **immaterial to substance**: the layered-test-strategy scope is
  unaffected by the rev-0.8 delta. The disposition flags the citation, not a
  substance defect.
- `Guidance.md` (DECL-003) carries **no** revision citation (only DAG-007) →
  ALIGNED. `_STATUS.md` (DECL-005) is current (2026-07-11) → ALIGNED.
  `MEMORY.md` (DECL-006) cites 0.7/DAG-006 only inside a **dated 2026-06-04 log
  entry** → per addendum 1 that is historical; drift noted on the surface row,
  not dispositioned as staleness → ALIGNED.
- Non-census surfaces `_CONTEXT.md` (Accepted Revision 0.7), `_REFERENCES.md`,
  and `_DEPENDENCIES.md` carry the same rev-0.7 citation. They are outside the
  addendum-1 DECLARED_STATE census so take no row; the drift is uniform and
  noted here for the owning refresh workflow.

## Material-surface analysis (the orientation-note question)

The orientation note flags 13 verification suites with no recorded execution
evidence (incl. 9 script-style pytest no-ops and the `src-tauri` crate outside
sweep roots) as possibly bearing on a test-strategy deliverable. Finding after
re-verification against the frozen indexes:

- DEL-00-08 has **no rows** in `VERIFICATION_INDEX.csv`,
  `VALIDATION_AND_PROVENANCE_INDEX.csv`, or `DECISIONS_INDEX.csv` (grep
  confirmed; the only DECISIONS mentions are DEC-007 and DEC-072, which name the
  whole PKG-00 DEL-00-01..08 range, not DEL-00-08-owned decisions).
- DEL-00-08 **is** co-attributed on 14 shared `IMPLEMENTATION_SURFACES.csv`
  surfaces (SURF-003/004/005/006/027/056/058/062/065/066/099/102/140/155/160/215/222
  — build configs, the `src-tauri` app crate, desktop services, core crates,
  fixture families, release tools). In **every** case the attribution provenance
  is this deliverable's own `_run_records/**` or `MEMORY.md` — i.e. sessions that
  happened to be logged under the DEL-00-08 folder while doing test-harness and
  exit-chain work (Playwright smoke, wasm-engine packaging regression, seam/
  corpus fan-out, unit-catalog deflake, evidence-sweep, release-readiness). Each
  of those surfaces is **also** attributed to the deliverables that actually own
  it (DEL-09-*, DEL-16-*, DEL-02/05/06/07/10-*, etc.).
- **Judgment:** these are shared surfaces already **mapped** to their owning
  deliverables, so they are not `IMPLEMENTED_UNMAPPED` candidates (addendum 8 /
  convention: `IMPLEMENTED_UNMAPPED` is for material surfaces lacking accepted
  mapping). And DEL-00-08's own scope **excludes** implementing tests/CI/
  benchmarks (EXC-001), so it does not *own* these surfaces as requirement
  implementations either. The unexecuted-suite gap therefore lives with the
  owning implementing deliverables (to be dispositioned in their own wave
  ledgers — `src-tauri` is SURF-005, a cross-package shared crate routed
  `AuthorityNeeded=OWNER` for mapping per addendum 8 when its owning wave reaches
  it), **not** as a DEL-00-08 residual. It does not lower any DEL-00-08
  requirement disposition: REQ-08-01/02 are met by *defining* the layered test/
  verification obligations, which the kit does; whether the downstream suites are
  executed is the owning deliverables' evidence question, not this documentation
  deliverable's. Recorded on the EXC-001 and DECL-006 rows and self-flagged.

## Self-flagged rows

- **DEL-00-08-REQ-001 / REQ-002 / REQ-004** — MEDIUM confidence, judgment call
  the conventions leave partly open. Read as ALIGNED because each requirement is
  a documentation obligation to *define/require* (layered tests; deterministic
  verification-before-release; architecture acceptance gates), met at
  architecture-runway grain by the kit, with concrete realization appropriately
  deferred to visible TBDs and the two unbuilt forward artifacts
  (`test_strategy.md`, gate matrix). Not `DOCUMENTED_UNIMPLEMENTED` /
  `PARTIALLY_IMPLEMENTED`: this is a documentation deliverable with no
  implementation surface of its own, so there is no partial *implementation* to
  weigh — only whether the obligation is articulated, which it is. A reviewer who
  reads the unbuilt `test_strategy.md`/gate-matrix as a required-artifact gap
  might prefer a partial/deferred disposition; I followed the DEL-00-03 peer
  precedent (unbuilt anticipated artifact = forward, ALIGNED with the gap in
  `RemainingWork`).
- **DEL-00-08-DECL-001 / DECL-002 / DECL-004** — judgment call (same as the
  DEL-00-03 peer): whether a one-revision-behind, decomp-sanctioned,
  substance-immaterial authority citation warrants `STALE_SETUP_SPECIFICATION`
  vs `ALIGNED`. Chose the honest drift-flag (convention 1 puts staleness on
  declared-state rows; addendum 4 covers post-alignment drift). Reviewer may
  prefer ALIGNED-with-note; disposition is at surface grain while only the
  revision pointer, not the surface's substance, is stale.
- **DEL-00-08-EXC-001 / DECL-006** — the material-surface / co-attribution
  judgment above. I ruled the 14 shared surfaces (incl. the `src-tauri` crate and
  the unexecuted suites) as *not* DEL-00-08-owned implementation and *not*
  `IMPLEMENTED_UNMAPPED`, on the strength of the explicit scope exclusion plus
  the surfaces' existing multi-deliverable mapping. Reviewer eyes wanted on
  whether any of these should instead surface as a DEL-00-08 orbit
  `IMPLEMENTED_UNMAPPED` row; I judged not.
- **DEL-00-08-EXC-003** — MEDIUM confidence. "No protected standards/proprietary
  data" rests on a non-exhaustive agent content scan of generic architecture
  prose; no protected payload observed, exhaustive certification out of scope and
  not claimed (F-PIP boundary).
- **General (requirement rows):** ALIGNED for REQ rows rests on runway-grain
  articulation plus a pending (not failed) human review; `SourceReliability=
  UNVERIFIED` per addendum 6 (agent-generated architecture prose, `_SEMANTIC.md`
  Audit PASS, human disposition pending). The prior PKG00 lock review
  (2026-05-11) was administratively set aside by the D-40 rebaseline (DEC-072)
  and preserved as historical evidence — it is **not** cited as current
  acceptance evidence on any row.

## Evidence-execution log

Read-only discovery only; **no re-executable tests exist for this documentation
deliverable** (it implements none), so nothing was re-executed and no
`not re-executed at frozen SHA 551f84ef6` recorded-pass citation was needed.
Checks run (all read-only, side-effect-free, over the frozen tree):

- `find <DELIV> -type f ! -name "*.md"` → 0 (backs EXC-001 / "no code/tests").
- Identity grep `DEL-00-08` per kit doc → Specification 2, Datasheet 3, Guidance
  1, Procedure 1 (backs ACC-001).
- `grep -n` rev-0.7 citations → Specification:32, Datasheet:34, Procedure:8
  (backs DECL-001/002/004); confirmed frozen `SOFTWARE_DECOMP.md` header
  `revision: 0.8`.
- `grep -niE certif|seal|code.compliance|professional.approval|authenticate`
  over kit → only disclaimers/boundary-preservation statements (backs EXC-004).
- `_SEMANTIC.md` → `Audit Result: PASS`; `_SEMANTIC_LENSING.md` → `Total
  warranted items: 0`, `Warnings: none` (backs ACC-004).
- R1 index cross-checks (re-verified against the frozen tree): DEL-00-08 present
  in `DELIVERABLE_INVENTORY.csv` (`IN_PROGRESS`; `RequirementIDs=REQ-08-01..05`;
  `NonBootstrapItems=NONE`; `SelectableUnderCurrentLoop=NO`); **absent** from
  `VERIFICATION_INDEX.csv`, `VALIDATION_AND_PROVENANCE_INDEX.csv`,
  `DECISIONS_INDEX.csv`; **co-attributed** on 14 shared
  `IMPLEMENTATION_SURFACES.csv` surfaces via run-record/MEMORY provenance only.
- `git -C FROZEN status --porcelain` run before and after all reads and after
  CSV generation → **empty every time** (0 lines). CSV generated with
  `PYTHONDONTWRITEBYTECODE=1`; all build/temp artifacts confined to the scratch
  dir; nothing written under the frozen tree.

## Convention friction notes

1. **Documentation-deliverable evidence columns.** §6/conventions are written for
   code-bearing deliverables. For a pure architecture-documentation deliverable
   the honest values are the surface locations of the articulated obligation
   (Implementation), `NONE_FOUND`-with-reason for Verification (acceptance is
   human review, not a test), and `NOT_APPLICABLE`-with-reason for Validation.
   Flagged so aggregation does not read `NONE_FOUND` verification as a defect.
2. **`TEST_SUITE` type token vs documentation reality.** The Datasheet Identity
   Type is `TEST_SUITE`, yet the Scope Boundary explicitly implements no tests.
   The type token names the *subject* (test strategy), not that this deliverable
   ships a suite. Handled by reading it as a documentation deliverable; noted so
   the token is not mis-read as an implemented test suite.
3. **Co-attribution via run-record provenance.** The R1
   `IMPLEMENTATION_SURFACES.csv` attributes shared surfaces to a deliverable when
   that deliverable's `_run_records`/`MEMORY` touched them. For DEL-00-08 this
   pulls in 14 shared surfaces from historical test-harness sessions even though
   the deliverable owns none of them as requirements. Resolved per the
   material-surface analysis above (not `IMPLEMENTED_UNMAPPED`; owned elsewhere);
   flagged because the provenance-based attribution could be mis-read as
   DEL-00-08 owning implementation surface.
4. **Acceptance vs exclusion overlap.** As in DEL-00-03, "no code/tests" and "no
   protected data" appear as both Acceptance Criteria and scope exclusions;
   resolved by ledgering each once as an EXCLUSION row to keep the ClaimType
   histogram meaningful.

## Boundary-compliance statement

- Discovery was strictly READ-ONLY outside the two W1 output files
  (`CLAIM_CONCORDANCE_DEL-00-08.csv`, `NOTES_DEL-00-08.md`). No `_STATUS.md`,
  register, product, or DAG file was edited; no lifecycle transition applied
  (`LIFECYCLE_REASSESSMENT_REQUIRED` not needed, not applied); no cross-project
  edits.
- F-PIP-1..5 held: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim appears anywhere in the ledger
  or these notes (exclusion rows assert the *absence* of such claims, consistent
  with the fences).
- All dispositions are agent judgments routed via `AuthorityNeeded` (all `NO`
  here); none phrased as owner or engineering rulings. No `DEFERRED_AGENT_WORKFLOW`
  items arose (the unexecuted-suite question routes to the owning deliverables'
  own waves, not to an agent-workflow redesign).
- Frozen tree porcelain clean before, during, and after (0 lines each check).
  Writes confined to the two output files under `WAVES/W1/`.
