# W3 Fan-in Verification — PKG-08 (Reporting, Audit, and Reproducibility)

Verifier: fable at high effort (owner-ruled wave-boundary verification per the
Receipt-17 steer). Scope: the six W3 discovery ledgers
`CLAIM_CONCORDANCE_DEL-08-01..06.csv` + `NOTES_DEL-08-01..06.md` under
`WAVES/W3/`, verified against the FROZEN evidence worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` (HEAD re-verified; tracked porcelain
empty before and after all verification reads — this verifier used read-only
git/grep/stat/file-read operations only; no build or test execution). Method
authority: `R1_CONVENTIONS.md` (conventions 1–8, addenda 1–13, Parts C–D),
`RUN_BASIS.md`, and the W1/W2 calibration items in
`PACKAGE_SUMMARIES/PKG-00..05.md` (the seven W3 items: undated-MEMORY-header
rule; README census grain; bootstrap cell scoping; byte-copy re-execution
conditions; weakest-leg SourceReliability; AuthorityNeeded-as-router; ATTESTED
path markers). All findings below are agent-authored and non-binding; nothing
here is an owner or engineering ruling. No ledger was edited.

**Verdicts: DEL-08-01 SOUND · DEL-08-02 SOUND · DEL-08-03 SOUND ·
DEL-08-04 SOUND · DEL-08-05 SOUND · DEL-08-06 SOUND.**
Spot-check totals: **70 PASS / 11 QUALIFIED / 5 FAIL.** All five FAILs are
remediable by owning-pilot string corrections plus one orchestrator
containment act; none requires re-encoding by a fresh pilot (the DEFECTIVE
bar), see §3.1–§3.2. **One material wave-level finding: four W3 pilots wrote
git-ignored artifacts into the frozen evidence tree (addendum-9 breach —
tracked porcelain stayed empty, so every pilot's porcelain check truthfully
passed while the writes occurred). Escalation and cleanup recommendation in
§3.1.** The named Part-C-style SECURITY spot-check on DEL-08-05: **PASS**
(§3.3).

## 1. Mechanical conformance (all six ledgers)

- **Structure:** every CSV parses RFC-4180 clean at exactly 20 columns with
  the byte-exact run header, CRLF line endings; no duplicate ClaimIDs; every
  ClaimID conforms to the addendum-12 fixed form `DEL-08-XX-<TYPE>-NNN`;
  every ID cell is `PKG-08/DEL-08-XX` (convention 4); `LifecycleState`
  uniformly `IN_PROGRESS` (matches the R1 inventory). 132 rows total
  (23/18/20/23/22/26).
- **Histograms:** independently recounted from each CSV — all Disposition and
  ClaimType histograms in all six notes files **reproduce exactly**
  (08-01: 19 ALIGNED/4 STALE; 08-02: 15/3; 08-03: 16/4; 08-04: 20/3;
  08-05: 17 ALIGNED/1 PARTIALLY/4 STALE; 08-06: 22/4; type censuses
  10-1-3-6-3 / 10-0-1-7-0 / 12-0-2-6-0 / 12-1-3-6-1 / 12-1-2-6-1 /
  14-1-5-6-0). Requirement censuses independently re-derived from the six
  frozen Specifications: exactly 10/10/12/12/12/14 requirement IDs — all
  match the ledgers and the R1 inventory.
- **Addendum-1 census:** 6/7/6/6/6/6 DECL rows. Four-document kit + `_STATUS`
  + `MEMORY` on all six; DEL-08-02 additionally censuses
  `core/reporting/audit_manifest/README.md`, whose self-identification as
  "the bounded implementation slice for `DEL-08-02`" I verified in the frozen
  file — exactly the W2→W3 README-census-grain calibration rule. The zero-
  README censuses on the other five are **verified true** (frozen
  `result_export` contains only `.gitignore`/`Cargo.toml`/`src`;
  `protected_content_linter` has no README; the `state_comparison` module
  holds only `__init__.py`/`engine.py`; the only `core/reporting` README is
  DEL-08-02's — DEL-08-01 correctly declined to census it). No staleness
  disposition anywhere rests on drift inside a dated MEMORY entry.
- **Addendum-2 bootstrap:** the seeded `(gated: D-41)` item appears verbatim
  only in each `_STATUS` surface row's `RecordedRemaining`, never as its own
  row, and is excluded from residual/gate/selectability analysis in all six
  (W2 exclusion variant uniformly). Byte-exactness programmatically verified
  against the six frozen `_STATUS.md ## Remaining` sections: **five of six
  ledgers byte-exact** (including all five REM-row items and the DEL-08-05
  owner/stage gate string verbatim). **DEL-08-06 DECL-005 fails byte-exactness
  by one character** — the bootstrap item's `§§6–8` (en dash, U+2013) was
  transcribed `§§6-8` (hyphen). String correction recommended (§3.2). No other
  cell in the package requires byte-exactness and fails it.
- **Convention 3 / addendum 3 selectability:** re-derived mechanically —
  DEL-08-01 4 YES (DECL-005 + REM-001..003, three ungated residuals),
  DEL-08-04 2 YES (DECL-005 + REM-001, one ungated residual), all other rows
  NO (08-05's sole residual is owner/stage-gated → NO; 08-02/03/06 have no
  non-bootstrap residual → NO per addendum 12). All match the ledgers and the
  R1 inventory grain; the owner suspension is kept run-level everywhere.
- **Addendum-6 ladder / weakest-leg calibration:** all DECL prose rows
  `NOT_APPLICABLE`; zero `VETTED` and zero `REVIEWED` claimed anywhere in the
  package — every technical row keys `UNVERIFIED` to the weakest load-bearing
  leg (agent-generated crate/script/schema evidence, human disposition
  pending; the reversed Gate-5 approvals are consistently NOT used to lift
  rows to REVIEWED). The W2 leg-keying risk does not recur in PKG-08.
- **Addendum-10 qualifiers:** used only by DEL-08-02 and DEL-08-03 (both over
  the RUST-09/RUST-sweep ancestor `e648462f1d05`). This verifier
  independently re-ran both: `git merge-base --is-ancestor` confirms
  ancestry, and `git diff` is **empty** over `core/` (08-02's claim) and over
  `core/reporting/report_sections/ + schemas/report_sections.schema.yaml +
  tests/test_report_sections_contract.py` (08-03's claim). Both qualifiers
  are backed by diffs the pilots actually ran and scoped only to diffed paths.
  DEL-08-01/04/05/06 correctly use no qualifier (no ancestor diff run).
- **Addendum-13:** DEL-08-02 REQ-002 (load-bearing pending-human finding
  PKG02-001) carries MEDIUM + OWNER — conforms. REQ-006's HIGH + NO beside
  the pending PKG02-002 finding rests on the disclosed metadata-only/not-
  load-bearing reading — QUALIFIED (§2.2), same scope question W1/W2 already
  queued for R3.
- **Addendum-11 / ACCEPTED_DIVERGENCE:** zero rows in the package (verified
  consistent: no divergence disposition is claimed anywhere).
- **DecisionBasis resolvability (convention 7):** every cited basis resolves
  in the frozen tree — DEC-018/021/028/058/061/065 rows in
  `SOFTWARE_DECOMP.md` §12, the `D-20_protected_content_scan.md` packet,
  SCA-001/SCA-002 (`ACCEPTANCE_RECORD.md` present), both
  `LIFECYCLE_CORRECTION_2026-05-11_2052` / `_2026-07-02_2050` Decision_Logs,
  `Reconciliation_Run_Summary_2026-05-02_DEL0805_CANDIDATE_E0621.md`, the H5
  RFC-8785 run record, and `WORKING_ITEMS_RUN_2026-07-10_TP-E3-CONTAINER-001`
  (its "Runner binding status" section verified). **No ATTESTED markers were
  needed or used in PKG-08**, and the `not re-executed at frozen SHA
  551f84ef6` marker is applied correctly to every recorded-only citation
  (desktop Vitest suites, DEV-001 PKG-02 audits, sweep passes).
- **Static test counts** independently recounted from frozen sources:
  report_generator 10, report_renderer 8, report_package 12, pdf_emitter 8,
  audit_manifest 13, report_sections 13, result_export 12,
  protected_content_linter 15, `test_release_candidate_scan.py` 14,
  `test_state_comparison_handoff_report_sections.py` 9, `test_results_schema.py`
  2 — all match the ledgers and `VERIFICATION_INDEX.csv` (RUST-09 and the
  PY-61 script-style 0-collectable row cross-checked).

## 2. Per-ledger verification tables

### 2.1 DEL-08-01 (23 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (19 ALIGNED / 4 STALE; types 10/1/3/6/3) | PASS |
| 2 | `RecordedRemaining` byte-exact vs frozen `_STATUS.md` (DECL-005 4-item join + REM-001..003 individually; bootstrap excluded from gate/source) | PASS |
| 3 | Selectability re-derivation: 4 YES (DECL-005, REM-001..003 all UNGATED) matches inventory | PASS |
| 4 | REQ-002 self-flag (field breadth via invented fixture only; ALIGNED at contract/omission-rule grain, MEDIUM): fixture present with the enumerated members; grain disclosed in-row | PASS |
| 5 | REQ-007 self-flag (nonlinear/IP-boundary classes fixture-only): same pattern, disclosed | PASS |
| 6 | REQ-008 self-flag (Phase C rule-pack-binding deferral): MEMORY 2026-06-11 "rule-pack refs → Phase C" verified in frozen file; contract-grain ALIGNED defensible | PASS |
| 7 | ACC-001 self-flag (acceptance census judgment): kit + semantic artifacts present; `Dependencies.csv` 18 data rows; the Procedure-named `check_four_documents.sh`/`check_min_viable_fileset.sh` verified absent (`tools/validation/` = one file) and correctly disclosed in-row | PASS |
| 8 | DECL-001 STALE: Specification "This setup session does not implement renderer source…" verified verbatim vs implemented report_generator/renderer/package/pdf_emitter + schema + fixture + tests | PASS |
| 9 | DECL-002 STALE: Datasheet "SEMANTIC_READY setup artifacts prepared" verified | PASS |
| 10 | DECL-003 STALE: Guidance OQ-08-01-001/002 verified overtaken (DEC-021 HTML renderer, DEC-061 PDF emitter, schema fields defined); OQ-003/004 verified genuinely surviving — RemainingWork names exactly those | PASS |
| 11 | DECL-004 STALE: Procedure names nonexistent checker tools (absence verified) | PASS |
| 12 | DECL-006 ALIGNED-with-note + rev-drift home: the `## Remaining TBDs` companion block and its 2026-06-11/2026-06-06 in-file corrections verified; **rev-0.7/DAG-006 drift confirmed to live only on non-census `_CONTEXT`/`_REFERENCES` + the dated 2026-06-04 entry** (no census surface carries a revision number — grep across all four kit docs) — the pilot's departure from calibration-1's assumed home is fact-correct; package-level adjudication §3.4 | QUALIFIED |
| 13 | ALIGNED extra spot 1 — REQ-001: all four crates + fixture + contract test present; static counts 10/8/12/8 match every cited pass | PASS |
| 14 | ALIGNED extra spot 2 — REQ-006: renderer unit rows + desktop seam (`reportRenderService.ts`, `render_calculation_report` Tauri command) verified; recorded Vitest 53/53 and 399/399 verified in MEMORY and correctly marked not re-executed | PASS |
| 15 | Evidence-execution (addendum 9): the pilot's `cargo test` on `report_generator` **wrote `core/reporting/report_generator/Cargo.lock` into the frozen tree** (git-ignored, untracked, mtime 2026-07-12 05:31; the crate's lockfile is gitignored and was absent at the SHA) — the in-row "re-executed side-effect-free … frozen tree porcelain clean after" claim is falsified as to side-effect-freedom (§3.1) | FAIL |

Tally: 13 PASS / 1 QUALIFIED / 1 FAIL.

### 2.2 DEL-08-02 (18 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (15 ALIGNED / 3 STALE; types 10/7/1) | PASS |
| 2 | Bootstrap-only `RecordedRemaining` byte-exact (incl. the `§§6–8` en dash — the notes record the pilot caught and fixed its own hyphen substitution) | PASS |
| 3 | REQ-002 self-flag (JCS at project/hash-seam grain): `ProjectLocalDeterministicJson` + doc-comment naming the shared renderer verified in frozen `lib.rs`; `canonical_json` crate + `hashService.ts` + H5 run record all resolve; `Review_Findings.csv` DEL-08-02-PKG02-001 WARNING/TECHNICALLY_ADDRESSED_PENDING_HUMAN verified; MEDIUM + OWNER per addendum 13 | PASS |
| 4 | REQ-008 SECURITY encoding (§3.3): exact em-dash marker + OWNER + no downgrade; mechanism symbols (`PrivacyClass`, `ProtectedSuspected`, `PrivatePayloadNotRedacted`) verified in frozen source | PASS |
| 5 | REQ-009 self-flag (tri-state via distinct manifest fields): `ProfessionalBoundary` struct + fields verified; interpretive call disclosed, MEDIUM | PASS |
| 6 | DECL-001 STALE: "This setup run does not implement hashing code…" verified verbatim vs the 13-test implemented crate | PASS |
| 7 | DECL-002 STALE: "Draft setup artifact; not implementation" verified | PASS |
| 8 | DECL-004 STALE: "documentation only; no hashing code…" verified | PASS |
| 9 | DECL-003 Guidance ALIGNED (advisory-principles reading; stricter STALE reading disclosed): facts support it — no unimplemented-state declaration, conditional satisfied; package Guidance-split adjudication §3.5 | QUALIFIED |
| 10 | DECL-005 MEMORY ALIGNED-with-note: undated "passed 9 focused tests" block and the dated 2026-06-06 "passed 13 focused tests" correction **both verified in the frozen file** (frozen crate has 13) — the W3 undated-MEMORY-header calibration item's corrected-in-file path applied exactly | PASS |
| 11 | DECL-007 README census: self-identification verified; content matches the frozen crate — W3 README-grain calibration item satisfied | PASS |
| 12 | REQ-006 HIGH + NO beside pending INFO finding PKG02-002 (disclosed metadata-only reading) — addendum-13 cap-scope question, carried to R3 with the W1/W2 queue | QUALIFIED |
| 13 | ALIGNED extra spot — REQ-001/REQ-003: hash/asset symbols at the cited line ranges verified; RUST-09 corroboration re-verified (ancestor OK; diff empty over `core/`) | PASS |
| 14 | Evidence-execution: the item-12 byte-identical out-of-tree copy pattern applied in full (identity diff, external target dir, porcelain, in-row disclosure); no artifact written into the frozen tree (audit_manifest has no lockfile and none appeared) — the cleanest execution discipline in the package | PASS |

Tally: 12 PASS / 2 QUALIFIED / 0 FAIL.

### 2.3 DEL-08-03 (20 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (16 ALIGNED / 4 STALE; types 12/6/2) | PASS |
| 2 | Bootstrap-only `RecordedRemaining` byte-exact | PASS |
| 3 | Addendum-10 qualifier independently re-run: ancestor confirmed; diff empty over exactly the three claimed paths; qualifier scoped only to actually-diffed paths (calibration-conformant) | PASS |
| 4 | Global contract-grain choice (all 12 REQ rows judged at report-section contract grain, renderer deferred to DEL-08-01): disclosed as the dominant judgment axis, concurring recorded agent review; a rendered-report grain would move rows to PARTIALLY — package-level R3 item §3.5 | QUALIFIED |
| 5 | REQ-004 self-flag (rule-pack checksum by reference): verified — no first-class checksum field in the contract; `report_renderer` `rule_pack_references` + `ChecksumRef` present | PASS |
| 6 | REQ-005 self-flag (canonical notice wording TBD): Guidance Q-005 verified open; boundary structurally enforced | PASS |
| 7 | REQ-009 self-flag (softest ALIGNED; reproducibility by reference, no integration fixture): facts verified; disclosed alternative PARTIALLY | QUALIFIED |
| 8 | REQ-011 process-grain (lint-fallback discipline; linter now exists): Procedure fallback guard + DEL-08-05 linter presence verified | PASS |
| 9 | DECL-001/DECL-002 STALE: future-artifacts framing and "no report code is implemented / field names TBD" verified vs implemented crate+schema | PASS |
| 10 | DECL-003 STALE: Q-002/003/004 verified overtaken (schema / DEL-08-05 / DEL-08-02 exist); Q-001/Q-005 verified surviving — RemainingWork matches exactly | PASS |
| 11 | DECL-004 STALE: `check_four_documents.sh` verified absent by listing | PASS |
| 12 | DECL-005/006 + PY-61 handling: lifecycle records resolve; script-style 41-assert direct invocation correctly distinguished from the 0-collectable pytest surface (matches the R1 index row) | PASS |
| 13 | Evidence-execution: script `main()` + schema validator only; deliberately no in-tree `cargo test` (the pilot named the exact hazard others tripped on); no artifacts written; deps VALID 15 rows re-verified by count | PASS |

Tally: 11 PASS / 2 QUALIFIED / 0 FAIL.

### 2.4 DEL-08-04 (23 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (20 ALIGNED / 3 STALE; types 12/1/3/6/1) | PASS |
| 2 | `RecordedRemaining` byte-exact (residual + bootstrap); UNGATED; 2 YES rows match inventory | PASS |
| 3 | DECL-003 Guidance ALIGNED (top self-flag): verified — frozen Guidance carries **no** unimplemented-state declaration and no settled open questions (its only setup-run sentence is a no-conflicts statement); widened-addendum-4 trigger genuinely absent; the deliberate departure from the 08-01 4-STALE pattern is fact-based (§3.5) | QUALIFIED |
| 4 | REQ-006 self-flag (no HUMAN_APPROVED enum value): schema comment "Automatic software-emitted analysis statuses … Human project acceptance is external and hash-bound" verified verbatim; deliberate-boundary reading defensible, alternative disclosed | QUALIFIED |
| 5 | REQ-003 grain + REM link (witnesses residual carried in RemainingWork, not downgraded): consistent with addendum-12/13 posture | PASS |
| 6 | ACC-001 (setup-kit acceptance): kit present; deps VALID 22 data rows re-verified by count | PASS |
| 7 | EXC-003 vs REQ-012 non-overlap (support-consumption vs implement-consumers): both encoded, disclosed | PASS |
| 8 | DECL-001 STALE: "This setup run does not implement exporter code…" verified verbatim | PASS |
| 9 | DECL-002 STALE: draft-setup/Construction-future declarations verified | PASS |
| 10 | DECL-004 STALE: SEMANTIC_READY-expectation overtaken (frozen `_STATUS` IN_PROGRESS) | PASS |
| 11 | DECL-006 rev-drift home (non-census `_CONTEXT`/`_REFERENCES` + dated entry only — verified; no census surface carries 0.7): same fact-correct departure as 08-01; §3.4 | QUALIFIED |
| 12 | ALIGNED extra spot — REQ-001: `results.schema.yaml` `deliverable_id const DEL-08-04` + envelope requireds verified; crate 12 static tests counted | PASS |
| 13 | REM-001 residual current: MEMORY 2026-06-17/18 unit-witness/inventory entries verified; no numeric claim promoted | PASS |
| 14 | Evidence-execution (addendum 9): the pilot's `cargo test` on `result_export` **wrote `core/reporting/result_export/Cargo.lock` into the frozen tree** (gitignored, untracked, mtime 2026-07-12 08:03) — side-effect-free claim falsified (§3.1). Note the same pilot's notes correctly reported "no README" from a listing that the foreign lockfile post-dates | FAIL |

Tally: 10 PASS / 3 QUALIFIED / 1 FAIL.

### 2.5 DEL-08-05 (22 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (17 ALIGNED / 1 PARTIALLY / 4 STALE; types 12/1/2/6/1; AuthorityNeeded NO 14 / OWNER 8; Selectable NO 22) | PASS |
| 2 | `RecordedRemaining` byte-exact incl. the full owner-gate string; `GateOrStageConstraint` copied verbatim (`gated: owner sole signatory per DEC-058; stage-gated: first release candidate`) onto DECL-005 + REM-001; Selectable NO derivation correct (gated residual) | PASS |
| 3 | **Part C SECURITY spot-check (named): the convention-6 split — PASS** (full adjudication §3.3) | PASS |
| 4 | REQ-002 (detection sufficiency deferred): synthetic markers + `standards_table_signature_findings` failing toward `UnknownProvenanceReviewRequired` verified in frozen `lib.rs`; ALIGNED-at-heuristic-grain with marker, no downgrade | PASS |
| 5 | REQ-004 (routing signals; quarantine movement TBD): review_route/pending symbols verified; quarantine-file-movement TBD verified in MEMORY Guardrails | PASS |
| 6 | REQ-010 PARTIALLY_IMPLEMENTED: `publication_gate: "blocked_pending_owner_sign_off"` verified in the DEC-058 runner; **CI-provider wiring verified absent** (no `.github/workflows` in the frozen tree; CI guard homed to DEL-10-04 per Procedure) — the partial rests on independent missing-implementation grounds, not on the sufficiency deferral (no-downgrade rule intact) | PASS |
| 7 | REM-001 gated residual: DEC-058 §12 row + `D-20_protected_content_scan.md` packet resolve; `validation/evidence/releases/` verified absent; owner-sole-signatory routing OWNER correct | PASS |
| 8 | REQ-006/REQ-012 self-flags (GOVERNANCE class + NO routing; disclosed SECURITY-adjacent/REVIEW alternatives): parallel to the DEL-08-01 exemplar rows; alternatives named for R3 | QUALIFIED |
| 9 | ACC-001: kit present; deps VALID 13 data rows re-verified by count | PASS |
| 10 | EXC-002: `Reconciliation_Run_Summary_2026-05-02_DEL0805_CANDIDATE_E0621.md` resolves; schema `candidate_edge_policy` const + DEL-11-04-absence contract test verified | PASS |
| 11 | DECL-001..004 STALE: "writing linter source code" out-of-scope list, "no linter source… implemented in this session", OQ-08-05-001..005 register (OQ-001/004/005 overtaken — dependency-free Rust crate, LintFinding schema, DEC-058 owner ownership; OQ-002/003 partly open), future-implementation Procedure — all verified in frozen kit | PASS |
| 12 | DECL-006 MEMORY ALIGNED (undated current-state blocks describe the frozen slice; TBDs verified surviving); rev-drift on dated entry + non-census `_CONTEXT` only — verified | PASS |
| 13 | ALIGNED extra spot — REQ-011: `Cargo.toml` `[dependencies]` verified empty; schema consts `heuristic_only`/`clean_scan_is_clearance` verified | PASS |
| 14 | Evidence-execution (addendum 9): the pilot's `python3 -m pytest tests/test_release_candidate_scan.py` **wrote `.pytest_cache/` into the frozen project root** (gitignored, untracked, mtime 2026-07-12 08:05; cached nodeids are exactly that module's 14 tests) — scratch `TMPDIR` did not redirect pytest's rootdir cache; side-effect-free claim falsified (§3.1) | FAIL |

Tally: 12 PASS / 1 QUALIFIED / 1 FAIL.

### 2.6 DEL-08-06 (26 rows) — SOUND

| # | Check | Result |
|---|---|---|
| 1 | Histogram recount (22 ALIGNED / 4 STALE; types 14/1/5/6/0); Selectable NO everywhere (bootstrap-only `## Remaining` verified) | PASS |
| 2 | Bootstrap `RecordedRemaining` byte-exactness: **FAIL by one character** — frozen `§§6–8` (en dash) transcribed `§§6-8` (hyphen). The notes' "recorded byte-exact" claim is false for this cell; everything else about the bootstrap handling (exclusion variant, no REM row, gate/source scoping) is correct. String correction §3.2 | FAIL |
| 3 | REQ-002 self-flag (SOW-024 coverage-map grain, fixture-only breadth): `_sow_024_coverage` verified in engine | PASS |
| 4 | REQ-006 self-flag (REPORTING not MECHANICS): metadata-presence checks verified (`SCH-COMPARISON-DELTA-UNIT-MISSING`); consistent with 08-01 REQ-006 | PASS |
| 5 | REQ-008 self-flag (key-redaction + invented-fixture grain; linter excluded by Scope): `PRIVATE_PAYLOAD_KEYS` + `_safe_public_context` verified; EXC-004 records the linter exclusion | PASS |
| 6 | REQ-011 self-flag disclosure verified in source: `model_state_comparison` hardcodes empty `unit_normalized_deltas`/`tolerance_profile_refs` (engine L348-349) while `analysis_run_comparison` populates them — exactly as disclosed; threshold policy is a requirement-declared TBD | PASS |
| 7 | REQ-013 (schema-first via typed Mapping envelopes; fragments TBD): SCA-002 `ACCEPTANCE_RECORD.md` resolves; no-bypass module docstring verified; parallel to 08-01 REQ-009 | PASS |
| 8 | REQ-014 "layered tests" = one 9-test Python module (soft reading, disclosed; Verification permits TBD gates) | QUALIFIED |
| 9 | ACC-001 on a thinner basis (kit has no `## Acceptance Criteria For This Setup Session` heading — disclosed); deps VALID 29 data rows re-verified by count | QUALIFIED |
| 10 | DECL-001 STALE at MEDIUM (hedged Scope prose; Verification evidence-state TBD-throughout basis verified) | PASS |
| 11 | DECL-002 STALE + rev-drift fold-in: **the only PKG-08 census surface carrying the rev-0.7/DAG-006 pointer** ("accepted revision 0.7", "22 ACTIVE approved DAG-006 rows" verified in the frozen Datasheet) — W1 STALE-side adjudication applied on the correct surface, with the caveat raised once; calibration-conformant | PASS |
| 12 | DECL-003/DECL-004 STALE: future-implementation Considerations/Steps + all-TBD Verification verified vs implemented engine + 9 passing tests | PASS |
| 13 | DECL-005/DECL-006: lifecycle records resolve; MEMORY verified all-dated (head is a dated 2026-05-07 entry; no undated setup-era block) | PASS |
| 14 | Evidence-execution (addendum 9): the pilot's `python3 -m py_compile` **wrote `__pycache__` bytecode into the frozen tree** (`core/reporting/state_comparison_handoff_sections/__pycache__/engine.cpython-313.pyc` + `tests/__pycache__/test_state_comparison_handoff_report_sections.cpython-313.pyc`, both mtime 2026-07-12 08:00:04, matching the two-file py_compile invocation; `PYTHONDONTWRITEBYTECODE` does not suppress explicit `py_compile` output). The pytest-avoidance reasoning in the notes was correct; the py_compile step defeated it (§3.1) | FAIL |

Tally: 12 PASS / 2 QUALIFIED / 2 FAIL.

## 3. Adjudications and cross-ledger findings

### 3.1 Frozen-tree containment breach by four pilots (addendum 9) — FAIL spot-checks; ledgers remain SOUND; wave-level escalation

`git status --porcelain --ignored=matching` over the frozen worktree shows six
untracked git-ignored artifacts. Four are attributable to W3 PKG-08 pilots by
path and mtime:

1. `core/reporting/report_generator/Cargo.lock` — 2026-07-12 05:31 — DEL-08-01
   pilot's in-tree `cargo test` (the crate's lockfile is gitignored and did
   not exist at the SHA; cargo writes it beside `Cargo.toml` even with an
   external `CARGO_TARGET_DIR`).
2. `core/reporting/result_export/Cargo.lock` — 2026-07-12 08:03 — DEL-08-04
   pilot, same mechanism.
3. `core/reporting/state_comparison_handoff_sections/__pycache__/` +
   `tests/__pycache__/` — 2026-07-12 08:00:04 — DEL-08-06 pilot's
   `python3 -m py_compile` (explicit compilation writes bytecode regardless of
   `PYTHONDONTWRITEBYTECODE`).
4. `.pytest_cache/` (project root) — 2026-07-12 08:05:25 — DEL-08-05 pilot's
   `python3 -m pytest tests/test_release_candidate_scan.py` (cache nodeids are
   exactly that module's 14 tests; no `-p no:cacheprovider` was used).

The remaining artifact, `validation/benchmarks/nonlinear/target/`
(2026-07-11 23:03), **pre-dates W3** and matches the W2 DEL-04-04 in-tree
`--locked` benchmark run disclosed in `W2_VERIFICATION_PKG-04.md` §3.5 — a
pre-existing instance of the same gap, discovered here and flagged for the
same cleanup.

Adjudication: addendum 9 forbids writes into the frozen tree "even on
git-ignored paths." Every pilot's porcelain check was truthful — plain
porcelain ignores ignored files — so the run's standard check cannot detect
this breach class. The test **results** are not invalidated (a generated
lockfile from a zero/fixed-dependency `Cargo.toml`, bytecode caches, and a
pytest cache do not affect pass/fail or any encoded fact), and no tracked
content changed (HEAD re-verified; tracked porcelain empty). The affected
in-row claims "re-executed side-effect-free … frozen tree porcelain clean
after" are, however, falsified as to side-effect-freedom — graded FAIL on the
evidence-execution spot-check of each of the four ledgers. Under the run's
DEFECTIVE definition (a FAIL requiring re-encoding by a fresh pilot), these
are not defects: no disposition, histogram, or evidence fact changes; the
remedy is string-level plus a containment act outside any pilot's fence.

**Recommendations (routed to the orchestrator, non-binding):**
- **Containment (before any W4 dispatch):** restore the frozen worktree —
  delete the six untracked ignored artifacts (or recreate the worktree from
  the pinned SHA) and re-verify with `git status --porcelain
  --ignored=matching` (empty) + `git rev-parse HEAD`. This matters materially:
  future byte-identity copy checks (item-12 pattern) over `result_export` or
  `report_generator` would silently copy the foreign lockfiles.
- **Run-rule tightening (candidate R3 addendum):** (a) the frozen-tree
  porcelain check is `--ignored=matching`, before and after; (b) `cargo test`
  in-tree is permitted only for crates with a committed `Cargo.lock` (else
  the item-12 out-of-tree copy pattern is mandatory — DEL-08-02 executed it
  perfectly); (c) pytest in-tree only with `-p no:cacheprovider`; (d) no
  `py_compile` against in-tree files (import-free script `main()` invocation
  with `PYTHONDONTWRITEBYTECODE=1`, as DEL-08-03 did, is the clean pattern).
- **String corrections (owning pilots):** see §3.2 items 2–5.

### 3.2 Recommended non-defect corrections (owning pilots)

1. **DEL-08-06 `DECL-005` / `RecordedRemaining`:** replace `§§6-8` with
   `§§6–8` (en dash, U+2013) to restore addendum-2 byte-exactness; also
   correct the notes §2 sentence claiming byte-exact transcription (or simply
   re-transcribe the cell and note the fix).
2. **DEL-08-01:** on rows whose `VerificationEvidence` cites the in-tree
   `cargo test` runs (REQ-001..REQ-010, EXC-003, REM-001), amend the
   `side-effect-free` qualifier to disclose the `report_generator/Cargo.lock`
   write (e.g. append `— except a generated Cargo.lock beside the
   report_generator manifest, an untracked gitignored artifact; see W3
   verification §3.1`), and mirror the disclosure in notes §3.
3. **DEL-08-04:** same amendment for the `result_export` cargo-citing rows
   (REQ-001..REQ-012, ACC-001 unaffected, EXC-002, REM-001) and notes §5.
4. **DEL-08-05:** same amendment for the two pytest-citing evidence patterns
   (REQ-002/003/004/009/010, REM-001) disclosing the `.pytest_cache/` write,
   and notes §4.
5. **DEL-08-06:** same amendment for the `py_compile`-citing evidence cells
   (all REQ rows carry it) disclosing the `__pycache__` writes, and notes §4
   (whose pytest-avoidance rationale should note that py_compile defeated it).

Corrections 2–5 may equivalently be satisfied by one standardized disclosure
string per ledger if the orchestrator prefers a uniform amendment; none
changes any disposition, histogram, or authority routing.

### 3.3 Named Part-C-style SECURITY spot-check (DEL-08-05, with DEL-08-02 REQ-008) — PASS

DEL-08-05 is the run's SECURITY-relevant reporting deliverable (report
protected-content linter, owner-gated under DEC-058). Its pilot applied the
convention-6 SECURITY encoding to **REQ-002, REQ-004, REQ-010, REM-001** and
left **REQ-001, REQ-005, REQ-008, REQ-011** verification-complete. I checked
the split row-by-row against the frozen evidence and the convention text
("SECURITY-class behavior claims whose accepted scope defers sufficiency
review: `ValidationEvidence=NONE_FOUND — sufficiency review deferred,
owner-gated`; no `VERIFIED_NOT_VALIDATED` downgrade on that ground"), plus the
W1 harmonizations (em-dash marker form; OWNER routing):

- **Marker form and routing:** all four marker cells carry the exact em-dash
  string byte-for-byte; all four route `AuthorityNeeded=OWNER`; none is
  downgraded to `VERIFIED_NOT_VALIDATED`; Confidence MEDIUM on all four —
  consistent with the W1 precedent rows (DEL-02-04 REQ-013, DEL-02-05
  REQ-013/025, DEL-03-07 REQ-003/004).
- **Marker rows genuinely defer sufficiency:** REQ-002's real-world
  detection sufficiency and REQ-004's routing sufficiency are exactly what
  DEC-058 reserves to the owner-gated release-candidate scan act (packet and
  §12 codification verified; `validation/evidence/releases/` verified absent
  — the act has not occurred); REQ-010 is the gated act's severity/CI
  enforcement (its PARTIALLY_IMPLEMENTED rests on the independently verified
  absence of CI wiring, not on the deferral — the no-downgrade rule is
  respected); REM-001 is the owner/stage-gated act itself.
- **Non-marker SECURITY rows are verification-complete on their facts:**
  REQ-001 (public-surface scoping) and REQ-005 (safe-metadata allowance) are
  deterministic behaviors with named passing tests; REQ-008 (fixture hygiene)
  is closed by the forbidden-term contract test; REQ-011 (no transmission by
  default) is a design property verified by the dependency-free `Cargo.toml`
  and absence of network I/O. Each carries `NOT_APPLICABLE` **with an explicit
  in-cell reason** that still names the DEC-058 owner-review context where
  relevant — no sufficiency judgment is silently claimed. The convention's
  trigger ("whose accepted scope defers sufficiency review") genuinely does
  not fire for these four.
- **DEL-08-02 REQ-008** (the package's other SECURITY row) applies the same
  encoding — exact marker, OWNER, ALIGNED-no-downgrade, MEDIUM — with the
  guardrail mechanism verified in the frozen crate. Consistent across the
  package.

**Verdict: PASS.** The split is convention-correct and, in my judgment, the
right reading of convention 6: marker where the requirement's own satisfaction
question is deferred to the owner-gated act, explicit-reason
`NOT_APPLICABLE` where verification closes the claim. Recommend R3 adopt this
"sufficiency-deferral-only" scoping as the run-level rule so later SECURITY
deliverables do not blanket-apply the marker to every SECURITY-class row.

### 3.4 rev-0.7/DAG-006 pointer drift lands off-census in 5 of 6 kits — pilots' departure from calibration 1's assumed home is fact-correct

Verified by grep across all 24 kit census documents: only DEL-08-06's
Datasheet carries the rev-0.7/DAG-006 pointer on an addendum-1 census surface
(encoded STALE-side there, correctly). For 08-01..08-05 the drift lives only
on non-census `_CONTEXT.md`/`_REFERENCES.md` and the dated 2026-06-04
`TP-AUTHORITY-REFRESH-0_7-DAG006` MEMORY entries. All five pilots recorded it
as an in-row MEMORY note (dated entry → note per addendum 1), declined to mint
non-census `_CONTEXT` DECL rows, and raised the owner-calibration caveat
exactly once per notes. This is internally consistent across the package and
consistent with the W1 adjudication's intent (STALE where a census declaration
is affected). QUALIFIED on 08-01/08-04 DECL-006 only because it is a named
departure from the calibration item's assumed home; **recommend R3 codify:**
the rev-drift STALE encoding applies only where the pointer sits on a census
DECLARED_STATE surface; elsewhere it is a MEMORY-row note + one caveat.

### 3.5 Other cross-ledger observations (package summary / R3)

- **Guidance-surface split is fact-driven, not convention drift:**
  08-01/08-03/08-05 Guidance STALE (settled open-question registers verified
  overtaken); 08-02/08-04 Guidance ALIGNED (no unimplemented-state
  declaration; principles still true — verified). Same species as the W2
  Procedure-surface adjudication; aggregate knowingly at R3.
- **Contract-vs-rendered grain** is PKG-08's dominant judgment axis
  (08-03 globally; 08-01 REQ-002/007/008; 08-04 REQ-003/006; 08-06
  REQ-002/008). All instances are disclosed with the deferred slice's
  downstream home named and verified. R3 should read PKG-08 ALIGNED
  requirement rows as contract-grain dispositions.
- **ACC-row minting variance:** 08-01/04/05/06 mint one setup-session
  acceptance row; 08-02/03 mint zero (restatement-only verification tables).
  Both readings have W1/W2 precedent; the 08-06 instance is the thinnest (no
  acceptance heading). Candidate R3 rule: mint ACC only where a kit carries a
  distinct acceptance section beyond requirement restatement.
- **Boilerplate-wide evidence cells** (08-04 repeats its full 12-test
  enumeration in every row's `VerificationEvidence`): aggregation-benign but
  bulky; a cite-once-in-notes alias would help W4/W5.
- **Zero** `IMPLEMENTED_UNMAPPED`, `UNKNOWN`, `AUTHORITY_CONFLICT`,
  `ACCEPTED_DIVERGENCE`, `VETTED`, and `REVIEWED` rows package-wide; orbit
  surfaces spot-checked against `IMPLEMENTATION_SURFACES.csv`
  (SURF-106/108/113/114/129/200/201/202/223 all resolve as claimed).

## 4. Fence compliance

All six ledgers and notes: no lifecycle/DAG/scope mutation proposed as
operative (all repair candidates routed via `AuthorityNeeded`/`RemainingWork`;
`LIFECYCLE_REASSESSMENT_REQUIRED` never applied); no F-PIP-1..5 claim language
outside clearly attributed quotes of the deliverables' own negative
obligations (this package's subject matter — certification/sealing/compliance
boundaries and protected content — is quoted and attributed throughout, never
asserted); agent dispositions nowhere phrased as rulings;
`SelectableUnderCurrentLoop` kept mechanical with the owner suspension
run-level; gate-state cells reflect the frozen register (D-41
AWAITING_RULING) per the RUN_BASIS codification. Each pilot's writes were
confined to its two output files **except** the four frozen-tree ignored-
artifact writes adjudicated in §3.1 (a containment breach, disclosed there
with remediation; no tracked file was touched and no record/register was
edited). This verifier wrote exactly this one file.

## 5. Frozen-tree status

`git rev-parse HEAD` = `551f84ef6be656f1603ce0acfa5e3935aa9683c7` and tracked
porcelain empty, re-verified at verification start and end. All verification
operations were read-only (git status/ls-files/diff/merge-base, grep, ls,
stat, file reads); no build, test, or compile execution was performed by this
verifier; the six pre-existing untracked ignored artifacts (§3.1) were
enumerated but not touched — their removal is outside this verifier's fence
and is escalated to the orchestrator.

## 6. Package summary line

**PKG-08: 6/6 ledgers SOUND (132 rows; spot-checks 70 PASS / 11 QUALIFIED /
5 FAIL — all FAILs string-correctable, none re-encoding-grade); named Part-C
SECURITY spot-check on the DEL-08-05 convention-6 split: PASS (plus DEL-08-02
REQ-008 consistent); one wave-level escalation: four pilots wrote git-ignored
artifacts into the frozen tree (two Cargo.locks, two __pycache__/.pytest_cache
writes) invisible to plain porcelain — restore the frozen worktree and adopt
the `--ignored=matching` + copy-out/no-cacheprovider/no-py_compile rules
before W4 (§3.1); five recommended owning-pilot string corrections (§3.2:
DEL-08-06 en-dash byte-exactness fix; four side-effect-disclosure amendments);
cross-ledger items for R3: rev-drift off-census home rule, Guidance-surface
fact-split, contract-grain reading, ACC-minting rule, SECURITY
sufficiency-deferral-only marker scoping (§3.3–§3.5).**
