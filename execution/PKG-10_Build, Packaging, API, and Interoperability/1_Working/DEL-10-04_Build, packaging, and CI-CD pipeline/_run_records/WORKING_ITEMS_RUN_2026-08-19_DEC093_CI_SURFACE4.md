---
doc_id: WI-RUN-DEL-10-04-2026-08-19-DEC093-CI-SURFACE4
doc_kind: execution.run_record
status: complete
created: 2026-08-19
deliverable_id: DEL-10-04
package_id: PKG-10
decision_basis: [D-65, DEC-093]
---

# DEC-093 CI-bound surface-4 tooling

Implemented the ruled CI alternative for DEC-025 surface 4. The sweep accepts
`--surface4-ci-binding <json>` and records schema v3 evidence for the fixed
workflow path, positive Actions run ID and run attempt, exact head SHA,
successful conclusion, affirmative registered-spec execution, and the exact
two registered viewport projects. The head SHA is checked against the sweep
commit both before surface 1 and during summary validation; the summary writer
and release-gate consumer reject invalid CI bindings. Same-SHA successful rerun
attempts remain bindable.

The host path is unchanged and remains the default: surface 4 still records
`execution_capability: host` and runs `test:e2e:desktop` followed by
`test:e2e:dist:desktop`. No workflow, historical sweep artifact, lifecycle,
release, publication, or professional-reliance surface changed.

The first independent review rejected the initial fan-in. Agent 0 amendments
V2 and V3 explicitly authorized the three canonical focused test files. The
manager then closed the remaining findings by structurally validating the
entire committed schema-v2 corpus, preventing partial summaries from being
selected as full DEC-025 evidence, and requiring the same valid/complete sweep
contract before release packaging can mark its authenticity chain verified.
The rejected review remains preserved under the run instance. A later 100%
frozen-diff review found three additional downstream integrity gaps; the final
state rejects Git-unverified sweep state in packaging, validates summaries
before release-gate Git dereference, and requires a parseable UTC
`started_utc` before declaring a summary complete or sorting it. The remediated
diff then received a further review that found contradictory clean flags with
non-empty dirty paths were still structurally admitted. The shared validator
now requires `working_tree_dirty == bool(dirty_paths)` when capture succeeds
and an empty path list when capture fails. This final state is subject to a
separately instantiated terminal read-only review.

## Focused validation

- `python3 -m pytest -q tests/test_evidence_sweep.py`: PASS, 53 tests; all 284
  committed schema-v2 summaries validated.
- `python3 -m pytest -q tests/test_release_gate_records_script.py -k 'not records_validate_against_their_own_schema'`: PASS, 24 tests; one case
  deselected because the available Python 3.13 environment lacks `jsonschema`.
- `python3 -m pytest -q tests/test_release_packaging_script.py`: PASS, 18 tests.
- Combined focused run: 95 passed, with the sole full-run failure reporting only
  that `jsonschema` is not importable in that interpreter.
- `python3 -m py_compile` over both changed tools and test modules: PASS.
- `git diff --check`: PASS.

Independent read-only review evidence is persisted under the governing
AgentRuns instance. Committed-head DEC-025 execution, repository self-check,
full practitioner-harness checks, commit, push, PR, receipt, and merge remain
outside this bounded manager run.
