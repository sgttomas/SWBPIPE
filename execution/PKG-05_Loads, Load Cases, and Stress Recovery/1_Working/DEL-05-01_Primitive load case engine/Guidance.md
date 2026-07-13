# Guidance: DEL-05-01 Primitive load case engine

## Purpose

This deliverable frames primitive load categories as explicit mechanics inputs for the OpenPipeStress solver boundary. The current implementation evidence is the bounded `core/loads/primitive_loads` crate, which preserves primitive category identity, deterministic findings, boundary metadata, load-case record shape, diagnostic bridge records, equivalent-static mechanics handling, lumped equivalent nodal conversion, and straight-pipe axial-effect helpers without adding code-specific combinations or compliance logic.

## Principles

- Treat primitive load categories as mechanics-boundary records and validation surfaces, not as a finished load-combination or rule-pack engine.
- Keep primitive category, primitive load, primitive load case, load-case algebra, code combination, and rule-pack evaluation as separate concepts.
- Preserve unit/dimension intent and provenance references for load quantities, boundary records, load-case records, and diagnostic records.
- Use explicit findings/errors for missing targets, magnitudes, spans, properties, provenance, invalid dimensions, invalid directions, invalid topology, and non-finite values.
- Keep wind, seismic, and occasional loads as explicit equivalent mechanics inputs with explicit basis/provenance refs unless a later sealed scope authorizes dynamic methods or lawful procedure generation.
- Avoid copying or deriving protected standard content, tables, examples, coefficients, jurisdictional factors, or proprietary defaults.

## Considerations

| Topic | Guidance |
|---|---|
| Weight | Use explicit mechanics quantities such as element `ForcePerLength`; any mass-source, gravity-vector, density, and conversion policy must come from lawful upstream data or future sealed scope. |
| Pressure | Pressure can be carried as an element primitive load and interpreted for straight-pipe pressure thrust only with caller-supplied properties; pressure stress formulas and code stress categories belong elsewhere. |
| Thermal | Thermal can be carried as element temperature change and interpreted for straight-pipe axial force only with caller-supplied material/section properties; reference temperature and material provenance remain open policy. |
| Displacement | Imposed displacement is a support-target mechanics input; support/restraint behavior remains an interface with the owning solver/support deliverables. |
| Hydrotest | Hydrotest is represented as a primitive mechanics category; do not imply hydrotest procedure defaults, code requirements, or test-fluid assumptions without sourced input. |
| Wind/seismic/occasional | Current implemented behavior is explicit equivalent nodal force/moment or element `ForcePerLength` where supported; `prepare_equivalent_static_loads` requires `EquivalentStaticMechanicsBasis`, and acceleration/dynamic placeholders are not treated as dynamic procedure support. |
| Load-case records | Use `PrimitiveLoadCaseRecord` as a storage-neutral single-category boundary record. Mixed-category algebra and user-defined combinations remain DEL-05-02. |
| Boundary metadata | Boundary records should carry schema binding, target/basis refs, JCS payload refs, payload-hash refs, unit metadata, and provenance refs; `TBD` is rejected where a concrete boundary ref is required. |
| Diagnostics | Diagnostic bridge records preserve local finding codes and local classes for result-envelope handoff; they do not define a shared diagnostic enum or final application API. |

## Trade-offs

| Trade-off | Current position |
|---|---|
| Category coverage vs. behavior breadth | Cover all SOW-013 primitive categories, but only implement bounded mechanics preparation and validation behavior evidenced in `core/loads/primitive_loads`. |
| Public defaults vs. user-supplied data | Do not bundle protected, jurisdictional, or catalog values; require explicit lawful/user-supplied inputs and provenance references. |
| Static equivalent inputs vs. dynamic methods | Wind, seismic, and occasional behavior remains equivalent mechanics input in this slice with caller-supplied basis/provenance refs; dynamic procedure generation remains `TBD`. |
| Primitive load case vs. load algebra | The primitive record binds one category and sorted load IDs; mixed cases, load algebra, and combinations belong to DEL-05-02. |
| Diagnostic bridge vs. final envelope | The crate can produce storage-neutral diagnostic records; final result-envelope/API integration remains downstream. |
| Axial helper formulas vs. code stress evaluation | Thermal and pressure axial-effect helpers compute mechanics forces from explicit properties only; they are not pressure stress formulas, code checks, or allowables. |

## Evidence Use

- Use `core/loads/primitive_loads/README.md` for the current crate boundary statement.
- Use `core/loads/primitive_loads/src/lib.rs` for implemented API names and unit-test evidence.
- Use `MEMORY.md` for historical tranche evidence, preserved boundaries, and remaining TBDs.
- Use `_DEPENDENCIES.md` for the active downstream interface to DEL-05-02.
- Do not promote lifecycle status or claim release/professional acceptance from these documents alone.

## Examples

No normative numeric load examples, code-specific combinations, coefficient examples, or jurisdictional examples are introduced here. Test values in `src/lib.rs` are invented mechanics fixtures for unit-test coverage only and are not design defaults.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict detected between the deliverable-local truth set and current primitive-load crate evidence. | N/A | N/A | N/A | N/A | N/A |

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The primitive-load slice now includes explicit loads and the DEC-068 equivalent-static wind, seismic, and occasional generators. Dynamic loading and code-prescribed generation are outside this bounded slice unless separately implemented and evidenced.
