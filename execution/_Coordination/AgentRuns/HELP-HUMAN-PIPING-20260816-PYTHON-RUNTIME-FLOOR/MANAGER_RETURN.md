# WORKING_ITEMS Return — Python Runtime Floor

## Result

- Manager result: `PASS`
- Objective result: `TM-PIP-027_IMPLEMENTED`
- Git disposition: `CHANGE_READY_NOT_STAGED`
- Source/base commit: `65735390590e500dbbea6b63a4a79ba42944bf6d`
- Branch: `codex/piping-python-runtime-floor`
- Sweep schema: unchanged at `2`

The DEC-025 entrypoint now requires Python 3.11 or newer immediately after
argument/root validation and before plan construction, tool discovery,
prerequisite probing, or surface execution. A rejected runtime reports both
the current and required versions and confirms that no prerequisite probe or
evidence surface ran. The four direct development dependencies are exact
pins: `jsonschema==4.26.0`, `coverage==7.15.3`, `pyyaml==6.0.3`, and
`pytest-xdist==3.8.0`.

## Product Diff

- `requirements-dev.txt`: four compatible ranges replaced by four exact pins.
- `tools/release/run_evidence_sweep.py`: Python floor constant, current-version
  capture, testable preflight result, and main-entry fail-fast handling.
- `tests/test_evidence_sweep.py`: accepted-minimum/current-runtime regression,
  rejected-3.10.14 fail-fast regression proving no downstream probe/surface
  function ran, and exact-pin contract regression.
- No build/release documentation change was needed because the executable
  preflight carries the user-facing current/required version message.

## Validation

- Focused sweep tests: `34 passed in 2.58s`.
- Real Apple Python `3.9.6` invocation: exit `1` with the explicit current /
  required message and `no prerequisite probe or evidence surface ran`.
- Full Piping pytest under the exact-pin Python 3.13 environment:
  `572 passed in 5.94s`.
- Exact installed-pin check: all four names/versions matched; `PASS`.
- Redirected-target sandbox-only DEC-025 sweep: `PASS`.
  - `CARGO_TARGET_DIR` was a fresh task-local temporary `cargo-target`
    directory outside the repository.
  - Cargo: all 36 discovered crate checks passed.
  - Python surface: `572 passed in 15.69s`.
  - Desktop Vitest: `29` files / `523` tests passed.
  - Desktop production build: passed.
  - Playwright was neither required, probed, nor run.
  - Summary:
    `validation/evidence/sweeps/SWEEP_20260816T220825Z_65735390590e-dirty.json`;
    schema `2`, four sandboxed surfaces `pass`, overall `pass`, explicitly
    partial (`complete_dec025_sweep=false`).
- Practitioner harness: `349 passed in 16.87s`.
- Repo self-check: exit `0`; its pre-existing review/warn findings remain
  informational and outside this write fence.
- Receipt validator, claims-language validator (`269` files), path-anchor
  validator (`1361` surfaces), candidate-whitespace validator, Python compile,
  JSON parse checks, and `git diff --check`: passed.

## Environmental Checks and Deferred Gate

- A fresh isolated `pip install --no-index -r requirements-dev.txt` was
  attempted and correctly failed because the sandbox exposes no usable pip
  cache (`jsonschema==4.26.0` had no local distribution). No install, network,
  or escalation was attempted afterward.
- The project-directory bare `python3` resolves to Apple Python `3.9.6` and
  lacks pytest. The full suite was therefore rerun with the existing isolated
  Python `3.13.14` environment containing exactly the four pinned versions.
- The full five-surface clean-commit host gate is intentionally deferred to
  CHANGE. From the project root after the scoped commit, in host context, run:

```sh
CARGO_TARGET_DIR="${TMPDIR%/}/chirality-python-runtime-floor-host-target" \
python3 tools/release/run_evidence_sweep.py --execute \
  --require-capability host \
  --output-dir "${TMPDIR%/}/chirality-python-runtime-floor-host-sweep"
```

Run that command from an activated Python `>=3.11` environment containing the
four exact pins, with `wasm-bindgen 0.2.123` and the host browser prerequisites
on `PATH`. The temporary output location preserves the activation's
one-repository-sweep artifact fence. Any different retained-artifact
disposition must come from the parent authority.

## Path Containment

The candidate contains exactly three product files, this named AgentRuns
package, and one sweep JSON. There are no `core/**`, desktop product source,
documentation, register, DAG, policy, lifecycle, foreign-loop, receipt, or
other deliverable-state changes.

## Decisions and Handoff

- Parked owner decisions: none.
- Requested next action: CHANGE stages only the contained paths, commits the
  scoped tranche, runs the exact host gate above from clean committed state,
  and performs the authorized push/PR/receipt closeout without merging.

## Model Attribution

WORKING_ITEMS used the inherited Codex GPT-5 capability; the exact runtime
model identifier was not exposed. No Agent 2 ran, and no model substitution
occurred.
