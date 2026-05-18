# Procedure: DEL-02-01 Canonical domain model schema

## Purpose

This procedure describes how to produce and review the canonical domain model schema for DEL-02-01 without crossing the sealed write scope or introducing protected/code-specific data. It is operational guidance for the future `schemas/model.schema.yaml` artifact and its review evidence.

## Prerequisites

- Confirm the active deliverable is DEL-02-01 in PKG-02 and that the accepted context is `_CONTEXT.md` revision 0.4.
- Confirm the schema baseline is JSON Schema 2020-12 from SOW-041 and SCA-001.
- Read the applicable invariants in `docs/CONTRACT.md`, especially IP/data boundary, units, provenance, authority, reports, privacy, and agent constraints.
- Read `docs/TYPES.md` for stable identifiers, analysis-status vocabulary, epistemic labels, and provenance labels.
- Read `docs/DIRECTIVE.md`, `docs/SPEC.md`, and the relevant PRD sections for object ontology, domain layers, diagnostics, reports, and data-boundary requirements.
- Confirm no human-declared dependency list exists in `_DEPENDENCIES.md`; treat dependency coordination as externally managed until humans provide a concrete list.
- Confirm implementation-level decisions still marked TBD by SCA-001 are not silently resolved in this deliverable.

## Steps

1. Confirm scope and status.
   - Verify `_STATUS.md` permits work.
   - Verify write scope before editing; do not modify `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, or `_SEMANTIC.md`.

2. Inventory required schema object families.
   - Start with the DEL-02-01 description: project, model, node, element, material, component, load, result, and report.
   - Cross-check `docs/SPEC.md` Section 3 for adjacent object families such as Section, Support, Combination, and RulePack references.
   - Record each required, adjacent, or deferred family in the object-family coverage crosswalk before claiming schema coverage.
   - Mark any ownership ambiguity as `TBD` rather than expanding into another package.

3. Define root schema metadata.
   - Set JSON Schema 2020-12 dialect.
   - Add schema version and identity fields.
   - Add hooks for persistence compatibility, migration status, and canonical hash metadata where applicable.
   - Leave concrete `$id` URI and code-generation tooling as `TBD` until approved.

4. Define common reusable records.
   - Stable local identifiers and references.
   - Unit-bearing quantity structure.
   - Provenance/source and redistribution/private-status structure.
   - Diagnostics/result envelope structure.
   - Analysis-status and epistemic-label references.
   - Record which field classes require units or unit references, and which record families require provenance/status fields.

5. Define model object records.
   - Project and Model containers.
   - Node coordinates and six-degree-of-freedom context.
   - Element endpoints, type, material/section references, local coordinate basis, and result-station hooks.
   - Material, Component, Section, Support/Restraint, LoadCase, Combination, Result, and Report records at the level authorized for DEL-02-01.

6. Apply data-boundary guards.
   - Do not populate protected standards values, copied formulas, dimensional tables, allowable tables, commercial software examples, proprietary vendor catalog data, or private user data.
   - Require provenance or explicit `TBD` for public data examples.
   - Represent code-specific values as user-supplied/private inputs or rule-pack references, not public defaults.

7. Encode status and authority boundaries.
   - Use analysis statuses consistent with `docs/TYPES.md`.
   - Exclude any automatic code-compliant/certified/approved status.
   - Preserve separate mechanics result, user-rule-check, and human-review/acceptance records.

8. Prepare examples and fixtures only when authorized.
   - Use invented, original, public-domain, or permissively licensed data with provenance.
   - Mark examples as non-code and non-compliance demonstrations.
   - Create a fixture provenance manifest or record a `TBD` manifest path before example-based tests are accepted.
   - Quarantine and escalate any suspected protected or proprietary source material.

9. Verify schema behavior.
   - Run JSON Schema validation against valid and invalid invented fixtures.
   - Check object-family coverage against the crosswalk in `Specification.md`.
   - Check required units, provenance, status, diagnostics, and public/private boundary behavior.
   - Check diagnostic/result/report envelope compatibility, including code/class/severity, source, affected object, message, remediation, and provenance.
   - Check hash payload scope and canonicalization basis where JSON payload hashes are generated; record explicit deferrals where hashes are not yet used.
   - Perform deterministic round-trip checks when persistence work is available.
   - Record deferred checks as `TBD` rather than implying they passed.

10. Prepare review handoff.
   - Summarize schema coverage against SOW-041 and DEL-02-01.
   - List TBDs and conflict-table items needing human ruling.
   - Reference the durable human-ruling record for objective/revision conflicts, schema layout, `$id` URI, code-generation tooling, fixture organization, and persistence/hashing decisions; if no record exists, mark the record path `TBD`.
   - Identify any required `docs/TYPES.md` update without modifying it unless the write scope authorizes that file.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Only DEL-02-01 authorized files are modified during this run. |
| Default document check | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist and retain their default sections. |
| Source-grounding check | Non-trivial requirements cite local governance/register/source slices or are labeled `ASSUMPTION`/`TBD`. |
| Schema baseline check | JSON Schema 2020-12 remains the declared baseline. |
| Object-family check | Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, and Report are covered. |
| Object-family crosswalk check | Required, adjacent, reference-only, and deferred object families are classified before acceptance. |
| Unit/provenance check | Unit-aware and provenance/status requirements are visible and mapped to field or record families. |
| Diagnostic/report crosswalk check | Result/report records can carry diagnostic envelope fields, warning classes, provenance, input manifest, and report notices. |
| Hash-scope check | JSON payload hashes have a documented payload boundary and canonicalization basis, or an explicit deferral. |
| Fixture provenance check | Public examples and fixtures have a provenance manifest or a visible `TBD` manifest path. |
| Data-boundary check | No protected standards/code text, copied formulas, protected tables, proprietary data, or private user data are introduced. |
| Authority check | No compliance, certification, approval, sealing, or professional-reliance claim is made by the software/schema. |
| Conflict check | Objective/revision discrepancies are surfaced in `Guidance.md` for human ruling. |
| Status check | `_STATUS.md` moves from `OPEN` to `INITIALIZED` only after Pass 1+2 document creation succeeds. |

## Records

Records produced by document drafting/enrichment runs:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_run_records/TASK_RUN_2026-04-30_0120.md`
- `_run_records/TASK_RUN_2026-04-30_0141.md`
- `_run_records/TASK_RUN_2026-04-30_0941.md`
- `_run_records/TASK_RUN_2026-04-30_0941_001.md`
- `_run_records/TASK_RUN_2026-04-30_0941_002.md`
- `_run_records/TASK_RUN_2026-04-30_0941_003.md`
- `_STATUS.md` safe lifecycle update, if a Pass 1+2 run completes from `OPEN`; Pass 3 does not update `_STATUS.md`

Records produced by dependency extraction:

- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_2026-04-30_0941_004.md`

Records expected from future implementation/review work:

- `schemas/model.schema.yaml`
- schema validation fixtures and results
- object-family coverage crosswalk review evidence
- unit-applicability and provenance/status acceptance evidence
- diagnostic/result/report envelope crosswalk evidence
- hash payload-scope decision or documented deferral
- fixture provenance manifest for public examples or validation fixtures
- protected-content/provenance gate evidence
- deterministic round-trip evidence
- any approved `docs/TYPES.md` update
- human rulings for conflict-table entries and unresolved `TBD` decisions, with ruling record path or ID
