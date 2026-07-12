# W4 Fan-in Verification — PKG-10 (DEL-10-01..05)

Verifier: **highest-capability GPT-5**, high-effort adversarial fan-in. Scope:
the five W4 discovery ledgers and notes for PKG-10, verified independently
against the frozen evidence worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Method authority:
`R1_CONVENTIONS.md` Parts A–D, pinned plan §§6–8, R0/R0b calibration and
review records, the binding W1–W3 carry-forward sections in
`PACKAGE_SUMMARIES/PKG-00..08.md`, and the W1–W3 package-verification
exemplars. All dispositions and verdicts below are agent judgments, not human
or engineering rulings. This verifier did not edit any ledger or notes file.

Verification covered every row self-flagged in a ledger or notes file, every
non-`ALIGNED` row, at least two `ALIGNED` rows per ledger, the full mechanical
convention sweep, evidence/basis resolvability, SECURITY-marker scoping, and
addendum-9 containment.

## Verdict summary

| Ledger | Verdict | Checks (PASS / QUALIFIED / FAIL) |
|---|---|---:|
| DEL-10-01 | **SOUND** | 11 / 0 / 0 |
| DEL-10-02 | **SOUND** | 14 / 1 / 0 |
| DEL-10-03 | **SOUND** | 12 / 0 / 0 |
| DEL-10-04 | **SOUND** | 13 / 0 / 0 |
| DEL-10-05 | **SOUND — owning-pilot corrections required** | 10 / 1 / 2 |
| **PKG-10 total** | **5 SOUND / 0 DEFECTIVE** | **60 / 2 / 2** |

The two FAILs are localized, deterministic corrections and do not require a
fresh-pilot re-encoding; therefore DEL-10-05 remains SOUND under the W3
verifier precedent. Exact correction routing is in §7.

## 1. Mechanical conformance and independent re-execution

All five CSVs parse as RFC-4180 with the exact 20-column run header, pure CRLF,
20 cells per row, controlled ClaimType/disposition enums, unique fixed-form
ClaimIDs, and contiguous per-type numbering. Total: **125 rows**.

- ClaimType: 54 REQUIREMENT / 17 ACCEPTANCE / 14 EXCLUSION / 30 DECLARED_STATE
  / 10 REMAINING_WORK.
- Disposition: 92 ALIGNED / 8 PARTIALLY_IMPLEMENTED / 6 ACCEPTED_DIVERGENCE /
  17 STALE_SETUP_SPECIFICATION / 1 STALE_REVIEW_OR_EVIDENCE /
  1 REMAINING_STATE_MISMATCH.
- All five notes' ClaimType and Disposition histograms reproduce from their
  ledgers. DEL-10-05's stated selectability histogram does not; see §6.
- Addendum-1 census is six declaration surfaces per ledger: four-document kit,
  `_STATUS.md`, and `MEMORY.md`; no deliverable-owned README was found.
- The D-41 bootstrap is confined to each `_STATUS.md` surface row and excluded
  from residual/selectability analysis.

Independent checks at the frozen SHA:

| Check | Result |
|---|---|
| DEL-10-01 API boundary and plugin-manifest static scripts | PASS; direct execution, no pytest-collection claim |
| DEL-10-02 adapter framework pytest | **14 passed** |
| Adjacent analytical boundary adapter pytest | **7 passed** |
| DEL-10-03 local-FEA contract pytest | **1 passed** |
| DEL-10-04 release-readiness + evidence-sweep pytest | **24 passed** |
| DEL-10-04 public-export non-cargo subset | **16 passed, 4 deselected** |
| Adjacent export-adapter SDK pytest | **8 passed** |
| DEL-10-05 Python contract script | PASS by direct execution |
| DEL-10-05 copied-tree Cargo tests | **16 library + 1 preview-binary + 5 final-binary passed** |

Python used `PYTHONDONTWRITEBYTECODE=1` and pytest used
`-p no:cacheprovider`. The lockless headless crate was copied outside the
frozen worktree together with its relative path dependencies and fixtures;
Cargo build output was external. Two preliminary scratch-only copy attempts
failed before test execution because relative dependencies/fixtures were not
yet copied; their scratch directories were removed and they made no frozen
write.

## 2. DEL-10-01 — Public API and plugin boundary — SOUND

| # | Row / check | Independent result | Verdict |
|---|---|---|---|
| 1 | Structure, census, histograms, bootstrap | 27 rows; 15/1/4/6/1 type census and 21/2/3/1 disposition census reproduce; conventions pass | PASS |
| 2 | REQ-002 PARTIALLY_IMPLEMENTED, self-flag | `x_operation_registry` covers import/export, solve, results, rule packs, reports and validation, but has no explicit model-creation or load-case-definition family | PASS |
| 3 | REQ-010 PARTIALLY_IMPLEMENTED, self-flag | analysis statuses distinguish model/rule/mechanics/human states, but no explicit `INVALID_INPUT` status or diagnostic class exists | PASS |
| 4 | DECL-001 STALE | Specification says repository-level API files are not edited; `api/api_boundary_contract.yaml`, plugin-boundary docs and tests exist | PASS |
| 5 | DECL-002 STALE | Datasheet declares deliverable-local/non-implementation form; the mapped strict repo-level boundary exists | PASS |
| 6 | DECL-003 STALE | Guidance still calls the contract deliverable-local/conceptual and exact fields/TBDs despite concrete registries and fields | PASS |
| 7 | REM-001 mismatch | finding `PKG10-DEL1001-PKG02-W001` is `TECHNICALLY_ADDRESSED_PENDING_HUMAN`, `HumanDisposition=TBD`, and is absent from all checked `## Remaining` homes | PASS |
| 8 | REQ-001 ALIGNED sample | strict JSON Schema 2020-12 boundary carries envelope and command/query/job registries; direct script passes | PASS |
| 9 | REQ-012 SECURITY sample | deny-by-default permission metadata is real while enforcement sufficiency remains deferred; exact marker, MEDIUM and OWNER are correct | PASS |
| 10 | REQ-014 pending-human sample | JCS/binary-manifest fields exist independently; pending finding is non-load-bearing, MEDIUM+OWNER satisfies addendum 13 | PASS |
| 11 | Procedure/README census self-flag | Procedure is self-scoped to the completed setup run, so ALIGNED-with-note follows PKG-06/07 calibration; no owned README exists | PASS |

## 3. DEL-10-02 — Import/export adapter framework — SOUND

| # | Row / check | Independent result | Verdict |
|---|---|---|---|
| 1 | Structure, census, histograms, bootstrap | 27 rows; notes reproduce; D-12/v0.2 residual and bootstrap encoding are mechanically correct | PASS |
| 2 | REQ-007 PARTIALLY_IMPLEMENTED | rule-hook declaration controls exist; no concrete runtime hook/execution model exists | PASS |
| 3 | REQ-008 PARTIALLY_IMPLEMENTED | report-boundary declarations exist; no concrete adapter/report runtime hook exists | PASS |
| 4 | ACC-007 PARTIALLY_IMPLEMENTED | sandbox/no-bypass acceptance is contract-only; end-to-end runtime hook evidence is absent | PASS |
| 5 | ACC-008 PARTIALLY_IMPLEMENTED | warning/limitation/notices acceptance is contract-only; end-to-end report hook evidence is absent | PASS |
| 6 | DECL-001 STALE | Specification says no source/tests/sample/repo artifacts; schema/framework/fixture/tests/UI exist | PASS |
| 7 | DECL-002 STALE | Datasheet's setup/future declaration is overtaken by the same mapped implementation | PASS |
| 8 | DECL-003 STALE | Guidance prepares later implementation although the format-neutral shell and panel landed | PASS |
| 9 | DECL-004 STALE | Procedure is future-only rather than a self-scoped historical procedure; landed schema/framework/fixture/hardening falsify it | PASS |
| 10 | REQ-001 ALIGNED sample | schema/framework enforce metadata, validation, diagnostics and no-bypass controls | PASS |
| 11 | REQ-005 SECURITY sample | deterministic export privacy checks are contract facts, not a sufficiency claim; no marker is appropriate | PASS |
| 12 | ACC-004 SECURITY self-flag | private-data export check exists at contract grain; GUI round-trip remains separately homed | PASS |
| 13 | ACC-005 SECURITY self-flag | refreshed invented fixture and protected-origin controls were human-accepted on 2026-06-07; no broad clearance is claimed | PASS |
| 14 | REM-001 ALIGNED self-flag | status text is byte-faithful; D-12 and v0.2 R6 gates make it non-selectable | PASS |
| 15 | Eight-row acceptance census self-flag | eight distinct Verification bullets exist and are re-keyed one-for-one; accepted, but ACC counts remain calibration-grain sensitive | QUALIFIED |

The recorded 27-test package fan-in resolves in `_REVIEW.md`; the focused
adapter module independently re-executed 14/14 at the frozen SHA. No stale
27-test count was promoted into a new validation claim.

## 4. DEL-10-03 — Local FEA handoff data contract — SOUND

| # | Row / check | Independent result | Verdict |
|---|---|---|---|
| 1 | Structure, census, histograms, bootstrap | 28 rows; notes reproduce; D-12 residual is gated/non-selectable | PASS |
| 2 | REQ-004 SECURITY self-flag | source/privacy/review fields exist; real-payload sufficiency is deferred; marker+OWNER correct | PASS |
| 3 | REQ-007 SECURITY self-flag | schema/reference-only protected/private boundary exists; content sufficiency remains deferred; marker+OWNER correct | PASS |
| 4 | EXC-002 SECURITY self-flag | protected/private/proprietary payload exclusion is real at schema/reference grain; marker scope correct | PASS |
| 5 | DECL-001 STALE | Specification says no repo schema/source is created; strict DEL-10-03 schema and integration exist | PASS |
| 6 | DECL-002 STALE | Datasheet current-artifact and rev-0.7 authority statements are overtaken | PASS |
| 7 | DECL-003 STALE | Guidance still frames implementation as future despite the implemented contract | PASS |
| 8 | DECL-004 STALE | Procedure remains setup/future-only despite the implemented contract and integration | PASS |
| 9 | REQ-003 ALIGNED sample | units manifest and preservation witnesses explicitly carry coordinates/forces/moments/displacements/rotations/stresses and dimensions | PASS |
| 10 | REQ-008 ALIGNED sample | schema-first no-bypass references bind to the API/adapter boundary; no runtime external adapter is claimed | PASS |
| 11 | REM-001 ALIGNED self-flag | distinct FR-025 export/deferral remains homed behind D-12; preview JSON is not mislabeled as FR-025 | PASS |
| 12 | Six-row acceptance census | six distinct interim setup gates exist; re-keying is faithful | PASS |

Focused contract re-execution passed and independently confirmed format,
adapter, mesh generation and external invocation remain `TBD`, with explicit
privacy, unit, diagnostic and non-reliance controls.

## 5. DEL-10-04 — Build, packaging, and CI/CD pipeline — SOUND

| # | Row / check | Independent result | Verdict |
|---|---|---|---|
| 1 | Structure, census, histograms, bootstrap | 23 rows; notes reproduce; four residuals and multi-residual gate order are correct | PASS |
| 2 | REQ-001 ACCEPTED_DIVERGENCE | DEC-025/057/059 expressly permit local/provider-neutral, unsigned-v0.1 and conditionally deferred hosted CI posture | PASS |
| 3 | REQ-003 ACCEPTED_DIVERGENCE | DEC-057 expressly narrows v0.1 to macOS Apple Silicon `.app` zip/checksum and defers expansion | PASS |
| 4 | DECL-001 STALE | future/no-artifact Specification is overtaken by readiness, packaging and export implementation | PASS |
| 5 | DECL-002 STALE | Datasheet SEMANTIC_READY/setup-only declaration is overtaken by IN_PROGRESS implementation and rulings | PASS |
| 6 | DECL-003 STALE | Guidance says no workflow/package/release artifacts exist and leaves ruled choices open | PASS |
| 7 | REM-001 ACCEPTED_DIVERGENCE | DEC-059 explicitly permits conditional first-publication activation behind named owner prerequisites; marker+OWNER correct | PASS |
| 8 | REM-002 ACCEPTED_DIVERGENCE | DEC-057 explicitly permits unsigned v0.1 and requires later D-06b/explicit deviation | PASS |
| 9 | REQ-006 SECURITY self-flag | fail-toward-exclusion export controls exist; owner-signed D-20 scan sufficiency is genuinely deferred; marker+OWNER correct | PASS |
| 10 | REM-003 ALIGNED self-flag | browser-provisioning policy is homed, ungated and mechanically selectable without activating CI | PASS |
| 11 | REM-004 ALIGNED self-flag | provider mapping is homed, ungated and selectable while activation remains separately gated | PASS |
| 12 | DECL-005 multi-residual surface | four status residuals retain source order; two ungated items make the surface selectable; bootstrap excluded | PASS |
| 13 | Evidence execution | 24 readiness/sweep tests and 16 non-cargo export tests independently pass under W4 controls | PASS |

## 6. DEL-10-05 — Headless CLI and structured I/O runner — SOUND, corrections required

| # | Row / check | Independent result | Verdict |
|---|---|---|---|
| 1 | Structure and census | CSV structure, row counts, IDs, controlled dispositions and claimed ClaimType/Disposition histograms pass | PASS |
| 2 | REQ-001 PARTIALLY_IMPLEMENTED | bounded schema-first local runner exists; persisted-project/full operation breadth remains absent | PASS |
| 3 | REQ-004 PARTIALLY_IMPLEMENTED | references and optional in-memory envelope validation exist; export-results payload/container binding remains open | PASS |
| 4 | DECL-001 STALE | mixed Specification still opens with “future runner” and says exact schema files TBD despite the landed runner/schema | PASS |
| 5 | DECL-003 STALE | Guidance says future runner/no executable examples/exact commands TBD, overtaken by DEC-065 and TP-RUNNER-015 | PASS |
| 6 | DECL-004 STALE | Procedure says no runner is implemented, directly overtaken | PASS |
| 7 | REM-001 ACCEPTED_DIVERGENCE | DEC-065 expressly permits an operation stub pending downstream export binding | PASS |
| 8 | REM-002 ACCEPTED_DIVERGENCE | DEC-065 expressly permits benchmark/regression evidence stubs pending downstream binding | PASS |
| 9 | REM-003 STALE_REVIEW_OR_EVIDENCE | witness has exactly 822 refs; Receipt 9/status record live output at 830; no refreshed witness exists | PASS |
| 10 | ACC-001 ALIGNED self-flag | acceptance behavior is supported by current copied-tree Cargo re-execution (16+1+5 pass), but its cited committed solve witness is overtaken; residual is correctly explicit | QUALIFIED |
| 11 | REQ-007 SECURITY | row admits no dedicated provenance review and Specification explicitly defers fixture protected-content/provenance review; this is a sufficiency deferral but lacks the exact marker and OWNER routing | **FAIL** |
| 12 | REQ-008 SECURITY sample | DEC-065 local foreground/no-network/no-telemetry/no-hidden-write policy and schema consts are deterministic; no sufficiency marker is needed | PASS |
| 13 | Gate/selectability mechanical sweep | nine no-residual rows incorrectly use `UNGATED`; three rows carrying ungated residuals incorrectly use `SelectableUnderCurrentLoop=NO`; notes therefore undercount YES 4 instead of 7 | **FAIL** |

The requirement and residual dispositions themselves remain supportable. The
FAILs are controlled-cell/routing corrections, not a fresh-census or
fresh-disposition exercise.

## 7. Exact correction routing — owning DEL-10-05 pilot only

Do not repair centrally. Route these changes to the DEL-10-05 discovery
pilot, preserving all other cells and semantics:

1. In `CLAIM_CONCORDANCE_DEL-10-05.csv`, change
   `GateOrStageConstraint` from `UNGATED` to `NONE_RECORDED` on exactly:
   `REQ-001`, `REQ-003`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`,
   `REQ-009`, `EXC-001`, and `DECL-001`.
2. Change `SelectableUnderCurrentLoop` from `NO` to `YES` on exactly:
   `REQ-002`, `REQ-004`, and `ACC-001`, because each carries an ungated
   recorded residual. Leave `DECL-005` and `REM-001..003` at `YES`.
3. In `NOTES_DEL-10-05.md`, change the selectability recount from
   `YES 4 / NO 16` to **`YES 7 / NO 13`**.
4. On `REQ-007`, replace `ValidationEvidence` with the standardized exact
   marker `NONE_FOUND — sufficiency review deferred, owner-gated` and change
   `AuthorityNeeded` from `NO` to `OWNER`. Keep `Disposition=ALIGNED` and
   `Confidence=MEDIUM`; the defect is marker/routing, not a downgrade.
5. Update the notes SECURITY sentence so REQ-007 is described as the
   owner-gated fixture-provenance/protected-content sufficiency row, while
   REQ-008 remains the deterministic local/no-transmission row without a
   marker.
6. Re-run the full structural/enums/CRLF/ID/histogram/selectability checks and
   report the exact correction diff to this verifier/orchestrator.

No correction is required for ACC-001's qualification: this verifier's
current copied-tree Cargo pass independently confirms the bounded behavior,
while REM-003 correctly preserves the stale-witness fact. The qualification
should remain visible in the package summary/R3 evidence reading.

## 8. Cross-ledger adjudications

- **SECURITY marker:** PKG-10 follows the binding sufficiency-only rule except
  DEL-10-05 REQ-007. Correct marker rows are DEL-10-01 REQ-012;
  DEL-10-03 REQ-004/007 and EXC-002; DEL-10-04 REQ-006 and REM-001. Contract-
  deterministic SECURITY rows correctly avoid blanket marker use.
- **STALE surfaces:** fact-driven Guidance/Procedure split follows PKG-07/08
  calibration. Self-scoped completed-setup procedures can remain ALIGNED;
  future-only procedures falsified by the frozen slice are STALE.
- **Rev-0.7 pointers:** where a pointer occurs on an already-STALE census
  surface it is folded into that row; dated MEMORY/non-census occurrences are
  notes only, consistent with PKG-08 carry-forward.
- **ACCEPTANCE grain:** DEL-10-02's eight and DEL-10-03's six rows correspond
  to distinct tables/bullets; DEL-10-05's one bundled TP-RUNNER-015 row is
  disclosed. ClaimType totals remain grain-sensitive at R3.
- **Contract-vs-runtime grain:** ALIGNED PKG-10 boundary rows do not imply a
  runtime public server, plugin loader, external FEA adapter, hosted CI, public
  publication, or completed downstream runner payloads.

## 9. Containment and fences

Frozen HEAD was re-verified as
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Before and after verification,
ignored-aware porcelain contained exactly the six run-allow-listed paths and
no seventh path:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

No frozen tracked, untracked, or additional ignored path was created or
modified by this verifier. No lifecycle, DAG, register, decision, product,
R4, or R5 surface was changed. No F-PIP-1..5 claim is made; the ledgers'
security/privacy/professional-boundary statements are treated only as
attributed deliverable claims subject to the stated evidence and authority
routing. This report is the verifier's only write.

## 10. Package summary line

**PKG-10 fan-in: 5/5 ledgers SOUND, 0 DEFECTIVE; 125 rows; 60 PASS / 2
QUALIFIED / 2 FAIL across the mandatory adversarial checks. DEL-10-05 requires
two localized owning-pilot correction clusters: nine gate-default cells plus
three selectability cells and the notes recount; and REQ-007 SECURITY marker
+ OWNER routing plus its notes sentence. All substantive dispositions remain
supportable. Independent current checks passed (adapter 14, local-FEA 1,
readiness/sweep 24, export non-cargo 16, headless copied-tree Cargo 16+1+5),
and addendum-9 ignored-aware containment remained exactly the six allow-listed
paths.**
