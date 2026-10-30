---
doc_id: CB-2026-07-20-T6-DEL-09-01-BENCH-EVIDENCE-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-20
package_id: PKG-09
deliverable_id: DEL-09-01
decision_basis: DEC-054 (named residual), DEC-026 (recorded comparison tier, reused), DEC-046 (preserved gate), DEC-065, DEC-080 (evidence-home precedent), DEC-081, DEC-085, DEC-087
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — T6 DEL-09-01 Benchmark-Evidence-System Bounded Construction

**Status:** `EFFECTIVE — EXECUTION RELEASED BY W4 MANAGER UNDER THE R14 CAMPAIGN CHAIN (VERIFIER COMMIT-SAFE)`

**Verification record (2026-07-20):** the fresh-context adversarial brief
verifier returned `COMMIT-SAFE`
(`instances/W4/T6/VERIFY_BRIEF.md`; C1–C10 confirmed against the live
tree; independent ten-class screen pass). Its one MEDIUM, non-load-bearing
defect (D1: the controlling HELP_HUMAN W4 dispatch had no durable
artifact) is cured by the verbatim transcript at
`instances/W4/W4_DISPATCH_TRANSCRIPT.md`; its two minor notes (DEC-054
quotation splice "complete"/"full", semantically identical; cargo target
directory note) are recorded without brief-text change.

**Prepared by:** WORKING_ITEMS (W4, PKG-09 package manager) for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W4, tranche T6

**Selected work item:** the partially-agent-lawful scope of the DEL-09-01
`_STATUS.md ## Remaining` third row — "Complete the PRD §16.2 benchmark
evidence system named residual by the conditional R4 gate (see also DEL-09-04
for §16.5) (source: PRD plan §3 D9 exit-refresh row / DEC-054)" — exactly as
bounded by the W3 read-only assessment
(`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W3/RETURN.md`,
per-row table row 3 and terminal summary): bounded evidence-system
CONSTRUCTION only — benchmark family inventory, provenance/redistribution
index construction, witness/fixture verification refresh, and deterministic
assembly of a labeled derivative evidence artifact set. The row itself is NOT
closed, NOT struck, and NOT partially struck by this tranche; residual
completion is judged at the owner's R5-exit gate. Progress is recorded in
DEL-09-01 History/MEMORY and the run record only.

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on
the D-52/`DEC-085` standing-approval overlay. Adoption remains the human
owner's conditional act under the standing rule; this document classifies and
proposes only. The adoption effect is `HELD` until independent refutation
returns `COMMIT-SAFE` and the W4 manager progresses the chain under the R14
campaign-plan execution rules. No execution writes are authorized by this
document in its current state.

## 1. Purpose and Accepted Basis

Purpose: construct, as a labeled derivative evidence package, the
benchmark-evidence-system artifacts that PDU-037 names as project-owned
verification evidence for DEL-09-01 — a current fixture-family inventory, a
dedicated provenance/redistribution index, and a recorded verification
refresh of the suite's test/fixture state at the execution head — assembled
deterministically, hash-manifested, and claim-calibrated, with every owner
gate preserved and the PDU-037 standing Remaining row kept open.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation
(branch `claude/piping-r14-pkg09-evidence` at post-wave-2 main
`e315fb840`, which includes the R14-W1 PKG-04 tranches, the R14-W2
sub-span wind tranche, the XREPAIR-01 desktop mirror cure, and Receipt-62):

- root and project `AGENTS.md`; active workplan
  `loop/WORKPLAN_2026-07-18b_piping_loop.md`; R14 campaign plan
  (`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/ORCHESTRATION_PLAN.md`)
  and the W4 dispatch (queue item T6, quoted scope and hard exclusions);
- the W3 read-only assessment (`instances/W3/RETURN.md`): row 3 is the
  single partially-agent-lawful PKG-09 Remaining row; its lawful slice is
  evidence-system construction under D-52/`DEC-085` + D-54/`DEC-087`,
  consistent with PDU-037 `AuthorityNeeded=NO` and the R12/R13 precedent
  (Receipts 59/60); row closure is owner-gated at the R5 exit;
- `execution/_DAG/_LATEST.md` → approved `DAG-007`; the DEL-09-01 rows in
  `execution/_DAG/DAG-007/DependencyEdges.csv`: three ANCHOR rows
  (`NOT_APPLICABLE`), four `SATISFIED` execution-upstream rows toward
  PKG-00, and seven execution-upstream rows at `SatisfactionStatus=TBD`
  (toward DEL-04-01/02/03, DEL-05-01, DEL-01-02, DEL-02-02, DEL-04-06)
  whose Notes record the cause as refresh-scope ("target maturity not
  verified from permitted read set"), not a discovered blocker. Posture:
  this tranche consumes only the COMMITTED live suite crate and committed
  witnesses as they exist at the execution head — it asserts nothing about
  upstream deliverable maturity — and identical-posture governed DEL-09-01
  evidence refreshes have proceeded on these same rows (D-41 R5 T6
  PDU-037 refresh, `_STATUS.md` History 2026-07-12; W3 assessment). Any
  DAG-pointer change at execution is a freeze-check stop;
- DEL-09-01 deliverable folder (under
  `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark suite/`):
  `_STATUS.md` (`IN_PROGRESS`; four Remaining rows, all preserved
  byte-identical by this tranche), `MEMORY.md` (newest relevant entry
  records the R5-era evidence point: "33 Rust tests passed and the
  existing inventory still covers 21 project-original fixture/hand-calc
  families. Dedicated provenance index, runner/release integration, and
  validation disposition remain open."), `ScopeOfWork.md`,
  `Dependencies.csv`;
- concordance basis:
  `execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/PROPOSED_DELIVERABLE_UPDATES.csv`
  PDU-037 row (`AuthorityNeeded=NO`; proposed update "Add or refresh the
  cited fixture, benchmark, harness, coverage, or independent validation
  evidence without promoting verification to validation"; prerequisite "A
  separately authorized bounded implementation or documentation tranche")
  and PDU-013 row (`AuthorityNeeded=ENGINEERING`; unit-catalog acceptance
  held — untouched here);
- suite ground truth (live source, read-only for this tranche):
  `validation/benchmarks/mechanics/src/lib.rs` — `fixture_inventory()`
  currently returns 24 fixtures (readiness test asserts
  `fixtures.len() == 24`; 21 pre-R14 + `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD`
  (W1 T2) + `MECH-CURVED-BEND-PRESSURE-THRUST-ARC` (W1 T3) +
  `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` (W2 T4)); `MechanicsBenchmark
  { fixture_id, family, description, assumptions, provenance, unit_basis,
  expected_values }`; `BenchmarkProvenance { source_name,
  source_location, source_license, contributor,
  contributor_certification, redistribution_status, review_disposition }`
  with `RedistributionStatus::PublicOriginal` as the only variant;
  `tolerance_policy_is_unresolved()`; the fixture-local unit basis
  `PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K`; the two enforced inventory
  mirrors `validation/benchmarks/mechanics/README.md` and
  `validation/hand_calcs/mechanics/README.md`;
- runner ground truth (live source, read-only):
  `core/runner/headless/src/bin/openpipestress-runner.rs` and
  `core/runner/headless/src/benchmark_binding.rs` — `run-benchmark` binds
  suite `mechanics` (DEL-09-01) with whole-suite default when `cases` is
  omitted, per-case `executed_and_matched` / `executed_and_mismatched` /
  `blocked` reporting, fail-closed
  `HEADLESS_RUNNER_BENCHMARK_CASE_COMPARISON_BASIS_NOT_REUSABLE` blocking
  codes, `REGRESSION_EVIDENCE_CLAIM_POSTURE`, and the DEC-065 exit policy
  (exit 0 only with no blocking diagnostic; a whole-suite run containing
  blocked cases exits 1 — regression evidence, not a defect record);
- committed witness surfaces (frozen, read-only): the five
  `validation/witness/inputs/del1005_payload_binding_*_input.json` /
  `validation/witness/generated/del1005_payload_binding_*.json` pairs
  (the benchmark input shape to mirror), their generator, and the three
  `tp_runner_015_final_cli_*` fixtures/witnesses;
- evidence-home precedent: `validation/evidence/` currently holds
  `coverage/`, `gates/`, `release_artifacts/`, `reproduction/` (DEC-080
  run-id bundle convention `REPRO_DEL0904_<UTC>Z_<sha12>/`), and
  `sweeps/`; the export pipeline excludes the `validation/evidence/`
  subtree from public export (`tests/test_export_public_openpipestress.py`);
- governance: DEC-054 (D-27 conditional R4 gate; "PRD §16.2/§16.5
  complete benchmark/manual evidence system remains residual work" — i.e.
  named ordinary residual work; the §16.2 designation is quoted as
  recorded in the plan/status lineage — the live `docs/PRD.md` benchmark
  requirements now sit at §22.1–§22.2, and this tranche treats the row's
  own text as the selector, not the current PRD numbering), DEC-026
  (recorded analytic-class comparison tier, reused as recorded), DEC-046
  (governed tolerance records; promotion owner-gated and untouched),
  DEC-065, DEC-080, DEC-081 (claims taxonomy), D-52/`DEC-085`,
  D-54/`DEC-087`, and the R14 W1/W2 tranche briefs plus
  `CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md` as
  structural models;
- `software-workflow.json` under `docs/SOFTWARE_WORKFLOW_PROFILE.md`, and
  the root tools `tools/software_workflow/run_registered_checks.py` and
  `tools/software_workflow/validate_change_scope.py`.

## 2. Live Selection Facts

- DEL-09-01 is `IN_PROGRESS` with exactly four Remaining rows. Rows 1
  (PDU-037 keep-open), 2 (PDU-013 unit-catalog hold), and 4 (PDU-060
  human disposition) are HELD/OWNER-GATED per W3 and are untouched. Row 3
  (the §16.2 residual) is the selected row; only its agent-lawful
  construction slice is executed, and the row text stays byte-identical.
- The last recorded DEL-09-01 evidence point is R5-era: 21 families / 33
  tests (MEMORY 2026-07-12). The live suite carries 24 fixtures after the
  three R14 additions; no current-state family inventory, no dedicated
  provenance/redistribution index, and no post-R14 verification-refresh
  record exists anywhere in the tree. The two README mirrors enumerate
  fixtures but do not record per-fixture provenance/redistribution fields
  or head suite-run status.
- The suite crate already encodes per-fixture structured provenance
  (`BenchmarkProvenance`, `RedistributionStatus::PublicOriginal`) — the
  index is therefore a derivative EXTRACTION of committed content, not
  new provenance authorship.
- The runner's whole-suite `run-benchmark` on suite `mechanics` provides
  a deterministic, machine-readable per-case report at the execution
  head; on the R12 head it recorded 11/21 matched + 10 blocked (manual
  page, pinned). Head tallies for 24 fixtures are recorded by the run
  itself, whatever they are — asserted from the captured output only.
- Receipt cursor is `Receipt-62`; the wave branch is
  `claude/piping-r14-pkg09-evidence` at base `e315fb840`. No receipt is
  appended by this wave (HELP_HUMAN fan-in act).

## 3. Objective and Acceptance Predicates

Construct the evidence package so that all of the following hold on the
implementation head:

1. **Bundle home and identity.** One new directory
   `validation/evidence/benchmarks/BENCHEVID_DEL0901_<UTC>Z_<sha12>/`
   (UTC stamp `YYYYMMDDTHHMMSSZ` at assembly; `<sha12>` = first 12 hex of
   the execution base commit), mirroring the DEC-080 run-id bundle
   convention. Everything the tranche produces as evidence lives inside
   this one bundle. The bundle is derivative and immutable once the
   tranche commits: labeled `DERIVATIVE EVIDENCE — NON-AUTHORITATIVE` in
   its manifest, citing its accepted upstream basis (base commit, DAG-007
   pointer, DEL-09-01 `_STATUS.md` state, PDU-037/PDU-013 rows).
2. **Whole-suite head capture.** The bundle contains
   `suite_run_mechanics_input.json` — assembled by mirroring the
   committed `del1005_payload_binding_benchmark_single_case_input.json`
   shape with suite `mechanics`, the `cases` list OMITTED (whole-suite
   default), and a bundle-local manifest ref id — and
   `SUITE_RUN_MECHANICS.json`, the captured runner stdout of
   `openpipestress-runner run-benchmark` on that input at the execution
   head (offline cargo run). The recorded exit code, per-case statuses,
   `whole_suite_default_applied=true`, `requested_case_count`, and any
   `..._CASE_COMPARISON_BASIS_NOT_REUSABLE` blocking diagnostics are
   recorded exactly as emitted — blocked cases and a nonzero exit are
   truthful regression evidence, not failures of this tranche and not a
   defect record. No committed witness file is touched.
3. **Family/provenance/redistribution index.** The bundle contains
   `FAMILY_PROVENANCE_INDEX.csv` with exactly one row per fixture in
   `fixture_inventory()` (24 at preparation; the count is re-derived at
   execution), each row carrying at least: `fixture_id`, `family`,
   `constructor_fn`, `provenance_source_name`,
   `provenance_source_location`, `provenance_source_license`,
   `contributor_certification`, `redistribution_status`,
   `review_disposition`, `unit_basis`, `expected_value_count`,
   `tolerance_policy_state`, `witness_anchor_paths`,
   `witness_anchor_exists`, and `head_suite_run_status` (+ blocking code
   when blocked). Every field is extracted from the committed crate
   source, the committed README mirrors, the committed
   `validation/hand_calcs/mechanics/` inventory, or the §3.2 captured
   run — no field is invented, and every `witness_anchor_exists` must be
   `true` (a missing anchor is a recorded finding that stops closeout).
4. **Verification refresh recorded.** The bundle contains
   `TEST_REFRESH.md` recording, at the execution head: the mechanics
   suite `cargo test --offline` outcome and test count; the inventory
   count (24) and its readiness-test assertion; the two README mirrors'
   consistency with `fixture_inventory()`; the fixture-local unit basis
   id; the prior R5-era reference point (21 families / 33 tests, MEMORY
   2026-07-12) quoted as history; and the explicit statement that this
   refresh is project-owned VERIFICATION evidence only — no
   verification→validation promotion, and no tolerance, threshold,
   acceptance, release, or CI-gate content.
5. **Deterministic assembly.** The bundle contains `assemble_index.py`,
   a bundle-local, stdlib-only, offline script that (re)builds
   `FAMILY_PROVENANCE_INDEX.csv` and `MANIFEST.json` deterministically
   from its recorded inputs (the captured suite-run JSON, the committed
   crate source, the two README mirrors, the hand-calcs inventory), so
   the assembly is rerunnable and refutable. Running it twice in place
   yields byte-identical index output (the manifest's generated-at
   timestamp is supplied as a recorded argument, not wall-clock inside
   the index). The script writes only inside the bundle directory.
6. **Hash manifest and labels.** The bundle contains `MANIFEST.json`
   recording: bundle id, generated-at UTC, base commit SHA, branch, the
   derivative/non-authoritative label, the upstream citations (§3.1),
   SHA-256 of every other bundle file, the tool/commands used, the claim
   posture (DEC-081 regression/verification evidence), and the explicit
   standing statements: PDU-037's provenance-index Remaining row REMAINS
   OPEN (this bundle builds the index as evidence; row closure is judged
   at the owner's gate); PDU-013's unit-catalog hold is untouched
   (fixture-local unit basis only); acceptance thresholds,
   runner/release integration, tolerance promotion, and
   verification→validation promotion remain owner-class and are not
   performed.
7. **No product or witness change.** No file outside the bundle and the
   §3.8 state surfaces is created or modified. In particular the suite
   crates, runner, solver, schemas, tests, tools, README mirrors,
   committed witnesses, hand-calcs, reproduction bundles, and sweep
   artifacts are byte-identical at the implementation head (§6 verifies
   via containment and `git status`).
8. **Bounded state update.** On success only: DEL-09-01 `_STATUS.md`
   gains exactly one new History entry (recording the bundle id and that
   the §16.2 residual row remains open, judged at the owner's gate) and
   an updated `Last Updated`; the `## Remaining` section is byte-identical
   (all four rows, no strike, no edit); `Current State: IN_PROGRESS`
   unchanged. `MEMORY.md` gains exactly one new entry (current inventory
   24 families, head test count, bundle pointer, preserved holds). One
   new
   `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T6_BENCH_EVIDENCE.md`.
9. **Claims calibration.** All new text stays inside the DEC-081
   regression/verification-evidence posture; the words chosen must not
   state or imply validation, acceptance, release readiness, threshold
   satisfaction, or professional reliance anywhere in the bundle or
   state surfaces.
10. **Checks.** The full §6 validation plan passes.

A successful run closes NO Remaining row anywhere, changes no lifecycle
state, and promotes nothing.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several shapes were defensible, the selection below was made under
D-54/`DEC-087`; the four-lens analysis and materially rejected
alternatives are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T6/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape:

- **One immutable derivative bundle under `validation/evidence/benchmarks/`**,
  mirroring the DEC-080 run-id convention — not a `docs/` page, not a
  crate change, not a scattered artifact set.
- **Index = extraction of committed content** (crate provenance structs,
  README mirrors, hand-calcs inventory) joined with a **head whole-suite
  runner capture** as the deterministic machine source for per-case
  status — no invented values, no new provenance authorship.
- **Mechanics-suite scope only** (DEL-09-01's own row); stress/nonlinear
  suites belong to DEL-09-02/DEL-09-03, whose Remaining rows are
  held/owner-gated per W3.
- **Bundle-local stdlib assembly script** for rerunnable determinism,
  instead of a `tools/` addition (keeps the tool surface untouched) or
  an unscripted hand-assembled index (weaker refutability).
- **Row kept open, History/MEMORY-only recording**, exactly per the W4
  dispatch and W3 row-3 classification.

Bounded tasks for the executor child:

### 4.1 Freeze the execution basis

- Begin on the wave branch `claude/piping-r14-pkg09-evidence` in the
  integration checkout; record the base commit before any durable write;
  verify the tree is clean apart from this run's lawful pre-existing
  state (the R14 W4 instance directory and this brief).
- Re-verify the §1 symbol-level facts (inventory count assertion,
  provenance struct fields, README mirror presence, runner whole-suite
  default and blocking codes, absence of any existing
  `validation/evidence/benchmarks/` directory). Stop if the DAG pointer,
  the DEL-09-01 lifecycle/Remaining text, the named sources, or
  `software-workflow.json` changed materially. Do not silently
  reinterpret scope.

### 4.2 Capture, extract, assemble

- Create the bundle directory; assemble `suite_run_mechanics_input.json`
  (§3.2); run the whole-suite capture offline
  (`CARGO_NET_OFFLINE=true cargo run --offline --manifest-path
  core/runner/headless/Cargo.toml --bin openpipestress-runner --
  run-benchmark --input <bundle input> --output <bundle capture>`);
  record exit code and stdout/`--output` identity. A missing local build
  prerequisite is a truthful blocked result, not permission to
  provision.
- Write `assemble_index.py`; build `FAMILY_PROVENANCE_INDEX.csv` and
  `MANIFEST.json`; verify §3.5 rerun byte-identity; write
  `TEST_REFRESH.md` after running the §6 suite test step.

### 4.3 Update deliverable state and close out

On success only: apply §3.8, then run §6 in order. On failure or block:
leave deliverable state unchanged (or record the truthful partial
state), write truthful evidence and `EXECUTE_RETURN.md` under
`instances/W4/T6/`, and return to the W4 manager. The executor does not
commit; the W4 manager commits after independent implementation
verification.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited
to (paths relative to `WORKING_ROOT` unless noted):

1. this candidate brief, only for the governed status record or a later
   superseding hold/rejection record;
2. NEW directory
   `validation/evidence/benchmarks/BENCHEVID_DEL0901_<UTC>Z_<sha12>/**`
   (the §3 bundle: input JSON, captured suite-run JSON, index CSV,
   TEST_REFRESH.md, MANIFEST.json, assemble_index.py — and nothing
   outside it);
3. DEL-09-01 deliverable folder
   (`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark suite/`):
   `_STATUS.md` (History + `Last Updated` only; Remaining byte-identical),
   `MEMORY.md` (one new entry), one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T6_BENCH_EVIDENCE.md`;
4. the tranche instance directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T6/**`;
5. no evidence-sweep artifact in this tranche (single wave-level DEC-025
   sweep at W4 closeout per the controlling HELP_HUMAN W4 dispatch,
   mirroring the recorded W1/W2 refinement).

Ephemeral writes: task-local Cargo target dirs and scratch captures
outside durable project paths.

No other project file is writable. In particular: no
`validation/benchmarks/**`, `validation/hand_calcs/**`,
`validation/witness/**`, `validation/evidence/{coverage,gates,release_artifacts,reproduction,sweeps}/**`;
no `core/**`, `schemas/**`, `tests/**`, `tools/**`, `docs/**`, or
`fixtures/**`; no other deliverable or package folder, register, DAG,
decomposition, decision packet, PRD/PLAN, workplan, or receipt
(`loop/LOOP_RECEIPTS.md` untouched); no root governance,
`_DomainEngines/**`, app-dev, PEC, or external path. No push, pull,
fetch, PR, or merge.

## 6. Evidence and Validation Plan

In sequence from `WORKING_ROOT` unless noted; every failure stops
subsequent state-changing closeout; all cargo offline
(`CARGO_NET_OFFLINE=true`, `--offline`); no check may install, fetch, or
update anything (a missing local prerequisite is a truthful blocked
result):

- `cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml`
  (read-only verification for the §3.4 record; no crate file changes);
- the §3.2 whole-suite capture: recorded exit code and output captured
  into the bundle; stdout/`--output` byte-identity confirmed;
- §3.5 determinism: rerun `assemble_index.py` and confirm byte-identical
  index CSV output;
- index integrity: row count equals live `fixture_inventory()` length;
  every `witness_anchor_exists=true`; every `redistribution_status`
  equals the crate-recorded value; every `head_suite_run_status` equals
  the captured per-case entry;
- JSON parse of every new/changed `.json` file; CSV parse of the index;
- frozen-surface guard: `git status --porcelain` over
  `validation/witness/**`, `validation/benchmarks/**`,
  `validation/hand_calcs/**`, `fixtures/**` is empty;
- byte-identity of the DEL-09-01 `## Remaining` section against the
  pre-edit text (all four rows);
- `python3 tools/validation/validate_claims_language.py --repo-root .`
  and `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `git diff --check` from `REPO_ROOT`;
- changed-path containment:
  `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
  --base <base-commit> --allowed <each §5 path>` from `REPO_ROOT`,
  persisting JSON stdout to
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T6/CHANGE_SCOPE_CONTAINMENT.json`.

The branch-level registered checks (`piping-pytest`, `evidence-sweep`,
`harness-pytest`, `harness-self-check`) run once at W4 wave closeout,
before any push or fan-in, per the controlling HELP_HUMAN W4 dispatch
(the single evidence sweep must yield exactly one new
`validation/evidence/sweeps/SWEEP_*.json`, committed as the DEC-025
pre-push gate for the branch).

## 7. Defect and Failure Disposition

- Fail closed. Any §3 predicate failure, §6 check failure, missing
  witness anchor, index/source mismatch, frozen-surface drift, or
  unexpected changed path stops closeout; record truthful evidence under
  `instances/W4/T6/` and return to the W4 manager.
- No scope drift: no tolerance/threshold creation, promotion, or policy
  of any kind; no verification→validation promotion; no
  acceptance/release/label effect; no PDU-013 unit-catalog act; no
  `PKG09-0901-PKG02-001` disposition; no PDU-037 row closure; no
  lifecycle/stage act; no code, schema, test, tool, or docs change; no
  DEL-09-02/03/04/05 write.
- A repair need outside the §5 fence — including any defect discovered
  in the suite crate, the runner, or the README mirrors — is reported
  and returned for a new lawful selection, not fixed here.
- Dirty checkout beyond lawful R14 state: stop and return the condition.

## 8. Rerun Triggers

A rerun (new bundle id, new execution record, same governed brief unless
superseded) is required when any of these changes after the
implementation base commit: the mechanics suite crate (inventory,
provenance records, tests), the runner benchmark binding or DEC-065 exit
policy, the README mirrors or hand-calcs inventory, the DEL-09-01
Remaining scope or lifecycle state, applicable DAG-007 rows or the DAG
pointer, or `software-workflow.json`. Completed bundles are never
edited; a superseding bundle cites its predecessor. A material
governance change (a new decision touching the benchmark evidence
surface; supersession of the D-52/D-54 lanes) returns the brief itself
to HELP_HUMAN before any rerun.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- tolerance or threshold promotion, selection, or policy content of any
  kind (DEC-024/DEC-026/DEC-046 gates untouched; every
  `tolerance_policy_state` is recorded as found, `TBD`/unresolved);
- verification→validation promotion, acceptance, release, packaging,
  labeling, CI-gate, or publication effects (DEC-057/DEC-058/DEC-059 and
  PB-TBD-003 untouched);
- the PDU-013 unit-catalog hold (fixture-local unit basis recorded as
  found; no project-grain acceptance act);
- the `PKG09-0901-PKG02-001` disposition or any PDU-060 act
  (`TECHNICALLY_ADDRESSED_PENDING_HUMAN` / `HumanDisposition=TBD`
  preserved);
- closure of PDU-037's provenance-index standing Remaining row: the
  index is BUILT as evidence, the ROW stays open, and closure is judged
  at the owner's gate — the row is not struck at closeout;
- runner/release integration work, `export-results` binding, or any
  solver/schema/runner/suite-crate code change;
- edits to committed witnesses, hand-calcs, README mirrors, reproduction
  bundles, sweeps, coverage, gates, or release artifacts;
- lifecycle transition, stage/milestone advancement, issuance, receipt
  append, push, PR creation/merge, hosted CI, network use, or any
  external commitment;
- professional approval, certification, sealing, authentication, or
  code-compliance claims.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval — Attribution and Effect Record

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (W4 manager, R14 campaign)
RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL
ClassifiedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4 / T6
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected shape per §4)
SelectedOutcome: one immutable derivative benchmark-evidence bundle (whole-suite head capture, extracted family/provenance/redistribution index, verification-refresh record, deterministic bundle-local assembly, hash manifest) under validation/evidence/benchmarks/ per §3–§4 within the §5 fence, with the §16.2 residual row and every owner gate preserved
JudgedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4 / T6
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (crate-level provenance exporter; README-only refresh; cross-suite index; docs/ home; tools/ script; row strike; deferral)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T6/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `instances/W4/T6/VERIFY_BRIEF.md` (C1–C10 confirmed; independent ten-class screen pass; D1 dispatch-provenance defect cured by `instances/W4/W4_DISPATCH_TRANSCRIPT.md`)
EffectStatus: EFFECTIVE — EXECUTION RELEASED BY W4 MANAGER UNDER THE R14 CAMPAIGN CHAIN (VERIFIER COMMIT-SAFE)
PreservedGates: PDU-037 row closure (owner gate); PDU-013 unit-catalog hold; PDU-060 / PKG09-0901-PKG02-001 disposition; DEC-024/DEC-026/DEC-046 tolerance-threshold promotion; verification-to-validation promotion; acceptance/release/label/CI-gate/publication (DEC-057/058/059, PB-TBD-003); runner/release integration; lifecycle/stage/issuance; reproduction acceptance and evidence-posture promotion; prover activation/correlation; merge authority; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the
agent classifies, selects among defensible shapes under D-54, and
proposes. The W4 manager progresses `EffectStatus` only after the
independent refutation returns `COMMIT-SAFE`, under the R14 campaign
plan's execution rules and the durably landed D-52/D-54 records. No
execution is released by this document in its current state.
