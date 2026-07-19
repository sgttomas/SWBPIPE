# ORCHESTRATOR Plan — DEL-09-04 Prerequisite Provision P1

## Authority and Purpose

HELP_HUMAN delegated this separate prerequisite-provisioning tranche to
ORCHESTRATOR after the immutable R7 reproduction closed `BLOCKED`.

The current-session owner direction sequence is recorded verbatim:

1. `can't you just download the dependencies you need?`
2. `override`

HELP_HUMAN interprets that sequence narrowly as authorization to provision the
missing locked dependencies outside the sealed reproduction run. It does not
amend, reopen, reuse, or reinterpret R7, and it does not authorize reproduction
execution or acceptance, lifecycle advancement, evidence promotion, release,
publication, or any external effect beyond dependency-registry and Playwright
browser-artifact access required by this tranche.

## Frozen Baseline

- repository root: `/Users/ryan/.codex/worktrees/2faf/chirality`
- working root: `/Users/ryan/.codex/worktrees/2faf/chirality/projects/chirality-piping`
- branch: `codex/piping-del0904-clean-repro-20260718-r7`
- baseline HEAD: `9843875fcc0fb5479ad74a0a9f45bcd0b364b1df`
- cleanup merge ancestor: `525ef0903e68b536ff5b22f985263ca737a67986`
- baseline Git status: empty, including all non-ignored untracked paths
- baseline `git diff --check`: exit `0`
- project `package-lock.json` SHA-256:
  `0dd1616e1ef3c596d5cdaa8e56994f26127c20943bfde066f26c9f924c65d732`
- registered preflight implementation SHA-256:
  `ac13dd8c6c590a6caf0b633c2ae50e47b410408feccbca4af697b2285f9309c4`
- candidate brief SHA-256:
  `72521c0ae90fc04d5d2e22ff3e3d0be5e96561fe3e2d3847b546c4fa26af1951`

The managed run path was proven absent before its first write.

## Work Graph

Exactly one serialized Agent 2 ephemeral generalist under the TASK shell base
contract is the integration owner. It may not delegate. The node performs the
provisioning commands and returns terminal evidence; ORCHESTRATOR then performs
fan-in validation and writes the manager return and handoff.

## Provisioning Sequence

1. Reconfirm that pre-dispatch baseline was clean and that the only current
   non-ignored changes are this managed run record.
2. Invoke the registered
   `tools/release/run_evidence_sweep.py::preflight_prerequisites(Path.cwd())`
   directly; preserve every error exactly.
3. Run `npm ci` from `WORKING_ROOT`, using only the committed
   `package-lock.json`; fail closed on nonzero exit or any lock/source change.
4. Invoke the same preflight again. Install Chromium only if its exact
   `missing local Playwright Chromium executable` error is present, using the
   project-local Playwright CLI.
5. Inventory the exact currently failing Cargo manifests from the authoritative
   preflight and, sequentially for only those manifests, run the minimal online
   `cargo fetch --manifest-path <manifest>` needed to create/fill ignored local
   lock/cache state. Fail closed on any command failure.
6. Invoke the exact registered preflight again. Closure requires zero errors.
7. Verify ignored-state containment, Git cleanliness apart from the managed
   record, `git diff --check`, claims/path/receipt validators, dependency-lock
   hashes, tool versions, and protected-path immutability.

## Exclusions

No reproduction clone, fixture generator, runner operation, runner test,
profile check, evidence sweep, reproduction bundle, source edit, lock update,
manifest edit, governance cleanup, sibling-project review, Git closeout, or
external repository operation is authorized.
