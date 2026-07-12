# Notes — DEL-00-02 Repository and module boundary architecture (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Binding set: `R1_CONVENTIONS.md`
(conventions 1–8 + addenda 1–13). Ledger: `CLAIM_CONCORDANCE_DEL-00-02.csv`
(16 rows).

## Requirement-ID scheme mapping (non-self-identifying → addendum-12 ClaimID)

DEL-00-02 uses scheme `REQ-02-*`. ClaimID TYPE segment disambiguates per
addendum 12 (`<DEL-ID>-<TYPE>-NNN`):

| Requirement ID | ClaimID |
|---|---|
| REQ-02-01 | DEL-00-02-REQ-001 |
| REQ-02-02 | DEL-00-02-REQ-002 |
| REQ-02-03 | DEL-00-02-REQ-003 |
| REQ-02-04 | DEL-00-02-REQ-004 |
| REQ-02-05 | DEL-00-02-REQ-005 |

ACC/EXC/DECL ClaimIDs are run-local (no corresponding source IDs); numbered
NNN from 001 within each TYPE.

## Disposition histogram (reproduces from CSV column 17)

- ALIGNED — 12
- STALE_SETUP_SPECIFICATION — 3
- PARTIALLY_IMPLEMENTED — 1

Total 16.

## ClaimType histogram (reproduces from CSV column 4)

- REQUIREMENT — 5
- ACCEPTANCE — 2
- EXCLUSION — 3
- DECLARED_STATE — 6

Total 16.

## Deliverable character and the central evidence finding

DEL-00-02 is a documentation-only architecture-runway deliverable (Type
`API_CONTRACT`; write boundary "document kit and semantic artifacts only";
scope explicitly forbids creating code). It carries no verification or
validation index rows (confirmed: DEL-00-02 absent from
`VERIFICATION_INDEX.csv` and `VALIDATION_AND_PROVENANCE_INDEX.csv`). There is
no behavioral/mechanics surface to validate, so `ValidationEvidence` is
`NOT_APPLICABLE` on every row and the "implementation" of each documentation
requirement is the documentation content existing and being accepted.

The decisive evidence for REQ-02-01/02/03 is architecture basis **AB-00-02**,
recorded in `execution/_Decomposition/SOFTWARE_DECOMP.md` (the AB-00-02 row),
keyed to DEL-00-02 and human-approved through SCA-001 Gate 3/4 (2026-04-30;
`DEC-008`). AB-00-02 verbatim covers the three substantive obligations —
layer/module responsibilities for the eleven named layers; dependencies point
inward toward domain contracts; adapters/plugins may translate but not bypass
governance/validation/diagnostics. Because that definition exists and is
human-accepted, those requirements are `ALIGNED` (SourceReliability
`REVIEWED`), not documentation-unimplemented. The anticipated concrete artifact
`docs/architecture/module_boundaries.md` does **not** exist in the frozen tree
(`git ls-files docs/architecture/` confirms), but the decomposition SOW-057
note ("concrete folder names may refine during implementation without changing
layer responsibilities") plus DEC-012 (SCA-001 TBD boundary) make the concrete
repository-layout artifact a permitted deferral, not an open residual — so
REQ-02-04 (record TBDs without inventing specifics) is `ALIGNED` and no
`REMAINING_WORK` row is warranted for it.

REQ-02-05 (define review checks that detect package-local architecture drift)
is the one `PARTIALLY_IMPLEMENTED` row: the kit defines a review gate
(Procedure Verification Checks + Specification Human Review Gate + _CONTEXT
Architecture Gate Rule holding PKG-01..PKG-12), but the checks are
kit-hygiene / sufficiency-gate oriented and no concrete drift-detection check
(e.g., comparing package-local choices against AB-00-02) is specified. Routed
`AuthorityNeeded=REVIEW` (the Human Review Gate owns the sufficiency call).

## Census decisions

- **REMAINING_WORK rows: zero.** `_STATUS.md ## Remaining` carries only the
  seeded `(gated: D-41)` concordance bootstrap item
  (`DELIVERABLE_INVENTORY.csv`: RemainingItemCount=1, BootstrapItemPresent=YES,
  NonBootstrapItems=NONE). Per addendum 2 it is copied verbatim into the
  `_STATUS.md` DECLARED_STATE row's `RecordedRemaining` only, never its own
  row, and excluded from residual/gate/selectability analysis. Consequently
  `SelectableUnderCurrentLoop=NO` on every row and `GateOrStageConstraint=
  NONE_RECORDED` throughout (matches inventory Selectable=NO).
- **DECLARED_STATE rows: six** per addendum 1 — one per four-document kit
  surface (Specification, Datasheet, Guidance, Procedure) plus `_STATUS.md` and
  `MEMORY.md`. No deliverable-owned in-tree README exists (folder holds only
  the kit, semantic artifacts, `_`-prefixed governance files, and
  `_run_records/`), so no README row. `_CONTEXT.md`, `_DEPENDENCIES.md`,
  `_REFERENCES.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md` are not part of the
  addendum-1 declared-state census and get no DECLARED_STATE rows (the semantic
  artifacts are instead cited as evidence on ACC-002). SourceReliability
  `NOT_APPLICABLE` on all six (addendum 6).
- **MEMORY.md** is a single dated log entry (2026-06-04). It declares a
  current authority basis (SOFTWARE_DECOMP rev 0.7 + approved DAG-006) but
  self-marks its preserved references as historical, not current-authority
  claims. The DAG-006 reference is superseded by DAG-007, but per addendum 1
  drift inside a dated MEMORY log is a note on the surface row, never a
  staleness disposition — recorded as ALIGNED with the note (Confidence
  MEDIUM).
- **ACCEPTANCE rows: two** at addendum-12 grain — kit-existence/identity
  (ACC-001) and semantic-artifact existence (ACC-002). The other Specification
  acceptance criteria restate REQ-02-04 (TBD visibility) or the exclusions and
  were not mirrored as separate ACCEPTANCE rows (census rule). Both routed
  `AuthorityNeeded=REVIEW` because human acceptance is not yet recorded
  (deliverable IN_PROGRESS); SourceReliability `UNVERIFIED`.
- **EXCLUSION rows: three** — no implementation code created (EXC-001);
  does not authorize/advance PKG-01..PKG-12 (EXC-002); no protected
  standards/proprietary content (EXC-003). The remaining Required Invariants
  (OPS-K-DATA-2/-AUTH-1/-MECH-1/-AGENT-1/-AGENT-3) are inherited cross-cutting
  invariants from `docs/CONTRACT.md`, not DEL-00-02-specific scope exclusions,
  and were not given rows.
- **IMPLEMENTED_UNMAPPED rows: zero.** The surfaces attributing DEL-00-02 in
  `IMPLEMENTATION_SURFACES.csv` (SURF-004 build-wasm-engine.mjs, SURF-023
  handoff, SURF-028 local-fea-handoff, SURF-065 wasmEngine loader, SURF-099
  operation_applier crate) are all multi-deliverable-mapped, so none is an
  unmapped surface (addendum 8 material-grain, unmapped-only test not met).

## Fan-in repair (fable re-run)

The W1 fan-in verification (`W1_VERIFICATION_PKG-00.md`) found this ledger
DEFECTIVE on three rows; per the owner ruling the ledger was re-run by a
fable repair pilot, who owns the changed claims below. All repair
dispositions remain agent judgments, not owner or engineering rulings.

**Named defects (verifier):** DECL-001, DECL-002, DECL-004 — the
SOFTWARE_DECOMP rev-0.7 authority-pointer drift neither dispositioned nor
noted (discovery miss); wrong disposition under the addendum-4 adjudication.
**Named QUALIFIED item:** EXC-001 `SourceReliability=REVIEWED` with
`DecisionBasis=NONE_FOUND` (addendum-6 edge).

**Independent re-verification at frozen SHA 551f84ef6 (not taken on faith):**

- `Specification.md` line 32: "Upstream authority is
  `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, the
  SCA-001/SCA-003/SCA-004 architecture-basis records, and approved
  `execution/_DAG/DAG-007/` graph authority." — confirmed present.
- `Datasheet.md` line 32: "`execution/_Decomposition/SOFTWARE_DECOMP.md`
  revision 0.7 for package and deliverable authority." — confirmed present.
- `Procedure.md` line 8: "`execution/_Decomposition/SOFTWARE_DECOMP.md`
  revision 0.7 is the current basis." — confirmed present.
- Frozen `execution/_Decomposition/SOFTWARE_DECOMP.md` header: `revision:
  0.8`, `status: current_basis` — confirmed; §13 gate posture ("This v0.8
  decomposition is the accepted current decomposition basis after SCA-005")
  states downstream surfaces "may be stale relative to revision 0.8 until
  refreshed by their owning workflows" — confirmed.
- Conclusion: the verifier's findings are correct. The original ledger's
  ALIGNED-with-no-note encoding was a discovery miss; under addendum 4
  (widened `STALE_SETUP_SPECIFICATION`: post-alignment drift, one controlled
  value) the Procedure prerequisite "revision 0.7 is the current basis" is a
  current-state declaration false at the frozen SHA, and the Specification/
  Datasheet authority pointers are the same overtaken-pointer species. The
  decomp's own sanction language makes the state permitted-pending-refresh,
  which informs `AuthorityNeeded=NO` and the immateriality note in
  `RemainingWork`, not the controlled disposition.
- EXC-001 ladder: addendum 6 requires "a named human ruling or recorded
  human disposition covering the cited record" for `REVIEWED`; the row's
  evidence is agent direct inspection with `DecisionBasis=NONE_FOUND`, so
  `REVIEWED` was not supported. `UNVERIFIED` is the correct rung (matches
  EXC-003's identical posture and DEL-00-07's package-better-side reading).
  EXC-002's `REVIEWED` rests on named human-approved DEC-008 and is
  unchanged.

**Disagreements with the verifier:** NONE.

**Changes (old → new), bounded to the named surface plus verification of
those rows:**

- DEL-00-02-DECL-001: Disposition ALIGNED → STALE_SETUP_SPECIFICATION;
  DeclaredState now cites the rev-0.7 Interface Commitments pointer;
  ImplementationEvidence `Specification.md` → drift evidence (frozen decomp
  header 0.8; DAG pointer matches, revision pointer does not);
  VerificationEvidence NOT_APPLICABLE → inspection citation (Spec:32 vs
  decomp header); RemainingWork NONE_OBSERVED+note → authority-pointer
  refresh 0.7→0.8 with sanction/immateriality note (prior SEMANTIC_READY/D-40
  note retained). Confidence HIGH and AuthorityNeeded NO unchanged.
- DEL-00-02-DECL-002: Disposition ALIGNED → STALE_SETUP_SPECIFICATION; same
  pattern (Datasheet:32 Required Source Basis pointer); prior
  module_boundaries.md deferral note retained.
- DEL-00-02-DECL-004: Disposition ALIGNED → STALE_SETUP_SPECIFICATION; same
  pattern (Procedure:8 "revision 0.7 is the current basis" false at frozen
  SHA); prior SEMANTIC_READY/D-40 note retained.
- DEL-00-02-EXC-001: SourceReliability REVIEWED → UNVERIFIED (addendum-6
  ladder repair; no disposition, confidence, or histogram impact).

**Owner-calibration caveat:** the verifier's addendum-4 adjudication of the
rev-0.7 pattern to the STALE side is the encoding applied here. If the owner
later calibrates this corpus-wide pattern to ALIGNED-with-note instead, the
three DECL rows above flip back to ALIGNED with the drift note retained; the
drift facts and evidence citations in the rows are ruling-neutral, so that
flip is mechanical.

## Self-flagged rows

- **DEL-00-02-EXC-001** — Judgment call worth reviewer eyes. The deliverable's
  "no implementation code" exclusion holds for its folder (verified: no code
  artifacts; write boundary respected). However, this documentation-only
  deliverable's `_run_records/WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_*`
  are the "primary record" home for the TP-SEAM-WASM-001 module-boundary
  implementation (wasm build seam: crate → browser asset; DEC-020 / ADR-0001),
  whose code landed OUTSIDE the folder (`apps/desktop/...`,
  `core/model_operations/operation_applier`) via a separate Type 1 tranche.
  Disposition kept ALIGNED for the exclusion; flagged for review of whether
  hosting implementation run records under a doc-only architecture deliverable
  is intended. Routed `AuthorityNeeded=REVIEW`.
- **DEL-00-02-REQ-005** — Judgment call the conventions leave open: whether the
  defined review-gate mechanisms satisfy "detect package-local architecture
  drift." Called PARTIALLY_IMPLEMENTED (mechanisms present but drift-detection
  not concretely specified); a reviewer could argue ALIGNED given the
  requirement's evidence bar is "Human review."
- **DEL-00-02-REQ-001/002/003** — Judgment call: these are ALIGNED by-reference
  to AB-00-02 (keyed to DEL-00-02, human-accepted) rather than by restatement
  inside the four-document kit itself; the kit names role categories but does
  not reproduce the AB-00-02 definitions. Flagged so a reviewer can confirm the
  by-reference reading is acceptable.

## Evidence-execution log

- No test/build re-execution performed. DEL-00-02 has no code, test, or
  validation surface of its own; nothing was side-effect-free-runnable and
  nothing required a recorded-pass citation for its own rows.
- Read-only discovery only: file reads under FROZEN, `git -C FROZEN ls-files`
  and `git -C FROZEN status --porcelain` (read-only), grep over R1 indexes and
  SOFTWARE_DECOMP.
- Verified directly at frozen SHA 551f84ef6 (filesystem/`git ls-files`, no
  build): four-document kit present and DEL-00-02-titled; `_SEMANTIC.md` (Audit
  PASS) and `_SEMANTIC_LENSING.md` present; folder contains no code artifacts;
  `docs/architecture/module_boundaries.md` absent while sibling
  `docs/architecture/*` files exist (owned by other PKG-00 deliverables, out of
  DEL-00-02's orbit).
- Frozen-tree `git status --porcelain` empty BEFORE and AFTER all work (exit 0
  both times); HEAD confirmed `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
- Fan-in repair pass (fable re-run): read-only frozen-tree inspection only
  (`sed`/file reads of the DEL-00-02 kit and SOFTWARE_DECOMP.md; no
  re-execution of any build/test); frozen-tree `git status --porcelain`
  empty before and after the repair reads; HEAD re-confirmed
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Repair writes confined to the
  same two W1 output files.

## Convention friction notes

- Documentation-only deliverable vs the §6 note "behavioral alignment requires
  implementation and reproducible verification." That bar targets behavioral
  claims; DEL-00-02's requirements are documentation obligations, so
  "implementation evidence" was read as the documentation content and its
  human acceptance (AB-00-02 / SCA-001), and `ValidationEvidence` is
  `NOT_APPLICABLE` throughout. Flagging the interpretation for consistency
  across other PKG-00 architecture-runway deliverables in this wave.
- ClaimClass for architecture-governance documentation requirements is not
  pinned by the convention set. Applied: DOCUMENTATION for content-authoring
  obligations (REQ-02-01, -04; ACC rows; all DECLARED_STATE rows) and
  GOVERNANCE for boundary/dependency/review-gate obligations and scope
  exclusions (REQ-02-02, -03, -05; EXC rows). Noted for cross-deliverable
  aggregation consistency.
- Lifecycle-target drift: the kit repeatedly names `SEMANTIC_READY` as target /
  completion condition, but the deliverable was rebaselined CHECKING →
  IN_PROGRESS by the D-40 administrative correction (2026-07-11). Per
  conventions this is not a staleness disposition on requirement rows; recorded
  as notes on the affected DECLARED_STATE rows (DECL-001, DECL-004). No
  `LIFECYCLE_REASSESSMENT_REQUIRED` disposition applied (fence: recorded, never
  applied — and here there is no evidence-vs-status technical gap to route).

## Boundary-compliance statement

- All fences held. Discovery was read-only; the only writes are the two W1
  output files (`CLAIM_CONCORDANCE_DEL-00-02.csv`, `NOTES_DEL-00-02.md`) under
  the RUN folder. No FROZEN write (frozen `git status --porcelain` empty before
  and after; no `target/`, `__pycache__`, or `.pytest_cache` created). No
  lifecycle transition, DAG mutation, cross-project edit, or edit to any
  `_STATUS.md`/register/product file. No release-readiness, issuance,
  certification, sealing, professional-approval, or code-compliance claim
  (F-PIP-1..5) appears in either output; the EXCLUSION rows only *record the
  deliverable's own exclusions*, which is descriptive, not a claim. All
  dispositions are agent judgments routed via `AuthorityNeeded`, never phrased
  as owner or engineering rulings.
