# Procedure: DEL-13-01 Design knowledge schema and provenance model

## Purpose

Define a conservative procedure for producing and reviewing the future `schemas/design_knowledge.schema.json` artifact and its design knowledge provenance model. This procedure is a setup-stage operating guide, not implementation evidence.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Deliverable context for DEL-13-01 | Present in `_CONTEXT.md`. |
| Reference index | Present in `_REFERENCES.md`. |
| Approved dependency mirror | Present as `_DEPENDENCIES.md` and `Dependencies.csv`; preserve all approved DAG-002 rows as ACTIVE. |
| Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5. |
| Applicable invariants | `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |
| Implementation brief | Not created in this setup task, by user instruction. |
| Product schema artifact | TBD; no implementation artifact exists in this deliverable folder at setup time. |

## Steps

1. Confirm scope identity.
   - Verify the active deliverable is DEL-13-01 under PKG-13.
   - Confirm scope item SOW-067 and objective OBJ-014.

2. Establish the source boundary.
   - Use `_CONTEXT.md`, `_REFERENCES.md`, the approved local dependency mirror, the decomposition, and cited governing documents.
   - Do not import owner standards, protected code data, private project values, proprietary vendor data, or inaccessible standards text.

3. Draft the design knowledge taxonomy.
   - Include the sourced categories: endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, owner/project metadata, source notes, and assumptions.
   - Mark exact field names, enum values, coordinate-frame policy, and geometry representation as `TBD` unless later implementation sources resolve them.

4. Draft the provenance model.
   - Include support for source name, source location, source license or redistribution basis, contributor, contributor certification, redistribution status, and review status where public data contribution rules apply.
   - Include source notes and unresolved assumptions attached to affected design knowledge records.

5. Apply unit and missing-data rules.
   - For unit-bearing physical values, require explicit unit metadata unless a field is explicitly dimensionless, ratio, percentage, or coefficient.
   - Represent missing data, missing units, unknown source, and unresolved assumptions as explicit records/findings.

6. Preserve boundaries to adjacent deliverables.
   - Keep deterministic constraint validation in DEL-13-03.
   - Keep constraint entity/provenance details that are not design-knowledge records in DEL-13-02.
   - Keep physical-to-analytical transformation warnings and omissions in DEL-13-04.
   - Use links/references rather than duplicating adjacent deliverable responsibilities.

7. Review the public/private and professional boundaries.
   - Confirm no protected standards text, code-specific tables, proprietary project data, owner standards, private rule-pack data, or copied commercial examples are present.
   - Confirm no automatic professional approval, certification, sealing, authentication, or code-compliance status is introduced.

8. Validate artifacts when implementation exists.
   - Validate JSON Schema syntax and declared draft/version.
   - Validate fixtures against the schema when fixtures exist.
   - Run protected-content/private-data checks appropriate to public examples.
   - Run dependency schema validation on local `Dependencies.csv` when present.

## Verification

| Check | Expected result |
|---|---|
| Scope check | DEL-13-01, PKG-13, SOW-067, and OBJ-014 match `_CONTEXT.md` and decomposition. |
| Four-document consistency | Datasheet, Specification, Guidance, and Procedure use the same category list and boundary terms. |
| Unsupported details | Unsupported implementation specifics are marked `TBD` or `ASSUMPTION`. |
| Public/private boundary | No prohibited protected/private data is introduced. |
| Professional boundary | No automatic professional or code-compliance authority is introduced. |
| Dependency mirror | Existing approved DAG-002 rows remain ACTIVE and structurally valid. |

## Records

| Record | Purpose |
|---|---|
| `Datasheet.md` | Descriptive identity, attributes, conditions, construction status, and references. |
| `Specification.md` | Normative scope, requirements, standards, verification, and documentation. |
| `Guidance.md` | Directional principles, considerations, trade-offs, examples, and conflicts. |
| `Procedure.md` | Operational production/review sequence and evidence expectations. |
| `_SEMANTIC.md` | Semantic lens generated after initial four-document setup. |
| `_SEMANTIC_LENSING.md` | Candidate enrichment register generated from semantic lenses and production docs. |
| `Dependencies.csv` | Approved local DAG-002 mirror/evidence surface; not independently regenerated in conflict with the project rule. |
