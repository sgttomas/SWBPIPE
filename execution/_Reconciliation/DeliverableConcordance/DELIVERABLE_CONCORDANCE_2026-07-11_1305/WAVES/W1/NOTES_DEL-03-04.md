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

- ALIGNED = 14
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

- **REQ-001..004, 007, EXC-001/002, DECL-001..006 -> ALIGNED.** The four-document
  kit was refreshed (2026-06-05 evidence reconciliation) to describe the frozen
  implemented slice in present tense; it does **not** carry setup-era
  future-tense prose, so declared-state rows are ALIGNED, not
  `STALE_SETUP_SPECIFICATION`. Requirement substance is implemented in
  `schemas/component.schema.yaml` + fixture and verified by
  `tests/test_component_section_schema.py`, re-executed passing at the frozen SHA
  (see evidence log).
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
