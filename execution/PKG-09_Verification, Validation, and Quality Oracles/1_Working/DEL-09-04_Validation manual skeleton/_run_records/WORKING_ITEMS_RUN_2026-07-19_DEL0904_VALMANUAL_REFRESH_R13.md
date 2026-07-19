# WORKING_ITEMS RUN — DEL-09-04 Validation-Manual Reproduction Page Refresh (R13)

**Run:** `HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13`, node N3
**Executor:** WORKING_ITEMS manager acting as its own single serialized,
non-delegating executor (governed Agent 1 execution child)
**Date:** 2026-07-19
**Adopted brief:** `CB-2026-07-19-DEL-09-04-VALMANUAL-REFRESH-001`
(`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md`;
post-adoption content SHA-256
`94f2be7fb801bab06517143243e3f20d01883882a81bbd0ab2e63f39b3127905`)
**Effect release:** HELP_HUMAN fan-in disposition after N2 `COMMIT-SAFE`
(`instances/N2/RETURN.md`, 20/20 claims confirmed; independent 10-class
re-screen pass); adoption authority `HUMAN_OWNER_BY_STANDING_APPROVAL`
(DEC-085 / D-52 §2, refined by DEC-087 / D-54 §1)
**Base commit:** `45ec0524d3b0c155392553a3b3e4190534ff0fe8`
(branch `claude/piping-r13-valmanual-refresh`; tree clean before execution
apart from this run's lawful pre-existing untracked state: the R13 AgentRuns
directory and the candidate brief)

Paths relative to `projects/chirality-piping/` unless noted.

## 1. Scope executed (brief §4.2–§4.3, inside the §5 fence)

1. **Brief status record** — §10 attribution progressed to
   `RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL`,
   `IndependentVerifier: COMMIT-SAFE`, `EffectStatus: EFFECTIVE — EXECUTION
   RELEASED BY HELP_HUMAN FAN-IN DISPOSITION (N2 COMMIT-SAFE)`; frontmatter
   `status`/`rule_activation` and the header Status line aligned to the R12
   adopted-brief convention. PreservedGates line unchanged.
2. **`docs/validation_manual/headless_runner_reproduction.md`** — refreshed
   to a two-part body: frozen E1 `tp_runner_015` procedure with the dated
   2026-07-19 historical note on case 3 (post-#287 expected diagnostic
   `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`, exit 1; stub diagnostic
   confined to `export-results`; committed witness and pinned bundles remain
   truthful), plus a new bound-path section: five
   `del1005_payload_binding_*` fixtures with generator, commands
   (`--input`/`--output` shape), expected exits 0/0/0/1/1, expected
   diagnostics, per-case counts (1/1 mechanics `DEL-09-01`; 3/3 stress
   `DEL-09-02`; 5/5 nonlinear `DEL-09-03` with
   `whole_suite_default_applied: true`), the five committed generated
   witnesses, fail-closed blocked-case semantics with the R12-head
   whole-suite figures (mechanics 11/21 + 10 blocked; stress 12/15 + 3
   blocked; nonlinear 5/5) stated strictly as regression evidence, the
   DEC-065 exit policy row, the rerun consequence per
   `CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001` §8, and the refreshed Remaining
   E2 Work paragraph (open: threshold disposition, MAINTAINER_REVIEWED
   promotion, GUI-workflow evidence, `export-results` binding; landed:
   bindings PR #287, clean-checkout demonstration R11). Review Checks
   section preserved (commands remain valid post-#287).
3. **`_STATUS.md`** — first Remaining bullet: the stale clause "runner
   benchmark/regression payload bindings still structured stubs (per-case
   reproduction runs through suite tests; see also DEL-10-05)" replaced with
   "runner benchmark/regression payload bindings landed via DEL-10-05 /
   PR #287 (Receipt-59; `export-results` remains the only structured runner
   stub, bounded DEL-10-05 work)"; MAINTAINER_REVIEWED and GUI-workflow
   clauses kept open verbatim; second (DEC-046 tolerance) bullet
   byte-identical; `Current State: IN_PROGRESS` unchanged; `Last Updated`
   2026-07-19; exactly one new History entry (newest-first).
4. **`MEMORY.md`** — exactly one new newest-first entry.
5. **This run record** — the one new file authorized under §5 item 3.

## 2. Witness-anchoring evidence (brief §6 mandatory cross-check)

- Parsed all five `validation/witness/generated/del1005_payload_binding_*.json`
  witnesses with python3: the three success witnesses serialize
  `"diagnostics": []` (empty array; brief §6 "null/empty" satisfied by the
  actual bytes — N2 Note A precision applied, `null` not asserted anywhere on
  the page) with `suite_run` counts 1/1 (mechanics,
  `MECH-TP-PHYS-004-LOAD-TO-RESULTANT`), 3/3 (stress, the three named
  cases), and 5/5 (nonlinear, `whole_suite_default_applied: true`, input
  `regression = {"suite": "nonlinear"}` with no `cases` key); the two
  payload-missing witnesses carry exactly
  `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` /
  `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`, inputs `request`-only.
- Exit codes 0/0/0/1/1 cross-checked against the preserved R12 N3 evidence
  (`.../HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N3/RETURN.md`
  §3 row 6; N4 v2 `COMMIT-SAFE`) and the DEC-065 exit policy.
- Live source inspection: bin `RunBenchmark`/`RunRegression` arms route
  through `execute_suite_verb` (payload-missing codes at the
  `execute_suite_verb` head); stub diagnostic string present in the bin only
  in the `ExportResults` arm (line 270) and two test assertions (657, 671);
  `benchmark_binding.rs` line 312 formats
  `HEADLESS_RUNNER_{verb_token}_CASE_COMPARISON_BASIS_NOT_REUSABLE`. Frozen
  stub input parsed: top-level keys `['request']` only.
- Optional offline spot-run (permitted by brief §6): the prebuilt local
  binary `core/runner/headless/target/debug/openpipestress-runner` was
  invoked directly (no cargo, no network, no provisioning) on the five
  documented bound-path inputs and the frozen E1 case 3 input; outputs
  written to the session scratchpad only (ephemeral). Observed exits
  0/0/0/1/1 and 1, with diagnostics `[]`,`[]`,`[]`,
  `[BENCHMARK_PAYLOAD_MISSING]`, `[REGRESSION_PAYLOAD_MISSING]`, and
  `[BENCHMARK_PAYLOAD_MISSING]` respectively — corroborating the committed
  witnesses and the historical note. The generator was NOT executed (it
  writes `validation/witness/inputs/**`, outside the fence).

## 3. Checks run (§6, each its own halting step; JSONs under `instances/N3/`)

| # | Check | Exit | Result |
|---|---|---:|---|
| 1 | `run_registered_checks.py … --check harness-pytest` → `instances/N3/CHECK_harness-pytest.json` | recorded in RETURN | see `instances/N3/RETURN.md` §4 |
| 2 | `run_registered_checks.py … --check harness-self-check` → `instances/N3/CHECK_harness-self-check.json` | recorded in RETURN | see `instances/N3/RETURN.md` §4 |
| 3 | `validate_claims_language.py --repo-root .` | recorded in RETURN | see `instances/N3/RETURN.md` §4 |
| 4 | `validate_path_anchors.py . --text` | recorded in RETURN | see `instances/N3/RETURN.md` §4 |
| 5 | `git diff --check` | recorded in RETURN | see `instances/N3/RETURN.md` §4 |
| 6 | Frozen-surface byte identity (7 paths, `git status --porcelain` empty) | recorded in RETURN | see `instances/N3/RETURN.md` §4 |
| 7 | Second Remaining bullet byte identity vs pre-edit text | recorded in RETURN | see `instances/N3/RETURN.md` §4 |
| 8 | Witness JSON parses (5 del1005 + frozen pair) | 0 | PASS (§2 above) |
| 9 | `validate_change_scope.py --base 45ec0524d…` → `instances/N3/CHECK_change-scope.json` | recorded in RETURN | see `instances/N3/RETURN.md` §4 |

`piping-pytest` and `evidence-sweep` were not selected (docs/execution-only
changed-path set under the live `software-workflow.json` path rules; the
brief §5 profile determination, independently confirmed by N2). No sweep
artifact was created.

## 4. Deltas from the brief (parent-directed)

- **No `loop/LOOP_RECEIPTS.md` append and no commit/stage/push/PR** in this
  node: the HELP_HUMAN dispatch for N3 directs that receipt append and git
  closeout mechanics are withheld from this execution child and handled by
  the parent chain (N4 verification precedes durable binding). The brief's
  §6 receipt-validator step therefore ran against the unmodified receipts
  file as a read-only structural sanity check.
- N2 precision notes applied, not propagated: the page asserts
  `"diagnostics": []` (empty array, never literal `null`), and no DAG
  row-count prose was carried onto any surface (the N1 "six rows" miscount
  is not restated; the enumerated eight satisfied EXECUTION UPSTREAM rows
  govern).

## 5. Boundaries preserved

No reproduction run; no bundle, witness, sweep, or `validation/**` write; no
`core/**`, `schemas/**`, `tests/**`, tool, DEL-10-05, or `index.md` write;
no lifecycle, stage, promotion, acceptance, threshold, or tolerance act
(DEC-046 owner gate untouched); no MAINTAINER_REVIEWED promotion; no
GUI-workflow evidence; no push, PR, network, or external state change.
Frozen E1 surfaces byte-identical; completed bundles untouched.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
record documents bounded documentation and deliverable-state work only; it
makes no release-readiness, acceptance, professional, certification,
sealing, authentication, or code-compliance claim.
