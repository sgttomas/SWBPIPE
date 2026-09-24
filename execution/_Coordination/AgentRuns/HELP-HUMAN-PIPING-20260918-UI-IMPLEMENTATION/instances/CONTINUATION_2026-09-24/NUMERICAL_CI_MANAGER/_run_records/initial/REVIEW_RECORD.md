# M27 hosted numerical CI

Status: implementation and local policy/negative verification complete. The
fresh independent review outcome is recorded in adjacent `review.md` and
`review-basis.json`. No full product numerical or hosted execution is claimed
by this record.

ROOT assigned the isolated `codex/piping-numerical-ci-20260924` checkout at
`f7e8b467cb2db244f11fe49cede636140031b387`. The change adds Rust coverage to
the existing Piping source workflow. Root retains Git integration, global
candidate checks, hosted execution and the undertaking graph.

The implementation preserves browser selection, accessibility barrier, four
full-source remainder shards and stable aggregate semantics, adding an independent
numerical requirement. Rust discovery consumes the existing project release
readiness cargo profile. The job fetches locked dependency graphs before offline
locked tests, with the existing 1.97.1 Rust pin and no browser provisioning.

The dependency audit includes fixtures, schemas, examples/rule_packs,
validation/hand_calcs and the retained frozen mechanics JSON consumed by
`core/runner/headless/src/benchmark_binding.rs`. That evidence input selects
numerical coverage even though it does not select browser coverage.

## Checks and evidence

- 44 focused Python unit tests passed; see `python-tests.log` and the frozen
  implementation hashes. These checks exercise selection, discovery, locked
  fetch/offline commands, candidate rejection and runner/aggregate failures.
  They also scan actual Rust include literals and runtime resource references
  so newly consumed evidence cannot silently inherit an exclusion.
- `git diff --check` passed. Canonical tranche `validate_manifest` checked the
  actual working-tree YAML with no failures using the existing DEC-025 Python
  environment. This is schema validation, not committed-range G4 coverage.
- Readiness discovers 38 crate graphs. Sixteen lacked committed lockfiles;
  ROOT authorized their offline generation with Rust 1.97.1. Crate-local
  ignore rules cover all sixteen; ROOT must force-add exactly the declared
  lock paths, with no ignore-file amendment. See
  `BASELINE_MISSING_LOCKS.json`, `LOCK_GENERATION.json` and
  `LOCK_VERSION_COMPARISON.json`. No manifests or existing locks changed.
  New cached resolutions include cfg-if 1.0.5, syn 3.0.6 and unicode-ident
  1.0.26; no unrelated graph version alignment was performed.
- The real expected-negative control compiled and ran one deliberately failing
  dependency-free Rust test (`4 != 5`). The actual runner CLI returned 101 and
  the aggregate returned 1. `negative-control/control.json` binds the exact
  production helper bytes, temporary fixture commit, command arguments and
  test inputs; its adjacent raw logs show the failed assertion and toolchain.
  The aggregate uses controlled browser result inputs and claims no browser
  execution. Script exit 0 means these expected failure assertions held.
- The negative control used its own temporary target and one Cargo build job,
  with ROOT-authorized bounded overlap with the solver's isolated jobs2 checks.
  Neither this check nor the metadata generation supports timing claims.

To repeat the negative control, use `negative-control/rerun.py` after adjusting
its `repo` assignment to the candidate checkout and obtaining the current
coordinator's Cargo resource slot. It copies the actual runner bytes into a
temporary Git fixture, performs candidate validation, executes locked fetch
and offline test through the CLI, and checks the expected nonzero exits.

## Limits and return

The independent review supplies fan-in advice; ROOT must run the actual product candidate's
required checks, committed-range G4 and hosted workflow and reconcile any
solver-branch manifest/lock integration before merge. Static workflow inspection
and the controlled local failure are not hosted execution, engineering
acceptance, practitioner evidence or release authorization.
