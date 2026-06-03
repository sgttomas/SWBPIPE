# TASK RUN: DEV-001 Stage 2 PKG-11 Finding Resolution

## Identity

- Date: 2026-05-16
- PackageID: PKG-11
- Posture: TASK / package-scoped finding resolution
- Scope: DEL-11-01 and DEL-11-04 PKG-02 downstream findings
- HumanDisposition changes: none; all affected findings remain `TBD`

## Inputs Read

- `docs/user_guide/index.md`
- `examples/models/invented/mechanics_only_toy_span.json`
- `examples/models/invented/fake_rule_pack_toy_model.json`
- `tests/test_invented_example_models.py`
- `schemas/model.schema.yaml`
- `schemas/project_persistence.schema.yaml`
- `core/project_persistence/service.py`
- DEL-11-01 and DEL-11-04 `_DEPENDENCIES.md`, `Dependencies.csv`, `_REVIEW.md`, and `Review_Findings.csv`

## Changes Made

- Added `HUMAN_APPROVED_FOR_PROJECT` to the user-guide status table as an external hash-bound human acceptance record only, with explicit no-software-approval/certification/sealing/authentication/code-compliance wording.
- Added invented public-safe `physical_source_of_truth` companion models to both DEL-11-04 example JSON files.
- Added analytical `source_model_ref` and `physical_to_analytical` traceability links from each analytical solver model back to its paired physical fixture.
- Replaced example model checksum placeholders with concrete sha256 project hashes using deterministic JSON/JCS-style canonicalization.
- Replaced the fake-rule example model's rule-pack reference checksum placeholder with a concrete sha256 checksum over `examples/rule_packs/invented_demo.yaml` and `canonicalization=NONE`.
- Expanded focused tests for full model JSON Schema validation, hash verification, project persistence schema validation, persistence diagnostics, project hash manifest consistency, and canonical JSON round-trip hash equality.
- Updated DEL-11-01 and DEL-11-04 `Review_Findings.csv` and `_REVIEW.md` to mark technical status `TECHNICALLY_ADDRESSED_PENDING_HUMAN` while preserving `HumanDisposition=TBD`.

## Validation

- `python3 -m pytest tests/test_invented_example_models.py tests/test_user_guide_status_wording.py -q`
  - Result: PASS, 8 passed.

## Boundaries

- No lifecycle/status files, DAG files, blocker queues, global dependency registers, release gates, unrelated package files, or protected/private data were edited.
- No release, certification, sealing, approval, professional acceptance, or code-compliance claim was made.
