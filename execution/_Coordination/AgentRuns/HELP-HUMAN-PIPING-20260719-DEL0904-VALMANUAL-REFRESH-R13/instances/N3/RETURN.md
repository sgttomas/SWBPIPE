# N3 TERMINAL RETURN — Execution of CB-2026-07-19-DEL-09-04-VALMANUAL-REFRESH-001

**Run:** `HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13`, node N3
**Role:** WORKING_ITEMS manager acting as its own single serialized,
non-delegating executor (governed Agent 1 execution child)
**Date:** 2026-07-19
**Base commit:** `45ec0524d3b0c155392553a3b3e4190534ff0fe8`
(branch `claude/piping-r13-valmanual-refresh`; pre-execution tree clean apart
from this run's lawful pre-existing untracked state: the R13 AgentRuns
directory and the candidate brief)
**Adopted brief content SHA-256 (post-attribution):**
`94f2be7fb801bab06517143243e3f20d01883882a81bbd0ab2e63f39b3127905`

Paths relative to `projects/chirality-piping/` unless noted.

## 1. Basis freeze (§4.1)

Recorded before any durable write: HEAD `45ec0524d`, branch confirmed,
`git status --porcelain` showed only the two lawful untracked entries. §4.1
stop-conditions checked against the live tree: DAG pointer (`DAG-007`,
approved), DEL-09-04 lifecycle/Remaining text, runner bin/binding sources,
the del1005 witness family, and `software-workflow.json` all matched the
brief's recorded state. No material change; execution proceeded.

## 2. Implementation summary (per-file)

### 2.1 `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md` (first act)

- Frontmatter line 4: `status: proposed_effect_held` →
  `status: adopted_effective_execution_released`; line 10:
  `rule_activation: not_activated` →
  `rule_activation: activate_owner_standing_approval` (R12 adopted-brief
  convention).
- Header Status line (line 15): → `EFFECTIVE — EXECUTION RELEASED BY
  HELP_HUMAN FAN-IN DISPOSITION (N2 COMMIT-SAFE)`.
- §10 block: `AgentClassification: CLASSIFY_ELIGIBLE (N1,
  `instances/N1/RETURN.md`)`; `RuleActivation:
  ACTIVATE_OWNER_STANDING_APPROVAL`; `IndependentVerifier: COMMIT-SAFE —
  `instances/N2/RETURN.md` (20/20 claims confirmed; independent 10-class
  re-screen pass)`; `EffectStatus: EFFECTIVE — EXECUTION RELEASED BY
  HELP_HUMAN FAN-IN DISPOSITION (N2 COMMIT-SAFE)`. `AdoptionAuthority`
  (HUMAN_OWNER_BY_STANDING_APPROVAL, SHA-bound at `f14fa7751…`),
  `OwnerCaseSelection: NONE`, and the PreservedGates line unchanged.

### 2.2 `docs/validation_manual/headless_runner_reproduction.md` (215 lines, restructured in place)

- Frontmatter (lines 1–13) unchanged
  (`OPS-VALIDATION-MANUAL-HEADLESS-RUNNER-REPRODUCTION`, `draft_evidence`).
- Scope (17–29): original boundary paragraph preserved verbatim; one added
  paragraph naming the two-part structure and PR #287 (`60841413a` /
  `45ec0524d`).
- Authority And Boundaries (31–41): table preserved; one added row stating
  the DEC-065 exit-code policy (0 = no blocking diagnostic; 1 = blocking
  diagnostics; 2 = usage/malformed input).
- Part 1 — Frozen E1 Procedure (43–91): Fixture Set rows 1–2 verbatim;
  row 3 corrected in place (exit stays 1; expected evidence now names
  `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` on post-#287 sources with the
  pre-#287 stub diagnostic labeled historical). Dated note (53–66):
  2026-07-19, PR #287 provenance, payload-less frozen input, stub confined
  to `export-results`, committed witness and pinned bundles (incl.
  `REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`) remain truthful, seven
  frozen surfaces byte-identical. Reproduction Procedure (74–91): the four
  frozen commands byte-identical to the prior page; closing paragraph gains
  one sentence pointing case 3's post-#287 diagnostic at the dated note.
- Part 2 — Bound path (93–167): intro names PR #287 /
  `CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001`, the three suite crates,
  `export-results` as the only remaining stub, and the generator
  `generate_del1005_payload_binding_inputs.py`. Bound Fixture Set table
  (105–115): the five del1005 cases exactly per brief §3.2 (mechanics 1/1
  `MECH-TP-PHYS-004-LOAD-TO-RESULTANT`; stress 3/3 with the three named
  cases; nonlinear 5/5 with `whole_suite_default_applied: true`; two
  payload-missing cases exit 1 with their codes). Paragraph 117–124 states
  `"diagnostics": []` (empty array) and exits 0/0/0/1/1 citing the R12 N3
  return §3. Witness list (126–132): the five committed generated files.
  Procedure (130–144): generator + five `cargo run … --input … --output`
  commands; expected exits restated. Fail-closed section (146–167):
  `suite_run` report fields, blocked-case codes
  `HEADLESS_RUNNER_{BENCHMARK,REGRESSION}_CASE_COMPARISON_BASIS_NOT_REUSABLE`,
  whole-suite-with-blocked-cases exits 1, R12-head figures (mechanics 11/21
  + 10 blocked; stress 12/15 + 3 blocked; nonlinear 5/5), stated strictly
  as regression evidence; thresholds/tolerance/CI-gate/professional
  reliance `TBD` owner-gated.
- Rerun Consequence (169–177): per `CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001`
  §8 — fresh run ID, new immutable bundle under
  `validation/evidence/reproduction/<run-id>/`, completed bundles never
  edited/reinterpreted/invalidated.
- Review Checks (179–188): preserved verbatim (commands valid post-#287).
- Recorded Reproduction Deltas (190–200): 2026-07-10 entry preserved
  verbatim; no new entry (no reproduction run performed).
- Remaining E2 Work (202–215): refreshed per §3.6 — open: threshold
  disposition (owner-gated), MAINTAINER_REVIEWED promotion, GUI-workflow
  evidence, `export-results` binding, R5-exit reviews; landed: bindings
  (PR #287, DEL-10-05) and clean-checkout demonstration (R11 bundle,
  `INTERNALLY_VERIFIED`, reproduction acceptance remains an owner gate).

### 2.3 DEL-09-04 `_STATUS.md`

- Line 4: `Last Updated` 2026-07-16 → 2026-07-19.
- Line 7 (first Remaining bullet), sole clause replaced: "runner
  benchmark/regression payload bindings still structured stubs (per-case
  reproduction runs through suite tests; see also DEL-10-05)" → "runner
  benchmark/regression payload bindings landed via DEL-10-05 / PR #287
  (Receipt-59; `export-results` remains the only structured runner stub,
  bounded DEL-10-05 work)". Every other byte of the bullet (lead-in,
  MAINTAINER_REVIEWED + GUI-workflow clause, DEC-080 storage clause, source
  tag) unchanged.
- Line 8 (second, owner-gated DEC-046 tolerance bullet): byte-identical
  (verified against HEAD).
- Line 3 `Current State: IN_PROGRESS`: unchanged.
- Exactly one new History entry (line 11, newest-first).

### 2.4 DEL-09-04 `MEMORY.md`

Exactly one new newest-first entry (lines 14–44): page refresh facts,
witness anchoring, spot-run corroboration, `_STATUS.md` bullet edit,
boundaries, claim fence.

### 2.5 New files

- `…/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_VALMANUAL_REFRESH_R13.md`
  (the one authorized new run record).
- `instances/N3/CHECK_harness-pytest.json`,
  `instances/N3/CHECK_harness-self-check.json`,
  `instances/N3/CHECK_change-scope.json` (persisted check evidence), and
  this `RETURN.md` — all inside §5 item 4.

### 2.6 Witness-anchoring work performed before page closeout (§6 mandatory)

Parsed all five del1005 generated witnesses and all five inputs, the frozen
stub input/witness pair, read the bin dispatch arms and
`benchmark_binding.rs` line 312, and cross-checked exits against the R12 N3
return §3 row 6. Optional offline spot-run exercised (permitted by §6): the
prebuilt binary `core/runner/headless/target/debug/openpipestress-runner`
run directly (no cargo, no network, no install) on the five documented
bound-path inputs and the frozen case 3 input; outputs to the session
scratchpad only. Observed: exits 0/0/0/1/1 with `[]`/`[]`/`[]`/
`[HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING]`/
`[HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING]`, and frozen case 3 exit 1
with `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` — all matching the page.
The del1005 generator was NOT executed (it writes
`validation/witness/inputs/**`, outside the fence).

## 3. Checks run (each its own halting step; commands from `REPO_ROOT`)

| # | Command | Exit | Result |
|---|---|---:|---|
| 1 | `python3 tools/software_workflow/run_registered_checks.py "projects/chirality-piping/software-workflow.json" --check harness-pytest --output …/instances/N3/CHECK_harness-pytest.json` | 0 | PASS |
| 2 | same, `--check harness-self-check --output …/instances/N3/CHECK_harness-self-check.json` | 0 | PASS |
| 3 | `python3 tools/validation/validate_claims_language.py --repo-root .` | 0 | PASS (262 files; DEC-081 taxonomy satisfied) |
| 4 | `python3 tools/validation/validate_path_anchors.py . --text` | 0 | PASS (631 surfaces) |
| 5 | `git diff --check` | 0 | PASS (no whitespace errors) |
| 6 | `git status --porcelain` over the seven frozen E1 surfaces | 0 | PASS (0 lines — byte-identical) |
| 7 | `diff` of the second Remaining bullet vs `git show HEAD:…_STATUS.md` | 0 | PASS (byte-identical) |
| 8 | python3 parse of all 16 witness JSONs (5 del1005 generated + 5 inputs + 3 frozen generated + 3 frozen inputs) | 0 | PASS |
| 9 | `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT" --base 45ec0524d… --allowed <the seven §5 surfaces>` → `instances/N3/CHECK_change-scope.json` | 0 | PASS (`status: PASS`, 12 changed paths, `violations: []`) |
| 10 | `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .` (read-only; no append performed — see §5 deltas) | 0 | PASS (receipts structurally valid, unmodified) |

`piping-pytest` and `evidence-sweep` were not selected: the changed-path set
is `docs/**` + `execution/**` only (§5 profile determination, N2-confirmed);
no sweep artifact exists or was created.

## 4. Acceptance-predicate verdicts (brief §3, one by one)

1. **Witness-anchored assertions — HOLDS.** Every command shape, exit code,
   diagnostic code, and per-case count on the page traces to the parsed
   committed witnesses (§2.6), the R12 N3 preserved evidence (§3 row 6;
   N4 v2 COMMIT-SAFE), the live bin/binding source, or the DEC-065 exit
   policy; the optional spot-run corroborated all six documented behaviors.
   No asserted value originates in this tranche.
2. **Bound-path documentation — HOLDS.** All five del1005 cases documented
   with the §3.2 facts (suites, deliverables, case IDs, counts, whole-suite
   default, exits 0/0/0/1/1, both payload-missing codes), plus the generator
   and the five committed generated witnesses (page lines 105–144).
3. **Fail-closed semantics truthful — HOLDS.** Blocked-case codes, exit-1
   whole-suite behavior, and the R12-head figures (11/21+10; 12/15+3; 5/5)
   stated strictly as regression evidence; thresholds, tolerance policy, CI
   gate policy, professional reliance stated `TBD` owner-gated (page
   146–167).
4. **Frozen E1 procedure preserved — HOLDS.** Three cases retained; dated
   2026-07-19 note with PR #287 provenance (`60841413a` / `45ec0524d`),
   payload-missing explanation, and pinned-truthfulness statement for the
   committed witness and prior bundles; check 6 proves the seven frozen
   surfaces byte-identical.
5. **Rerun-trigger consequence stated — HOLDS.** Page Rerun Consequence
   section cites `CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001` §8: fresh run
   ID, new immutable bundle, completed bundles never edited, reinterpreted,
   or invalidated.
6. **Remaining E2 truthfulness — HOLDS.** Open and landed items exactly per
   §3.6 (page 202–215), including `export-results` as the only remaining
   structured stub and the R11 bundle cited `INTERNALLY_VERIFIED` with
   reproduction acceptance remaining an owner gate.
7. **Claims calibration — HOLDS.** Regression-evidence posture throughout;
   no new tolerance/threshold/criterion/normative content anywhere in the
   tranche; the Scope paragraph retaining acceptance and professional
   judgment with the responsible engineer is preserved verbatim; check 3
   (claims-language) PASS.
8. **Bounded state update — HOLDS.** `_STATUS.md`: only the bindings clause
   of the first Remaining bullet changed (landed, not deleted from
   history); second bullet byte-identical (check 7); `IN_PROGRESS`
   unchanged; exactly one History entry; `Last Updated` refreshed.
   `MEMORY.md`: exactly one newest-first entry. Exactly one new run record
   with the exact required name.
9. **Non-acts — HOLDS.** No reproduction run; no bundle/witness/sweep
   write; no lifecycle/stage/promotion/acceptance act; no DEL-10-05, code,
   schema, test, tool, or `validation/**` write (check 9: zero violations);
   §6 plan passed (checks 1–10).

Predicate tally: **9 of 9 HOLD.**

## 5. Deltas from the brief (parent-directed; recorded per dispatch)

1. **No `loop/LOOP_RECEIPTS.md` append; no commit/stage/push/PR.** Brief
   §4.4/§5-item-5 authorize one receipt append and closeout mechanics; the
   HELP_HUMAN N3 dispatch explicitly withholds both from this node (durable
   binding follows N4). The receipts file is untouched (porcelain empty);
   the §6 receipt validator ran read-only and passed (check 10).
2. **N2 precision notes applied, not propagated.** The page asserts
   `"diagnostics": []` (empty array; never literal `null`), matching the
   committed bytes (N2 Note A); no DAG execution-upstream row-count prose
   was carried onto any written surface (N2 Note B — the brief's
   enumeration of eight satisfied rows governs; N1's "six" not restated).
3. **Brief frontmatter/header status alignment** (§2.1) performed alongside
   the dispatch-enumerated §10 fields, following the R12 adopted-brief
   convention, within §5 item 1's status-record allowance.

No other delta. No scope reinterpretation.

## 6. Enumerated refutable claims for N4

Run from `REPO_ROOT=$(git rev-parse --show-toplevel)`;
`WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping`. Page = 
`docs/validation_manual/headless_runner_reproduction.md`.

1. **Fence containment, exact path count.** The complete changed-path set
   vs base `45ec0524d` is exactly 12 paths, all inside the §5 fence: the
   page; DEL-09-04 `_STATUS.md`, `MEMORY.md`, and the new run record; the
   candidate brief; and 7 files under the R13 AgentRuns directory
   (CURRENT_CANDIDATE_RATIONALE.md, ORCHESTRATION_PLAN.md, N1/RETURN.md,
   N2/RETURN.md, and N3's three CHECK JSONs) — plus this RETURN.md written
   after the check (13th path, same fenced directory). Check:
   `instances/N3/CHECK_change-scope.json` (`status: PASS`,
   `violations: []`) and rerun the same command.
2. **Frozen-surface byte-identity.** `git status --porcelain` over the
   seven frozen E1 surfaces (three `tp_runner_015_final_cli_*_input.json`,
   `generate_tp_runner_015_inputs.py`, three committed
   `tp_runner_015_final_cli_*.json`) returns 0 lines. The del1005 witness
   family (generator, five inputs, five generated) is likewise absent from
   `git status --porcelain`. Check: rerun both.
3. **Exact `_STATUS.md` bullet edit.** `git diff HEAD -- "<DEL-09-04
   dir>/_STATUS.md"` shows changes on exactly three surfaces: the
   `Last Updated` line, the first Remaining bullet (only the bindings
   clause replaced, as quoted in §2.3), and one inserted History entry; the
   second Remaining bullet line is byte-identical to HEAD. Check: run the
   diff; `diff` the `^- Promote final` line vs `git show HEAD:…`.
4. **Page: bound single-case row anchor.**
   `validation/witness/generated/del1005_payload_binding_benchmark_single_case.json`
   records `command: run-benchmark`, `suite_run.suite: mechanics`,
   `suite_deliverable: DEL-09-01`, `requested_case_count: 1`, case
   `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` `executed_and_matched`, top-level
   `diagnostics: []` — matching the page's row 1 and exit 0 under DEC-065.
   Check: parse the JSON.
5. **Page: bound multi-case row anchor.** `…benchmark_multi_case.json`
   records suite `stress` / `DEL-09-02`, exactly the three case IDs printed
   on the page, 3/3 `executed_and_matched`, `diagnostics: []`. Check:
   parse.
6. **Page: regression full-suite row anchor.**
   `…regression_full_suite.json` records `command: run-regression`, suite
   `nonlinear` / `DEL-09-03`, `whole_suite_default_applied: true`, 5/5
   `executed_and_matched`, `diagnostics: []`; its input contains
   `regression = {"suite": "nonlinear"}` with no `cases` key. Check: parse
   both JSONs.
7. **Page: payload-missing rows anchor.** `…benchmark_payload_missing.json`
   / `…regression_payload_missing.json` each carry exactly one blocking
   diagnostic — `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` /
   `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`; their inputs are
   `request`-only. Check: parse all four.
8. **Page: exit-code line anchor.** The page's 0/0/0/1/1 line cites the R12
   N3 return; that return's §3 row 6 records exactly `0, 0, 0, 1, 1` for
   the five commands in the same order. Check: read
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N3/RETURN.md`
   line 134.
9. **Page: historical-note anchor (source side).** In the live bin
   `core/runner/headless/src/bin/openpipestress-runner.rs`, the stub
   diagnostic string occurs only at line 270 (`ExportResults` arm) and test
   lines 657/671; `execute_suite_verb` maps `RunBenchmark` →
   `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` and returns exit 1 when the
   payload is absent. Check: grep + read.
10. **Page: historical-note anchor (input side).** The frozen
    `tp_runner_015_final_cli_benchmark_stub_input.json` has top-level keys
    `['request']` only (no `benchmark` payload); its committed witness's
    diagnostics list is exactly the stub code. Check: parse both.
11. **Page: fail-closed figures anchor.** The blocked-case code family is
    formatted at `core/runner/headless/src/benchmark_binding.rs` line 312;
    the whole-suite figures on the page (mechanics 11/21 + 10 blocked;
    stress 12/15 + 3 blocked; nonlinear 5/5) match the R12 N3 return §2
    (lines 118–119). Check: read both.
12. **Page: rerun-consequence anchor.**
    `CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001` §8 requires a new run and new
    immutable bundle when the E1 procedure or its documenting surfaces
    change; the page's Rerun Consequence section states exactly that
    consequence and names the pinned bundle
    `REPRO_DEL0904_20260719T202023Z_23eeaabc9040/` as never edited. Check:
    read brief §8 and page lines 169–177.
13. **Page: Remaining E2 paragraph anchor.** DEL-10-05 `_STATUS.md`
    Remaining holds only the `export-results` bullet (bindings landed,
    Receipt-59); the R11 bundle exists and its run record states
    `INTERNALLY_VERIFIED`. The page's open/landed split matches. Check:
    read DEL-10-05 `_STATUS.md`, the R11 run record, and page lines
    202–215.
14. **No stale stub expectation survives.** On the refreshed page, the stub
    diagnostic `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`
    appears only as historical/pre-#287 or `export-results`-scoped prose
    (Fixture Set row 3 historical clause, the dated note); no current-source
    expectation of it remains for `run-benchmark`/`run-regression`, and the
    string "remain structured stubs" is gone. Check: grep the page.
15. **No `null` propagation.** The page nowhere asserts
    `diagnostics: null`; it asserts `"diagnostics": []` for the three
    success witnesses, matching the committed bytes. Check: grep the page
    for `null`.
16. **Frozen commands preserved.** The four command lines in the frozen E1
    Reproduction Procedure block are byte-identical to the pre-edit page
    (same fixtures, same `--input`/`--output` shape). Check:
    `git show HEAD:…headless_runner_reproduction.md` lines 56–59 vs live
    lines 78–81.
17. **Check results.** The ten §3-table check commands returned exit 0 with
    the outputs recorded there; the three persisted JSONs exist under
    `instances/N3/` and parse. Check: rerun any; parse the JSONs.
18. **Receipts untouched.** `git status --porcelain --
    projects/chirality-piping/loop/LOOP_RECEIPTS.md` is empty and the
    receipts validator passes on the unmodified file. Check: rerun both.

Claims: 18.

## 7. Boundaries and claim fence

No reproduction run, bundle, witness, sweep, or `validation/**` write; no
`core/**`, `schemas/**`, `tests/**`, tool, DEL-10-05, or `index.md` write;
no lifecycle/stage/promotion/acceptance/threshold/tolerance act; no push,
PR, network, install, or toolchain change. Spot-run outputs were ephemeral
(session scratchpad).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
return records implementation and verification observations only; it makes
no release-readiness, acceptance, compliance, professional, certification,
sealing, or authentication claim.

## 8. Overall result

**PASS.** Predicates: 9/9 HOLD. Files modified: 4 (candidate brief §10 +
status record; the reproduction page; DEL-09-04 `_STATUS.md`; DEL-09-04
`MEMORY.md`). Files created: 5 (the DEL-09-04 run record; three CHECK JSONs
under `instances/N3/`; this RETURN.md). Checks: 10/10 PASS (harness-pytest,
harness-self-check, claims-language, path-anchors, `git diff --check`,
frozen-surface byte-identity, second-bullet byte-identity, 16-file JSON
parse, change-scope containment with 12 fenced changed paths and zero
violations, read-only receipts validation). Deltas: 3 recorded (no receipt
append / no git closeout per dispatch; N2 precision notes applied without
propagation; brief status-field alignment). Enumerated refutable claims for
N4: 18.
