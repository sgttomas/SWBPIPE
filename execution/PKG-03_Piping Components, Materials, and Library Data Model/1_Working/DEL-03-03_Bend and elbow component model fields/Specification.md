# Specification: DEL-03-03 Bend and elbow component model fields

## Scope

This deliverable specifies setup evidence for a future bend/elbow component model slice. The planned implementation surface is limited to model fields and validation tests for bend/elbow geometry, user-entered SIFs, user-entered flexibility factors, and source metadata.

Out of scope:

- Solver implementation.
- Rule-pack evaluator implementation.
- B31J or other protected SIF/flexibility tables, formulas, examples, or paraphrased derivations.
- Public bundled engineering values or default code data.
- Certification, approval, sealing, or compliance claims.

## Requirements

| ID | Requirement | Basis | Verification Hook |
|---|---|---|---|
| R01 | The model must represent bend/elbow component identity separately from other component families. | DEL-03-03; PKG-03 | Validation tests cover bend/elbow record identity. |
| R02 | The model must provide fields for user-entered bend geometry. | SOW-007 | Schema/model tests confirm required geometry slots and unit metadata. |
| R03 | The model must provide fields for user-entered SIFs. | SOW-007 | Tests confirm values are accepted only as user/import inputs with provenance. |
| R04 | The model must provide fields for user-entered flexibility factors. | SOW-007 | Tests confirm values are accepted only as user/import inputs with provenance. |
| R05 | The model must carry source/provenance metadata for SIFs, flexibility factors, and imported or library-sourced bend data. | OPS-K-DATA-3; AB-00-04; AB-00-07 | Schema tests require provenance/source fields or explicit TBD markers. |
| R06 | The implementation must not bundle or encode protected SIF/flexibility tables, copied formulas, or protected examples. | OPS-K-IP-1; OPS-K-IP-3; SOW-007 note | Protected-content gate and review checklist reject bundled protected content. |
| R07 | Missing solve-required or rule-check-required values must produce explicit diagnostics, not silent defaults. | OPS-K-DATA-2; AB-00-06 | Validation tests assert missing values become findings. |
| R08 | Numeric geometry inputs must be unit-aware and dimensionally checked. | OPS-K-UNIT-1; AB-00-04 | Unit tests cover dimension validation and serialization. |
| R09 | Public rule-pack or example data associated with this model must use invented non-code values and non-engineering notices. | OPS-K-RULE-1 | Example/test fixture review confirms no code-derived values. |
| R10 | The model must remain compatible with the 3D centerline/frame analysis model, with local FEA handoff only as a downstream path. | OPS-K-MECH-1 | Model documentation and tests avoid shell/solid assumptions in the core component record. |
| R11 | Adapters/plugins must not bypass unit, provenance, redistribution, validation, or diagnostic boundaries. | AB-00-02; AB-00-07 | API/adapter tests are future TBD but this deliverable records the constraint. |

## Standards

| Reference | Status |
|---|---|
| SOW-007 | Available via decomposition/register text. |
| B31J or other protected SIF/flexibility sources | Not bundled, not paraphrased, and not used as public source text in this setup pass. |
| Project invariant catalog | Available in `docs/CONTRACT.md`. |

## Verification

- Confirm the four-document kit contains no protected standards text, SIF/flexibility tables, copied formulas, or invented engineering values.
- Confirm all unknown implementation specifics are marked `TBD`.
- Confirm planned validation tests cover user-entered geometry, user-entered SIFs, user-entered flexibility factors, provenance metadata, missing-value diagnostics, and unit metadata.
- Confirm outputs remain draft/proposal material until accepted by a human gate.

## Documentation

Required local setup artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/`
