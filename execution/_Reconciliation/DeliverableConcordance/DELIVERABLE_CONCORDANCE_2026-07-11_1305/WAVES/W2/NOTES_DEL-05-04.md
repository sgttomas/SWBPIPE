# W2 Discovery Notes — DEL-05-04 "Analysis status semantics" (PKG-05)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W2 (batch 3). Frozen
evidence tree `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-05-04.csv` (21 claim rows + header).

**NormativeSource path alias (declared once per addendum 12):** bare filenames
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`, `_REVIEW.md`,
`Review_Findings.csv`) resolve to the deliverable folder
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-04_Analysis status semantics/`.
All other paths are repo-relative under `projects/chirality-piping/`.

**Requirement-ID form:** the kit's native scheme is `REQ-05-04-001..014`
(self-identifying); ClaimIDs use the addendum-12 fixed form
`DEL-05-04-REQ-NNN` with the kit-native ID recorded in each row's
`NormativeSource`.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (21 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 18 |
| STALE_SETUP_SPECIFICATION | 2 |
| PARTIALLY_IMPLEMENTED | 1 |

ClaimType histogram (21 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 14 |
| EXCLUSION | 1 |
| DECLARED_STATE | 6 |

Acceptance rows: 0. The Specification "Verification" table restates the
fourteen requirements at test-approach grain (no distinct acceptance criteria,
expected values, or tolerances of its own), and `_REVIEW.md` AC/AP checklist
rows mirror the same requirements; per addendum 12 no mirrored ACCEPTANCE rows
were minted. REMAINING_WORK rows: 0 — `_STATUS.md ## Remaining` carries only
the seeded `(gated: D-41)` bootstrap item, transcribed byte-exact verbatim
into the `_STATUS.md` surface row's `RecordedRemaining` per addendum 2 /
W1 calibration item 5 (gate/source cells left at `NONE_RECORDED` defaults).
IMPLEMENTED_UNMAPPED rows: 0 — every material surface in this deliverable's
orbit carries a deliverable attribution in `IMPLEMENTATION_SURFACES.csv`
(SURF-002 api contract, SURF-116 completeness_checker, SURF-131 solver
diagnostics, SURF-174/175/176/187/202 schemas); none of the 8 R1
`NONE_FOUND`-attribution surfaces (build-readiness panel, telemetry panel,
export unit-disclosure helper, product_preview, workspace manifest, tool
registry, two coordination tools) is status-semantics orbit.

Declared-state census (addendum 1): four kit surfaces + `_STATUS.md` +
`MEMORY.md` = 6 DECL rows. No deliverable-owned in-tree README exists for
DEL-05-04 (the shared crates in its orbit are owned by other deliverables;
`docs/architecture/analysis_status_semantics.md` is a cited product
architecture note, not a README, and is treated as implementation/DeclaredState
evidence rather than a censused kit surface).

**Owner-calibration caveat (stated once per W1 calibration item 1):** the kit
cites SOFTWARE_DECOMP revision 0.7 (`Datasheet.md` References row;
`Procedure.md` Prerequisites row; also `_CONTEXT.md` Decomposition
Reference/Architecture Basis Injection and `_REFERENCES.md`, which are not
censused declared-state surfaces and are recorded here as a note only) while
the frozen decomposition header is revision 0.8, `status: current_basis`.
Encoded STALE-side on the affected DECLARED_STATE surface rows (DECL-002,
DECL-004) per the W1 fan-in adjudication; `AuthorityNeeded=NO` because the
drift is the authority pointer only — the kit's declared TBD registers
(human-acceptance workflow ownership/storage/UI, non-JSON hash
canonicalization, USER_RULE_PASSED vocabulary question) all remain open at the
frozen SHA, so no overtaken-TBD OWNER routing applies.

## 2. Self-flagged rows

- **DEL-05-04-REQ-008** — forward-looking record-design requirement
  (calibration item 6): encoded ALIGNED at record-contract grain because the
  hash-binding contract (`bound_hashes` minItems 1, human-only actor,
  conditional `acceptance_status`) is fully encoded in the frozen schema,
  while "shall not survive content changes" is documented usage semantics not
  mechanically enforceable at JSON-schema grain and not exercised by a
  negative test. A reviewer could defensibly read this as
  PARTIALLY_IMPLEMENTED; I kept the runtime-invalidation residue on REQ-014
  (where the kit itself homes the stale-hash test expectation) to avoid
  double-counting one gap across two rows.
- **DEL-05-04-REQ-010** — grain call (calibration item 6): ALIGNED at
  present-test grain. The anti-collapse tests exist and pass at the frozen
  SHA; the "future release gates" clause is not yet exercisable because no
  release gates exist and release readiness is an explicit kit exclusion.
  Grain stated in-row; MEDIUM confidence.
- **DEL-05-04-REQ-014** — judgment call: PARTIALLY_IMPLEMENTED. Negative
  automatic-approval/compliance tests exist and pass, but I found no
  stale-hash acceptance-reuse negative test anywhere in the frozen suite
  (the only related assertion is that `bound_hashes` is a required member),
  despite the kit's own verification approach ("Negative tests for automatic
  approval/compliance plus stale-hash acceptance reuse") and Procedure's
  fixture-selection bullet ("cannot survive bound-content hash changes")
  naming one. The requirement is forward-gated, so ALIGNED-at-contract-grain
  was arguable; I encoded the present-day coverage gap because the kit's own
  verification cells imply the negative test should exist now.
- **DEL-05-04-EXC-001** — single merged EXCLUSION row covering the
  Specification Scope sentence's five excluded areas (lifecycle promotion,
  release readiness, GUI acceptance workflow ownership, report rendering
  acceptance, professional approval). They are one sentence with one shared
  posture and one evidence base; splitting felt like row inflation, but
  reviewer eyes welcome.
- **DEL-05-04-DECL-006** — MEMORY census judgment: MEMORY.md is entirely
  dated log entries (no undated current-state header block). I included the
  row because the latest dated entries carry still-current boundary
  declarations; the 2026-06-16 vs 2026-06-17 date discrepancy between the
  K-CONFLICT-1 ruling's recorded header reversal and MEMORY's Lifecycle
  Housekeeping entry is recorded in-row as a historical note per addendum 1,
  never a staleness disposition.

## 3. Evidence-execution log

Re-executed at the frozen SHA (addendum 9; all read-only, no build/bytecode
artifacts; `PYTHONDONTWRITEBYTECODE=1`; pytest `-p no:cacheprovider`;
`git -C <frozen worktree> status --porcelain` empty BEFORE and AFTER, HEAD
verified `551f84ef6be656f1603ce0acfa5e3935aa9683c7`):

1. `python3 -m pytest -p no:cacheprovider tests/test_analysis_status_schema.py
   tests/test_analysis_boundary_schema.py tests/test_results_schema.py
   tests/test_api_boundary_contract.py` — **6 passed, 0 failed** (matches the
   2026-06-05 lifecycle-readiness review's recorded 6-test collection).
2. `python3 tests/test_api_boundary_contract.py` (direct invocation) —
   exit 0. This exercises the script-style `main()` with 37 static asserts
   that VERIFICATION_INDEX row PY-15 records as having **no** pytest-collected
   tests and no recorded direct-invocation execution; this re-execution
   closes that gap for the rows citing the API contract (REQ-007/010/013/014).
   No CARGO_TARGET_DIR was needed — no cargo invocation was made.
3. `python3 tools/validation/validate_dependencies_schema.py
   "<deliverable>/Dependencies.csv"` — VALID (29 required columns, 10 data
   rows), matching the `_REVIEW.md` DS-001 recorded counts.

Cited as recorded (not re-executed):

- DEC-025 sweep `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (python_pytest pass, clean tree, commit `e648462f1d05…` ancestor of frozen
  SHA) — carried beside the re-execution as the recorded pass. Because the
  suites were re-executed directly at the frozen SHA, no addendum-10
  content-identical qualifier was needed or written on any row.
- 2026-06-05 lifecycle-readiness review snapshot
  `execution/_Reconciliation/Reviews/REV_DEL-05-04_2026-06-05_2053/` and its
  IP-001/PB-001 focused scans — recorded evidence, `not re-executed at frozen
  SHA 551f84ef6` (REQ-011, EXC-001).
- Human rulings resolved in the frozen tree: PKG05 blocker-closure ruling
  packet `PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md`
  (Review_Findings `DEL-05-04-PKG02-I001` = ACCEPT_AS_IS/RESOLVED — no
  addendum-13 pending-disposition cap applies) and K-CONFLICT-1
  `LIFECYCLE_CORRECTION_2026-07-02_2050/Decision_Log.md` (DECL-005
  DecisionBasis).

SourceReliability: REQUIREMENT/EXCLUSION rows are REVIEWED under addendum 6 —
the named 2026-06-05 human blocker-closure ruling accepted the
lifecycle-readiness recommendation grounded on exactly the schema/API/test
slice these rows cite, and the surfaces are unchanged in the frozen tree since
the 2026-06-05/06-18 tranches per the deliverable's own records. DECL rows are
NOT_APPLICABLE per addendum 6.

## 4. Convention friction notes

- **One gap, two candidate homes (REQ-008 vs REQ-014):** the missing
  stale-hash acceptance-reuse negative test could be charged to REQ-008
  ("shall not survive content changes") or REQ-014 ("prevent … stale
  human-acceptance reuse"). The conventions do not say where a shared
  evidence gap should live when two requirements overlap; I homed it on
  REQ-014 (whose verification approach names the negative test) and kept
  REQ-008 at record-contract grain. Both rows self-flagged.
- **Forward-gated requirements in a solver-mechanics package:** REQ-010 and
  REQ-014 constrain "future release gates" that cannot exist yet (release
  readiness is excluded and F-PIP fences bar readiness claims). Calibration
  item 6's ALIGNED-vs-PARTIALLY split covers the capability grain but not the
  case where the gate's *absence* is itself the declared design; a standing
  rule for "requirement whose subject artifact is future-by-design" would
  remove per-pilot judgment.
- **Architecture-note census status:** `docs/architecture/analysis_status_semantics.md`
  is deliverable-authored (frontmatter `deliverable_id: DEL-05-04`,
  `status: draft`) but is neither a four-document kit surface nor a README,
  so addendum 1 gives it no DECL row; its content is fully consistent with
  the frozen schema, so nothing was lost — but a deliverable whose primary
  product artifact is a prose doc sits slightly outside the census taxonomy.

## 5. Boundary compliance

- All discovery reads were from the frozen worktree at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; porcelain verified empty before
  and after the only re-executions (read-only Python checks; no cargo run, no
  frozen-tree writes, git-ignored paths included).
- Writes were confined to exactly two files:
  `WAVES/W2/CLAIM_CONCORDANCE_DEL-05-04.csv` and `WAVES/W2/NOTES_DEL-05-04.md`
  (plus the generator script `build_csv_DEL-05-04.py` in the session
  scratchpad outside both trees).
- No lifecycle transition, DAG mutation, register edit, `_STATUS.md` edit, or
  cross-project edit. No release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim appears in either output
  (F-PIP-1..5 held). All dispositions are agent discovery judgments, not
  owner or engineering rulings; no authority routing beyond `AuthorityNeeded`
  cells (all NO for this ledger). `SelectableUnderCurrentLoop` cells are
  mechanical only (no recorded non-bootstrap items → NO per addendum 12); the
  owner suspension remains a run-level caveat.

## 6. Post-fan-in amendment (2026-07-12)

- **REQ-014 evidence-pointer addition (verifier provenance:
  `W2_VERIFICATION_PKG-05.md`, DEL-05-04 check 2 and section 7 item 6).**
  The W2 fan-in verification found the REQ-014 evidence inventory
  under-reported two declaration-level const-flag asserts present in the
  frozen tree. Re-verified independently by this owning pilot at frozen SHA
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7` before editing:
  `tests/test_persistence_schema.py` lines 236–239 (inside
  `check_schema_contract`, pytest-collected via
  `test_project_persistence_schema_contract`) asserts `HumanAcceptanceRef`
  requires `invalidates_on_hash_change` with `const` true, and
  `tests/test_model_state_schema.py` line 164 (inside `main`,
  pytest-collected via `test_model_state_schema_contract`) asserts
  `ImmutabilityPolicy.hash_invalidates_external_acceptance` `const` true.
  The REQ-014 `VerificationEvidence` cell was extended to cite both, with
  the explicit qualifier that these are declaration-level checks (the
  schemas declare the invalidation semantic), not the missing stale-hash
  acceptance-reuse negative test. No other cell or row was changed; the
  disposition remains PARTIALLY_IMPLEMENTED and the confidence remains
  MEDIUM. Histograms in section 1 are unaffected (no disposition or
  ClaimType changed). Frozen-tree porcelain verified empty before and after
  the re-verification reads.
