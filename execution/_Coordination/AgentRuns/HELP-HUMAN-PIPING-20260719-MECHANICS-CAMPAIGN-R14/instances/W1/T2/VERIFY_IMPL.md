# VERIFY_IMPL — Fresh-Context Implementation Verification, T2 DEL-04-03 Constant-Effort Solve

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T2
**Role:** Fresh-context implementation verifier (governed Agent 2, non-delegating; adversarial refutation before commit)
**Objects reviewed:** `instances/W1/T2/EXECUTE_RETURN.md` (claims E1–E18); sealed brief `CB-2026-07-19-T2-DEL-04-03-CONSTANT-EFFORT-001` v3 EFFECTIVE (§3 predicates, §5 fence, §6 checks); verifier history v1 BLOCK / v2 BLOCK / v3 COMMIT-SAFE (V3-DEF-1, V3-DEF-2 Low residues); live uncommitted working tree on base `723c95b0f`, branch `claude/piping-r14-pkg04-mechanics` (HEAD confirmed unchanged at `723c95b0f` — the executor made no commit).
**Date:** 2026-07-19. All cargo work offline (`CARGO_NET_OFFLINE=true`, `--offline`). Writes by this verifier: this file only; before/after runner captures and the base-commit rebuild were done in ephemeral scratch outside durable paths.

## 1. Independent Fence-Containment Derivation

Derived directly from `git status --porcelain` and `git diff --stat 723c95b0f` (not from the executor's containment JSON).

Tracked modifications (6):

| Path (relative to WORKING_ROOT) | Fence item |
|---|---|
| `core/product_physics/src/lib.rs` | §5.2 |
| `validation/benchmarks/mechanics/src/lib.rs` | §5.4 |
| `validation/benchmarks/mechanics/README.md` | §5.4 |
| `validation/hand_calcs/mechanics/README.md` | §5.4 |
| `execution/.../DEL-04-03_Linear support and restraint models/_STATUS.md` | §5.5 |
| `execution/.../DEL-04-03_Linear support and restraint models/MEMORY.md` | §5.5 |

Untracked executor writes (4):

| Path | Fence item |
|---|---|
| `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md` (NEW) | §5.3 |
| `execution/.../DEL-04-03.../_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T2_CONSTANT_EFFORT.md` (NEW) | §5.5 |
| `instances/W1/T2/EXECUTE_RETURN.md` (NEW) | §5.6 |
| `instances/W1/T2/CHANGE_SCOPE_CONTAINMENT.json` (NEW) | §5.6 |

Lawful non-executor untracked state set aside (all pre-dating the executor's final writes by mtime): the T2 candidate brief (§5.1, manager-progressed), `CANDIDATE_BRIEF_2026-07-19_T3_DEL-04-01_ARC_PRESSURE_THRUST.md` + `instances/W1/T3/CURRENT_CANDIDATE_RATIONALE.md` (manager-owned T3 planning), `ORCHESTRATION_PLAN.md`, `instances/W3/RETURN.md`, and the pre-existing T2 instance files (`CURRENT_CANDIDATE_RATIONALE.md`, `VERIFY_BRIEF.md`, `VERIFY_BRIEF_V2.md`, `VERIFY_BRIEF_V3.md`).

**No path outside the §5 fence or the lawful set-aside exists in the working tree. Containment independently CONFIRMED (10 executor writes, matching E14 and the persisted containment JSON: status PASS, 10 paths, 0 violations, JSON parses).** No `docs/**`, `core/runner/**`, `core/solver/**`, schema, `tests/**`, or `validation/witness/**` change anywhere in the diff.

## 2. Re-Run Check Results (independent execution)

| # | Check | Result |
|---|---|---|
| 1 | `cargo fmt --check` core/product_physics | PASS |
| 2 | `cargo test --offline` core/product_physics | PASS — 83 passed, 0 failed |
| 3 | `cargo fmt --check` validation/benchmarks/mechanics | PASS |
| 4 | `cargo test --offline` validation/benchmarks/mechanics | PASS — 34 passed, 0 failed |
| 5 | `cargo test --offline` core/runner/headless (T1 envelope-producer path) | PASS — 23 (lib, incl. `result_envelope_binding` producer tests) + 1 + 15 (bin), 0 failed |
| 6 | `git diff --check` (REPO_ROOT) | PASS |
| 7 | `validate_claims_language.py --repo-root .` (REPO_ROOT) | PASS — 262 files scanned, DEC-081 taxonomy satisfied |
| 8 | `validate_path_anchors.py . --text` (REPO_ROOT) | PASS — no literal home-dir absolute paths; 654 surfaces at verification head (see D2) |
| 9 | Two-mirror README check | PASS — both mirrors list `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD`; `git diff --numstat` = `1 0` for each (exactly one added line, zero removed) |
| 10 | JSON parse of new/changed `.json` | PASS — only `CHANGE_SCOPE_CONTAINMENT.json` is new; parses; no project `.json` modified |

## 3. Empirical CLI Refutation Attempt (all reproduced, none refuted)

Runner built offline at the working-tree head; base-commit (`723c95b0f`) runner rebuilt independently in ephemeral scratch via `git archive`.

- **Pinned case** `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json` (`solve --input ... --output ...`): exit **0**, `runner_result.job.state = COMPLETED`, runner diagnostics empty — before AND after. Stdout SHA-256 recomputed from my own runs: before `738d3c074dd90ca97497f2710aac424385e0e85144e93bcee09ba6c2a0151614`, after `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613` — **both exactly match the executor's recorded digests.**
- **Structural before/after diff (full recursive JSON walk): exactly 5 leaves** — (i) `mechanics_envelope/diagnostics` 30→31 with one added element: `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED`, severity `warning`, id `diagnostic:constant-effort-support:support-CE-120:not-consumed`, affected refs `support:CE-120`/`node:N-120`, message naming the unmet condition ("no translational restraint DOF is declared"); (ii)+(iii) `results[828]` and `results[829]` (`constant_effort_user_input_review`) `sign_convention` text updated from the "no global constant-effort load" clause to the truthful opt-in assembled-solve disclosure; (iv) `runner_result/checksums[1]` (envelope checksum) value change. Result-row count 830→830, no row added/removed, top-level `CliOutput` key set identical. **The semantic delta is exactly the claimed three categories; nothing else.**
- **del1005 five cases**: `benchmark_single_case` exit 0, `benchmark_multi_case` exit 0, `benchmark_payload_missing` exit 1, `regression_full_suite` exit 0, `regression_payload_missing` exit 1 — every output **byte-identical** (`cmp`) to its committed witness under `validation/witness/generated/`.

## 4. Per-Claim Verification E1–E18

| Claim | Verdict | Evidence |
|---|---|---|
| E1 | CONFIRMED | `classify_constant_effort_consumption` (lib.rs): errors are exactly MissingConstantLoad / NonPositiveConstantLoad (`positive_finite`) / UnparseableRestraintDof / NoTranslationalRestraintDof / MultipleTranslationalRestraintDofs / UnknownNode; family + no-`nonlinear` guaranteed by the `constant_effort_solve_dispositions` filter (`support.nonlinear.is_none() && is_constant_effort_support`). No other condition exists. |
| E2 | CONFIRMED | Exactly one non-test call site (lib.rs:1153), in `solve_load_case`, immediately after `add_thermal_equivalent_loads` and before `reduce_system`; the same `force` then feeds `reduce_system`, `solve_preview_reduced_system` (dense + sparse parity, which also receives `&force`), `append_sparse_live_path_evidence(&force,…)`, and `append_nonlinear_support_loop_results(…, &force, …)` (lib.rs:1271–1279). Single seam; no double-count path. |
| E3 | CONFIRMED | `build_model` support filter unchanged (lib.rs:3077 `if is_constant_effort_support(support) { return None; }` after the `nonlinear` branch); the diff adds no stiffness/restraint contribution for constant-effort supports anywhere (`add_constant_effort_support_loads` touches only the force vector). |
| E4 | CONFIRMED | `append_constant_effort_consumption_diagnostics` called once per solve at envelope level (lib.rs:819); id/code/severity exactly as claimed (verified in code and in the live pinned output); the only new diagnostic codes in the diff are the two `warning`-severity codes — no new blocking code. |
| E5 | CONFIRMED | `validation.rs` absent from the diff (git status/diff-stat); empirical: removing `constant_load` yields `MODEL_INCOMPLETE` + blocking `CONSTANT_EFFORT_LOAD_MISSING` (test `constant_effort_missing_or_nonpositive_load_keeps_existing_blocking_and_no_defaults`, which also unit-tests both classifier reasons directly). |
| E6 | CONFIRMED | Applied-row basis format string contains `mechanics_consumption=assembled_solve;dec_ref=DEC-049`; `sign_convention: CONSTANT_EFFORT_APPLIED_SIGN_CONVENTION.to_string()`; the constant's full text appears in the hand-calc witness verbatim modulo markdown line-wrapping (whitespace-normalized identity verified programmatically; witness lines 12–18). |
| E7 | CONFIRMED (with qualification D1) | lib.rs contains exactly one occurrence of the clause — the NEGATIVE test assertion `assert!(!…contains("no global constant-effort load"))`. No emitted-row production code carries it. Repo grep otherwise hits: prior run record (2026-06-21), T2 coordination chain (brief, EXECUTE_RETURN, VERIFY_BRIEF v1/v2, rationale), dated reproduction-evidence outputs, and — not named in E7's category list — the committed fallback fixture `fixtures/product_preview/invented_mechanics_result.json` (see D1, Low). |
| E8 | CONFIRMED | `append_constant_effort_support_results` compares `computed.abs() > limit.value` for user-entered `movement_limit`/`travel_range` only (`positive_finite` gate; post-normalization metres); no numeric literal exists in the new non-test production code (only test/fixture/witness invented values). Warning text discloses the no-software-threshold basis; tested exceeded + quiet paths. |
| E9 | CONFIRMED | Empirically reproduced end-to-end, including both SHA-256 digests and the exactly-three-category semantic delta (§3). |
| E10 | CONFIRMED | Five-case byte-identity reproduced (§3); no `validation/witness/**` path in the diff. |
| E11 | CONFIRMED | Full benchmarks diff read: additive `ConstantEffortSupport` variant, fixture fn, solve fn, result struct, one test, inventory registration; sole modification to existing content is `assert_eq!(fixtures.len(), 21)`→`22` (the V3-DEF-1-recorded convention). No existing fixture value, `tolerance_policy` (all `None`), README claim-posture/`TBD`/note text changed; both mirror diffs exactly `1 0`. |
| E12 | CONFIRMED | Fixture test compares via existing `INTERNAL_ASSERTION_EPSILON = 1.0e-9` (lib.rs:52, the recorded DEC-026 analytic-class basis); no new tolerance/threshold/policy JSON in the diff; witness "Tolerance policy: `TBD`" preserves the suite posture. |
| E13 | CONFIRMED | Pinned-capture top-level key sets identical before/after; only envelope-internal values and the envelope checksum differ; no `core/runner/headless`, `core/solver`, schema, or `tests/**` file in the diff. |
| E14 | CONFIRMED | Independent derivation (§1): exactly the 10 claimed writes, all in-fence; remaining untracked files are pre-existing lawful R14 state (mtimes pre-date the executor's writes). |
| E15 | CONFIRMED | `_STATUS.md` git diff: exactly the sole Remaining row removed (section left empty, matching convention), `Last Updated` 2026-07-12→2026-07-19, one new History entry citing the brief/tranche and DEC-049 exclusions, `Current State: IN_PROGRESS` untouched; no other deliverable's state files changed (git status). |
| E16 | CONFIRMED | Reproduction-manual case-1 follow-on present in the run record (2 mentions incl. `headless_runner_reproduction`), the new MEMORY entry, and EXECUTE_RETURN; no `docs/**` change in the diff. |
| E17 | CONFIRMED (within verifiability) | HEAD remains `723c95b0f` on `claude/piping-r14-pkg04-mechanics` — no commit/branch/merge occurred; every result reproduces fully offline in this verification, corroborating the offline claim; no provisioning artifacts observed. The executor's own process network posture is not independently observable beyond this. |
| E18 | CONFIRMED | Eight new `#[test]` fns counted in the diff (superposition identity; direction convention; two-load-case application; non-consuming shapes incl. the empty-restraints pinned shape, `RX`-only, two-DOF, unparseable-DOF, unknown-node; missing/non-positive load; user-limit; nonlinear coexistence + `nonlinear`-field precedence; no-op guard); suite passes 83/83 at head. The superposition unit test asserts the identity against the closed form `F·L³/(3EI)` computed from the model's own section/material, and the benchmark test asserts the witness's five expected values exactly — together covering the identity-plus-values requirement. |

Additional targeted refutation attempts, all failed (i.e., implementation held): witness closed-form arithmetic re-derived by hand (−6000/14400 = −0.416666…; 3000/14400 = 0.208333…; 9000/14400 = 0.625; r_y = −3.0; m_z = −30.0 — all match fixture expressions exactly); the fallback-fixture test (`generated_result_surface_matches_fallback_fixture_force_metadata`) compares only force-row metadata, so the stale fixture text cannot mask a regression; no `pub` item added or existing public signature changed in the product_physics diff (zero `+pub` lines; all new items private), so the public API is unbroken.

## 5. Defects

| ID | Severity | Description |
|---|---|---|
| D1 | Low | E7's grep-category enumeration is incomplete in strict letter: `fixtures/product_preview/invented_mechanics_result.json` (a committed fallback fixture outside the §5 fence, consumed only for force-row metadata comparison) still carries the superseded "no global constant-effort load…" review-row text in fields no test compares. It is a pinned earlier-generation output analogous to the committed witnesses (which the brief likewise forbids editing), so the load-bearing claim — no emitted-row production code carries the clause — is true. Recommend adding this fixture surface to the already-recorded HELP_HUMAN docs-lane follow-on. |
| D2 | Low | EXECUTE_RETURN check 10 records "652 surfaces" for the path-anchors validator "re-run after all durable writes including this file"; at the verification head the count is 654 — exactly the last two instance writes (`EXECUTE_RETURN.md`, `CHANGE_SCOPE_CONTAINMENT.json`), indicating the final recorded count pre-dates those two files. The check itself PASSES at head; the discrepancy is scan-count metadata only, not a governed claim, and no E-claim depends on the number. |

Neither defect changes solver behavior, breaches the fence, falsifies a §3 predicate, or converts authority. The two v3 verifier Low residues (V3-DEF-1 count-assertion fence wording; V3-DEF-2 rationale phrasing) were handled as predicted: the 21→22 bump landed in-fence with suite tests green, and no "fail-closed" misreading reached the implementation.

## 6. Verdict

- Claims: 18/18 CONFIRMED (E7 and E17 with recorded qualifications; none refuted).
- Checks: 10/10 independently re-run PASS; empirical CLI: pinned case and both SHA digests exactly reproduced, semantic delta exactly as claimed, del1005 5/5 byte-identical.
- Containment: independently derived, exactly the 10 in-fence writes.
- Defects: 2 Low (D1 enumeration gap on a stale out-of-fence fixture surface; D2 recorded scan-count metadata), both non-blocking.

VERDICT: COMMIT-SAFE

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
