# VERIFY_IMPL — T7 DEL-09-04 Reproduction-Manual Stale-Text Refresh

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W4 / T7
**Role:** Fresh-context implementation verifier (governed Agent 2, read-only over product state; this file is the single durable write)
**Object:** the uncommitted T7 working-tree tranche at base `db9197a5dfb250eaf1f454be865b388e75497364` (branch `claude/piping-r14-pkg09-evidence`), executor return `instances/W4/T7/EXECUTE_RETURN.md` (E1–E12)
**Date:** 2026-07-20
**Verdict:** COMMIT-SAFE (0 defects)

All paths relative to `projects/chirality-piping` unless noted. All commands offline (`CARGO_NET_OFFLINE=true`, `--offline`); live-run outputs were ephemeral (session scratchpad only, outside all durable paths). HEAD confirmed at `db9197a5d` before, during, and after verification; the tranche is uncommitted working-tree state as dispatched.

## Independent Re-Run Check Tally

| # | Check (re-run by this verifier) | Outcome |
|---|---|---|
| 1 | Live offline case-1 solve: `cargo run --offline --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- solve --input validation/witness/inputs/tp_runner_015_final_cli_solve_input.json --output <scratch>` | PASS — exit **0**; `runner_result.job.state` `COMPLETED`; `runner_result.diagnostics` `[]`; `request_validation.diagnostics` `[]`; `result_validation.diagnostics` `[]`; `result_refs` length **830**; exactly one `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` diagnostic in the entire output, at `mechanics_envelope.diagnostics[4]` (31 envelope diagnostics total), severity `"warning"`, id `diagnostic:constant-effort-support:support-CE-120:not-consumed`, `affected_refs` `["support:CE-120", "node:N-120"]` |
| 2 | Output digests | PASS — regenerated `--output` file and stdout capture both SHA-256 `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613` (equal to the W1 T2 recorded post-change digest); committed witness `validation/witness/generated/tp_runner_015_final_cli_solve.json` SHA-256 `c406d9c2d8b6e739cd8faf86fcd67ff8f685342f9ee056b5544685a769705188` — no byte-match; witness carries 822 `result_refs` and zero `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` occurrences |
| 3 | Page diff surface count (`git diff` vs `db9197a5d`) | PASS — exactly 3 hunks: corrected case-1 cell; one dated 2026-07-20 Part 1 note; one dated 2026-07-20 Part 2 currency note; frontmatter and every other section untouched |
| 4 | Frozen input anchor | PASS — `supports[6]` = `{id: support:CE-120, family: constant_effort_support, restraints: [], hanger.constant_load: 375 N}`; 7 supports total |
| 5 | Source anchors in `core/product_physics/src/lib.rs` | PASS — `append_constant_effort_consumption_diagnostics` (line 7563) called at envelope assembly (line 839), pushes code `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` severity `"warning"` (line 7574) for the non-consuming shape |
| 6 | T6 bundle anchors | PASS — `SUITE_RUN_MECHANICS.json`: suite `mechanics`, `requested_case_count` 24, 24 case entries, tally exactly {11 `executed_and_matched`, 13 `blocked`}, 0 mismatched, `whole_suite_default_applied` true; `MANIFEST.json`: `recorded_exit_code` 1, label `DERIVATIVE EVIDENCE — NON-AUTHORITATIVE` |
| 7 | Crate readiness assertions | PASS — mechanics `assert_eq!(fixtures.len(), 24)` (line 6057); stress `assert_eq!(fixtures.len(), 15)` (line 1959); nonlinear `assert_eq!(fixtures.len(), 5)` (line 4750) |
| 8 | Three named R14 fixture additions | PASS — `git diff 723c95b0f e315fb840 -- validation/benchmarks/mechanics/README.md` adds exactly `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD`, `MECH-CURVED-BEND-PRESSURE-THRUST-ARC`, `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE`; wave commits `faee4faed`, `6326b2f93`, `a854d43a1` exist with matching subjects |
| 9 | Wave provenance | PASS — `faee4faed` ("constant-effort spring-hanger assembled-solve consumption (R14-W1 T2)") is an ancestor of `581a15b1c` ("Merge pull request #292"), itself an ancestor of HEAD; `loop/LOOP_RECEIPTS.md` Receipt-61 is the R14 wave-1 receipt pointing at tranche commit `faee4faed`; receipt cursor remains Receipt-62; receipts file untouched vs base |
| 10 | Stress/nonlinear currency | PASS — `git log 60841413a..HEAD -- validation/benchmarks/stress validation/benchmarks/nonlinear` empty; five `del1005_payload_binding_*` witnesses byte-identical at head (clean `git status` over `validation/witness/**`) |
| 11 | `_STATUS.md` bounded diff | PASS — `## Remaining` byte-identical to `git show db9197a5d` (section extraction, string-equal; 2 bullets); History bullets 15→16 (exactly one new, newest-first); `Current State: IN_PROGRESS` unchanged; `Last Updated` 2026-07-20; no other line changed |
| 12 | `MEMORY.md` bounded diff | PASS — single purely-additive hunk directly under the file title (newest-first); one new entry recording the two dated notes, the three §2.3–§2.5 no-edit dispositions, and the rerun-consequence restatement; no existing entry modified |
| 13 | Run record | PASS — `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T7_VALMANUAL_STALE_TEXT.md` exists; its recorded live-run evidence (exit, state, 830 refs, warning locus/id, both SHAs), anchors, dispositions, and boundaries match the executor return and this verifier's independent observations; it uses the currency-lapse framing, not "contradicted" (VERIFY_BRIEF D1 heeded) |
| 14 | Frozen-surface guard | PASS — `git status --porcelain` over `validation/witness/**`, `validation/evidence/**`, `fixtures/**`, `validation/benchmarks/**`, `validation/hand_calcs/**` empty (0 lines) |
| 15 | `python3 tools/validation/validate_claims_language.py --repo-root .` (REPO_ROOT) | PASS — 262 files scanned; DEC-081 registry taxonomy satisfied |
| 16 | `python3 tools/validation/validate_path_anchors.py . --text` (REPO_ROOT) | PASS — 691 live path-anchor surfaces, no literal home-dir absolute paths |
| 17 | `git diff --check` (REPO_ROOT) | PASS — clean |
| 18 | JSON parse | PASS — `instances/W4/T7/CHANGE_SCOPE_CONTAINMENT.json` parses; no committed project `.json` changed |
| 19 | Containment, independently re-run: `validate_change_scope.py` vs base `db9197a5d` with the six §5 fence paths | PASS — status PASS, 0 violations; independently re-derived working-tree delta = 3 modified files + the run record + the T7 instance directory (4 files, two of them pre-existing verifier artifacts) + the candidate brief — every path inside the §5 fence; the executor's persisted JSON (PASS, 0 violations, its 6 writes) is consistent with this re-derivation |
| 20 | Non-acts | PASS — HEAD still `db9197a5d` (no commit/branch/merge; tranche uncommitted); `loop/LOOP_RECEIPTS.md` diff vs base empty; `fixtures/product_preview/invented_mechanics_result.json` and `core/product_physics/src/validation.rs` NOT edited (clean status over `fixtures/**` and `core/**`); no other `docs/**` file changed (only the one page in the delta); no scratch or ephemeral file in any durable path |

Tally: 20/20 PASS.

## Per-E-Claim Refutation Table

| Claim | Result | Evidence |
|---|---|---|
| E1 — page diff exactly three surfaces | CONFIRMED | Diff has exactly 3 hunks. (a) The case-1 cell retains all three prior predicates verbatim and appends exactly "; on sources at or after R14-W1 T2, the solve result envelope additionally carries one non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning for `support:CE-120`, per the dated 2026-07-20 note below."; (b) the dated 2026-07-20 note sits between the case-3 note's final paragraph and "The committed generated witness outputs are:" (page lines 68–85 vs 87); (c) the currency note is appended inside "Per-Case Reporting and Fail-Closed Semantics" before "## Rerun Consequence". Frontmatter (`OPS-VALIDATION-MANUAL-HEADLESS-RUNNER-REPRODUCTION`, `draft_evidence`) and all other sections byte-unchanged. |
| E2 — live-run behavior and digest | CONFIRMED | Independently re-run (tally rows 1–2): exit 0; `COMPLETED`; three empty diagnostics arrays; 830 `result_refs`; exactly one warning diagnostic, in `mechanics_envelope.diagnostics` (index 4), severity `warning`, subject `support:CE-120`; SHA-256 `b3cd85af8565…8613` for both stdout and `--output`. |
| E3 — committed witness state | CONFIRMED | SHA-256 `c406d9c2…5188`; 822 `result_refs`; zero `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` occurrences; inside a clean `validation/witness/**` status (byte-identical to `db9197a5d`). |
| E4 — frozen input + emission site | CONFIRMED | Tally rows 4–5: `supports[6]` exactly as claimed (7 supports); envelope-level emission at lines 839/7563/7574 for the empty-restraints non-consuming shape. |
| E5 — Part 2 currency-note anchors | CONFIRMED | Tally rows 6–8: 24 entries, 11+13+0 tally, `requested_case_count` 24; `recorded_exit_code` 1 with derivative/non-authoritative label; `fixtures.len() == 24` at line 6057; README delta between `723c95b0f` and `e315fb840` adds exactly the three named ids. |
| E6 — stress/nonlinear unchanged | CONFIRMED | Empty `git log 60841413a..HEAD` over both suite crates; del1005 witnesses byte-identical at head. |
| E7 — `_STATUS.md` bounded | CONFIRMED | Tally row 11: Remaining string-equal to base (both bullets); History 15→16 newest-first; `IN_PROGRESS` and header form unchanged; `Last Updated` 2026-07-20. |
| E8 — `MEMORY.md` bounded | CONFIRMED | Tally row 12: one purely-additive newest-first entry with the required content; no existing entry touched. |
| E9 — changed-path set and containment | CONFIRMED | Tally row 19: independently re-derived delta matches; executor writes are exactly the 6 listed paths; the candidate brief, `VERIFY_BRIEF.md`, and `CURRENT_CANDIDATE_RATIONALE.md` are pre-existing lawful W4 state (present before execution per the brief's §4.1 record and the VERIFY_BRIEF date); containment PASS, 0 violations, both persisted and re-run. |
| E10 — no out-of-fence file in diff | CONFIRMED | Tally rows 14 and 20: no `validation/**`, `fixtures/**`, `core/**`, `schemas/**`, `tests/**`, `tools/**` change; no other `docs/**` file; `loop/LOOP_RECEIPTS.md` untouched, cursor Receipt-62. |
| E11 — claims calibration | CONFIRMED | Validator PASS (tally row 15); both new notes and the corrected cell read in full: warning consistently framed as review evidence about a non-consuming user-entered data shape ("not a defect, not a solve error, and not a new acceptance criterion"); currency note states "regression evidence under the recorded claim posture … no figure in this note is a release judgment or an acceptance criterion"; TBD owner gates restated; no new tolerance/threshold/acceptance/normative content anywhere in the diff. |
| E12 — process non-acts | CONFIRMED (to the limit of durable evidence) | HEAD unchanged at `db9197a5d`; tranche uncommitted; no new branch state, receipt, sweep, or scratch file in durable paths; the executor's offline/ephemeral process claims are consistent with all durable state and were reproduced offline by this verifier (build cache already warm — `Finished dev profile in 0.07s` — itself consistent with a prior local offline run). |

## Page-Diff Assessment

The full-page read and diff confirm the tranche is exactly the brief's selected shape: one in-place cell correction that removes no predicate and adds one true, live-run-witnessed statement; one Part 1 dated note in strict parallel to the existing case-3 precedent note (same historical framing, same "remains truthful for its pinned commits and is not edited" posture); one Part 2 dated currency note whose every figure traces to the committed T6 bundle, the crate assertions, the README inventory delta, or preserved W1/W2 chain evidence. No asserted value originates in this tranche; every number and identifier in the new text was independently re-derived by this verifier (exit code, state, 830, index 4, severity, `support:CE-120`, `restraints: []`, 375 N, both SHAs, 24/11/13, exit 1, 15, 5, 12/15+3, 5/5, `faee4faed`, `581a15b1c`, PR #292, Receipt-61, `60841413a`, `e315fb840`, the three fixture ids, the bundle path). DEC-081 posture is kept throughout; the routed follow-ons (fallback fixture, `validation.rs` info-text) were recorded, not fixed, exactly as the brief requires.

## Defects

None. All twelve executor claims CONFIRMED; every refutation attempt failed.

## Terminal Verdict

**COMMIT-SAFE**

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
