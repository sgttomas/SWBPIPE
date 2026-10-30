# TP-VERIFY-017 Release-Readiness Gap Closeout

## 1. Boundary

This artifact records evidence-only fan-in for TP-VERIFY-017 remediation of the
three gaps recorded by `TP_VERIFY_016_INTEGRATED_RELEASE_READINESS_SWEEP.md`.

This closeout does not change lifecycle state, promote candidate rows, close a
release gate, authorize a release, approve waivers, accept professional
reliance, certify code compliance, or create a professional approval record.

## 2. Baseline

Baseline evidence:

- `TP_VERIFY_016_INTEGRATED_RELEASE_READINESS_SWEEP.md`
- DEL-09-05 `MEMORY.md` TP-VERIFY-016 addendum
- `DAG-005` as approved active graph authority
- Current DEV-001 coordination derivatives

TP-VERIFY-016 recorded three gaps:

| Gap | Baseline finding | TP-VERIFY-017 disposition |
|---|---|---|
| TP-VERIFY-016-GAP-001 | `validation.witness.tools` was not importable from `tests/test_calculation_witness.py`. | Remediated by restoring `validation/witness/tools/**`; focused witness tests pass. |
| TP-VERIFY-016-GAP-002 | DEL-12-04 security wording gate rejected the phrase `real secret`. | Remediated by replacing only the forbidden DEL-12-04 memory wording with `credential-value storage`; security tests pass. |
| TP-VERIFY-016-GAP-003 | Release readiness invoked direct pytest collection under `tools/coordination`, which collected no tests. | Remediated by changing the command to `tests/test_coordination_maintenance.py` and locking it with focused script tests. |

## 3. Changes

DEL-09-02 witness tooling:

- Added `validation/witness/tools/__init__.py`.
- Added `validation/witness/tools/witness_validator.py`.
- Added DEL-09-02 run record:
  `_run_records/TASK_RUN_2026-05-31_TP-VERIFY-017_DEL-09-02.md`.
- Updated DEL-09-02 `MEMORY.md`.

DEL-12-04 security wording:

- Updated only DEL-12-04 `MEMORY.md`, replacing `real secret storage` with
  `credential-value storage`.

DEL-10-04 release-readiness command surface:

- Updated `tools/release/check_release_readiness.py` so the coordination
  regression step uses `tests/test_coordination_maintenance.py`.
- Updated `tests/test_release_readiness_script.py` to assert the intended
  command for `python` and `all` profiles.
- Added DEL-10-04 run record:
  `_run_records/TASK_RUN_2026-05-31_TP-VERIFY-017_DEL-10-04.md`.
- Updated DEL-10-04 `MEMORY.md`.

## 4. Validation Evidence

Passing focused evidence:

| Check | Result |
|---|---|
| `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check` | PASS before focused validation. |
| `python3 validation/witness/tools/witness_validator.py --write-generated --check-generated` | PASS. |
| `python3 -m pytest -q tests/test_calculation_witness.py` | PASS; 9 tests. |
| `python3 -m pytest -q tests/security/test_secret_private_library_handling.py` | PASS; 7 tests. |
| `python3 -m pytest -q tests/test_release_readiness_script.py tests/test_coordination_maintenance.py` | PASS; 9 tests. |
| `python3 tools/release/check_release_readiness.py --profile security --execute` | PASS; 31 tests. |
| `git diff --check` | PASS. |

Integrated readiness evidence:

| Check | Result |
|---|---|
| `python3 tools/release/check_release_readiness.py --profile python --execute` | FAIL; Python contract suite now reaches `tests/test_invented_example_models.py`, where 267 tests pass and 2 fail. |
| `python3 tools/release/check_release_readiness.py --profile all --execute` | FAIL at the same Python contract step before security and Cargo profile execution. |

## 5. Newly Exposed Residual Gap

TP-VERIFY-017 resolved the three TP-VERIFY-016 gaps, but integrated
release-readiness is still not closed.

New residual finding:

| Gap ID | Finding | Evidence | Disposition |
|---|---|---|---|
| TP-VERIFY-017-RESIDUAL-001 | `examples/models/invented/fake_rule_pack_toy_model.json` no longer validates against the current full model schema and persistence envelope path. | `tests/test_invented_example_models.py::test_invented_examples_validate_against_full_model_schema` and `tests/test_invented_example_models.py::test_invented_examples_persist_and_round_trip_with_schema_validation` fail. The schema errors report missing `local_coordinate_system.y_reference` in both fake-rule models and a load record that is not valid under the current load schemas. | Out of TP-VERIFY-017 write scope. Candidate next tranche should be owned by DEL-11-04 / schema-example alignment, with review against public invented-example and IP/data-boundary rules. |

## 6. Closeout

TP-VERIFY-017 closes the three TP-VERIFY-016 command/evidence gaps as bounded
remediation evidence. It does not support a passing integrated
release-readiness profile because of `TP-VERIFY-017-RESIDUAL-001`.

Lifecycle state, candidate rows, blocker queues, dependency records,
implementation evidence rows, release records, acceptance records, CI
workflows, release automation, signing, attestation, professional-boundary
decisions, code-compliance decisions, and release claims were not changed.
