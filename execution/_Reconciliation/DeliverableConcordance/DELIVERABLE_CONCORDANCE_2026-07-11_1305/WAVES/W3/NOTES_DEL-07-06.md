# W3 Discovery Notes — DEL-07-06 Accessibility and usability baseline

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 (activation D-41/DEC-073), R2 wave W3
**Deliverable:** DEL-07-06 "Accessibility and usability baseline" (PKG-07, GUI/UX class, IN_PROGRESS)
**Frozen evidence tree:** `/Users/ryan/ai-env/projects/chirality/.claude-worktrees/piping-frozen-551f84ef6` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
**Ledger:** `CLAIM_CONCORDANCE_DEL-07-06.csv` — 18 claim rows
**NormativeSource path alias (addendum 12):** deliverable folder = `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/`.

## 1. Histograms (recount from the CSV)

**Disposition histogram (18):**
- ALIGNED — 9
- VERIFIED_NOT_VALIDATED — 6
- STALE_SETUP_SPECIFICATION — 3

**ClaimType histogram (18):**
- REQUIREMENT — 10
- DECLARED_STATE — 6
- EXCLUSION — 1
- REMAINING_WORK — 1

Both reproduce exactly from the ledger (verified by recount script before writing).

## 2. Row census rationale

- **10 REQUIREMENT rows**, one per current requirement ID DEL-07-06-RQ-001..010 (re-verified against the frozen Specification.md; matches the R1 inventory). ClaimIDs use the addendum-12 fixed form `DEL-07-06-REQ-NNN`; the native RQ ID is recorded in `NormativeSource`.
- **0 ACCEPTANCE rows.** The Specification's Verification section is a "Verification area | Minimum setup expectation" table whose eight rows each merely restate a requirement's verification hook (keyboard→RQ-002, labels→RQ-003, contrast→RQ-004, warning separation→RQ-006, review workflow→RQ-001, unit/provenance→RQ-007, protected content→RQ-009, professional boundary→RQ-010). Per the brief ("verification tables that merely restate requirements do NOT get mirrored ACCEPTANCE rows") none warrant an ACCEPTANCE row. DEL-07-06's Specification carries no named, addendum-12-grain acceptance register (unlike the pre-addenda DEL-07-05 exemplar's VER-07-05-00x items); the setup-gate checks (four-document presence, dependency-schema validity, SEMANTIC_READY) live only in the Procedure surface and are covered by its DECLARED_STATE row. See §4(a).
- **1 EXCLUSION row** (DEL-07-06-EXC-001) for the durable product boundary in Scope ¶2 (no final accessibility/WCAG target; no certification/sealing/approval/code-compliance claim). The same sentence's setup-era clause ("does not implement UI behavior, edit GUI source, edit tests, edit schemas") is overtaken by the implemented slice and is ledgered on the Specification DECLARED_STATE row per the convention-1 two-signal split, not as an exclusion.
- **6 DECLARED_STATE rows** (addendum-1 census): Specification, Datasheet, Guidance, Procedure, `_STATUS.md`, `MEMORY.md`. **No README** exists in the deliverable folder (full `find -iname readme` = none), so no README DECL row. `_CONTEXT.md`/`_REFERENCES.md` are not census surfaces.
- **1 REMAINING_WORK row** (DEL-07-06-REM-001) for the single real ungated residual ("packaged-Tauri smoke over a saved project with edited load data"). The seeded `(gated: D-41)` bootstrap item is recorded verbatim only in the `_STATUS.md` surface row's `RecordedRemaining` (addendum 2) and takes no row of its own and no residual/gate/selectability analysis.
- **0 IMPLEMENTED_UNMAPPED rows.** Every material surface in this deliverable's orbit (core/gui/accessibility [SURF-072], accessibility-baseline panel [SURF-008], nativeMenu [SURF-055], unitConversion [SURF-063]) already carries a DEL-07-06 attribution in `IMPLEMENTATION_SURFACES.csv`; none of the R1 NONE_FOUND-attribution shortlist is in this orbit.

## 3. Disposition posture (the central judgment)

This is a setup-declared deliverable that in fact has a landed implementation: a 36 KB deterministic contract-review engine (`core/gui/accessibility/engine.py`) plus a focused test (`tests/test_accessibility_usability_baseline.py`, 2026-05-09 Type 2, commit `bfb3931`), a desktop `AccessibilityBaselinePanel.tsx`, and a stream of R3UX usability tranches recorded in MEMORY.

- **Requirement rows never take STALE (convention 1).** They take substance dispositions; the setup-era future-tense staleness is carried on the per-surface DECLARED_STATE rows instead.
- **Verification ≠ usability validation (brief steer, W1 item 8).** The accessibility engine is a deterministic review of GUI *contract records* — an axe/lint-grade check at tooling/verification grain. It is **never** promoted to usability validation. There is no live-desktop-runtime accessibility audit, no selected WCAG target (engine `accessibility_target_status=TBD_by_human_project_authority`, `desktop_runtime_evaluation=not_performed`), and no human usability evaluation anywhere in the frozen tree.
- **Grain chosen (W1 item 6 — stated and self-flagged):** for the accessibility/usability *quality* requirements whose satisfaction is a suitability/sufficiency question ("visible enough for engineering review", "clear labels", "sufficient", keyboard/undo discoverability) I used **VERIFIED_NOT_VALIDATED** — implementation present and verification-tested, but the validation basis the claim class calls for (usability evaluation / accessibility-conformance audit) is deferred/TBD: RQ-001, RQ-002, RQ-003, RQ-004, RQ-005, RQ-008.
- For the **structural/boundary** requirements that a test can settle without usability validation I used **ALIGNED**: RQ-006 (warning-class separation, engine + PY-06 token asserts), RQ-007 (unit visibility, evidenced + no silent default), RQ-009 (IP/privacy boundary), RQ-010 (no compliance/certification language). This mirrors the DEL-07-05 exemplar's ALIGNED-with-`ValidationEvidence=NOT_APPLICABLE` treatment of verifiable GUI-class claims.
- **Divergence from the DEL-07-05 H4 posture, explained:** DEL-07-05's requirements were result-*display* claims (verifiable → uniformly ALIGNED). DEL-07-06's differ because they assert a usability/accessibility *baseline* whose conformance validation is the deliverable's own explicitly-deferred subject; treating that deferral as an unmet validation basis (VERIFIED_NOT_VALIDATED) is the honest reading and is what the brief's accessibility caveat demands.

## 4. Convention friction notes

(a) **ACCEPTANCE-row grain.** The pre-adopted DEL-07-05 exemplar mirrored named setup-gate VER items (document review, dependency-register validation, semantic-setup review) as ACCEPTANCE rows. The adopted addendum-12 grain + the brief's "restating verification tables" rule cut those, and DEL-07-06's Specification has no named acceptance register anyway. I recorded zero ACCEPTANCE rows and folded the setup-gate checks into the Procedure DECLARED_STATE row. Reviewer confirmation welcome that this is the intended addendum-12 reading.

(b) **W1 item 1 (rev-0.7 / DAG-006 authority drift) lands on no census surface here.** The kit citation of SOFTWARE_DECOMP revision 0.7 + DAG-006 (frozen header is revision 0.8 `status: current_basis`; live DAG is DAG-007 per RUN_BASIS) appears only in `_CONTEXT.md` (lines 42, 52) and `_REFERENCES.md` (line 15) — neither an addendum-1 census DECLARED_STATE surface — and in one *dated* MEMORY entry (2026-06-04), where addendum 1 makes it a note not a staleness disposition. No kit document (Specification/Datasheet/Guidance/Procedure) or `_STATUS.md` carries the rev-0.7/DAG-006 pointer as a current declaration, so **no STALE_SETUP_SPECIFICATION disposition attaches on the pointer-drift basis**, and per W1 item 1 AuthorityNeeded stays NO for pure pointer drift. Owner-calibration caveat recorded once, here.

(c) **W2 item 13 (SourceReliability keys to the weakest load-bearing leg) tightens the exemplar.** DEL-07-05 used REVIEWED broadly; I used **UNVERIFIED** on every evidence-bearing row (requirements, exclusion, REM) because no human ruling positively validates DEL-07-06's technical evidence — the CHECKING-readiness and test-discovery records are agent TASK reviews, and the PKG-07 human disposition (2026-06-06) resolved *sibling* findings and explicitly found no DEL-07-06 local finding (an absence of adverse ruling, not a positive validation). DECLARED_STATE prose rows are `NOT_APPLICABLE` (addendum 6).

(d) **W2 item 14 (AuthorityNeeded is an adjudication router, not a work queue).** Only RQ-004 routes **OWNER** — the measurable accessibility conformance target is a genuine owner-gated human-authority decision the deliverable itself escalates (DEL-07-06-CF-001; Specification Standards). Its `ValidationEvidence` uses the owner-gated deferral marker form harmonized in W1 item 2. The other deferred-usability rows route NO (recorded future-work deferral, no numeric/authority claim promoted). The STALE DECLARED_STATE rows route OWNER as R5 repair candidates (owner-authorized repair tranche). The bootstrap `(gated: D-41)` item is a gate-named token disclosed here per W2 item 14 / W1 D-07b precedent; it is excluded from selectability and gate analysis.

(e) **W2 item 11 (bootstrap `_STATUS` cell scoping).** On DEL-07-06-DECL-005 the two Remaining items appear verbatim in `RecordedRemaining`, while `RemainingSource`/`GateOrStageConstraint` are scoped to the non-bootstrap residual only (`PRD plan §4 FR-007 A4 residual / seam plan §9.5` / `UNGATED`) — the exclusion variant, not bootstrap annotation. Bootstrap text transcribed byte-exact (W1 item 5): `§§6–8` en-dash and `§` preserved.

(f) **Residual homing / selectability.** The packaged-Tauri smoke residual is a cross-cutting GUI-workflow validation item (FR-007 A4), not a gate on any single accessibility requirement's implemented slice. I homed it at deliverable grain: mechanical convention-6 selectability YES on DEL-07-06-REM-001 and the `_STATUS.md` surface row (DEL-07-06-DECL-005) only; all requirement rows NO. This matches inventory-grain `SelectableUnderCurrentLoop=YES` while keeping per-row derivation mechanical. The owner suspension is a run-level caveat, written into no row.

## 5. Self-flagged rows

- **DEL-07-06-REQ-001, -002, -003, -005, -008** — VERIFIED_NOT_VALIDATED grain call: implemented + verification-tested but usability/accessibility validation deferred. The verification-vs-usability-validation boundary (brief steer) drove these off the exemplar's uniform-ALIGNED posture; reviewer eyes on whether GUI-class deferred-usability should instead be ALIGNED-with-NOT_APPLICABLE.
- **DEL-07-06-REQ-004** — same VNV grain, plus the sole AuthorityNeeded=OWNER routing (owner-gated measurable accessibility target, CF-001) and the owner-gated `ValidationEvidence` deferral marker. Flagged for the routing judgment.
- **DEL-07-06-DECL-003 (Guidance)** — ALIGNED vs STALE_SETUP_SPECIFICATION judgment under widened addendum 4: Guidance is forward-advisory and makes no overtaken factual declaration (its conformance-TBD claim is accurate), so I held ALIGNED-with-note while the Specification/Datasheet/Procedure surfaces (which declare a no-implementation state) took STALE. Reviewer eyes on the line drawn.
- **DEL-07-06-DECL-006 (MEMORY)** — ALIGNED under the W2 item 9 self-correcting-undated-header rule: the header names the in-file DEC-047 correction of the older F-4/A3-open entries. Flagged as a judgment on the header-block rule.

## 6. Evidence-execution log

Re-executed **side-effect-free inside the frozen tree** (`git -C FROZEN status --porcelain` empty before and after every batch; `PYTHONDONTWRITEBYTECODE=1`; no `CARGO_TARGET_DIR` needed — no cargo run):
1. `python3 tests/test_accessibility_usability_baseline.py` → **PASS** (exit 0; PY-06 wrapper `main()`; asserts diagnostic-class tokens present and FORBIDDEN_TEXT absent).
2. `python3 tools/validation/validate_dependencies_schema.py <DEL-07-06 Dependencies.csv>` → **VALID** (29 required columns, 16 data rows).
3. Git plumbing only (`git diff --stat e648462f1..551f84ef6` over `apps/,core/,tests/,fixtures/,schemas/`) → **empty**, establishing product-code byte-identity between the DEC-025 sweep commit and the frozen SHA; the recorded desktop Vitest/Playwright passes therefore bind a product-code state identical to the frozen SHA.

**Cited as recorded (not re-executed):** the DEC-025 five-surface sweep `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` at commit `e648462f1` (desktop_vitest + desktop_playwright_e2e pass), carried with the marker `not re-executed at frozen SHA 551f84ef6` plus the byte-identity qualifier above. **Not run:** node/Vitest/Playwright, desktop build, Tauri packaging (not side-effect-free; would risk frozen-tree writes — prohibited by the brief's GUI clause).

## 7. Boundary-compliance statement

All discovery reads and the three re-executed checks ran inside the frozen worktree; `git status --porcelain` was verified empty before and after every execution batch — the frozen tree was never modified (no writes even to git-ignored paths; no `target/`, `__pycache__`, or `.pytest_cache`). No lifecycle transition was applied (no LIFECYCLE_REASSESSMENT_REQUIRED disposition was needed or used); no DAG mutation; no cross-project edit; no edit to any `_STATUS.md`, register, or product file. No release-readiness, issuance, certification, sealing, professional-approval, or code-compliance claim appears anywhere in these outputs (F-PIP-1..5 held). All dispositions are agent judgments routed via `AuthorityNeeded`, never phrased as owner or engineering rulings. No `DEFERRED_AGENT_WORKFLOW` claim arose. The only writes of this pilot are the two files `RUN/WAVES/W3/CLAIM_CONCORDANCE_DEL-07-06.csv` and `RUN/WAVES/W3/NOTES_DEL-07-06.md`.
