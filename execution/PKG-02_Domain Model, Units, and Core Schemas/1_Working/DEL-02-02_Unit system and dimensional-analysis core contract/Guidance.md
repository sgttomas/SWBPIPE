---
doc_id: DEL-02-02-GUIDANCE
doc_kind: deliverable.guidance
status: draft
created: 2026-04-30
---

# Guidance - Unit System and Dimensional-Analysis Core Contract

## Purpose

DEL-02-02 exists to give the solver, schema layer, rule evaluator, adapters, persistence, and reporting surfaces one conservative unit contract. Its central job is to prevent untagged or dimensionally incompatible engineering values from moving through the system silently.

This guidance is source-grounded in the sealed DEL-02-02 context, SOW-025, the project invariant catalog, the technical specification, the IP/data-boundary policy, validation strategy, and the SCA-001 architecture basis. It does not introduce protected code text, standards tables, commercial examples, or code-compliance claims.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Unit safety is mandatory | Treat unit metadata and dimensional compatibility as part of the domain model, not as UI decoration or optional import metadata. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/DIRECTIVE.md` Section 3 |
| Missing data is visible | Unknown units, missing units, unsupported conversion constants, or incomplete rule-check inputs should become diagnostics, `TBD`, or explicit open issues. | `docs/CONTRACT.md` OPS-K-DATA-2; `INIT.md` Agent rule |
| Preserve boundaries | Unit checks support mechanics and rule evaluation, but they do not establish professional code compliance or human approval. | `docs/CONTRACT.md` OPS-K-AUTH-1; `SOFTWARE_DECOMP.md` Section 8.1 AB-00-03 |
| Keep public data clean | Unit and conversion data introduced as public records need provenance and redistribution status; suspected protected content is stopped and escalated. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 3-5 |
| Determinism beats convenience | Prefer explicit registries, strict parsers, and repeatable conversion behavior over permissive parsing or hidden fallback conversion. | `docs/_Registers/ScopeLedger.csv` row SOW-025; `docs/VALIDATION_STRATEGY.md` Section 2 |
| Make adapters boring | Imports, exports, plugins, and adapters should pass through the same unit validation and diagnostics as internal services. | `SOFTWARE_DECOMP.md` Section 8.1 AB-00-07; `docs/SPEC.md` Section 1 |
| Keep schema and runtime aligned | JSON Schema quantity definitions and Rust domain-core types should represent the same contract. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` Sections 1-2 |

## Considerations

### Unit catalog and conversion sources

The accessible sources do not provide an authoritative unit catalog or conversion-factor list. A future implementation should therefore avoid presenting any first-pass unit list as final. Each accepted unit family and conversion factor should be backed by a permissible source or explicit project decision record. Public fixtures should remain invented or permissively sourced.

Before executable conversion tests or schema examples depend on a catalog, record the minimal early-test unit set and conversion source set as `TBD` or as a human-approved decision. The lensing worklist does not supply those units or factors.

### Dimensional signatures

Dimensional signatures should be stable identifiers or vectors that support equality and compatibility checks across calculations, schemas, imports, exports, rule evaluation, and reports. The exact basis is TBD. ASSUMPTION: likely first-pass dimensions include length, force, mass, time, temperature, angle/rotation, pressure/stress, moment, density, and stiffness-related derived dimensions, because the domain objects and solver/load/stress surfaces in `docs/SPEC.md` require these concepts. This assumption requires human review.

### Offset and reference quantities

Some engineering quantities require more than multiplicative scaling. Temperature scales versus temperature intervals, gauge versus absolute pressure, rotations versus mathematical angles, and normalized ratios need explicit semantics. Until those semantics are accepted, related conversions should remain `TBD` or emit blocking diagnostics.

### Storage and reproducibility

The architecture basis requires unit-aware, schema-governed, deterministic round trips and canonical JSON/JCS-compatible hashing where JSON payloads are hashed. This creates a design tension: preserving the user-entered unit representation helps auditability and round-trip behavior, while a canonical calculation representation helps deterministic computation. The recommended draft direction is to preserve the entered unit plus use explicit canonicalization rules for calculations and hashes, but the final data shape is a human design decision.

### Diagnostics

Unit errors should be specific enough for a GUI, CLI, report, rule evaluator, or adapter to identify what failed and how to remediate it. Use project diagnostic classes where applicable. For example, missing physical units for a solve-required load may be solve-blocking; a rule-pack input with incompatible units may be rule-check-blocking. Exact diagnostic code names are TBD.

### Protected-content boundary

Do not seed the unit system with copied standards tables, protected dimensional tables, code-derived formulas, commercial benchmark files, or proprietary vendor data. Unit mechanics and schemas may be public; protected or private engineering data belongs in user-controlled private paths unless redistribution rights are documented.

## Trade-offs

| Trade-off | Conservative direction |
|---|---|
| Broad unit catalog now vs. small verified catalog first | Start with the smallest source-backed or decision-backed catalog needed for tests and early solver work; expand through review. |
| Preserve entered units vs. normalize everything | Preserve entered units for audit/round-trip behavior and define a separate canonical calculation/hash path if accepted. |
| Floating point factors vs. exact/rational/decimal factors | TBD. Choose only after considering deterministic testing, Rust implementation cost, schema representation, and required tolerances. |
| Permissive import parsing vs. strict imports | Prefer strict imports with explicit diagnostics; adapters should not guess units silently. |
| Unit mismatch warning vs. hard error | Treat incompatible dimensions in calculations and schema validation as blocking. Use warnings only for provenance weakness or assumptions that do not invalidate the computation. |
| Embed conversion examples vs. keep examples deferred | Keep numeric examples deferred until sources or invented examples are reviewed for provenance and protected-content safety. |

## Examples

TBD. The accessible sources do not provide source-backed numeric unit-conversion examples or an accepted quantity schema. Do not add numeric examples until the schema, unit catalog, conversion constants, and tolerance policy have accepted sources or human decisions. Future examples should be original/invented or public/permissive, and should include:

- a valid same-dimension conversion case;
- an invalid cross-dimension conversion case;
- a unit-aware load or material-property schema fragment;
- a rule-pack unit mismatch case;
- a serialization round-trip case showing preserved unit metadata;
- a JSON hash-stability case where unit-bearing JSON payloads are hashed.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| C-DEL-02-02-001 | Repository overview text still describes v0.3 as the current downstream decomposition basis, while the sealed deliverable context and decomposition for this run identify revision 0.4 with SCA-001 injection. | `docs/README.md` governance map text | `_CONTEXT.md` Decomposition Reference and Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` revision notes | References, run record, source basis | Use the sealed `_CONTEXT.md` revision 0.4 and `SOFTWARE_DECOMP.md` revision 0.4 for this deliverable. | TBD |
