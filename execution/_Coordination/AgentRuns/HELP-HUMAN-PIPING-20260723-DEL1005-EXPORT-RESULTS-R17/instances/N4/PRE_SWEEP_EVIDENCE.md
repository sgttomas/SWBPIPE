---
doc_id: R17-DEL1005-N4-PRE-SWEEP-EVIDENCE
doc_kind: coordination.implementation_evidence
status: pre_sweep_pass
created: 2026-07-23
frozen_git_basis: 1f2ecc1d06375c01a409041b8380e4d65b2a9f9a
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v2
---

# N4 pre-sweep evidence

This is run-local implementation evidence. It is not independent verification,
acceptance, release, issuance, or a `COMMIT-SAFE` decision.

## Implementation result

N4 implemented the adopted v2 boundary:

- the accepted report-package wire DTO/conversion is now a PKG-08 core adapter
  with caller-supplied linked solver identity;
- the desktop native bridge is a thin compatibility wrapper over that adapter;
- only the runner `export-results` stub path is replaced;
- runner success returns exact container bytes once plus bounded identity,
  hash, and member summaries;
- missing payload, cross-binding mismatch, invalid wire input, producer-blocked
  output, and absent local-private intent fail closed without an output file;
- exact `$.report_package` is classified once without scalar descent;
- the other four runner verbs and existing benchmark/regression bindings are
  unchanged.

## Final focused gates

All commands were run from `{REPO_ROOT}` after the final implementation edit.

| Command | Result |
|---|---|
| `cargo fmt --manifest-path projects/chirality-piping/core/reporting/report_package/Cargo.toml -- --check` | PASS, exit 0 |
| `cargo fmt --manifest-path projects/chirality-piping/core/runner/headless/Cargo.toml -- --check` | PASS, exit 0 |
| `cargo test --manifest-path projects/chirality-piping/core/reporting/report_package/Cargo.toml` | PASS, 16 tests, exit 0 |
| `cargo test --manifest-path projects/chirality-piping/core/runner/headless/Cargo.toml` | PASS, 46 tests, exit 0 |
| `python3 -m pytest -q projects/chirality-piping/tests/test_headless_runner_contract.py` | PASS, 15 tests, exit 0 |

The report-package result comprises 4 wire unit tests and 12 unchanged
container-contract tests. The runner result comprises 30 library tests, 1
compatibility-binary test, and 15 final-binary tests.

## Registered union

The final registered invocation was:

```text
python3 tools/software_workflow/run_registered_checks.py \
  projects/chirality-piping/software-workflow.json \
  --check desktop-test \
  --check desktop-build \
  --check piping-pytest \
  --check harness-pytest \
  --check harness-self-check \
  --output projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17/instances/N4/evidence/registered_pre_sweep_final.json \
  --timeout-seconds 1800
```

Result: PASS, exit 0.

- desktop Vitest: 29 files, 516 tests passed;
- desktop production build: PASS;
- piping pytest: 546 passed;
- practitioner-harness pytest: 311 passed;
- repository self-check: PASS under its registered deterministic contract.

Normalized evidence:
`instances/N4/evidence/registered_pre_sweep_final.json`.

## Static and repository gates

| Command | Result |
|---|---|
| `python3 tools/validation/validate_claims_language.py --repo-root .` | PASS; 268 files; DEC-081 taxonomy satisfied |
| `python3 tools/validation/validate_path_anchors.py --json .` | PASS; 972 files; zero findings |
| `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .` | PASS; receipt contract frozen through Receipt-44 |
| Parse every R17 `.json` and `.jsonl` file | PASS |
| `git diff --check` | PASS |
| `git diff --cached --check` | PASS |
| `git diff --cached --name-only` | empty |

Every dirty path is an N4, governed-candidate, or managed-run row in the
adopted 24-row `WRITE_MATRIX.csv`. The ignored runner `Cargo.lock` is the
expressly authorized resolver-only consequence and is not staged or represented
as a tracked output. The three W3-only deliverable closeout rows are untouched.

## Desktop Rust residual

The full desktop Tauri Rust suite reached 74/75. The sole failure is the
frozen-basis, out-of-scope
`validate_rule_pack_command_reports_example_pack_clean_and_draft_findings`
assertion for stale `No professional` notice text. All report-package and
save-path tests passed. The failing test and its governing source are unchanged
from `1f2ecc1d06375c01a409041b8380e4d65b2a9f9a`; no waiver or repair is claimed.
The registered desktop Vitest/build gates and packaged-native regression proof
are green.

## Sweep posture

At this evidence freeze all required pre-sweep gates are green. There are 304
pre-existing `SWEEP_*.json` artifacts. N4 has invoked no R17 DEC-025 sweep yet.
The next and only permitted mutation outside the managed run root is exactly
one registered `evidence-sweep` invocation.
