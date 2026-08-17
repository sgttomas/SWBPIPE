# WORKING_ITEMS MANAGER RETURN — N1

## Verdict

`N1` is implemented, resolved, and accepted. HELP_HUMAN completed the exact-pin
environment rerun, so the candidate is fully `PASS` and `CHANGE_READY`.

## Product Result

- Dirty runs with no `--output-dir` now route the summary to the system
  temporary `openpipestress-dirty-sweeps/` directory and print why canonical
  evidence output is disabled.
- Clean runs retain the canonical
  `validation/evidence/sweeps/` default.
- `--allow-dirty-canonical-output` is the explicit opt-in for an intentional
  dirty canonical summary.
- An explicit dirty canonical `--output-dir` without that opt-in fails with a
  clear message and writes no summary.
- Explicit non-canonical `--output-dir` behavior is preserved for dirty runs.
- Unverified Git state also defaults outside canonical evidence and cannot be
  explicitly written there.
- Summary schema remains version 2; no summary fields changed.

## Validation

- Focused `tests/test_evidence_sweep.py`: `38 passed in 3.80s`.
- `git diff --check`: pass.
- AgentRuns JSON parse: pass.
- Repository practitioner-harness `self-check`: exit 0; existing review/warn
  findings only, with no new finding attributable to this tranche.
- Full practitioner-harness pytest: `349 passed in 16.70s`.
- Full Piping pytest: blocked during collection because `jsonschema` is not
  installed in the available Python 3.13 runtime (five collection errors).
  A follow-up run excluding those five collection modules reached
  `487 passed, 17 skipped, 30 failed`; the failures were downstream schema or
  validation behavior under the same missing dependency. No repository
  dependency or environment mutation was attempted.
- HELP_HUMAN fan-in rerun in the existing exact-pin Python environment:
  focused evidence-sweep tests
  `38 passed in 2.53s`; full Piping pytest `576 passed in 15.49s`.
- Fan-in verdict: `PASS`; accepted graph: `WORK_GRAPH_V2.json`.

## Containment

Modified product paths are only:

- `projects/chirality-piping/tools/release/run_evidence_sweep.py`
- `projects/chirality-piping/tests/test_evidence_sweep.py`

Coordination writes are confined to this named AgentRuns package. Historical
sweep evidence, receipts, registers, deliverable state, and foreign paths are
unchanged. No commit, push, PR, or merge was performed.

## Closeout

No rerun or owner decision remains. Route the accepted candidate to CHANGE for
Git closeout.
