# N4 TERMINAL RETURN — Independent Implementation Verification (R13)

**Run:** `HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13`, node N4
**Role:** fresh-context independent implementation verifier (governed Agent 2);
refutation posture — default refuted absent positive confirmation
**Date:** 2026-07-19
**Verified against:** live tree at base `45ec0524d3b0c155392553a3b3e4190534ff0fe8`
(HEAD unchanged during verification), the sealed brief
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md`
(§3 predicates, §5 fence, §6 checks), and the N3 executor return
(`instances/N3/RETURN.md`, 18 enumerated claims).

Paths relative to `projects/chirality-piping/` unless noted. All witness JSON
parses, source reads, validator runs, and the offline spot-run below were
performed by this node independently; no N3 result was accepted on assertion.

## 1. Per-claim verdicts (18 of 18)

| # | Claim | Verdict | Independent evidence |
|---|---|---|---|
| 1 | Fence containment, exact path count | **CONFIRMED** | Independent rerun of `tools/software_workflow/validate_change_scope.py` against base `45ec0524d` with the seven §5 allowed surfaces: `status: PASS`, exactly **13** changed paths, `violations: []` — the 12 paths N3's persisted `CHECK_change-scope.json` recorded (parsed: `status: PASS`, 12 paths, zero violations) plus `instances/N3/RETURN.md` written after that check, inside the fenced AgentRuns directory. `git status --porcelain -uall` and `git diff HEAD --name-only` independently re-derived the same set: 3 modified + 10 untracked, no path outside the §5 fence. |
| 2 | Frozen-surface byte-identity | **CONFIRMED** | `git status --porcelain` over the seven frozen E1 surfaces (3 `tp_runner_015_final_cli_*_input.json`, `generate_tp_runner_015_inputs.py`, 3 committed `tp_runner_015_final_cli_*.json`): **0 lines**. Same over the eleven del1005 family files (generator + 5 inputs + 5 generated): **0 lines**. |
| 3 | Exact `_STATUS.md` bullet edit | **CONFIRMED** | `git diff HEAD` on DEL-09-04 `_STATUS.md` shows exactly three changed surfaces: `Last Updated` 2026-07-16→2026-07-19; the first Remaining bullet with only the bindings clause replaced (old and new clause text byte-match N3 §2.3's quotes; lead-in, MAINTAINER_REVIEWED + GUI-workflow clause, DEC-080 clause, and source tag unchanged); one inserted History entry (newest-first). The second (`Promote final public-benchmark…DEC-046`) bullet does not appear in the diff — byte-identical to HEAD. `Current State: IN_PROGRESS` unchanged. |
| 4 | Bound single-case row anchor | **CONFIRMED** | Parsed `validation/witness/generated/del1005_payload_binding_benchmark_single_case.json`: `command: run-benchmark`, `suite_run.suite: mechanics`, `suite_deliverable: DEL-09-01`, `requested_case_count: 1`, one case `fixture_id: MECH-TP-PHYS-004-LOAD-TO-RESULTANT` with `status: executed_and_matched`, top-level `diagnostics: []`. Matches page row 1; exit 0 consistent with DEC-065 (no blocking diagnostic). |
| 5 | Bound multi-case row anchor | **CONFIRMED** | Parsed `…benchmark_multi_case.json`: suite `stress` / `DEL-09-02`, `requested_case_count: 3`, cases `STRESS-AXIAL-NORMAL-ORIGINAL`, `STRESS-RANGE-MECHANICS-ORIGINAL`, `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` all `executed_and_matched` (3/3), `diagnostics: []` — exactly the three IDs printed on the page. |
| 6 | Regression full-suite row anchor | **CONFIRMED** | Parsed `…regression_full_suite.json`: `command: run-regression`, suite `nonlinear` / `DEL-09-03`, `whole_suite_default_applied: true`, `requested_case_count: 5`, 5/5 `executed_and_matched`, `diagnostics: []`. Parsed its input: top-level keys `['regression','request']`, `regression = {"suite": "nonlinear"}`, no `cases` key. |
| 7 | Payload-missing rows anchor | **CONFIRMED** | Parsed all four: `…benchmark_payload_missing.json` carries exactly one blocking diagnostic `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`; `…regression_payload_missing.json` exactly one blocking `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`; both inputs have top-level keys `['request']` only. |
| 8 | Exit-code line anchor | **CONFIRMED** | R12 N3 return (`…DEL1005-RUNNER-PAYLOADS-R12/instances/N3/RETURN.md`) **line 134**, §3 row 6: `0, 0, 0, 1, 1 (single-case, multi-case, regression-full-suite, benchmark-payload-missing, regression-payload-missing)` — same values, same order as the page's exit line, which cites that return. |
| 9 | Historical-note anchor (source side) | **CONFIRMED** | `grep -n` on `core/runner/headless/src/bin/openpipestress-runner.rs`: `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` occurs only at line 270 (read in context: inside the `RunnerOperation::ExportResults` arm, exit 1) and test assertions at lines 657 and 671. `execute_suite_verb` (read lines 291–335) maps `RunBenchmark → ("benchmark", "HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING")` and returns `(1, …)` when the payload is `None`. |
| 10 | Historical-note anchor (input side) | **CONFIRMED** | Parsed frozen `tp_runner_015_final_cli_benchmark_stub_input.json`: top-level keys `['request']` only (no `benchmark`). Parsed its committed witness: exactly one diagnostic, code `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`. |
| 11 | Fail-closed figures anchor | **CONFIRMED** | `core/runner/headless/src/benchmark_binding.rs` **line 312** formats `HEADLESS_RUNNER_{verb_token}_CASE_COMPARISON_BASIS_NOT_REUSABLE` in the `CaseStatus::Blocked` arm. R12 N3 return §2 lines 118–120: mechanics 11/21 `executed_and_matched` + 10 blocked; stress 12/15 + 3 blocked; nonlinear 5/5 — identical to the page's figures, which cite that section. |
| 12 | Rerun-consequence anchor | **CONFIRMED** | `CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md` §8 (lines 455–470) requires a new run and new immutable bundle when the E1 procedure or its documenting surfaces change and states completed bundles are never mutated. Page Rerun Consequence section (lines 169–177) states exactly that consequence, cites the brief and `DEC-080`, and names `REPRO_DEL0904_20260719T202023Z_23eeaabc9040/` as not edited, reinterpreted, or invalidated. That bundle directory exists under `validation/evidence/reproduction/`. |
| 13 | Remaining E2 paragraph anchor | **CONFIRMED** | DEL-10-05 `_STATUS.md` Remaining holds exactly one bullet — the `export-results` binding; its newest History entry records the R12 bindings landing (Receipt-59). R11 run record `WORKING_ITEMS_RUN_2026-07-19_DEL0904_CLEAN_REPRO_R11.md` line 32 states the bundle is `INTERNALLY_VERIFIED`. Page lines 202–215 match this open/landed split, keep threshold disposition owner-gated, MAINTAINER_REVIEWED and GUI-workflow open, and state reproduction acceptance remains an owner gate. |
| 14 | No stale stub expectation | **CONFIRMED** | `grep -n` on the live page: the stub diagnostic string appears only at line 51 (Fixture Set row 3 historical clause) and line 57 (dated note, export-results-scoped). The string "remain structured stubs" is absent; the only "remains a structured stub" phrasing (line 98) is export-results-scoped. No current-source stub expectation for `run-benchmark`/`run-regression` remains. |
| 15 | No `null` propagation | **CONFIRMED** | `grep -n "null"` on the page: zero matches. The page asserts `"diagnostics": []` (line 115), matching the committed bytes of all three success witnesses (parsed: empty array, not null). |
| 16 | Frozen commands preserved | **CONFIRMED** | Byte-level `diff` of `git show HEAD:…headless_runner_reproduction.md` lines 56–59 against live lines 79–82: identical (all four command lines). Also byte-identical: page frontmatter (lines 1–13) and Fixture Set rows 1–2 (HEAD 41–42 vs live 49–50). |
| 17 | Check results | **CONFIRMED** | Parsed all three persisted N3 JSONs: `CHECK_harness-pytest.json` PASS, `CHECK_harness-self-check.json` PASS, `CHECK_change-scope.json` PASS / 12 paths / zero violations. Independently re-ran: harness-pytest (exit 0, PASS), harness-self-check (exit 0, PASS), claims-language (exit 0; 262 files, DEC-081 satisfied), path-anchors (exit 0; 633 surfaces — vs N3's 631, the growth being files created after N3's check; PASS either way), `git diff --check` (exit 0), frozen-surface porcelain (0 lines), second-bullet identity (absent from diff), the full 16-file witness JSON parse, change-scope (PASS, 13 paths), receipts validator (exit 0). All match N3's §3 table. |
| 18 | Receipts untouched | **CONFIRMED** | `git status --porcelain -- …/loop/LOOP_RECEIPTS.md`: 0 lines. `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`: exit 0, `VALID … versioned receipt contract satisfied` on the unmodified file. |

**Tally: 18 CONFIRMED, 0 REFUTED, 0 UNVERIFIABLE.**

## 2. Independent fence re-derivation

From `REPO_ROOT`: `git status --porcelain -uall` and `git diff HEAD --name-only`
yield 3 modified paths (the page; DEL-09-04 `_STATUS.md`; DEL-09-04
`MEMORY.md`) and 10 untracked paths (the DEL-09-04 run record; the candidate
brief; 8 files under the R13 AgentRuns directory including N3's RETURN and
three CHECK JSONs). Every path maps to §5 items 1–4. Independent
`validate_change_scope.py` rerun (base `45ec0524d`, the seven §5 allowed
surfaces): `status: PASS`, 13 paths, `violations: []`. §5 item 5
(`loop/LOOP_RECEIPTS.md`) is untouched, consistent with N3's recorded
parent-directed delta (append withheld pending this verification). No
`core/**`, `schemas/**`, `tests/**`, `validation/**`, tool, DEL-10-05, or
`index.md` write exists (`git diff HEAD --stat` over those trees: empty).
**Independent fence result: PASS.**

## 3. Independent spot-run (optional §6 path, exercised)

Prebuilt binary `core/runner/headless/target/debug/openpipestress-runner`
invoked directly (no cargo build/fetch, `CARGO_NET_OFFLINE=true`, no network,
outputs to the session scratchpad only) on the six documented inputs:

| Input | Exit | Diagnostics |
|---|---:|---|
| `del1005_payload_binding_benchmark_single_case_input.json` | 0 | `[]` |
| `del1005_payload_binding_benchmark_multi_case_input.json` | 0 | `[]` |
| `del1005_payload_binding_regression_full_suite_input.json` | 0 | `[]` |
| `del1005_payload_binding_benchmark_payload_missing_input.json` | 1 | `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` |
| `del1005_payload_binding_regression_payload_missing_input.json` | 1 | `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING` |
| `tp_runner_015_final_cli_benchmark_stub_input.json` (frozen case 3) | 1 | `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` |

All six match the refreshed page exactly, including the historical-note
prediction for the frozen case 3 on post-#287 sources. The del1005 generator
was not executed (writes inside `validation/**`).

## 4. Claim-calibrated language check

Read in full on every changed narrative surface (the page, `_STATUS.md`,
`MEMORY.md`, the run record, the candidate-brief §10/status record, N3
RETURN): the regression-evidence posture is preserved throughout; blocked
cases are explicitly "not a defect record"; thresholds, tolerance policy, CI
gate policy, and professional reliance stated `TBD`, owner-gated;
MAINTAINER_REVIEWED is stated as open, never claimed; the R11 bundle is cited
`INTERNALLY_VERIFIED` with reproduction acceptance remaining an owner gate;
the Scope paragraph retaining acceptance and professional judgment with the
responsible engineer is byte-preserved from HEAD. No release-readiness,
acceptance, certification, sealing, authentication, compliance, or
professional claim appears. `validate_claims_language.py`: PASS (262 files).

## 5. Command tally (this node)

17 verification command batches, all read-only against project state except
one ephemeral temp file (see note): (1) `git status --porcelain -uall` +
`git diff HEAD --name-only` + `git log`; (2) `git diff HEAD` on `_STATUS.md`
and `MEMORY.md`; (3) python parse of the five del1005 generated+input pairs;
(4) python parse of all six frozen witness/input files + bin greps + binding
sed; (5) witness case-entry key/ID inspection + bin lines 240–310; (6) bin
lines 310–335 + binding line-312 grep + R12 N3 §2/§3 reads; (7) R12 N3
line-134 and figures greps; (8) DEL-10-05 `_STATUS.md` read + protected-tree
`git diff --stat`; (9) R11 run-record grep + bundle `ls` + clean-repro brief
§8 read; (10) HEAD-vs-live page section extraction + stale-string/`null`
greps; (11) frozen-surface (7), del1005-family (11), and receipts porcelain
checks; (12) `validate_claims_language.py`, `validate_path_anchors.py`,
`validate_piping_loop_receipts.py`, `git diff --check` (all exit 0);
(13) parse of the three persisted N3 CHECK JSONs; (14) independent
`validate_change_scope.py` rerun (PASS, 13 paths); (15) offline spot-run of
the six documented commands; (16) byte-diffs of frontmatter, frozen command
block, and fixture rows 1–2; (17) independent harness-self-check and
harness-pytest reruns (both exit 0, PASS).

Note: `run_registered_checks.py` requires `--output` inside the workspace
root, so check 17 wrote one temp JSON at the repo root per check and deleted
it immediately after parsing (porcelain re-verified clean of it). No project
file was modified by this node except this RETURN.md. No stage, commit, or
push was performed.

## 6. Observations (non-blocking)

- Path-anchors surface count: 633 here vs 631 in N3's run — expected growth
  from files created after N3's check (N3's own RETURN and check artifacts);
  the validator passes identically.
- The changed-path count is now 13 (N3's recorded 12 + N3 RETURN.md); this
  RETURN.md will be the 14th, in the same fenced directory — both inside §5
  item 4.

## 7. Claim fence

This return records independent verification observations only. Standard
claim fence applies (F-PIP-2; claims taxonomy per DEC-081): no
release-readiness, acceptance, compliance, professional, certification,
sealing, or authentication claim is made or implied.

VERDICT: COMMIT-SAFE
