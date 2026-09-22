# DEL-10-05 notes: Headless CLI and structured I/O analysis runner

Worker: W3 PKG-10 G2 (TASK). Brief `R2-WORKER_brief.md` (SHA-256 `2d793d0a…0db141`, verified).
Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`. The forward ledger is sealed in
`DEL-10-05_SEAL.txt`. DEL-10-05 was not an R0 pilot.

## Counts

Forward: 99 rows (66 required keys, 27 `.rNN` rows, 6 `.sNN` rows).
- 45 ALIGNED, 18 STALE_SETUP_SPECIFICATION, 17 STALE_REVIEW_OR_EVIDENCE.
- 9 COVERED_BY_CHILDREN, 10 NOT_ASSESSED.
- No UNKNOWN rows.

Reverse: 387 capabilities. 12 CLAIMED_BY, 1 PARTIAL, 1 UNKEYED, 373 NOT_MINE.

## Path aliases

- `D5/` = `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/`.
- Runner code is cited as `core/runner/headless/...`, which resolves to `projects/chirality-piping/core/...`.
- The parity records are cited as repository-root `execution/_Coordination/AgentRuns/...`.

## Judgment calls

- **Stub drift (FG-DEL-10-05-02).** The TP-RUNNER-015-era text (2026-07-05, post-migration) still says that `export-results`, `run-benchmark` and `run-regression` return blocking stubs, and that the input wrapper has only `request`, `solve.preview_model` and `rule_check_aggregate`. At the freeze:
  - `benchmark_binding.rs` executes the mechanics, stress and nonlinear suites (2026-07-19, extended 2026-08-12).
  - `export-results` produces a report-package projection (2026-07-23).
  - The wrapper accepts `benchmark`, `regression` and `export_results`.

  The affected rows are CLM-002.r06, CLM-004.r02, CLM-005.s01, CLM-012 and AC-001. They are STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE · LOCAL_DESIGN. `_STATUS` history records the bindings; the SOW was not updated.
- **Setup-era TBDs overtaken by rulings (FG-DEL-10-05-06).** The rows are STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING. The rulings: DEC-065 (final CLI syntax and names), DEC-025/059/093 (CI posture), DEC-057 (matrix) and DEC-017/028 (container). CLM-014 is post-migration (f033dd24d, 2026-07-04), so it takes STALE_REVIEW_OR_EVIDENCE.
- **Setup framing overtaken by implementation (FG-DEL-10-05-04).** CLM-009.s01, CLM-011.r01/r03, CLM-017, CLM-018, CLM-027.r02 and CLM-028 are STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE.
- **R-01 to R-09 are all ALIGNED.** Each rests on code at the freeze and on runner tests in the gate log: 33 library, 1 preview-binary and 15 final-binary tests, all passing, plus `tests/test_headless_runner_contract.py` inside the 1,138-pass Python surface.
  - R-01 (MEDIUM): the runner validates a RunnerRequest before any work and calls the same product_physics preview service as the desktop. No separate application-service crate exists; I read the shared service plus the request/job/result envelope as the governed boundary.
  - R-04 (MEDIUM): see the verifier note below.
- **CLM-004.r04 (format list TBD) is ALIGNED (MEDIUM).** DEC-012 keeps the format list implementation-level and no ruling fixes it.
- **Pointer rows (CP-02, FG-DEL-10-05-07).**
  - CLM-002.r05 cites `skills/semantic-matrix-build/SKILL.md`, which is not at the freeze.
  - CLM-003.r06 cites `INIT.md`, which is not at the freeze.
  - In both rows the substance holds and only the pointer diverges.
- **CLM-006 (references) is ALIGNED (MEDIUM).** All cited documents and section numbers exist. The SPEC section list (1, 7–10) does not map one-to-one onto the named topics: deliverable mechanics sit in SPEC §11. I judged that loose citation, not a stale pointer.
- **The empty `STATUS#remaining` block is pre-typed NON_NORMATIVE** and is NOT_ASSESSED. The open work it could have held (persisted-project input, CI, public transport, release, adapters) is recorded as future in CLM-012 and CLM-009.s02, where it is outside this deliverable. A4: Remaining is not authority.

**For the verifier: possible VERIFICATION_REMOVED, not decided.**
- Commit `b43cc00c4` (2026-09-13, "Repair canonical result integrity…") rewrote `core/runner/headless/src/result_envelope_binding.rs`. That module's tests went from 8 to 2.
- Removed: `nonlinear_bearing_solve_attaches_validated_envelope_with_context`, `linear_only_solve_attaches_envelope_without_nonlinear_context`, `structural_failure_appends_blocking_runner_diagnostic`, `producer_rejects_missing_envelope_checksum_structurally`, `model_incomplete_solve_attaches_nothing_and_adds_no_diagnostics` and three others.
- The rewrite belongs to the governed result-integrity run whose MEMORY entry cites `FINAL_ACCEPTANCE_V1.md`. Library tests in `lib.rs` still cover envelope-checksum validation.
- No SOW claim names the removed tests. R-04 stays ALIGNED at MEDIUM confidence. The smallest check: compare the removed test names with the acceptance record's verification list, and confirm the behaviour is covered elsewhere.

## Canonical departures

None. CS rows use their assigned fields.

The following CP situations were applied:
- CP-03 with CP-02 fields: CLM-008, CLM-016, CLM-023.
- CP-02: CLM-002.r05, CLM-003.r06.
- CP-01: CLM-013, CLM-019, CLM-020, CLM-021.
- CP-04: the SOW SURFACE row, default variant.
- CP-09: VER-001 and the output-matrix OUT-001. PASS records bind `f22605f1…`; none matches the frozen `e5b331a9…`.

## Convention friction

- **CP-04 identifiers.** The binary `openpipestress-runner` was itself ruled by DEC-065. DEC-101 (iv) later names binary, crate and schema `$id` renames for an identity tranche that had not run at the freeze. I recorded this once on the SURFACE row, default CP-04 fields, AuthorityNeeded OWNER. Items that state the verbs (CLM-004.r01, R-06) are judged on substance and are ALIGNED.
- **The residue list.** It covers the binary, the crate `open_pipe_stress_headless_runner`, the CLI artifact constant `openpipestress.headless_runner_cli_output` and the schema `$id` host `openpipestress.org`.

## UNKNOWN rows

None.

## Reverse pass

Answers:
- **CLAIMED_BY:**
  - the CLI, the local-private output gate, request/result validation and the in-memory preview bridge;
  - the benchmark/regression binding and the export-results binding;
  - the runner schema, the compatibility preview binary and the result-export adapter in the runner crate;
  - the three witness sets (TP-RUNNER-015, payload binding, export-results).
- **PARTIAL:** the validation-manual reproduction page. It documents DEL-10-05's evidence inside DEL-09-04's manual.
- **UNKEYED:** the desktop HeadlessRunnerPanel. It embeds `deliverable_id: "DEL-10-05"`, but no key covers a GUI preview; the nearest key is R-01.
- **NOT_MINE with specific reasons (F5):**
  - 26 product_physics capabilities. DEL-10-05 cites product_physics `lib.rs` only as the shared preview service.
  - the report-package wire adapter (PKG-08) and result-export derivatives (DEL-08-04);
  - the evidence sweep (DEL-10-04);
  - the governing documents and the product_preview fixtures;
  - the three benchmark suites (DEL-09-01).

**Did the reverse pass change my view of anything sealed?**
- The routing note on RC-10-0172 independently observes that the TP-RUNNER-015 benchmark-stub witness predates the suite-bound outputs. That matches CLM-012 being non-aligned.
- The HeadlessRunnerPanel is a DEL-10-05-identified GUI surface that the SOW does not mention. It could be an IMPLEMENTED_UNDOCUMENTED candidate for R3.
- No sealed row would change.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-10-04 and DEL-10-05: PASS, 0 consistency findings.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9). Since 2026-09-19 Piping selects work through owner-steered work graphs.

## Verification basis

No suite was run. Test-pass statements rest on `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` and its log (runner crate 33+1+15 tests `ok`; Python 1,138 passed; product bytes identical to the freeze) and on frozen run records and witnesses, which were not rerun. Runner benchmark and regression outputs are regression evidence, not validation (A5).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Nothing here states or implies release, approval, compliance or certification. Agent dispositions are not owner rulings.
