# R0 Calibration Review — Independent Reviewer Pass

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 / R0_CALIBRATION (activation D-41/DEC-073)
**Reviewer posture:** adversarial method-calibration review per plan §8 R0 — authority precedence, technical-evidence boundaries, disposition consistency, false positives. All findings and recommendations below are agent-authored, non-binding method-calibration input for the owner. Nothing here is an owner or engineering ruling; no lifecycle, DAG, scope, or workflow change is proposed as operative.
**Evidence source state:** all verification reads and re-executions performed against the frozen read-only worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; frozen tree verified clean (`git status --porcelain` empty) after this review's read-only re-executions. No pilot artifact and no frozen-tree file was modified.

---

## 1. Scope and sample

Reviewed artifacts (all six, in full):

- `R0_CLAIM_CONCORDANCE_DEL-04-01.csv` (35 rows) + `R0_NOTES_DEL-04-01.md`
- `R0_CLAIM_CONCORDANCE_DEL-10-05.csv` (18 rows) + `R0_NOTES_DEL-10-05.md`
- `R0_CLAIM_CONCORDANCE_DEL-12-02.csv` (32 rows) + `R0_NOTES_DEL-12-02.md`
- `RUN_BASIS.md`

Method authority read at the frozen revision: plan §§3, 5–8 of `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md`; shared kernel `docs/DELIVERABLE_CONCORDANCE_METHOD.md` (Revision 1, RATIFIED).

Row-count and histogram arithmetic re-verified for all three ledgers: DEL-04-01 disposition histogram in the notes (12/11/4/3/2/2/1) reproduces exactly from the CSV; DEL-12-02 histogram (15/12/3/1/1) reproduces exactly; DEL-10-05 row-type breakdown (9/1/1/2/3/2) reproduces exactly. All 85 rows use only §7 controlled disposition values and §6 controlled enum values for `ClaimType`, `ClaimClass`, `SourceReliability`, `Confidence`, and `AuthorityNeeded`.

Spot-check sample: 30 citation checks across the three ledgers (11 / 8 / 11), weighted toward `ALIGNED` and `IMPLEMENTED_UNDOCUMENTED` rows per tasking. Checks included file existence, exact-line content verification, enum/token comparison, JSON witness content counts, decision-row text comparison, and read-only re-execution of the checks DEL-12-02 claimed to have re-executed.

---

## 2. Citation spot-check results

**Tally: 28 PASS, 2 QUALIFIED (citation does not resolve in the frozen tree; recorded-attestation only), 0 FAIL.**

| # | Row | Cited evidence checked | Verdict |
|---|---|---|---|
| 1 | 04-01 REQ-001 (STALE_SETUP) | Spec line 21 "Future implementation shall model…"; `frame_kernel` 36 `#[test]`; hand-calc witnesses (`cantilever_tip_force.md`, `portal_frame_sway.md`, `branch_assembly.md`); benchmarks crate 33 tests | PASS — all verbatim/counts exact |
| 2 | 04-01 REQ-004 (STALE_SETUP) | Spec line 24 "exact convention is TBD"; `inclined_member_transform.md` witness | PASS |
| 3 | 04-01 REQ-009 (ALIGNED) | README `## Non-Compliance Boundary` (line 38); `_REVIEW.md` DEL-02-03 PASS (line 31); `contains_forbidden_reliance_claim` in benchmarks crate (line 587) | PASS |
| 4 | 04-01 REQ-010 (ALIGNED) | `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` is the newest sweep at the frozen SHA | PASS |
| 5 | 04-01 C02 (STALE_SETUP) | Datasheet lines 30–31: "Solver numerical library | TBD", "Tolerance policy | TBD" vs DEC-023/DEC-026 rulings | PASS (see §6 for a claim-granularity caveat) |
| 6 | 04-01 C04 (ALIGNED) | `core/solver/straight_pipe` separate crate exists | PASS |
| 7 | 04-01 C17 (REMAINING_STATE_MISMATCH) | README `## Current Limitation` declares live-solve-boundary binding "remains TBD"; `_STATUS.md ## Remaining` carries no such item | PASS — mismatch is real |
| 8 | 04-01 C19 (ALIGNED) | `_STATUS.md` Remaining item 1 verbatim; `core/product_physics/src/lib.rs` line 6644 `pressure_thrust_treatment=straight_chord_axial_end_forces` provenance string | PASS |
| 9 | 04-01 UNMAPPED-01 (IMPLEMENTED_UNDOCUMENTED) | `core/solver/sparse_direct` exists; zero mentions of `sparse_direct` in Specification.md and Datasheet.md; DEC-050/DEC-053 observation JSON present under `validation/benchmarks/` | PASS |
| 10 | 04-01 UNMAPPED-02 (IMPLEMENTED_UNDOCUMENTED) | `core/solver/curved_bend` exists; zero curved-bend mentions in Spec/Datasheet; `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` witness exists | PASS |
| 11 | 04-01 C01 DecisionBasis (`DEV-001 dispatch`, path `execution/_Coordination/DEV-001_DISPATCH_DEL-04-01.md`) | File does not exist anywhere in the frozen tree (`find` over the whole checkout); only `MEMORY.md` (2026-05-01) attests it | **QUALIFIED** — see §3 finding A2 |
| 12 | 10-05 R-03 (ALIGNED) | `tp_runner_015_final_cli_validation_blocking.json`: exactly one blocking diagnostic `HEADLESS_RUNNER_LOAD_BASIS_MISSING` ("must identify load-case or combination basis references"); "Exit: 1" is in the run record (TP-RUNNER-015 line 70), not the witness artifact | PASS — with the note that exit codes bind to the run record, correctly within the row's citation set |
| 13 | 10-05 R-06/R-08 (ALIGNED) | `[[bin]] openpipestress-runner`; stub-diagnostic code at bin lines 241/548; witness `policy` block (`network_allowed: false`, `telemetry_allowed: false`, `daemon_allowed: false`) | PASS |
| 14 | 10-05 C11 (ALIGNED) | Solve witness `result_refs` counted directly: exactly 822; run-record exits 0/1/1 for solve/blocking/benchmark-stub | PASS |
| 15 | 10-05 C13/C14 ACCEPTED_DIVERGENCE basis | D-33 packet line 88 evidence-stub clause ("…benchmark/regression evidence stub if the operation is not fully implemented…"); DEC-065 §12 row matches the use | PASS — decision-backed deferral genuinely held |
| 16 | 10-05 C15 (STALE_REVIEW_OR_EVIDENCE) | 822 in committed witness vs `loop/LOOP_RECEIPTS.md` line 246 "live runner solve emits 830 result_refs vs 822" | PASS |
| 17 | 10-05 C16 (STALE_SETUP) | Spec line 32 "exact schema files TBD"; `schemas/headless_runner.schema.yaml` named canonically in `docs/SPEC.md` (schema table) | PASS |
| 18 | 10-05 UNMAPPED-01 (IMPLEMENTED_UNDOCUMENTED) | `apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx` exists | PASS |
| 19 | 10-05 UNMAPPED-02 (IMPLEMENTED_UNDOCUMENTED) | `fn validate_result_with_optional_envelope_payload` at `core/runner/headless/src/lib.rs` line 561 — exact | PASS |
| 20 | 12-02 REXC-REQ-004 (ALIGNED) | Schema `RedactionAction`-family enums (`warning_only`, `redact_value`, `redact_field`, `omit_field`, `block_export`) + `explicit_local_private_intent`/`local_private_action` members present | PASS |
| 21 | 12-02 REXC-REQ-007 (ALIGNED) | Reviewer re-ran `pytest tests/security/test_redaction_export_controls.py -q -p no:cacheprovider` at the frozen SHA: **11 passed** — pilot's re-execution claim independently reproduced | PASS |
| 22 | 12-02 REXC-VER-002 (ALIGNED) | Reviewer re-ran `tools/validation/validate_dependencies_schema.py`: VALID, 29 columns, 18 data rows, exit 0 | PASS |
| 23 | 12-02 REXC-VER-003 (ALIGNED) | Reviewer re-ran grep: 0 matches for `MatrixError`/`MATRIX_ERROR` in `_SEMANTIC.md` | PASS |
| 24 | 12-02 REXC-VER-004 (ALIGNED) | Reviewer counted lens-coverage data rows programmatically: exactly 96 | PASS |
| 25 | 12-02 C04/C05 (STALE_SETUP) | Spec Scope line 7 "still does not … integrate runtime report/export routes" vs `App.tsx` line 1403 `<RedactionExportControlsPanel model={model} />` (exact line) | PASS |
| 26 | 12-02 C06 (IMPLEMENTED_DIFFERENTLY) | Datasheet lines 28–29 uppercase vocabularies (`WARN_ONLY`…`ALLOW_PRIVATE_EXPORT`; `SHARED_REDACTED`, `PUBLIC_TEMPLATE`) vs implemented schema `ExportContext` enum (`public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private`) — token sets genuinely differ | PASS |
| 27 | 12-02 C07 (STALE_SETUP) | Guidance lines 81/86: REXC-OI-002 and REXC-OI-007 still TBD/unwired vs landed panel binding | PASS |
| 28 | 12-02 C08 (ALIGNED) | `_STATUS.md ## Remaining` item 1 verbatim match, incl. PR #167 and source suffix | PASS |
| 29 | 12-02 UNMAPPED-01 (IMPLEMENTED_UNDOCUMENTED) | `ExportReviewPanel.tsx` carries `unit_policy_summary` (line 1165) and `unit_evidence_matrix` (line 1233); no REXC requirement scopes unit-evidence classification (nearest is REXC-REQ-012's no-bypass "unit … controls" clause, which does not map this display surface) | PASS — near-miss to REXC-REQ-012 noted; the pilot's `AuthorityNeeded=OWNER`, no-absorption handling is correct |
| 30 | 12-02 DecisionBasis "DEV-001 rev 0.5 Tranche A (commit abdecbd)" (on ~11 rows) | `git cat-file -t abdecbd` fails in the frozen repository — the commit is not resolvable in the object store (plausibly squash-merged away). The value traces to the deliverable's own records, so it is a transcribed recorded value, not invention | **QUALIFIED** — see §3 finding A2 |

Additional verifications outside the numbered sample: `D-41 | AWAITING_RULING` at the frozen SHA and zero occurrences of `DEC-073` anywhere in the frozen register/decomposition — all three pilots' activation-basis observation is exactly right; `SOFTWARE_DECOMP.md` §12 last codification is `DEC-072`, matching RUN_BASIS. All three `_STATUS.md ## Remaining` blocks match the ledgers' `RecordedRemaining` text verbatim, including the `(gated: D-41)` bootstrap item each pilot correctly excluded and quoted in notes.

---

## 3. Authority-precedence findings

**A1 — Decision citations are accurate.** Every `DecisionBasis` decision ID sampled was read in `SOFTWARE_DECOMP.md` §12 at the frozen SHA and says what the row uses it for: DEC-023 (Option C in-repo skyline LDLT), DEC-025 (five-surface sweep, hosted CI deferred), DEC-026 (T-C tiered tolerance classes, analytic 1.0e-9 seed), DEC-044 ("new PKG-04 integration tranche bridging DEL-04-04 and DEL-04-01" — genuinely deliverable-less, exactly as UNMAPPED-DEL0401-03 uses it), DEC-046, DEC-050 (Option B sparse evidence lane, dense default), DEC-053, DEC-054 (D-27 conditional R4 gate advancing to R5), DEC-065 (D-33 O-A verbs/process policy), DEC-070 (O-B curved-bend macro-element, user-entered factors), DEC-018. No row stretches a decision beyond its text. The strongest case is DEL-10-05 C13/C14: the ACCEPTED_DIVERGENCE rests on an explicit clause in the ruled packet (stub permitted "if the operation is not fully implemented yet") — this is the model the method intends.

**A2 — Two decision-basis citations do not resolve in the evidence tree (the QUALIFIED rows).** (i) DEL-04-01 cites `execution/_Coordination/DEV-001_DISPATCH_DEL-04-01.md` (via C01 and as "DEV-001 dispatch" on ~9 rows); the file does not exist anywhere in the frozen tree — only `MEMORY.md` attests it. (ii) DEL-12-02 cites "commit abdecbd" as the Tranche A basis; that object is not resolvable in the frozen repository. Neither pilot flagged the non-resolvability. These are attestation-level bases (second-hand, via MEMORY/run records), not verifiable decision records. **This matters for boundary 1**: the DEV-001 dispatch is the load-bearing "human authorization" that lets DEL-04-01 treat implemented-kernel-vs-setup-exclusion as staleness rather than `AUTHORITY_CONFLICT` or unauthorized absorption. The dispositions chosen (STALE_SETUP with `AuthorityNeeded=OWNER`; UNMAPPED rows at `SCOPE_CHANGE`) remain conservative, so no boundary-1 violation occurred — nothing was absorbed and no acceptance was manufactured — but at wave scale an unresolvable basis must be visibly downgraded, not silently cited (see convention 7).

**A3 — `HumanDisposition` taken on column faith.** DEL-04-01's two ACCEPTED_DIVERGENCE rows (REQ-007, C12) rest on `Review_Findings.csv` `HumanDisposition=ACCEPT_AS_IS / Status=RESOLVED` (verified present, dated 2026-06-05). The `ReviewerID` on that row is `TASK-PACKAGE-AUDIT-PKG04-PKG02` — a task identifier. The human-ness of the disposition rests entirely on the column's semantics. Acceptable as a recorded disposition, but the method should state what record suffices as evidence of a human act (convention 7).

**A4 — `DecisionBasis` doing double duty.** DEL-12-02 cites DEC-054 (a stage-advancement gate) on twelve requirement rows as context for the E4 tranche, alongside the Tranche A dispatch. DEC-054 does not accept the redaction slice specifically; because those rows are all PARTIALLY_IMPLEMENTED (never ACCEPTED_DIVERGENCE), no acceptance was inferred — but the column mixes "decision that governs this claim" with "stage context in which work occurred." A convention should require the former (convention 7).

**A5 — Activation-basis time-split handled correctly and consistently.** All three pilots independently found D-41 `AWAITING_RULING`/no DEC-073 at the frozen SHA, all three correctly declined to record `AUTHORITY_CONFLICT` (time-separated states, not contradictory sources), and all three surfaced it in notes. RUN_BASIS's frozen-tree note anticipates this. No pilot invented precedence anywhere; the three `AUTHORITY_CONFLICT: none` declarations were each accompanied by the near-candidates and the reasoning, which is the right shape.

---

## 4. Technical-evidence-boundary findings

**T1 — Equation-source boundary: clean.** Zero references to `domains/piping-design` and zero PDF/OCR-sourced artifacts in any evidence chain across all 85 rows (grep-verified). All DEL-04-01 mechanics validation cites in-repo project-original hand-calc witnesses, witness-bound benchmark fixtures at the DEC-026 analytic tier, and the dense-oracle/hand-checked-LDLT parity chain — verified to exist.

**T2 — Verification/validation distinction: held, with one inconsistent row-pattern.** No row promotes a unit test alone to validation; DEL-04-01's mechanics requirement rows all carry hand-calc/witness/benchmark validation distinct from the crate test suite. However, DEL-04-01 REQ-008 is classed `MECHANICS` with `ValidationEvidence=NOT_APPLICABLE`. §6 requires mechanics claims to carry validation *or an explicit statement that validation remains open*; `NOT_APPLICABLE` asserts a third thing — that no validation is required — which contradicts the class. Either the class is wrong (the pilot itself flagged the MECHANICS/REPORTING/WORKFLOW ambiguity for diagnostics-behavior claims) or the cell should state why validation does not apply. One-row defect; a class rubric fixes it (convention 5).

**T3 — SECURITY rows: no compliance/assurance language anywhere** (grep-verified; every "compliance/certification" hit in the ledgers is a quoted exclusion or an explicit disclaimer). But the two ledgers with SECURITY rows apply **divergent validation conventions**: DEL-10-05 gives SECURITY rows populated `ValidationEvidence` (DEC-025 sweep artifacts; witness policy blocks) and `ALIGNED`; DEL-12-02 records `NONE_FOUND` with an explicit deferred-sufficiency note and deliberately declines `VERIFIED_NOT_VALIDATED`. Both are internally reasoned; they are not comparable across deliverables. This is exactly the "ruled checking-entry profile for SECURITY_CONTROL deliverables" question DEL-12-02's notes raise (convention 6).

**T4 — Recorded-pass vs re-executed: disclosed everywhere, but three different practices** (see divergence 6). No pilot presented a recorded pass as a frozen-SHA execution; DEL-12-02's re-execution claims were independently reproduced by this review (11 passed; VALID/29/exit 0; 0 MatrixError matches; 96 lens rows) and the frozen tree remained clean.

---

## 5. Cross-ledger consistency divergences

Each divergence is real (verified in the CSVs, not just the notes) and is followed by the single convention this reviewer recommends for R1+ scale-out. **All recommendations are non-binding method-calibration input for the owner.**

**D1 — Stale-prose vs implemented-substance precedence (three different conventions; the largest comparability defect).**
- DEL-04-01 puts `STALE_SETUP_SPECIFICATION` on the requirement row itself even where the substance is implemented, verified, and witness-validated (11 of 35 rows).
- DEL-12-02 puts the substance disposition (`PARTIALLY_IMPLEMENTED`/`ALIGNED`) on requirement rows and ledgers the prose lag on separate declared-state rows (C04/C05/C07).
- DEL-10-05 sits between: substance disposition on requirement rows, one aggregate framing-staleness row (C12).
Consequence: identical underlying situations produce a "documentation-crisis" histogram for DEL-04-01 and an "implementation-breadth" histogram for DEL-12-02. Package summaries computed from claim rows (§7) will not be comparable across waves.
**Convention 1:** adopt the two-signal split as the single rule — requirement rows always carry the substance disposition; declaration staleness is ledgered on separate per-surface declared-state claim rows; `STALE_SETUP_SPECIFICATION` is reserved for those declaration rows. This requires clarifying §7 `ALIGNED`'s "declaration" term to mean the deliverable's current declared-state surfaces, not the requirement sentence's own tense. (If the owner prefers DEL-04-01's row-level precedence instead, that is equally workable — but exactly one rule must be named before R1.)

**D2 — Declared-state claims have no `ClaimType`.** DEL-04-01 folded declared-state statements into `DeclaredState` cells of nearby rows; DEL-10-05 and DEL-12-02 force-fit standalone stale prose to `ClaimType=REQUIREMENT` with `ClaimClass=DOCUMENTATION`/`SCHEMA`. All three notes independently request the same fix.
**Convention 2:** owner-amend §6 to add a `DECLARED_STATE` ClaimType (or explicitly codify the fold-in rule). Required for D1's split model to be well-formed.

**D3 — `REMAINING_STATE_MISMATCH` vs residuals homed in another deliverable.** DEL-04-01 C18 flagged `REMAINING_STATE_MISMATCH` (LOW) for a TBD that may be homed in DEL-04-06/DEL-00-06; DEL-12-02 REXC-REQ-003 recorded `NONE_RECORDED` (no finding) for linter work recorded under DEL-08-05/E7. Opposite handling of "residual possibly owned elsewhere."
**Convention 3:** before flagging an omission, check the named candidate homes' `_STATUS.md`; if verifiably homed elsewhere → no finding, cross-reference recorded; if unresolved after that check → `UNKNOWN` with the smallest next check (not `REMAINING_STATE_MISMATCH`). Whether deliverables must mirror residuals owned elsewhere stays an R3 question.

**D4 — Merged `PackageID/DeliverableID` column and its UNMAPPED encoding.** All three merged the two contract names into one column (consistent, and the header count works out), but unmapped values diverge: `PKG-04/UNMAPPED` (04-01), free-text `UNMAPPED (run records live under PKG-10/DEL-10-05; GUI panel ownership plausibly PKG-07/PKG-08)` (10-05 — breaks machine-readability of the ID column), bare `UNMAPPED` (12-02).
**Convention 4:** fix §6 to a single `PackageID/DeliverableID` column with controlled values only: `PKG-XX/DEL-XX-XX`, `PKG-XX/UNMAPPED`, or `UNMAPPED`; ownership hypotheses go in `RemainingWork`/notes, never in the ID cell. Also settles all three notes' 20-vs-21-column question.

**D5 — `GateOrStageConstraint` default when no residual exists.** DEL-10-05 wrote `UNGATED` on requirement rows with `RecordedRemaining=NONE_RECORDED`; DEL-04-01 and DEL-12-02 wrote `NONE_RECORDED`. Same fact, two encodings — gate-state queries across the corpus would miscount.
**Convention 5 (with the class rubric):** `NONE_RECORDED` when no residual exists; `UNGATED` only for an existing residual lacking a gate suffix. Bundle here the one-line `ClaimClass` rubric for diagnostics/data-policy claims (T2) and the rule that a `MECHANICS` row may carry `ValidationEvidence=NOT_APPLICABLE` only with an explicit in-cell reason; and a `SourceReliability` rubric (what earns `VETTED` vs `REVIEWED` — DEL-04-01's notes request this; DEL-12-02 additionally used `UNVERIFIED`/`NOT_APPLICABLE` on prose rows with no stated rule).

**D6 — `SelectableUnderCurrentLoop` under the owner's suspension declaration: a direct three-way contradiction.** For ungated residuals on an `IN_PROGRESS` deliverable: DEL-04-01 → `UNKNOWN`, DEL-10-05 → `UNKNOWN`, DEL-12-02 → `YES`. Same program-state question, contradictory answers in the same run.
**Convention 6:** derive the column mechanically from DAG/lifecycle/gate rules only (per the §6 column contract — DEL-12-02's `YES` is actually the contract-literal reading), and record the suspension as a single run-level caveat in `RUN_BASIS.md` rather than per-row. Bundle here the SECURITY-class validation rule (T3): a fixed encoding for security-behavior claims — e.g. `ValidationEvidence=NONE_FOUND — sufficiency review deferred, owner-gated` with no `VERIFIED_NOT_VALIDATED` downgrade where the accepted scope defers sufficiency review — pending the ruled SECURITY_CONTROL checking-entry profile.

**D7 — Recorded-pass vs re-executed verification evidence.** DEL-04-01 re-executed nothing; DEL-10-05 re-executed nothing and flagged "not re-executed at frozen SHA" inline; DEL-12-02 re-executed side-effect-free checks (pytest, validator, greps) and cited the rest as recorded. All disclosed; none standardized.
**Convention 7:** re-execute only side-effect-free checks in the frozen tree (the DEL-12-02 pattern; verify tree cleanliness after); otherwise cite the recorded pass with its run record and commit binding plus a **standardized marker string** ("not re-executed at frozen SHA <sha>"). Add to the same convention the decision-basis resolvability rule from A2–A4: a `DecisionBasis` must resolve to an artifact in the evidence tree or be explicitly marked attestation-level (e.g. `ATTESTED: MEMORY.md 2026-05-01, record not present in tree`), and must be a decision that governs the claim, not stage context.

**D8 — D-41 frozen-tree observability caveat.** Handled consistently in substance by all three pilots (notes only, no `AUTHORITY_CONFLICT`, correct reasoning) — the one divergence-prone area that did **not** diverge. Needs only codification: rows reflect the frozen register state; the run's authority comes from the dispatch/live register per §3 boundary 6; state this once in `RUN_BASIS.md` and stop re-deriving it per pilot. (Also: `RecordedRemaining` copying breadth diverged — DEL-12-02 copies the deliverable residual onto every row it touches, ~12 rows; DEL-04-01 quotes it only on directly-touched rows. Fold the rule "copy verbatim only on rows the residual's claim touches; `NONE_RECORDED` elsewhere" into the same codification.)

**D9 — Disposition of an accurately-recorded residual.** DEL-04-01 C19 and DEL-12-02 C08 → `ALIGNED`; DEL-10-05 C13/C14 → `ACCEPTED_DIVERGENCE` (decision-permitted stubs). Not contradictory — the situations differ — but the precedence when both fit is unstated.
**Convention 8:** for `REMAINING_WORK` rows: `ACCEPTED_DIVERGENCE` when a human decision permits the deferred state; `ALIGNED` when the residual is accurate but decision-less; `STALE_REVIEW_OR_EVIDENCE`/`REMAINING_STATE_MISMATCH` per their definitions. When both fit, `ACCEPTED_DIVERGENCE` wins (it carries more information).

---

## 6. False positives / negatives found

**FN1 (understated problem) — DEL-10-05 R-02 and C11 are `ALIGNED` while their only cited validation witness is overtaken.** The 822-vs-830 `result_refs` delta means `tp_runner_015_final_cli_solve.json` does not bind the frozen source state (the ledger's own C15 row says so). The pilot avoided double-counting by cross-referencing C15, and confidence is MEDIUM — but a package summary computed from claim rows would show two `ALIGNED` behavioral claims whose class-relevant evidence does not bind the reviewed source state, which §10 QA ("evidence tied to the reviewed source state") disallows for `ALIGNED`. At scale this pattern silently launders stale evidence. Recommended rule (fold into convention 8): a claim row whose cited validation evidence is overtaken for the reviewed source state may not be plain `ALIGNED`; it takes `STALE_REVIEW_OR_EVIDENCE` (or `ALIGNED` only if independent bind-current evidence is also cited — here the crate tests are verification, not the reproducibility validation witness).

**FP1 (overstated, mild) — DEL-04-01 C02 conflates two surfaces.** The `NormativeSource` is the Scope exclusion (tolerances/solver-library selection out of kernel scope) — which still holds; the kernel defines no tolerance policy, as the row itself concedes. What is stale is the *Datasheet TBD register* (verified: "Solver numerical library | TBD" etc.). Dispositioning the exclusion row `STALE_SETUP_SPECIFICATION` marks a still-valid exclusion as stale. Under convention 1/2 this splits cleanly: exclusion row `ALIGNED`, Datasheet TBD rows as `DECLARED_STATE` claims `STALE_SETUP_SPECIFICATION`. Direction of the finding is right; the row placement overstates.

**FP2 (borderline, disclosed) — DEL-10-05 UNMAPPED-02.** The pilot itself concedes the optional envelope-payload validation is "arguably within R-04's result-envelope alignment scope" (LOW confidence). Flagging library-internal capability granularity as unmapped implementation risks flooding the R1 unmapped inventory with sub-requirement features. Convention for R1: `IMPLEMENTED_UNMAPPED` is for *material surfaces* (crates, binaries, schemas, app panels), not internal functions plausibly inside an existing requirement — the DEL-04-01 and DEL-12-02 unmapped rows (whole crates / whole panel surfaces) are the right grain; this row is the wrong grain, though honestly labeled.

**Checked and NOT false positives:** DEL-12-02 C04/C05/C07 staleness (spec line 7, Guidance OI rows verified genuinely lagging the landed App.tsx:1403 binding); DEL-12-02 C06 `IMPLEMENTED_DIFFERENTLY` (token sets genuinely differ); DEL-04-01 C17 (README Current Limitation TBD genuinely absent from `## Remaining`); DEL-10-05 C16 (schema exists and is canonically named); all three UNMAPPED crate/panel rows (all genuinely absent from the four-document kits; decisions cited genuinely deliverable-less or mapping-less). No case was found of `STALE_SETUP` applied to accurate prose, and no case of `IMPLEMENTED_UNDOCUMENTED` where a four-document mapping exists that the pilot missed (REXC-REQ-012's "unit … controls" clause was the nearest candidate and does not map the Export Safety Review unit-evidence surface).

---

## 7. Boundary compliance statement

Verified across all 85 rows and three notes files:

- **No lifecycle transition proposed or applied.** `LifecycleState` is copied only; zero uses of `LIFECYCLE_REASSESSMENT_REQUIRED`; DEL-12-02 REXC-VER-006 explicitly records the read-without-edit check.
- **No DAG mutation or dependency-register change proposed.** DAG-007 appears only as re-verified provenance via `_DAG/_LATEST.md` (10-05 notes), per §3 boundary 6.
- **No cross-project edit.** All writes are inside the run folder; the frozen tree is clean (git-verified by this review after its own re-executions).
- **No agent-workflow redesign.** Zero `DEFERRED_AGENT_WORKFLOW` rows; both notes that mention agent files record them as frozen process inputs only.
- **No compliance/certification/release-readiness/professional claims** (F-PIP-1..5): grep-verified; all matches are quoted exclusions or explicit disclaimers.
- **Agent dispositions are nowhere phrased as owner or engineering rulings.** All three notes carry the disclaimer; `RemainingWork` cells phrase repairs as R5 candidates "after owner decision"; `AuthorityNeeded` correctly routes to `OWNER`/`SCOPE_CHANGE`/`REVIEW` instead of self-resolving (e.g., neither unmapped crate was absorbed into DEL-04-01 scope despite decision bases existing — the exact boundary-1 temptation this pilot set tested).
- **Work-selection fence respected:** the bootstrap `(gated: D-41)` item was excluded from claim analysis and quoted verbatim in all three notes, per the RUN_BASIS special rule.

One process observation, not a violation: DEL-12-02 executed read-only commands (pytest, validator) inside the frozen tree; the tree remained clean, and this review reproduced the same commands with the same result — but the method should say explicitly whether frozen-tree execution is permitted (convention 7 covers this).

---

## 8. Verdict

**READY WITH NAMED CONVENTIONS.** The §6/§7 method is not defective — the pilots' evidence work is accurate (28/30 citation checks passed outright; the 2 qualified checks are unresolvable-record disclosures, not fabrications; 0 failed), authority precedence was never invented, the equation-source and compliance boundaries are fully clean, and every judgment call is disclosed rather than buried. But the method as written is **not ready for 101-deliverable scale-out as-is**: three pilots produced three incompatible conventions for the single most common finding shape (stale prose over implemented substance, D1), a three-way contradiction on `SelectableUnderCurrentLoop` (D6), and enough column-encoding drift (D2, D4, D5) that cross-package histograms, gate queries, and the unmapped-implementation inventory would not be trustworthy aggregation surfaces. All are closable by the eight conventions in §5 (owner-ruled amendments per the ratified kernel's §7 amendment path) plus the two disposition rules in §6 (FN1, FP2). Recommend: rule the conventions, spot-amend the three pilot ledgers only where D1/FN1 change dispositions (or accept them as-is with a run-level note that R0 predates the conventions), then proceed to R1.

**Recommended R1+ conventions (non-binding method-calibration input for the owner):**
1. Single stale-prose rule: substance disposition on requirement rows; staleness on separate declared-state rows; `STALE_SETUP_SPECIFICATION` reserved for the latter; clarify §7 `ALIGNED`'s "declaration" term. (D1)
2. Add `DECLARED_STATE` to the §6 `ClaimType` enum (or codify fold-in). (D2)
3. Cross-deliverable residual homing: check candidate homes before `REMAINING_STATE_MISMATCH`; unresolved → `UNKNOWN` + smallest next check. (D3)
4. One merged `PackageID/DeliverableID` column with controlled values incl. `PKG-XX/UNMAPPED`; no free text in the ID cell; fixes the 20/21-column contract. (D4)
5. Column-default and rubric bundle: `NONE_RECORDED` vs `UNGATED` rule; `ClaimClass` rubric for diagnostics/data-policy claims; `MECHANICS` + `NOT_APPLICABLE` validation only with in-cell reason; `SourceReliability` (`VETTED` vs `REVIEWED`) rubric. (D5, T2)
6. `SelectableUnderCurrentLoop` derived mechanically from DAG/lifecycle/gate rules with a run-level suspension caveat; fixed SECURITY-class validation encoding pending the ruled SECURITY_CONTROL profile. (D6, T3)
7. Evidence-execution and basis-resolvability rule: re-execute only side-effect-free checks; standardized "not re-executed at frozen SHA" marker; `DecisionBasis` must resolve in the evidence tree or be marked attestation-level, and must govern the claim (not stage context). (D7, A2–A4)
8. Residual-row and stale-evidence disposition precedence: `ACCEPTED_DIVERGENCE` > `ALIGNED` when both fit; overtaken class-required validation evidence bars plain `ALIGNED`; `IMPLEMENTED_UNMAPPED` is material-surface-grained. (D8, D9, FN1, FP2)
