# ROOT browser discovery for the PR905 repair checkpoint

HELP_HUMAN (ROOT), resumed session of 2026-09-25, before the browser repair was applied. Candidate `5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295` (PR905 head `e65001ad` plus the preserved record and lockfile repairs). Hosted source run 36139859999 had passed accessibility and shard 1, failed five cases in shard 4 and cancelled shards 2 and 3, so the complete failure set was unknown. Environment and exact commands: [_run_records/environment.txt](_run_records/environment.txt).

| Observation | Result | Record |
|---|---|---|
| Local reproduction of the five hosted failures, both projects and all `ui-foundation.spec.ts:677` variants | 10 of 18 failed: the hosted five plus their `chromium-desktop` counterparts and the light-theme 1280x800 variants the cancelled shards never ran | [repro-18-cases-5b1c.log](_run_records/repro-18-cases-5b1c.log) |
| Same 18 cases on current main `aa312755e` in the same environment | 18 of 18 passed, so the failures belong to PR905, not to the local Chromium | [main-aa312755-baseline-18-cases.log](_run_records/main-aa312755-baseline-18-cases.log) |
| Complete source suite on `5b1ccd356`: 499 identities, both projects, one worker | 466 passed, 20 skipped, 13 failed | [full-source-5b1c.log](_run_records/full-source-5b1c.log), [outcomes](_run_records/full-source-5b1c-outcomes.json), [raw JSON (gzip)](_run_records/full-source-5b1c.json.gz) |
| `full-cohort-controller.spec.ts:468`, compact, isolated rerun | Passed; its original failure text was `ENOSPC: no space left on device` during a shared-disk exhaustion window | [cohort468-compact-isolated-rerun-5b1c.log](_run_records/cohort468-compact-isolated-rerun-5b1c.log) |
| Practitioner self-check on a clean `5b1ccd356` checkout | Exit 0 after the clone was unshallowed, as observed by the invoking shell; the log itself records no exit status. Findings: 14 INFO, 1 NOT_APPLICABLE, 4 REVIEW and 124 WARN, none in Piping. The earlier COMMIT_NOT_FOUND receipt findings came from the shallow clone and that output was not retained ([notes](_run_records/pytest-and-self-check-notes.txt)) | [harness-self-check-5b1c-unshallow.log](_run_records/harness-self-check-5b1c-unshallow.log) |

The 12 real failures fall in four spec files and three causes, which the [browser repair](../BROWSER_REPAIR/RETURN.md) diagnoses and repairs:

- A sourced status chip was checked with a helper that assumed title and token are equal: `gui-workflow-validation.spec.ts:107` ×2 and `linear-authoring.spec.ts:60` ×2.
- Assertions written for the old fixture "solve" remained after the honest browser refusal: `r2-smoke.spec.ts:338` ×2 and `r2-smoke.spec.ts:1680` ×2.
- A real narrow-column layout defect let the result-filter summary cover the search input: `ui-foundation.spec.ts:677` at 1280x800 ×4.

No other source spec failed. These are local development observations: local Chromium 1194 is older than the Chromium 1223 bundled with the CI-pinned Playwright 1.60.0. Hosted CI on the frozen candidate remains the required check.

## Later additions (candidate `347e214fe`)

A full piping pytest run on a clean checkout of `347e214fe` ([log](_run_records/piping-pytest-347e214-n3.log), [notes](_run_records/pytest-and-self-check-notes.txt)) found 11 failures that hosted CI does not execute but DEC-025 surface 2 does:

- `test_stress_neutral_physics_source.py::test_method_namespace_removal_substitution_and_relabel_are_rejected` ×2. This PR905-introduced test re-hashed an unknown-method packet outside its `pytest.raises`. The product correctly refuses at materialization (`SN-CSV-PROFILE-MISMATCH`), because the CSV wire policy is method-bound. ROOT's repair asserts that the materializer and the validator (`SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`) each refuse. That strengthens the check; the file passes 63/63.
- `test_headless_runner_contract.py` ×8. These are stale PR905 tests, and ROOT's initial diagnosis was only partly right. Two solve the bundled legacy demo, which PR905 refuses with `PRESSURE_MODEL_REAUTHOR_REQUIRED`. The other six export-results tests load recorded witness inputs whose solver identity is `open_pipe_stress_product_physics@0.1.0`. PR905 raised that crate to 0.2.0, so the runner refuses them with `REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH`. The refusal happens before its hash and producer checks, which masked the expected codes. Both are repaired in [HEADLESS_RUNNER_TESTS](../HEADLESS_RUNNER_TESTS/RETURN.md), with an explicit legacy refusal control and an unlinked-identity refusal control.
- `test_qualification_gate.py::GateTests::test_nonzero_timeout_output_limit_and_malformed_output` ×1. An `input_closed` race under parallel load; it passed when rerun serially.

Corrections to the [browser repair return](../BROWSER_REPAIR/RETURN.md), per the [independent review](../INDEPENDENT_REVIEW/RETURN.md):

- Several of its `file:line` paths are relative to `apps/desktop/`, not `projects/chirality-piping/`.
- Its statement that the DEC-025 sweep runs the dist lane holds only for a host-mode sweep. The CI-bound sweep binds surface 4 to the hosted source run and does not run the dist lane (see [DIST_LANE](../DIST_LANE/RETURN.md)).
