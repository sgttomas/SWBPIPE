---
doc_id: TP-VERIFY-017
doc_kind: implementation.plan
status: proposed
created: 2026-05-31
---

# TP-VERIFY-017 Release-Readiness Gap Remediation

## Purpose

Prepare the next bounded implementation tranche for evidence-preserving
remediation of the three release-readiness gaps recorded by
`TP-VERIFY-016_INTEGRATED_RELEASE_READINESS_SWEEP`.

This plan is a handoff artifact only. It does not implement the tranche, change
lifecycle state, promote candidate rows, close a release gate, authorize a
release, approve waivers, accept professional reliance, certify code
compliance, or create a professional approval record.

## Current State

- `DAG-005` is the approved active graph authority.
- `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`
  passed during planning, confirming current DEV-001 coordination derivatives.
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` reports 101 unblocked and
  0 blocked deliverables using active `DAG-005` edges only.
- `TP_VERIFY_016_INTEGRATED_RELEASE_READINESS_SWEEP.md` is present as current
  DEL-09-05 evidence and records a failing integrated release-readiness profile.
- The DEL-09-05 `MEMORY.md` addendum for TP-VERIFY-016 is treated as intended
  current baseline evidence.
- Current in-project dirty state includes the DEL-09-05 TP-VERIFY-016 evidence
  file, the DEL-09-05 memory addendum, and `execution/_Coordination/_COORDINATION.md`.
  These are not to be reverted by this tranche.
- Dirty git state in sibling project paths such as `../west-doe-combined/` is
  external-scope noise and must be recorded and bypassed, not fixed.

## Objective

Resolve the three TP-VERIFY-016 release-readiness command and evidence gaps so
the integrated release-readiness profile can pass without changing governance
authority, lifecycle state, implementation evidence rows, release records, or
professional-boundary decisions.

## Scope

### DEL-09-02 - Calculation Witness Tooling

Restore the validation-local formal witness tool surface referenced by the
existing TP-WITNESS-023 records and by `tests/test_calculation_witness.py`.

Expected changes:

- Add `validation/witness/tools/__init__.py`.
- Add `validation/witness/tools/witness_validator.py`.
- Provide the import surface required by tests:
  `WitnessError`, `load_json`, `evaluate_witness`, `render_markdown`, and
  `assert_generated_artifacts_current`.
- Provide CLI support for
  `python3 validation/witness/tools/witness_validator.py --write-generated --check-generated`.
- Preserve the existing witness contract:
  canonical SHA-256 over sorted-key compact JSON, validation-local OpenMath
  phrasebook only, deterministic Markdown and MathML renderings, schema
  validation, dimension checks, formula evaluation independent from production
  solver/stress/section-property code, and comparison against the existing
  invented result-export fixture.

### DEL-12-04 - Security Wording Gate

Reconcile the single failing security wording gate without weakening the
security policy or test intent.

Expected changes:

- Replace only the forbidden `real secret` phrase in DEL-12-04 `MEMORY.md`
  with non-example boundary wording.
- Do not add credential material, private payloads, private paths, protected
  standards content, or proprietary data.
- Do not weaken
  `tests/security/test_secret_private_library_handling.py::test_changed_files_do_not_embed_disallowed_example_content`.

### DEL-10-04 - Release-Readiness Command Surface

Correct the release-readiness script's coordination regression command so it
uses the existing test surface.

Expected changes:

- Change the coordination regression step in
  `tools/release/check_release_readiness.py` from direct pytest collection
  under `tools/coordination` to
  `python3 -m pytest -q tests/test_coordination_maintenance.py`.
- Update `tests/test_release_readiness_script.py` to lock the intended
  coordination command.
- Keep the readiness script provider-neutral, local-only, and dry-run by
  default.

### DEL-09-05 - Parent Fan-In

Record the remediation closeout as release-readiness evidence only.

Expected changes:

- Add `TP_VERIFY_017_RELEASE_READINESS_GAP_CLOSEOUT.md` under the DEL-09-05
  deliverable folder.
- Update DEL-09-05 `MEMORY.md` with concise TP-VERIFY-017 fan-in evidence.
- Add deliverable-local run records for the bounded worker slices as needed.
- Cite TP-VERIFY-016 as the baseline evidence and preserve it unchanged.

## Write Bounds

Allowed implementation write scope for the next session:

- `validation/witness/tools/**`
- `tests/test_calculation_witness.py` only if needed to preserve the existing
  test contract without weakening it
- DEL-09-02 `MEMORY.md` and `_run_records/**`
- DEL-12-04 `MEMORY.md`
- `tools/release/check_release_readiness.py`
- `tests/test_release_readiness_script.py`
- DEL-10-04 `MEMORY.md` and `_run_records/**`
- DEL-09-05 `TP_VERIFY_017_RELEASE_READINESS_GAP_CLOSEOUT.md`
- DEL-09-05 `MEMORY.md` and `_run_records/**`

Do not edit:

- lifecycle `_STATUS.md` files;
- DAG artifacts or dependency registers;
- blocker queues;
- `DEV-001_IMPLEMENTATION_EVIDENCE.csv`;
- release records or acceptance records;
- CI workflows, release automation, signing, attestation, or publishing
  surfaces;
- solver, rule-engine, GUI, report, export, or production schema behavior
  outside the explicit scope above.

## Validation

Before implementation and after implementation:

- `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`

Focused validation:

- `python3 validation/witness/tools/witness_validator.py --write-generated --check-generated`
- `python3 -m pytest -q tests/test_calculation_witness.py`
- `python3 -m pytest -q tests/security/test_secret_private_library_handling.py`
- `python3 -m pytest -q tests/test_release_readiness_script.py tests/test_coordination_maintenance.py`
- `python3 tools/release/check_release_readiness.py --profile python --execute`
- `python3 tools/release/check_release_readiness.py --profile security --execute`
- `python3 tools/release/check_release_readiness.py --profile all --execute`
- `git diff --check`

Acceptance criteria:

- TP-VERIFY-016-GAP-001 is resolved: Python contract tests collect and the
  calculation witness tests pass.
- TP-VERIFY-016-GAP-002 is resolved: the DEL-12-04 security wording gate passes
  without reducing private-data or secret-handling protections.
- TP-VERIFY-016-GAP-003 is resolved: the release-readiness command surface uses
  the existing coordination maintenance test and no longer runs a no-test
  directory probe.
- The integrated release-readiness profile passes.
- Coordination derivatives remain valid before and after the tranche.

## Explicit Non-Changes

This tranche must not:

- change lifecycle state;
- promote candidate rows;
- change DAG authority;
- regenerate or edit blocker queues except by separately approved coordination
  maintenance workflow;
- edit implementation evidence rows;
- create release records, acceptance records, waiver records, or release
  approvals;
- assert release readiness for reliance;
- assert certification, sealing, authentication, professional approval,
  professional reliance, or standards/code compliance.

## Assumptions

- The untracked TP-VERIFY-016 artifact and the DEL-09-05 memory addendum are
  intended current evidence, not files to revert.
- The current dirty `execution/_Coordination/_COORDINATION.md` change is
  outside this tranche's write scope unless the next session receives explicit
  direction to edit coordination records.
- Human-owned release governance remains open: CI provider, release matrix,
  numerical thresholds, coverage thresholds, signing, attestation, release
  authority, waiver roles, acceptance workflow, and professional-boundary
  decisions.
- Public validation fixtures remain invented or cleared public content only.
