# NOTES — DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

Wave W1 discovery pilot. Frozen evidence SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ledger: `WAVES/W1/CLAIM_CONCORDANCE_DEL-03-05.csv` (20 claim rows).

## Run-level NormativeSource path alias (addendum 12)

`KIT/` = `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-05_Rigid component models for valves, flanges, reducers, and specialty items/`
(relative to `projects/chirality-piping/`). Used in `NormativeSource`, and in `DecisionBasis`/`ImplementationEvidence` where it points inside the deliverable folder. `GATEC` shorthand in the CSV `DecisionBasis` = `execution/PKG-03.../1_Working/_run_records/WORKING_ITEMS_RUN_2026-06-05_HUMAN_DISPOSITION_GATE_C.md`.

## Histograms (recount from the CSV; reproduce exactly)

Disposition histogram (20 rows):
- ALIGNED — 13
- PARTIALLY_IMPLEMENTED — 3
- STALE_SETUP_SPECIFICATION — 4

ClaimType histogram (20 rows):
- REQUIREMENT — 11
- EXCLUSION — 3
- DECLARED_STATE — 6
- (ACCEPTANCE — 0; REMAINING_WORK — 0; IMPLEMENTED_UNMAPPED — 0)

## Row census rationale

- **REQUIREMENT (11):** one row per current requirement R01–R11 (Specification requirements table). No requirement/acceptance/exclusion row takes STALE_SETUP_SPECIFICATION (convention 1).
- **ACCEPTANCE (0):** the Specification "Verification hook" column and "Verification" prose merely restate the requirements; per the addendum-12 grain rule they get no mirrored ACCEPTANCE rows. No independent acceptance-criteria surface exists.
- **EXCLUSION (3):** the three bullets under Specification "Out of scope for this reconciliation" (repo-level editing/ DEL-03-01 boundary; protected dimensional/vendor/catalog data + default properties; compliance/certification/professional-acceptance claims).
- **DECLARED_STATE (6):** four-document kit (Specification, Datasheet, Guidance, Procedure) + `_STATUS.md` + `MEMORY.md`. No deliverable-owned in-tree README exists. `_REVIEW.md` and `Review_Findings.csv` are dated audit records, not current-state declaration surfaces, so they carry no DECLARED_STATE row (they are used as evidence).
- **REMAINING_WORK (0):** the only `_STATUS.md ## Remaining` entry is the seeded `(gated: D-41)` bootstrap item, recorded verbatim in the `_STATUS.md` surface row (DECL-005) `RecordedRemaining` and excluded from all residual/gate/selectability analysis (addendum 2). No non-bootstrap residual is recorded in `_STATUS.md`; requirement-tied deferrals are carried in the relevant requirement rows' `RemainingWork` (see Deferred-scope inventory).
- **IMPLEMENTED_UNMAPPED (0):** see self-flag SF-4.

## R0N → REQ-00N mapping and the DEL-03-03/DEL-03-05 requirement-ID collision (recorded per dispatch)

ClaimID form (addendum 12): `DEL-03-05-REQ-00N` disambiguates the bare tokens mechanically.

| Bare token | ClaimID |
|---|---|
| R01 | DEL-03-05-REQ-001 |
| R02 | DEL-03-05-REQ-002 |
| R03 | DEL-03-05-REQ-003 |
| R04 | DEL-03-05-REQ-004 |
| R05 | DEL-03-05-REQ-005 |
| R06 | DEL-03-05-REQ-006 |
| R07 | DEL-03-05-REQ-007 |
| R08 | DEL-03-05-REQ-008 |
| R09 | DEL-03-05-REQ-009 |
| R10 | DEL-03-05-REQ-010 |
| R11 | DEL-03-05-REQ-011 |

**Collision, verified independently:** DEL-03-03 (Bend and elbow component model fields) and DEL-03-05 both use the bare token set R01–R11. The tokens collide as *labels only* — the requirement substance differs entirely between the two deliverables (DEL-03-03 R01 = "bend/elbow identity separate from other families"; DEL-03-05 R01 = "support rigid/semi-rigid valve/flange/reducer/rigid/specialty structures"). A whole-folder grep of DEL-03-05 for `\bR[0-9]{2}\b` returns matches **only** inside DEL-03-05's own Specification requirements table (lines 17–27); no record in DEL-03-05's kit, memory, review, run-records, or dependency files cites a bare `R0N` token in a cross-reference that could resolve to DEL-03-03. **DEL-03-05's own records are inert on the collision** (same posture the DEL-03-03 pilot found for itself). No finding encoded; the addendum-12 ClaimID form is the standing mechanical disambiguation.

## Primary concordance signal — stale review-finding-disposition prose across the whole four-document kit (DECL-001..004, STALE_SETUP_SPECIFICATION)

All four kit documents declare review findings `PKG03-DEL-03-05-PKG02-001/002` as still pending human disposition (Specification "Human disposition: TBD; CSV not edited"; Datasheet "Human review disposition remains TBD"; Guidance Conflict Table "Human ruling TBD" x2; Procedure step 6 "remain conceptually TECHNICALLY_ADDRESSED_PENDING_HUMAN with HumanDisposition=TBD; do not edit Review_Findings.csv").

This is contradicted by the frozen-tree authority:
- `KIT/Review_Findings.csv` shows both findings `HumanDisposition=ACCEPT_AS_IS`, `Status=RESOLVED` (reviewer 2026-05-16 row updated 2026-06-05).
- Gate C human ruling record `.../_run_records/WORKING_ITEMS_RUN_2026-06-05_HUMAN_DISPOSITION_GATE_C.md` explicitly "Accepted 6 listed DEL-03-04/05/06 findings as ACCEPT_AS_IS / RESOLVED" and updated the CSV rows from `HumanDisposition=TBD`/`Status=TECHNICALLY_ADDRESSED_PENDING_HUMAN`.

Cause (from `MEMORY.md`): the four docs were reconciled in the 2026-06-05 "Evidence reconciliation" pass, which deliberately preserved the findings as pending and did NOT edit the CSV; the *same-day-later* Gate C acceptance edited the CSV to RESOLVED but never refreshed the four docs. The docs uniformly under-report the review state. Encoded as `STALE_SETUP_SPECIFICATION` (widened per addendum 4 to post-alignment drift on declaration surfaces), `AuthorityNeeded=REVIEW`, `Confidence=HIGH`. The remainder of each document (requirements, implemented-slot descriptions, principles) is accurate; the staleness is bounded to the review-disposition sub-claim. Disposition is an agent judgment, routed to REVIEW — not represented as an owner or engineering ruling, and no document edit proposed (discovery is read-only).

## Deferred-scope inventory (carried in requirement rows' RemainingWork, not as standalone REMAINING_WORK rows — see SF-6)

Recorded deferred items across the kit/memory that are NOT `_STATUS.md ## Remaining` residuals:
- COG coordinate convention / reference frame (→ REQ-010 `RemainingWork`, PARTIALLY_IMPLEMENTED, ENGINEERING).
- Adapter/plugin no-bypass tests (→ REQ-007, per `_REVIEW.md` DEL-02-04 PASS_WITH_DEFERRED_EVIDENCE).
- Concrete round-trip / canonical-JSON hash fixtures (→ REQ-008, per `_REVIEW.md` DEL-02-05 WARNING).
- Accepted public rigid-component source catalogs; public rigid-component fixture-value policy (→ REQ-011 note; provenance gate itself is honored/ALIGNED).
- Exact solver treatment of semi-rigid stiffness inputs; concrete import formats; per-family engineering profiles; downstream component-editor/GUI behavior — future/downstream scope beyond this schema-contract deliverable; recorded but not attributed as DEL-03-05 residual rows to avoid over-attribution.

## Self-flagged rows

- **SF-1 (DECL-001..004):** disposition choice `STALE_SETUP_SPECIFICATION` vs `STALE_REVIEW_OR_EVIDENCE` vs `REMAINING_STATE_MISMATCH`. The drift is a declaration-surface statement *about* a review disposition that has been overtaken. I chose `STALE_SETUP_SPECIFICATION` because addendum 4 explicitly widens it to post-alignment drift on declaration surfaces and convention 1 reserves that value for non-requirement/acceptance/exclusion rows (i.e. DECLARED_STATE). Reviewer may prefer STALE_REVIEW_OR_EVIDENCE (the overtaken-review framing) or REMAINING_STATE_MISMATCH (ruled-shut item still recorded as pending). High confidence in the drift itself; medium confidence in the value label.
- **SF-2 (REQ-006):** ALIGNED at MEDIUM confidence. The centerline/frame boundary is asserted structurally by `$defs.ComponentMechanicsInterface` (no shell/solid requirement), but `test_component_section_schema.py` has no dedicated behavioral test for R06. Reviewer may prefer VERIFIED_NOT_VALIDATED framing; I judged the boundary structurally present and honored.
- **SF-3 (REQ-010):** PARTIALLY_IMPLEMENTED vs ENGINEERING_AUTHORITY_REQUIRED. R10 is a declared bounded TBD (COG convention), but no named human ruling *permits* the deferral, so addendum-5/addendum-11 bar ACCEPTED_DIVERGENCE. I encoded the schema-slot-present-but-convention-unsettled state as PARTIALLY_IMPLEMENTED with AuthorityNeeded=ENGINEERING; a reviewer could instead classify the unsettled coordinate-frame formulation as ENGINEERING_AUTHORITY_REQUIRED.
- **SF-4 (IMPLEMENTED_UNMAPPED = 0):** the 2026-06-21 app-absorption slice (`WORKING_ITEMS_RUN_...TP-R4-D3-RIGIDVIS-001.md`, DEC-045/D-18) added rigid-component evidence to `core/product_physics` and ~15 `apps/desktop` panels + `fixtures/product_preview` (invented `component:C-130`). These material surfaces ARE mapped to DEL-03-05 in `IMPLEMENTATION_SURFACES.csv` (SURF-102, SURF-030/031/032/036/038/043/051/052, SURF-160) via that run record, so they are implementation evidence, not unmapped. `core/product_preview` (SURF-104, `NONE_FOUND`) is a cross-cutting preview service, not specifically in DEL-03-05's rigid-component orbit; I did not attribute an UNMAPPED row to DEL-03-05 for it (leave to the wave's cross-package unmapped shortlist). Flagging the judgment.
- **SF-5 (kit vs app-absorption scope):** the four-document kit (last reconciled 2026-06-05) describes only the schema/fixture/test slice and does not mention the later 2026-06-21 desktop/preview implementation now mapped to DEL-03-05. I treated this as within-scope-of-kit (the kit's declared scope is the schema contract) and did NOT disposition the kit stale on that basis. Reviewer may wish to confirm.
- **SF-6 (REMAINING_WORK = 0):** I folded requirement-tied deferrals into requirement rows' `RemainingWork` rather than minting standalone REMAINING_WORK rows, because `_STATUS.md ## Remaining` records only the excluded bootstrap item and the remaining deferred items are either requirement-scoped or downstream/future scope. Flagging in case the reviewer wants explicit REMAINING_WORK rows for the deferred-policy items.
- **SF-7 (SourceReliability REVIEWED on REQ-001/REQ-003):** marked REVIEWED because the Gate C human ruling specifically covers the ComponentType/specialty enum (finding-001) and the split-stiffness dimension (finding-002); the other nine requirement rows are UNVERIFIED (agent-generated schema/fixture/test evidence, agent-audited, broad human disposition still TBD — addendum 6).

## Evidence-execution log

- **Re-executed (side-effect-free, addendum 9):** `python3 -m pytest tests/test_component_section_schema.py -p no:cacheprovider -q` with `PYTHONDONTWRITEBYTECODE=1`, run from the frozen worktree. Result: **2 passed** (the two collected tests exercise the ComponentType==model enum equality, rigid family contract for valve/flange/reducer/rigid/specialty, split stiffness dimensions, the `RIGID_COMPONENT_GEOMETRY_INCOMPLETE` diagnostic, and protected-content guardrails). `git -C FROZEN status --porcelain` was empty BEFORE and AFTER. No `CARGO_TARGET_DIR` needed (no Rust build re-run in this pilot).
- **Cited as recorded (not re-executed here):** the app-absorption validation battery in `WORKING_ITEMS_RUN_2026-06-21_TP-R4-D3-RIGIDVIS-001.md` (cargo 33/33, pytest 20/20, desktop vitest 406/406, Playwright 18/18) — recorded passes, `not re-executed at frozen SHA 551f84ef6`; not relied on for any requirement disposition here (those surfaces are app-layer evidence, not the schema-contract requirements).
- **Cross-checked read-only:** `Review_Findings.csv` (both findings ACCEPT_AS_IS/RESOLVED), Gate C record, model.schema.yaml component_type enum (includes specialty), the DEL-03-03 Specification (collision check).
- **Corroborating recorded pass:** `VERIFICATION_INDEX.csv` PY-20 records `test_component_section_schema.py` passing in sweep `SWEEP_20260711T040758Z` at ancestor commit `e648462f1d05`, with the addendum-10 qualifier `content-identical at frozen SHA 551f84ef6be656f1603ce0acfa5e3935aa9683c7 (diff empty over tests/, schemas/, fixtures/, ...)`. My own re-execution at the frozen SHA supersedes this as the primary verification citation.

## Convention friction notes

- **DecisionBasis absence for some requirements:** the `DEV-001_DISPATCH_DEL-03-05.md` brief cited by `MEMORY.md` and the `DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/` resolution dir cited by `MEMORY.md` (2026-05-16 addendum) are **absent from the frozen tree**. Both citations sit in dated MEMORY log entries, so per addendum 1 they are recorded as historical-entry notes on the MEMORY surface row (DECL-006 `RemainingWork`), not a staleness disposition. Where a requirement had no in-tree governing decision I used the accepted decomposition scope row (`SOFTWARE_DECOMP.md` SOW-009/DEL-03-05) as `DecisionBasis`; R10 has no decision that settles the COG convention and is marked `NONE_FOUND`.
- **Fixture-name variance:** the live evidence (four-doc kit + test) uses `fixtures/component/invented_component_library_valid.json`; older records (`MEMORY.md` 2026-05-01/2026-05-11 write-scope, `_REVIEW.md`) name `invented_section_component_library_valid.json`. Both files exist in the frozen tree; the test loads the non-`section` variant. Recorded as a historical-entry note on DECL-006, not a live-surface drift.
- **INVENTORY selectability vs per-row:** `DELIVERABLE_INVENTORY.csv` marks DEL-03-05 `SelectableUnderCurrentLoop=YES` with remaining-count 1 — that reflects the pre-exclusion mechanical read of the single bootstrap item. Under addendum 2 the bootstrap is excluded from selectability, so post-exclusion the deliverable has no selectable residual and every claim row is mechanically `NO` (addendum 12: NO on rows with no recorded item). The owner suspension is treated as a run-level caveat only, never per-row (convention 6).
- **Bootstrap gate cell:** DECL-005 (`_STATUS.md`) carries the bootstrap text verbatim in `RecordedRemaining` but `GateOrStageConstraint=NONE_RECORDED` and `SelectableUnderCurrentLoop=NO`, because the `(gated: D-41)` item is excluded from gate/selectability analysis (addendum 2).

## Fan-in repair (fable re-run)

W1 fan-in verification (`W1_VERIFICATION_PKG-03.md`, DEL-03-05 verdict: DEFECTIVE — mechanical
encoding defects only; all substance dispositions verified sound) triggered an owner-ruled fable
re-run of this ledger. The repair pilot independently re-verified the convention texts
(`R0B_CALIBRATION/R0B_CONVENTIONS.md` convention 5; pinned plan §6 `RemainingSource` contract;
R1_CONVENTIONS addendum 6) and the affected evidence (frozen `_STATUS.md ## Remaining` carries only
the addendum-2 bootstrap item — no residual exists on any REQ/EXC row; `docs/CONTRACT.md` /
`docs/DIRECTIVE.md` inspected at the frozen SHA) before re-encoding. **No Disposition, Confidence,
AuthorityNeeded, or evidence cell was changed; both histograms recounted from the repaired CSV and
reproduce the values above exactly.**

Cells changed (19 cells, 16 rows):

1. **`GateOrStageConstraint`: `UNGATED` → `NONE_RECORDED`** on all 14 REQUIREMENT/EXCLUSION rows
   (REQ-001..REQ-011, EXC-001..EXC-003). Convention 5 fixes `NONE_RECORDED` when no residual
   exists; `UNGATED` is reserved for an existing residual lacking a gate suffix. Every one of these
   rows has `RecordedRemaining=NONE_RECORDED` (re-verified against the frozen `_STATUS.md`), so
   `UNGATED` falsely encoded 14 ungated residuals into any cross-package aggregation.
2. **`RemainingSource` → `NONE_RECORDED`** on the 3 no-residual rows that carried a populated
   source: REQ-007 (was `KIT/_REVIEW.md DEL-02-04 PASS_WITH_DEFERRED_EVIDENCE`), REQ-008 (was
   `KIT/_REVIEW.md DEL-02-05 WARNING`), REQ-010 (was `KIT/Specification.md#R10 (Human ruling
   remains TBD)`). §6 contract: "source named by the residual or NONE_RECORDED" — with
   `RecordedRemaining=NONE_RECORDED` there is no residual to name a source. No information lost:
   each row's deferral citation is retained verbatim in its `VerificationEvidence`/`RemainingWork`
   cells.
3. **`SourceReliability`: `REVIEWED` → `UNVERIFIED`** on EXC-002 and EXC-003 (addendum-6 ladder,
   applied independently). Repair-pilot verification note: `docs/CONTRACT.md` and
   `docs/DIRECTIVE.md` are owner-RATIFIED at the frozen SHA (named ruling of record 2026-07-11,
   K-AUTH-1 — "You can now take all the `docs/` out of the DRAFT state, making them
   authoritative"), so the original REVIEWED was not baseless; but the ratification covers those
   governance documents themselves, not a human disposition covering these rows' cited records
   (EXC-002's fixture/test guardrail evidence is project-original agent-generated with agent
   audit; EXC-003 cites no technical record at all). Addendum 6's REVIEWED bar ("named human
   ruling or recorded human disposition covering the cited record") is therefore not met.
   `UNVERIFIED` chosen over `NOT_APPLICABLE` for in-ledger consistency with EXC-001 (verified
   PASS by the fan-in verifier) and with siblings DEL-03-01/DEL-03-06; the
   `NOT_APPLICABLE`-for-pure-prose alternative (DEL-03-03/DEL-03-07 pattern) remains open as a
   wave-level canonicalization question.

Disagreements with the verifier report: NONE on defects 1 and 2 (verified mechanically). On
defect 3 a nuance is recorded above (the documents do carry a named ratification ruling), but the
verification supports the downgrade the verifier suggested. Frozen tree porcelain re-verified
empty before and after all repair-pilot reads; writes confined to this ledger's two files.

## Boundary-compliance statement

- Fences held: discovery was read-only over the frozen tree; no lifecycle transition applied (none proposed); no DAG mutation; no cross-project edit; no edit to any `_STATUS.md`, register, review CSV, schema, or product file. No release-readiness, issuance, certification, sealing, professional-approval, or code-compliance claim appears in these outputs (F-PIP-1..5). All dispositions are agent judgments routed via `AuthorityNeeded`, never phrased as owner/engineering rulings.
- Frozen tree clean: `git -C .claude-worktrees/piping-frozen-551f84ef6 status --porcelain` empty before and after all reads and the sandboxed pytest run.
- **Write-scope disclosure (transparency):** a stale `build_csv.py` left in the shared session scratchpad by a prior pilot re-ran during my CSV build (a Write collision left the old script in place) and regenerated the sibling `WAVES/W1/CLAIM_CONCORDANCE_DEL-00-05.csv`. `git diff HEAD` and `git status --porcelain` on that path are **empty** — the regeneration was byte-identical to the committed version (only mtime touched); no content changed. My own two output files are the sole substantive writes. Corrected by switching to a uniquely named builder (`build_del0305_csv.py`).
- Writes confined to the two W1 output files: `WAVES/W1/CLAIM_CONCORDANCE_DEL-03-05.csv` and `WAVES/W1/NOTES_DEL-03-05.md` (plus scratchpad-only helper scripts).
