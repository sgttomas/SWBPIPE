# N2 TERMINAL RETURN — Independent Adversarial Verification

**Run:** `HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12`, node N2
**Role:** Fresh-context independent adversarial verifier (governed Agent 2);
no stake in the outcome; default posture = refute
**Date:** 2026-07-19
**Live-tree basis at verification:** HEAD
`96563e8e09b89908e13e6b2f1f1139aca3283855`, branch
`claude/chirality-piping-loop-init-a45657`; working tree clean except the
untracked R12 AgentRuns directory and the untracked candidate brief (both
authored by this run's N1, consistent with the N1 return)
**Method:** every enumerated claim in
`instances/N1/RETURN.md` §5 was re-checked by opening the cited live-tree
source directly (Read/grep/csv-parse/`--help`); no cargo command was run; no
file other than this return was written; no commit was made.

Paths relative to `projects/chirality-piping/` unless noted.

## 1. Per-Claim Verdict Table

| # | Claim (short) | Verdict | Evidence opened |
|---|---|---|---|
| 1 | Remaining bullet text (selected scope) | CONFIRMED | DEL-10-05 `_STATUS.md` lines 7–8: exact bullet "Bind benchmark/regression runner payloads (E2 per-case reproduction currently runs through suite tests) (source: TP-E2-VALMANUAL-001 residuals)" present as a distinct item after the `export-results` bullet |
| 2 | DEL-10-05 `IN_PROGRESS` | CONFIRMED | Same file, line 3: `**Current State:** IN_PROGRESS` |
| 3 | Stub arm location/behavior + test | CONFIRMED | `core/runner/headless/src/bin/openpipestress-runner.rs` lines 237–257: single match arm `RunnerOperation::ExportResults \| RunBenchmark \| RunRegression` pushes blocking `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` and returns `(1, …)`; test `downstream_operation_verbs_are_stable_but_stubbed` (line 543) asserts code 1 + diagnostic for `run-benchmark` |
| 4 | Verb mapping stable | CONFIRMED | `operation_for_verb` (bin lines 390–398) maps the five verbs; `RunnerOperation` enum in `core/runner/headless/src/lib.rs` lines 52–60 (claim said ~53–61; within tolerance of "~") |
| 5 | DAG-007 DEL-10-05 execution-upstream posture | CONFIRMED | `execution/_DAG/DAG-007/DependencyEdges.csv` parsed with csv.DictReader: ACTIVE EXECUTION/UPSTREAM rows for FromDeliverable DEL-10-05 are exactly the 10 claimed rows with exactly the claimed SATISFIED/TBD statuses (E001/E002 SATISFIED; E003–E008 TBD; both DEV-001-STAGE2 rows SATISFIED; file lines 869–891). All other matching rows are `RETIRED`. `_LATEST.md` lines 3–7: DAG-007, `approved_active_graph_authority` |
| 6 | TBD rows accepted as deferred | CONFIRMED | DEL-10-05 `_STATUS.md` history entry 2026-06-07: "…accepted active dependency `TBD` rows as deferred for the current bounded runner-contract boundary" (verbatim) |
| 7 | Mechanics suite shape/posture | CONFIRMED | `validation/benchmarks/mechanics/Cargo.toml` name `open_pipe_stress_mechanics_benchmarks`; `src/lib.rs` `pub fn fixture_inventory()` line 546, `fixture_inventory_ids()` line 572, `MechanicsBenchmark` lines 239–246 with `fixture_id` and `expected_values`; README lines 12–16/78–80: regression evidence; release thresholds / final tolerance policy / CI gate policy / professional reliance `TBD` pending human approval |
| 8 | Stress suite shape/posture | CONFIRMED | `validation/benchmarks/stress/Cargo.toml` name `open_pipe_stress_stress_benchmarks`; `src/lib.rs` `fixture_inventory()` line 437, `StressBenchmark.fixture_id` line 190; README lines 21–24: same regression-evidence / `TBD`-thresholds posture |
| 9 | Nonlinear suite shape/posture | CONFIRMED | `validation/benchmarks/nonlinear/Cargo.toml` name `open_pipe_stress_nonlinear_benchmarks`; `src/lib.rs` `fixture_inventory()` line 1118; `NonlinearRegressionCase` lines 322–334 with `fixture_id`, `expected_states`, `expected_residual_norm`; ten-plus `*.dec046.json` policy records in the crate directory; README line 6: "The fixtures are software verification aids only" |
| 10 | DEC-065 policy constraints | CONFIRMED | `execution/_Decomposition/SOFTWARE_DECOMP.md` line 647, row DEC-065: binary `openpipestress-runner`, the five verbs, schema-first JSON stdin/caller-named file, structured JSON stdout with explicit optional output path, single foreground local process, no daemon/network/telemetry/hidden-mutation/private-write/user-home-scan/direct-SQL, exit policy 0/1/2 as claimed |
| 11 | Check registry facts | CONFIRMED | `software-workflow.json` read in full: six named checks with claimed commands/cwds; `always_checks=["harness-self-check"]`; path rules `core/**,validation/**,tests/**` → `piping-pytest`+`evidence-sweep`; `execution/**,docs/**,AGENTS.md,loop/**` → `harness-pytest`. Live `python3 tools/software_workflow/run_registered_checks.py --help` (root): `profile` positional, `--check`, required `--output`, `--timeout-seconds`. Root `docs/SOFTWARE_WORKFLOW_PROFILE.md` and `tools/software_workflow/validate_change_scope.py` exist |
| 12 | E1 procedure and frozen fixtures | CONFIRMED | `docs/validation_manual/headless_runner_reproduction.md` (project docs) Fixture Set table: three cases; case 3 = `run-benchmark`, expected exit 1, stub diagnostic (line 43). Three `tp_runner_015_final_cli_*_input.json` inputs + generator exist; benchmark stub input read in full — contains only a `request` object with `"operation": "run_benchmark"`, no payload. "Remaining E2 Work" (line 97): `run-benchmark`/`run-regression` "remain structured stubs" |
| 13 | DEL-09-04 E2 residual | CONFIRMED | DEL-09-04 `_STATUS.md` Remaining: first bullet contains "runner benchmark/regression payload bindings still structured stubs (per-case reproduction runs through suite tests; see also DEL-10-05)"; second bullet gates final public-benchmark release tolerances under the DEC-046 convention (owner threshold promotion) |
| 14 | DEL-08-04 envelope refs in runner | CONFIRMED | `core/runner/headless/src/lib.rs` lines 350/357: `schemas/results.schema.yaml` as `schema_ref`; line 588: payload must identify `DEL-08-04` wrapper |
| 15 | Runner crate has no suite deps | CONFIRMED | `core/runner/headless/Cargo.toml` read in full: only `open_pipe_stress_canonical_json`, `open_pipe_stress_product_physics`, `serde`, `serde_json`, `sha2`. Each suite crate has its own `Cargo.lock`; no `[workspace]` and no project-root `Cargo.toml`/`Cargo.lock` exist, so lockfile effects stay crate-local (see finding F2) |
| 16 | Governance lane availability | CONFIRMED | D-54 packet read in full (reasoned selection, prospective supersession of exactly-one-outcome); `SOFTWARE_DECOMP.md` line 669 row DEC-087; `loop/LOOP_RECEIPTS.md` Receipt-56 Gate-Outcome "D-54/DEC-087 landing state reconciled"; Receipt-58 records the DEL-09-04 clean reproduction with DEL-09-04 remaining `IN_PROGRESS`; `loop/WORKPLAN_2026-07-18b_piping_loop.md` exists, is committed at HEAD, and is the lexicographically newest `loop/WORKPLAN_*.md` |
| 17 | CLI wrapper precedent | CONFIRMED | `CliInput` struct (bin lines 31–37): `request` + optional `solve` (`SolveInput.preview_model`) + optional `rule_check_aggregate`; DEL-10-05 `ScopeOfWork.md` CLM-004 conditions table records exactly that settled wrapper; `MEMORY.md` 2026-07-05 entry: downstream verbs "return structured blocking diagnostics until downstream payload bindings exist" |
| 18 | Write-fence completeness | CONFIRMED | Every durable write required by brief §4.1–§4.4 and §6 maps to a §5 item (mapping in §3 below); ephemeral cargo/pytest caches are gitignored at both levels (`**/target/`, `**/__pycache__/`, `**/.pytest_cache/`) and therefore excluded by `validate_change_scope.py`, which read live uses `git diff --name-only` plus `git ls-files --others --exclude-standard` when no `--path` args are given — exactly as the brief describes |
| 19 | No-new-normative-content | CONFIRMED | Each §3 predicate traced to an accepted source: exit mapping = DEC-065 row (line 647); comparison bases = suite crates' recorded `expected_values`/expected-state/residual records + README regression-evidence posture; scope = the two Remaining bullets; diagnostics naming = existing `HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING` (bin line 270). Brief §3.4/§9 forbid new tolerance/threshold/criterion and require fail-closed handling where a recorded basis cannot be reused |
| 20 | Stub retention claim | CONFIRMED | Brief §3.6: stub diagnostic "emitted for `export-results` ONLY after this tranche"; §4.2 narrows the existing test to `export-results`; consistent with the `export-results` Remaining bullet staying open (claim 1 source) |
| 21 | Repo state at authoring | CONFIRMED | `git rev-parse HEAD` = `96563e8e09b89908e13e6b2f1f1139aca3283855`; `git status --porcelain` shows only the R12 AgentRuns directory and the candidate brief (the brief is N1's own recorded write, created after intake), consistent with N1's "clean at intake except the pre-existing R12 directory" |

**Counts: 21 CONFIRMED / 0 REFUTED / 0 UNVERIFIABLE.**

## 2. Independent Fast-Reject Re-Screen (D-52 §4.1, D-54 §3.1 refined)

Run independently against the brief's actual scope, not N1's rationale.

| # | Class | Verdict | Independent basis |
|---|---|---|---|
| 1 | Irreducible preference / multiple defensible outcomes | PASS | The surviving alternatives (payload naming, case addressing, verb-to-suite mapping, module layout) are implementation shape. D-54 §2 prospectively removed multiple-defensible-outcomes as an automatic referral; rejected alternatives are recorded in the rationale §4 with reasons. No irreducible personal preference is required by any §3 predicate |
| 2 | Professional/safety/legal/fiduciary accountability | PASS | Output confined to the suites' existing regression-evidence posture; thresholds/professional reliance are named PreservedGates and remain `TBD` per the three READMEs |
| 3 | Conflict ruling not determined by authority chain | PASS | DEC-065 determines exit/process policy; suites determine recorded values; Remaining bullets determine scope. The one identified tension (E1 case-3 expectation becomes historical post-binding) is flagged and routed to HELP_HUMAN as follow-on (§7), not ruled |
| 4 | New acceptance criteria / normative content | PASS | Re-checked hardest. Pass/fail semantics: exit 0/1/2 reuses DEC-065's already-ruled policy verbatim; per-case match/fail must "reuse the recorded comparison basis already encoded in the suite crates" and is expressed only in the regression-evidence claim posture; §3.4 and §9 expressly forbid new tolerance constants and require fail-closed structured diagnostics where a recorded basis cannot be reused (executor "must not invent a tolerance"). New payload fields/diagnostic codes follow recorded precedents (CLM-004 wrapper; `HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING`) and were expressly anticipated by TP-RUNNER-015 ("until downstream payload bindings exist") — implementation shape, not normative content. No tolerance policy is introduced anywhere in the brief |
| 5 | Lifecycle/stage/release/acceptance act | PASS | None inside the brief: DEL-10-05 stays `IN_PROGRESS`; closure limited to removing the one Remaining bullet (same model as the accepted DEL-09-04 precedent, Receipt-58); §9 excludes lifecycle/stage/issuance/release/acceptance; sweep run declared non-authoritative (§6) |
| 6 | External commitment | PASS | §5 closing text: no push, pull, fetch, PR creation, self-merge, or network/external state change is authorized; DEC-065 forbids network/daemon/telemetry in the product surface; nothing exceeds the recorded branch-first + PR discipline (which the brief leaves to the owner) |
| 7 | Merge/integration authority, destructive action | PASS | Owner merges (§9, §10 PreservedGates); frozen E1 fixtures/generator/witnesses and completed reproduction bundles enumerated read-only; failures fail closed; no history rewriting anywhere |
| 8 | Protected/private data | PASS | Stub input read in full: provenance declares "invented non-engineering example" / `public_metadata`; suite READMEs state no protected standards content; no protected path in the fence |
| 9 | Evidence unavailable/stale, claim beyond warrant | PASS | Every outcome-determining premise re-verified live (§1 table). The six `TBD` DAG rows are covered by the recorded 2026-06-07 human-approved deferral for the bounded runner-contract boundary, and the tranche stays inside that boundary (does not touch the DEL-08-04 envelope surface — claim 14). The E1 case-3 staleness is disclosed, not papered over. The brief claims no run result; effect `HELD` |
| 10 | Domain-engine/prover/secrets/higher-order boundaries | PASS | Fence excludes `_DomainEngines/**`, tools, root governance, app-dev; no prover, account, secret, or ResponsibleParty surface involved |

**Re-screen result: PASS on all 10 classes; no near-miss condition observed.**

## 3. Fence-Completeness and Predicate-Checkability Findings

Fence mapping (brief §4/§6 requirement → §5 item): §4.1 run-record writes →
§5.9; §4.2 bin/lib/new module/Cargo.toml/Cargo.lock → §5.2, schema + contract
test → §5.3, additive suite accessors (+ suite Cargo.lock) → §5.4, runner unit
tests live inside §5.2 files; §4.3 NEW witness inputs/generator/generated
outputs → §5.5; §4.4 DEL-10-05 `_STATUS.md`/`MEMORY.md`/one run record →
§5.6, gated DEL-09-04 update → §5.7, receipt append → §5.10; §6 persisted
check JSON → §5.9, exactly one sweep JSON → §5.8; brief status block → §5.1.
**No §4/§6 task requires a durable write outside §5.** Non-blocking notes:

- **F1 (note).** `core/runner/headless/` currently contains no `Cargo.lock`
  (only `Cargo.toml` and `src/`). §5 item 2 enumerates
  `core/runner/headless/Cargo.lock`, so its creation during the tranche is
  in-fence; recording here so the executor's one-file deltas are not
  misread as drift.
- **F2 (note).** No `[workspace]` manifest and no project-root
  `Cargo.toml`/`Cargo.lock` exist, so cargo activity cannot ripple a lockfile
  outside the enumerated crate paths.
- **F3 (note, execution-risk not eligibility defect).** Mechanics/stress
  `ExpectedValue.tolerance_policy` fields are `None`/unresolved in the live
  crates (e.g., mechanics lines 235/250–253); the nonlinear crate carries
  governed DEC-046 policy records. If some benchmark cases turn out to have
  no reusable encoded comparison predicate, brief §3.4/§7 already dictate the
  lawful result: those cases fail closed with structured diagnostics and the
  conflict returns to HELP_HUMAN. This cannot become a silent new tolerance
  under the brief as written.
- **F4 (note).** Ephemeral caches (`target/`, `__pycache__/`,
  `.pytest_cache/`) are gitignored at root and project level and are excluded
  by `validate_change_scope.py`'s `--exclude-standard` untracked scan, so the
  §6 containment check is sound as specified.

Predicate checkability: §3.1–§3.9 are checkable as written — exit codes and
per-case output content are observable; "unchanged byte-for-byte" (§3.8) is
checkable by hash/`git diff`; "NEW files only" (§3.9, §5.5) is checkable by
before/after listing; §3.4's reuse requirement is checkable by code review
plus the mandated fail-closed diagnostics. The brief stops before every
nondelegable act (adoption progression, DEL-09-04 release, merge, push,
threshold promotion, lifecycle). Claim language is calibrated: no
release-readiness, reproduction-acceptance, or compliance claim appears; the
attribution block matches the D-52 §4.5 form as refined by D-54 §3.3, with
`EffectStatus: HELD`, `RuleActivation: NOT_ACTIVATED`,
`OwnerCaseSelection: NONE`, and `IndependentVerifier: PENDING` (this return).

## 4. Terminal Verdict

VERDICT: COMMIT-SAFE

No claim refuted, no fast-reject class hit, no BLOCK-grade finding. Notes
F1–F4 are informational and impose no condition beyond what the brief already
requires.
