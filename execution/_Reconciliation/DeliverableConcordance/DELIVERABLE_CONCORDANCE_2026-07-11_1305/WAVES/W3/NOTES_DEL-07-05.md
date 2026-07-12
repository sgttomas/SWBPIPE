# W3 Notes — DEL-07-05 Results viewer

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W3 (PKG-07).
**Deliverable:** DEL-07-05 "Results viewer" (PKG-07, GUI class, IN_PROGRESS).
**Frozen evidence tree:** `.claude-worktrees/piping-frozen-551f84ef6` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
**Ledger:** `CLAIM_CONCORDANCE_DEL-07-05.csv` — 21 claim rows.
**Binding rules:** `R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13) plus W1 calibration items 1–8 and W2 calibration items 9–15.

This deliverable was the R0b calibration exemplar
(`R0B_CALIBRATION/R0B_CLAIM_CONCORDANCE_DEL-07-05.csv`, 23 rows). It is re-encoded
here as an ordinary W3 member under the CURRENT binding set, which has moved in
several places since the exemplar. Deliberate divergences from the exemplar are
itemized in §6.

## 1. Histograms (recount from the CSV; reproduce exactly)

**Disposition histogram (21 rows):**
- ALIGNED — 15
- STALE_SETUP_SPECIFICATION — 4
- PARTIALLY_IMPLEMENTED — 1
- UNKNOWN — 1

**ClaimType histogram (21 rows):**
- REQUIREMENT — 9
- DECLARED_STATE — 6
- ACCEPTANCE — 3
- REMAINING_WORK — 2
- EXCLUSION — 1

Cross-cut tallies: SourceReliability UNVERIFIED 15 / NOT_APPLICABLE 6;
SelectableUnderCurrentLoop YES 2 (`DECL-005` _STATUS surface row, `REM-001`
ratio residual) / NO 19; AuthorityNeeded NO 16 / OWNER 5;
ClaimClass GUI 11 / DOCUMENTATION 8 / GOVERNANCE 2.

## 2. Row census rationale

- **9 REQUIREMENT rows** (REQ-001..009) — one per current requirement ID
  REQ-07-05-001..009 (re-verified against `Specification.md` at frozen SHA).
  Substance dispositions only (convention 1); no requirement row takes
  STALE_SETUP_SPECIFICATION. REQ-005 = PARTIALLY_IMPLEMENTED (ratio blocked-state
  exists at the core-contract layer, not the UI); the other eight ALIGNED.
- **3 ACCEPTANCE rows** (ACC-001..003) at addendum-12 grain, mapping the
  Verification-table entries that add a *distinct setup-acceptance criterion*:
  VER-001 Document review (kit presence), VER-003 Dependency-register validation,
  VER-004 Semantic setup review. **VER-002 (Boundary review) and VER-005 (Future
  implementation test) merely restate REQ-008 and REQ-009 and are NOT mirrored as
  ACCEPTANCE rows** — their acceptance substance is folded into the REQ-008 and
  REQ-009 VerificationEvidence cells (with an in-cell note). See §6 for the
  divergence from the exemplar (which kept all five).
- **1 EXCLUSION row** (EXC-001) — the durable product-boundary portion of Scope
  paragraph 2 (no solver logic / stress recovery / rule-pack evaluation / report
  generation / protected data). The same sentence's setup-era clause ("does not
  implement GUI source code, tests, schemas, exporter code") is overtaken and
  ledgered on DECL-001, not here (convention-1 two-signal split).
- **6 DECLARED_STATE rows** (DECL-001..006), addendum-1 census: Specification,
  Datasheet, Guidance, Procedure (all STALE_SETUP_SPECIFICATION → OWNER),
  `_STATUS.md` (DECL-005, ALIGNED), `MEMORY.md` (DECL-006, ALIGNED). No
  deliverable-owned in-tree README exists, so none added. `_CONTEXT.md` /
  `_REFERENCES.md` are not census surfaces.
- **2 REMAINING_WORK rows** — REM-001 (the one real ungated residual, the
  governing-ratio views item) and REM-002 (a run-record-disclosed candidate:
  rotational-deformation visualization; UNKNOWN, see §3). The seeded
  `(gated: D-41)` bootstrap item gets NO row (addendum 2); it appears verbatim
  only in the DECL-005 RecordedRemaining cell.
- **0 IMPLEMENTED_UNMAPPED rows.** Every material surface in this deliverable's
  orbit resolves to an owner: `core/gui/results_viewer` → DEL-07-05 (SURF-077);
  `apps/desktop/src/features/results` → DEL-07-05/07-06/08-05 (SURF-041); the
  viewport deformation overlay → SOW-023 graphical review / DEL-07-01;
  `ResultExportPanel.tsx` self-declares DEL-08-04. Nothing unmapped in orbit.

## 3. Self-flagged rows

- **DEL-07-05-REQ-001** — forward-looking-capability-gap judgment (W1 calibration
  item 6). Encoded ALIGNED **at the conditional-contract grain**: the requirement
  is scoped "when those categories are present in validated result envelopes";
  present categories display, and the rule-pack ratio category is vacuously
  satisfied because no ratio rows exist upstream at frozen SHA. The alternative
  (PARTIALLY_IMPLEMENTED, counting the never-exercised ratio category as a viewer
  gap) is defensible; I judged the gap upstream, not in the viewer. Reviewer eyes
  requested on the grain choice.
- **DEL-07-05-REQ-007** — same forward-looking pattern: the "when those envelope
  fields exist" clause (rule-pack name/version/checksum) is vacuous at frozen SHA
  (no rule-pack rows in preview envelopes). Encoded ALIGNED at conditional-contract
  grain rather than PARTIALLY_IMPLEMENTED.
- **DEL-07-05-ACC-001 / ACC-002 / ACC-003 (acceptance-grain judgment)** — I
  dropped VER-002 and VER-005 as ACCEPTANCE rows because they restate REQ-008 /
  REQ-009 (addendum-12 grain; brief census rule). The R0b exemplar mirrored all
  five verification IDs. This is a deliberate convention-driven divergence; a
  reviewer preferring the exemplar's 5-row treatment would re-add EXC/ACC-style
  rows for VER-002 (ALIGNED) and VER-005 (PARTIALLY_IMPLEMENTED). Flagged so the
  fan-in can confirm the grain call.
- **DEL-07-05-REM-002 (rotational-deformation candidate → UNKNOWN)** — the
  prior-question friction from R0b persists under the adopted rules: the item is a
  disclosed limitation in a dated run record (rx/ry/rz emitted, not visualized),
  is NOT in any deliverable's `## Remaining`, and the governing plan
  (`PLAN_2026-06-17_prd_completion.md` §4 line 275) marks FR-015 met for
  "six-DOF directional deformation" without rotational rendering. Under addendum 5
  I held it at UNKNOWN (evidence-backed residual candidate whose status/home cannot
  be resolved from the deliverable's own records) with the smallest next check and
  AuthorityNeeded=OWNER, rather than REMAINING_STATE_MISMATCH. Whether a
  run-record-disclosed limitation is a residual *at all* when the governing plan
  closed the parent requirement without it is the open question.
- **DEL-07-05-DECL-002 (rev-drift + STALE interaction)** — the Datasheet is
  STALE_SETUP_SPECIFICATION on its planning-documents-only declaration (→ OWNER for
  the R5 repair), AND it carries the W1-item-1 authority-pointer drift (cites
  SOFTWARE_DECOMP revision 0.7 / DAG-006; frozen header is revision 0.8
  status current_basis; `_DAG/_LATEST.md` → DAG-007). Pure pointer drift alone
  routes AuthorityNeeded=NO, but the row already routes OWNER for the substantive
  staleness and because the Datasheet's overtaken UI-library TBDs meet the W1-item-1
  "kit also carries overtaken TBD registers" trigger. Drift facts recorded in-row;
  owner-calibration caveat stated once in §5.

## 4. Evidence-execution log

**Re-executed side-effect-free inside the frozen tree** (`PYTHONDONTWRITEBYTECODE=1`;
`git -C FROZEN status --porcelain` verified EMPTY before and after each):
1. `python3 tests/test_results_viewer_contract.py` → PASS (exit 0). Cited on
   REQ-002/003/004/005/006/007/008/009 and EXC-001 with the marker
   `re-executed side-effect-free at frozen SHA 551f84ef6 (... frozen porcelain empty after)`.
2. `python3 tools/validation/validate_dependencies_schema.py "<DEL-07-05 folder>/Dependencies.csv"`
   → VALID (29 required columns, 19 data rows). Cited on ACC-002.

**Git-plumbing byte-identity proof (read-only; no writes).** The sweep commit
`e648462f1d0521e26df15d04a988391343018886` is an ancestor of frozen HEAD, and
`git diff --stat e648462f1..551f84ef6` is EMPTY over every product path this
ledger cites. The diffed path set (this is the scope of the addendum-10
`content-identical at frozen SHA 551f84ef6be656f1603ce0acfa5e3935aa9683c7` qualifier
used in the evidence cells — W1 calibration item 4, each path actually diffed):
- `core/gui/results_viewer`
- `apps/desktop/src/features/results`
- `apps/desktop/src/features/viewport/PipeViewport.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/src/services/unitConversion.ts`, `unitConversion.test.ts`, `previewService.ts`
- `tests/test_results_viewer_contract.py`
- `fixtures/product_preview/invented_mechanics_result.json`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `schemas/results.schema.yaml`
All EMPTY. No exclusion clause is needed because no cited path differs.

**Cited as recorded (not re-executed).** The DEC-025 five-surface sweep
`validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
(commit `e648462f1`; `overall_status=pass`, `desktop_vitest` pass,
`desktop_playwright_e2e` pass) is cited with `not re-executed at frozen SHA
551f84ef6` plus the byte-identity qualifier above. Per the GUI-deliverable rule,
no node/vitest/Playwright/cargo tooling was run (those are not side-effect-free
against the frozen tree); the recorded passes bind product code proven
byte-identical to the frozen SHA. The two recorded boundary reviews on REQ-008
(`_REVIEW.md` DEV-001 PKG-02 audit 2026-05-16; TP-RECON-01) are cited as recorded.

**Re-verified frozen facts** (direct read, side-effect-free): requirement IDs
REQ-07-05-001..009 present; frozen `SOFTWARE_DECOMP.md` header `revision: 0.8`,
`status: current_basis`; `tools/validation/` contains only
`validate_dependencies_schema.py` (`check_four_documents.sh`, `validate_enum.py`
absent — DECL-004); fixture = 830 result rows, 0 rule-pack ratio rows, 45
`global_nodal_rotation_x/y/z` rad rows; `buildDeformationOverlay` consumes only
`global_nodal_displacement_x/y/z` (REM-002); `results.schema.yaml` line 607 lists
`solved_from_global_linear_system` (the R0b schema-basis candidate has landed — no
finding, no row); all DecisionBasis tokens resolve in-tree (DEC-018 decomp §12
line 596; DEC-025 decomp line 603; H4 amendment `_COORDINATION.md` lines 75-76 +
`plans/DRAFT_2026-06-11_H4_coordination_evidence_posture.md`; SCA-001 `_CONTEXT.md`;
Receipt 12 `loop/LOOP_RECEIPTS.md` line 340; FR-015 A6 residual
`PLAN_2026-06-17_prd_completion.md` line 275) — no convention-7 ATTESTED markers
were needed (W2 calibration item 15 not triggered).

## 5. Convention friction / calibration notes

- **W1 calibration item 1 (rev-0.7→0.8 authority-pointer drift) — owner-calibration
  caveat, stated once:** the kit (Datasheet Identification, `_CONTEXT.md`,
  `_REFERENCES.md`, and dated MEMORY 2026-06-04) cites SOFTWARE_DECOMP revision 0.7
  / DAG-006 while the frozen decomp header is revision 0.8 `current_basis` and
  `_DAG/_LATEST.md` → DAG-007. This is pure authority-pointer drift, adjudicated
  STALE-side on the affected census surface row (DECL-002, the Datasheet) with the
  drift facts in-row; the non-census surfaces (`_CONTEXT.md`, `_REFERENCES.md`) and
  the dated MEMORY entry carry the same pointer but are not census DECL rows, so
  they are not separately ledgered. This is an agent observation, not an owner or
  engineering ruling.
- **W2 calibration item 11 (bootstrap `_STATUS` cell scoping):** DECL-005 uses the
  exclusion variant — RecordedRemaining carries both items verbatim (ratio item
  ` ; ` bootstrap item, byte-exact, `§§6–8` preserved), while RemainingSource
  (`PRD plan §4 FR-015 A6 residual`) and GateOrStageConstraint (`UNGATED`) are
  scoped to the non-bootstrap residual only; the bootstrap's gate/source are not
  annotated. Consistent with the sibling W3 ledger DEL-07-02-DECL-005.
- **W2 calibration item 9 (undated MEMORY head):** not triggered — `MEMORY.md`
  carries no undated header/state block; every entry is dated, so the 737→830-row
  drift inside the dated entries is a note on DECL-006, not a staleness disposition
  (addendum 1). Recorded on the row.
- **W2 calibration item 13 (SourceReliability keys to the weakest load-bearing
  leg):** all substance rows key to UNVERIFIED, not REVIEWED. For the GUI/GOVERNANCE
  requirement, acceptance, and exclusion rows, validation is NOT_APPLICABLE (GUI/IP
  class) and the load-bearing legs are implementation + verification whose witnesses
  are agent-generated automated suites (the DEC-025 sweep and the contract test) or
  agent direct-inspection, with no human ruling covering them. The RESOLVED
  ACCEPT_AS_IS human disposition on `PKG07-DEL0705-PKG02-001` covers one PKG-02
  compatibility finding, not the full verification leg, so it does not lift the rows
  to REVIEWED (item 13). DECLARED_STATE prose rows are NOT_APPLICABLE (addendum 6).
- **Convention 6 SECURITY encoding (W1 item 2):** not exercised — no SECURITY-class
  claims in this deliverable.
- **Mechanical selectability (convention 6):** only the `_STATUS` surface row
  (DECL-005) and the ratio REMAINING_WORK row (REM-001) are YES (deliverable
  IN_PROGRESS + one ungated recorded residual). All requirement/acceptance/exclusion
  rows and the non-`_STATUS` DECL rows are NO (no recorded item; addendum 12).
  REM-002 is NO (UNKNOWN candidate, not a recorded `## Remaining` item). The owner
  suspension is a run-level caveat and is not written into any row.
- **Residual open questions** (unchanged from the R0b friction, still uncovered by
  the adopted set): (a) whether a run-record-disclosed limitation is a residual when
  the governing plan closed the parent requirement without it (REM-002); (b) whether
  overtaken *verification* (not validation) passes force STALE_REVIEW_OR_EVIDENCE —
  moot here because the near-frozen DEC-025 sweep plus the empty byte-identity diff
  prove the recorded passes bind the frozen product state.

## 6. Deliberate divergences from the R0b exemplar

1. **SourceReliability:** exemplar used REVIEWED on the requirement/acceptance/
   verification rows; here they are UNVERIFIED per W2 calibration item 13 (weakest
   load-bearing leg is agent-generated). DECLARED_STATE rows stay NOT_APPLICABLE.
2. **SelectableUnderCurrentLoop:** exemplar put YES on REQ-001, REQ-005, VER-005
   (rows the ratio residual "touches"). Here only DECL-005 (`_STATUS`) and REM-001
   carry YES; requirement rows carry NO (addendum 12: NO on rows with no recorded
   item), matching sibling W3 ledgers DEL-07-01/02/03.
3. **RecordedRemaining placement:** exemplar copied the ratio residual verbatim into
   REQ-001/REQ-005/VER-005 RecordedRemaining cells. Here requirement rows carry
   NONE_RECORDED; the verbatim residual text lives only on DECL-005 and REM-001, and
   the residual detail for REQ-005 lives in its RemainingWork column (sibling pattern).
4. **Acceptance grain:** exemplar mirrored all five VER IDs as ACCEPTANCE rows; here
   VER-002 and VER-005 are dropped as mere restatements of REQ-008/REQ-009 (folded
   into those rows), leaving 3 ACCEPTANCE rows (addendum-12 grain; brief census rule).
5. **ClaimID form:** exemplar used mixed `REQ-07-05-NNN` / `VER-07-05-NNN` /
   `DEL-07-05-CNN`; here all rows use the addendum-12 fixed form
   `DEL-07-05-<TYPE>-NNN` (native requirement/verification IDs recorded in
   NormativeSource).
6. **`_STATUS` gate cell:** exemplar annotated `(gated: D-41)` into the gate cell;
   here the exclusion variant (W2 item 11) keeps gate/source scoped to the
   non-bootstrap residual.

## 7. Boundary-compliance statement

- All discovery reads and the two re-executed checks ran inside the frozen worktree;
  `git -C FROZEN status --porcelain` was EMPTY before and after every execution
  batch, and is EMPTY at completion (HEAD still `551f84ef6...`). No file under the
  frozen tree was modified, including git-ignored paths (external redirection env
  set; no build/bytecode written into the tree).
- The only writes of this pilot are the two output files
  `WAVES/W3/CLAIM_CONCORDANCE_DEL-07-05.csv` and `WAVES/W3/NOTES_DEL-07-05.md`. No
  `_STATUS.md`, register, DAG, or product file was touched; no lifecycle transition,
  DAG mutation, or cross-project edit was made.
- No release-readiness, issuance, certification, sealing, professional-approval, or
  code-compliance claim appears anywhere in the outputs (F-PIP-1..5). All
  dispositions are agent judgments routed through AuthorityNeeded (5 OWNER: DECL-001..
  004 for R5 documentation-repair authorization, REM-002 for the residual-status
  question); none is phrased as an owner or engineering ruling. No
  DEFERRED_AGENT_WORKFLOW row was required (no claim resolution depended on agent
  instructions or workflow semantics). No STOP-worthy contradiction was found: the
  D-41 frozen-register `AWAITING_RULING` state is ruling-after-freeze mechanics per
  RUN_BASIS, not re-derived here.
