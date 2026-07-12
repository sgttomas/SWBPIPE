# NOTES — DEL-00-06 Diagnostics, warning, and result-envelope contract (R2 wave W1)

Frozen tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Binding rules:
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13). Ledger:
`CLAIM_CONCORDANCE_DEL-00-06.csv` (17 rows, 20 columns, RFC-4180).

DEL-00-06 is a PKG-00 architecture-runway, documentation-only deliverable
(`DATA_MODEL_CHANGE`, lifecycle `IN_PROGRESS`, prior `SEMANTIC_READY`). It
defines the diagnostics/warning/result-envelope architecture contract; it
produces no product code of its own. Its governing architecture basis
`AB-00-06` (SOFTWARE_DECOMP.md line 431, authorized by human-accepted
`SCA-001`) is nearly verbatim with the deliverable's requirements, which is
why the requirement rows carry `SourceReliability=REVIEWED` and `ALIGNED`.

## Requirement ID scheme mapping (non-self-identifying scheme REQ-06-*)

The deliverable's requirement IDs are `REQ-06-0N`; the addendum-12 `ClaimID`
form disambiguates against the deliverable ID (`DEL-00-06`):

| Requirement ID | ClaimID |
|---|---|
| REQ-06-01 | DEL-00-06-REQ-001 |
| REQ-06-02 | DEL-00-06-REQ-002 |
| REQ-06-03 | DEL-00-06-REQ-003 |
| REQ-06-04 | DEL-00-06-REQ-004 |
| REQ-06-05 | DEL-00-06-REQ-005 |

## 1. Histograms (recomputed from the CSV)

Disposition histogram:

| Disposition | Count |
|---|---|
| ALIGNED | 14 |
| STALE_SETUP_SPECIFICATION | 3 |
| (all other dispositions) | 0 |

ClaimType histogram:

| ClaimType | Count |
|---|---|
| REQUIREMENT | 5 |
| ACCEPTANCE | 3 |
| EXCLUSION | 3 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 0 |
| IMPLEMENTED_UNMAPPED | 0 |
| **Total** | **17** |

Census rationale:
- 5 REQUIREMENT rows: one per current requirement ID (REQ-06-01..05).
- 3 ACCEPTANCE rows at addendum-12 grain: the acceptance-specific criteria
  that are not durable exclusions — kit-existence (ACC-001), TBD-visibility
  (ACC-002), semantic-artifact existence (ACC-003).
- 3 EXCLUSION rows: the durable scope exclusions — no in-folder implementation
  (EXC-001), no PKG-01..12 authorization (EXC-002), no protected/proprietary
  data (EXC-003). The Specification Acceptance Criteria bullets on "no
  implementation code" and "no protected data" are encoded as these exclusion
  rows rather than mirrored again as ACCEPTANCE rows, to avoid double-counting
  the same substance (matches the sibling PKG-00 ledgers' pattern).
- 6 DECLARED_STATE rows: one per four-document-kit surface (Specification,
  Datasheet, Guidance, Procedure) plus `_STATUS.md` and `MEMORY.md`
  (addendum 1). No deliverable-owned in-tree README exists (verified), so no
  additional DECLARED_STATE row. `_SEMANTIC.md`/`_SEMANTIC_LENSING.md` are not
  four-document-kit surfaces and are covered by ACC-003, not their own rows.
- 0 REMAINING_WORK rows: the only `## Remaining` entry is the seeded
  `(gated: D-41)` concordance bootstrap item, recorded verbatim in the
  `_STATUS.md` surface row's `RecordedRemaining` and excluded from all
  residual/gate/selectability analysis (addendum 2). R1 inventory confirms
  `NonBootstrapItems=NONE`, `GateSuffixes=NONE`, `SelectableUnderCurrentLoop=NO`.
- 0 IMPLEMENTED_UNMAPPED rows: see §3.

## 2. Calibration context — convention-3 homing of the DEL-04-01-C18 residual

The dispatch flagged that R0's `DEL-04-01` ledger (row `DEL-04-01-C18`) recorded
a possible omitted "final result-envelope integration across service
boundaries" residual whose owning deliverable was not named, listing
`DEL-04-06`/`DEL-00-06` as candidate homes, and directed convention-3 residual
homing before any `REMAINING_STATE_MISMATCH`.

Homing result for DEL-00-06 (no `REMAINING_STATE_MISMATCH` fired):
- DEL-00-06's `_STATUS.md ## Remaining` does **not** carry a result-envelope
  integration residual; its only entry is the D-41 concordance bootstrap item.
- DEL-00-06 is the contract-definition deliverable, not an integration owner.
  The result-envelope *format/integration* is an SCA-001/SOW-061-permitted TBD
  ("exact schema syntax and localization policy remain TBD"), and the
  cross-service *integration* is downstream implementation scope: the material
  surfaces (`schemas/results.schema.yaml`, `schemas/headless_runner.schema.yaml`,
  `core/reporting/result_export`, `core/runner/headless`) are attributed to
  downstream implementation deliverables (DEL-08-04 et al.), not to this
  architecture-runway deliverable.
- DEL-00-06's own `MEMORY.md` (2026-05-17 TP-VERIFY-013E) already classified the
  envelope crossings as `READY_FOR_RUNTIME_TRACE_TRANCHE` /
  `READY_FOR_SECTION_EVIDENCE_SCHEMA_TRANCHE` / `KEEP_AS_TBD` — future-tranche
  implementation items, not omitted DEL-00-06 `## Remaining` residuals.

Conclusion: the C18 candidate residual is homed to downstream implementation
scope and to permitted/future-tranche TBDs, not to a missing DEL-00-06 residual.
Per convention 3 this is a cross-reference resolution, so no finding is raised on
DEL-00-06. Recorded on `DEL-00-06-REQ-003` `RemainingWork` and here.

## 3. Unmapped-surface note (no IMPLEMENTED_UNMAPPED rows)

Every material surface in this deliverable's orbit that R1
`IMPLEMENTATION_SURFACES.csv` co-attributes to DEL-00-06 (SURF-101
physical_to_analytical, SURF-114 result_export, SURF-121 headless runner,
SURF-187 headless_runner.schema.yaml, SURF-202 results.schema.yaml, SURF-226/228
benchmark crates) already carries a multi-deliverable `DeliverableAttribution`;
none are `NONE_FOUND`. They are shared crates/schemas owned at named-slice grain
by downstream implementation deliverables. DEL-00-06's co-attribution is via
`_run_records` (TP-DIAG-019 / TP-VERIFY-013E boundary reconciliation), i.e.
architecture-contract provenance, not code ownership. Therefore no
`IMPLEMENTED_UNMAPPED` row is warranted for DEL-00-06 (addendum 8; the surfaces
are mapped).

## 4. Self-flagged rows

- **DEL-00-06-REQ-003** (result envelopes; ClaimClass call): classed `REPORTING`
  per addendum 7's explicit result-envelope mapping. An `SCHEMA`/`INTEROP`
  reading (the requirement asks producers to adopt the envelope *structure*
  across layers) is also defensible. Disposition unaffected.
- **DEL-00-06-REQ-004** (failure states; ClaimClass call): classed `WORKFLOW`
  per convention D5 (diagnostics/failure-state behavior default) since the five
  states are status/workflow distinctions; a `SCHEMA` reading is defensible.
- **DEL-00-06-REQ-001 / -002** (SourceReliability=REVIEWED): the human approval
  of the TP-DIAG-019 diagnostic-vocabulary ruling is attested in this
  deliverable's own `MEMORY.md`/run record (executed in the parent/orchestrator
  thread), not in an independent decision-register disposition. `REVIEWED` rests
  primarily on the human-accepted `AB-00-06`/`SCA-001` architecture basis, which
  is verbatim with these requirements; TP-DIAG-019 is corroborating. Reviewer
  may prefer to weight TP-DIAG-019 as attestation-level.
- **DEL-00-06-DECL-001 / -002 / -004** (decomp-revision drift; re-encoded in
  fan-in repair): the kit cites `SOFTWARE_DECOMP` revision 0.7 as current basis
  (Specification.md:32, Datasheet.md:33, Procedure.md:8) while the frozen tree
  is revision 0.8 / `status: current_basis`. Now dispositioned
  `STALE_SETUP_SPECIFICATION` per the W1 fan-in adjudication of addendum 4
  (post-alignment drift; one controlled value) — see `## Fan-in repair` below.
  The decomp ~line 655 sanction ("may be stale relative to revision 0.8 until
  refreshed by their owning workflows") is carried as immateriality context in
  `RemainingWork`, not as grounds for `ALIGNED`. The original discovery pass
  kept these `ALIGNED` on a basis-of-record reading that the adjudication
  rejected (the Procedure line declares a fact about the current authority
  basis, not about the docs-only slice).
- **DEL-00-06-DECL-002** (Datasheet anticipated artifacts): the Datasheet lists
  `docs/architecture/diagnostics_contract.md` and a result envelope schema as
  "Outputs Expected From This Deliverable", but neither exists in the tree and
  the Specification's acceptance boundary requires only the document kit
  (no implementation). Read as forward-looking downstream-implementation outputs
  (runway framing), not a false current-state claim, so `ALIGNED` at MEDIUM
  confidence rather than a mismatch. Reviewer eyes welcome on whether the
  Datasheet wording should be a refresh candidate.
- **Interpretive call (all REQ rows)**: for a documentation-only architecture
  obligation, "the contract is defined in the kit and the human-accepted basis"
  is treated as the implementation, and the pending deliverable-level Human
  Review Gate (Specification) is treated as a lifecycle gate, not a
  per-requirement technical gap. Requirement rows therefore carry
  `AuthorityNeeded=NO`; the pending acceptance review is routed via the
  ACCEPTANCE rows' `AuthorityNeeded=REVIEW`.

## 5. Evidence-execution log

- **Re-executed (side-effect-free):** none beyond read-only inspection. No test
  suites were re-run. `CARGO_TARGET_DIR`/`PYTHONDONTWRITEBYTECODE` precautions
  were prepared but not needed (no builds/tests executed against the frozen
  tree). `PYTHONDONTWRITEBYTECODE=1` was set for the CSV-generator script, which
  runs entirely outside the frozen tree (in scratch).
- **Read-only inspection at frozen SHA 551f84ef6:** deliverable kit + `_STATUS`,
  `MEMORY`, `_CONTEXT`, `_REFERENCES`, `_DEPENDENCIES`, `_run_records/**`;
  `SOFTWARE_DECOMP.md` (revision 0.8; AB-00-06 line 431; SOW-061; SCA-001);
  directory listings confirming (a) no in-folder code, (b) no deliverable README,
  (c) `docs/architecture/diagnostics_contract.md` absent, (d) corroborating
  downstream surfaces present (`schemas/results.schema.yaml`,
  `schemas/headless_runner.schema.yaml`, `core/solver/diagnostics`,
  `core/reporting/result_export`, `core/runner/headless`).
- **Cited as recorded (not re-executed):** TP-DIAG-019 validation runs
  (`tests/test_results_schema.py`, `tests/test_headless_runner_contract.py`,
  adapter/transform pytest, `result_export`/`headless`/benchmark crate tests,
  `git diff --check`) — carried with the marker `not re-executed at frozen SHA
  551f84ef6`.
- **Frozen-tree porcelain:** `git -C <FROZEN> status --porcelain` empty before
  the run and empty after (see §6). No ancestor-commit `content-identical`
  qualifier was used (not needed).

## 6. Convention friction notes

- **Documentation-obligation dispositioning.** The §7 disposition set and the
  "behavioral alignment requires implementation + reproducible verification"
  rule are written for behavioral/mechanics deliverables. For a documentation
  architecture-runway deliverable whose requirements are satisfied by kit +
  human-accepted basis and whose acceptance evidence is a pending human review,
  `ALIGNED` is the best fit but "applicable evidence agree" is interpreted as
  "the human-accepted basis governs the claim" rather than "the acceptance
  review has completed." The sibling PKG-00 ledgers resolve this the same way.
- **Source-metadata staleness (RESOLVED by fan-in adjudication).** The original
  discovery pass argued a superseded decomp-revision citation (0.7 vs frozen
  0.8) "has no clean home" among the §7 dispositions and recorded it as a
  self-flagged note under `ALIGNED`. The W1 fan-in verification adjudicated the
  pattern under addendum 4: "revision 0.7 is the current basis" is a
  current-state declaration false at the frozen SHA — post-alignment drift,
  `STALE_SETUP_SPECIFICATION`, one controlled value. The decomp's own sanction
  language (permitted-pending-refresh) informs `AuthorityNeeded=NO` and
  immateriality notes, not the controlled disposition. The ledger now encodes
  this on DECL-001/-002/-004, matching DEL-00-01/03/08.
- **Near-all-ALIGNED result.** Unlike GUI/solver deliverables where setup prose
  contradicts landed code, DEL-00-06's setup/architecture prose still describes
  the frozen slice (documents only, no code, TBDs surfaced). The only staleness
  that fires is the corpus-wide rev-0.7 authority-pointer drift on the three kit
  surfaces that carry it (DECL-001/-002/-004); the remaining 14 rows are
  `ALIGNED`.

## 7. Boundary-compliance statement

- Discovery was **read-only** outside the two output files
  (`CLAIM_CONCORDANCE_DEL-00-06.csv`, `NOTES_DEL-00-06.md`), both under
  `RUN/WAVES/W1/`. No writes anywhere under the frozen tree (including
  git-ignored paths); the CSV generator ran in scratch.
- **Frozen tree clean:** `git status --porcelain` empty before and after.
- **No lifecycle transition applied** (none proposed; `_STATUS.md` untouched).
  No DAG mutation, no cross-project edit, no register/product-file edit.
- **F-PIP-1..5 held:** no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim appears in the ledger or
  notes. REQ-06-05 is encoded as the deliverable's own no-certification
  obligation, not as a compliance assertion by this audit.
- Dispositions are **agent judgments**, routed via `AuthorityNeeded`
  (REVIEW/NO here), never phrased as owner or engineering rulings. No
  `DEFERRED_AGENT_WORKFLOW` items arose.

## Fan-in repair (fable re-run)

The W1 fan-in verification (`W1_VERIFICATION_PKG-00.md`, package finding 1 and
the DEL-00-06 section) found this ledger DEFECTIVE on the corpus-wide rev-0.7
authority-pointer drift: DECL-001/DECL-002 noted + self-flagged the drift but
kept `ALIGNED` (wrong side under the addendum-4 adjudication), and DECL-004's
Procedure.md:8 drift ("revision 0.7 is the current basis") was uncaptured even
as a note. The all-ALIGNED histogram was an artifact of the wrong-side
encoding. Owner-ruled repair: re-run by a fable pilot, who now owns the changed
claims.

**Independent re-verification at frozen SHA 551f84ef6 (before re-encoding):**
Specification.md:32 cites "revision 0.7"; Datasheet.md:33 cites "revision 0.7
for package and deliverable authority"; Procedure.md:8 declares "revision 0.7
is the current basis"; frozen `SOFTWARE_DECOMP.md` header is `revision: 0.8` /
`status: current_basis`; the ~line-655 sanction language ("may be stale
relative to revision 0.8 until refreshed by their owning workflows") is
present. All verifier facts confirmed; **no disagreement** with the defect
findings or the adjudicated encoding.

**Rows re-encoded (old → new):**

| ClaimID | Old | New |
|---|---|---|
| DEL-00-06-DECL-001 | `ALIGNED` / MEDIUM (drift noted, self-flagged) | `STALE_SETUP_SPECIFICATION` / HIGH; inspection evidence added to `VerificationEvidence`; `RemainingWork` = authority-pointer refresh 0.7→0.8 with sanction-as-immateriality note |
| DEL-00-06-DECL-002 | `ALIGNED` / MEDIUM (drift noted in passing) | `STALE_SETUP_SPECIFICATION` / HIGH; same pattern (Datasheet.md:33); anticipated-artifacts self-flag retained as a non-staleness note |
| DEL-00-06-DECL-004 | `ALIGNED` / HIGH (rev drift uncaptured; only SEMANTIC_READY wording noted) | `STALE_SETUP_SPECIFICATION` / HIGH; Procedure.md:8 drift captured; SEMANTIC_READY completion-condition note retained |

All other 14 rows unchanged. `AuthorityNeeded` stays `NO` on all three rows
(the decomp sanction makes the refresh a mechanical owning-workflow action, not
a ruling need). Histograms in §1 recounted from the repaired CSV
(ALIGNED 14, STALE_SETUP_SPECIFICATION 3).

**Owner-calibration caveat (flip risk):** the adjudication carries an explicit
caveat — if the owner later calibrates this corpus-wide pattern to
ALIGNED-with-note instead of STALE, these three rows flip back to `ALIGNED`
(with the drift notes retained). The drift facts are now captured in-row on all
three surfaces, so the flip in either direction is mechanical.
