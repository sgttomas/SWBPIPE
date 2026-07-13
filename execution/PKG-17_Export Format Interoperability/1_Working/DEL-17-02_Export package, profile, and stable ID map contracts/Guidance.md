# Guidance: DEL-17-02 Export package, profile, and stable ID map contracts

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-17-02-DECL-003`.

## Reader Orientation

Use this deliverable as the shared contract spine for PKG-17. It tells later exporters what they must disclose and preserve before they write native JSON, CAEPIPE MBF, CAEPIPE harness records, stress-neutral CSV/JSON, PCF, GLB/glTF, or adapter SDK targets.

The important design rule is simple: an export may be partial, but it shall be explicit. Unsupported target behavior, omitted entities, approximations, delegated execution, and unanswered source questions must be visible in the package manifest, ID map, and loss report.

## Design Guidance

### Export package

An export package should be treated as an auditable bundle concept, not only as a target file. Even when the visible output is a single target file, the project still needs a manifest, stable-ID record, diagnostics, and loss report to explain what the output represents.

### Export profile

An export profile should be the only place where target-specific export assumptions are declared. Later implementation work should consume the profile and fail or warn from its declared coverage. Hidden target defaults are a risk because they make later parser, comparison, and support workflows ambiguous.

Profile authors should prefer narrow, source-confirmed profiles over broad profiles with unproven target behavior. If a target behavior is plausible but not yet sourced, mark it `TBD` and include the affected downstream deliverables.

Use `DEL-17-01 Source_Basis_Register.md` as the upstream authority for admitted source IDs. For non-trivial target statements, cite one or more of `CAEPIPE-IMPORT-MBF`, `CAEPIPE-EXPORT-DATA`, `CAEPIPE-EXPORT-MBF`, `CAEPIPE-BATCH`, `CAEPIPE-PCF`, `GLTF-2.0`, or a project reference such as `CONTRACT`, `IP-DATA`, `SPEC`, `TYPES`, and `DAG-006`. If only `PLAN-EXPORT-INTEROP` supports the planning direction, preserve its plan-location `TBD` and do not convert strategy into target-field evidence.

### Stable ID map

Stable IDs are the recovery path when a target format cannot preserve the OpenPipeStress model structure directly. The map should be useful for:

- locating a target artifact from a canonical model entity;
- tracing a target result row back to a model entity;
- explaining omitted entities;
- reconciling external run outputs with OpenPipeStress records;
- diagnosing target-specific approximations.

Do not use target-generated record order as the only identity mechanism unless the manifest also records why no stronger identity is available.

Distinguish direct target-carried IDs from sidecar mappings. A direct target-carried ID requires a source-confirmed target field, metadata slot, name, or equivalent carrier. A sidecar mapping is the default when the carrier is absent, ambiguous, or still `TBD`. For CAEPIPE MBF, `TBD-17-01-003` remains open, so later MBF work should not claim direct in-file stable-ID carrying until that question is closed.

### Loss report

A loss report should be written for normal exports, not only error cases. A successful target export can still have delegated, omitted, approximated, unsupported, or TBD behavior. This is especially important for CAEPIPE, PCF, and review-geometry targets where public source evidence does not authorize broad compatibility claims.

Use the categories consistently:

- `exported`: represented directly in the target artifact or package member;
- `omitted`: intentionally not emitted, with affected canonical IDs;
- `approximated`: emitted through a declared approximation;
- `delegated`: passed through to target configuration or a user-owned workflow without local interpretation;
- `unsupported`: known unsupported behavior or target limitation;
- `tbd`: behavior not source-confirmed.

Do not let `delegated` or `approximated` entries hide target limitations. If target code/check options appear, treat them as pass-through target configuration unless a separate public rule-pack design admits local logic.

## Target-Specific Carryforward

| Target path | Carryforward from DEL-17-01 |
|---|---|
| CAEPIPE MBF | CAEPIPE version/profile, initial MBF record families, required fields, and stable ID carrying strategy remain `TBD` until downstream source review or developer-team clarification closes them. |
| CAEPIPE external run/CSV | CSV section stability, parser coverage, and licensed executable availability remain `TBD`; harnesses are user-owned and optional. |
| PCF | Conservative subset and translator-default warning rules remain `TBD`; PCF work shall be loss-report driven. |
| GLB/glTF | Identity metadata and sidecar policy remain `TBD`; review geometry shall not be treated as solver validation. |
| Adapter SDK | Additional targets shall inherit this contract and declare their own source basis before target-specific claims. |

## External Execution Guidance

External target execution is optional and user-owned. A profile may record executable path fields, version fields, license/environment notes, invocation metadata, and output-member hashes where those are useful for a later harness. The contract shall not require bundled executables, copied target examples, license-independent operation, or local interpretation of target solver/code-check behavior.

## Boundary Guidance

- Use public, official, project-owned, or rights-cleared sources only.
- Do not copy protected standards tables, commercial examples, proprietary model files, or private target data.
- Do not describe external tool execution as bundled, automatic, or license-independent.
- Do not claim compatibility, code compliance, professional acceptance, formal validation, or release readiness from this contract.
- Do not resolve `TBD` behavior by inference from target names or common industry practice.

## Reviewer Checklist

- Does every target-specific statement trace to `DEL-17-01` or a project governance source?
- Are unresolved questions marked `TBD` instead of softened into support language?
- Does the ID-map guidance preserve canonical identity even when the target format cannot?
- Does the loss report include successful-export loss categories?
- Are later deliverables identified as consumers without being populated in this tranche?
- Did semantic matrix, lens-register, four-document, minimum-fileset, dependency-schema, and diff-hygiene validation pass before downstream consumption?
