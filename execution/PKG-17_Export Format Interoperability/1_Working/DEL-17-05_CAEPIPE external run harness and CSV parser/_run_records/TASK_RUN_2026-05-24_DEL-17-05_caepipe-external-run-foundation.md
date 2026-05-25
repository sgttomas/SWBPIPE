# TASK RUN: DEL-17-05 CAEPIPE external-run evidence foundation

**Date:** 2026-05-24  
**Persona:** WORKING_ITEMS  
**Deliverable:** DEL-17-05 CAEPIPE external run harness and CSV parser  
**Authority:** DAG-005 active graph authority; DEV-001 development path; candidate rows non-gating  
**Lifecycle:** Preserved as `SEMANTIC_READY`; no lifecycle promotion authorized

## Scope

Implemented a bounded foundation for optional CAEPIPE external-run evidence:

- deterministic skipped-run evidence when no user-owned executable is configured;
- parser-only evidence for invented public CSV fixtures;
- JSON Schema 2020-12 evidence package contract;
- invented CSV and package fixtures;
- focused tests for skip behavior, parser coverage, diagnostics, privacy boundaries, authority wording, and deterministic sidecar writing.

## Files In Scope

- `core/handoff/caepipe_external/`
- `schemas/caepipe_external_run.schema.json`
- `fixtures/caepipe_external/invented/`
- `tests/test_caepipe_external_run_package.py`
- this DEL-17-05 `MEMORY.md` and run record

## Boundary Controls

- No CAEPIPE executable is bundled, discovered, installed, licensed, or invoked.
- No private user output, proprietary example, protected standards table/value, or commercial tool payload is committed.
- Parsed CSV output is classified as regression and handoff evidence only.
- No release readiness, compatibility, code-compliance, solver-validation, certification, approval, sealing, or professional-reliance claim is made.

## Remaining TBDs

- Live invocation profile and command-line/batch-mode reconciliation.
- Source-confirmed parser coverage beyond invented public fixture sections.
- Target version/profile closure.
- Any future live-run evidence interpretation, which remains user-owned and non-authoritative unless separately governed.

## Validation

Validation passed:

```bash
python3 -m py_compile core/handoff/caepipe_external/*.py tests/test_caepipe_external_run_package.py
pytest tests/test_caepipe_external_run_package.py
pytest tests/test_caepipe_mbf_export_package.py tests/test_native_json_export_package.py tests/test_handoff_export_workflow.py
git diff --check -- core/handoff/caepipe_external schemas/caepipe_external_run.schema.json fixtures/caepipe_external tests/test_caepipe_external_run_package.py 'execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser'
```
