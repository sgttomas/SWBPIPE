# RK sealed launch brief — K8 canonical route-A review

Status: SEALED. Run: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`. Instance: `RK`. Parent: `/root` (`HELP_HUMAN`, Agent 0). Execution form: fresh ephemeral Agent 2, delegated-harness-native. Requested runtime configuration: `gpt-5.6-sol`, reasoning `high`, fresh/minimum context. Native execution identity exposed to the run: `/root/canonical_independent_review`. Role and non-delegation are instruction/config asserted where the harness cannot mechanically prove them.

## Purpose and bounded objective

Perform an independent design/code review of 100% of the frozen K8 canonical route-A packet. Return `PASS` or `CHANGES_REQUIRED` with exact actionable findings, review inventory and hashes, validation evidence, limitations, derivative status, blockers/reruns, and next owner. This review supplies evidence to root fan-in; it does not select or adopt a public compatibility/migration design and is not engineering or lifecycle acceptance.

## Governing basis

- Resolve `REPO_ROOT` using `git rev-parse --show-toplevel`; define `WORKING_ROOT={REPO_ROOT}/projects/chirality-piping` and `RUN_ROOT={WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`.
- Source basis: branch `codex/piping-physics-ui-execution-20260908`, source commit `779dedb8670625b36af07b89fc5557470e47c50e`.
- Read root and project `AGENTS.md`; the five run root controls; `amendments/R/R_ROLE_AMENDMENT_V1.md`; and `amendments/R/REVIEW_RELEASE_V1.md`.
- Frozen packet identity: `RUN_ROOT/instances/K8/MANIFEST.json` with required SHA-256 `a6ad4f8ab2881801a2ebabb483e5c22e4cafaa029e07cc398bf420013b576a65`. Review every file bound by that manifest, including DEL-08-04 `_run_records/PHYSICS_UI_EXECUTION_20260908/ROUTE_A_DESIGN.md`, `PUBLIC_COMPATIBILITY_DECISION.md`, the Draft 2020-12 schema, `build_candidate.py`, `SOURCE_KIND_MAPPING.csv`, and `COVERAGE_VALIDATION.json`.
- Additional read-only evidence may include current producer/schema/consumer files and the prior W10 V2 controls needed to establish original semantics. No broad engineering review is authorized.

## Required review questions

1. Verify exact canonical row coverage: 2,348 current rows, omitted rows 9/33/39, and native totals 786/830/813.
2. Verify the complete 45-kind/unit mapping and full equality of every native `source_record` to its source row.
3. Verify that `semantic_status` prevents source-specific/raw transport values from being represented as standardized mechanics.
4. Verify checksum payload versus checksum label distinctions and schema restrictions that discriminate valid variants.
5. Verify fail-closed behavior for unknown schema version and unknown source kind.
6. Verify concrete consumer, ownership, and migration choices while preserving candidate-only public-version/compatibility status.
7. Exercise adversarial cases: duplicate IDs; disagreement between mirrored canonical fields and `source_record`; non-finite numeric values; unsupported units; schema-validation versus builder/code-validation boundaries; and actual legacy consumers.
8. Preserve native pressure meanings pending P5; do not infer or invent accepted pressure semantics. Passing syntactic/coverage checks does not constitute physical approval.

## Tools and permissions

Allowed: read-only filesystem and Git inspection; hashing; CSV/JSON parsing; focused execution of the frozen candidate builder and schema validation/tests using prepared or existing local dependencies; temporary files outside the repository when needed. No network access is needed. No full harness or native build.

Only durable write target: `RUN_ROOT/instances/RK/**`. No source, schema, producer, consumer, deliverable run-record, K8, governance, decomposition/DAG, decision-register, pointer, lifecycle, or historical-evidence edits. Do not delegate or message sibling agents. Do not publish, commit, push, open/merge a PR, or accept findings.

## Required outputs and acceptance checks

- `LAUNCH_BRIEF.md` (this sealed brief), written before substantive packet review.
- `REVIEW_INVENTORY.json` binding the K8 manifest and every reviewed file by SHA-256.
- `VALIDATION.md` describing reproducible focused checks and observed results.
- `RETURN.md` with the bounded verdict, severity-ranked actionable findings with exact evidence, explicit limits, derivative-package status, rerun requirements, blockers, and next owner.
- `STATUS.json` recording terminal status and verdict.

Acceptance requires manifest hash match; 100% manifest-file inventory; direct examination of the requested semantic, schema, validation, and migration properties; focused adversarial execution where feasible; no writes outside `instances/RK/**`; and a verdict that distinguishes design adequacy from public adoption and physical-model acceptance.
