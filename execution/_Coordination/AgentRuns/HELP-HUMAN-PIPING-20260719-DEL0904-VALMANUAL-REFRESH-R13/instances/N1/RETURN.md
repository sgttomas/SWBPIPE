# N1 TERMINAL RETURN — Brief Authoring for HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13

**Run:** `HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13`, node N1
**Role:** ORCHESTRATOR (Agent 1) brief author, governed non-delegating child
of HELP_HUMAN; coordination artifacts only
**Date:** 2026-07-19
**Basis HEAD at authoring:** `45ec0524d3b0c155392553a3b3e4190534ff0fe8`
(branch `claude/piping-r13-valmanual-refresh`, post-merge of PR #287)

**RESULT: `COMPLETE` — candidate brief and D-54 rationale authored; effect
`HELD`; classification `ELIGIBLE_PENDING_VERIFICATION` (`CLASSIFY_ELIGIBLE`,
pending independent refutation at N2).**

Paths relative to `projects/chirality-piping/` unless noted.

## 1. Files written by N1 (complete list)

1. `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md`
   (doc_id `CB-2026-07-19-DEL-09-04-VALMANUAL-REFRESH-001`; §10
   `EffectStatus: HELD`).
2. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13/CURRENT_CANDIDATE_RATIONALE.md`
   (D-54 rationale: sources, 10-class screen PASS, four-lens, 7 rejected
   alternatives, adversarial self-test, attribution fields,
   `OwnerCaseSelection: NONE`, `EffectStatus: HELD`).
3. This `RETURN.md`.

No other file was written. No code, docs, or deliverable-state edit; no
cargo invocation; no commit, stage, push, PR, or receipt append; no
reproduction run.

## 2. Profile determination (evidence-sweep trigger)

The tranche's write fence contains only `docs/**` (the manual page),
`execution/**` (DEL-09-04 state files, run record, R13 AgentRuns dir), and
`loop/**` (receipt append) paths. Under the live `software-workflow.json`
path rules, those paths select `harness-pytest` (rule paths `execution/**`,
`docs/**`, `AGENTS.md`, `loop/**`) plus always-check `harness-self-check`.
`piping-pytest` and `evidence-sweep` are selected only by `core/**`,
`validation/**`, or `tests/**` writes, which the fence forbids. **Therefore
evidence-sweep does NOT trigger, no new
`validation/evidence/sweeps/SWEEP_*.json` is authorized, and the fence
contains no `validation/**` path** — a self-consistent fixed point (running
the sweep would itself create the `validation/**` write that would have
triggered it).

## 3. Enumerated refutable claims

Each claim is one factual assertion plus its exact live source/check. Run
commands from `REPO_ROOT=$(git rev-parse --show-toplevel)` or
`WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping` as appropriate.

1. **Bound verbs.** In
   `core/runner/headless/src/bin/openpipestress-runner.rs`, the
   `RunnerOperation::RunBenchmark` and `RunnerOperation::RunRegression` match
   arms (lines 254–267) route through `execute_suite_verb` with the
   `input.benchmark` / `input.regression` payloads; they do not emit the stub
   diagnostic. Check: read the file at those lines.
2. **Stub confined to export-results.** The diagnostic string
   `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` appears in
   the bin only in the `RunnerOperation::ExportResults` arm (line 270) and in
   the narrowed test assertions (lines 657, 671). Check:
   `grep -n HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD core/runner/headless/src/bin/openpipestress-runner.rs`.
3. **Payload-missing codes.** `execute_suite_verb` maps `RunBenchmark` to
   blocking code `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` and
   `RunRegression` to `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING` (bin
   lines 299–302). Check: read those lines.
4. **Blocked-case code family.** `core/runner/headless/src/benchmark_binding.rs`
   line 312 formats the fail-closed code
   `HEADLESS_RUNNER_{verb_token}_CASE_COMPARISON_BASIS_NOT_REUSABLE`, and the
   suite report carries `claim_posture` and `whole_suite_default_applied`
   fields (lines 108–110). Check: grep/read.
5. **Witness: benchmark single case.**
   `validation/witness/generated/del1005_payload_binding_benchmark_single_case.json`
   records `command: run-benchmark`, `suite_run.suite: mechanics`,
   `suite_deliverable: DEL-09-01`, requested case count 1 with the single
   case `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` `executed_and_matched`, and
   `diagnostics: null` (no blocking diagnostic → exit 0 under DEC-065).
   Check: parse the JSON.
6. **Witness: benchmark multi case.**
   `...benchmark_multi_case.json` records suite `stress`
   (`DEL-09-02`), the three cases `STRESS-AXIAL-NORMAL-ORIGINAL`,
   `STRESS-RANGE-MECHANICS-ORIGINAL`,
   `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS`, 3/3
   `executed_and_matched`, `diagnostics: null`. Check: parse the JSON.
7. **Witness: regression full suite.**
   `...regression_full_suite.json` records `command: run-regression`, suite
   `nonlinear` (`DEL-09-03`), `whole_suite_default_applied: true` (the input
   omits `cases`), 5/5 `executed_and_matched` over
   `NL-ACTIVE-ONE-WAY-ORIGINAL`, `NL-GAP-CLOSURE-ORIGINAL`,
   `NL-LIFT-OFF-ORIGINAL`, `NL-FRICTION-STICK-SLIDE-ORIGINAL`,
   `NL-NONCONVERGENCE-LIMIT-ORIGINAL`, `diagnostics: null`. Check: parse the
   JSON and the corresponding `_input.json` (payload `{"suite": "nonlinear"}`
   with no `cases` key).
8. **Witness: payload-missing pair.**
   `...benchmark_payload_missing.json` carries exactly the blocking code
   `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` and
   `...regression_payload_missing.json` exactly
   `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`; their inputs contain only a
   `request` object. Check: parse all four JSONs.
9. **Exit codes 0/0/0/1/1.** The R12 implementation return records the five
   witness commands exiting 0, 0, 0, 1, 1 (single-case, multi-case,
   regression-full-suite, benchmark-payload-missing,
   regression-payload-missing). Source:
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N3/RETURN.md`
   §3 row 6 (N4 v2 `COMMIT-SAFE` over that return). Consistent with the bin
   exit policy: exit 0 only with no blocking diagnostics. Check: read the
   return; optionally rebuild offline and rerun.
10. **Fail-closed whole-suite facts.** The same R12 return §2 records, on the
    implementation head: mechanics whole-suite 11/21 `executed_and_matched`
    + 10 blocked; stress whole-suite 12/15 + 3 blocked; nonlinear 5/5
    matched — regression evidence only, no threshold invented. Source: R12
    N3 RETURN §2 (and its claim 15 for the stress detail). Check: read;
    optionally run the CLI offline.
11. **Frozen stub input is payload-less.**
    `validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json`
    contains only a top-level `request` key (operation `run_benchmark`, no
    `benchmark` payload), so on post-#287 sources the documented E1 case-3
    command exits 1 with `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` rather
    than the stub diagnostic recorded in its committed witness
    `validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`
    (whose diagnostics list is exactly the stub code). Check: parse both
    JSONs + claims 1–3.
12. **Current page stale rows.** In the live
    `docs/validation_manual/headless_runner_reproduction.md`: line 43 (Fixture
    Set row expecting the stub diagnostic from `run-benchmark`), line 59 (the
    `run-benchmark` command in the Reproduction Procedure), and lines 95–99
    (Remaining E2 Work stating "`run-benchmark` / `run-regression` remain
    structured stubs" at lines 96–97 and still listing "a clean environment
    demonstration record" as remaining, which R11/Receipt-58 landed). Check:
    read the file at those lines.
13. **DEL-09-04 Remaining text.** `_STATUS.md` (under the DEL-09-04 folder)
    has `Current State: IN_PROGRESS` and exactly two `## Remaining` bullets;
    the first begins "Close E2 manual residuals (first assembly landed
    TP-E2-VALMANUAL-001, PR #154): runner benchmark/regression payload
    bindings still structured stubs (per-case reproduction runs through suite
    tests; see also DEL-10-05); MAINTAINER_REVIEWED case-page promotion and
    GUI-workflow validation evidence open;" — the bindings clause is the only
    part the brief authorizes striking; the second bullet (DEC-046
    owner-gated tolerance promotion) must stay byte-identical. Check: read
    `_STATUS.md` lines 3, 6–8.
14. **Bindings actually landed.** DEL-10-05 closed its benchmark/regression
    Remaining bullet at R12 (`export-results` bullet retained), receipted as
    `Receipt-59` with `Examined-Through: 96563e8e09b89908e13e6b2f1f1139aca3283855`
    and PR #287 merged at HEAD `45ec0524d3b0c155392553a3b3e4190534ff0fe8`
    (implementation commit `60841413aca1753229df62997a4637d2179114db`).
    Check: `loop/LOOP_RECEIPTS.md` tail; `git log --oneline -3`; DEL-10-05
    `_STATUS.md`.
15. **DAG posture.** `execution/_DAG/_LATEST.md` resolves to `DAG-007`
    (`approved_active_graph_authority`); the DEL-09-04 rows in
    `execution/_DAG/DAG-007/DependencyEdges.csv` (lines 766–777) are three
    ANCHOR rows (`NOT_APPLICABLE`), six EXECUTION UPSTREAM rows all
    `SATISFIED` (`DAG-002-E0286/E0287/E0288/E0289` constraints,
    `DAG-002-E0543/E0544/E0545` suite prerequisites, `DAG-002-E0546`
    professional-boundary constraint), and one DOWNSTREAM ENABLES row
    `DEL-09-04-E001` `SATISFIED`. No `TBD` execution-upstream row exists for
    DEL-09-04. Check: read those rows.
16. **Profile check-trigger determination.** `software-workflow.json`
    `path_rules` select `evidence-sweep` and `piping-pytest` only for
    `core/**`, `validation/**`, `tests/**`; `harness-pytest` for
    `execution/**`, `docs/**`, `AGENTS.md`, `loop/**`; `always_checks` is
    `["harness-self-check"]`. Hence this docs/execution/loop-only tranche
    triggers `harness-pytest` + `harness-self-check` and no evidence-sweep
    (§2 above). Check: read `software-workflow.json` lines 13–18.
17. **Fence completeness.** The brief's §5 fence enumerates every durable
    write the tranche needs — the brief's own status record, the manual page,
    DEL-09-04 `_STATUS.md`/`MEMORY.md`/one named new run record, the R13
    AgentRuns directory, and one append-only receipt — and nothing else; in
    particular no `validation/**`, `core/**`, `schemas/**`, `tests/**`,
    DEL-10-05, or index.md path. Check: read brief §5 against the §4 bounded
    tasks and §6 plan (no step writes outside the fence; the sweep is
    excluded by claim 16).
18. **No new criteria.** Every exit code, diagnostic code, case count, and
    blocked-case description the brief lets the page assert is traceable to
    committed witness evidence (claims 5–11) or the R12 preserved evidence
    (claims 9–10) under the owner-ruled DEC-065 exit policy; the brief's §3.1
    predicate makes witness-anchoring itself an acceptance condition, §3.7
    forbids new tolerance/threshold/criterion content, and §9 preserves the
    DEC-046 and MAINTAINER_REVIEWED gates (the promotion is not performed).
    Check: read brief §3, §9 against the cited witnesses.
19. **Frozen-surface preservation is checkable.** The brief §3.4 and §6
    require the seven frozen E1 surfaces (three
    `tp_runner_015_final_cli_*_input.json`,
    `generate_tp_runner_015_inputs.py`, three
    `tp_runner_015_final_cli_*.json` witnesses) to remain byte-identical,
    verified by empty `git status --porcelain` over those paths. Check: read
    brief §3.4/§6; the surfaces exist in the live tree.
20. **Rerun consequence grounded.** `CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001`
    §8 (lines 455–468) requires a new run and new immutable bundle when the
    E1 procedure or its documenting surfaces change; the R12
    `HANDOFF_STATE.md` records the same fresh-run-ID consequence. The brief
    (§3.5, §8) makes the page state it, and the completed bundle
    `validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`
    (pinned `23eeaabc9040...`, `INTERNALLY_VERIFIED` per the R11 run record)
    is read-only and remains truthful for its pinned commit. Check: read the
    three cited artifacts.

Claims: 20.

## 4. Classification and hold

- **Classification: `ELIGIBLE_PENDING_VERIFICATION`** (`CLASSIFY_ELIGIBLE`
  under the D-52 §4.1 10-class screen as refined by D-54 §3.1; item-by-item
  table in `../../CURRENT_CANDIDATE_RATIONALE.md` §2; no limit hit, no
  near-miss).
- `EffectStatus: HELD` in the brief §10 and the rationale §7;
  `RuleActivation: NOT_ACTIVATED`; `OwnerCaseSelection: NONE`. No execution
  write is authorized until N2 returns `COMMIT-SAFE` and HELP_HUMAN
  progresses the chain.
- Preserved gates (unperformed and unauthorized here): reproduction
  acceptance and evidence-posture promotion; MAINTAINER_REVIEWED case-page
  promotion; GUI-workflow validation evidence; DEC-046 threshold/tolerance
  promotion; export-results binding; lifecycle/stage/release/acceptance;
  merge; publication/external action.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
return records brief-authoring facts and live-tree observations only; it
makes no run-result, acceptance, release, professional, certification,
sealing, authentication, or code-compliance claim.
