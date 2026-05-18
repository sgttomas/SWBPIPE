# Guidance: DEL-17-02 Export package, profile, and stable ID map contracts

## Reader Orientation

Use this deliverable as the shared contract spine for PKG-17. It tells later exporters what they must disclose and preserve before they write native JSON, CAEPIPE MBF, CAEPIPE harness records, stress-neutral CSV/JSON, PCF, GLB/glTF, or adapter SDK targets.

The important design rule is simple: an export may be partial, but it shall be explicit. Unsupported target behavior, omitted entities, approximations, delegated execution, and unanswered source questions must be visible in the package manifest, ID map, and loss report.

## Design Guidance

### Export package

An export package should be treated as an auditable bundle concept, not only as a target file. Even when the visible output is a single target file, the project still needs a manifest, stable-ID record, diagnostics, and loss report to explain what the output represents.

### Export profile

An export profile should be the only place where target-specific export assumptions are declared. Later implementation work should consume the profile and fail or warn from its declared coverage. Hidden target defaults are a risk because they make later parser, comparison, and support workflows ambiguous.

Profile authors should prefer narrow, source-confirmed profiles over broad profiles with unproven target behavior. If a target behavior is plausible but not yet sourced, mark it `TBD` and include the affected downstream deliverables.

### Stable ID map

Stable IDs are the recovery path when a target format cannot preserve the OpenPipeStress model structure directly. The map should be useful for:

- locating a target artifact from a canonical model entity;
- tracing a target result row back to a model entity;
- explaining omitted entities;
- reconciling external run outputs with OpenPipeStress records;
- diagnosing target-specific approximations.

Do not use target-generated record order as the only identity mechanism unless the manifest also records why no stronger identity is available.

### Loss report

A loss report should be written for normal exports, not only error cases. A successful target export can still have delegated, omitted, approximated, unsupported, or TBD behavior. This is especially important for CAEPIPE, PCF, and review-geometry targets where public source evidence does not authorize broad compatibility claims.

## Target-Specific Carryforward

| Target path | Carryforward from DEL-17-01 |
|---|---|
| CAEPIPE MBF | CAEPIPE version/profile, initial MBF record families, required fields, and stable ID carrying strategy remain `TBD` until downstream source review or developer-team clarification closes them. |
| CAEPIPE external run/CSV | CSV section stability, parser coverage, and licensed executable availability remain `TBD`; harnesses are user-owned and optional. |
| PCF | Conservative subset and translator-default warning rules remain `TBD`; PCF work shall be loss-report driven. |
| GLB/glTF | Identity metadata and sidecar policy remain `TBD`; review geometry shall not be treated as solver validation. |
| Adapter SDK | Additional targets shall inherit this contract and declare their own source basis before target-specific claims. |

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
