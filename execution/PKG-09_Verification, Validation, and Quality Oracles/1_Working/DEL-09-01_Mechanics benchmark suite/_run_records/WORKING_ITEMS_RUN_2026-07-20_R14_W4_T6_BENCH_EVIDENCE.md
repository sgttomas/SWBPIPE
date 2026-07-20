# WORKING_ITEMS Run Record — R14 W4 T6 DEL-09-01 Benchmark-Evidence-System Bounded Construction

**Run:** HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W4 / T6
**Date:** 2026-07-20
**Executor:** T6 executor (governed Agent 2, serialized, non-delegating; model claude-fable-5)
**Brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T6_DEL-09-01_BENCH_EVIDENCE_SYSTEM.md` (`CB-2026-07-20-T6-DEL-09-01-BENCH-EVIDENCE-001`, EFFECTIVE, verifier COMMIT-SAFE at `instances/W4/T6/VERIFY_BRIEF.md`)
**Base commit:** `e315fb8406d44dce684cbec091f3174c261efee4` (branch `claude/piping-r14-pkg09-evidence`)
**Bundle:** `validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/` (DERIVATIVE EVIDENCE — NON-AUTHORITATIVE)

## Actions

1. Froze the execution basis: base commit recorded above; working tree clean apart from the lawful R14 W4 state (W4 instance directory, T6 brief, dispatch transcript); re-verified the brief §1 symbol facts live (`fixture_inventory()` 24 fixtures with `assert_eq!(fixtures.len(), 24)`; `BenchmarkProvenance`/`RedistributionStatus::PublicOriginal`; both README mirrors present; runner whole-suite default and `HEADLESS_RUNNER_BENCHMARK_CASE_COMPARISON_BASIS_NOT_REUSABLE` fail-closed blocking with DEC-065 exit policy; no pre-existing `validation/evidence/benchmarks/`; DAG pointer still `DAG-007`; DEL-09-01 `IN_PROGRESS` with the four Remaining rows as quoted). No material drift.
2. Created the bundle and assembled `suite_run_mechanics_input.json` mirroring the committed del1005 benchmark single-case input shape with suite `mechanics`, the `cases` key omitted (whole-suite default), and bundle-local manifest/request identifiers.
3. Ran the offline whole-suite capture (`CARGO_NET_OFFLINE=true cargo run --offline --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark ...`): recorded exit code **1** with 13 `..._CASE_COMPARISON_BASIS_NOT_REUSABLE` blocking diagnostics — fail-closed regression evidence recorded exactly as emitted per DEC-065, not a defect record. Captured stdout and `--output` were byte-identical; the `--output` capture is `SUITE_RUN_MECHANICS.json`. Head tallies: 24 cases, 11 `executed_and_matched`, 0 `executed_and_mismatched`, 13 `blocked`, `whole_suite_default_applied=true`.
4. Wrote the bundle-local, stdlib-only, deterministic `assemble_index.py` and built `FAMILY_PROVENANCE_INDEX.csv`: one row per `fixture_inventory()` fixture (24 rows), every field extracted from the committed crate source, the committed README mirrors, the committed hand-calcs inventory, or the captured run; every `witness_anchor_exists=true`; every `redistribution_status=PublicOriginal` as crate-recorded; per-fixture `head_suite_run_status` (+ blocking code) from the capture; `tolerance_policy_state` recorded as found (`TBD_unresolved_no_tolerance_policy_recorded`).
5. Ran the mechanics suite test refresh offline (**38 passed; 0 failed**) and wrote `TEST_REFRESH.md` (inventory 24 + readiness assertion; mirror consistency; fixture-local unit basis id; R5-era 21-family/33-test reference quoted as history; explicit verification-evidence-only posture).
6. Ran `assemble_index.py` to produce `MANIFEST.json` (bundle id, generated-at as recorded argument, base commit, branch, derivative/non-authoritative label, upstream citations, SHA-256 of every other bundle file, commands, DEC-081 claim posture, and the explicit PDU-037-row-stays-open / PDU-013-hold / owner-gate standing statements); reran it and confirmed byte-identical `FAMILY_PROVENANCE_INDEX.csv` (and `MANIFEST.json`) output.
7. Applied the bounded §3.8 state updates: `_STATUS.md` exactly one new History entry + `Last Updated: 2026-07-20` (Remaining byte-identical, all four rows; `Current State: IN_PROGRESS` unchanged); `MEMORY.md` exactly one new entry (appended newest-last per the file's convention); this run record.

## Check Tally (brief §6, in order)

| # | Check | Outcome |
|---|---|---|
| 1 | `CARGO_NET_OFFLINE=true cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml` | PASS — 38 passed; 0 failed; 0 ignored (read-only) |
| 2 | Whole-suite capture recorded; stdout/`--output` byte-identity | PASS — recorded exit 1 (blocked cases, DEC-065 regression evidence); `cmp` byte-identical |
| 3 | `assemble_index.py` rerun determinism | PASS — index CSV (and manifest) byte-identical on rerun with the recorded arguments |
| 4 | Index integrity (row count = live inventory length; witness anchors exist; redistribution matches crate; head statuses match capture) | PASS — 24 rows; enforced fail-closed inside `assemble_index.py`, which stops on any mismatch |
| 5 | JSON parse of every new/changed `.json`; CSV parse of the index | PASS |
| 6 | Frozen-surface guard: `git status --porcelain` over `validation/witness/**`, `validation/benchmarks/**`, `validation/hand_calcs/**`, `fixtures/**` empty | PASS |
| 7 | DEL-09-01 `## Remaining` byte-identity (all four rows) | PASS — `cmp` against the pre-edit section bytes and `git diff` scoped to the section |
| 8 | `python3 tools/validation/validate_claims_language.py --repo-root .` (from REPO_ROOT) | PASS |
| 9 | `python3 tools/validation/validate_path_anchors.py . --text` (from REPO_ROOT) | FAIL — exactly one finding, in the PRE-EXISTING out-of-fence W4 manager artifact `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/W4_DISPATCH_TRANSCRIPT.md` line 19 (a literal machine-local home-dir path inside the verbatim dispatch quote, written before this tranche as the VERIFY_BRIEF D1 cure). Zero findings in any file written by this tranche (independently re-checked by grep over every tranche write). The cure is outside the §5 fence and belongs to the W4 manager; reported and returned per brief §7, not fixed here |
| 10 | `git diff --check` (from REPO_ROOT) | PASS |
| 11 | Changed-path containment (`validate_change_scope.py --base e315fb840 --allowed <each §5 fence path> --path <each tranche write>`); JSON persisted to `instances/W4/T6/CHANGE_SCOPE_CONTAINMENT.json` | PASS — status PASS, 0 violations |

## Preserved Gates

PDU-037 provenance-index standing Remaining row remains open (index built as evidence only; closure judged at the owner's gate). PDU-013 project-grain unit-catalog hold untouched (fixture-local basis recorded as found). No tolerance/threshold creation, selection, or promotion (DEC-024/DEC-026/DEC-046 untouched); no verification-to-validation promotion; no acceptance, release, label, CI-gate, packaging, or publication effect (DEC-057/DEC-058/DEC-059, PB-TBD-003 untouched); no `PKG09-0901-PKG02-001` / PDU-060 act; no lifecycle or stage change; no code, schema, test, tool, docs, witness, hand-calc, or README-mirror change; no receipt append; no push/PR/merge; no Remaining-row edit anywhere.

## Claim Posture

All artifacts of this run are derivative regression/verification evidence for the committed suite state at the recorded base commit only. Acceptance thresholds, tolerances, CI-gate policy, and release policy remain TBD and owner-gated.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
