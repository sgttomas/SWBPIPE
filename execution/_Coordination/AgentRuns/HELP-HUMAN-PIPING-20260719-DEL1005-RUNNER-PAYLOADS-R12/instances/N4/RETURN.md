# N4 TERMINAL RETURN — Independent Implementation Verification (Refutation Pass)

**Run:** `HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12`, node N4
**Role:** Fresh-context independent implementation verifier (governed Agent 2)
**Date:** 2026-07-19
**Verified against:** live tree at base `96563e8e09b89908e13e6b2f1f1139aca3283855`,
branch `claude/chirality-piping-loop-init-a45657`
**Inputs:** N3 `RETURN.md` (30 enumerated claims); sealed brief
`CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001` (§3/§5/§6/§7);
`ENVIRONMENT_REPAIR_DISPOSITION.md` (R12-ENVREPAIR-01); live tree,
`git status`, `git diff`.

Method: every claim was checked against the live tree; every re-runnable
command was re-run independently and offline (no npm, no network, no
provisioning); the changed-path set and fence containment were re-derived
independently of the N3 containment JSON. Default posture: refuted unless
positively confirmed. No project file was modified except this single
return; nothing was staged, committed, or pushed.

Paths relative to `projects/chirality-piping/` unless noted.

## 1. Independent fence check (re-derived, not trusted from N3)

`git status --porcelain=v1 --untracked-files=all` from `REPO_ROOT` yields
exactly **37 changed paths** (7 modified + 30 untracked). I checked every
path against the brief §5 fence myself:

- fence 1: the candidate brief (1 path) — in fence;
- fence 2: `core/runner/headless/Cargo.toml`, `src/lib.rs`,
  `src/bin/openpipestress-runner.rs`, new `src/benchmark_binding.rs`
  (4 paths) — in fence (`Cargo.lock` is crate-gitignored, untracked-ignored,
  not in the changed set);
- fence 4: `validation/benchmarks/mechanics/src/lib.rs`,
  `validation/benchmarks/stress/src/lib.rs` (2 paths) — in fence, and
  verified additive-only by diff (§3 below);
- fence 5: 11 NEW files under `validation/witness/inputs/` and
  `validation/witness/generated/` — in fence; no pre-existing witness file
  touched;
- fence 6: DEL-10-05 `_STATUS.md`, `MEMORY.md`, one new
  `_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL1005_RUNNER_PAYLOADS_R12.md`
  (3 paths) — in fence;
- fence 8: exactly one new
  `validation/evidence/sweeps/SWEEP_20260719T220236Z_96563e8e09b8-dirty.json`
  — in fence;
- fence 9: 15 paths under
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/**`
  — in fence.

No path outside the fence. Explicitly verified untouched: frozen E1 surfaces
(seven paths, `git status` empty and `git diff HEAD` empty), DEL-09-04
deliverable folder, `loop/LOOP_RECEIPTS.md`, `docs/**`,
`validation/benchmarks/nonlinear/**`, `schemas/headless_runner.schema.yaml`,
`tests/test_headless_runner_contract.py`. **Fence check: PASS.** The N3
containment JSON's path list (37 entries) matches my independent set exactly;
my own re-run of `validate_change_scope.py` with the §5 `--allowed` list
returned `status: PASS`, `violations: []`, 37 paths.

Note: writing this N4 return adds one further path inside fence item 9.

## 2. Independently re-run commands (exit codes)

All run by N4 from `REPO_ROOT` or `WORKING_ROOT` as appropriate,
`CARGO_NET_OFFLINE=true` for all cargo invocations:

| # | Command | Exit / result |
|---|---|---|
| 1 | `git status --porcelain=v1 -uall` | 0; 37 paths (7 M + 30 ??) |
| 2 | `git diff --check` | 0 |
| 3 | frozen E1 seven paths: `git status --porcelain` + `git diff HEAD --stat` | empty both — byte-identical to HEAD |
| 4 | DEL-09-04 folder, `loop/LOOP_RECEIPTS.md`, nonlinear crate, `docs/`, schema + contract test: `git status --porcelain` | all empty |
| 5 | `grep -nE '[0-9]e-[0-9]'` and `grep -nE '[0-9]\.[0-9]+'` on `benchmark_binding.rs` | no numeric literal of either form in code |
| 6 | `git diff 96563e8e --` Cargo.toml / mechanics lib / stress lib | exactly 3 added path deps + comment; mechanics +12 lines additive accessor only; stress additive accessor + `#[cfg(test)]` un-gating, `1.0e-9` value unchanged |
| 7 | `cargo fmt --manifest-path core/runner/headless/Cargo.toml --check` | 0 |
| 8 | `cargo test --offline` headless crate | 0; 15 bin tests pass incl. the ten new binding tests |
| 9 | `cargo test --offline` mechanics crate | 0; 33 passed |
| 10 | `cargo test --offline` stress crate | 0; 23 passed |
| 11 | `cargo build --offline --bin openpipestress-runner` | 0 |
| 12 | `python3 tests/test_headless_runner_contract.py` | 0 |
| 13 | `python3 validation/witness/inputs/generate_del1005_payload_binding_inputs.py` + SHA-256 before/after | 0; all five inputs byte-identical |
| 14 | runner CLI on the five new witness inputs (offline, output to scratch) | exits 0, 0, 0, 1, 1; every output byte-identical (`cmp`) to its committed generated witness |
| 15 | runner CLI stress whole-suite (`benchmark: {"suite":"stress"}`, scratch input) | exit 1; 15 cases = 12 executed_and_matched + 0 mismatched + 3 blocked; blocked fixture_ids exactly as claimed |
| 16 | JSON parse of all 18 changed `.json` files | 18/18 parse |
| 17 | sweep artifact parse | `overall_status: pass`; base commit string present under `git` key |
| 18 | `ls validation/evidence/sweeps/ \| wc -l`; `diff sweeps_before sweeps_after`; `comm -13 sweeps_before <(ls \| sort)` | 286; before==after (285); delta exactly `SWEEP_20260719T220236Z_96563e8e09b8-dirty.json` |
| 19 | `python3 tools/validation/validate_claims_language.py --repo-root .` | 0; VALID, 262 files |
| 20 | `python3 tools/validation/validate_path_anchors.py . --text` | 0; PASS, 622 surfaces |
| 21 | `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .` | 0; VALID (read-only; no receipt appended at this node) |
| 22 | `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT" --base 96563e8e… --allowed <each §5 fence path>` | 0; `status: PASS`, `violations: []`, 37 paths |

Tally: 22 independent command groups run; every one matched the executor's
recorded result. Zero re-run failures.

## 3. Per-claim verdicts (N3 §8, claims 1–30)

| # | Verdict | Evidence |
|---|---|---|
| 1 | **REFUTED (count only; operative content confirmed)** | The changed-path set IS exactly the paths listed in `CHANGE_SCOPE_CONTAINMENT.json`, with `status: PASS` and empty `violations` — independently re-derived, 37 = 37. But the claim asserts "the **31** paths in" that JSON; the JSON contains **37** paths and the live changed set is 37. The "31" is a stale pre-closeout figure (it excludes the closeout/repair paths) republished as current in the final return. The claim's own check procedure passes; its stated cardinality is false. |
| 2 | CONFIRMED | Brief §10 contains every quoted line exactly (RuleActivation, OwnerCaseSelection: NONE, IndependentVerifier: COMMIT-SAFE naming `instances/N2/RETURN.md`, AdoptionAuthority ending `f14fa77518a06f112ae72a8fcce4de0fab958d47)`, EffectStatus EFFECTIVE line); `PreservedGates:` line present with the expected gate list. Sub-note: byte-comparison to the pre-adoption text is not independently checkable (the brief is a single untracked file; no durable pre-adoption copy found); content is consistent with the N2-reviewed gate list. Non-blocking. |
| 3 | CONFIRMED | Diff shows exactly the three `open_pipe_stress_{mechanics,stress,nonlinear}_benchmarks` path deps plus provenance comment; no other dependency line changed. |
| 4 | CONFIRMED | No scientific-notation or decimal numeric literal in `benchmark_binding.rs` code; no `.abs()`/raw float inequality; comparisons route through injected `comparison_holds` (the crates' `recorded_comparison_holds`), mechanics public `validate_*` predicates, and exact `==` on the nonlinear residual (line 1224) mirroring `matches_expected_outcome`. |
| 5 | CONFIRMED | Mechanics diff: 12 insertions, one additive `pub fn recorded_comparison_holds` + doc comment; nothing else. `INTERNAL_ASSERTION_EPSILON` pre-existed un-gated at base (line 52). |
| 6 | CONFIRMED | Stress diff: same additive accessor + removal of `#[cfg(test)]` (comment added); `const INTERNAL_ASSERTION_EPSILON: f64 = 1.0e-9;` value unchanged. |
| 7 | CONFIRMED | `git status --porcelain` on nonlinear crate: empty. |
| 8 | CONFIRMED | At HEAD the stub diagnostic is emitted only in the `RunnerOperation::ExportResults` arm (line 270); other grep hits are the narrowed test and a negative assertion in the benchmark payload-missing test; the message string is byte-identical to the base emit block. |
| 9 | CONFIRMED | Headless tests exit 0; 15 bin tests listed, all ten new binding tests present by name. |
| 10 | CONFIRMED | mechanics 33 passed, stress 23 passed, exit 0 each, offline. |
| 11 | CONFIRMED | `cargo fmt --check` exit 0. |
| 12 | CONFIRMED | Generator re-run reproduces all five inputs byte-identically (SHA-256 before/after identical). |
| 13 | CONFIRMED | Exit codes 0, 0, 0, 1, 1; additionally each scratch output is byte-identical to its committed generated witness. |
| 14 | CONFIRMED | `requested_case_count` 5, executed_and_matched 5, `whole_suite_default_applied` true, `diagnostics` [], claim_posture contains "regression evidence". |
| 15 | CONFIRMED | Independent CLI run: exit 1; 12 matched + 3 blocked; blocked ids exactly the three claimed. |
| 16 | CONFIRMED | All seven frozen paths byte-identical to base (empty status and diff). |
| 17 | CONFIRMED | Schema + contract test byte-unchanged; contract test exit 0. |
| 18 | CONFIRMED | Three PASS check JSONs as claimed; `CHECK_evidence-sweep.json` records FAIL/exit 1 with stderr naming playwright/tsc/vite/vitest and offline Cargo cache. |
| 19 | CONFIRMED as historical record, superseded by 26 | `sweeps_before.txt` == `sweeps_after.txt` (285 lines each) — truthful record of the blocked attempt; live dir is now 286 per the recorded repair (claim 26 governs the final state). |
| 20 | CONFIRMED as historical record, superseded by 27 | `node_modules` now exists (materialized by R12-ENVREPAIR-01) and is gitignored; the pre-repair absence and non-provisioning are consistent with the FAIL check record and disposition. |
| 21 | CONFIRMED as historical record, superseded by 28/29 | `_STATUS.md`/`MEMORY.md` are now changed exactly per the closeout claims. |
| 22 | CONFIRMED | `loop/LOOP_RECEIPTS.md` byte-unchanged; receipts validator exit 0 (VALID). |
| 23 | CONFIRMED | DEL-09-04 folder status empty. |
| 24 | CONFIRMED | claims-language exit 0 (VALID, 262 files); path-anchors exit 0 (PASS, 622 surfaces) on the final tree. |
| 25 | CONFIRMED | Rerun JSON: id `evidence-sweep`, PASS, exit 0; sweep artifact parses, `overall_status: "pass"`, base commit string present. |
| 26 | CONFIRMED | 286 entries; `comm -13` delta is exactly the one new sweep filename; reverse comm empty. |
| 27 | CONFIRMED | Disposition file exists, names `R12-ENVREPAIR-01`, records offline copy of gitignored state, no network, no durable project write; my independent containment re-run shows no out-of-fence path. |
| 28 | CONFIRMED | Exactly one `## Remaining` bullet (`export-results`); `**Current State:** IN_PROGRESS`; `**Last Updated:** 2026-07-19`; 2026-07-19 History entry cites the brief, run R12, the blocked→repaired sweep sequence, and the sweep artifact by name. |
| 29 | CONFIRMED | `MEMORY.md` begins with the exact claimed 2026-07-19 heading; boundary paragraph keeps thresholds/tolerance/CI-gate/DEC-046 promotion and lifecycle acts owner-gated; run record disposition `SUCCESS` with rows 7 (FAIL) and 7b (rerun PASS) both preserved. |
| 30 | CONFIRMED | My independent containment re-run over the full final set: `status: PASS`, `violations: []`, 37 paths; `git diff --check` exit 0. |

Claim-calibrated-language spot check on every new/changed durable narrative
(brief §10 update, `_STATUS.md` History, `MEMORY.md` entry, run record, N3
return, repair disposition): all express results as regression evidence for
current solver behavior with thresholds/tolerance/CI-gate/promotion and
lifecycle acts owner-gated; no release-readiness, acceptance, certification,
or compliance claim found. The claims-language validator passes over the
final tree.

## 4. Verdict basis

Confirmed: 29 of 30 claims (including three lawfully superseded historical
records verified as truthful for their time). Refuted: 1 (claim 1, cardinality
only). Fence: PASS (independent). Frozen surfaces: no drift. Tolerances: none
invented. Re-run commands: 22/22 match, zero failures.

**Most material finding.** N3 RETURN.md §8 claim 1 asserts the changed-path
set "is exactly the 31 paths" in `CHANGE_SCOPE_CONTAINMENT.json`; the JSON
lists 37 paths and my independent derivation confirms 37. The number is a
stale pre-closeout count republished as current in the final return — the
return's own supersession preamble corrects claims 19–21 but not this one.
The operative content (set identity with the JSON, PASS, empty violations)
is fully confirmed and the tree is otherwise clean in every checked respect,
but the false cardinality sits inside a durable evidence file that this
commit would enshrine, and this verifier's mandate treats any refuted claim
as blocking. Remedy is minimal: correct "31" to "37" in N3 RETURN.md §8
claim 1 (fence item 9 surface), then re-verify; every other surface needs no
change on present evidence.

VERDICT: BLOCK
