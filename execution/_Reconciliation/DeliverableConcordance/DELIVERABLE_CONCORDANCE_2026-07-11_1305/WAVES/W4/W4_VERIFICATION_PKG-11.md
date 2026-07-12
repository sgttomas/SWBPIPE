# W4 Fan-in Verification — PKG-11 Documentation, Examples, and Education

Verifier: **highest-available-capability GPT-5**, high-effort adversarial fan-in.
Scope: all five W4 discovery ledgers and notes for DEL-11-01 through DEL-11-05,
verified against the frozen evidence worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Method authority:
`R1_CONVENTIONS.md` Parts A–D, the pinned plan §§6–8, R0/R0b calibration and
review records, all PKG-00..08 carry-forward calibration items, W1–W3 fan-in
exemplars, and the addendum-9 controls recorded in the active run basis.

No discovery ledger or notes file was edited. Findings are agent judgments,
not owner, legal, professional, release, certification, or code-compliance
rulings.

**Verdicts: DEL-11-01 DEFECTIVE · DEL-11-02 DEFECTIVE · DEL-11-03 SOUND ·
DEL-11-04 DEFECTIVE · DEL-11-05 DEFECTIVE.**

Claim-row verification scope: **82 checked rows: 69 PASS / 3 QUALIFIED / 10
FAIL**. This covers every self-flagged row, every non-ALIGNED row, and at least
two ALIGNED rows in each ledger. Nine package-level mechanical/evidence checks
also PASS; one acceptance-grain check is QUALIFIED and one missing-residual
census check FAILS. Combined verification-check total: **78 PASS / 4 QUALIFIED
/ 11 FAIL (93 checks)**.

The FAILs are bounded ledger/notes corrections. None requires product,
governance, lifecycle, DAG, decision, dependency, review-finding, or R4/R5
work. Exact owning-pilot routes are in §4.

## 1. Package-wide mechanical and evidence checks

| # | Check | Result |
|---:|---|---|
| 1 | All 114 CSV rows parse at exactly 20 columns with the W3 byte-exact header; RFC-4180 CRLF throughout | PASS |
| 2 | ClaimIDs are unique, fixed-form, and contiguous within every ClaimType; combined ID cell is `PKG-11/DEL-11-0X` | PASS |
| 3 | All five notes' required ClaimType and Disposition histograms independently recount exactly | PASS |
| 4 | Each bootstrap string is byte-exact to frozen `_STATUS.md`, occurs only in DECL-005, and is excluded from residual/gate/selectability analysis | PASS |
| 5 | Declaration census is six in every ledger: four kit documents + `_STATUS.md` + `MEMORY.md`; no deliverable-owned README exists | PASS |
| 6 | SourceReliability ladder is structurally conformant: all 30 DECL rows `NOT_APPLICABLE`; technical rows weakest-leg `UNVERIFIED` except DEL-11-01 REQ-004/009, whose exact status wording has a named `ACCEPT_AS_IS` human disposition | PASS |
| 7 | All 114 rows are mechanically non-selectable; the only recorded `_STATUS` item in each deliverable is the excluded D-41 bootstrap | PASS |
| 8 | Re-executed with addendum-9 controls: `test_user_guide_status_wording.py` 1/1 and `test_invented_example_models.py` 7/7, total **8 passed**; all five dependency CSVs VALID at 29 columns and respectively 20/15/12/17/11 data rows | PASS |
| 9 | Frozen ignored-aware porcelain showed exactly the six addendum-9 allow-listed paths before and after; no seventh path or tracked change appeared | PASS |
| 10 | ACCEPTANCE minting in DEL-11-02/05: the eight-row encodings preserve eight distinct review/setup obligations, but only DEL-11-05 has a separately headed Acceptance Criteria surface; W3 carry-forward records this as census-shape variance, so totals are not face-comparable | QUALIFIED |
| 11 | DEL-11-02 residual census: current product guide still calls DEC-022's ruled typed-AST grammar `TBD` and names revision 0.7 as current authority, while `_STATUS.md` has no non-bootstrap home | FAIL |

## 2. Per-ledger verification tables

### 2.1 DEL-11-01 User guide skeleton — DEFECTIVE

Checked rows: **9 PASS / 0 QUALIFIED / 3 FAIL**.

| Checked rows | Count | Result | Independent verification |
|---|---:|---|---|
| REQ-001, REQ-004, REQ-009, REQ-010 | 4 | PASS | Product guide has the required workflow sections; canonical status table and external hash-bound human-acceptance boundary are present; focused status test passed. |
| REQ-002 | 1 | PASS | PARTIALLY_IMPLEMENTED is supported: revision 0.7 is called current and several resolved/implemented surfaces remain in the guide's TBD inventory, while other packaging/transport/release details genuinely remain open. |
| DECL-001, DECL-002, DECL-006 | 3 | PASS | Specification/Datasheet product-absence framing is overtaken; the undated MEMORY head retains a current-shaped stale TBD inventory. |
| REM-001 | 1 | PASS | The bounded guide-currentness repair is evidence-backed and absent from `_STATUS.md`; no alternate status home is named. `NONE_RECORDED`/NO selectability follows addenda 2/12. |
| REQ-008 | 1 | FAIL | The exact SECURITY marker is over-applied. This is a documentation-content exclusion checked by direct inspection/recorded scan; accepted scope does not record an owner-gated sufficiency review for the current guide. General human review for unclear future sources is not this row's validation gate. |
| DECL-003 | 1 | FAIL | Guidance contains surviving advisory/conditional principles (“do not present future behavior as implemented unless supported”); product implementation satisfies rather than falsifies that rule. No specific false present-state declaration was identified. W3 Guidance calibration requires ALIGNED-with-note here. |
| DECL-004 | 1 | FAIL | STALE remains defensible, but the encoded reason is wrong: a later separately authorized guide does not falsify a self-scoped setup procedure. The actual stale facts are dead `check_four_documents.sh` / `validate_enum.py` commands and the operational SEMANTIC_READY expectation. Cells must cite those facts. |

### 2.2 DEL-11-02 Developer guide — DEFECTIVE

Checked claim rows: **15 PASS / 0 QUALIFIED / 3 FAIL**, plus the package-level
missing-REM FAIL in §1.

| Checked rows | Count | Result | Independent verification |
|---|---:|---|---|
| REQ-001, REQ-002, REQ-009, REQ-014 | 4 | PASS | Guide architecture, frame-mechanics, evidence-family, and historical setup-fence claims are present at documentation/setup-run grain. |
| ACC-001..006, ACC-008 | 7 | PASS | Each stated aggregate guide/setup check is factually satisfied; setup-vs-later-authoring separation is preserved. Census-grain comparability remains the package QUALIFIED item. |
| EXC-002 | 1 | PASS | Setup-session no-product-write boundary is historical and separately authorized Tranche A publication does not falsify it. |
| DECL-001..003 | 3 | PASS | Specification/Datasheet future-product framing is overtaken; Guidance additionally retains specifically false TBDs for the ruled grammar and selected project license. |
| REQ-012 | 1 | FAIL | DEC-022 ruled the typed AST grammar on 2026-06-11, but `docs/developer_guide/index.md` still lists “rule expression grammar” as a current TBD. The requirement is to leave only unresolved choices TBD; disposition must be PARTIALLY_IMPLEMENTED. |
| ACC-007 | 1 | FAIL | “Unresolved choices remain TBD” is not fully satisfied for the same ruled grammar. It must carry the same partial/currentness defect rather than ALIGNED. |
| DECL-004 | 1 | FAIL | STALE is supportable, but product existence alone does not falsify this explicitly future “produce or refresh” procedure. Re-encode its stale basis around the revision-0.7 authority instruction and overtaken grammar/license TBDs. |

The omitted residual is material: add `REM-001` for guide authority/TBD
currentness, with `REMAINING_STATE_MISMATCH`, `RecordedRemaining` /
`GateOrStageConstraint` = `NONE_RECORDED`, selectability `NO`, and OWNER
routing. This is the same species correctly captured by DEL-11-01 REM-001.

### 2.3 DEL-11-03 Theory notes — SOUND

All **24 rows checked: 21 PASS / 3 QUALIFIED / 0 FAIL**.

| Checked rows | Count | Result | Independent verification |
|---|---:|---|---|
| REQ-001..003, REQ-005..012 | 11 | PASS | Product note preserves educational/no-reliance scope, mechanics/rule/human separation, conceptual centerline/frame and local-FEA boundaries, no silent defaults, qualitative claim grain, and explicit source/TBD limits. |
| REQ-004 | 1 | QUALIFIED | Public-source inventory and claim limits exist; NASA/MIT/Open Textbook license assertions remain recorded agent spot-check evidence and were not independently re-browsed. UNVERIFIED/MEDIUM is correct. |
| ACC-001 | 1 | QUALIFIED | One grouped acceptance row for the distinct final-note coverage/checklist surface is defensible; acceptance-grain variance remains non-comparable across PKG-11. |
| EXC-001..004 | 4 | PASS | Product note excludes design-code authority, protected/proprietary content, formula-level/local-FEA overreach, and setup-session product edits at the correct grains. |
| DECL-001, DECL-002 | 2 | PASS | Explicit future/no-product setup declarations are overtaken. |
| DECL-003 | 1 | QUALIFIED | STALE is supported by the now-overtaken centerline/frame source-selection TBD, while three narrower source scopes remain genuinely deferred; MEDIUM captures the mixed surface. |
| DECL-004..006 | 3 | PASS | Procedure is correctly ALIGNED at historical setup-run grain; Status and dated/current Memory evidence are accurate. |
| REM-001 | 1 | PASS | Human-DEFERRED RF-11-03-C-003 names three bounded source scopes; sole-status-surface omission is correctly REMAINING_STATE_MISMATCH without converting deferral into rejection. |

No SECURITY marker is used, correctly: recorded protected-content review and
bounded source deferral do not create a separate owner-gated security-
sufficiency validation claim.

### 2.4 DEL-11-04 Invented educational examples — DEFECTIVE

Checked rows: **10 PASS / 0 QUALIFIED / 2 FAIL**.

| Checked rows | Count | Result | Independent verification |
|---|---:|---|---|
| REQ-002, REQ-004 | 2 | PASS | Fixtures carry educational/non-reliance and fictional-rule boundaries; focused tests passed. These satisfy the required two independent ALIGNED spots. |
| REQ-003, REQ-006 | 2 | PASS | Independent fixture/test evidence supports substance; each load-bearing PKG-02 finding remains pending human, so MEDIUM + OWNER satisfies addendum 13. |
| DECL-001..004, DECL-006 | 5 | PASS | Product fixtures/tests falsify future/materialization-absent setup declarations; undated MEMORY head is overtaken. Procedure also contains specifically false future-dependency/materialization facts. |
| REM-001 | 1 | PASS | Two `TECHNICALLY_ADDRESSED_PENDING_HUMAN` findings are absent from the sole `_STATUS` Remaining surface; routing does not perform the human disposition. |
| REQ-001, EXC-002 | 2 | FAIL | Both convention-6 markers are over-applied. The rows assert invented/public-safe content boundaries, and focused tests/inspection provide the cited evidence. No accepted row-specific owner-gated sufficiency-review clause exists; the two pending human findings concern source-of-truth mapping and persistence/hash evidence, not protected-content sufficiency. |

Remove the exact marker and OWNER routing on these two rows; do not disturb the
separate OWNER routing on REQ-003/006 or REM-001, which is correctly tied to
the pending PKG-02 dispositions.

### 2.5 DEL-11-05 Contributor onboarding — DEFECTIVE

Checked rows: **14 PASS / 0 QUALIFIED / 2 FAIL**.

| Checked rows | Count | Result | Independent verification |
|---|---:|---|---|
| REQ-001, REQ-002, REQ-009 | 3 | PASS | Contributor orientation, PKG/DEL identity, and historical setup-gate claims are evidenced. |
| ACC-001..008 | 8 | PASS | Direct inspection confirms setup files/lenses/history; dependency CSV revalidated; dead helper pointers do not falsify the achieved presence results. Acceptance census remains covered by the package-level QUALIFIED item. |
| EXC-002 | 1 | PASS | Historical setup fence is satisfied; later Tranche M publication was separately authorized. |
| DECL-001, DECL-002 | 2 | PASS | Specification/Datasheet setup-only and future-publication declarations are overtaken by the contributor guide and repo links. |
| DECL-003 | 1 | FAIL | Guidance is surviving advisory content, not a false present-state declaration. Its “tutorial should” language is satisfied by the current guide; its final conflict note is explicitly scoped to the setup session. W3 Guidance calibration requires ALIGNED-with-note. |
| DECL-004 | 1 | FAIL | STALE remains supportable, but later product publication does not falsify this self-scoped setup procedure. The actual stale operational facts are the three absent helper scripts (`check_min_viable_fileset.sh`, `check_four_documents.sh`, `validate_enum.py`) and the setup-state command assumptions; cells must cite them. |

## 3. Cross-ledger adjudications

### 3.1 SECURITY convention-6 scope

PKG-11 has three marker rows: DEL-11-01 REQ-008 and DEL-11-04 REQ-001 /
EXC-002. All three use the exact em-dash spelling and OWNER routing, but form
alone is insufficient. W3 binding calibration says the marker is limited to a
SECURITY-class behavior claim whose **accepted scope explicitly defers
sufficiency review owner-gated**. These are documentation/fixture content-
boundary rows with direct inspection or focused-test evidence. General human
review language for future unclear sources, or unrelated pending PKG-02
compatibility findings, is not a row-specific owner-gated sufficiency basis.

Result: all three markers are removed by their owning pilots. Other SECURITY-
adjacent PKG-11 rows correctly use `NOT_APPLICABLE` or bounded evidence without
manufacturing a sufficiency gate.

### 3.2 Guidance and Procedure split

W3 calibration is fact-driven:

- Guidance is STALE only when a specific present-state register or factual
  declaration is falsified. DEL-11-02 (ruled grammar/license still TBD),
  DEL-11-03 (centerline/frame source-selection TBD overtaken), and DEL-11-04
  (actual examples declared future/nonexistent) satisfy that test.
- DEL-11-01 and DEL-11-05 Guidance are advisory/conditional principles now
  satisfied by product docs; they become ALIGNED-with-note.
- A historical setup Procedure is not stale merely because separately
  authorized product work later occurred. DEL-11-01/02/05 still have a STALE
  basis, but it is their dead/overtaken operational instructions, not product
  existence. DEL-11-03 correctly uses ALIGNED after replacing the dead helper
  with explicit shell tests; DEL-11-04 remains STALE because its dependency and
  materialization facts are overtaken.

### 3.3 DEL-11-02 currentness residual

DEC-022 is present in frozen `SOFTWARE_DECOMP.md` and resolves the typed AST
grammar. Both the developer guide and its kit Guidance/Procedure continue to
call that grammar TBD, and the guide calls revision 0.7 current while frozen
decomposition is revision 0.8. This affects requirement substance (REQ-012),
its aggregate verification obligation (ACC-007), and sole-surface residual
homing (new REM-001). Treating it only as a note would be inconsistent with
DEL-11-01's correctly encoded currentness species.

## 4. Exact correction routes (owning pilots only)

### DEL-11-01 → owning pilot `/root/w4_del_09_02`

1. REQ-008: replace marker with a bounded `NOT_APPLICABLE` documentation-
   content reason; OWNER → NO; remove “human-gated” RemainingWork wording.
2. DECL-003: STALE → ALIGNED; OWNER → NO; state that advisory/conditional
   principles remain current. Revised histograms: ALIGNED 13, STALE 4,
   PARTIALLY_IMPLEMENTED 1, REMAINING_STATE_MISMATCH 1. Revised authority:
   NO 13 / OWNER 6.
3. DECL-004: keep STALE, but rewrite DecisionBasis/DeclaredState/evidence/
   RemainingWork around absent `check_four_documents.sh` / `validate_enum.py`
   and operational status drift, not later product existence.
4. Update notes self-flags/SECURITY discussion and recounted totals.

### DEL-11-02 → owning pilot `/root/w4_del_09_01`

1. REQ-012 and ACC-007: ALIGNED → PARTIALLY_IMPLEMENTED; cite DEC-022 and the
   guide's false grammar-TBD; route OWNER.
2. Add contiguous `DEL-11-02-REM-001` for revision/TBD guide currentness:
   REMAINING_STATE_MISMATCH, `NONE_RECORDED` gate/source/recorded item,
   Selectable NO, OWNER.
3. DECL-004: keep STALE but cite revision-0.7 and overtaken grammar/license
   instructions rather than product existence alone.
4. Final census/histograms: 31 rows; REQUIREMENT 14 / ACCEPTANCE 8 /
   EXCLUSION 2 / DECLARED_STATE 6 / REMAINING_WORK 1; ALIGNED 24 /
   PARTIALLY_IMPLEMENTED 2 / STALE_SETUP_SPECIFICATION 4 /
   REMAINING_STATE_MISMATCH 1; authority NO 24 / OWNER 7.

### DEL-11-04 → owning pilot `/root/w4_del_09_02`

1. REQ-001 and EXC-002: remove convention-6 marker; use a bounded
   `NOT_APPLICABLE` fixture/document-content reason; OWNER → NO; remove
   human-gated sufficiency wording. Confidence may remain MEDIUM.
2. Keep addendum-13 OWNER routing on REQ-003/006 and REM-001 unchanged.
3. Update notes SECURITY paragraph and authority total to NO 8 / OWNER 8;
   ClaimType/Disposition histograms do not change.

### DEL-11-05 → owning pilot `/root/w4_del_09_01`

1. DECL-003: STALE → ALIGNED; OWNER → NO; encode surviving advisory guidance.
   Revised histograms: ALIGNED 22 / STALE 3; authority NO 22 / OWNER 3.
2. DECL-004: keep STALE, but rewrite its basis/evidence around the three absent
   helper scripts and setup-state command drift, not later product existence.
3. Update notes self-flags and recounted totals.

DEL-11-03 requires no correction.

## 5. Containment and fence statement

Verification read only frozen product/kit/review/run evidence and the five W4
ledger/note pairs. The only write is this fan-in report. Re-executed Python
tests used `PYTHONDONTWRITEBYTECODE=1`, an external `PYTHONPYCACHEPREFIX`, and
pytest `-p no:cacheprovider`; no cargo or `py_compile` ran. Ignored-aware
porcelain remained exactly the six allow-listed addendum-9 paths. No lifecycle,
DAG, register, decision, product, kit, finding, dependency, R4, or R5 surface
was changed. No release-readiness, legal clearance, security assurance,
professional approval, certification, sealing, authentication, or code-
compliance claim is made.
