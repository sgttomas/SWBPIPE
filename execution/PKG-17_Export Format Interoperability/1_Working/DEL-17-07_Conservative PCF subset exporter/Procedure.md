# Procedure: DEL-17-07 Conservative PCF subset exporter

## Purpose

This procedure describes how to produce and use the conservative PCF subset exporter deliverable without relying on hidden translator defaults or making unsupported interoperability claims.

## Prerequisites

- DEL-17-01 source basis has been consumed for PCF source evidence and unresolved questions.
- DEL-17-02 export package/profile/stable-ID/loss-report contract has been consumed.
- Declared upstream dependencies are reviewed: `DEL-17-02`, `DEL-03-02`, `DEL-13-04`, and `DEL-15-02`.
- PCF source behavior is limited to accessible public/project-owned evidence.
- Fixture data is invented or otherwise redistribution-safe.
- No protected standards text, proprietary examples, private project data, commercial solver files, or license-bypass instructions are used.

## Steps

### 1. Establish the source basis

1. Read DEL-17-01 `Source_Basis_Register.md` and `CAEPIPE_Question_Dossier.md`.
2. Read DEL-17-02 `Specification.md` for profile, ID-map, manifest, and loss-report requirements.
3. Read `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` section `5. PCF`.
4. Read the admitted public PCF translator source slices before making any PCF behavior claim.
5. Mark unsupported, source-absent, or version-sensitive behavior as `TBD`.

### 2. Define the PCF profile

1. Create a profile record for the PCF target family.
2. Declare profile ID, profile version, target version basis, source-basis IDs, and boundary notes.
3. Declare unit policy, coordinate policy, identity policy, supported entity families, and loss-report policy.
4. Classify each candidate entity family as exported, omitted, approximated, delegated, unsupported, or `TBD`.
5. Keep target version/profile basis as `TBD` until public evidence or human project authority resolves it.

### 3. Select the conservative subset

1. Start from the plan-listed subset only.
2. Keep straight-pipe, bend, tee, reducer, flange, valve, end-connection, line/component identity, nominal-size, OD/wall-thickness, material/spec label, coordinate, and stable-ID behavior separate in the profile.
3. Require explicit unit-bearing and provenance-bearing source data for exported physical values.
4. Move support/restraint semantics to separate handling unless the selected profile proves reliable preservation.
5. Do not add hidden defaults for missing pressure, temperature, fluid density, component weight, material properties, support type, or boundary conditions.

### 4. Implement writer behavior in a later implementation tranche

1. Emit only profile-supported PCF records.
2. Emit deterministic ordering where the profile and package contract require it.
3. Preserve canonical IDs directly only when source-confirmed; otherwise write a stable ID sidecar.
4. Produce diagnostics when required source data is missing, ambiguous, unsupported, or delegated.
5. Produce a loss report for every export attempt.
6. Keep PCF writing inside the approved deliverable write scope for the implementation tranche.

### 5. Build invented fixtures

1. Use invented model names, line IDs, component IDs, coordinates, and unit-bearing values.
2. Record fixture provenance and redistribution status.
3. Include positive cases for supported subset records.
4. Include negative cases for unsupported supports/restraints, ambiguous units, missing OD/wall thickness, unsupported components, and unresolved boundary conditions.
5. Do not use proprietary plant models, vendor examples, protected standards data, owner criteria, commercial catalog values, material allowables, or SIF/flexibility data.

Fixture provenance records should use this minimum template before any fixture is admitted:

| Field | Required content |
|---|---|
| Fixture ID | Stable invented fixture identifier |
| Fixture purpose | Positive or negative case covered by the PCF profile |
| Invented model name | Artificial name with no client/vendor/project source |
| Invented line IDs and component IDs | Artificial IDs traceable to canonical OpenPipeStress IDs |
| Coordinates and unit-bearing values | Explicit units, dimensions, and provenance note that values are invented |
| Entity classes covered | Candidate PCF component/support/attribute classes exercised |
| Redistribution status | `public_permissive`, `private_only`, `unknown`, or `protected_suspected` as applicable |
| Contributor certification | Statement that the fixture is not copied from protected standards, vendor examples, proprietary plant models, owner criteria, catalog values, material allowables, or SIF/flexibility data |
| Review disposition | Accepted, rejected, quarantined, or human/legal review required |

Sources: `docs/IP_AND_DATA_BOUNDARY.md` public-data provenance and protected-content rules; DEL-17-02 fixture/data-boundary requirements; Guidance `Examples`.

### 6. Verify outputs

1. Confirm the PCF target file is emitted only for supported profile content.
2. Confirm the manifest references the profile, source basis, target file, stable ID sidecar, diagnostics, and loss report.
3. Confirm the stable ID sidecar links canonical IDs to emitted target records or loss-report entries.
4. Confirm every unsupported, approximated, delegated, omitted, or `TBD` behavior appears in the loss report.
5. Confirm no release, compatibility, code-compliance, formal-validation, or professional-acceptance claim is present.
6. Confirm generated PCF export evidence is not described as downstream import compatibility, solver validation, release readiness, code compliance, professional acceptance, or external-tool approval.

## Verification

For this Phase A documentation pass, verification consists of:

- confirming the four-document kit exists;
- checking the default section headings required by the four-documents skill;
- checking that source-grounding gaps remain `TBD`, assumptions, or conflict-table entries;
- checking that no implementation code, schema files, protected standards data, proprietary examples, or professional/release/compatibility claims were introduced.

Future implementation verification remains `TBD` and should be defined only after the PCF profile and writer tranche are sealed.

## Records

The deliverable should retain or produce:

- PCF subset profile;
- PCF target file;
- export manifest;
- stable ID sidecar map;
- diagnostics;
- loss report;
- invented fixture provenance records;
- run records for TASK execution and validation.
