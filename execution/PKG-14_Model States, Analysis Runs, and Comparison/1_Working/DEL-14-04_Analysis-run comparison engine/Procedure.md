# Procedure: DEL-14-04 Analysis-run comparison engine

**Generated:** 2026-05-03
**Document Role:** Operational draft
**Source Basis:** `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`

## Purpose

Describe the conservative production procedure for creating and verifying the DEL-14-04 analysis-run comparison engine artifact without inventing implementation details or engineering values.

## Prerequisites

- Read `_CONTEXT.md`, `_REFERENCES.md`, and the current deliverable-local dependency mirror before implementation work.
- Confirm the applicable scope items are SOW-072 and SOW-073, and that OBJ-016 is the objective context.
- Treat all existing approved DAG-002 rows in `Dependencies.csv` as ACTIVE evidence rows, not as rows to rewrite during this setup workflow.
- Confirm upstream contract availability before implementing dependent behavior:
  - DEL-14-02 analysis run records;
  - DEL-14-05 comparison mapping, tolerance, and export contracts;
  - DEL-08-04 result export format;
  - DEL-02-02 unit system and dimensional-analysis core contract;
  - architecture-basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
- Keep unsupported implementation choices as `TBD` until accepted by the appropriate upstream deliverable or human decision.

## Steps

1. Establish the comparison input pair from exact analysis-run records.
   - Required evidence includes exact model-state basis, solver version, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes.
   - Concrete schema/API field names are `TBD`.

2. Resolve comparison mapping.
   - Prefer stable-ID alignment where safe.
   - Use manual mappings where required by SOW-073.
   - Do not invent mapping workflow semantics; consume the DEL-14-05 contract when available.

3. Validate unit comparability before computing deltas.
   - Compare only same-dimension unit-bearing values after accepted normalization.
   - Emit diagnostics for missing, ambiguous, or unsupported unit metadata; do not silently supply dimensionless or default units.

4. Compute result deltas for mapped comparison subjects.
   - Include mapped nodes, elements, supports, terminals, stress/result locations, diagnostics, and settings where represented by the accepted upstream result/run records.
   - Preserve raw delta evidence before tolerance-based classification.

5. Apply tolerance profiles.
   - Use accepted tolerance profile contracts.
   - Defaults and mapping workflows remain `TBD` under OI-014.

6. Produce deterministic comparison output.
   - Use stable ordering and schema-first envelopes consistent with the accepted architecture basis.
   - Preserve diagnostics, provenance, units, hashes/status boundaries, and professional-boundary notices.

7. Create result delta tests.
   - Cover deterministic repeatability, stable-ID matching, manual mapping, unit-normalized deltas, missing-unit diagnostics, settings deltas, diagnostic deltas, and tolerance classification.
   - Use invented/public/permissive fixtures only; fixture set and command are `TBD`.

## Verification

| Check | Evidence |
|---|---|
| Inputs preserve exact run basis | Test or contract assertion that run metadata is available to comparison logic. |
| Determinism | Repeated comparison of identical input pair yields identical ordered output. |
| Mapping behavior | Stable-ID and manual-mapping cases both exercised. |
| Unit normalization | Same-dimension conversion path and missing/incompatible-unit diagnostic path both exercised. |
| Scope coverage | Mapped result locations, diagnostics, and settings are represented where upstream records expose them. |
| Tolerance behavior | Raw deltas remain available while tolerance classification changes with profile. |
| Governance boundary | Output avoids professional approval, certification, sealing, authentication, and automatic code-compliance labels. |
| Protected-content boundary | Fixtures and examples contain no protected standards text/tables, proprietary values, private project data, or private rule-pack payloads. |

## Records

- Run comparison engine implementation artifact (`TBD` path).
- Result delta tests (`TBD` path/command).
- Fixture provenance notes for any public examples.
- Diagnostics/status assertions.
- Mapping/tolerance contract references.
- Any unresolved implementation decisions carried forward as `TBD`.
