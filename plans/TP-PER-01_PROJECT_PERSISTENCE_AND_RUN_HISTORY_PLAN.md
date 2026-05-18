---
doc_id: TP-PER-01
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-PER-01 Project Persistence And Run History Spine

## Purpose

Add a schema-shaped persistence service for invented or cleared project
payloads, immutable model-state refs, analysis-run history, deterministic
hashes, and round-trip validation.

TP-RUN-01 is the closed runtime baseline for this tranche. TP-PER-01 should
persist preview-run context around that baseline without adding final desktop
save/open UX, physical container format, CLI syntax, external storage, or
professional acceptance workflows.

## Scope

- Add `core/project_persistence` service functions for building, validating,
  hashing, and round-tripping `openpipestress.project_persistence` envelopes.
- Extend `schemas/project_persistence.schema.yaml` only as needed to carry
  optional immutable run-history refs or records.
- Use deterministic canonical JSON with stable key ordering for project
  payload, model payload, model-state records, analysis-run records, result
  envelope refs, and full project-envelope hashes.
- Add an invented public persistence fixture backed by a canonical-model
  payload rather than directly wrapping the product-preview model.
- Preserve existing TP-MAC-05 and TP-RUN-01 mechanics/runtime behavior.
- Update coordination handoff artifacts when the tranche closes.

## Boundaries

- No desktop file picker, save/open UX, or user-facing persistence workflow.
- No final physical project container or package format; keep
  `physical_container.status = "TBD"`.
- No CLI syntax, package scripts, external storage, network transport,
  migrations, private data handling beyond schema markers, or production
  persistence claims.
- No protected standards content, owner data, private project data, allowables,
  SIF/flexibility tables, code criteria, or hidden engineering defaults.
- No compliance, certification, sealing, professional approval,
  release-readiness, production-readiness, or engineering acceptance claims.

## Public Interfaces

- `build_project_persistence_envelope(...)`
- `validate_project_persistence_envelope(...)`
- `canonical_json(...)`
- `project_hash_manifest(...)`
- `round_trip_project_envelope(...)`

Persistence operation outputs must use structured diagnostics for schema,
migration, unit metadata, provenance, private-data, IP-boundary, and
professional-boundary failures. Analysis-run records continue to use
`schemas/analysis_run.schema.json`; model-state records continue to use
`schemas/model_state.schema.json`.

## Acceptance Criteria

- A valid invented persistence fixture contains a canonical-model payload,
  private-data marker, validation profile, round-trip manifest, hash manifest,
  run-history refs, and TP-RUN-01 result-envelope/endpoint-stress refs.
- The persistence service emits stable canonical hashes and changed hashes for
  changed payloads.
- Round-trip validation returns semantic equality for the invented fixture.
- Missing provenance, private-data, professional-boundary, or required
  run-history refs produce structured diagnostics.
- Existing product-preview, result, analysis-run, headless-runner, model-state,
  and persistence schema checks continue to pass.

## Verification

- `python3 tests/test_persistence_schema.py`
- `python3 tests/test_model_state_schema.py`
- `python3 tests/test_analysis_run_schema.py`
- `python3 tests/test_analysis_run_records.py`
- `python3 tests/test_headless_runner_contract.py`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py`
- `git diff --check`

## Assumptions

- This tranche is service plus envelope only.
- Public fixtures use invented data only.
- Existing TP-MAC-05 and TP-RUN-01 mechanics/runtime behavior remains
  unchanged.
- Physical project container, desktop save/open UX, final CLI syntax,
  migrations, external storage, and professional acceptance workflows remain
  deferred.

## Closeout

TP-PER-01 is implemented for the current schema-shaped persistence workflow.

Delivered:

- `core/project_persistence` service helpers for building, validating, hashing,
  and canonical round-tripping persistence envelopes;
- optional run-history support in `schemas/project_persistence.schema.yaml`
  for model-state refs, analysis-run refs or records, result-envelope refs,
  result refs, and hash manifests;
- explicit persistence professional-boundary fields and validation diagnostics;
- deterministic SHA-256 hash coverage for project payloads, model payloads,
  hashless project envelopes, and optional embedded run-history records;
- invented persisted-preview fixture with a canonical-model payload and
  TP-RUN-01 result-envelope/endpoint-stress refs;
- focused persistence service tests for validation, hashes, round-trip
  semantics, mutation detection, and missing boundary diagnostics.

Final verification was run on 2026-05-10:

- `python3 tests/test_persistence_schema.py`
- `python3 -m pytest tests/test_project_persistence_service.py`
- `python3 tests/test_model_state_schema.py`
- `python3 tests/test_analysis_run_schema.py`
- `python3 tests/test_analysis_run_records.py`
- `python3 tests/test_headless_runner_contract.py`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_results_schema.py`
- `python3 tests/test_results_schema.py`
- `python3 -m py_compile core/project_persistence/service.py core/project_persistence/__init__.py`
- `git diff --check`

Physical project containers, desktop save/open UX, final CLI syntax,
migrations, external storage, private data workflows beyond schema markers,
release/professional claims, and professional acceptance workflows remain
deferred.
