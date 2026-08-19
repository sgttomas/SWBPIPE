# Handoff — WI-PKG10-DEL1004-DEC093

- Accepted upstream basis: D-65 / `DEC-093`; DAG-009 prerequisites satisfied.
- Accepted integrated basis: `N1_DIFF_MANIFEST_V5.md` plus terminal PASS at
  `children/TASK-REVIEW-005/RETURN.md`.
- Closure verdict: package implementation and in-session review gate complete;
  governing tranche closeout remains outstanding.
- Remaining blockers: none inside the package. The current Python 3.13 test
  environment lacks `jsonschema`, so one focused schema-validation test needs a
  requirements-complete environment.
- Derivative packages: DEL-10-04 status/memory/run record and this run-root
  evidence are current derivative records; no decomposition snapshot or loop
  receipt was written.

## Rerun and governing closeout

From `projects/chirality-piping`, with the development requirements available:

```text
python3 -m pytest -q tests/test_evidence_sweep.py tests/test_release_gate_records_script.py tests/test_release_packaging_script.py
python3 -m pytest -q tests -n auto --dist loadscope
python3 tools/release/run_evidence_sweep.py --execute
```

The last command is host-capability work when the default host surface is used.
For the ruled CI alternative, after a successful Actions run is bound to the
exact committed SHA, supply the closed DEC-093 JSON and run:

```text
python3 tools/release/run_evidence_sweep.py --execute --surface4-ci-binding <binding.json> --require-capability ci
```

From repository root:

```text
python3 -m pytest -q tools/practitioner_harness
python3 tools/practitioner_harness/harness.py self-check
```

After these pass on the committed integrated state, CHANGE may commit/push/open
the tranche PR and Agent 0 may write the single fan-in receipt. Any change to a
reviewed implementation or deliverable file invalidates the V5 hash basis and
requires focused reruns plus fresh integrated-diff review.
