---
doc_id: CB-2026-07-20-DEL-09-04-CLEAN-REPRO-R14-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-20
package_id: PKG-09
deliverable_id: DEL-09-04
decision_basis: DEC-080, DEC-081, DEC-085, DEC-087, DEC-065 (exit policy), DEC-046 (preserved gate)
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — DEL-09-04 Clean-Checkout Reproduction at the R14 Head (W5)

**Status:** `EFFECTIVE — EXECUTION RELEASED BY W5 MANAGER UNDER THE R14 CAMPAIGN CHAIN (VERIFIER COMMIT-SAFE)`

**Verification record (2026-07-20):** the fresh-context adversarial brief
verifier returned `COMMIT-SAFE` (`instances/W5/VERIFY_BRIEF.md`; C1–C10
confirmed; independent ten-class screen no hit; all twelve pinned SHA-256
digests recomputed and matched; strike-no-rows confirmed against the live
`_STATUS.md`). Its two Low, non-operative defects were cured in place
before effect: the §2 prior-bundle count corrected to the live lineage
(one `PASS` plus two `FAIL` and one `BLOCKED`), and the §2 "Rerun
Consequence" quotation corrected to the manual's actual "post-#287"
wording. Its three INFO notes (D-54 §2 DEL-09-04-exclusion disposition
via the R11-era-protection reading; §4.4 trigger-clause wording read as
bounded by its action clause; historical 11-hex name of the oldest
bundle) are recorded without text change — none is load-bearing.

**Prepared by:** WORKING_ITEMS (Agent 1, W5 reproduction-wave manager) for
HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W5

**Selected work item:** one fresh actor-neutral clean-checkout reproduction
of the CURRENT documented validation-manual procedure
(`docs/validation_manual/headless_runner_reproduction.md`, both Part 1 and
Part 2) from pinned source commit
`a5235340aae3c41cf227f5617e593b268936f6b3` (the R14 head — merge of PR #294,
the third merged R14 wave), producing one new immutable `INTERNALLY_VERIFIED`
derivative bundle in the DEC-080 evidence home
`validation/evidence/reproduction/<RUN_ID>/`. This restores current-head
R6-criterion reproduction evidence after the R14 mechanics waves changed the
procedure, the runner, and the solver. The tranche closes NO Remaining row
(DEL-09-04 `## Remaining` stays byte-identical — both bullets are owner-gated
per W3 rows 7–8), performs no reproduction-result acceptance, no lifecycle
change, no evidence-posture promotion, and no code, schema, test, fixture, or
witness change.

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on
the D-52/`DEC-085` standing-approval overlay. Adoption remains the human
owner's conditional act under the standing rule; this document classifies and
proposes only. The adoption effect was `HELD` at authoring until the
fresh-context independent refutation returned `COMMIT-SAFE` (see the
verification record above), after which the W5 manager progressed the
chain under the R14 campaign-plan execution rules; §10 records the
effective state.

## 1. Purpose, Trigger, and Accepted Basis

Purpose: produce one actor-neutral, local-only, offline reproduction of ALL
cases documented by the current validation-manual reproduction page — the
three frozen E1 `tp_runner_015` cases under their CURRENT dated expectations
and the five bound `del1005_payload_binding_*` cases — from one clean
checkout of one pinned source commit, recording environment, tool versions,
exact commands, exit codes, predicate verdicts, and output hashes in a new
immutable DEC-080 bundle.

Trigger: the prior clean-reproduction brief
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`
(`CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001`) §8 requires a new run and a new
immutable bundle when the E1 procedure, the runner, or the solver changes.
All three changed after its source commit `23eeaabc904064e2297690e391df153dea116ff0`:

- the documented procedure was refreshed twice (R13
  `CB-2026-07-19-DEL-09-04-VALMANUAL-REFRESH-001`; R14-W4 T7
  `CB-2026-07-20-T7-DEL-09-04-VALMANUAL-STALE-001`) — it now carries Part 2
  (five bound cases) and dated case-1/case-3 notes;
- the runner changed (PR #287 bound the `run-benchmark`/`run-regression`
  payloads, implementation commit `60841413a`, merged `45ec0524d`);
- the solver changed (R14 W1 T2 constant-effort consumption `faee4faed`,
  W1 T3 arc pressure thrust, W2 T4 sub-span wind — merged at `581a15b1c` and
  `e315fb840`).

The R11 brief's pinned per-case predicates are therefore stale (its case 3
expects the pre-#287 stub diagnostic; its case 1 expects a diagnostic-clean
solve). This is a NEW brief with predicates re-anchored to the live manual;
it does not amend, reinterpret, or invalidate the R11 brief or the completed
R11 bundle
`validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`,
which remains immutable and truthful for its pinned commit.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation (branch
`claude/piping-r14-w5-clean-repro` at
`a5235340aae3c41cf227f5617e593b268936f6b3`, working tree clean):

- root and project `AGENTS.md`; active workplan
  `loop/WORKPLAN_2026-07-18b_piping_loop.md`; the R14 campaign plan
  (`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/ORCHESTRATION_PLAN.md`)
  and the W1–W4 manager returns; receipt cursor `Receipt-63`
  (structurally valid `loop/LOOP_RECEIPTS.md`);
- the current procedure
  `docs/validation_manual/headless_runner_reproduction.md`, SHA-256
  `fa714cf44d5c3e8a54ff6e2f6883676b81e01755e2e07d36a5bd118576b299c1` at the
  source commit (Part 1 frozen E1 fixture set with dated 2026-07-19 case-3
  and 2026-07-20 case-1 notes; Part 2 bound fixture set with five
  `del1005_payload_binding_*` cases, expected exits 0/0/0/1/1; Review
  Checks section);
- the R11 precedent brief (execution mechanics §3, bundle layout §3.5,
  profile-check list §4.2 — reused below with only the deltas the current
  tree requires) and the completed R11 bundle (layout precedent);
- DEC-080 (evidence home
  `validation/evidence/reproduction/<run-id>/`), amended `docs/PRD.md`
  §§22.1, 22.5, 24 R6; DEC-065 exit policy; DEC-081 claims taxonomy;
  D-52/`DEC-085` and D-54/`DEC-087` (standing-approval overlay and
  reasoned-selection refinement, durably SHA-bound at governance commit
  `f14fa77518a06f112ae72a8fcce4de0fab958d47`);
- DEL-09-04 deliverable folder (under
  `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`):
  `_STATUS.md` (`IN_PROGRESS`; two Remaining bullets, both owner-gated,
  both preserved byte-identical by this tranche), `MEMORY.md` (W4 T7 entry
  newest), and the R11/R13/W4-T7 run records;
- frozen surfaces at the source commit (read-only; SHA-256 pinned below in
  §3.4);
- the environment-repair precedent
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/ENVIRONMENT_REPAIR_DISPOSITION.md`
  (R12-ENVREPAIR-01: offline, ignored-paths-only materialization of already
  present local state — adapted in bounded form by §4.4 only if the named
  failure class occurs);
- `software-workflow.json` (SHA-256
  `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f`) and
  the root tools `tools/software_workflow/run_registered_checks.py` and
  `tools/software_workflow/validate_change_scope.py`.

## 2. Live Selection Facts

- DEL-09-04 is `IN_PROGRESS`; its former clean-checkout-reproduction
  Remaining bullet was discharged by the R11 run and no Remaining bullet
  names this rerun; the rerun obligation arises from the R11 brief's §8
  rerun clause and the manual's own "Rerun Consequence" section ("any
  subsequent clean-checkout reproduction executes from a post-#287 source
  commit under a fresh run ID and a new immutable bundle under
  `validation/evidence/reproduction/<run-id>/`"). Consequently this
  tranche strikes NO Remaining row anywhere; progress is recorded in
  History/MEMORY/run-record only.
- The two current Remaining bullets (E2 residuals; DEC-046 tolerance
  promotion) are owner-gated (W3 rows 7–8) and stay byte-identical.
- The DEC-080 evidence home and the `REPRO_DEL0904_<UTC>_<SHA12>` run-ID
  convention are unchanged. Four prior bundles live under
  `validation/evidence/reproduction/` (one `PASS` — the R11 bundle — plus
  a preserved two-`FAIL`/one-`BLOCKED` truthful lineage); none is edited.
- The worktree at `REPO_ROOT` carries the R12-ENVREPAIR-01 offline
  provisioning (`node_modules/**`, per-crate gitignored `Cargo.lock`
  files). A clean CLONE of this repo will NOT carry the gitignored lock
  state; the R11 run succeeded from the same clean-clone posture using the
  shared local Cargo cache offline. §4.4 bounds the only permitted
  contingency.
- Receipt cursor is `Receipt-63`; W5 appends NO receipt (HELP_HUMAN fan-in
  act).

## 3. Objective and Enumerated Acceptance Predicates

From one local clean checkout of pinned
`SOURCE_COMMIT=a5235340aae3c41cf227f5617e593b268936f6b3`, run both
documented reproduction procedures offline and validate every predicate
below. Freeze one `RUN_ID` before the first durable write:
`REPRO_DEL0904_<UTC-YYYYMMDDTHHMMSSZ>_a5235340aae3`.

### 3.1 Part 1 — frozen E1 cases (current dated expectations)

- **P1 (generator determinism).**
  `python3 validation/witness/inputs/generate_tp_runner_015_inputs.py`
  exits 0 and a scoped `git diff --exit-code` over the three
  `tp_runner_015_final_cli_*_input.json` fixtures proves byte-identical
  regeneration. Anchor: manual Part 1 procedure; R11 precedent §3.4.
- **P2 (case 1, solve).** Exit `0`; `runner_result.job.state` is
  `COMPLETED`; `request_validation.diagnostics` and
  `result_validation.diagnostics` are empty; `runner_result.result_refs`
  is non-empty; and the output carries exactly one
  `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` diagnostic in the entire
  document, located in `mechanics_envelope.diagnostics`, severity
  `warning` (non-blocking), with `support:CE-120` among its affected
  refs. The result is NOT diagnostic-clean and that is the documented
  expectation. Anchor: manual Part 1 case-1 row plus the dated 2026-07-20
  note; frozen solve input `supports[6]` (`support:CE-120`,
  `restraints: []`).
- **P3 (case 1, witness relationship).** The regenerated case-1 output
  does NOT byte-match the committed witness
  `validation/witness/generated/tp_runner_015_final_cli_solve.json`
  (SHA-256
  `c406d9c2d8b6e739cd8faf86fcd67ff8f685342f9ee056b5544685a769705188`), per
  the dated 2026-07-20 note; both digests are recorded. Informational,
  non-predicate observation: the W4 T7 live head run recorded regenerated
  digest
  `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613` and
  830 `result_refs`; a differing regenerated digest with P2 fully passing
  is recorded truthfully and is not by itself `FAIL`.
- **P4 (case 2, validate-input).** Exit `1`; request validation reports
  `HEADLESS_RUNNER_LOAD_BASIS_MISSING`; no solver result is emitted.
  Anchor: manual Part 1 case-2 row. The byte comparison of the case-2
  output against
  `validation/witness/generated/tp_runner_015_final_cli_validation_blocking.json`
  (SHA-256
  `0d707ee26f4bbdd979f06580b46c658a7e10fdcde7f7833585bb15f2863dd1c1`) is
  recorded as an observation, not a predicate.
- **P5 (case 3, run-benchmark on the frozen payload-less input).** Exit
  `1`; blocking diagnostic `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`
  (post-#287 source, per the dated 2026-07-19 note). The committed
  benchmark-stub witness
  `validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`
  (SHA-256
  `420484c82e798b9f0bb12d17d4381dab18b17f0c7468aa6110607c16ded94d4d`)
  records the historical pre-#287 stub diagnostic, so a byte mismatch
  against it is EXPECTED and recorded; the diagnostic predicate above is
  what passes or fails.

### 3.2 Part 2 — bound per-case benchmark/regression cases

- **P6 (generator determinism).**
  `python3 validation/witness/inputs/generate_del1005_payload_binding_inputs.py`
  exits 0 and a scoped `git diff --exit-code` over the five
  `del1005_payload_binding_*_input.json` fixtures proves byte-identical
  regeneration. Anchor: manual Part 2 ("writes only
  `del1005_payload_binding_*` input files and does not touch the frozen
  `tp_runner_015` surfaces" — the scoped diff plus the P13 whole-tree
  check proves both halves).
- **P7 (benchmark, single named case).** Exit `0`; `suite_run.suite`
  `mechanics`; `requested_case_count` 1; case
  `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` `executed_and_matched` (1/1);
  top-level `diagnostics` empty; output byte-identical to the committed
  witness
  `validation/witness/generated/del1005_payload_binding_benchmark_single_case.json`
  (SHA-256
  `813a702b74be74a88755626d5b4530716d4fd5e27a1b988e48f3da3be3306728`).
- **P8 (benchmark, multiple named cases).** Exit `0`; suite `stress`;
  the three named cases `STRESS-AXIAL-NORMAL-ORIGINAL`,
  `STRESS-RANGE-MECHANICS-ORIGINAL`,
  `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` all
  `executed_and_matched` (3/3); `diagnostics` empty; output
  byte-identical to
  `validation/witness/generated/del1005_payload_binding_benchmark_multi_case.json`
  (SHA-256
  `8feb3d25e50e78dcd7fcc85e2253021610faa971a37837eefb63df5cea456d68`).
- **P9 (regression, whole-suite default).** Exit `0`; suite `nonlinear`;
  `whole_suite_default_applied` `true`; 5/5 `executed_and_matched`;
  `diagnostics` empty; output byte-identical to
  `validation/witness/generated/del1005_payload_binding_regression_full_suite.json`
  (SHA-256
  `2f89adce9e4d6250280cee347822a567f4405eafbb8bc666483c6ce4cbd87593`).
- **P10 (benchmark, payload missing).** Exit `1`; blocking diagnostic
  `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`; output byte-identical to
  `validation/witness/generated/del1005_payload_binding_benchmark_payload_missing.json`
  (SHA-256
  `9596c052c76a178e13bcf29faa5841848df6d9453983b184ebff3fd5fc2449a4`).
- **P11 (regression, payload missing).** Exit `1`; blocking diagnostic
  `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`; output byte-identical to
  `validation/witness/generated/del1005_payload_binding_regression_payload_missing.json`
  (SHA-256
  `61cba4f28bcf109510489125b1de11e44796a25285ca64bf9c0714e870d9f518`).

Anchors for P7–P11: manual Part 2 Bound Fixture Set table and Reproduction
Procedure ("Expected exits are 0, 0, 0, 1, 1 respectively"); the five
committed witnesses (which serialize `"diagnostics": []` for the three
success cases). Byte-identity against the committed witnesses is required
here per the W5 dispatch; the witnesses were reproduced against these
commands in the R12 chain and are byte-identical at the source commit.

### 3.3 Review checks, cleanliness, bundle, and state

- **P12 (documented review checks).**
  `cargo test --offline --manifest-path core/runner/headless/Cargo.toml`
  (with `CARGO_NET_OFFLINE=true` and a temporary `CARGO_TARGET_DIR`) exits
  0, and `python3 tests/test_headless_runner_contract.py` exits 0, both
  run inside the clean clone. Anchor: manual Review Checks section.
- **P13 (clean-checkout discipline).** The clone is created from the local
  `REPO_ROOT` only (`git clone --no-hardlinks --no-checkout`), detached at
  `SOURCE_COMMIT`, with empty
  `git status --porcelain=v1 --untracked-files=all` and clean
  `git diff --exit-code` / `git diff --cached --exit-code` both before the
  first command and after the last (tracked surfaces; ignored build
  metadata is not evidence and is never copied into the repository).
- **P14 (bundle integrity and label).** The bundle at
  `validation/evidence/reproduction/<RUN_ID>/` is complete per §5.2,
  `SHA256SUMS.txt` verifies, the bundle is never overwritten or reused,
  and `README.md` uses the `INTERNALLY_VERIFIED` evidence label only on
  `PASS`, states the adoption basis (this brief under D-52/`DEC-085` as
  refined by D-54/`DEC-087`, SHA-bound at
  `f14fa77518a06f112ae72a8fcce4de0fab958d47`), and states explicitly that
  NO owner acceptance of the reproduction result occurred. It must not use
  `PROVER_CORRELATED` or `ENGINEER_ACCEPTED`.
- **P15 (bounded state update; strike NO rows).** DEL-09-04 `_STATUS.md`:
  `## Remaining` byte-identical (both bullets); exactly one new History
  entry and updated `Last Updated`; `Current State: IN_PROGRESS`
  unchanged. `MEMORY.md`: exactly one new entry (newest-first).
  Exactly one new
  `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W5_CLEAN_REPRO.md`.
  No receipt append.
- **P16 (checks).** The §6 validation plan passes in full, including the
  single evidence-sweep with a proven one-file delta.

### 3.4 Frozen-surface identity (read-only pins at the source commit)

The three `tp_runner_015` input fixtures, their generator (SHA-256
`5a9f29b7a24163706c78c0ce412b4fa270aafdcd19f44a3d4edba2de84feae68`), the
three committed `tp_runner_015` witnesses (P3–P5 digests), the five
`del1005_payload_binding_*` input fixtures, their generator (SHA-256
`4c81b645723495dee04a5ad854d7274b6e863883ed382d3117e728585c062b9f`), and
the five committed `del1005` witnesses (P7–P11 digests) are read-only
frozen surfaces. They remain byte-identical in the primary tree after this
tranche (P13/§6 guard).

A successful run closes no Remaining bullet, promotes nothing, and accepts
nothing.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several defensible shapes existed, the selection below was made under
D-54/`DEC-087`; the ten-class fast-reject screen, four-lens analysis, and
materially rejected alternatives are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W5/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape: **one new brief, one clean clone, all eight documented
cases, one immutable bundle** — reuse the R11 brief's execution mechanics
verbatim where the tree is unchanged (local-only mktemp clone, detached
checkout, cleanliness proofs, offline cargo, separate argv/stdout/stderr/
exit capture, immutable bundle layout, checksum discipline) and re-anchor
only what the current tree changed (current per-case predicates incl. the
non-diagnostic-clean case 1 and the post-#287 case 3; the five added bound
cases with witness byte-comparison; strike-no-rows state handling; W5
chain artifacts instead of R11's).

Bounded tasks for the executor child:

### 4.1 Freeze the execution basis

- Work on branch `claude/piping-r14-w5-clean-repro` in the primary
  checkout; verify `git rev-parse HEAD` equals
  `SOURCE_COMMIT=a5235340aae3c41cf227f5617e593b268936f6b3` and the tree is
  clean apart from this run's lawful pre-existing W5 state (this brief and
  `instances/W5/**`). Stop on material drift of the DAG pointer,
  DEL-09-04 lifecycle/Remaining, DEC-080, the procedure page, or
  `software-workflow.json`; do not silently reinterpret scope.
- Freeze `RUN_ID` before the first durable write; record the adopted brief
  content SHA-256 (at its effective state) in the run record and manifest.

### 4.2 Preflight without provisioning

Record existing tools only (`git --version`, `python3 --version`,
`rustc --version --verbose`, `cargo --version --verbose`, `uname -a`,
`sw_vers` on Darwin, plus `command -v` absolute paths). Do not install,
update, download, or activate any toolchain or dependency. If an existing
tool or the offline Cargo cache is insufficient, finalize the run as
truthful `BLOCKED` evidence and stop (§4.4 names the single bounded
exception).

### 4.3 Create the clean local-only clone

Per the R11 mechanics: `mktemp -d`;
`git clone --no-hardlinks --no-checkout "$REPO_ROOT" "$REPRO_CHECKOUT"`
(URL/fetch/pull/submodule/network sources forbidden);
`git -C "$REPRO_CHECKOUT" checkout --detach "$SOURCE_COMMIT"`; prove
identity and cleanliness (P13); set
`REPRO_PROJECT="$REPRO_CHECKOUT/projects/chirality-piping"` and keep
`CARGO_TARGET_DIR` under the temporary root, outside the checkout. The
clone, target dir, and scratch logs are ephemeral; they are removed only
after their evidence is copied and hashed, and only by deleting the
validated task-created temporary root.

### 4.4 Bounded environment contingency (only on the named failure class)

The clean clone lacks the gitignored per-crate `Cargo.lock` files (the R11
run succeeded from this same posture via the shared local Cargo cache,
offline). If — and only if — an offline cargo command fails specifically
because of this missing gitignored-lockfile/ignored-build-state class, the
executor may adapt the R12-ENVREPAIR-01 precedent in bounded form: copy the
relevant already-present gitignored `Cargo.lock` files (ignored paths only;
nothing tracked; no `node_modules` — this reproduction does not use them;
no network, registry, download, or install) from the primary `REPO_ROOT`
tree into the clone, recording each copied path, the failure evidence that
triggered it, and the precedent citation in `commands.jsonl`, the bundle
README, and the run record. Cleanliness proofs must remain satisfied
(tracked status unchanged). Any other missing prerequisite is a truthful
`BLOCKED` result — no provisioning.

### 4.5 Execute the documented procedures offline

From `REPRO_PROJECT`, preserving each command as an argv/cwd/environment
record with separately captured stdout, stderr, and actual exit code; never
place expected-nonzero commands in a shell chain that could skip them or
let a failed mandatory assertion fall through:

1. P1 generator + scoped determinism diff (three `tp_runner_015` inputs);
2. the three frozen E1 runner commands (manual Part 1 procedure) with
   `CARGO_NET_OFFLINE=true`, `--offline`, temporary `CARGO_TARGET_DIR`,
   and explicit `--output` files under the temporary output dir;
3. P6 generator + scoped determinism diff (five `del1005` inputs);
4. the five bound runner commands (manual Part 2 procedure), same offline
   posture, explicit `--output` files;
5. the two documented review checks (P12), as separate recorded commands;
6. the P13 post-run cleanliness checks.

Cargo may compile from already-present local cache; it may not retrieve
missing material.

### 4.6 Validate outputs and build the immutable bundle

Using Python's standard JSON parser (or another already-present local
tool), assert every §3 predicate against the eight output files; perform
the byte comparisons (`cmp` or hash equality) for P3–P5 records and
P7–P11 predicates; write the bundle per §5.2; compute and verify
`SHA256SUMS.txt` last; never overwrite or reuse the run directory.

### 4.7 Update bounded state and close out

On `PASS` only: apply P15, then run §6 in order. On `FAIL` or `BLOCKED`:
keep DEL-09-04 state limited to a truthful History/MEMORY/run-record
account (Remaining and lifecycle unchanged in all outcomes), preserve the
truthful failed/blocked bundle per the R11 precedent §7, write
`EXECUTE_RETURN.md` under `instances/W5/`, and return to the W5 manager —
no repair in this tranche. The executor does not commit; the W5 manager
commits after independent implementation verification.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited to:

1. this candidate brief, only for the governed status record or a later
   superseding hold/rejection record;
2. exactly one new `validation/evidence/reproduction/<RUN_ID>/**` bundle;
3. exactly one new tool-emitted
   `validation/evidence/sweeps/SWEEP_*.json` from the single mandatory
   profile-run evidence sweep, verified as a one-file before/after delta;
4. DEL-09-04 only (under
   `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`):
   `_STATUS.md` (History + `Last Updated` only; `## Remaining`
   byte-identical), `MEMORY.md` (one new entry), one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W5_CLEAN_REPRO.md`;
5. the wave instance directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W5/**`;
6. NO receipt append (`loop/LOOP_RECEIPTS.md` untouched; HELP_HUMAN
   fan-in act), and no branch/commit act by the executor (the W5 manager
   commits).

Ephemeral writes are limited to one task-specific temporary local clone,
Cargo target directory, outputs, and logs under the `mktemp -d` root, plus
the §4.4 ignored-paths-only contingency inside that clone. Cleanup may
remove only the resolved temporary root after its absolute path has been
validated as the task-created directory.

### 5.2 Required bundle contents (R11 layout precedent)

```text
README.md
manifest.json
environment.txt
source_commit.txt
procedure.sha256
git-status-before.txt
git-status-after.txt
commands.jsonl
validation_summary.json
SHA256SUMS.txt
stdout/<command-id>.txt
stderr/<command-id>.txt
exit_codes/<command-id>.txt
outputs/tp_runner_015_solve.json
outputs/tp_runner_015_validation_blocking.json
outputs/tp_runner_015_benchmark_stub.json
outputs/del1005_benchmark_single_case.json
outputs/del1005_benchmark_multi_case.json
outputs/del1005_regression_full_suite.json
outputs/del1005_benchmark_payload_missing.json
outputs/del1005_regression_payload_missing.json
checks/<check-id>.json-or-txt   (incl. witness-compare, sweeps-before/after/delta, change-scope.json)
```

`manifest.json` identifies the bundle as a derivative package
(`authoritative_truth: false`); cites `SOURCE_COMMIT`, the adopted brief
and its SHA-256, DEC-080, the procedure path plus its §1 SHA-256, and the
profile SHA-256; records run ID, managed run/wave, UTC timestamps, cwd,
argv, environment overrides, expected and actual exit codes, input hashes,
output hashes, witness digests, and every §3 predicate verdict; and states
`overall_status` as `PASS`, `FAIL`, or `BLOCKED`. `README.md` requirements
per P14, including the sentence: "Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081)."

No other project file is writable. In particular, do not write or fix:
`core/**`, `tests/**`, `schemas/**`, `tools/**`, `fixtures/**`, fixture or
generator inputs, committed witnesses, the validation-manual pages, any
other deliverable or package (including DEL-10-05), registers, DAGs,
decomposition, decision packets, PRD/PLAN, workplan, prior bundles or
sweeps, root governance, `_DomainEngines/**`, app-dev, PEC, or any external
path. No push, pull, fetch, PR, merge, or other network/external state
change by any W5 node.

## 6. Evidence and Validation Plan

Because this tranche writes `validation/**` and `execution/**`, the
project profile selects `piping-pytest`, `evidence-sweep`,
`harness-pytest`, and always-check `harness-self-check`. Division of labor
per the W5 dispatch: the EXECUTOR runs `evidence-sweep` (the check bound to
the `validation/**` write, with the one-file delta proof); the W5 MANAGER
runs the remaining three registered checks once at wave closeout with
normalized JSONs to `instances/W5/`.

Executor sequence, every failure stopping subsequent state-changing
closeout; no check may install, fetch, or update anything:

- §3 predicate validation (P1–P13) and bundle assembly (§4.6, P14);
- state updates (P15);
- evidence-sweep: snapshot `validation/evidence/sweeps/` before/after, run
  `python3 tools/software_workflow/run_registered_checks.py
  "$WORKING_ROOT/software-workflow.json" --check evidence-sweep
  --output "$BUNDLE/checks/evidence-sweep.json"` from `REPO_ROOT`, and
  stop unless the delta is exactly one new `SWEEP_*.json`;
- `python3 tools/validation/validate_claims_language.py --repo-root .`
  and `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `python3 tools/validation/validate_piping_loop_receipts.py
  --repo-root .` from `REPO_ROOT` (consistency only — the receipts file
  must be untouched by W5);
- `git diff --check` from `REPO_ROOT`; JSON/JSONL parse of every new
  `.json`/`.jsonl` file;
- changed-path containment from `REPO_ROOT` after freezing `RUN_ID`,
  `SWEEP_FILE`, and the run-record filename:

```text
python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT" \
  --base "$SOURCE_COMMIT" \
  --allowed projects/chirality-piping/execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_DEL-09-04_CLEAN_REPRO_R14.md \
  --allowed "projects/chirality-piping/validation/evidence/reproduction/$RUN_ID" \
  --allowed "projects/chirality-piping/validation/evidence/sweeps/$SWEEP_FILE" \
  --allowed "projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md" \
  --allowed "projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/MEMORY.md" \
  --allowed "projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W5_CLEAN_REPRO.md" \
  --allowed "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W5"
```

persisting its JSON stdout to `$BUNDLE/checks/change-scope.json` by
ordinary redirection (the tool includes untracked, non-ignored paths).

Manager closeout (after the evidence commit): `piping-pytest`,
`harness-pytest`, `harness-self-check` via
`tools/software_workflow/run_registered_checks.py`, each as its own
halting step, JSONs to `instances/W5/CHECK_<name>.json`.

DEC-025 judgment: this evidence/state tranche touches no code, so the
workplan's code-touching pre-push rule does not independently trigger the
sweep; `software-workflow.json` selects the same `evidence-sweep` because
the bundle lands under `validation/**`, and it remains mandatory on that
distinct profile basis. Running the sweep creates no stage, lifecycle,
release, or acceptance authority.

## 7. Defect and Failure Disposition

- Fail closed. Expected exit `1` for the three diagnostic cases (P4, P5,
  P10, P11) is evidence only when the matching diagnostic predicate
  passes; any different exit or diagnostic is `FAIL`.
- A P7–P11 witness byte-mismatch, output-schema drift, changed frozen
  fixture bytes, review-check failure, hash or checksum mismatch,
  sweep/validator failure, non-clean tracked source state, or unexpected
  changed path is `FAIL` and stops closeout.
- Missing offline dependencies/tools outside the §4.4 named class are
  `BLOCKED`; do not install or fetch.
- Preserve the unique truthful failed/blocked bundle and record the
  observed evidence; Remaining and lifecycle stay unchanged in every
  outcome; a rerun uses a new `RUN_ID` and cites the failed run. No
  repair of runner code, fixtures, tests, docs, governance, or another
  deliverable in this tranche — return the condition to the W5 manager,
  who returns it to HELP_HUMAN.
- If the primary checkout is dirty beyond lawful W5 state, or the clone is
  not clean before execution: stop and return the condition; no stash,
  reset, revert, or interpretation of unrelated work.

## 8. Rerun Triggers

A new run and new immutable bundle are required when any of these changes
after `SOURCE_COMMIT`: the documented procedures (either part), either
generator, any of the eight input fixtures, any committed witness, the
headless-runner manifest/source/dependencies, the suite crates they bind,
the contract test, the solver surfaces they exercise; DEC-080, PRD §§22/24,
the DEL-09-04 acceptance/Remaining scope or the evidence home; applicable
DAG rows or the approved DAG pointer; `software-workflow.json` or the root
profile; any toolchain/environment change when evidence is compared across
runs; or a prior `FAIL`/`BLOCKED` after its condition resolves. Completed
bundles are never edited, reused, or overwritten.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- code, fixture, witness, test, documentation, schema, tool, dependency,
  or workflow fixes (including the routed fallback-fixture and
  `validation.rs` observations, and the `export-results` DEL-10-05 stub);
- network access, dependency/tool installation (§4.4's ignored-paths-only
  local copy is the sole bounded exception), hosted CI, cloud/daemon/
  telemetry operation, publication, push, pull, fetch, PR creation,
  merge, rebase, or release;
- reproduction-result acceptance, `ENGINEER_ACCEPTED` or
  `PROVER_CORRELATED` status, evidence-posture promotion, or third-party/
  publication-era reproduction;
- `MAINTAINER_REVIEWED` case-page promotion, GUI-workflow evidence,
  public-benchmark tolerance/threshold promotion (DEC-046), or closure of
  any Remaining bullet;
- external-prover procurement/activation or comparison;
- target-stage advancement, lifecycle transition, issuance, receipt
  append, signing, notarization, any D-06b act, or any D-45 act.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval — Attribution and Effect Record

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (W5 manager, R14 campaign)
RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL
ClassifiedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W5
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected shape per §4)
SelectedOutcome: one fresh actor-neutral clean-checkout reproduction of the current documented procedure (all eight cases, current dated expectations) from source commit a5235340aae3c41cf227f5617e593b268936f6b3 into one new immutable INTERNALLY_VERIFIED-only DEC-080 bundle, R11 mechanics reused, strike-no-rows state handling, per §3–§4 within the §5 fence
JudgedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W5
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (rerun under the stale R11 brief; E1-only reproduction; defer to a later quiet head; witness regeneration; predicate-only comparison without witness byte-checks)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W5/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `instances/W5/VERIFY_BRIEF.md` (C1–C10 confirmed; ten-class screen no hit; two Low defects cured in place; three INFO notes recorded)
EffectStatus: EFFECTIVE — EXECUTION RELEASED BY W5 MANAGER UNDER THE R14 CAMPAIGN CHAIN (VERIFIER COMMIT-SAFE)
PreservedGates: reproduction-result acceptance and evidence-posture promotion; MAINTAINER_REVIEWED case-page promotion; GUI-workflow validation evidence; DEC-046 threshold/tolerance promotion; export-results binding (DEL-10-05); lifecycle/stage/issuance/release/acceptance; prover activation/correlation; publication/external action; merge authority; D-45; D-06b; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the agent
classifies, selects among defensible reproduction shapes under D-54, and
proposes. The W5 manager progressed `EffectStatus` to `EFFECTIVE` only
after the independent refutation returned `COMMIT-SAFE`, under the R14
campaign plan's execution rules and the durably landed D-52/D-54 records.
Execution is released to the bounded W5 executor within the §5 fence;
adoption does not accept a reproduction result, advance a lifecycle or
target stage, or perform any other owner act.
