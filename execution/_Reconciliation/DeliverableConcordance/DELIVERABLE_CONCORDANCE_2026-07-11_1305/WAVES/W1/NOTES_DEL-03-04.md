# NOTES — DEL-03-04 Branch connection component model fields (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W1/CLAIM_CONCORDANCE_DEL-03-04.csv` (15 rows).

## Requirement-ID normalization (RQ -> REQ mapping)

The Specification uses `DEL-03-04-RQ-00N`; normalized to the addendum-12 ClaimID
form `DEL-03-04-REQ-00N`. Mapping is 1:1 identity:

- DEL-03-04-RQ-001 -> DEL-03-04-REQ-001 (branch data fields: geometry/reinforcement/user SIF-flexibility)
- DEL-03-04-RQ-002 -> DEL-03-04-REQ-002 (user-supplied/private, not bundled public defaults)
- DEL-03-04-RQ-003 -> DEL-03-04-REQ-003 (missing values -> explicit diagnostics, not silent defaults)
- DEL-03-04-RQ-004 -> DEL-03-04-REQ-004 (provenance/source fields)
- DEL-03-04-RQ-005 -> DEL-03-04-REQ-005 (unit-aware / dimensionally checked)
- DEL-03-04-RQ-006 -> DEL-03-04-REQ-006 (public/private boundary; no-bypass)
- DEL-03-04-RQ-007 -> DEL-03-04-REQ-007 (validation tests included)

## 1. Histograms (recount from the CSV)

Disposition histogram (15 rows):

- ALIGNED = 11
- STALE_SETUP_SPECIFICATION = 3
- PARTIALLY_IMPLEMENTED = 1

ClaimType histogram (15 rows):

- REQUIREMENT = 7
- EXCLUSION = 2
- DECLARED_STATE = 6

(ACCEPTANCE = 0, REMAINING_WORK = 0, IMPLEMENTED_UNMAPPED = 0.)

## Census rationale

- **7 REQUIREMENT rows** — one per current requirement ID; substance dispositions
  only (convention 1); no requirement row takes `STALE_SETUP_SPECIFICATION`.
- **0 ACCEPTANCE rows** — the Specification requirements table carries an inline
  per-requirement "Verification" column that merely restates the verification
  method for each requirement; there is no separately enumerated acceptance/
  verification-ID surface. Per the addendum-12 grain, restating verification
  tables do not get mirrored ACCEPTANCE rows.
- **2 EXCLUSION rows** — the durable product-boundary exclusions from the
  Specification Scope: EXC-001 (no protected standards content/proprietary data;
  no bundled public defaults) and EXC-002 (no certification/sealing/approval/
  authentication/compliance claim). The other Scope-list items ("this
  evidence-reconciliation pass excludes product implementation code; repo-level
  schema edits; fixture or test edits; dependency/review/DAG/coordination/
  lifecycle/status edits") are pass write-scope statements, not durable product
  exclusions; they are folded into the Specification declared-state row, not
  given their own EXC rows.
- **6 DECLARED_STATE rows** — one per four-document kit surface (Specification,
  Datasheet, Guidance, Procedure) plus `_STATUS.md` and `MEMORY.md` (addendum 1).
  No deliverable-owned in-tree README exists, so no extra README declared-state
  row. `_REVIEW.md`/`Review_Findings.csv` are review artifacts (not READMEs and
  not kit surfaces); they are cited as evidence on requirement rows, not given
  declared-state rows.
- **0 REMAINING_WORK rows** — the deliverable's `## Remaining` contains only the
  seeded `(gated: D-41)` bootstrap item, recorded verbatim in the `_STATUS.md`
  declared-state row's `RecordedRemaining` and excluded from residual/gate/
  selectability analysis (addendum 2). No non-bootstrap residual is recorded
  (consistent with R1: DEL-03-04 is not among the 26 deliverables carrying
  non-bootstrap remaining items).
- **0 IMPLEMENTED_UNMAPPED rows** — `schemas/component.schema.yaml`,
  `fixtures/component/invented_component_library_valid.json`, and
  `tests/test_component_section_schema.py` are this deliverable's mapped surfaces.
  The sibling section schema/fixtures and the bend/rigid/expansion component
  families that share `component.schema.yaml` map to other PKG-03 component
  deliverables, not this one, and are not unmapped material surfaces.

## Disposition rationale (key rows)

- **REQ-001..004, 007, EXC-001/002, DECL-001/005/006 -> ALIGNED.** The
  four-document kit was refreshed (2026-06-05 evidence reconciliation) to
  describe the frozen implemented slice in present tense; requirement substance
  is implemented in `schemas/component.schema.yaml` + fixture and verified by
  `tests/test_component_section_schema.py`, re-executed passing at the frozen
  SHA (see evidence log).
- **DECL-002/003/004 -> STALE_SETUP_SPECIFICATION** (fan-in repair; see the
  repair section below). Each of Datasheet.md L85–87, Guidance.md Review
  Posture (L58–64), and Procedure.md step 6 (L39–40) declares the two PKG-02
  review findings still conceptually `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with
  `HumanDisposition=TBD` and the CSV "not edited" — contradicted at the frozen
  SHA by `Review_Findings.csv` (PKG03-DEL-03-04-PKG02-001/002 both
  `ACCEPT_AS_IS`/`RESOLVED`) and the `_STATUS.md` History 2026-06-05 Gate C
  record (package Gate C run record
  `_run_records/WORKING_ITEMS_RUN_2026-06-05_HUMAN_DISPOSITION_GATE_C.md`,
  which covers DEL-03-04). This is post-alignment drift under the widened
  convention-1 definition (addendum 4): the surfaces' review-disposition
  declaration no longer describes the frozen state. Identical pattern to the
  DEL-03-05 DECL-001..004 encoding under the same Gate C record. The rest of
  each surface (implemented-slice descriptions, named verification command)
  remains accurate — that is recorded in `RemainingWork`, not a downgrade of
  the staleness call. Specification.md carries no such prose (grep-verified),
  so DECL-001 remains ALIGNED. MEMORY.md's pending-disposition language sits
  inside dated entries (2026-05-16, 2026-06-05) — historical per addendum 1;
  DECL-006 already discloses the then-pending -> Gate C-accepted sequence.
- **REQ-006 -> PARTIALLY_IMPLEMENTED.** The schema-level public/private boundary
  and no-bypass property (additionalProperties=false, required privacy_class/
  provenance/completeness, no `default` keys) is implemented and schema-tested.
  But the requirement's own stated verification method — "Architecture and
  service-boundary tests verify validation paths are used" — is recorded as
  deferred future evidence in `_REVIEW.md` (DEL-02-04 no-bypass constraints
  documented; adapter/service tests future, `PASS_WITH_DEFERRED_EVIDENCE`). A
  bounded portion of the requirement's verification is therefore open. The
  deferred service-boundary verification is DEL-02-04 (cross-package) scope,
  routed per convention 3 as a cross-reference rather than adjudicated here.
  `AuthorityNeeded=REVIEW`.
- **REQ-005 -> ALIGNED (substance) with a dependency-bookkeeping residual note.**
  Unit-awareness is directly implemented and tested (ComponentQuantityDimension
  is a subset of the accepted DimensionId vocabulary; retired dimensions
  excluded; COMPONENT_UNIT_MISSING/UNIT_CONTRACT_ERROR diagnostics). The related
  PKG-02 audit finding (DEL-02-02 unit-dependency row `SatisfactionStatus=TBD`)
  was human-dispositioned `ACCEPT_AS_IS/RESOLVED` at Gate C 2026-06-05; the open
  dependency row is bookkeeping, distinct from the implemented unit-awareness,
  and is noted in `RemainingWork`.

## SourceReliability rationale

Per addendum 6: DECLARED_STATE prose rows are always `NOT_APPLICABLE`.
Requirement/exclusion rows resting on the project-original agent-generated
schema/fixture/test evidence with agent audit but no covering human disposition
are `UNVERIFIED` (REQ-001/002/004/007, EXC-001/002). Rows whose cited record is
covered by a named human disposition are `REVIEWED`: REQ-003 (Gate C ACCEPT_AS_IS
on PKG03-DEL-03-04-PKG02-002, diagnostic class/source envelope), REQ-005 (Gate C
ACCEPT_AS_IS on PKG03-DEL-03-04-PKG02-001, unit dependency), and REQ-006 (SCA-001
architecture-basis injection plus recorded Gate C review). Note: the two PKG-02
findings passed through `TECHNICALLY_ADDRESSED_PENDING_HUMAN` (addendum 13) but
reached a human disposition (ACCEPT_AS_IS/RESOLVED) at Gate C, so the addendum-13
Confidence cap no longer binds.

## 2. Self-flagged rows

- **DEL-03-04-REQ-003** — ClaimClass judgment call. Encoded `SCHEMA` (the claim is
  realized as the schema's diagnostic-code enum + `completeness_rules` + the
  no-`default` schema property for a data-model deliverable). R0b convention 5
  maps "diagnostics/warning behavior" to `WORKFLOW`; that convention targets
  runtime diagnostic emission (e.g., a results viewer), whereas here the
  diagnostic is a static schema-contract element. Flagged for reviewer eyes on
  SCHEMA-vs-WORKFLOW.
- **DEL-03-04-REQ-005** — ClaimClass judgment call. Encoded `SCHEMA` (schema
  unit-metadata/dimension-vocabulary structure). Addendum 7 maps "numeric/unit
  correctness" to `MECHANICS`; per the dispatch domain note, numeric unit/
  mechanics correctness is downstream PKG-04/05 scope, and this data-model
  deliverable only asserts schema unit-awareness. Flagged for SCHEMA-vs-MECHANICS.
- **DEL-03-04-REQ-006** — Disposition judgment call (ALIGNED vs
  PARTIALLY_IMPLEMENTED). Chose PARTIALLY_IMPLEMENTED because the requirement's
  own verification method (architecture/service-boundary tests) is deferred
  (DEL-02-04), even though the schema-level boundary is fully implemented and
  tested. A reviewer may prefer ALIGNED-with-cross-reference if the
  service-boundary verification is judged wholly out of this deliverable's slice.
- **DEL-03-04-DECL-002/003/004** — value-label judgment call (fan-in repair
  rows). Encoded `STALE_SETUP_SPECIFICATION` (widened convention-1 definition
  covers post-alignment drift), matching the DEL-03-05 DECL-001..004 treatment
  of the identical overtaken review-disposition prose under the same Gate C
  record. A reviewer weighing STALE vs STALE_REVIEW_OR_EVIDENCE vs
  ALIGNED-with-note (the DEL-03-08 Guidance treatment) has the drift facts
  in-row either way; cross-package census consistency with the sibling ledger
  drove the choice. `AuthorityNeeded=REVIEW` for the kit-prose refresh routing.
- **DEL-03-04-EXC census** — judgment call on splitting durable product
  exclusions (EXC-001 IP/no-defaults, EXC-002 no professional claims) from the
  reconciliation-pass write-scope statements (folded into DECL-001). Flagged in
  case a reviewer wants the pass write-scope items as their own EXC rows.

## 3. Evidence-execution log

Re-executed side-effect-free (addendum 9) with `PYTHONDONTWRITEBYTECODE=1`,
`CARGO_TARGET_DIR` not needed (no Rust build), pytest `-p no:cacheprovider`:

- `python3 -m pytest tests/test_component_section_schema.py -p no:cacheprovider`
  -> `2 passed` (test_component_section_schema_contract and
  test_component_section_jsonschema_validation_helper). The jsonschema helper
  ran (not skipped), so the component fixture was instance-validated against the
  schema. This underwrites REQ-001..007, EXC-001/002, DECL-001/DECL-004.
- Frozen-tree porcelain: `git -C <FROZEN> status --porcelain` empty BEFORE and
  AFTER the pytest run; HEAD confirmed `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

Cited-as-recorded (not re-executed) evidence:

- MEMORY.md 2026-06-21 branch app-absorption (DEC-045) product-physics Rust
  tests / desktop Vitest / Playwright e2e — cited on REQ-006 with the marker
  `not re-executed at frozen SHA 551f84ef6` (downstream app surface, outside this
  data-model deliverable's re-executable slice).
- Gate C human disposition 2026-06-05 (Review_Findings.csv ACCEPT_AS_IS/RESOLVED;
  _STATUS.md History) — cited as DecisionBasis on REQ-003/REQ-005; both records
  resolve in the frozen tree.
- Fan-in repair cross-check (direct inspection at frozen SHA 551f84ef6):
  Datasheet.md L85–87, Guidance.md L58–64, Procedure.md L39–40 pending-
  disposition prose read directly; `Review_Findings.csv` both rows
  `ACCEPT_AS_IS`/`RESOLVED`; `_STATUS.md` History 2026-06-05 Gate C line;
  package Gate C run record
  `_run_records/WORKING_ITEMS_RUN_2026-06-05_HUMAN_DISPOSITION_GATE_C.md`
  (accepts the 6 listed DEL-03-04/05/06 findings) — cited as DecisionBasis on
  DECL-002/003/004; all records resolve in the frozen tree. `grep` sweep
  confirmed `TECHNICALLY_ADDRESSED_PENDING_HUMAN` appears in Datasheet,
  Guidance, Procedure, MEMORY (dated entries), and the dated 2026-06-05 task
  run record, but NOT in Specification.md. Read-only inspection; no
  re-execution needed; porcelain re-verified empty after the repair pass.

## 4. Convention-friction notes

- **Diagnostics/unit ClaimClass at the data-model grain.** Convention 5
  (diagnostics -> WORKFLOW) and addendum 7 (unit correctness -> MECHANICS) were
  written for runtime/solver surfaces. For a schema/data-model deliverable whose
  diagnostics and unit-awareness are static schema-contract elements, `SCHEMA`
  reads more faithfully. Recorded as REQ-003/REQ-005 self-flags; the domain note
  (mechanics formulation correctness -> PKG-04/05) supports routing numeric
  correctness downstream.
- **Requirement whose verification method names cross-package tests (REQ-006).**
  The conventions give no explicit disposition for a requirement that is
  substantively implemented at this deliverable's grain but whose stated
  verification depends on another deliverable's (DEL-02-04) deferred tests.
  Handled as PARTIALLY_IMPLEMENTED + convention-3 cross-reference + REVIEW.
- **Scope list mixing pass write-scope and product exclusions.** The
  Specification Scope "excludes" bullet list conflates reconciliation-pass write
  restrictions with durable product-boundary exclusions; only the latter were
  ledgered as EXCLUSION rows.

## 5. Boundary-compliance statement

- All fences held. Discovery was read-only outside the two output files; no
  lifecycle transition applied; no DAG mutation; no cross-project edit; no edit
  to any `_STATUS.md`, register, or product file.
- F-PIP-1..5 respected: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim is made anywhere in these
  outputs (auditing that the deliverable makes no such claim is not itself such a
  claim).
- All dispositions are agent judgments routed via `AuthorityNeeded`; none is
  phrased as an owner or engineering ruling. No `LIFECYCLE_REASSESSMENT_REQUIRED`
  or `DEFERRED_AGENT_WORKFLOW` disposition was needed.
- Frozen evidence tree verified clean (porcelain empty) before and after all
  re-execution; HEAD at the pinned SHA. Writes confined to
  `WAVES/W1/CLAIM_CONCORDANCE_DEL-03-04.csv` and `WAVES/W1/NOTES_DEL-03-04.md`.

## Fan-in repair (fable re-run)

W1 fan-in verification (`W1_VERIFICATION_PKG-03.md`, DEL-03-04 verdict:
DEFECTIVE) named a discovery miss: the ledger's DECL-002/003/004 were silent
`ALIGNED`/`NONE_OBSERVED` despite overtaken review-disposition prose in the
frozen kit. Bounded re-run by a fable repair pilot (owner-ruled); the repair
pilot now owns the changed claims.

**Independent re-verification (before re-encoding), frozen SHA 551f84ef6:**

- `Datasheet.md` L85–87: "Review findings remain conceptually
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`; the CSV
  was not edited by this reconciliation" — read directly; present.
- `Guidance.md` Review Posture (heading L58, prose L60–64): same pending
  declaration for the unit-dependency and diagnostic-envelope findings —
  present. (Verifier cited L59–62; the section spans L58–64 by direct count —
  same passage, no substantive disagreement.)
- `Procedure.md` step 6 (L39–40): "Do not edit `Review_Findings.csv`; keep its
  findings conceptually `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with
  `HumanDisposition=TBD`" — present.
- `Review_Findings.csv`: PKG03-DEL-03-04-PKG02-001 and -002 both
  `HumanDisposition=ACCEPT_AS_IS`, `Status=RESOLVED`.
- `_STATUS.md` History 2026-06-05: Gate C acceptance of both findings as
  `ACCEPT_AS_IS`/`RESOLVED`; package run record
  `_run_records/WORKING_ITEMS_RUN_2026-06-05_HUMAN_DISPOSITION_GATE_C.md`
  confirms Gate C accepted the 6 listed DEL-03-04/05/06 findings.
- `Specification.md`: no pending-disposition prose (grep-verified) — DECL-001
  re-checked, stays ALIGNED.

**Verification outcome: AGREE with the verifier.** The drift is real and is
the same pattern DEL-03-05 encodes as STALE_SETUP_SPECIFICATION under the same
Gate C record. Disagreements: NONE (line-range bookkeeping on the Guidance
passage only, noted above).

**Rows changed (old -> new):**

| ClaimID | Old | New |
|---|---|---|
| DEL-03-04-DECL-002 | ALIGNED / HIGH; DecisionBasis NONE_FOUND; RemainingWork = revision-0.7 authority-cadence note only; AuthorityNeeded NO | STALE_SETUP_SPECIFICATION / HIGH; DecisionBasis = Gate C run record 2026-06-05; DeclaredState/evidence now carry the Datasheet L85–87 overtaken prose + Review_Findings.csv contradiction; RemainingWork = refresh L85–87 to ACCEPT_AS_IS/RESOLVED (revision-0.7 note retained); AuthorityNeeded REVIEW |
| DEL-03-04-DECL-003 | ALIGNED / HIGH; DecisionBasis NONE_FOUND; RemainingWork NONE_OBSERVED; AuthorityNeeded NO | STALE_SETUP_SPECIFICATION / HIGH; DecisionBasis = Gate C run record 2026-06-05; DeclaredState/evidence now carry the Review Posture (L58–64) overtaken prose + contradiction; RemainingWork = refresh Review Posture; AuthorityNeeded REVIEW |
| DEL-03-04-DECL-004 | ALIGNED / HIGH; DecisionBasis NONE_FOUND; RemainingWork NONE_OBSERVED; AuthorityNeeded NO | STALE_SETUP_SPECIFICATION / HIGH; DecisionBasis = Gate C run record 2026-06-05; DeclaredState/evidence now carry the step 6 (L39–40) overtaken keep-pending direction + contradiction; RemainingWork = refresh step 6 (pytest re-execution evidence retained); AuthorityNeeded REVIEW |

Rows NOT changed: all REQ/EXC rows, DECL-001 (re-checked, no overtaken prose),
DECL-005, DECL-006 (MEMORY pending-language is inside dated entries;
addendum-1 note treatment already in place). Histograms above recounted from
the repaired CSV (11 ALIGNED / 3 STALE_SETUP_SPECIFICATION /
1 PARTIALLY_IMPLEMENTED; ClaimType histogram unchanged). CSV header and
20-column contract re-validated (csv module parse: 16 lines x 20 fields).
Repair-pass fences: reads confined to the frozen tree (read-only) and run
folder; writes confined to the same two output files; frozen-tree porcelain
verified empty after the repair (no re-execution was performed in this pass).
