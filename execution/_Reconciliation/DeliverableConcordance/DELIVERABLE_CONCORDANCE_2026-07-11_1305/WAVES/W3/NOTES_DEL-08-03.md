# Notes — DEL-08-03 Warnings, assumptions, and provenance report section (Wave W3)

Deliverable: **DEL-08-03** (PKG-08 Reporting, Audit, and Reproducibility), status IN_PROGRESS.
Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ledger: `WAVES/W3/CLAIM_CONCORDANCE_DEL-08-03.csv` (20 rows, RFC-4180, CRLF, 20 columns byte-exact header).

## 1. Histograms (recounted from the CSV)

**Disposition histogram (20 rows):**

| Disposition | Count |
|---|---|
| ALIGNED | 16 |
| STALE_SETUP_SPECIFICATION | 4 |
| **Total** | **20** |

**ClaimType histogram (20 rows):**

| ClaimType | Count |
|---|---|
| REQUIREMENT | 12 |
| DECLARED_STATE | 6 |
| EXCLUSION | 2 |
| ACCEPTANCE | 0 |
| REMAINING_WORK | 0 |
| IMPLEMENTED_UNMAPPED | 0 |
| **Total** | **20** |

Census rationale:
- **12 REQUIREMENT** rows — one per current requirement ID DEL-08-03-REQ-001..012 (re-verified against the frozen Specification.md; exactly 12, matching the R1 inventory).
- **0 ACCEPTANCE** rows — the Specification's per-requirement "Verification" column and its separate "Verification" check table only restate requirement coverage at fixture-check grain; they carry no distinct numbered acceptance criteria, so no mirrored ACCEPTANCE rows (addendum-12 grain rule).
- **2 EXCLUSION** rows — EXC-001 (adjacent-deliverable/renderer ownership boundary: DEL-08-01/02/04/05) and EXC-002 (protected-content / professional-acceptance-record exclusion).
- **6 DECLARED_STATE** rows — Specification, Datasheet, Guidance, Procedure, `_STATUS.md`, `MEMORY.md` (addendum-1 census). `_CONTEXT.md` is not a census surface (see §4).
- **0 REMAINING_WORK** rows — the only `## Remaining` item is the seeded `(gated: D-41)` bootstrap, recorded verbatim in the `_STATUS` surface row's `RecordedRemaining` only and excluded from residual/gate/selectability analysis (addendum 2 / calibration 11). No real recorded residual exists; the deliverable's own TBDs (full renderer, template, redaction/export) have accepted downstream homes (DEL-08-01/02/04/05), so no omitted-residual UNKNOWN candidate is warranted (addendum 5).
- **0 IMPLEMENTED_UNMAPPED** rows — the two material surfaces in this deliverable's orbit (SURF-113 `core/reporting/report_sections` crate; SURF-201 `schemas/report_sections.schema.yaml`) are both already deliverable-mapped to DEL-08-03 in R1 IMPLEMENTATION_SURFACES.csv; the rendered report-section adapter lives in `core/reporting/report_renderer` (SURF-112, mapped to DEL-08-01). No unmapped surface in orbit.

## 2. Self-flagged rows

These rows carry judgment calls the conventions leave open and want reviewer eyes.

- **Global grain choice (all 12 REQUIREMENT rows)** — calibration item 6. DEL-08-03's accepted implemented slice is the report-section **contract** (schema `report_sections.schema.yaml` + validation crate `core/reporting/report_sections`), with the full report renderer/template/GUI/CLI explicitly deferred (Spec Out-of-scope; schema `ReportRendererStatus` all-TBD) and owned by DEL-08-01. I judged each requirement's constraint **at the report-section contract grain** (is the constraint enforced by schema+crate?), treating the rendered-report layer as downstream-deferred, not a DEL-08-03 gap. The recorded agent review (`_REVIEW.md` AC-001) reached the same "REQ-001..012 addressed at the contract layer" reading. An alternative grain (judging each requirement against a rendered report section) would push most rows to PARTIALLY_IMPLEMENTED. Stated here per calibration 6.
- **DEL-08-03-REQ-004** — rule-pack identity/privacy/provenance are enforced in the contract, but rule-pack **checksum** has no first-class field in the report-section contract; it is carried by reference and rendered downstream by `report_renderer` (DEL-08-01, `assemble_report_sections` rule_pack_references + `ChecksumRef`). Marked ALIGNED (contract grain), Confidence MEDIUM.
- **DEL-08-03-REQ-005** — the decision-support / human-review professional boundary is structurally enforced, but the **canonical professional-notice wording** for final templates is TBD (Guidance Open Question Q-005). Called ALIGNED at contract grain rather than PARTIALLY; the missing artifact is release-time wording, not a boundary gap. Confidence MEDIUM.
- **DEL-08-03-REQ-009** — softest ALIGNED. Reproducibility is met **by reference** (`model_ref`/`run_ref`); the concrete manifest fields (model hash, software/solver version, rule-pack checksum) are DEL-08-02 scope and rendered by DEL-08-01. No DEL-08-03-owned integration fixture exercises the reproducibility path at frozen SHA. Confidence MEDIUM. A reviewer could reasonably prefer PARTIALLY_IMPLEMENTED here.
- **DEL-08-03-REQ-011** — process/governance requirement judged at documentation grain: the lint-fallback discipline is recorded in the Procedure + review checklist, and the linter dependency is now satisfied (DEL-08-05), so the fallback branch is currently unexercised. The supporting review is an agent self-check (`WORKING_ITEMS_REVIEW_2026-06-06_1025`), hence SourceReliability=UNVERIFIED. Confidence MEDIUM.

## 3. Evidence-execution log

Frozen-tree porcelain checked **before and after** all re-executions — empty throughout (`git -C <frozen> status --porcelain` produced no output at start and at finish).

Re-executed side-effect-free at frozen SHA 551f84ef6 (addendum 9 discipline: `PYTHONDONTWRITEBYTECODE=1`, no writes into the frozen tree):
- `tests/test_report_sections_contract.py` **main() direct invocation** — 41 static schema assertions PASS (exit 0). Note: this file is script-style (R1 VERIFICATION_INDEX PY-61 records it as 0 pytest-collectable; `pytest` imports but runs no checks). Direct `python3` invocation runs the assertions; it only reads `schemas/report_sections.schema.yaml`, no writes.
- `tools/validation/validate_dependencies_schema.py <DEL-08-03>/Dependencies.csv` — VALID, 29 required columns, 15 data rows.

Cited as recorded / content-identical (not re-executed):
- `core/reporting/report_sections` crate `cargo test` (13 unit tests) — cited from recorded pass `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` surface `cargo_crate_sweep=pass` at commit `e648462f1d0521e26df15d04a988391343018886` (clean tree). In-place `cargo test` would write `target/` into the frozen tree, so it was **not** re-executed. I ran my own `git diff --stat e648462f1..551f84ef6` over `core/reporting/report_sections/`, `schemas/report_sections.schema.yaml`, and `tests/test_report_sections_contract.py` — **empty**, and confirmed `e648462f1` is an ancestor of the frozen SHA; the addendum-10 qualifier `content-identical at frozen SHA 551f84ef6be656f1603ce0acfa5e3935aa9683c7 (diff empty over core/reporting/report_sections/, schemas/report_sections.schema.yaml, tests/test_report_sections_contract.py)` is used only over those actually-diffed paths (calibration item 4).
- `_REVIEW.md` and the PKG-02 downstream audit are agent-authored (SELF_CHECK / TASK_PACKAGE_AUDIT), cited as recorded process evidence, not re-executed.

Existence checks at frozen SHA (full-tree `find`): `tools/validation/check_four_documents.sh` — **absent** (drives DECL-004 staleness); `tools/validation/validate_dependencies_schema.py` — present (re-executed above).

## 4. Convention friction notes

- **Rev-0.7 authority-pointer drift lands only on a non-census surface (calibration item 1 / owner-calibration caveat).** For DEL-08-03 none of the four kit documents, `_STATUS.md`, or the `MEMORY.md` undated header cite decomposition revision 0.7. The stale `Accepted Revision: 0.7 / Status: current_basis / Decomposition Revision ... revision 0.7` pointer lives in **`_CONTEXT.md`**, which is not an addendum-1 DECLARED_STATE census surface (the W1/exemplar ledgers create no `_CONTEXT` row). The only other 0.7 citation is the **dated** `MEMORY.md` 2026-06-04 entry, which addendum 1 protects as a historical record (note only). Consequently there is **no census DECL surface** on which to encode a rev-drift `STALE_SETUP_SPECIFICATION` disposition. I recorded the `_CONTEXT.md` pointer drift as a note on the MEMORY surface row (DECL-006 RemainingWork) and here, rather than minting a non-census `_CONTEXT` row. **Owner-calibration caveat (recorded once):** the frozen decomposition header is revision 0.8, `status: current_basis`, and SOFTWARE_DECOMP §13 classifies unrefreshed downstream surfaces as "stale relative to revision 0.8 until refreshed by their owning workflows" — permitted-pending-refresh; the `_CONTEXT.md` 0.7 pointer is immaterial to the report-section content. If the owner later rules kit-wide revision-pointer refresh material, the `_CONTEXT.md` pointer should be swept with it. AuthorityNeeded stays NO for this pure pointer drift.
- **Contract-vs-rendered grain is the dominant judgment axis** (§2 global self-flag). This deliverable is unusual for a REPORTING deliverable in that its accepted product is a validation/schema **contract**, not a rendered artifact; the four kit documents still speak as if implementation is entirely future (driving the four STALE DECL rows), while the report-section contract is substantively implemented.
- **DECL staleness routing.** All four setup-stale kit DECL rows (Spec, Datasheet, Guidance, Procedure) need substantive rewrites (not a mere revision bump): setup/future framing overtaken by the implemented slice, plus overtaken TBD/Open-Question registers (Datasheet field-name TBDs → schema; Guidance Q-002/Q-003/Q-004 → schema/DEL-08-05/DEL-08-02) and a named-but-absent verification tool (Procedure `check_four_documents.sh`). Routed `AuthorityNeeded=OWNER`, matching the DEL-07-05 calibration exemplar (C02–C05 all OWNER). `_STATUS`/`MEMORY` DECL rows are ALIGNED.
- **SourceReliability keyed to the weakest load-bearing leg (calibration 13).** Requirement/exclusion rows are `UNVERIFIED`: the load-bearing verification leg (crate unit tests + contract test) is project-original agent-generated with agent audit and no human disposition covering the technical evidence (the 2026-06-06 human Gate-5 approval was administrative lifecycle, later reset to IN_PROGRESS by the K-CONFLICT-1 ruling, and did not rule on report-section technical sufficiency). No leg is human-ruled, so no row reaches `REVIEWED`. DECL prose rows are `NOT_APPLICABLE` (addendum 6).
- **No ACCEPTANCE / no ENGINEERING routing.** DEL-08-03 carries no numeric thresholds and is not a SECURITY-class deliverable, so no ENGINEERING/REVIEW/SCOPE_CHANGE routing arises; AuthorityNeeded is OWNER (4 DECL rows) or NO (16 rows). No AUTHORITY_CONFLICT, no UNKNOWN, no DEFERRED_AGENT_WORKFLOW rows.

## 5. Boundary-compliance statement

- **Writes confined to the two W3 output files** (`WAVES/W3/CLAIM_CONCORDANCE_DEL-08-03.csv`, `WAVES/W3/NOTES_DEL-08-03.md`) plus the scratch generator `build_csv_DEL-08-03.py` outside all repos. No `_STATUS.md`, register, kit, product, or DAG file was edited. No lifecycle transition applied (none proposed; `LIFECYCLE_REASSESSMENT_REQUIRED` not used). No cross-project edits.
- **Frozen tree strictly read-only**: porcelain empty before and after every read and re-execution; no writes anywhere under the frozen worktree (no `target/`, `__pycache__`, or `.pytest_cache`).
- **F-PIP-1..5 held.** Provenance/assumptions content is described as the deliverable's boundary enforcement; no release-readiness, issuance, certification, sealing, professional-approval, or code-compliance claim is asserted anywhere in these outputs. Protected-content language is referenced by attribution only, not reproduced. Warning-class semantics that cross into DEL-07-04's panel were left to that deliverable; only DEL-08-03's own concordance facts are recorded here.
- Dispositions are agent judgments, not owner or engineering rulings; authority is routed via `AuthorityNeeded`.
