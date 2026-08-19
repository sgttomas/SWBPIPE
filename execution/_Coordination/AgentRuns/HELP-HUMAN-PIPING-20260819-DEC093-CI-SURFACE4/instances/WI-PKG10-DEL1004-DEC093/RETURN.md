# WORKING_ITEMS package return — WI-PKG10-DEL1004-DEC093

- RunID: `HELP-HUMAN-PIPING-20260819-DEC093-CI-SURFACE4`
- Package: `PKG-10`
- Deliverable coverage: `DEL-10-04`
- Status: `COMPLETE`
- FanInValidity: `VALID`
- Accepted basis: D-65 / `DEC-093`, DAG-009 satisfied upstream state, manager
  launch brief, and exact authority amendments `V2` and `V3`.

## Accepted output

Implemented the DEC-093 CI-bound alternative for DEC-025 surface 4 while
preserving the default host route. Schema-v3 summaries record and strictly
validate the fixed workflow, positive run ID/attempt, matching full head SHA,
successful registered-spec execution, and exact desktop/compact viewport set.
Same-SHA later attempts remain admissible; mismatched, failed, incomplete,
extra-field, partial, dirty, Git-unverified, timestamp-malformed, and
Git-state-contradictory evidence is rejected at the shared validator and both
downstream consumer boundaries.

Changed implementation and deliverable paths:

- `projects/chirality-piping/tools/release/run_evidence_sweep.py`
- `projects/chirality-piping/tools/release/run_release_gate_records.py`
- `projects/chirality-piping/tools/release/package_release_artifact.py`
- `projects/chirality-piping/tests/test_evidence_sweep.py`
- `projects/chirality-piping/tests/test_release_gate_records_script.py`
- `projects/chirality-piping/tests/test_release_packaging_script.py`
- `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_STATUS.md`
- `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/MEMORY.md`
- `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_run_records/WORKING_ITEMS_RUN_2026-08-19_DEC093_CI_SURFACE4.md`
- governed coordination evidence under this run root and instance.

## Fan-in and review

- `TASK-REVIEW-001`: rejected; four findings remediated. V2/V3 cured the exact
  test-path authorization issue.
- `TASK-REVIEW-002`: interrupted without verdict after its hash basis changed;
  no result accepted or reused.
- `TASK-REVIEW-003`: rejected; three downstream integrity findings remediated.
- `TASK-REVIEW-004`: rejected; one dirty-state coherence finding remediated.
- `TASK-REVIEW-005`: accepted terminal `PASS`; 32/32 hashes verified twice,
  100% of the frozen diff reviewed, scope PASS, zero actionable findings, and
  `FanInValidity: VALID`.

The accepted child return is
`children/TASK-REVIEW-005/RETURN.md`; the frozen accepted implementation basis
is `N1_DIFF_MANIFEST_V5.md`.

## Validation

- Evidence sweep: `53 passed`, including all 284 committed schema-v2 summaries.
- Release gate records: `24 passed, 1 deselected`; the deselected test is the
  sole `jsonschema`-dependent case unavailable in the installed Python 3.13
  interpreter.
- Release packaging: `18 passed`.
- Combined selected focused suite: `95 passed, 1 deselected`.
- Full focused suite: `95 passed, 1 failed`; the sole failure says
  `jsonschema` is not importable and requests `requirements-dev.txt`.
- `py_compile` over all six changed Python tool/test modules: PASS.
- `git diff --check`: PASS.
- Deterministic affected-check selection: `evidence-sweep`, `piping-pytest`,
  `harness-pytest`, and `harness-self-check`.

No host-capability command, workflow edit, commit, push, PR, receipt, release,
publication, lifecycle transition, or professional-reliance act occurred.

## Closure state

- Blockers inside this package: none.
- Decisions / waivers / notices: no owner decision or waiver required; V2/V3
  are acknowledged and applied exactly.
- Derivative disposition: DEL-10-04 memory/status/run record and all run-root
  manifests/review records are derivative evidence current to D-65/DEC-093 and
  base `219f695d348f1d83ba904ef4dd38781636b423a6`; they do not replace
  decomposition truth or constitute an immutable acceptance snapshot.
- Runtime summary: child session status/return records are complete; no
  separate telemetry ledger was activated for this bounded work graph, and
  token/context occupancy was not exposed to this manager runtime.
- Rerun requirements and next-owner commands are in `HANDOFF.md`.
- Requested Agent 0 action: accept this package return, integrate through
  CHANGE, run the deferred registered/committed-head checks, and point the one
  tranche receipt at the accepted node and proving runs.

No lifecycle, release, publication, or professional-reliance claim is made.
