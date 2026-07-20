---
doc_id: WORKING-ITEMS-RUN-2026-07-20-R14-W5-CLEAN-REPRO
doc_kind: execution.run_record
date: 2026-07-20
deliverable_id: DEL-09-04
package_id: PKG-09
managed_run: HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14
wave: W5
status: complete
---

# Run Record — DEL-09-04 Clean-Checkout Reproduction at the R14 Head (R14-W5)

## Basis

- Controlling brief: `CB-2026-07-20-DEL-09-04-CLEAN-REPRO-R14-001`
  (`execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_DEL-09-04_CLEAN_REPRO_R14.md`,
  status `EFFECTIVE — EXECUTION RELEASED`; verifier `COMMIT-SAFE` at
  `instances/W5/VERIFY_BRIEF.md`), SHA-256 at its effective state
  `702b56012c3d4afe7fe64a1dea4f703beb23571a9abe18f796282ada527539d4`.
- Adoption: owner standing approval D-52/`DEC-085` as refined by
  D-54/`DEC-087`, SHA-bound at governance commit
  `f14fa77518a06f112ae72a8fcce4de0fab958d47`. Decision basis: DEC-080,
  DEC-085, DEC-087.
- `SOURCE_COMMIT=a5235340aae3c41cf227f5617e593b268936f6b3` (branch
  `claude/piping-r14-w5-clean-repro`; HEAD verified equal; primary tree
  clean apart from lawful untracked W5 state).
- Procedure: `docs/validation_manual/headless_runner_reproduction.md`,
  SHA-256 `fa714cf44d5c3e8a54ff6e2f6883676b81e01755e2e07d36a5bd118576b299c1`
  (verified at the source commit).
- Profile: `software-workflow.json`, SHA-256
  `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f`.

## RUN_ID and bundle

- `RUN_ID=REPRO_DEL0904_20260720T074714Z_a5235340aae3` (frozen before the
  first durable write).
- Bundle:
  `validation/evidence/reproduction/REPRO_DEL0904_20260720T074714Z_a5235340aae3/`
  — new, immutable, `overall_status: PASS`, evidence label
  `INTERNALLY_VERIFIED`; `SHA256SUMS.txt` computed last and verified.

## Command tally

17 recorded commands (`commands.jsonl`; each with separately captured argv,
cwd, env overrides, stdout, stderr, actual exit; no shell chains), all
matching expected exits:

| Group | Commands | Exits (observed = expected) |
|---|---|---|
| Part 1 generator + scoped determinism diff | cmd01, cmd02 | 0, 0 |
| Part 1 runner cases (solve / validate-input / run-benchmark) | cmd03-cmd05 | 0, 1, 1 |
| Part 2 generator + scoped determinism diff | cmd06, cmd07 | 0, 0 |
| Part 2 bound runner cases | cmd08-cmd12 | 0, 0, 0, 1, 1 |
| Review checks (`cargo test --offline`, contract test) | cmd13, cmd14 | 0, 0 |
| Post-run cleanliness | cmd15-cmd17 | 0, 0, 0 (empty porcelain) |

Pre-run cleanliness proofs recorded in `git-status-before.txt` (HEAD equal
to `SOURCE_COMMIT`; empty porcelain incl. untracked; clean diff and cached
diff). Offline posture: `CARGO_NET_OFFLINE=true`, `--offline`, temporary
`CARGO_TARGET_DIR` outside the checkout; local-only
`git clone --no-hardlinks --no-checkout`.

## Predicate verdicts

| Predicate | Verdict | Observed |
|---|---|---|
| P1 | PASS | generator exit 0; three `tp_runner_015` inputs byte-identical (scoped diff clean) |
| P2 | PASS | exit 0; `COMPLETED`; empty req/res validation diagnostics; 830 `result_refs`; exactly one `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` in the document, at `mechanics_envelope.diagnostics[4]`, severity `warning`, affected `support:CE-120` (+ `node:N-120`) |
| P3 | PASS | regenerated `b3cd85af8565…` ≠ committed witness `c406d9c2d8b6…` (expected mismatch; equals the W4 T7 live-head digest) |
| P4 | PASS | exit 1; `HEADLESS_RUNNER_LOAD_BASIS_MISSING` at `request_validation.diagnostics[0]`; no solver result (observation: bytes `5fb2f8a9c826…` ≠ witness `0d707ee26f4b…`) |
| P5 | PASS | exit 1; `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` (post-#287); expected byte mismatch vs pre-#287 stub witness recorded (regenerated `9596c052c76a…`) |
| P6 | PASS | generator exit 0; five `del1005` inputs byte-identical (scoped diff clean) |
| P7 | PASS | mechanics; `requested_case_count` 1; `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` executed_and_matched 1/1; diagnostics []; byte-identical to witness `813a702b74be…` |
| P8 | PASS | stress; 3/3 executed_and_matched (`STRESS-AXIAL-NORMAL-ORIGINAL`, `STRESS-RANGE-MECHANICS-ORIGINAL`, `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS`); diagnostics []; byte-identical to witness `8feb3d25e50e…` |
| P9 | PASS | nonlinear; `whole_suite_default_applied` true; 5/5 executed_and_matched; diagnostics []; byte-identical to witness `2f89adce9e4d…` |
| P10 | PASS | exit 1; `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`; byte-identical to witness `9596c052c76a…` |
| P11 | PASS | exit 1; `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`; byte-identical to witness `61cba4f28bcf…` |
| P12 | PASS | `cargo test --offline` exit 0; `python3 tests/test_headless_runner_contract.py` exit 0 (inside the clone) |
| P13 | PASS | clone local-only, detached at `SOURCE_COMMIT`; empty porcelain (untracked-files=all) and clean diffs before first and after last command |
| P14 | PASS | bundle complete per brief §5.2; `SHA256SUMS.txt` verified; `INTERNALLY_VERIFIED` on PASS only; adoption basis and no-acceptance statement present; no `PROVER_CORRELATED`/`ENGINEER_ACCEPTED` |
| P15 | PASS | `## Remaining` byte-identical pre/post (SHA-256 `20dfd8d99094…` both sides); exactly one new History entry; `Last Updated` 2026-07-20; `IN_PROGRESS` unchanged; one new MEMORY entry (newest-first); this run record; no receipt append |
| P16 | PASS | §6 checks all pass (see check tally) |

## Check tally (§6, executor share)

- evidence-sweep via `tools/software_workflow/run_registered_checks.py`:
  PASS; before/after snapshot delta exactly one new file —
  `SWEEP_20260720T075521Z_a5235340aae3-dirty.json` — output at
  `checks/evidence-sweep.json`.
- `validate_claims_language.py --repo-root .`: PASS.
- `validate_path_anchors.py . --text`: PASS.
- `validate_piping_loop_receipts.py --repo-root .`: PASS (receipts file
  untouched; cursor Receipt-63).
- `git diff --check`: clean. JSON/JSONL parse of every new `.json`/`.jsonl`
  file: PASS.
- Changed-path containment
  (`tools/software_workflow/validate_change_scope.py --base a5235340aae3…`
  with exactly the seven brief §6 `--allowed` entries): PASS — output at
  `checks/change-scope.json`.

Manager closeout checks (`piping-pytest`, `harness-pytest`,
`harness-self-check`) remain with the W5 manager per the brief §6 division
of labor.

## Contingency record

§4.4 bounded environment contingency: **NOT USED**. No offline cargo
failure of the named gitignored-lockfile/ignored-build-state class occurred;
all cargo commands compiled from the already-present local cache (42 crates)
with no copy, no install, no network. R12-ENVREPAIR-01 was not invoked.

## Closeout

- The temporary mktemp root (clone, target dir, outputs, logs) was removed
  after its evidence was copied and hashed into the bundle; only the
  validated task-created directory was deleted.
- Non-acts: no Remaining strike; no lifecycle or target-stage change; no
  owner acceptance of the reproduction result (owner gate); no evidence
  posture promotion; no receipt append; no commit/push/branch act by the
  executor (the W5 manager commits); no code, fixture, witness, schema,
  test, tool, or manual edit.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
