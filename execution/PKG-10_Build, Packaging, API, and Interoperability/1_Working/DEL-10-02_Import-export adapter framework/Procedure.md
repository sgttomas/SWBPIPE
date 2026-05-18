# Procedure: DEL-10-02 Import/export adapter framework

## Purpose

Define how a future implementation agent should produce the import/export adapter framework while preserving the sealed scope and governance boundaries recorded for `DEL-10-02`.

## Prerequisites

- Confirm the sealed brief authorizes implementation work beyond this setup/document production run.
- Read `_CONTEXT.md`, `Specification.md`, and the applicable `AB-00-*` architecture basis rows.
- Confirm the write scope before creating adapter source, tests, sample adapters, package manifests, or repo-level artifacts.
- Confirm whether the target work is framework-only or a concrete external format. Concrete external format selection requires human approval if not already recorded.

## Steps

1. Identify the adapter operation types needed by the authorized implementation slice: import, export, validation, diagnostics, manifest/hash, rule-pack hook, report hook, and private-data boundary check.
2. Define adapter metadata fields for identity, capability, supported direction, supported payload class, source/provenance requirements, redistribution posture, and privacy behavior.
3. Route nontrivial adapter operations through schema-first command/query/job result envelopes.
4. Add unit-validation hooks before imported values can become domain objects.
5. Add provenance and redistribution-status validation before imported data can be accepted for reuse or public contribution.
6. Add diagnostics for missing fields, invalid units, missing provenance, unclear redistribution, protected-content suspicion, private-data export risk, and failed reasonableness checks.
7. Add rule-pack hooks only through the sandboxed, unit-aware rule-pack boundary; do not allow adapters to execute arbitrary rule code.
8. Add report/export hooks that preserve warnings, assumptions, limitations, provenance, and professional-responsibility notices.
9. Keep specific external formats, commercial tool behavior, public API transport, and package/container details as `TBD` unless a human ruling is cited.
10. Use invented data only for public sample adapters or fixtures.

## Verification

- Verify no protected standards text, copied tables, proprietary examples, or private project data are introduced.
- Verify missing required data and missing provenance produce diagnostics rather than defaults.
- Verify unit-bearing imported/exported values are dimensionally checked.
- Verify private-boundary checks run before shared exports.
- Verify result envelopes do not claim certification, approval, sealing, or automatic code compliance.
- Verify tests and examples use invented or otherwise redistributable data with provenance.

## Records

Future implementation work should record:

- interface files and schema contracts created;
- adapter validation tests and diagnostics tests;
- provenance/protected-content gate results;
- external format decisions and human approval references when applicable;
- open `TBD` decisions that remain after implementation.
